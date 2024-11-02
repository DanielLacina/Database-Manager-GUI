use crate::database::Repository;
use crate::home::Home;

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

#[derive(Debug, Clone)]
pub struct Components {
    home: Home,
}

impl Components {
    pub async fn new() -> Self {
        let repository = Repository::new().await;
        Self {
            home: Home::new(repository),
        }
    }
}
