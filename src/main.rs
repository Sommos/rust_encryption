mod caesar_cipher;

fn main() {
    let mut user_input = String::new();

    loop {
        println!("Please enter your plaintext:\n");

        user_input.clear();

        match std::io::stdin().read_line(&mut user_input) {
            Ok(_) => {
                break;
            }
            Err(e) => {
                println!("Error: {}. Please try again.", e);
            }
        }
    }

    println!("Your input: {}", user_input.trim());

    let mut user_process = String::new();

    loop {
        println!("\nSelect a process:");
        println!("1. Encrypt the plaintext");
        println!("2. Decrypt the ciphertext");
        println!("3. Exit\n");

        user_process.clear();
        if std::io::stdin().read_line(&mut user_process).is_err() {
            println!("Error reading selection. Please try again:");
            continue;
        }

        match user_process.trim() {
            "1" => {
                println!("\nYou have chosen to encrypt the plaintext.");
                user_process = String::from("ENCRYPT");
                break;
            }
            "2" => {
                println!("\nYou have chosen to decrypt the ciphertext.");
                user_process = String::from("DECRYPT");
                break;
            }
            "3" => {
                println!("\nExiting program");
                return;
            }
            _ => {
                println!("Invalid selection. Please choose 1, 2 or 3.");
            }
        }
    }

    println!("Your option chosen: {}", user_process.trim());

    if user_process == "ENCRYPT" {
        let user_encrypted_text = caesar_cipher::caesar_cipher_encrypt(&user_input, 1);
        println!("Your encrypted string: {}", user_encrypted_text)
    } else if user_process == "DECRYPT" {
        let user_decrypted_text = caesar_cipher::caesar_cipher_decrypt(&user_input, 1);
        println!("Your decrypted string: {}", user_decrypted_text)
    } else {
        println!("Process was unsuccessful. Exiting program");
        return;
    }
}
