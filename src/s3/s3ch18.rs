use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;
use hex;

pub fn xor<T: AsRef<[u8]>>(t1: T, t2: T) -> Vec<u8> {
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
pub fn aes_ctr_crypt<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
    plaintext: T,
    key: U,
    nonce: V,
) -> Vec<u8> {
    let plaintext = plaintext.as_ref();
    let nonce = nonce.as_ref();
    let key = GenericArray::clone_from_slice(key.as_ref());
    let cipher = Aes128::new(&key);
    let mut counter = vec![0_u8; 8];
    let mut i = 0;
    let mut ciphertext = Vec::new();
    for plain_block in plaintext.chunks(16) {
        let mut keystream = GenericArray::from_exact_iter(
            vec![nonce.to_vec(), counter.to_vec()]
                .into_iter()
                .flatten()
        ).unwrap();
        if counter[i] == 0xff_u8 && i < counter.len() {
            i = i + 1;
        } else if i == counter.len() {
            counter = vec![0_u8; 8];
            i = 0;
        }
        counter[i] = counter[i] + 1_u8;
        // let mut keystream = GenericArray::clone_from_slice(&keystream);
        cipher.encrypt_block(&mut keystream);
        xor(plain_block, &keystream)
            .into_iter()
            .for_each(|x| ciphertext.push(x));
    }
    ciphertext
}
