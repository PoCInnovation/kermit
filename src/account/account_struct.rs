use anyhow::Result;

use crate::account::{address::Address, signature::PrivateKey};

pub struct Account {
    pub private_key: Box<dyn PrivateKey>,
    pub address: Address,
    pub group: u8,
}

impl Account {
    pub fn new(private_key: Box<dyn PrivateKey>) -> Result<Self> {
        let public_key = private_key.get_public_key()?;
        let address = Address::new(&public_key)?;
        let group = address.group_from_bytes();

        Ok(Self {
            private_key,
            address,
            group,
        })
    }
}
