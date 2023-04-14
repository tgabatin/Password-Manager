mod password;
mod encryption;

use std::io::{self, Write};

fn main() {
    // Prompt the user to enter their password
    print!("Enter your password: ");
    io::stdout().flush().unwrap();

    // Read the password from the user
    let mut password = String::new();
    io::stdin().read_line(&mut password).unwrap();

    // Normalize the password
    let normalized_password = password::normalize_password(password.trim());

    // Generate a random encryption key
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);

    // Encrypt the normalized password using the encryption key
    let (ciphertext, iv) = encryption::encrypt_password(&normalized_password, &key);

    // Print the ciphertext and initialization vector
    println!("Ciphertext: {:?}", ciphertext);
    println!("Initialization Vector: {:?}", iv);
}
