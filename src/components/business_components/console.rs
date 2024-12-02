use crate::components::business_components::component::repository_module::BRepositoryConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct Console {
    pub messages: Vec<String>,
    repository_console: Arc<AsyncMutex<BRepositoryConsole>>,
}

impl Console {
    pub fn new(repository_console: Arc<AsyncMutex<BRepositoryConsole>>) -> Self {
        Self {
            messages: Vec::new(),
            repository_console,
        }
    }

    pub fn get_database_messages(&self) -> Vec<String> {
        let locked_repository_console = self.repository_console.blocking_lock();
        locked_repository_console.messages.clone()
    }

    pub fn clear_messages(&mut self) {
        self.messages = vec![];
    }

    pub fn clear_database_messages(&mut self) {
        let locked_repository_console = self.repository_console.blocking_lock();
        locked_repository_console.clear_messages();
    }

    pub fn write(&mut self, message: String) {
        self.messages.push(message);
    }
}
