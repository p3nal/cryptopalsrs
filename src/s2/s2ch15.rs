pub fn validate_padding<T: AsRef<[u8]>>(input: T) -> String {
    let string_to_validate = input.as_ref();
    let length = string_to_validate.len();
    let supposed_padding_size = *string_to_validate.last().unwrap() as usize;
    let padding = string_to_validate
        .iter()
        .rev()
        .take(supposed_padding_size)
        .collect::<Vec<&u8>>();
    // [length - 1 - *string_to_validate.last().unwrap() as u8..length -1]
    if supposed_padding_size != padding.len() {
        panic!("wrong padding");
    }
    for i in padding {
        if *i as usize != supposed_padding_size {
            panic!("wrong padding");
        }
    }
    String::from_utf8(
        string_to_validate
            .iter()
            .take(length - supposed_padding_size)
            .map(|x| *x)
            .collect::<Vec<u8>>(),
    )
    .unwrap()
}
