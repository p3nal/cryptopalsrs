use base64;
use rand::Rng;

#[derive(Debug)]
pub struct Oracle {
    random_prepended_string: Vec<u8>,
    random_appended_string: Vec<u8>,
    random_key: Vec<u8>,
}

impl Oracle {
    /// constructs new Oracle struct, takes in random key length and the random_appended_string in base64
    pub fn new(length: usize, random_appended_string: &str) -> Oracle {
        Oracle {
            random_prepended_string: crate::s2::s2ch11::generate_random_bytes(
                rand::thread_rng().gen_range(4..40),
            ),
            random_appended_string: base64::decode(random_appended_string).unwrap().to_vec(),
            random_key: crate::s2::s2ch11::generate_random_bytes(length),
        }
    }

    /// encrypts text under a given key and appends a given string
    pub fn encryption_oracle<T: AsRef<[u8]>>(&self, text: T) -> Vec<u8> {
        let plaintext: Vec<u8> = vec![
            self.random_prepended_string.clone(),
            text.as_ref().to_vec(),
            self.random_appended_string.clone(),
        ]
        .into_iter()
        .flatten()
        .collect();
        crate::s1::s1ch7::encrypt_aes_in_ecb(&plaintext, &self.random_key)
    }
}

// pub fn how_long_til_block(oracle: &Oracle, block_size: usize) -> usize {
//     let mut input = (0..block_size).map(|_| "A").collect::<String>();
//     let mut prev_size = oracle.encryption_oracle(&input).len();
//     let mut count = 0;
//     loop {
//         count += 1;
//         input.push_str("A");
//         let size = oracle.encryption_oracle(&input).len();
//         if size > prev_size {
//             break;
//         }
//     }
//     count % block_size
// }

pub fn number_of_prepended_blocks(oracle: &Oracle, block_size: usize) -> usize {
    let empty_cipher = oracle.encryption_oracle("");
    let some_cipher = oracle.encryption_oracle("A");
    empty_cipher
        .chunks(block_size)
        .zip(some_cipher.chunks(block_size))
        .enumerate()
        .find(|(_, (x, y))| x != y)
        .unwrap()
        .0
}

pub fn till_block(oracle: &Oracle, number_of_prepended_blocks: usize, block_size: usize) -> usize {
    let ref_aaa = (0..block_size + 1).map(|_| "A").collect::<String>();
    let mut aaa = String::from("A");
    let reference = oracle.encryption_oracle(ref_aaa);
    let mut count = 1;
    loop {
        if oracle.encryption_oracle(&aaa)
            [number_of_prepended_blocks * block_size..(number_of_prepended_blocks + 1) * block_size]
            == reference[number_of_prepended_blocks * block_size
                ..(number_of_prepended_blocks + 1) * block_size]
        {
            return count;
        }
        count += 1;
        aaa.push('A');
    }
}

/// dont mind the name it actually cracks the whole thing not just one block,
/// i just like how it sounds lol.. crakablok
pub fn crack_a_block(oracle: &Oracle, block_size: usize) -> Vec<u8> {
    let mut cracked_bytes: Vec<u8> = Vec::new();
    let number_of_prepended_blocks = number_of_prepended_blocks(oracle, block_size);
    let till_block = till_block(oracle, number_of_prepended_blocks, block_size);
    let size_zero = oracle.encryption_oracle("").len();
    let size_of_target_cipher =
        size_zero - number_of_prepended_blocks * block_size - (block_size - till_block); // including
                                                                                         // padding
    for byte in 0..size_of_target_cipher {
        // couldve done for i in 16 for number of appended blocks instead
        let chunk_index = byte / block_size;
        let block: Vec<u8> =
            (0..till_block + (chunk_index + 1) * block_size - 1 - cracked_bytes.len()) // |prep&AAA|AAAAaaaX|
                .map(|_| 65_u8)
                .collect::<Vec<u8>>();
        let cipherblocks_to_compare_with = oracle.encryption_oracle(&block)
            [(number_of_prepended_blocks + 1 + chunk_index) * block_size
                ..(number_of_prepended_blocks + 1 + chunk_index + 1) * block_size]
            // [chunk_index * block_size..(chunk_index + 1) * block_size]
            .to_vec();
        for i in 0..=255_u8 {
            if cipherblocks_to_compare_with
                == oracle.encryption_oracle(
                    vec![block.clone(), cracked_bytes.clone(), vec![i]]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<u8>>(),
                    // )[chunk_index * block_size..(chunk_index + 1) * block_size]
                )[(number_of_prepended_blocks + 1 + chunk_index) * block_size
                    ..(number_of_prepended_blocks + 1 + chunk_index + 1) * block_size]
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
    let block_size = 16;
    let number_of_prepended_blocks = number_of_prepended_blocks(&oracle, block_size);
    let prepended_text_size = number_of_prepended_blocks * block_size
        + till_block(&oracle, number_of_prepended_blocks, block_size);
    println!(
        "{}",
        String::from_utf8(crack_a_block(&oracle, block_size)).unwrap()
    );
}
