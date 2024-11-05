use crate::components::business_components::{
    component::{BusinessComponent, BusinessTableOut},
    repository::BusinessRepository,
};

#[derive(Debug, Clone)]
pub struct Home {
    repository: BusinessRepository,
    pub title: Option<String>,
    pub tables: Option<Vec<BusinessTableOut>>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.tables = Some(self.repository.get_tables().await.unwrap());
        self.title = Some(String::from("Home Component"));
    }
}

impl Home {
    pub fn new(repository: BusinessRepository) -> Self {
        Self {
            repository,
            title: None,
            tables: None,
        }
    }
}
