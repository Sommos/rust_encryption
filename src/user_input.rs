use std::io;

pub fn get_user_input(prompt: &str) -> String {
    let mut input = String::new();
    println!("{}", prompt);
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

pub fn get_process() -> String {
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

pub fn get_algorithm_choice() -> i32 {
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

pub fn get_shift_value() -> i32 {
    loop {
        let shift_value = get_user_input("Please enter a shift value:");

        match shift_value.parse::<i32>() {
            Ok(shift) => return shift,
            Err(_) => println!("Invalid input, please enter an integer."),
        }
    }
}

pub fn preprocess_key(key: &str) -> String {
    key.chars()
        .filter(|c| c.is_alphabetic())
        .collect()
}