use std::{collections::HashMap, convert::TryFrom};

use anyhow::{Context, Error, Result, bail};
use i256::{I256, U256};
use indexmap::IndexMap;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};
use serde_json::Value;

use crate::{
    account::address::Address,
    common::crypto::{is_b58, is_hex_string},
    config::config_contracts::HelperFieldType,
    contracts_funcs::compile_project::{
        compile_project_deserialize::FieldValueHelper,
        compile_project_structs::{FieldsTypesMapMut, FieldsVec, Struct},
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RalphValue {
    Bool(bool),
    U256(U256),
    I256(I256),
    ByteVec(Vec<u8>),
    Address(String),
    Array(Vec<RalphValue>), // Same Type
    Tuple(Vec<RalphValue>), // Different type
    Structure(HashMap<String, RalphValue>),
}

impl Serialize for RalphValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (type_name, value) = match self {
            Self::Bool(b) => ("Bool", serde_json::to_value(b).map_err(ser::Error::custom)?),
            Self::U256(u) => (
                "U256",
                serde_json::to_value(u.to_string()).map_err(ser::Error::custom)?,
            ),
            Self::I256(i) => (
                "I256",
                serde_json::to_value(i.to_string()).map_err(ser::Error::custom)?,
            ),
            Self::ByteVec(bytes) => (
                "ByteVec",
                serde_json::to_value(hex::encode(bytes)).map_err(ser::Error::custom)?,
            ),
            Self::Address(addr) => (
                "Address",
                serde_json::to_value(addr).map_err(ser::Error::custom)?,
            ),
            Self::Array(arr) => (
                "Array",
                serde_json::to_value(arr).map_err(ser::Error::custom)?,
            ),
            _ => {
                return Err(ser::Error::custom(
                    "Unsupported RalphValue type (Structure, Tuple)",
                ));
            },
        };

        let mut obj = serde_json::Map::new();
        obj.insert(
            "type".to_string(),
            serde_json::Value::String(type_name.to_string()),
        );
        obj.insert("value".to_string(), value);
        obj.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for RalphValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        let obj = value
            .as_object()
            .ok_or_else(|| de::Error::custom("Expected object for RalphValue"))?;

        let type_str = obj
            .get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| de::Error::custom("Missing or invalid 'type' field in RalphValue"))?;

        let type_name: TypeName = type_str.try_into().map_err(de::Error::custom)?;

        let value_field = obj
            .get("value")
            .ok_or_else(|| de::Error::custom("Missing 'value' field in RalphValue"))?;

        Self::from_typename_and_value(&type_name, value_field).map_err(de::Error::custom)
    }
}

fn try_into_field(
    initial_field_name: &str,
    initial_field: HelperFieldType,
    fields_types: &FieldsTypesMapMut,
    override_type: Option<&(TypeName, bool)>,
) -> Result<Vec<(RalphValue, bool)>> {
    let (type_name, is_mutable) = if let Some(override_type) = override_type {
        override_type
    } else {
        fields_types.get(initial_field_name).context(format!(
            "Type for field '{initial_field_name}' not found in provided fields types map"
        ))?
    };

    let value = match initial_field {
        HelperFieldType::String(s) => {
            // Bool, Numbers and String will be parsed using the default RalphValue parser
            // However, we need to make an exception for ByteVec which may be human readable
            // strings. This is because the blockchain automatically encodes
            // strings in hex, and since we use the default parser, we need to convert it
            // before calling the function So we need to check if the input is a
            // hex string (which always begin with 0x) to seperate the true hex strings
            // inputs from human readable ones
            let s = if *type_name == TypeName::ByteVec {
                if is_hex_string(&s) {
                    s.strip_prefix("0x")
                        .context("Expected hex string with '0x' or '0X' prefix")?
                        .to_string()
                } else {
                    hex::encode(s.as_bytes())
                }
            } else {
                s
            };

            vec![(
                RalphValue::from_typename_and_value(type_name, &Value::String(s))?,
                *is_mutable,
            )]
        },
        HelperFieldType::Array(arr) => {
            if let TypeName::Array((elem_type, elem_size)) = type_name {
                if arr.len() != *elem_size {
                    bail!(
                        "Array length mismatch for field '{initial_field_name}': expected\
                        {elem_size}, got {}",
                        arr.len()
                    )
                }

                let values = arr
                    .into_iter()
                    .map(|v| {
                        try_into_field(
                            "",
                            v,
                            fields_types,
                            Some(&(*elem_type.clone(), *is_mutable)),
                        )
                    })
                    .collect::<Result<Vec<_>>>()?;

                values.into_iter().flatten().collect()
            } else if let TypeName::Tuple(elem_types) = type_name {
                if arr.len() != elem_types.len() {
                    bail!(
                        "Tuple length mismatch for field '{initial_field_name}': expected {},\
                        got {}",
                        elem_types.len(),
                        arr.len()
                    )
                }

                let values = arr
                    .into_iter()
                    .zip(elem_types.iter())
                    .map(|(v, ty)| {
                        try_into_field("", v, fields_types, Some(&(ty.clone(), *is_mutable)))
                    })
                    .collect::<Result<Vec<_>>>()?;

                values.into_iter().flatten().collect()
            } else {
                bail!(
                    "Type mismatch: expected Array type for field '{initial_field_name}',\
                    got {type_name:?}",
                );
            }
        },
        HelperFieldType::Structure(helper_fields) => {
            if let TypeName::Structure((_struct_name, struct_fields)) = type_name {
                let zipped_fields = struct_fields
                    .into_iter()
                    .filter_map(|(k, v1)| helper_fields.get(k).map(|v2| (k, (v1, v2.to_owned()))))
                    .collect::<Vec<_>>();

                if zipped_fields.is_empty() {
                    bail!(
                        "No matching fields found in structure for '{initial_field_name}' in\
                        '{struct_fields:?}'",
                    );
                }

                zipped_fields
                    .into_iter()
                    .map(|(field_name, (type_name, value))| {
                        try_into_field(field_name, value, struct_fields, Some(type_name))
                    })
                    .collect::<Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect()
            } else {
                bail!(
                    "Type mismatch: expected Structure type for field '{initial_field_name}',\
                    got {type_name:?}"
                );
            }
        },
    };
    Ok(value)
}

pub fn config_fields_to_vec(
    initial_fields: &HashMap<String, HelperFieldType>,
    fields_types: &FieldsTypesMapMut,
) -> Result<FieldsVec> {
    let values = fields_types
        .into_iter()
        .map(|(name, _)| match name.as_str() {
            "__stdInterfaceId" => Ok(vec![]), // Skip this field, it's automatically added by
            // compiler
            _ => {
                let helper_field = initial_fields
                    .get(name)
                    .context(format!(
                        "Field '{name}' not found in provided initial fields"
                    ))?
                    .to_owned();

                try_into_field(name, helper_field, fields_types, None)
            },
        })
        .collect::<Result<Vec<Vec<_>>>>()?;
    Ok(values.into_iter().flatten().collect())
}

impl TryFrom<&str> for RalphValue {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
        if is_b58(s) {
            let s = &s.get(4..).context("Invalid b58 str")?; // remove b58: prefix
            let addr = Address::new_b58(s)?;
            return Ok(Self::ByteVec(addr.bytes));
        }

        let hex_str = hex::encode(s.as_bytes());
        Self::try_into_hexified_str(&hex_str)
    }
}

impl RalphValue {
    pub fn from_typename_and_value(ty: &TypeName, value: &Value) -> Result<Self> {
        match ty {
            TypeName::Bool => {
                if let Some(b) = value.as_bool() {
                    Ok(Self::Bool(b))
                } else if let Some(s) = value.as_str() {
                    match s {
                        "true" => Ok(Self::Bool(true)),
                        "false" => Ok(Self::Bool(false)),
                        _ => bail!("Expected 'true' or 'false' string for Bool"),
                    }
                } else {
                    bail!("Expected bool value or 'true'/'false' string")
                }
            },
            TypeName::U256 => {
                let n = value
                    .as_str()
                    .context("Expected U256 as string or number")?;
                let parsed = n.parse::<U256>().map_err(Error::msg)?;
                Ok(Self::U256(parsed))
            },
            TypeName::I256 => {
                let n = value
                    .as_str()
                    .context("Expected I256 as string or number")?;
                let parsed = n.parse::<I256>().map_err(Error::msg)?;
                Ok(Self::I256(parsed))
            },
            TypeName::ByteVec => {
                if let Some(arr) = value.as_array() {
                    let bytes = arr
                        .iter()
                        .map(|v| {
                            v.as_u64()
                                .map(|b| b as u8)
                                .context("Expected u8 in ByteVec array")
                        })
                        .collect::<Result<Vec<u8>>>()?;
                    Ok(Self::ByteVec(bytes))
                } else if let Some(s) = value.as_str() {
                    Self::try_into_hexified_str(s)
                } else {
                    bail!("Expected ByteVec as array or string")
                }
            },
            TypeName::Address => {
                let addr = value.as_str().context("Expected Address as string")?;
                Ok(Self::Address(addr.to_string()))
            },
            TypeName::Array((elem_type, elem_size)) => {
                let array = value.as_array().context("Expected Array as array")?;

                if array.len() != *elem_size {
                    bail!(
                        "Array length mismatch: expected {}, got {}",
                        elem_size,
                        array.len()
                    )
                }

                Ok(Self::Array(
                    array
                        .iter()
                        .map(|elem| Self::from_typename_and_value(elem_type, elem))
                        .collect::<Result<Vec<_>>>()?,
                ))
            },
            TypeName::Structure((_, fields)) => {
                let obj = value.as_object().context("Expected Structure as object")?;

                let mut structure = HashMap::new();
                for (field_name, (field_ty, _)) in fields {
                    let field_value = obj
                        .get(field_name)
                        .context(format!("Missing field '{field_name}' in structure"))?;
                    let parsed_value = Self::from_typename_and_value(field_ty, field_value)?;
                    structure.insert(field_name.clone(), parsed_value);
                }
                Ok(Self::Structure(structure))
            },
            TypeName::Tuple(elems) => {
                let arr = value.as_array().context("Expected Tuple as array")?;
                if arr.len() != elems.len() {
                    bail!(
                        "Tuple length mismatch: expected {}, got {}",
                        elems.len(),
                        arr.len()
                    );
                }

                let values = arr
                    .iter()
                    .zip(elems.iter())
                    .map(|(v, ty)| Self::from_typename_and_value(ty, v))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Self::Tuple(values))
            },
            TypeName::Other(name) => {
                if let Some(s) = value.as_str() {
                    // If the value is a string, treat it as a contract reference
                    // (the value must be the contract ID)
                    let decoded = bs58::decode(s)
                        .into_vec()
                        .context("Failed to decode base58 address")?;

                    return Ok(Self::ByteVec(
                        decoded
                            .get(1..)
                            .context("Invalid b58 typename str")?
                            .to_vec(),
                    ));
                }

                bail!("Unsupported type name or structure: {name}")
            },
        }
    }

    // This is only used on hexified strings, don't put "human readable" strings
    pub fn try_into_hexified_str(s: &str) -> Result<Self> {
        let decoded = hex::decode(s)?;
        Ok(Self::ByteVec(decoded))
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum TypeName {
    Bool,
    U256,
    I256,
    ByteVec,
    Address,
    Array((Box<TypeName>, usize)),
    Tuple(Vec<TypeName>),
    Structure((String, FieldsTypesMapMut)),
    Other(String),
}

impl TryFrom<&str> for TypeName {
    type Error = anyhow::Error;

    fn try_from(name: &str) -> Result<Self> {
        Self::from_name_and_structures(name, &HashMap::new())
    }
}

impl TypeName {
    pub fn from_name_and_structures(
        name: &str,
        structures: &HashMap<String, Struct>,
    ) -> Result<Self> {
        match name {
            "Bool" => Ok(Self::Bool),
            "U256" => Ok(Self::U256),
            "I256" => Ok(Self::I256),
            "ByteVec" => Ok(Self::ByteVec),
            "Address" => Ok(Self::Address),
            s if s.starts_with('[') && s.ends_with(']') => {
                // Examples: [U256; 2]
                // Examples: [[U256; 2]; 2]
                let inner = &s.get(1..s.len() - 1).context("Invalid type array")?;
                let (elem_type_str, elem_size) = if let Some((elem, size)) = inner.rsplit_once(';')
                {
                    let elem = elem.trim();
                    let size_part = size
                        .trim()
                        .parse::<usize>()
                        .context("Invalid array size in type annotation")?;
                    (elem, size_part)
                } else {
                    bail!("Dynamic-size arrays are not supported in this context")
                };
                Ok(Self::Array((
                    Box::new(Self::from_name_and_structures(elem_type_str, structures)?),
                    elem_size,
                )))
            },
            s if s.starts_with('(') && s.ends_with(')') => {
                // Example: (U256,ByteVec,Bool)
                let inner = &s.get(1..s.len() - 1).context("Invalid type tuple")?;
                let elems = inner
                    .split(',')
                    .map(|part| Self::from_name_and_structures(part, structures))
                    .collect::<Result<Vec<_>>>()?;
                Ok(Self::Tuple(elems))
            },
            s if structures.contains_key(s) => {
                let target_struct = structures
                    .get(s)
                    .context(format!("Structure named '{s}' isn't found"))?;

                let fields = target_struct
                    .field_types
                    .iter()
                    .zip(target_struct.is_mutable.iter())
                    .zip(target_struct.field_names.iter())
                    .map(|((value_name, is_mutable), field_name)| {
                        Ok((
                            field_name.clone(),
                            (
                                Self::from_name_and_structures(value_name, structures)?,
                                *is_mutable,
                            ),
                        ))
                    })
                    .collect::<Result<IndexMap<_, _>>>()?;
                Ok(Self::Structure((s.to_string(), fields)))
            },
            s => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct FieldValue {
    pub type_name: TypeName,
    pub value: RalphValue,
}

// This method of deserializing must be used when you know there will be no
// structures
impl<'de> Deserialize<'de> for FieldValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let helper = FieldValueHelper::deserialize(deserializer)?;
        let type_name = helper
            .type_name
            .as_str()
            .try_into()
            .map_err(de::Error::custom)?;
        let value = RalphValue::from_typename_and_value(&type_name, &helper.value)
            .map_err(de::Error::custom)?;
        Ok(Self { type_name, value })
    }
}

impl TryFrom<Value> for FieldValue {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        let type_name = value
            .as_str()
            .context(format!(
                "Expected type name as string, found: {value:?} in FieldValue",
            ))?
            .try_into()?;
        let value = RalphValue::from_typename_and_value(&type_name, &value)
            .context("Failed to convert Value to RalphValue")?;
        Ok(Self { type_name, value })
    }
}
