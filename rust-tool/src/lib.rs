use std::error::Error;
use std::str;
use colored::*;

pub struct Config {
    pub imei: String,
    pub mac: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            return Err("No IMEI or MAC");
        }
        let imei: String = args[1].clone();
        let mac: String = args[2].clone();
        Ok(Config { imei, mac })
    }
}

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let imei: &str = config.imei.as_str();
    let test_pass: String = test_password(imei);
    println!("Test password: {}", test_pass.green());
    let operator_pass: String = operator_pass(imei);
    println!("Operator password is: {}", operator_pass.green());
    let mac: &str = config.mac.as_str();
    let processed_mac = format_mac_address(mac);
    let user_pass = user_pass(&processed_mac, false, true);
    println!("User password is: {}",user_pass.green());
    Ok(())
}

fn operator_pass(imei: &str) -> String {
    if imei.chars().count() < 15 {
        return "IMEI too short".red().to_string();
    }
    let imei_chars: Vec<u32> = imei.chars().map(|c| c as u32).collect();
    let imei_length = imei_chars.len();
    let mut password = ['\0'; 8];

    for i in 0..8 {
        let mut hash: i64 = 1;

        for index in 0..imei_length {
            if hash > 0xFFFFFF {
                hash = (!hash) & 0xFFFFFF;
            }

            let char_index = (index + i) % imei_length;
            let imei_char = imei_chars[char_index];
            let multiplier = (((i + 1) * (index + 1)) & 0xFF) as i64;

            hash += (imei_char as i64) * multiplier;
        }

        if hash > 0xFFFFFF {
            hash = (!hash) & 0xFFFFFF;
        }

        let hash_mod = (hash % 0x34) as u32;
        let mut char_code: u32;

        if hash_mod < 0x24 {
            if hash_mod < 10 {
                char_code = hash_mod + 0x30;
            } else {
                char_code = hash_mod + 0x37;
            }
        } else {
            char_code = hash_mod + 0x3D;
        }
        if matches!(char_code, 0x31 | 0x49 | 0x69 | 0x4C | 0x6C) {
            char_code += 1;
        }

        password[i] = std::char::from_u32(char_code).unwrap_or('?');
    }

    password.iter().collect()
}

fn test_password(imei: &str) -> String {
    if imei.chars().count() < 15 {
        return "IMEI too short".red().to_string();
    }
    let slice: &str = &imei[7..];
    let mut pass: [char; 8] = ['0'; 8];
    let mut starter: i32 = 0;
    for (index, ch) in slice.chars().enumerate() {
        if index >= 8 {
            break;
        }
        if let Some(element) = ch.to_digit(10) {
            starter += index as i32 + element as i32;
            pass[index] = char::from_digit((starter % 10) as u32, 10).unwrap();
        } else {
            return "Invalid IMEI".red().to_string();
        }
    }
    pass.iter().collect()
}

fn user_pass(input: &[u8], param3: bool, param4: bool) -> String {
    let input_len = input.len();
    let mut result = Vec::with_capacity(8);
    for iteration in 0..8 {
        let mut accumulator: u32 = 1;
        for position in 0..input_len {
            if accumulator > 0xFFFFFF {
                accumulator = (!accumulator) & 0xFFFFFF;
            }
            let index = (position + iteration) % input_len;
            let multiplier = ((iteration + 1) * (position + 1)) & 0xFF;
            accumulator = accumulator.wrapping_add(input[index] as u32 * multiplier as u32);
        }
        if accumulator > 0xFFFFFF {
            accumulator = (!accumulator) & 0xFFFFFF;
        }
        let char_byte = if !param3 {
            let mod_val = (accumulator % 0x34) as u8;
            let mut char_code = match mod_val {
                0..=9 => mod_val + b'0',
                10..=35 => mod_val + 0x37,
                _ => mod_val + b'=',
            };
            if param4 {
                if matches!(char_code, 0x31 | 0x49 | 0x69 | 0x4C | 0x6C) {
                    char_code += 1;
                }
            }
            char_code
        } else {
            ((accumulator % 10) as u8) + b'0'
        };
        result.push(char_byte);
    }
    String::from_utf8(result).unwrap_or_else(|_| "INVALID MAC".red().to_string())
}

fn format_mac_address(mac: &str) -> Vec<u8> {
    let clean_mac: String = mac.chars().filter(|c| c.is_ascii_hexdigit()).collect();
    let mut formatted = Vec::new();
    for (i, chunk) in clean_mac.as_bytes().chunks(2).enumerate() {
        if i > 0 {
            formatted.push(b':');
        }
        formatted.extend_from_slice(chunk);
    }
    formatted
}
