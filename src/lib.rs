// src/lib.rs

/// Encrypts a message using the Caesar cipher
///
/// # Arguments
///
/// * `message` - Message to be encrypted
/// * `shift` - The number of positions to shift each character
///
/// # Examples
///
/// ```
/// use cipher_blend::encrypt;
/// let encrypted = encrypt("Hello, Rust!", 3);
/// assert_eq!(encrypted, "Khoor, Uxvw!");
/// ```
pub fn encrypt(message: &str, shift: u8) -> String {
    message
        .chars()
        .map(|c| {
            if c.is_ascii_alphabetic() {
                let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
                ((c as u8 - base + shift) % 26 + base) as char
            } else {
                c
            }
        })
        .collect()
}

/// Decrypts a message using the Caesar cipher
///
/// # Arguments
///
/// * `message` - The message to be decrypted
/// * `shift` - The number of positions to shift each character
///
/// # Examples
///
/// ```
/// use cipher_blend::decrypt;
/// let decrypted = decrypt("Khoor, Uxvw!", 3);
/// assert_eq!(decrypted, "Hello, Rust!");
/// ```
pub fn decrypt(message: &str, shift: u8) -> String {
    encrypt(message, 26 - shift) // Shift in the opposite direction
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt() {
        assert_eq!(encrypt("Hello, Rust!", 3), "Khoor, Uxvw!");
    }

    #[test]
    fn test_decrypt() {
        assert_eq!(decrypt("Khoor, Uxvw!", 3), "Hello, Rust!");
    }
}
