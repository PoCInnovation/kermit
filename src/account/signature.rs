use anyhow::{Context, Result};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use blake2::Blake2bVar;
use blake2::digest::{Update, VariableOutput};
use bs58;
use hex;

fn get_contract_address(public_key_hex: &str) -> Result<String> {
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

    Ok(bs58::encode(address_bytes).into_string())
}

pub trait PrivateKey {
    fn is_valid(hex_key: &str) -> bool;
    fn as_hex(&self) -> String;
    fn get_public_key(&self) -> Result<String>;
}

pub struct GLSecp256k1PrivateKey {
    hex_key: String,
    key: SecretKey,
}

impl PrivateKey for GLSecp256k1PrivateKey {
    fn is_valid(hex_key: &str) -> bool {
        hex_key.len() == 64 && hex_key.chars().all(|c| c.is_ascii_hexdigit())
    }

    fn as_hex(&self) -> String {
        self.hex_key.clone()
    }

    fn get_public_key(&self) -> Result<String> {
        let secp = Secp256k1::new();
        let public_key = PublicKey::from_secret_key(&secp, &self.key);
        Ok(public_key.to_string())
    }
}

impl GLSecp256k1PrivateKey {
    pub fn new(key: &str) -> Self {
        GLSecp256k1PrivateKey {
            key: SecretKey::from_slice(&hex::decode(key).expect("Invalid hex key"))
                .expect("Failed to create secret key"),
            hex_key: key.to_string(),
        }
    }
}
