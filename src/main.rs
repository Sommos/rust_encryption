mod algorithms;
mod user_input;
mod gui;

fn main() -> Result<(), iced::Error> {
    // run the app from main function
    gui::gui_init()
}

// fn main() {
//     let user_input = user_input::get_user_input("Please enter your plaintext: ");
//     let process = user_input::get_process();
//     let algorithm_choice = user_input::get_algorithm_choice();

//     match algorithm_choice {
//         1 => handle_caesar_cipher(&user_input, &process),
//         2 => handle_vigenere_cipher(&user_input, &process),
//         _ => panic!("Invalid algorithm choice"),
//     }
// }

fn handle_caesar_cipher(user_input: &str, process: &str) {
    let shift_value = user_input::get_shift_value();

    let result = match process {
        "ENCRYPT" => algorithms::caesar::caesar_cipher_encrypt(user_input, shift_value),
        "DECRYPT" => algorithms::caesar::caesar_cipher_decrypt(user_input, shift_value),
        _ => panic!("Invalid process type"),
    };

    println!("Result: {}", result);
}

fn handle_vigenere_cipher(user_input: &str, process: &str) {
    let mut key = user_input::get_user_input("Please enter your key: ");
    key = user_input::preprocess_key(&key);

    if key.is_empty() {
        panic!("Key cannot be empty or contain only non-alphabetic characters");
    }

    let result = match process {
        "ENCRYPT" => algorithms::vigenere::vigenere_cipher_encrypt(user_input, &key),
        "DECRYPT" => algorithms::vigenere::vigenere_cipher_decrypt(user_input, &key),
        _ => panic!("Invalid process type"),
    };

    println!("Result: {}", result);
}