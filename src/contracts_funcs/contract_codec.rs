use i256::{I256, U256};

const ONE_BYTE_BOUND: i32 = 0x40;
const TWO_BYTE_BOUND: i32 = ONE_BYTE_BOUND << 8;
const FOUR_BYTE_BOUND: i32 = ONE_BYTE_BOUND << (8 * 3);

const SINGLE_BYTE_PREFIX: u8 = 0x00;
const TWO_BYTE_PREFIX: u8 = 0x40;
const FOUR_BYTE_PREFIX: u8 = 0x80;
const MULTI_BYTE_PREFIX: u8 = 0xc0;

const SINGLE_BYTE_NEG_PREFIX: u8 = 0xc0;
const TWO_BYTE_NEG_PREFIX: u8 = 0x80;
const FOUR_BYTE_NEG_PREFIX: u8 = 0x40;

pub fn encode_i32(value: i32) -> Vec<u8> {
    if value >= 0 {
        encode_positive_i32(value)
    } else {
        encode_negative_i32(value)
    }
}

fn encode_positive_i32(value: i32) -> Vec<u8> {
    if value < ONE_BYTE_BOUND {
        vec![(SINGLE_BYTE_PREFIX + value as u8) & 0xff]
    } else if value < TWO_BYTE_BOUND {
        vec![
            (TWO_BYTE_PREFIX + ((value >> 8) as u8)) & 0xff,
            (value & 0xff) as u8,
        ]
    } else if value < FOUR_BYTE_BOUND {
        vec![
            (FOUR_BYTE_PREFIX + ((value >> 24) as u8)) & 0xff,
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        ]
    } else {
        vec![
            MULTI_BYTE_PREFIX,
            ((value >> 24) & 0xff) as u8,
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        ]
    }
}

fn encode_negative_i32(value: i32) -> Vec<u8> {
    if value >= -ONE_BYTE_BOUND {
        vec![((value ^ SINGLE_BYTE_NEG_PREFIX as i32) & 0xff) as u8]
    } else if value >= -TWO_BYTE_BOUND {
        vec![
            (((value >> 8) ^ TWO_BYTE_NEG_PREFIX as i32) & 0xff) as u8,
            (value & 0xff) as u8,
        ]
    } else if value >= -FOUR_BYTE_BOUND {
        vec![
            (((value >> 24) ^ FOUR_BYTE_NEG_PREFIX as i32) & 0xff) as u8,
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        ]
    } else {
        vec![
            MULTI_BYTE_PREFIX,
            ((value >> 24) & 0xff) as u8,
            ((value >> 16) & 0xff) as u8,
            ((value >> 8) & 0xff) as u8,
            (value & 0xff) as u8,
        ]
    }
}

///////////////////

pub struct BigIntCodec;

impl BigIntCodec {
    pub fn encode(value: I256) -> Vec<u8> {
        // Special case for zero.
        if value == I256::from(0) {
            return vec![0];
        }

        let is_negative = value < I256::from(0);
        let mut abs_value = if is_negative { -value } else { value };
        let mut bytes = Vec::new();

        // Extract bytes from absolute value (little-endian).
        while abs_value > I256::from(0) {
            let byte = (abs_value & I256::from(0xff)).as_i32() as u8;
            bytes.push(byte);
            abs_value = abs_value >> 8;
        }

        // If positive and MSB has high bit set, prefix with zero byte.
        if !is_negative && !bytes.is_empty() && (bytes[bytes.len() - 1] & 0x80) != 0 {
            bytes.push(0);
        }

        // If negative, compute two's complement.
        if is_negative {
            let mut carry = true;
            for b in &mut bytes {
                *b = !*b & 0xff;
                if carry {
                    if *b == 0xff {
                        *b = 0;
                    } else {
                        *b = b.wrapping_add(1);
                        carry = false;
                    }
                }
            }
            // If carry remains or MSB does not have sign bit, append 0xff.
            if carry || bytes.is_empty() || (bytes[bytes.len() - 1] & 0x80) == 0 {
                bytes.push(0xff);
            }
        }

        // Reverse to big-endian.
        bytes.reverse();
        bytes
    }
}

pub fn encode_i256(value: I256) -> Vec<u8> {
    if value >= I256::from(-0x20000000) && value < I256::from(0x20000000) {
        encode_i32(value.as_i32())
    } else {
        let bytes = BigIntCodec::encode(value);
        let header = ((bytes.len() as u8 - 4 + MULTI_BYTE_PREFIX) & 0xff) as u8;
        [vec![header], bytes].concat()
    }
}

pub fn encode_u256(value: U256) -> Vec<u8> {
    // let zero = U256::from(0u32);
    // let upper_bound = U256::from(1u32) << 256;
    let four_byte_bound = U256::from(FOUR_BYTE_BOUND as u32);

    if value < four_byte_bound {
        encode_i32(value.as_i32())
    } else {
        let mut bytes = BigIntCodec::encode(value.as_signed());
        if !bytes.is_empty() && bytes[0] == 0 {
            bytes.remove(0);
        }
        let header = ((bytes.len() as u8 - 4 + MULTI_BYTE_PREFIX) & 0xff) as u8;
        let mut result = Vec::with_capacity(1 + bytes.len());
        result.push(header);
        result.extend(bytes);
        result
    }
}
