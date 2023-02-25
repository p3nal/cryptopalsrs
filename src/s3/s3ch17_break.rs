pub fn break_s3ch17() {
    let cipher = crate::s3::s3ch17::Cipher::new();
    let (ciphertext, iv) = crate::s3::s3ch17::first_function(&cipher);
    let mut plaintext: Vec<char> = Vec::new();
    let mut payload: Vec<u8> = vec![iv.to_vec(), ciphertext[..16].to_vec()]
        .into_iter()
        .flatten()
        .collect::<Vec<u8>>();
    // iv block block block
    //    inter inter inter
    //    plain plain plain
    for block_index in (0..ciphertext.len()).step_by(16) {
        let mut intermediate_block = vec![0; 16];
        let mut plain_block = vec![0 as char; 16];

        if block_index != 0 {
            payload = ciphertext[block_index - 16..block_index + 16].to_vec();
        }
        for byte_index in (0..16).rev() {
            let cipher_byte = if block_index == 0 {
                // iv
                iv[byte_index]
            } else {
                // minus 16 to fall back on ciphertext..
                ciphertext[block_index + byte_index - 16]
            };
            for byte_guess in 0..=255_u8 {
                // bruteforcing to find the byte which yeilds a valid padding
                payload[byte_index] = byte_guess;
                // if padding is valid
                if crate::s3::s3ch17::second_function(&cipher, &payload) {
                    // this next block here is to check for valid padding coincidences...
                    if byte_index > 0 {
                        payload[byte_index - 1] = 0xff_u8;
                        if !crate::s3::s3ch17::second_function(&cipher, &payload) {
                            // in this case weve fallen on an undexpected valid padding
                            // see https://crypto.stackexchange.com/questions/40800/is-the-padding-oracle-attack-deterministic
                            // for more info
                            continue;
                        }
                    }
                    // intermediate_byte = 0x01 ^ byte_guess
                    let intermediate_byte = 16 - byte_index as u8 ^ byte_guess;
                    intermediate_block[byte_index] = intermediate_byte;
                    let plain_byte;
                    // plain_byte = intermediate_byte ^ prevcipherbyte[byte_index]
                    plain_byte = intermediate_byte ^ cipher_byte;
                    // push found byte
                    plain_block[byte_index] = plain_byte as char;
                    // setting the last bytes to 0x02 or 0x03 0x03 or 0x04 0x04 0x04 etc...
                    break;
                }
            }
            for byte in byte_index..16 {
                // padding byte for next time
                let padding_byte = (16 - byte_index + 1) as u8;
                payload[byte] = padding_byte ^ intermediate_block[byte] as u8;
            }
        }
        plain_block.iter().for_each(|&x| plaintext.push(x));
    }
    println!("{:?}", plaintext.iter().collect::<String>());
}
