use crate::components::business_components::component::repository_module::BRepositoryConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct Console {
    pub messages: Arc<AsyncMutex<Vec<String>>>,
    repository_console: Arc<BRepositoryConsole>,
}

impl Console {
    pub fn new(repository_console: Arc<BRepositoryConsole>) -> Self {
        Self {
            messages: Arc::new(AsyncMutex::new(Vec::new())),
            repository_console,
        }
    }

    pub fn get_messages(&self) -> Vec<String> {
        self.messages.blocking_lock().clone()
    }
    pub fn get_database_messages(&self) -> Vec<String> {
        self.repository_console.messages.blocking_lock().clone()
    }

    pub fn clear_messages(&self) {
        *self.messages.blocking_lock() = vec![];
    }

    pub fn clear_database_messages(&self) {
        self.repository_console.clear_messages();
    }

    pub fn write(&self, message: String) {
        self.messages.blocking_lock().push(message);
    }
}
