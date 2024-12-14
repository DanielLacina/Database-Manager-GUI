use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct RepositoryConsole {
    pub messages: Arc<AsyncMutex<Vec<String>>>,
}

impl RepositoryConsole {
    pub fn new() -> Self {
        Self {
            messages: Arc::new(AsyncMutex::new(Vec::new())),
        }
    }

    pub fn write(&self, message: String) {
        self.messages.blocking_lock().push(message);
    }

    pub fn clear_messages(&self) {
        *self.messages.blocking_lock() = vec![];
    }
}
