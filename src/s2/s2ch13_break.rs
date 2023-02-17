pub fn break_s2ch13() {
    // encrypt(profile_for(input_email)) -> cipher ------ check_admin(parsing_routing(decrypt(cipher)))
    let cipher = crate::s2::s2ch13::Cipher::new();
    let ciphertext = crate::s2::s2ch13::client(
        &cipher,
        &std::str::from_utf8(&crate::s2::s2ch9::pkcs_7_padding("AAAAAAAAAAadmin", 26)).unwrap(), // the 26
        // is put there to get the padding at exactly 16 so i can get a good encrypted block of the
        // admin padded to 16
    );
    // client sending to server, intercepting...
    let intercepted_ciphertext = ciphertext;
    let admin_block = intercepted_ciphertext.chunks(16).nth(1).unwrap().to_vec();
    let ciphertext = crate::s2::s2ch13::client(&cipher, "abc@gmail.com");
    let first_block = ciphertext
        .chunks(16)
        .take(2)
        .flatten()
        .map(|x| *x)
        .collect::<Vec<u8>>();
    let payload = vec![first_block, admin_block]
        .into_iter()
        .flatten()
        .collect();
    let result = crate::s2::s2ch13::server(&cipher, payload);
    println!("admin role = {result}");
}
