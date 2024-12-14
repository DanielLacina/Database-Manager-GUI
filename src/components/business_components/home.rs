use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTableIn, BTableInfo, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Arc<BRepository>,
    pub title: Arc<AsyncMutex<Option<String>>>,
    console: Arc<BusinessConsole>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&self) {
        let mut locked_title = self.title.lock().await;
        *locked_title = Some(String::from("Home Component"));
        self.console
            .write(String::from("Home Component Initialized"));
    }
}

impl Home {
    pub fn new(repository: Arc<BRepository>, console: Arc<BusinessConsole>) -> Self {
        Self {
            repository,
            title: Arc::new(AsyncMutex::new(None)),
            console,
        }
    }
}
