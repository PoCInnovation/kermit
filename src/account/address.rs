use anyhow::{Context, Result, bail};

use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use bs58;

use crate::utils::crypto::{djb2, xor_byte};
use serde::{Deserialize, Serialize, Serializer};

const TOTAL_NUMBER_OF_GROUPS: u8 = 4;

#[repr(u8)]
#[allow(dead_code)]
pub enum AddressType {
    P2PKH = 0x00,
    P2MPKH = 0x01,
    P2SH = 0x02,
    P2C = 0x03,
    P2PK = 0x04,
    P2HMPK = 0x05,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Address {
    pub key: String,
    pub full_bytes: Vec<u8>,
    pub bytes: Vec<u8>,
}

impl Serialize for Address {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.key)
    }
}

impl<'de> Deserialize<'de> for Address {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let key = String::deserialize(deserializer)?;
        Address::new(&key).map_err(serde::de::Error::custom)
    }
}

impl Address {
    pub fn new(public_key_str: &str) -> Result<Self> {
        let public_key_bytes = hex::decode(public_key_str).context("Invalid hex")?;
        let mut hasher = Blake2bVar::new(32).context("Failed to create Blake2bVar")?;
        hasher.update(&public_key_bytes);
        let mut hash_bytes = [0u8; 32];
        hasher
            .finalize_variable(&mut hash_bytes)
            .context("Failed to finalize Blake2bVar")?;
        let hash_bytes = &hash_bytes[..32];

        let mut address_bytes = Vec::with_capacity(1 + 32);
        address_bytes.push(AddressType::P2PKH as u8);
        address_bytes.extend_from_slice(hash_bytes);

        Ok(Self {
            key: bs58::encode(&address_bytes).into_string(),
            full_bytes: address_bytes,
            bytes: hash_bytes.into(),
        })
    }

    pub fn new_b58(b58_str: &str) -> Result<Self> {
        let address_bytes = bs58::decode(b58_str)
            .into_vec()
            .context("Invalid base58 string")?;
        if address_bytes.len() < 33 {
            bail!("Invalid address length");
        }

        let key = b58_str.to_string();
        let full_bytes = address_bytes.clone();
        let bytes = address_bytes[1..].to_vec();

        Ok(Self {
            key,
            full_bytes,
            bytes,
        })
    }

    pub fn group_from_bytes(&self) -> u8 {
        let hint = djb2(&self.bytes) | 1;
        let hash = xor_byte(hint);
        hash % TOTAL_NUMBER_OF_GROUPS
    }
}
