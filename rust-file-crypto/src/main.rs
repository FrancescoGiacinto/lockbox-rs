use aes_gcm::aead::{Aead, KeyInit}; // Importing necessary AES-GCM traits
use aes_gcm::{Aes256Gcm, Key, Nonce}; // Import AES-256-GCM cipher
use std::fs;
use std::io::{Read, Write};
use clap::{Arg, Command}; // CLI argument parsing
use rand::RngCore;
use rand::rngs::OsRng; // Import OsRng for generating a random key

// Save the key to a file
fn save_key_to_file(key: &[u8], path: &str) {
    fs::write(path, key).expect("Failed to save key");
}


// Generate a random key and save it
fn get_or_generate_key(path: &str) -> Key<Aes256Gcm> {
    if let Ok(key_bytes) = fs::read(path) {
        println!("🔑 Loaded existing key from '{}'", path);
        Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
    } else {
        let mut key_bytes = [0u8; 32];
        OsRng.fill_bytes(&mut key_bytes);
        save_key_to_file(&key_bytes, path);
        println!("🔑 Generated new encryption key and saved to '{}'", path);
        Key::<Aes256Gcm>::from_slice(&key_bytes).clone()
    }
}

/// Encrypts a file using AES-256-GCM
fn encrypt_file(input_path: &str, output_path: &str, key: &Key<Aes256Gcm>) {
    let nonce = Nonce::from_slice(&[0u8; 12]); // 12-byte fixed nonce (should be randomized in production)
    let cipher = Aes256Gcm::new(key);

    // Read the file contents
    let data = fs::read(input_path).expect("Error reading input file");

    // Encrypt the data
    let encrypted_data = cipher.encrypt(nonce, data.as_ref())
        .expect("Encryption failed");

    // Write nonce and encrypted data to the output file
    let mut output_file = fs::File::create(output_path)
        .expect("Error creating output file");
    output_file.write_all(&nonce).unwrap();
    output_file.write_all(&encrypted_data).unwrap();

    println!("✅ File encrypted and saved as '{}'", output_path);
}

/// Decrypts a file using AES-256-GCM
fn decrypt_file(input_path: &str, output_path: &str, key: &Key<Aes256Gcm>) {
    // Open the encrypted file
    let mut file = fs::File::open(input_path).expect("Error opening input file");
    let mut file_content = Vec::new();
    file.read_to_end(&mut file_content).expect("Error reading file");

    // Split nonce and encrypted data
    let (nonce, encrypted_data) = file_content.split_at(12);
    let cipher = Aes256Gcm::new(key);

    // Decrypt the data
    let decrypted_data = cipher.decrypt(Nonce::from_slice(nonce), encrypted_data)
        .expect("Decryption failed");

    // Write decrypted data to the output file
    fs::write(output_path, &decrypted_data)
        .expect("Error writing decrypted file");

    println!("✅ File decrypted and saved as '{}'", output_path);
}

/// Command-line interface setup
fn main() {
    let matches = Command::new("Rust File Crypto")
        .version("1.0")
        .author("Your Name")
        .about("Encrypts and decrypts files using AES-256-GCM")
        .arg(Arg::new("encrypt")
            .short('e')
            .long("encrypt")
            .value_name("FILE")
            .help("File to encrypt")
            .num_args(1))  // Replaces takes_value(true)
        .arg(Arg::new("decrypt")
            .short('d')
            .long("decrypt")
            .value_name("FILE")
            .help("File to decrypt")
            .num_args(1))  // Replaces takes_value(true)
        .arg(Arg::new("output")
            .short('o')
            .long("output")
            .value_name("FILE")
            .help("Output file")
            .num_args(1))  // Replaces takes_value(true)
        .get_matches();

    // 🔑 Use a persistent key
    let key = get_or_generate_key("key.bin"); // Save or load key from file

    if let Some(input) = matches.get_one::<String>("encrypt").map(String::as_str) {
        if let Some(output) = matches.get_one::<String>("output").map(String::as_str) {
            encrypt_file(input, output, &key);
        } else {
            println!("⚠️ Please specify an output file using -o");
        }
    } else if let Some(input) = matches.get_one::<String>("decrypt").map(String::as_str) {
        if let Some(output) = matches.get_one::<String>("output").map(String::as_str) {
            decrypt_file(input, output, &key);
        } else {
            println!("⚠️ Please specify an output file using -o");
        }
    } else {
        println!("⚠️ Use --encrypt or --decrypt to execute the program.");
    }
}
