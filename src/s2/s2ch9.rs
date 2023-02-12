use hex;

pub fn pkcs_7_padding<T: AsRef<[u8]>>(text: T, block_length: usize) -> Vec<u8> {
    let mut text = text.as_ref().clone().to_vec();
    let len = text.len();
    let remainder = block_length - len % block_length;
    text.extend([remainder as u8].repeat(remainder));
    // text.append(&mut [remainder as u8].repeat(remainder));
    text
}
