pub fn caesar_cipher_encrypt(plain_text: &str, shift: i32) -> String {
    let normalised_shift = ((shift % 26) + 26) % 26;

    plain_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + normalised_shift as u8) % 26) + base) as char
            } else {
                c
            }
        })
        .collect()
}

pub fn caesar_cipher_decrypt(cipher_text: &str, shift: i32) -> String {
    let normalised_shift = ((shift % 26) + 26) % 26;

    cipher_text
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                (((c as u8 - base + 26 - (normalised_shift as u8)) % 26) + base) as char
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
    fn test_caesar_cipher_encrypt_alphabetic() {
        assert_eq!(caesar_cipher_encrypt("This is a test for encryption using a caesar cipher", 6), "Znoy oy g zkyz lux ktixevzout ayotm g igkygx iovnkx");
    }

    #[test]
    fn test_caesar_cipher_encrypt_alphanumeric() {
        assert_eq!(caesar_cipher_encrypt("Th1s 1s a t3st for 3ncrypt1on us1ng a ca3sar c1ph3r", 4), "Xl1w 1w e x3wx jsv 3rgvctx1sr yw1rk e ge3wev g1tl3v");
    }

    #[test]
    fn test_caesar_cipher_encrypt_special() {
        assert_eq!(caesar_cipher_encrypt("Th1$ 1s£ a t3$t for ^3ncrypt1on u$1ng a c**a3$ar c1ph(/3r", 15), "Iw1$ 1h£ p i3$i udg ^3crgnei1dc j$1cv p r**p3$pg r1ew(/3g");
    }

    #[test]
    fn test_caesar_cipher_encrypt_large_shift() {
        assert_eq!(caesar_cipher_encrypt("This is a test for a large shift value using caesar cipher encryption", 148), "Lzak ak s lwkl xgj s dsjyw kzaxl nsdmw mkafy uswksj uahzwj wfujqhlagf");
    }

    #[test]
    fn test_caesar_cipher_encrypt_negative_shift() {
        assert_eq!(caesar_cipher_encrypt("This is a test for a negative shift value using caesar cipher encryption", -17), "Cqrb rb j cnbc oxa j wnpjcren bqroc ejudn dbrwp ljnbja lryqna nwlahycrxw");
    }

    #[test]
    fn test_caesar_cipher_decrypt_alphabetic() {
        assert_eq!(caesar_cipher_decrypt("Estd td l epde qzc opncjaetzy fdtyr l nlpdlc ntaspc", 11), "This is a test for decryption using a caesar cipher");
    }

    #[test]
    fn test_caesar_cipher_decrypt_alphanumeric() {
        assert_eq!(caesar_cipher_decrypt("Ao1z 1z h a3za mvy k3jyfwa1vu bz1un h jh3zhy j1wo3y", 7), "Th1s 1s a t3st for d3crypt1on us1ng a ca3sar c1ph3r");
    }

    #[test]
    fn test_caesar_cipher_decrypt_special() {
        assert_eq!(caesar_cipher_decrypt("Vj1$ 1u£ c v3$v hqt ^f3etarv1qp w$1pi c e**c3$ct e1rj(/3t", 2), "Th1$ 1s£ a t3$t for ^d3crypt1on u$1ng a c**a3$ar c1ph(/3r");
    }

    #[test]
    fn test_caesar_cipher_decrypt_large_shift() {
        assert_eq!(caesar_cipher_decrypt("Vjku ku c vguv hqt c nctig ujkhv xcnwg wukpi ecguct ekrjgt fgetarvkqp", 366), "This is a test for a large shift value using caesar cipher decryption");
    }

    #[test]
    fn test_caesar_cipher_decrypt_negative_shift() {
        assert_eq!(caesar_cipher_decrypt("Guvf vf n grfg sbe n artngvir fuvsg inyhr hfvat pnrfne pvcure qrpelcgvba", -39), "This is a test for a negative shift value using caesar cipher decryption");
    }

    #[test]
    fn test_caesar_cipher_encrypt_and_decrypt() {
        let input = "Th1$ 1s£ a t3$t for ^3ncrypt1on u$1ng a c**a3$ar c1ph(/3r!";
        let shift = -999;
        let encrypted = caesar_cipher_encrypt(input, shift);
        let decrypted = caesar_cipher_decrypt(&encrypted, shift);

        assert_eq!(decrypted, input);
    }

    #[test]
    fn test_caesar_cipher_shift_zero() {
        let input = "This is a test for no shift";
        let shift = 0;
        assert_eq!(caesar_cipher_encrypt(input, shift), input);
        assert_eq!(caesar_cipher_decrypt(input, shift), input);
    }
}