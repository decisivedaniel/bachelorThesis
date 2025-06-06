use std::fs;
use std::io;
use std::str;

fn main() {
    let mut white_noise = String::new();
    let row_white_noise = "WWWWWWWWBBBBBBBBWWWWBBBBWWBBBWWR";
    let message = String::from("Reading Dilbert strips or encoding Elbonian messages are not good excuses for failing the XBC009 final exam.");

    let response = user_selection();

    create_wn_pattern(&mut white_noise, row_white_noise);

    if response == 0 {
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

fn user_selection() -> u64 {
    println!("enter 0 for encryption, enter 1 for decryption");
    let mut buf = String::new();
    match io::stdin().read_line(&mut buf) {
        Ok(_) => {
            return buf.parse::<u64>().unwrap_or(0);
        }
        Err(error) => println!("Error: {error}")
    };
    return 0;
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
    for message_char in message.chars() {
        if last_char == message_char && count < 9 {
            count += 1;
        } else {
            encoded.push((count + 48) as char);
            encoded.push(last_char);
            count = 1;
        }
        last_char = message_char;
    }
    return encoded;
}

fn run_length_decode(encoded_message: &str) -> String {
    let mut decoded = String::new();
    let mut encoded_msg_iterator = encoded_message.chars().peekable();
    while encoded_msg_iterator.peek().is_some() {
        let amount = encoded_msg_iterator.next().unwrap().to_digit(10).unwrap();
        let letter = encoded_msg_iterator.next().unwrap();
        for _ in 0..amount {
            decoded.push(letter);
        }
    }
    return decoded;
}

fn create_wn_pattern(white_noise: &mut String, row_white_noise: &str){
    for _ in 0..32 {
        white_noise.push_str(row_white_noise);
    }
}

fn create_image_header(bmp : &mut Vec<u8>) {
    // signature "BM"
    bmp.push('B' as u8);
    bmp.push('M' as u8);

    // file size 14 + 40 + 1024 = 1078 or 010000110110
    // 0000 0100 0011 0110 or 0x0436
    bmp.append(&mut vec![0x36,0x04,0x00,0x00]);

    // reserved field (four bytes hex)
    // 00 00 00 00
    bmp.append(&mut vec![0x00,0x00,0x00,0x00]);

    // offset pixel data (four bytes int)
    // 54 or 36 00 00 00
    bmp.append(&mut vec![0x36,0x00,0x00,0x00]);
    
    // Bitmap Header
    // header: 40 or 28 00 00 00
    // width: 32 or 20 00 00 00
    // height: 32 or 20 00 00 00
    // reserved: 01 00
    // bits per pixel: 24 or 18 00
    // compression: 00 00 00 00
    // size of pixel data: 1024 or 00 04 00 00
    // horizontal resolution: 2835 or 13 0B 00 00
    // vertical resolution: 2835 or 13 0B 00 00
    // color palette: 00 00 00 00
    // important colors: 00 00 00 00
    bmp.append(&mut vec![
        0x28, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00,
        0x20, 0x00, 0x00, 0x00,
        0x01, 0x00,
        0x18, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x04, 0x00, 0x00,
        0x13, 0x0B, 0x00, 0x00,
        0x13, 0x0B, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00
    ]);
    
}

fn create_image(message: &str, white_noise: &str){
    //println!("whiteNoise {}", white_noise);
    let mut bmp: Vec<u8> = Vec::new();

    create_image_header(&mut bmp);
    let mut message_chars = message.chars();
    // Pixel Data
    for current in white_noise.char_indices() {
        let mut pixel_value: Vec<u8> = vec![0,0,0];
        if current.1 == 'W' {
            pixel_value[0] = 0xff;
            pixel_value[1] = 0xff;
        } else {
            pixel_value[0] = 0x00;
            pixel_value[1] = 0x00;
        }
        if current.1 != 'B' {
            pixel_value[2] = 0xff;
        } else {
            pixel_value[2] = 0x00;
        }
        if current.0 < message.len() {
            let current_letter = message_chars.next().unwrap();
            let mut divide = current_letter as u8 / 16;
            let mut mod_value = current_letter as u8 % 16; 
            let mut flag = 0;
            if divide >= 8 {
                divide -= 8;
                flag += 2;
            }
            if mod_value >= 8 {
                mod_value -= 8;
                flag += 1;
            }
            pixel_value[0] ^= divide;
            pixel_value[1] ^= mod_value;
            pixel_value[2] ^= flag;
        }
        bmp.append(&mut pixel_value);
    }
    
    _ = fs::write("output.bmp", bmp);
}

fn read_image(encrypted_white_noise: &mut String, white_noise: &str) {
    let image = fs::read("output.bmp").unwrap();
    let mut image_index = 54;
    for current in white_noise.chars() {
        let mut divide: u8 = 0;
        let mut mod_value: u8 = 0;
        let mut flag: u8 = 0;
        let pixel = vec![image[image_index], image[image_index+1], image[image_index+2]];
        image_index += 3;
        if current == 'W' {
            divide = 0xff;
            mod_value = 0xff;
        }
        if current != 'B' {
            flag = 0xff;
        }
        divide ^= pixel[0];
        mod_value ^= pixel[1];
        flag ^= pixel[2];
        if flag > 1 {
            divide += 8;
            flag -= 2;
        }
        if flag > 0 {
            mod_value += 8;
        }
        divide *= 16;
        divide += mod_value;
        encrypted_white_noise.push(divide as char);
    }
}

