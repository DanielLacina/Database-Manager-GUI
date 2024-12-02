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

    pub fn database_messages(&self) -> Vec<String> {
        let locked_repository_console = self.repository_console.blocking_lock();
        locked_repository_console.messages.clone()
    }

    pub fn write(&mut self, message: String) {
        self.messages.push(message);
    }
}
