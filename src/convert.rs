pub fn convert(s: &mut String) {
    let first_char = s
        .chars()
        .next()
        .expect("Error: String doesn't contain a character!");
    let vocals: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    if vocals.contains(&first_char) {
        s.push_str("-fay");
    } else {
        let first_ch_len: usize= first_char.len_utf8();
        let string_slice = &s[first_ch_len..];
        *s = format!("{}-{}ay", string_slice, first_char);
    }
}
