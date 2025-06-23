pub fn initials(names: Vec<&str>) -> Vec<String> {
    names
        .into_iter()
        .map(|name| {
            let mut initials = String::with_capacity(5);
            let mut words = name.split_whitespace();
            
            if let Some(first) = words.next() {
                if let Some(c) = first.chars().next() {
                    initials.push(c);
                    initials.push('.');
                }
            }
            
            if let Some(second) = words.next() {
                initials.push(' ');
                if let Some(c) = second.chars().next() {
                    initials.push(c);
                    initials.push('.');
                }
            }
            
            initials
        })
        .collect()
}