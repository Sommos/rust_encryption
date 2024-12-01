fn main() {
    let mut user_plaintext = String::new();

    loop {
        println!("Please enter your plaintext:\n");

        user_plaintext.clear();

        match std::io::stdin().read_line(&mut user_plaintext) {
            Ok(_) => {
                break;
            }
            Err(e) => {
                println!("Error: {}. Please try again.", e);
            }
        }
    }

    println!("Your input: {}", user_plaintext.trim());

    let mut user_process_type = String::new();

    loop {
        println!("\nSelect a process:");
        println!("1. Encrypt the plaintext");
        println!("2. Decrypt the ciphertext");
        println!("3. Exit\n");

        user_process_type.clear();
        if std::io::stdin().read_line(&mut user_process_type).is_err() {
            println!("Error reading selection. Please try again:");
            continue;
        }

        match user_process_type.trim() {
            "1" => {
                println!("\nYou have chosen to encrypt the plaintext.");
                user_process_type = String::from("ENCRYPT");
                break;
            }
            "2" => {
                println!("\nYou have chosen to decrypt the ciphertext.");
                user_process_type = String::from("DECRYPT");
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

    println!("Your option chosen: {}", user_process_type.trim());
}
