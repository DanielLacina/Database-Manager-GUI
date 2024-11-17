use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTable, BTableIn, BTableInfo,
    BusinessComponent,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Arc<BRepository>,
    pub title: Option<String>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.title = Some(String::from("Home Component"));
    }
}

impl Home {
    pub fn new(repository: Arc<BRepository>) -> Self {
        Self {
            repository,
            title: None,
        }
    }
}
