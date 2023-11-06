use super::s3ch21::MT19937;

pub fn mt19937_stream_cipher<T: AsRef<[u8]>>(key: u16, plaintext: T) -> Vec<u8> {
    let plaintext = plaintext.as_ref();
    let mut mt = MT19937::new(key.into());
    let mut keystream = 0u32;
    plaintext
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let index = i%4;
            if index == 0 {
                keystream = mt.extract_number();
            }
            c ^ keystream.wrapping_shr(i as u32*8) as u8
        })
        .collect::<Vec<u8>>()
}
