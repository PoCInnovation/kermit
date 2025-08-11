use std::convert::TryInto;

use i256::i256;

const SIGN_FLAG: u8 = 0x20;
const ONE_BYTE_BOUND: i32 = 0x20;
const TWO_BYTE_BOUND: i32 = ONE_BYTE_BOUND << 8;
const FOUR_BYTE_BOUND: i32 = ONE_BYTE_BOUND << (8 * 3);

const SINGLE_BYTE_PREFIX: u8 = 0x00;
const SINGLE_BYTE_NEG_PREFIX: u8 = 0x20;
const TWO_BYTE_PREFIX: u8 = 0x40;
const TWO_BYTE_NEG_PREFIX: u8 = 0x60;
const FOUR_BYTE_PREFIX: u8 = 0x80;
const FOUR_BYTE_NEG_PREFIX: u8 = 0xa0;
const MULTI_BYTE_PREFIX: u8 = 0xc0;


const MASK_REST: u8 = 0xc0;
const MASK_MODE: u8 = 0x3f;
const MASK_MODE_NEG: u32 = 0xffffffc0;

#[derive(Debug, Clone, Copy)]
pub enum ModeType {
    SingleByte,
    TwoByte,
    FourByte,
    MultiByte,
}

#[derive(Debug, Clone, Copy)]
pub struct Mode {
    pub mode_type: ModeType,
}

pub struct Signed;

impl Signed {
    pub fn encode_i32(value: i32) -> Vec<u8> {
        assert!(
            value >= i32::MIN && value < i32::MAX,
            "Invalid i32 value: {}",
            value
        );
        if value >= 0 {
            Self::encode_positive_i32(value)
        } else {
            Self::encode_negative_i32(value)
        }
    }

    pub fn encode_positive_i32(value: i32) -> Vec<u8> {
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

    pub fn encode_negative_i32(value: i32) -> Vec<u8> {
        if value >= -ONE_BYTE_BOUND {
            vec![((value as u8) ^ SINGLE_BYTE_NEG_PREFIX) & 0xff]
        } else if value >= -TWO_BYTE_BOUND {
            vec![
                (((value >> 8) as u8) ^ TWO_BYTE_NEG_PREFIX) & 0xff,
                (value & 0xff) as u8,
            ]
        } else if value >= -FOUR_BYTE_BOUND {
            vec![
                (((value >> 24) as u8) ^ FOUR_BYTE_NEG_PREFIX) & 0xff,
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

    pub fn encode_i256(value: i256) -> Vec<u8> {
        if value >= i256::from(-0x20000000) && value < i256::from(0x20000000) {
            Self::encode_i32(value.as_i32())
        } else {
            let bytes = BigIntCodec::encode(value);
            let header = ((bytes.len() as u8 - 4 + MULTI_BYTE_PREFIX) & 0xff) as u8;
            let mut result = vec![header];
            result.extend(bytes);
            result
        }
    }

    pub fn decode_int(mode: ModeType, body: &[u8]) -> i32 {
        let is_positive = (body[0] & SIGN_FLAG) == 0;
        if is_positive {
            Self::decode_positive_int(mode, body)
        } else {
            Self::decode_negative_int(mode, body)
        }
    }

    pub fn decode_positive_int(mode: ModeType, body: &[u8]) -> i32 {
        match mode {
            ModeType::SingleByte => body[0] as i32,
            ModeType::TwoByte => {
                assert!(body.len() == 2, "Length should be 2");
                (((body[0] & MASK_MODE) as i32) << 8) | ((body[1] & 0xff) as i32)
            },
            ModeType::FourByte => {
                assert!(body.len() == 4, "Length should be 4");
                (((body[0] & MASK_MODE) as i32) << 24)
                    | (((body[1] & 0xff) as i32) << 16)
                    | (((body[2] & 0xff) as i32) << 8)
                    | ((body[3] & 0xff) as i32)
            },
            _ => panic!("Invalid mode for decode_positive_int"),
        }
    }

    pub fn decode_negative_int(mode: ModeType, body: &[u8]) -> i32 {
        match mode {
            ModeType::SingleByte => (body[0] | MASK_MODE_NEG.into() as u8),
            ModeType::TwoByte => {
                assert!(body.len() == 2, "Length should be 2");
                (((body[0] | MASK_MODE_NEG) as i32) << 8) | ((body[1] & 0xff) as i32)
            },
            ModeType::FourByte => {
                assert!(body.len() == 4, "Length should be 4");
                (((body[0] | MASK_MODE_NEG) as i32) << 24)
                    | (((body[1] & 0xff) as i32) << 16)
                    | (((body[2] & 0xff) as i32) << 8)
                    | ((body[3] & 0xff) as i32)
            },
            _ => panic!("Invalid mode for decode_negative_int"),
        }
    }

    pub fn decode_i32(mode: ModeType, body: &[u8]) -> i32 {
        match mode {
            ModeType::SingleByte | ModeType::TwoByte | ModeType::FourByte => {
                Self::decode_int(mode, body)
            },
            ModeType::MultiByte => {
                if body.len() == 5 {
                    ((body[1] as i32) << 24)
                        | (((body[2] & 0xff) as i32) << 16)
                        | (((body[3] & 0xff) as i32) << 8)
                        | ((body[4] & 0xff) as i32)
                } else {
                    panic!("Expect 4 bytes int, but get {} bytes int", body.len() - 1);
                }
            },
        }
    }

    pub fn decode_i256(mode: ModeType, body: &[u8]) -> i256 {
        match mode {
            ModeType::SingleByte | ModeType::TwoByte | ModeType::FourByte => {
                Self::decode_i32(mode, body).into()
            },
            ModeType::MultiByte => {
                let bytes = &body[1..];
                assert!(bytes.len() <= 32, "Expect <= 32 bytes for I256");
                BigIntCodec::decode_signed(bytes)
            },
        }
    }
}

// Dummy BigIntCodec for demonstration
pub struct BigIntCodec;

impl BigIntCodec {
    pub fn encode(value: i256) -> Vec<u8> {
        // Special case for zero.
        if value == i256::from(0) {
            return vec![0];
        }

        let is_negative = value < i256::from(0);
        let mut abs_value = if is_negative { -value } else { value };
        let mut bytes = Vec::new();

        // Extract bytes from absolute value (little-endian).
        while abs_value > i256::from(0) {
            let byte = (abs_value & i256::from(0xff)).as_i32() as u8;
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

    pub fn decode_unsigned(encoded: &[u8]) -> i256 {
        // Special case for zero.
        if encoded.len() == 1 && encoded[0] == 0 {
            return i256::from(0);
        }

        let mut value = i256::from(0);
        for &byte in encoded {
            value = (value << 8) | i256::from(byte);
        }
        value
    }

    pub fn decode_signed(encoded: &[u8]) -> i256 {
        // Special case for zero.
        if encoded.len() == 1 && encoded[0] == 0 {
            return i256::from(0);
        }

        let is_negative = (encoded[0] & 0x80) != 0;
        let mut value = i256::from(0);
        for &byte in encoded {
            value = (value << 8) | i256::from(byte);
        }

        if is_negative {
            let bitlen = 8 * encoded.len();
            let mask = (i256::from(1) << bitlen) - i256::from(1);
            value = -((!value & mask) + i256::from(1));
        }

        value
    }
}
