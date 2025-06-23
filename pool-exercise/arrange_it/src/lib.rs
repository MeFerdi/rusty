pub fn arrange_phrase(phrase: &str) -> String {
    let mut words_with_pos: Vec<(u32, String)> = phrase
        .split_whitespace()
        .map(|word| {
            let pos = word.chars()
                .find(|c| c.is_ascii_digit())
                .and_then(|c| c.to_digit(10))
                .unwrap_or(0);
            let cleaned = word.chars()
                .filter(|c| !c.is_ascii_digit())
                .collect();
            (pos, cleaned)
        })
        .collect();
    
    words_with_pos.sort_by_key(|&(pos, _)| pos);
    
    words_with_pos.into_iter()
        .map(|(_, word)| word)
        .collect::<Vec<String>>()
        .join(" ")
}