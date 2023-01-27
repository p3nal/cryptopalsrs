use base64;
use hex;

// set 1 challenge 1
//
pub fn convert_hex_to_base64(hex: &str) -> String {
    base64::encode(hex::decode(hex).unwrap())
}
