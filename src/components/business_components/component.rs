use crate::components::business_components::database::models::TableOut;

pub type BusinessTableOut = TableOut;

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

pub async fn initialize_business_component<T: BusinessComponent>(mut business_component: T) -> T {
    business_component.initialize_component().await;
    business_component
}

pub(super) mod repository_module {
    use crate::components::business_components::database::repository::Repository;

    pub type BusinessRepository = Repository;
}
