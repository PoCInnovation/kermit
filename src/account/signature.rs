use anyhow::{Context, Result};
use secp256k1::{PublicKey, Secp256k1, SecretKey};
use hex;

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
