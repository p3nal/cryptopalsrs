pub struct Cipher {
    key: Vec<u8>,
}

impl Cipher {
    pub fn new() -> Cipher {
        Cipher {
            key: crate::s2::s2ch11::generate_random_bytes(16),
        }
    }
    /// client
    pub fn encrypt(&self, plaintext: &str) -> Vec<u8> {
        crate::s1::s1ch7::encrypt_aes_in_ecb(plaintext.as_bytes(), &self.key)
    }
    /// server
    pub fn decrypt(&self, plaintext: Vec<u8>) -> Vec<u8> {
        crate::s1::s1ch7::decrypt_aes_in_ecb(&plaintext, &self.key)
    }
}

/// client
pub fn profile_for(email: &str) -> String {
    format!("email={email}&uid=10&role=user")
}

/// server
pub fn kv_parsing_routine(input: &str) -> Vec<(&str, &str)> {
    input
        .trim()
        .splitn(3, '&')
        .map(|x| x.split_once('=').unwrap())
        .collect::<Vec<(&str, &str)>>()
}

/// server
pub fn check_admin(vec: Vec<(&str, &str)>) -> bool {
    vec.into_iter()
        .find(|x| x.0 == "role")
        .unwrap_or(("role", "not admin lol"))
        .1
        == "admin"
}

pub fn client/* <T: AsRef<[u8]>> */(cipher: &Cipher, email: &str) -> Vec<u8> {
    cipher.encrypt(&profile_for(email))
}

pub fn server(cipher: &Cipher, ciphertext: Vec<u8>) -> bool {
    check_admin(kv_parsing_routine(
        std::str::from_utf8(&cipher.decrypt(ciphertext)).unwrap(),
    ))
}
