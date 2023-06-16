use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use ring::hmac;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

fn print_help() {
    let help_message = r#"
Usage:
    totp-generator <secret_key> <password>

Parameters:
    secret_key   The TOTP secret key.
    password     The password to generate the TOTP code for.
"#;
    println!("{}", help_message);
}

fn generate_totp_code(secret_key: &str) -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let secret_key_bytes = base32::decode(base32::Alphabet::RFC4648 { padding: false }, secret_key)
        .expect("Failed to decode secret key");
    let key = hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &secret_key_bytes);
    let code = hmac::sign(&key, &(timestamp / 30).to_be_bytes());
    let ref_code = code.as_ref();
    // 19 is the last index of HMAC::SHA1 bytes array
    let offset = (ref_code[19] & 0x0F) as usize;
    let truncated_code: u32 = ((ref_code[offset] as u32 & 0x7f) << 24
        | (ref_code[offset + 1] as u32) << 16
        | (ref_code[offset + 2] as u32) << 8
        | (ref_code[offset + 3] as u32))
        % 1_000_000;
    truncated_code.to_string()
}

fn print_result(totp_code: String, whole_pwd: String) {
    println!("TOTP Code: {}", totp_code);
    println!("Whole pwd: {}", whole_pwd);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Welcome to TOTP Generator!");
        println!("Please provide both the TOTP secret key and password as command-line arguments.");
        print_help();
        return;
    }
    let secret_key = &args[1];
    let pwd = &args[2];
    let totp_code = generate_totp_code(secret_key);
    let whole_pwd = pwd.clone() + &totp_code;
    // Copy the TOTP code to the clipboard
    match ClipboardProvider::new() {
        Ok(ctx) => {
            let mut context: ClipboardContext = ctx;
            if let Err(err) = context.set_contents(whole_pwd.clone()) {
                eprintln!("Error copying to clipboard: {}", err);
                print_result(totp_code.clone(), whole_pwd.clone());
            }
        }
        Err(err) => {
            eprintln!("Error initializing clipboard: {}", err);
            print_result(totp_code.clone(), whole_pwd.clone());
        }
    }
    print_result(totp_code, whole_pwd);
}
