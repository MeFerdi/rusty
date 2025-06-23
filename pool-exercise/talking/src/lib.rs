pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_yelling = trimmed[..trimmed.len() - 1].chars().all(is_uppercase)
        && trimmed.chars().any(char::is_alphabetic);
    let is_question = match trimmed.chars().last() {
        Some('?') => true,
        _ => false,
    };

    if is_question && is_yelling {
        "Quiet, I am thinking!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}

pub fn is_uppercase(c: char) -> bool {
    !c.is_alphabetic() || c.is_uppercase()
}