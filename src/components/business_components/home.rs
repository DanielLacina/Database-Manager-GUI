use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTable, BTableIn, BTableInfo,
    BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Arc<BRepository>,
    pub title: Option<String>,
    console: Arc<Mutex<BusinessConsole>>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.title = Some(String::from("Home Component"));
        let mut locked_console = self.console.lock().unwrap();
        locked_console.write(String::from("Home Component Initialized"));
    }
}

impl Home {
    pub fn new(repository: Arc<BRepository>, console: Arc<Mutex<BusinessConsole>>) -> Self {
        Self {
            repository,
            title: None,
            console,
        }
    }
}
