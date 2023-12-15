use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;
use hex;

pub fn xor<T: AsRef<[u8]>, U: AsRef<[u8]>>(t1: T, t2: U) -> Vec<u8> {
    let t1 = t1.as_ref();
    let t2 = t2.as_ref();
    let length = t1.len().max(t2.len());
    let xor_bytes: Vec<u8> = t1
        .iter()
        .take(length)
        .zip(t2.iter().take(length))
        .map(|(b1, b2)| b1 ^ b2)
        .collect();
    xor_bytes
}

/// takes plaintext, 16 byte key, 8 bytes nonce
pub fn aes_ctr_crypt<T: AsRef<[u8]>, U: AsRef<[u8]>>(
    plaintext: T,
    key: U,
    nonce: u64,
) -> Vec<u8> {
    let plaintext = plaintext.as_ref();
    let key = GenericArray::clone_from_slice(key.as_ref());
    let cipher = Aes128::new(&key);
    plaintext.chunks(16).into_iter().enumerate().map(|(counter, chunk)| {
        let nonce = nonce.to_le_bytes();
        let counter = (counter as u64).to_le_bytes();
        let mut keystream = GenericArray::from_exact_iter(
            vec![nonce, counter]
                .into_iter()
                .flatten()
        ).unwrap();
        cipher.encrypt_block(&mut keystream);
        xor(keystream, chunk)
    }).flatten().collect()
}
