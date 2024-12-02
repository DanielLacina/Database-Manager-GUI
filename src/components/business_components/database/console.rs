#[derive(Debug, Clone)]
pub struct RepositoryConsole {
    pub messages: Vec<String>,
}

impl RepositoryConsole {
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    pub fn write(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn clear_messages(&mut self) {
        self.messages = vec![];
    }
}
