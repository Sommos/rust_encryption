pub fn vigenere_cipher_encrypt(plaintext: &str, key: &str) -> String {
    if key.is_empty() {
        panic!("Key cannot be empty");
    }

    let key = key.chars().cycle();
    let mut encrypted_text = String::new();

    for (p, k) in plaintext.chars().zip(key) {
        if p.is_alphabetic() {
            let base = if p.is_lowercase() { 'a' } else { 'A' };
            let p_offset = p.to_ascii_lowercase() as u8 - base as u8;
            let k_offset = k.to_ascii_lowercase() as u8 - base as u8;
            let encrypted_char = ((p_offset + k_offset) % 26) + base as u8;
            
            encrypted_text.push(encrypted_char as char);
        } else {
            encrypted_text.push(p);
        }
    }

    encrypted_text
}

pub fn vigenere_cipher_decrypt(ciphertext: &str, key: &str) -> String {
    if key.is_empty() {
        panic!("Key cannot be empty");
    }

    let key = key.chars().cycle();
    let mut decrypted_text = String::new();

    for (c, k) in ciphertext.chars().zip(key) {
        if c.is_alphabetic() {
            let base = if c.is_lowercase() { 'a' } else { 'A' };
            let c_offset = c.to_ascii_lowercase() as u8 - base as u8;
            let k_offset = k.to_ascii_lowercase() as u8 - base as u8;
            let decrypted_char = ((c_offset + 26 - k_offset) % 26) + base as u8;

            decrypted_text.push(decrypted_char as char);
        } else {
            decrypted_text.push(c);
        }
    }

    decrypted_text
}