// cbc... finally
pub struct Cipher {
    key: Vec<u8>,
    iv: Vec<u8>,
}

impl Cipher {
    pub fn new() -> Cipher {
        Cipher {
            key: crate::s2::s2ch11::generate_random_bytes(16),
            iv: crate::s2::s2ch11::generate_random_bytes(16),
        }
    }
    fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        crate::s2::s2ch10::aes_cbc_encrypt(plaintext.as_bytes(), &self.key, &self.iv)
    }
    fn decrypt(&self, ciphertext: Vec<u8>) -> Vec<u8> {
        crate::s2::s2ch10::aes_cbc_decrypt(&ciphertext, &self.key, &self.iv)
    }
}

pub fn first_function(cipher: &Cipher, input_str: String) -> Vec<u8> {
    let input_str = input_str.chars().filter(|x| *x != ';' && *x != '=').collect::<String>();
    let encoded_str = format!("comment1=cooking%20MCs;userdata={input_str};comment2=%20like%20a%20pound%20of%20bacon");
    cipher.encrypt(&encoded_str)
}

pub fn second_function(cipher: &Cipher, ciphertext: Vec<u8>) -> bool {
    String::from_utf8_lossy(&cipher.decrypt(ciphertext)).contains(";admin=true;")
}
