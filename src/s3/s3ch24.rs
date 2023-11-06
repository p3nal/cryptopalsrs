use super::s3ch21::MT19937;

pub fn mt19937_stream_cipher<T: AsRef<[u8]>>(key: u16, input_text: T) -> Vec<u8> {
    let plaintext = input_text.as_ref();
    let mut mt = MT19937::new(key.into());
    let mut keystream = 0u32;
    plaintext
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let index = i % 4;
            if index == 0 {
                keystream = mt.extract_number();
            }
            c ^ keystream.wrapping_shr(i as u32 * 8) as u8
        })
        .collect::<Vec<u8>>()
}

/// cracks mt19937 key when given a known keystream which is the 14 last bytes... see cryptopals s3ch24
pub fn bruteforce_mt19937_key<T: AsRef<[u8]>>(ciphertext: &T, known_keystream: T) -> u16 {
    let ciphertext = ciphertext.as_ref();
    let known_keystream = known_keystream.as_ref();
    let ciphertext_len = ciphertext.len();
    for i in 0..=0xffffu16 {
        let plaintext = mt19937_stream_cipher(i, ciphertext);
        if plaintext[ciphertext_len-14..].eq(known_keystream) {
            return i
        }
    }
    0u16
}
