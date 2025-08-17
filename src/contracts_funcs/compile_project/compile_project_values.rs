use anyhow::{Context, Error, Result, anyhow};
use i256::{I256, U256};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de, ser};
use serde_json::Value;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::hash::{Hash, Hasher};

use crate::config::config_contracts::{ConfigContract, HelperFieldType};
use crate::contracts_funcs::compile_project::compile_project::{FieldsTypesMapMut, Struct};
use crate::contracts_funcs::compile_project::compile_project_deserialize::FieldValueHelper;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RalphValue {
    Bool(bool),
    U256(U256),
    I256(I256),
    ByteVec(Vec<u8>),
    Address(String),
    Array(Vec<RalphValue>),
    Map(HashMap<RalphValue, RalphValue>),
    Structure(HashMap<String, RalphValue>),
}

impl Serialize for RalphValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let (type_name, value) = match self {
            RalphValue::Bool(b) => ("Bool", serde_json::to_value(b).map_err(ser::Error::custom)?),
            RalphValue::U256(u) => (
                "U256",
                serde_json::to_value(&u.to_string()).map_err(ser::Error::custom)?,
            ),
            RalphValue::I256(i) => (
                "I256",
                serde_json::to_value(&i.to_string()).map_err(ser::Error::custom)?,
            ),
            RalphValue::ByteVec(bytes) => (
                "ByteVec",
                serde_json::to_value(&hex::encode(bytes)).map_err(ser::Error::custom)?,
            ),
            RalphValue::Address(addr) => (
                "Address",
                serde_json::to_value(addr).map_err(ser::Error::custom)?,
            ),
            RalphValue::Array(arr) => (
                "Array",
                serde_json::to_value(arr).map_err(ser::Error::custom)?,
            ),
            _ => {
                return Err(ser::Error::custom(
                    "Unsupported RalphValue type (including Map and Structure)",
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

        RalphValue::from_typename_and_value(&type_name, value_field).map_err(de::Error::custom)
    }
}

impl ConfigContract {
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
                "Type for field '{}' not found in provided types map",
                initial_field_name
            ))?
        };

        let value = match initial_field {
            HelperFieldType::String(s) => {
                vec![(
                    RalphValue::from_typename_and_value(type_name, &Value::String(s))?,
                    is_mutable.clone(),
                )]
            },
            HelperFieldType::Array(arr) => {
                if let TypeName::Array(elem_ty) = type_name {
                    let values = arr
                        .into_iter()
                        .map(|v| {
                            ConfigContract::try_into_field(
                                "",
                                v,
                                fields_types,
                                Some(&(*elem_ty.clone(), is_mutable.clone())),
                            )
                        })
                        .collect::<Result<Vec<_>>>()?;

                    values.into_iter().flatten().collect()
                } else {
                    return Err(anyhow!(
                        "Type mismatch: expected Array type for field '{}', got {:?}",
                        initial_field_name,
                        type_name
                    ));
                }
            },
            HelperFieldType::Structure(helper_fields) => {
                if let TypeName::Structure(struct_fields) = type_name {
                    let zipped = helper_fields
                        .into_iter()
                        .filter_map(|(k, v1)| struct_fields.get(&k).map(|v2| (k, (v1, v2))))
                        .collect::<Vec<(String, (HelperFieldType, &(TypeName, bool)))>>();

                    zipped
                        .into_iter()
                        .map(|(field_name, (value, type_name))| {
                            ConfigContract::try_into_field(
                                &field_name,
                                value,
                                struct_fields,
                                Some(type_name),
                            )
                        })
                        .collect::<Result<Vec<_>>>()?
                        .into_iter()
                        .flatten()
                        .collect()
                } else {
                    return Err(anyhow!(
                        "Type mismatch: expected Structure type for field '{}', got {:?}",
                        initial_field_name,
                        type_name
                    ));
                }
            },
        };
        Ok(value)
    }

    pub fn try_into_fields(
        initial_fields: HashMap<String, HelperFieldType>,
        fields_types: &FieldsTypesMapMut,
    ) -> Result<Vec<(RalphValue, bool)>> {
        let values = initial_fields
            .into_iter()
            .map(|(name, helper_field)| {
                let val = ConfigContract::try_into_field(&name, helper_field, fields_types, None)?;
                Ok(val)
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        Ok(values.into_iter().flatten().collect())
    }
}

impl Hash for RalphValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Hash the variant discriminant first so different variants
        // with the same inner value don't collide
        std::mem::discriminant(self).hash(state);
        match self {
            RalphValue::Bool(b) => b.hash(state),
            RalphValue::U256(u) => u.hash(state),
            RalphValue::I256(i) => i.hash(state),
            RalphValue::ByteVec(bytes) => bytes.hash(state),
            RalphValue::Address(addr) => addr.hash(state),
            RalphValue::Map(map) => {
                for (key, value) in map {
                    key.hash(state);
                    value.hash(state);
                }
            },
            RalphValue::Structure(structure) => {
                for (key, value) in structure {
                    key.hash(state);
                    value.hash(state);
                }
            },
            RalphValue::Array(arr) => {
                for value in arr {
                    value.hash(state);
                }
            },
        }
    }
}

impl TryFrom<&str> for RalphValue {
    type Error = anyhow::Error;

    fn try_from(s: &str) -> Result<Self> {
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
                        _ => Err(anyhow!("Expected 'true' or 'false' string for Bool")),
                    }
                } else {
                    Err(anyhow!("Expected bool value or 'true'/'false' string"))
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
                        .collect::<Result<Vec<u8>>>();
                    Ok(Self::ByteVec(bytes?))
                } else if let Some(s) = value.as_str() {
                    Ok(Self::try_into_hexified_str(s)?)
                } else {
                    Err(anyhow!("Expected ByteVec as array or string"))
                }
            },
            TypeName::Address => {
                let addr = value.as_str().context("Expected Address as string")?;
                Ok(Self::Address(addr.to_string()))
            },
            TypeName::Map(key_ty, val_ty) => Ok(Self::Map(
                value
                    .as_object()
                    .context("Expected Map as object")?
                    .iter()
                    .map(|(k, v)| {
                        Ok((
                            Self::from_typename_and_value(key_ty, &Value::String(k.clone()))?,
                            Self::from_typename_and_value(val_ty, v)?,
                        ))
                    })
                    .collect::<Result<HashMap<_, _>>>()?,
            )),
            TypeName::Array(elem_ty) => Ok(Self::Array(
                value
                    .as_array()
                    .context("Expected Array as array")?
                    .iter()
                    .map(|elem| Self::from_typename_and_value(elem_ty, elem))
                    .collect::<Result<Vec<_>>>()?,
            )),
            TypeName::Structure(fields) => {
                let obj = value.as_object().context("Expected Structure as object")?;

                let mut structure = HashMap::new();
                for (field_name, (field_ty, _)) in fields {
                    let field_value = obj
                        .get(field_name)
                        .with_context(|| format!("Missing field '{}' in structure", field_name))?;
                    let parsed_value = Self::from_typename_and_value(field_ty, field_value)?;
                    structure.insert(field_name.clone(), parsed_value);
                }
                Ok(Self::Structure(structure))
            },
            TypeName::Other(name) => {
                if let Some(s) = value.as_str() {
                    // If the value is a string, treat it as a contract reference
                    // (the value must be the contract ID)
                    return Ok(Self::ByteVec(s.try_into()?));
                }

                Err(anyhow!("Unsupported type name or structure: {}", name))
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
    Map(Box<TypeName>, Box<TypeName>),
    Array(Box<TypeName>),
    Structure(HashMap<String, (TypeName, bool)>),
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
            s if s.starts_with("Map[") && s.ends_with(']') => {
                // Example: Map[U256,U256]
                let inner = &s[4..s.len() - 1];
                let mut parts = inner.split(',');
                let key = parts.next().context("Missing key type in Map")?;
                let value = parts.next().context("Missing value type in Map")?;
                Ok(Self::Map(
                    Box::new(Self::from_name_and_structures(key, structures)?),
                    Box::new(Self::from_name_and_structures(value, structures)?),
                ))
            },
            s if s.starts_with("Array[") && s.ends_with(']') => {
                // Example: Array[U256]
                let inner = &s[6..s.len() - 1];
                Ok(Self::Array(Box::new(Self::from_name_and_structures(
                    inner, structures,
                )?)))
            },
            s if structures.contains_key(s) => {
                let fields = structures
                    .get(s)
                    .context(format!(
                        "Structure named '{}' isn't found among: {:?}",
                        s,
                        structures.keys().collect::<Vec<_>>()
                    ))?
                    .field_types
                    .iter()
                    .zip(
                        structures
                            .get(s)
                            .context(format!("Structure named '{}' isn't found", s))?
                            .is_mutable
                            .iter(),
                    )
                    .map(|(struct_name, is_mutable)| {
                        Ok((
                            struct_name.clone(),
                            (
                                Self::from_name_and_structures(struct_name, structures)?,
                                is_mutable.clone(),
                            ),
                        ))
                    })
                    .collect::<Result<HashMap<_, _>>>()?;
                Ok(Self::Structure(fields))
            },
            s => Ok(Self::Other(s.to_string())),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FieldValue {
    pub type_name: TypeName,
    pub value: RalphValue,
}

// This method of deserializing must be used when you know there will be no structures
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
        Ok(FieldValue { type_name, value })
    }
}

impl TryFrom<Value> for FieldValue {
    type Error = anyhow::Error;

    fn try_from(value: Value) -> Result<Self> {
        let type_name = value
            .get("type")
            .and_then(Value::as_str)
            .context("Missing 'type' field in FieldValue")?
            .try_into()?;
        let value = RalphValue::from_typename_and_value(&type_name, &value)
            .context("Failed to convert Value to RalphValue")?;
        Ok(FieldValue { type_name, value })
    }
}
