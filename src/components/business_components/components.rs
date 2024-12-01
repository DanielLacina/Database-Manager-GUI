use crate::components::business_components::component::repository_module::BRepository;
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
        let repository = Arc::new(BRepository::new(None).await);
        let console = Arc::new(Mutex::new(Console::new()));
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
