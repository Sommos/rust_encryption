mod algorithms;

fn main() {
    let user_input = get_user_input("Please enter your plaintext: ");
    let process = get_process();
    let algorithm_choice = get_algorithm_choice();
    
    if algorithm_choice == 1 {
        let shift_value = get_shift_value();

        if process == "ENCRYPT" {
            let encrypted_text = algorithms::caesar::caesar_cipher_encrypt(&user_input, shift_value);
            println!("Encrypted text: {}", encrypted_text);
        } else if process == "DECRYPT" {
            let decrypted_text = algorithms::caesar::caesar_cipher_decrypt(&user_input, shift_value);
            println!("Decrypted text: {}", decrypted_text);
        }
    } else if algorithm_choice == 2 {
        let mut key = get_user_input("Please enter your key: ");
        key = preprocess_key(&key);

        if key.is_empty() {
            panic!("Key cannot be empty or contain only non-alphabetic characters");
        }

        if process == "ENCRYPT" {
            let encrypted_text = algorithms::vigenere::vigenere_cipher_encrypt(&user_input, &key);
            println!("Encrypted text: {}", encrypted_text);
        } else if process == "DECRYPT" {
            let decrypted_text = algorithms::vigenere::vigenere_cipher_decrypt(&user_input, &key);
            println!("Decrypted text: {}", decrypted_text);
        }
    }
}

fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();

    println!("{}", prompt);
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

fn get_process() -> String {
    loop {
        println!("Select a process:\n1. Encrypt\n2. Decrypt\n3. Exit");
        let choice = get_user_input("");

        match choice.as_str() {
            "1" => return "ENCRYPT".to_string(),
            "2" => return "DECRYPT".to_string(),
            "3" => {
                println!("Exiting...");
                std::process::exit(0);
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_algorithm_choice() -> i32 {
    loop {
        println!("Select an algorithm:\n1. Caesar Cipher\n2. Vigenère Cipher\n3. Exit");
        let choice = get_user_input("");

        match choice.as_str() {
            "1" => return 1,
            "2" => return 2,
            "3" => {
                println!("Exiting...");
                std::process::exit(0);
            },
            _ => println!("Invalid choice, please try again."),
        }
    }
}

fn get_shift_value() -> i32 {
    loop {
        let shift_value = get_user_input("Please enter a shift value:");

        match shift_value.parse::<i32>() {
            Ok(shift) => return shift,
            Err(_) => println!("Invalid input, please enter an integer."),
        }
    }
}

fn preprocess_key(key: &str) -> String {
    key.chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}
