pub fn rotate(input: &str, key: i8) -> String {
    let key = key % 26;
    input.chars().map(|c| {
        if c.is_ascii_alphabetic() {
            let base = if c.is_ascii_lowercase() { b'a' } else { b'A' } as i8;
            let original_pos = c as i8 - base;
            let new_pos = (original_pos + key).rem_euclid(26);
            (base + new_pos) as u8 as char
        } else {
            c
        }
    }).collect()
}