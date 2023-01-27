use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

// set 1 challenge 4
pub fn detect_single_character_xor(path: &Path) {
    let file = File::open(path).expect("error opening file");
    let reader = BufReader::new(file);
    let mut found_text = String::new();
    let mut max_score = f64::MIN;
    for line in reader.lines() {
        let line = line.expect("error reading line");
        let (plaintext, score, _) = crate::s1::s1ch3::single_byte_xor_cypher(&line);
        if score > max_score {
            found_text = plaintext;
            max_score = score;
        }
    }
    println!("found text {} with score {}", found_text, max_score);
}
