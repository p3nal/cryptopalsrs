// set 4
//
// back to ctr.. where was it again?
//

use aes::cipher::{generic_array::GenericArray, BlockEncrypt, KeyInit};
use aes::Aes128;

use crate::s3::s3ch18::xor;

pub fn edit<T: AsRef<[u8]>, U: AsRef<[u8]>, V: AsRef<[u8]>>(
    ciphertext: T,
    key: U,
    nonce: u64,
    offset: usize,
    newtext: V,
) -> Vec<u8> {
    let ciphertext = ciphertext.as_ref();
    let key = GenericArray::clone_from_slice(key.as_ref());
    let cipher = Aes128::new(&key);
    let newtext = newtext.as_ref();
    let block_number = offset / 16;
    let length = newtext.len();
    println!("block number = {block_number}");
    let counter = block_number as u64;
    let newtext_keystream = ciphertext
        .get(block_number..block_number + (length / 16 + 1))
        .expect("cant edit; out of bounds...")
        .chunks(16)
        .enumerate()
        .map(|(i, _)| {
            let counter = (counter + i as u64).to_le_bytes();
            let nonce = nonce.to_le_bytes();
            // counter + nonce
            let mut keystream =
                GenericArray::from_exact_iter(vec![nonce, counter].into_iter().flatten()).unwrap();
            cipher.encrypt_block(&mut keystream);
            keystream
        })
        .flatten()
        .collect::<Vec<u8>>();
    let newtext_keystream = newtext_keystream.get(offset % 16..).unwrap();
    let newtext_ciphertext = xor(newtext_keystream, newtext);
    println!(
        "newtext_ciphertext len = {}, original newtext len = {}",
        newtext_ciphertext.len(),
        newtext.len()
    );
    vec![
        ciphertext.get(..offset).unwrap().to_vec(),
        newtext_ciphertext.to_vec(),
        ciphertext
            .get(offset + newtext_ciphertext.len()..)
            .unwrap()
            .to_vec(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
