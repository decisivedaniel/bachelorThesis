use std::io;

fn main() {
    let mut white_noise = String::new();
    let row_white_noise = "WWWWWWWWBBBBBBBBWWWWBBBBWWBBBWW";
    let message = String::from("Reading Dilbert strips or encoding Elbonian messages are not good excuses for failing the XBC009 final exam.");

    let response = user_selection();

    create_wn_pattern(&mut white_noise, row_white_noise);

    if response {
        encrypt(white_noise.as_str(), message.as_str());
    } else {
        decrypt(white_noise.as_str());
    }
}

fn encrypt(white_noise: &str, message: &str) {
    let packed_message = get_message(message);
    let encoded_message = run_length_encode(&packed_message);
    create_image(&encoded_message, white_noise);
}

fn decrypt(white_noise: &str) {
    let mut encrypted_white_noise = String::new();
    read_image(&mut encrypted_white_noise, white_noise);
    let decoded_message = run_length_decode(&encrypted_white_noise);
    let trimmed_message = retrieve_message(&decoded_message);
    println!("Message {trimmed_message}");
}

fn user_selection() -> bool {
    println!("enter 0 for encryption, enter 2 for decryption\n");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            return buf.parse::<bool>().unwrap_or_default();
        }
        Err(error) => println!("Error: {error}")
    };
    return true;
}

fn get_message (message: &str) -> String {
    let lead = "XXXXXXXXBBBBCCCCOOOO000099999999";
    let package = lead.to_string() + message + lead;
    return package;
} 

fn retrieve_message(message: &str) -> String {
    let lead = "XXXXXXXXBBBBCCCCOOOO000099999999";
    return message.strip_prefix(lead).unwrap()
        .strip_suffix(lead).unwrap().to_string();
}

fn run_length_encode(message: &str) -> String {
    let mut encoded = String::new();
    let mut last_char = message.chars().nth(0).unwrap();
    let mut count : u8 = 0;
    for message_char in message.char_indices() {
        if last_char == message_char.1 && count < 9 {
            count += 1;
        } else {
            encoded.push((count + 48) as char);
            encoded.push(last_char);
            count = 1;
        }
        last_char = message_char.1;
    }
    return encoded;
}

fn run_length_decode(encoded_message: &str) -> String {
    
}

fn create_wn_pattern(white_noise: &mut String, row_white_noise: &str){

}

fn create_image(encryptedWhiteNoise: &str, whiteNoise: &str){

}

fn read_image(encryptedWhiteNoise: &str, whiteNoise: &str) {

}

