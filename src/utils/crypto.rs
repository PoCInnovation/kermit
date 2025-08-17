pub fn djb2(bytes: &[u8]) -> i32 {
    let mut hash: i32 = 5381;
    for &byte in bytes {
        hash = ((hash << 5)
            .wrapping_add(hash)
            .wrapping_add((byte & 0xff) as i32)) as i32;
    }
    hash
}

pub fn xor_byte(int_value: i32) -> u8 {
    let byte0 = ((int_value >> 24) & 0xff) as u8;
    let byte1 = ((int_value >> 16) & 0xff) as u8;
    let byte2 = ((int_value >> 8) & 0xff) as u8;
    let byte3 = (int_value & 0xff) as u8;
    (byte0 ^ byte1 ^ byte2 ^ byte3) & 0xff
}

pub fn is_hex_string(input: &str) -> bool {
    input.starts_with("0x")
        && (input.len() - 2) % 2 == 0
        && input[2..].chars().all(|c| c.is_ascii_hexdigit())
}

pub fn is_b58(input: &str) -> bool {
    input.starts_with("b58:")
        && input[4..].chars().all(|c| {
            c.is_ascii_alphanumeric()
                && "123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz".contains(c)
        })
}
