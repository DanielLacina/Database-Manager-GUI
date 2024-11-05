use crate::components::business_components::home::Home;
use crate::components::business_components::repository::BusinessRepository;

#[derive(Debug, Clone)]
pub struct BusinessComponents {
    pub home: Home,
}

impl BusinessComponents {
    pub async fn new() -> Self {
        let repository = BusinessRepository::new().await;
        Self {
            home: Home::new(repository),
        }
    }
}
