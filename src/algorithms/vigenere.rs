pub fn vigenere_cipher_encrypt(plain_text: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(plain_text);
    }

    let mut index = 0;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';

                index += 1;
                (first + (c as u8 + shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn vigenere_cipher_decrypt(ciphertext: &str, key: &str) -> String {
    let key: String = key.chars().filter(|&c| c.is_ascii_alphabetic()).collect();
    let key = key.to_ascii_lowercase();

    let key_len = key.len();
    if key_len == 0 {
        return String::from(ciphertext);
    }

    let mut index = 0;

    ciphertext
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let first = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                let shift = key.as_bytes()[index % key_len] - b'a';

                index += 1;
                (first + (c as u8 + 26 - shift - first) % 26) as char
            } else {
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vigenere_cipher_encrypt_alphabetic() {
        assert_eq!(vigenere_cipher_encrypt("This is a test for encryption using a vigenere cipher", "KEY"),"Dlgc mq k xccx dyv cxgpitrssl ewgxk y fmeorcbi astfov");
    }

    #[test]
    fn test_vigenere_cipher_encrypt_alphanumeric() {
        assert_eq!(vigenere_cipher_encrypt("Th1s 1s a t3st for 3ncrypt1on us1ng a v1genere c1ph3r", "KEY"),"Dl1q 1c e r3cx dyv 3lmvwzx1mx yq1xk y f1kcxipo g1nr3v");
    }

    #[test]
    fn test_vigenere_cipher_encrypt_special() {
        assert_eq!(vigenere_cipher_encrypt("Th1$ 1s£ a t3$t for ^3ncrypt1on u$1ng a v1g3nere c1ph(/3r", "LONGKEY"),"Ev1$ 1f£ g d3$x dzf ^3aibcne1ca a$1xk y g1u3akbi a1av(/3e");
    }

    #[test]
    fn test_vigenere_cipher_encrypt_long_key() {
        assert_eq!(vigenere_cipher_encrypt("This is a test for a long key vigenere encryption", "VERYVERYLONGKEY"),"Olzq dw r rpgg lyv y gsee fip tturtovc zrtpttkgzb");
    }

    #[test]
    fn test_vigenere_cipher_decrypt_alphabetic() {
        assert_eq!(vigenere_cipher_decrypt("Dlgc mq k xccx dyv bogpitrssl ewgxk y fmeorcbi astfov", "KEY"),"This is a test for decryption using a vigenere cipher");
    }

    #[test]
    fn test_vigenere_cipher_decrypt_alphanumeric() {
        assert_eq!(vigenere_cipher_decrypt("Dl1q 1c e r3cx dyv b3mvwzx1mx yq1xk y f1kcxipo g1nr3v", "KEY"),"Th1s 1s a t3st for d3crypt1on us1ng a v1genere c1ph3r");
    }

    #[test]
    fn test_vigenere_cipher_decrypt_special() {
        assert_eq!(vigenere_cipher_decrypt("Ev1$ 1f£ g d3$x dzf ^q3ibcne1ca a$1xk y g1u3akbi a1av(/3e", "LONGKEY"),"Th1$ 1s£ a t3$t for ^d3crypt1on u$1ng a v1g3nere c1ph(/3r");
    }

    #[test]
    fn test_vigenere_cipher_decrypt_long_key() {
        assert_eq!(vigenere_cipher_decrypt("Olzq dw r rpgg lyv y gsee fip tturtovc zrtpttkgzb", "VERYVERYLONGKEY"),"This is a test for a long key vigenere encryption");
    }

    #[test]
    fn test_vigenere_cipher_encrypt_and_decrypt() {
        let input = "Th1$ 1s£ a t3$t for ^3ncrypt1on u$1ng a v1g3nere c1ph(/3r!";
        let key = "MYSECRETKEY";
        let encrypted = vigenere_cipher_encrypt(input, key);
        let decrypted = vigenere_cipher_decrypt(&encrypted, key);

        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_vigenere_cipher_empty_key() {
        assert_eq!(vigenere_cipher_encrypt("This is a test for encryption with an empty key", ""), "This is a test for encryption with an empty key");
        assert_eq!(vigenere_cipher_decrypt("This is a test for decryption with an empty key", ""), "This is a test for decryption with an empty key");
    }
}