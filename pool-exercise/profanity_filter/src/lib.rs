pub struct Message {
    pub content: String,
}

impl Message {
    pub fn new(content: &str) -> Self {
        Message {
            content: content.to_string(),
        }
    }

    pub fn send_ms(&self) -> Option<&str> {
        if self.content.is_empty() || self.content.contains("stupid") {
            None
        } else {
            Some(&self.content)
        }
    }
}

pub fn check_ms(message: &str) -> Result<&str, &'static str> {
    // Create the Message but don't store it beyond this check
    if message.is_empty() || message.contains("stupid") {
        Err("ERROR: illegal")
    } else {
        Ok(message) // Return the input string slice directly
    }
}