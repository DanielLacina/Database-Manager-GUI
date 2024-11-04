use crate::database::Repository;
use crate::home::Home;

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

async fn initialize_component<T: BusinessComponent>(mut business_component: T) -> T {
    business_component.initialize_component().await;
    business_component
}

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
