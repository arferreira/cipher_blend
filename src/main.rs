use cipher_blend::{decrypt, encrypt};

fn main() {
    let original_message = "Hello, Rust!";
    let shift = 3;

    // Encrypt
    let encrypted_message = encrypt(original_message, shift);
    println!("Encrypted: {}", encrypted_message);

    // Decrypt
    let decrypted_message = decrypt(&encrypted_message, shift);
    println!("Decrypted: {}", decrypted_message);
}
