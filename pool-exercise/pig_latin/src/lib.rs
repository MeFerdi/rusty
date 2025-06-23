pub fn pig_latin(text: &str) -> String {
    text.split_whitespace()
        .map(|s| {
            let mut nb_chars_to_move = 0;
            for c in s.chars() {
                if !is_vowel(c) {
                    nb_chars_to_move += 1;
                } else {
                    break;
                }
            }
            if nb_chars_to_move >= 2
                && nb_chars_to_move < s.len()
                && s.chars().nth(nb_chars_to_move - 1) == Some('q')
                && s.chars().nth(nb_chars_to_move) == Some('u')
            {
                nb_chars_to_move += 1;
            }
            format!("{}{}ay", &s[nb_chars_to_move..], &s[0..nb_chars_to_move])
        })
        .collect::<Vec<String>>()
        .join(" ")
}

fn is_vowel(mut c: char) -> bool {
    c = lowercase(c);
    c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u'
}

fn lowercase(c: char) -> char {
    c.to_lowercase().to_string().chars().next().unwrap()
}
