use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let alphabet: HashSet<char> = ('a'..='z').collect();
    let input_chars: HashSet<char> = s.to_lowercase()
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .collect();
    alphabet.is_subset(&input_chars)
}