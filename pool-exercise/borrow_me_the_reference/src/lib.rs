pub fn delete_and_backspace(s: &mut String) {
    let mut result = Vec::new();
    let mut delete_count = 0;
    
    for c in s.chars() {
        match c {
            '-' => {
                if !result.is_empty() {
                    result.pop();
                }
            },
            '+' => {
                delete_count += 1;
            },
            _ => {
                if delete_count > 0 {
                    delete_count -= 1;
                } else {
                    result.push(c);
                }
            }
        }
    }
    
    *s = result.into_iter().collect();
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if let Some(pos) = s.find('+') {
            let left = s[..pos].parse::<i32>().unwrap_or(0);
            let right = s[pos+1..].parse::<i32>().unwrap_or(0);
            *s = (left + right).to_string();
        } else if let Some(pos) = s.find('-') {
            let left = s[..pos].parse::<i32>().unwrap_or(0);
            let right = s[pos+1..].parse::<i32>().unwrap_or(0);
            *s = (left - right).to_string();
        }
    }
}