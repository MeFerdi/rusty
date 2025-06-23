mod messenger;
pub use messenger::*;
pub use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Worker {
    pub track_value: Rc<usize>,
    pub mapped_messages: RefCell<HashMap<String, String>>,
    pub all_messages: RefCell<Vec<String>>,
}

impl Worker {
    pub fn new(s: usize) -> Worker {
        Worker {
            track_value: Rc::new(s),
            mapped_messages: RefCell::new(HashMap::new()),
            all_messages: RefCell::new(vec![]),
        }
    }
}

impl Logger for Worker {
    fn warning(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn info(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
    fn error(&self, message: &str) {
        let v: Vec<&str> = message.split(": ").collect();
        self.mapped_messages
            .borrow_mut()
            .insert(v[0].to_string(), v[1].to_string());
        self.all_messages.borrow_mut().push(message.to_string());
    }
}