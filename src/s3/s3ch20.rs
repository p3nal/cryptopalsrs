use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub fn get_encrypted_stuff(path: &Path) {
    let file = File::open(path).expect("error opening file");
    let reader = BufReader::new(file);
    for line in reader.lines() {
    }
}

pub fn break_fixed_nonce_ctr(path: &Path) {
}
