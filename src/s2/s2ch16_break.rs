/// here is a demo of a cbc bitflipping attack
/// ; in ascii is 00111011 we bitflip the 3rd bit to get 00111111 which is ascii
/// for ?, escaping the filters. same thing goes for = which is 00111101 and
/// becomes also 00111111
/// comment1=cooking%20MCs;userdata=some;admin=true;comment2=%20like%20a%20pound%20of%20bacon
pub fn break_s2ch16() {
    let cipher = crate::s2::s2ch16::Cipher::new();
    // ?'s to bitflip later
    let my_arbitrary_controlled_input_string = format!("some?admin?true");
    let mut ciphertext =
        crate::s2::s2ch16::first_function(&cipher, my_arbitrary_controlled_input_string);
    // after capturing the ciphertext we have to bitflip the ?'s to make ; and =
    ciphertext[36 - 16] = ciphertext[36 - 16] ^ 0b0000100;
    ciphertext[42 - 16] = ciphertext[42 - 16] ^ 0b0000010;
    let result = crate::s2::s2ch16::second_function(&cipher, ciphertext);
    println!("are we admin yet?\n{}", if result {
        "yes!"
    } else {
        "no..."
    });
}
