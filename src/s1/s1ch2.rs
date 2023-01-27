use hex;
// set 1 challenge 2
//
pub fn fixed_or(buf1: &str, buf2: &str) -> Vec<u8> {
    // supposing they are the same length here
    let decoded_buf1 = hex::decode(buf1).unwrap();
    let decoded_buf2 = hex::decode(buf2).unwrap();
    // let decoded_buf2 =

    let xor_bytes: Vec<u8> = decoded_buf1
        .iter()
        .zip(decoded_buf2.iter())
        .map(|(b1, b2)| b1 ^ b2)
        .collect();

    // hex::encode(xor_bytes)
    xor_bytes
}
