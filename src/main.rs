mod algorithms;

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

    let mut user_process = String::new();

    loop {
        println!("\nSelect a process:");
        println!("1. Encrypt the plaintext");
        println!("2. Decrypt the ciphertext");
        println!("3. Exit\n");

        user_process.clear();
        if std::io::stdin().read_line(&mut user_process).is_err() {
            println!("Error reading selection. Please try again:\n");
            continue;
        }

        match user_process.trim() {
            "1" => {
                println!("You have chosen to encrypt the plaintext.\n");
                user_process = String::from("ENCRYPT");
                break;
            }
            "2" => {
                println!("You have chosen to decrypt the ciphertext.\n");
                user_process = String::from("DECRYPT");
                break;
            }
            "3" => {
                println!("Exiting program\n");
                return;
            }
            _ => {
                println!("Invalid selection. Please choose 1, 2 or 3.\n");
            }
        }
    }
    
    let mut user_algorithm = String::new();

    loop {
        println!("\nSelect a algorithm:");
        println!("1. Caesar Cipher");
        println!("2. Exit\n");

        user_algorithm.clear();
        if std::io::stdin().read_line(&mut user_algorithm).is_err() {
            println!("Error reading selection. Please try again:\n");
            continue;
        }

        match user_algorithm.trim() {
            "1" => {
                println!("You have chosen the Caesar Cipher\n");

                let user_shift_value: i32;

                loop {
                    println!("Please enter your shift value:\n");

                    let mut input = String::new();

                    match std::io::stdin().read_line(&mut input) {
                        Ok(_) => {
                            match input.trim().parse::<i32>() {
                                Ok(shift) => {
                                    user_shift_value = shift;
                                    break;
                                }
                                Err(_) => {
                                    println!("Error: Please enter a valid integer for the shift value.");
                                }
                            }
                        }
                        Err(e) => {
                            println!("Error: {}. Please try again.", e);
                        }
                    }
                }

                if user_process == "ENCRYPT" {
                    let user_encrypted_text = algorithms::caesar::caesar_cipher_encrypt(&user_input, user_shift_value);
                    println!("Your encrypted string: {}", user_encrypted_text)
                } else if user_process == "DECRYPT" {
                    let user_decrypted_text = algorithms::caesar::caesar_cipher_decrypt(&user_input, user_shift_value);
                    println!("Your decrypted string: {}", user_decrypted_text)
                } else {
                    println!("Process was unsuccessful. Exiting program");
                    return;
                }

                break;
            }
            "2" => {
                println!("Exiting program\n");
                return;
            }
            _ => {
                println!("Invalid selection. Please choose 1 or 2.\n");
            }
        }
    }
}
