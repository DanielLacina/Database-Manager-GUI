use crate::components::business_components::component::repository_module::BusinessRepository;
use crate::components::business_components::home::Home;

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
