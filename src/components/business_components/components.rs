use crate::components::business_components::component::repository_module::BusinessRepository;
use crate::components::business_components::home::Home;

pub type BusinessHome = Home;

#[derive(Debug, Clone)]
pub struct BusinessComponents {
    pub home: BusinessHome,
}

impl BusinessComponents {
    pub async fn new() -> Self {
        let repository = BusinessRepository::new().await;
        Self {
            home: BusinessHome::new(repository),
        }
    }
}
