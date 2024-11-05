use crate::components::business_components::database::Table;

pub type BusinessTable = Table;

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

pub async fn initialize_business_component<T: BusinessComponent>(mut business_component: T) -> T {
    business_component.initialize_component().await;
    business_component
}
