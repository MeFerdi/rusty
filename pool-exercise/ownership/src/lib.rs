pub fn first_subword(mut s: String) -> String {
    let mut end = 0;
    let chars: Vec<char> = s.chars().collect();
    
    for (i, &c) in chars.iter().enumerate().skip(1) {
        if c == '_' || c.is_uppercase() {
            end = i;
            break;
        }
    }
    
    if end > 0 {
        s.truncate(end);
    }
    s
}