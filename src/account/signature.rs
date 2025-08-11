use anyhow::{Context, Result, anyhow};
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

pub trait PrivateKey: Send + Sync {
    fn is_valid(hex_key: &str) -> bool
    where
        Self: Sized;
    fn as_hex(&self) -> String;
    fn get_public_key(&self) -> Result<String>;
    fn sign(&self, tx_id: &str) -> Result<String>;
}

pub struct GLSecp256k1PrivateKey {
    pub hex_key: String,
    pub key: SecretKey,
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

    fn sign(&self, tx_id: &str) -> Result<String> {
        let tx_id_bytes = hex::decode(tx_id)?;
        let message = Message::from_digest(
            tx_id_bytes
                .try_into()
                .map_err(|_| anyhow!("Invalid hash length"))?,
        );

        let secp = Secp256k1::new();
        let signature = secp.sign_ecdsa(&message, &self.key);
        let serialized = signature.serialize_compact();
        let signature = hex::encode(serialized);

        Ok(signature)
    }
}

impl GLSecp256k1PrivateKey {
    pub fn new(key: &str) -> Result<Self> {
        let private_key = SecretKey::from_slice(&hex::decode(key).context("Invalid hex key")?)
            .context("Failed to create secret key")?;

        Ok(GLSecp256k1PrivateKey {
            key: private_key,
            hex_key: key.to_string(),
        })
    }
}
