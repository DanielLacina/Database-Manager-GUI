use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTable, BTableIn, BTableInfo,
    BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct Home {
    repository: BRepository,
    pub title: Option<String>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.title = Some(String::from("Home Component"));
    }
}

impl Home {
    pub fn new(repository: BRepository) -> Self {
        Self {
            repository,
            title: None,
        }
    }
}
