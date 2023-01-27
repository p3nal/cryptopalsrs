mod s1;
use std::path::Path;

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
    let path = Path::new("./src/s1/8.txt");
    let (ecb, count) = s1::s1ch8::detects_aes_in_acb(path);
    println!("ecb line {} with count {}", ecb, count);
}
