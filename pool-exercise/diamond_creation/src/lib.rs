pub fn get_diamond(c: char) -> Vec<String> {
    if c == 'A' {
        return vec!["A".to_string()];
    }
    let n = (c as u8 - b'A' + 1) as usize;
    let total_rows = 2 * n - 1;
    let mut diamond = Vec::with_capacity(total_rows);
    
    for i in 0..n {
        let current_char = (b'A' + i as u8) as char;
        let outer_spaces = " ".repeat(n - 1 - i);
        if current_char == 'A' {
            let row = format!("{}{}{}", outer_spaces, current_char, outer_spaces);
            diamond.push(row);
        } else {
            let inner_spaces = " ".repeat(2 * i - 1);
            let row = format!("{}{}{}{}{}", outer_spaces, current_char, inner_spaces, current_char, outer_spaces);
            diamond.push(row);
        }
    }
    
    for i in (0..n-1).rev() {
        diamond.push(diamond[i].clone());
    }
    
    diamond
}