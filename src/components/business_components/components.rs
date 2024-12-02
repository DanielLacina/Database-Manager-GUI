use crate::components::business_components::component::repository_module::{
    BRepository, BRepositoryConsole,
};
use crate::components::business_components::{
    console::Console, home::Home, tables::tables::Tables,
};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

pub type BusinessHome = Home;
pub type BusinessTables = Tables;
pub type BusinessConsole = Console;

#[derive(Debug, Clone)]
pub struct BusinessComponents {
    pub home: Arc<AsyncMutex<BusinessHome>>,
    pub tables: Arc<AsyncMutex<BusinessTables>>,
    pub console: Arc<Mutex<Console>>,
}

impl BusinessComponents {
    pub async fn new() -> Self {
        let repository_console = Arc::new(AsyncMutex::new(BRepositoryConsole::new()));
        let repository = Arc::new(BRepository::new(None, repository_console.clone()).await);
        let console = Arc::new(Mutex::new(Console::new(repository_console.clone())));
        Self {
            home: Arc::new(AsyncMutex::new(BusinessHome::new(
                repository.clone(),
                console.clone(),
            ))),
            tables: Arc::new(AsyncMutex::new(BusinessTables::new(
                repository.clone(),
                console.clone(),
            ))),
            console: console.clone(),
        }
    }
}
