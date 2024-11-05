use crate::components::business_components::component::{BusinessComponent, BusinessTable};
use crate::components::business_components::database::Repository;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Repository,
    pub title: Option<String>,
    pub tables: Option<Vec<BusinessTable>>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.tables = Some(self.repository.get_tables().await.unwrap());
        self.title = Some(String::from("Home Component"));
    }
}

impl Home {
    pub fn new(repository: Repository) -> Self {
        Self {
            repository,
            title: None,
            tables: None,
        }
    }
}
