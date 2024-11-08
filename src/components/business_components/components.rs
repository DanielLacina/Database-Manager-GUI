use crate::components::business_components::component::repository_module::BusinessRepository;
use crate::components::business_components::home::Home;
use tokio;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_business_component_initialization() {
        BusinessComponents::new().await;
    }
}
