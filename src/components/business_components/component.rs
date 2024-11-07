use crate::components::business_components::database::models::TableOut;

pub type BusinessTableOut = TableOut;

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

pub(super) mod repository_module {
    use crate::components::business_components::database::repository::Repository;

    pub type BusinessRepository = Repository;
}
