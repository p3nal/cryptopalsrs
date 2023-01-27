use openssl::symm::{Cipher, decrypt};
use std::path::Path;

pub fn decrypt_aes_in_ecb(path: &Path) -> Vec<u8> {
    let ciphertext: Vec<u8> = crate::s1::s1ch6::read_file_contents(path);
    let key = "YELLOW SUBMARINE";

    let cipher = Cipher::aes_128_ecb();
    let plaintext = decrypt(cipher, key.as_bytes(), None, &ciphertext).unwrap();
    plaintext
}
