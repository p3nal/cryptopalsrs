// Byte-at-a-time ECB decryption (Simple)
// happy i made it here
use base64;

#[derive(Debug)]
pub struct Oracle {
    random_string: Vec<u8>,
    random_key: Vec<u8>,
}

impl Oracle {
    /// constructs new Oracle struct, takes in random key length and the random_string in base64
    pub fn new(length: usize, random_string: &str) -> Oracle {
        Oracle {
            random_string: base64::decode(random_string).unwrap().to_vec(),
            random_key: crate::s2::s2ch11::generate_random_bytes(length),
        }
    }

    /// encrypts text under a given key and appends a given string
    pub fn encryption_oracle<T: AsRef<[u8]>>(&self, text: T) -> Vec<u8> {
        let plaintext: Vec<u8> = vec![text.as_ref().to_vec(), self.random_string.clone()]
            .into_iter()
            .flatten()
            .collect();
        crate::s1::s1ch7::encrypt_aes_in_ecb(&plaintext, &self.random_key)
    }
}

pub fn discover_block_size(oracle: &Oracle) -> usize {
    let mut input = String::from("A");
    let mut prev_size = oracle.encryption_oracle(&input).len();
    let mut count = 0;
    for _ in 1..=2 {
        count = 0;
        loop {
            count += 1;
            input.push_str("A");
            let size = oracle.encryption_oracle(&input).len();
            if size > prev_size {
                prev_size = size;
                break;
            }
        }
    }
    count
}

pub fn detect_mode_operation(oracle: &Oracle, block_size: usize) -> crate::s2::s2ch11::Mode {
    let plaintext = (0..block_size * 2 + 1).map(|_| "A").collect::<String>();
    crate::s2::s2ch11::ebc_cbc_detection_oracle(oracle.encryption_oracle(plaintext), block_size)
}

pub fn crack_a_block(oracle: &Oracle, block_size: usize) -> Vec<u8> {
    let mut cracked_bytes: Vec<u8> = Vec::new();
    for byte in 0..oracle.encryption_oracle("").len() {
        let chunk_index = byte / block_size;
        let block: Vec<u8> = (0..(chunk_index + 1) * block_size - 1 - cracked_bytes.len())
            .map(|_| 65_u8)
            .collect::<Vec<u8>>();
        let cipherblocks_to_compare_with = oracle.encryption_oracle(&block)
            [chunk_index * block_size..(chunk_index + 1) * block_size]
            .to_vec();
        for i in 0..=255_u8 {
            if cipherblocks_to_compare_with
                == oracle.encryption_oracle(
                    vec![block.clone(), cracked_bytes.clone(), vec![i]]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<u8>>(),
                )[chunk_index * block_size..(chunk_index + 1) * block_size]
                    .to_vec()
            {
                cracked_bytes.push(i);
                break;
            }
        }
    }
    cracked_bytes
}

pub fn demo() {
    let random_string = "Um9sbGluJyBpbiBteSA1LjAKV2l0aCBteSByYWctdG9wIGRvd24gc28gbXkgaGFpciBjYW4gYmxvdwpUaGUgZ2lybGllcyBvbiBzdGFuZGJ5IHdhdmluZyBqdXN0IHRvIHNheSBoaQpEaWQgeW91IHN0b3A/IE5vLCBJIGp1c3QgZHJvdmUgYnkK";
    let oracle = Oracle::new(16, random_string);
    let block_size = discover_block_size(&oracle);
    println!("block size discovered = {block_size}");
    println!("mode = {}", detect_mode_operation(&oracle, block_size));
    println!("{}", String::from_utf8(crack_a_block(&oracle, block_size)).unwrap());
}
