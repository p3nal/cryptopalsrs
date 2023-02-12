use rand::Rng;

/// generates random bytes with length length
pub fn generate_random_bytes(length: usize) -> Vec<u8> {
    (0..length)
        .map(|_| rand::thread_rng().gen::<u8>())
        .collect()
}

/// function that encrypts under CBC or EBC randomly with a random key and a random iv
pub fn encryption_oracle<T: AsRef<[u8]>>(text: T) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let (left_padding, right_padding) = (
        generate_random_bytes(rng.gen_range(5..=10_usize)),
        generate_random_bytes(rng.gen_range(5..=10_usize)),
    );
    let plaintext: Vec<u8> = vec![left_padding, text.as_ref().to_vec(), right_padding]
        .into_iter()
        .flatten()
        .collect();
    let key = generate_random_bytes(16);
    let ciphertext = match rng.gen_bool(1.0 / 2.0) {
        true => {
            // for ecb
            // println!("ecb"); // for debugging
            crate::s1::s1ch7::encrypt_aes_in_ecb(plaintext, key)
        }
        false => {
            // for cbc
            // println!("cbc"); // for debugging
            let iv = generate_random_bytes(16);
            crate::s2::s2ch10::aes_cbc_encrypt(plaintext, key, iv)
        }
    };
    ciphertext
}

#[derive(Debug)]
pub enum Mode {
    ECB,
    CBC,
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Mode::ECB => write!(f, "ECB"),
            Mode::CBC => write!(f, "CBC"),
        }
    }
}

/// ebc cbc detection oracle, takes in ciphertext and block size
pub fn ebc_cbc_detection_oracle<T: AsRef<[u8]>>(ciphertext: T, block_size: usize) -> Mode {
    let ciphertext = ciphertext.as_ref().to_vec();
    let similar_blocks = crate::s1::s1ch8::count_similar_blocks(ciphertext, block_size);
    return if similar_blocks > 0 {
        Mode::ECB
    } else {
        Mode::CBC
    }
}
