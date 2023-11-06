use super::s3ch21::MT19937;

pub fn mt19937_stream_cipher<T: AsRef<[u8]>>(key: u16, plaintext: T) -> Vec<u8> {
    let plaintext = plaintext.as_ref();
    let mut mt = MT19937::new(key.into());
    plaintext
        .iter()
        .map(|c| c ^ mt.extract_number() as u8)
        .collect::<Vec<u8>>()
}
