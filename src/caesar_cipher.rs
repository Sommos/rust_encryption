pub fn caesar_cipher_encrypt(input: &str, shift: i32) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            (((c as u8 - base + (shift as u8)) % 26) + base) as char
        } else {
            c
        }
    })
    .collect()
}

pub fn caesar_cipher_decrypt(input: &str, shift: i32) -> String {
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
            (((c as u8 - base + 26 - (shift as u8 % 26)) % 26) + base) as char
        } else {
            c
        }
    })
    .collect()
}