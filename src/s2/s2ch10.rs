use aes::Aes128;
use aes::cipher::{
    BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};

fn xor<T: AsRef<[u8]>>(b1: T, b2: T) -> Vec<u8> {
    let xor_bytes: Vec<u8> = b1
        .as_ref()
        .iter()
        .zip(b2.as_ref().iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();
    xor_bytes
}

pub fn aes_cbc_encrypt<T: AsRef<[u8]>>(plaintext: T, key: T, iv: T) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(key.as_ref());
    let cipher = Aes128::new(&key);
    let padded_plaintext = crate::s2::s2ch9::pkcs_7_padding(plaintext, 16);
    let mut ciphertext: Vec<Vec<u8>> = Vec::new();
    for block in padded_plaintext.chunks(16) {
        let block = block.to_vec();
        let xor_block = xor(block, ciphertext.last().unwrap_or(&iv.as_ref().to_vec()).to_vec());
        let mut encrypted_block = GenericArray::clone_from_slice(&xor_block);
        // let encrypted_block = &encrypt(cipher, key.as_ref(), None, &xor_block).unwrap()[0..16];
        cipher.encrypt_block(&mut encrypted_block);
        ciphertext.push(encrypted_block.to_vec());
    }
    ciphertext.into_iter().flatten().collect()
}

pub fn aes_cbc_decrypt<T: AsRef<[u8]>>(ciphertext: T, key: T, iv: T) -> Vec<u8> {
    let ciphertext = ciphertext.as_ref();
    // let ciphertext = hex::decode(ciphertext.as_ref()).unwrap().to_vec();
    let key = GenericArray::clone_from_slice(key.as_ref());
    let cipher = Aes128::new(&key);
    let mut plaintext: Vec<Vec<u8>> = Vec::new();
    for i in (0..ciphertext.len()).step_by(16) {
        let prev_cipherblock = if i==0 {
            &iv.as_ref()
        } else {
            &ciphertext[i-16..i]
        };
        let mut block = GenericArray::clone_from_slice(&ciphertext[i..i+16]);
        cipher.decrypt_block(&mut block);
        let block_to_xor = block.to_vec();
        let xored_block = xor(block_to_xor, prev_cipherblock.to_vec());
        plaintext.push(xored_block);
    }
    let plaintext: Vec<u8> = plaintext.into_iter().flatten().collect();
    // plaintext.get(..plaintext.len() - *plaintext.last().unwrap_or(&0) as usize).unwrap().to_vec()
    // padding not removed
    plaintext
}
