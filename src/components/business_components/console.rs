#[derive(Debug, Clone)]
pub struct Console {
    pub messages: Vec<String>,
}

impl Console {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn write(&mut self, message: String) {
        self.messages.push(message);
    }
}
