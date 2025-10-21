use anyhow::{Context, Result};
use i256::{I256, U256};

use crate::contracts_funcs::contract_codec::{encode_i32, encode_i256, encode_u256};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VmValType {
    Bool = 0,
    I256 = 1,
    U256 = 2,
    ByteVec = 3,
    Address = 4,
}

fn encode_vmbyte(t: VmValType, bytes: Vec<u8>) -> Vec<u8> {
    [vec![t as u8], bytes].concat()
}

pub fn encode_vmbyte_bool(b: bool) -> Vec<u8> {
    encode_vmbyte(VmValType::Bool, vec![b as u8])
}

pub fn encode_vmbyte_i256(n: I256) -> Vec<u8> {
    let bytes = encode_i256(n);
    encode_vmbyte(VmValType::I256, bytes)
}

pub fn encode_vmbyte_u256(n: U256) -> Vec<u8> {
    let bytes = encode_u256(n);
    encode_vmbyte(VmValType::U256, bytes)
}

// TODO: add group addresses support
pub fn encode_vmbyte_address(s: &str) -> Result<Vec<u8>> {
    let decoded = bs58::decode(s)
        .into_vec()
        .context("Failed to decode base58 address")?;

    Ok(encode_vmbyte(VmValType::Address, decoded))
}

pub fn encode_vmbyte_vec(hex_bytes: &[u8]) -> Vec<u8> {
    let size = encode_i32(hex_bytes.len() as i32);

    let final_bytes = [size, hex_bytes.to_vec()].concat();
    encode_vmbyte(VmValType::ByteVec, final_bytes)
}
