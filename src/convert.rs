pub fn convert(s: &mut String) {
    let first_char = s
        .chars()
        .next()
        .expect("Error: String doesn't contain a character!");
    let vocals: [char; 10] = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
    let mut starts_with_vocal: bool = false;
    for i in vocals {
        if first_char == i {
            starts_with_vocal = true;
        }
    }
    if starts_with_vocal {
        s.push_str("-fay");
    } else {
        let mut boilerplate: bool = false;
        let mut str = String::new();
        for c in s.chars() {
            if boilerplate {
                str.push(c);
            } else {
                boilerplate = true;
            }
        }
        *s = str;
        s.push_str(&(first_char.to_string() + "ay"));
    }
}
