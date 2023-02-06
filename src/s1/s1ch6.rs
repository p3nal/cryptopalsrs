use base64;
use std::fs::read_to_string;
use std::path::Path;

// set 1 challenge 6
//
pub fn read_b64_file_contents(path: &Path) -> Vec<u8> {
    base64::decode(read_to_string(path).unwrap().replace("\n", "")).unwrap()
}

pub fn read_hex_file_contents(path: &Path) -> Vec<u8> {
    hex::decode(read_to_string(path).unwrap().replace("\n", "")).unwrap()
}

fn hamming_distance(bytes1: &[u8], bytes2: &[u8]) -> usize {
    if bytes1.len() != bytes2.len() {
        panic!("unequal number of bytes");
    }
    let ones_to_count: Vec<u8> = bytes1
        .iter()
        .zip(bytes2.iter())
        .map(|(a, b)| *a ^ *b)
        .collect();
    let mut count = 0;
    for b in ones_to_count {
        for i in 0..8 {
            if (b & (1 << i)) != 0 {
                count += 1;
            }
        }
    }
    count
}

fn calc_average_hamming_distance(keysize: &usize, contents: &Vec<u8>) -> f64 {
    // let mut keysize_array: Vec<(usize, usize)> = Vec::new();
    let chunks: Vec<&[u8]> = contents.chunks(*keysize).collect();
    let mut sum_of_distances = 0f64;
    let mut denominator = 0;
    for pair in chunks.chunks(2) {
        if pair.len() < 2 || pair[0].len() != pair[1].len() {
            break;
        }
        sum_of_distances += hamming_distance(pair[0], pair[1]) as f64 / *keysize as f64;
        denominator += 1;
    }
    sum_of_distances as f64 / denominator as f64
}

// speculates the keysize using the hamming distance
fn find_keysize(contents: &Vec<u8>) -> usize {
    let mut keysize_array: Vec<(usize, f64)> = Vec::new();
    for keysize in 2..=40 {
        let normalized_hamming_distance = calc_average_hamming_distance(&keysize, &contents);
        keysize_array.push((keysize, normalized_hamming_distance));
    }
    let (probably_the_keysize, min_normalized_hamming_distance) = keysize_array.iter().min_by(|x, y| x.1.partial_cmp(&y.1).unwrap()).unwrap();
    *probably_the_keysize
}

pub fn break_repeating_key_xor(path: &Path) -> (String, String) {
    let contents: Vec<u8> = read_b64_file_contents(path);

    // ok sure thing now i probably know the keysize... lets see
    let keysize: usize = find_keysize(&contents);
    // breaking cyphertext into blocks and doing crypto stuff
    let mut guessed_key: Vec<u8> = Vec::new();
    for i in 0..keysize {
        let block: Vec<u8> = contents
            .chunks(keysize)
            .map(|chunk| match chunk.get(i) {
                Some(x) => *x,
                None => 0,
            })
            .collect();
        guessed_key.push(crate::s1::s1ch3::single_byte_xor_cypher(&block).2 as u8);
    }
    let result = crate::s1::s1ch5::repeating_key_xor(&contents, &guessed_key);
    let (plaintext, key) = (String::from_utf8(result).unwrap(), String::from_utf8(guessed_key).unwrap());
    (plaintext, key)
}
