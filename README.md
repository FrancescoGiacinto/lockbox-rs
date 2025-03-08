## Rust File Crypto

A simple Rust-based tool for encrypting and decrypting files using **AES-256-GCM** encryption. This project serves as a portfolio demonstration of file security techniques using Rust.

## Features
- AES-256-GCM encryption for high security.
- Automatic key generation and storage in `key.bin`.
- CLI-based interface for encrypting and decrypting files.
- Secure file handling using the `aes-gcm` Rust crate.

❗ Note: Antivirus Issues
Some antivirus software (Windows Defender, Avast, etc.) may block the build process and flag the compiled binary as a false positive.
If you encounter an error like "Access is denied (os error 5)", try the following:

Add an exclusion in your antivirus for the project folder.
Run the terminal as Administrator before building.
Disable real-time protection temporarily while building.

## 🛠 Installation

1️⃣ Install Rust
Ensure you have Rust installed. If not, install it via [Rustup](https://rustup.rs/):

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

2️⃣ Clone this repository

git clone https://github.com/YOUR_GITHUB_USERNAME/rust-file-crypto.git

cd rust-file-crypto

3️⃣ Install dependencies

cargo build

## Usage

# Encrypt a file

cargo run -- --encrypt input.txt -o output.enc

This encrypts input.txt and saves the encrypted data to output.enc.
A random AES-256 key is generated once and stored in key.bin.

# Decrypt a file

cargo run -- --decrypt output.enc -o decrypted.txt


Example

echo "Hello, Rust Encryption!" > input.txt
cargo run -- --encrypt input.txt -o output.enc
cargo run -- --decrypt output.enc -o decrypted.txt
cat decrypted.txt

✅ Output:


Hello, Rust Encryption!


## Key Management
The encryption key is stored in key.bin.
The same key is used for both encryption and decryption.
If key.bin is lost, encrypted files cannot be decrypted! 🛑


🏗 Project Structure

📂 rust-file-crypto
 ┣ 📂 src
 ┃ ┣ 📜 main.rs        # Main Rust program
 ┣ 📜 Cargo.toml       # Dependencies & metadata
 ┣ 📜 README.md        # Project documentation
 ┣ 📜 key.bin          # AES-256 encryption key (generated after first run)
 ┣ 📜 .gitignore       # Ignore unnecessary files
 ┗ 📜 #all files will be create inside the main folder

📜 License
This project is licensed under the MIT License.
You are free to use, modify, and distribute this code. See LICENSE for details.

👨‍💻 Author
Created by Francesco Giacinto | [GitHub](https://github.com/FrancescoGiacinto)
If you found this useful, give it a ⭐ on GitHub! 🚀

🛡️ Security Disclaimer

⚠️ This tool is for educational purposes only!

Do not use it to encrypt sensitive data in production.

