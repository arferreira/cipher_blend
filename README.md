# Cipher Blend

[![Build Status](https://travis-ci.org/your_username/cipher_blend.svg?branch=master)](https://travis-ci.org/your_username/cipher_blend)
[![Crates.io](https://img.shields.io/crates/v/cipher_blend.svg)](https://crates.io/crates/cipher_blend)
[![License](https://img.shields.io/crates/l/cipher_blend.svg)](https://opensource.org/licenses/MIT)

Cipher Blend is a Rust library for simple Caesar cipher encryption and decryption.

## Features

- Encrypt messages using the Caesar cipher algorithm.
- Decrypt messages using the Caesar cipher algorithm.

## Installation

Add the following line to your `Cargo.toml` file:

```toml
[dependencies]
cipher_blend = "0.1.0"
```

## Usage

```rust
use cipher_blend::{encrypt, decrypt};

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
```

## Contributing

If you find any issues or have suggestions for improvement, feel free to open an issue or submit a pull request.

## License

This crate is distributed under the terms of the [MIT License](https://opensource.org/licenses/MIT).
