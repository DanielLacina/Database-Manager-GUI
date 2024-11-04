use crate::business_components::database::Repository;
use crate::business_components::home::Home;

#[derive(Debug, Clone)]
pub struct BusinessComponents {
    pub home: Home,
}

impl BusinessComponents {
    pub async fn new() -> Self {
        let repository = Repository::new().await;
        Self {
            home: Home::new(repository),
        }
    }
}
