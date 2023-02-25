pub fn validate_padding<T: AsRef<[u8]>>(input: T) -> Result<String, String> {
    let string_to_validate = input.as_ref();
    let length = string_to_validate.len();
    let supposed_padding_size = *string_to_validate.last().unwrap() as usize;
    if supposed_padding_size < 1 || supposed_padding_size > 16 {
        return Err("wrong padding".to_string())
    }
    let padding = string_to_validate
        .iter()
        .rev()
        .take(supposed_padding_size)
        .collect::<Vec<&u8>>();
    if supposed_padding_size != padding.len() {
        return Err("wrong padding".to_string())
    }
    for i in padding {
        if *i as usize != supposed_padding_size {
            return Err("wrong padding".to_string())
        }
    }
    Ok(String::from_utf8_lossy(
        &string_to_validate
            .iter()
            .take(length - supposed_padding_size)
            .map(|x| *x)
            .collect::<Vec<u8>>(),
    ).to_string())
}
