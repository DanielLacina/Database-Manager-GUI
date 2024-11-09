use crate::components::business_components::database::models::Table;
use crate::components::business_components::database::schemas::{Column, DataType, TableIn};

pub type BTable = Table;
pub type BColumn = Column;
pub type BDataType = DataType;
pub type BTableIn = TableIn;

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

pub(super) mod repository_module {
    use crate::components::business_components::database::repository::Repository;

    pub type BRepository = Repository;
}
