use hex;
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn count_similar_blocks<T: AsRef<[u8]>>(line: T) -> usize {
    let mut count = 0;
    let chunks = line.as_ref().chunks(16);
    let length = chunks.len();
    for (i, chunk) in chunks.clone().enumerate() {
        for another_chunk in chunks.clone().rev().take(length - i) {
            if chunk.eq(another_chunk) {
                count += 1;
            }
        }
    }
    count
}

pub fn detects_aes_in_acb(path: &Path) -> (String, usize) {
    let file = File::open(path).expect("error opening file");
    let reader = BufReader::new(file);
    let mut count;
    let mut max_count = 0;
    let mut ecb_line = Vec::new();
    for line in reader.lines() {
        let line = hex::decode(line.expect("error reading line")).unwrap();
        count = count_similar_blocks(&line);
        if count > max_count {
            max_count = count;
            ecb_line = line;
        }
    }
    (hex::encode(ecb_line), max_count)
}
