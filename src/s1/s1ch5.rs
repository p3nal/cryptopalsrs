// set 1 challenge 5
//
pub fn repeating_key_xor<T: AsRef<[u8]>>(message: T, key: T) -> Vec<u8> {
    // let key: Vec<char> = key.chars().cycle().take(message.len()).collect();
    let message = message.as_ref();
    let key: Vec<u8> = key.as_ref().iter().cloned().cycle().take(message.len()).collect();
    let cypher_bytes: Vec<u8> = message
        .iter()
        .zip(key.iter())
        .map(|(a, b)| (*a as u8) ^ (*b as u8))
        .collect();
    // let cyphertext = hex::encode(cypher_bytes);
    // cyphertext
    cypher_bytes
}
