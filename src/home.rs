use crate::component::Component;
use crate::database::Repository;
use crate::database::Table;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Repository,
    pub title: Option<String>,
    pub tables: Option<Vec<Table>>,
}

impl Component for Home {
    pub async fn initialize_component(&mut self) {
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
