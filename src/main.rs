mod s1;
mod s2;
mod s3;
use hex;
use std::path::Path;

use crate::s2::s2ch10::xor;

fn main() {
    // every preceding challenge has been commented out lol
    // let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
    // s1::s1ch3::single_byte_xor_cypher(input);
    // let path = Path::new("./src/s1/4.txt");
    // s1::s1ch4::detect_single_character_xor(path);
    // let input = "Burning 'em, if you ain't quick and nimble\nI go crazy when I hear a cymbal";
    //     let input = r#"this is my story, this is my song
    // And to them rudypoots don't attempt to try this at home
    // It's just a poem until y'all learn right from wrong
    // Know when to bless a situation, when to grab the chrome
    // But it's back on, another stormy night in Atlanta, Georgia
    // Overcast, but on behalf of OutKast, I cordially
    // Invite you to an emotion-filled theater
    // Bring your umbrellas, 'cause young fella it gets no weirder
    // "#;
    //     let key = "erifyknohc";
    //     println!("cyphertext: {}", s1::s1ch5::repeating_key_xor(input, key));
    // let str1 = "this is a test\n\n";
    // let str2 = "wokka wokka!!!\n\n";
    // let str1b: String= str1.as_bytes().iter().map(|b| format!("{:b}", *b)).collect();
    // let str2b: String= str2.as_bytes().iter().map(|b| format!("{:b}", *b)).collect();
    // println!("{:?}\n{:?}", str1b, str2b);
    // let path = Path::new("./src/s1/6.txt");
    // let (text, key) = s1::s1ch6::break_repeating_key_xor(path);
    // println!("broken xor with key = \"{}\" and contents:\n{}", key, text);
    //
    // let path = Path::new("./src/s1/7.txt");
    // println!("plaintext: \n{}", String::from_utf8(s1::s1ch7::decrypt_aes_in_ecb(path)).unwrap());
    //
    // let path = Path::new("./src/s1/8.txt");
    // let (ecb, count) = s1::s1ch8::detects_aes_in_ecb(path);
    // println!("ecb line {} with count {}", ecb, count);
    // s2::s2ch9::pkcs_7_padding("YELLOW SUBMARINE", 20);
    // let plaintext = "AAAAAAAAAAAAAAAAAAAAAA";
    // let key = r#"YELLOW SUBMARINE"#;
    // let iv = r#"asdfasdfasdfasdf"#;
    // let ciphertext = "f612270f052b7c86151ee99d3674cbeab6d27ffa97bd1db9218c480b987d0fdaee2a5a49b3727e655bae78362e5c93244757ab41b0953b65146b416cbd32f1f0";
    // let ciphertext = hex::decode(ciphertext).unwrap();
    // let ciphertext = s2::s2ch10::aes_cbc_encrypt(plaintext, key, iv);
    // let plaintext = s2::s2ch10::aes_cbc_decrypt(ciphertext, key.as_bytes().to_vec(), iv.as_bytes().to_vec());
    // println!("encoded ciphertext = {}", hex::encode(ciphertext));
    // println!("{:?}", ciphertext);
    // println!("plaintext {}", String::from_utf8_lossy(&plaintext));
    // println!("{}", hex::encode(s2::s2ch11::generate_random_aes_key(0,16)));
    // let cipher = s2::s2ch11::encryption_oracle(plaintext);
    // println!("{}\n{:#?}", hex::encode(&cipher), s2::s2ch11::ebc_cbc_detection_oracle(cipher));
    // let cipher = s2::s2ch12::encryption_oracle(plaintext.as_bytes().to_vec());
    // println!("{}\n{:#?}", hex::encode(&cipher), s2::s2ch11::ebc_cbc_detection_oracle(cipher));

    // s2::s2ch12::demo();
    //
    // s2::s2ch13_break::break_s2ch13();
    //
    // s2::s2ch14::demo();
    //
    // PKCS#7 padding validation:
    // let string1 = String::from("ICE ICE BABY\x04\x04\x04\x04");
    // let string2 = String::from("ICE ICE BABY\x05\x05\x05\x05");
    // let string3 = String::from("ICE ICE BABY\x01\x02\x03\x04");
    // println!("unpadded string:  {}", s2::s2ch15::validate_padding(string3));
    //
    // CBC bitflipping attacks
    // s2::s2ch16_break::break_s2ch16();
    //
    // set 3 strting...
    // s3::s3ch17_break::break_s3ch17();
    //
    // ch18 CTR here we go
    // let key = "YELLOW SUBMARINE";
    // let nonce = vec![0_u8; 8];
    // let ciphertext =
    //     base64::decode("L77na/nrFsKvynd6HzOoG7GHTLXsTVu9qvY/2syLXzhPweyyMTJULu/6/kXX0KSvoOLSFQ==")
    //         .unwrap();
    // println!(
    //     "plaintext: {}",
    //     String::from_utf8(s3::s3ch18::aes_ctr_crypt(
    //         ciphertext,
    //         key.as_bytes().to_vec(),
    //         nonce
    //     )).unwrap()
    // );
    // ch19
    // s3::s3ch19::crack_me();
    //
    // ch21
    // let mut mt = s3::s3ch21::MT19937::new();
    // s3::s3ch21::MT19937::seed_mt(&mut mt, 1131464071);
    // for _ in 0..10 {
    //     let random = s3::s3ch21::MT19937::extract_number(&mut mt);
    //     println!("random number = {random}");
    // }
    //
    // ch22
    // let rng = s3::s3ch22::go_get_coffee_while_this_runs();
    // let cracked_seed = s3::s3ch22::crack_mt19937_seed(rng);
    // println!("cracked seed = {cracked_seed}");
    //
    // ch23
    // s3::s3ch23::splice_mt19937();
    //
    // ch24
    //               aaaabbbbccccddddeeeeffffddddgggghhhh
    let plaintext = "VERYSECRETSTUFFHEREAAAAAAAAAAAAAA";
    let ciphertext = s3::s3ch24::mt19937_stream_cipher(0xace1, &plaintext);
    let cipher_len = ciphertext.len();
    let keystream_aaa = xor(&ciphertext[cipher_len - 14..], b"AAAAAAAAAAAAAA");
    let found_key = s3::s3ch24::bruteforce_mt19937_key(&ciphertext, b"AAAAAAAAAAAAAA".to_vec());
    println!(
        "cipher = {}\nxored = {}\nfound key = {:#x}",
        hex::encode(ciphertext),
        hex::encode(keystream_aaa),
        found_key
    )
}
