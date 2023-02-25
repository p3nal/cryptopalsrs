// The CBC padding oracle
// This is the best-known attack on modern block-cipher cryptography.
// ok
//
use base64;
use rand::Rng;

pub struct Cipher {
    selected_string: Vec<u8>,
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Cipher {
    pub fn new() -> Cipher {
        let list =
"MDAwMDAwTm93IHRoYXQgdGhlIHBhcnR5IGlzIGp1bXBpbmc=
MDAwMDAxV2l0aCB0aGUgYmFzcyBraWNrZWQgaW4gYW5kIHRoZSBWZWdhJ3MgYXJlIHB1bXBpbic=
MDAwMDAyUXVpY2sgdG8gdGhlIHBvaW50LCB0byB0aGUgcG9pbnQsIG5vIGZha2luZw==
MDAwMDAzQ29va2luZyBNQydzIGxpa2UgYSBwb3VuZCBvZiBiYWNvbg==
MDAwMDA0QnVybmluZyAnZW0sIGlmIHlvdSBhaW4ndCBxdWljayBhbmQgbmltYmxl
MDAwMDA1SSBnbyBjcmF6eSB3aGVuIEkgaGVhciBhIGN5bWJhbA==
MDAwMDA2QW5kIGEgaGlnaCBoYXQgd2l0aCBhIHNvdXBlZCB1cCB0ZW1wbw==
MDAwMDA3SSdtIG9uIGEgcm9sbCwgaXQncyB0aW1lIHRvIGdvIHNvbG8=
MDAwMDA4b2xsaW4nIGluIG15IGZpdmUgcG9pbnQgb2g=
MDAwMDA5aXRoIG15IHJhZy10b3AgZG93biBzbyBteSBoYWlyIGNhbiBibG93"
            .split('\n')
            .collect::<Vec<&str>>();
        Cipher {
            selected_string: base64::decode(list[rand::thread_rng().gen_range(0..list.len())]).unwrap(),
            key: crate::s2::s2ch11::generate_random_bytes(16),
            iv: crate::s2::s2ch11::generate_random_bytes(16),
        }
    }
}

/// returns ciphertext and IV
pub fn first_function(cipher: &Cipher) -> (Vec<u8>, &Vec<u8>) {
    (
        crate::s2::s2ch10::aes_cbc_encrypt(
            &cipher.selected_string,
            &cipher.key,
            &cipher.iv,
        ),
        &cipher.iv,
    )
}

/// a decryption function that side-channel leaks
pub fn second_function<T: AsRef<[u8]>>(cipher: &Cipher, ciphertext: T) -> bool {
    let plaintext = crate::s2::s2ch10::aes_cbc_decrypt(
        ciphertext.as_ref(),
        &cipher.key,
        &cipher.iv,
    );
    match crate::s2::s2ch15::validate_padding(plaintext) {
        Ok(_) => true,
        Err(_) => false,
    }
}
