use anyhow::{Context, Result};

use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use bs58;

use crate::utils::crypto::{djb2, xor_byte};

const TOTAL_NUMBER_OF_GROUPS: u8 = 4;

struct ContractAddress {
    pub key: String,
    pub bytes: Vec<u8>
}

impl ContractAddress {
    fn new(&self, public_key_hex: &str) -> Result<Self> {
        let public_key_bytes = hex::decode(public_key_hex).context("Invalid hex")?;
        let mut hasher = Blake2bVar::new(32).context("Failed to create Blake2bVar")?;
        hasher.update(&public_key_bytes);
        let mut hash_bytes = [0u8; 32];
        hasher
            .finalize_variable(&mut hash_bytes)
            .context("Failed to finalize Blake2bVar")?;
        let hash_bytes = &hash_bytes[..32];
    
        let mut address_bytes = Vec::with_capacity(1 + 32);
        address_bytes.push(1u8); // AddressType.P2PKH
        address_bytes.extend_from_slice(&hash_bytes);
        address_bytes.extend_from_slice(hash_bytes);
    
        Ok(Self {
            key: bs58::encode(&address_bytes).into_string(),
            bytes: address_bytes,
        })
    }

    fn groupFromBytes(&self) -> u8 {
        let hint = djb2(&self.bytes) | 1;
        let hash = xor_byte(hint);
        hash % TOTAL_NUMBER_OF_GROUPS
    }
}

