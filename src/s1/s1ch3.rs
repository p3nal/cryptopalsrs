// use base64;
// use hex;
use std::str;

// set 1 challenge 3

// ive tried different methods for calculating the similarity score between this
// and english text, the only one that works is by multiplicaiton of the english
// frequency with the observed frequency
pub fn score(text: &str) -> f64 {
    let english_freq = [
        0.08167, 0.01492, 0.02782, 0.04253, 0.12702, 0.02228, 0.02015, // A-G
        0.06094, 0.06966, 0.00153, 0.00772, 0.04025, 0.02406, 0.06749, // H-N
        0.07507, 0.01929, 0.00095, 0.05987, 0.06327, 0.09056, 0.02758, // O-U
        0.00978, 0.02360, 0.00150, 0.01974, 0.00074, 0.20000, // V-Z
    ];

    let mut counts = vec![0_u32; 27];
    let mut sc: f64 = 0_f64;

    text.chars().for_each(|c| match c {
        'a'..='z' => counts[c as usize - 97] += 1,
        'A'..='Z' => counts[c as usize - 65] += 1,
        ' ' => counts[26] += 1,
        _ => {},
    });

    for i in 0usize..27 {
        // sc += (counts[i] as f64 / text.len() as f64 - english_freq[i]).powi(2) / english_freq[i];
        sc += (counts[i] as f64 * english_freq[i]).powi(2)
    }
    // let degrees_of_freedom = (text.len() - 1) as f64; // number of tokens - 1
    // let chi_sq = ChiSquared::new(degrees_of_freedom).unwrap();
    // let pvalue = chi_sq.cdf(sc);
    // pvalue
    sc / (text.len() as f64)
    // sc
}


pub fn single_byte_xor_cypher<T: AsRef<[u8]>>(cypher: T) -> (String, f64, usize) {
    let mut score_array = Vec::new();
    let mut max_score = f64::MIN;
    let mut plaintext = String::new();
    let mut found_key = 0usize;

    for key in 0..256 {
        let xored_hex: Vec<u8> = cypher.as_ref().iter().map(|b| (*b as u8)^(key as u8)).collect();
        // analyzing frequency
        let text = String::from_utf8_lossy(&xored_hex);
        let score = score(&text);
        score_array.push(score);
        if score > max_score {
            max_score = score;
            plaintext = text.to_string();
            found_key = key;
        }
    }
    (plaintext, max_score, found_key)
}
