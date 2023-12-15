use openssl::symm::{Cipher, decrypt, encrypt};
use std::path::Path;

pub fn decrypt_aes_in_ecb<T: AsRef<[u8]>, U: AsRef<[u8]>>(ciphertext: T, key: U) -> Vec<u8> {
    // let ciphertext: Vec<u8> = crate::s1::s1ch6::read_b64_file_contents(path);
    let ciphertext = ciphertext.as_ref().to_vec();

    let cipher = Cipher::aes_128_ecb();
    decrypt(cipher, key.as_ref(), None, &ciphertext).unwrap()
}

pub fn encrypt_aes_in_ecb<T: AsRef<[u8]>>(plaintext: T, key: T) -> Vec<u8> {
    // let ciphertext: Vec<u8> = crate::s1::s1ch6::read_b64_file_contents(path);
    let plaintext = plaintext.as_ref().to_vec();

    let cipher = Cipher::aes_128_ecb();
    encrypt(cipher, key.as_ref(), None, &plaintext).unwrap()
}
