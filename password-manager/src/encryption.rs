use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Cbc};
use rand::RngCore;

pub fn encrypt_password(password: &[u8; 32], key: &[u8; 32]) -> (Vec<u8>, [u8; 16]) {
    // Generate a new random initialization vector
    let mut iv = [0u8; 16];
    rand::thread_rng().fill_bytes(&mut iv);

    // Create a new AES-256 cipher in CBC mode with PKCS#7 padding
    let cipher = Cbc::<Aes256, Pkcs7>::new_var(&key, &iv).unwrap();

    // Encrypt the password using the cipher
    let ciphertext = cipher.encrypt_vec(&password);

    // Return the ciphertext and initialization vector
    (ciphertext, iv)
}
