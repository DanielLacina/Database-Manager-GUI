use crate::components::business_components::database::models::{ColumnsInfo, Table};
use crate::components::business_components::database::schemas::{
    Column, DataType, TableChangeEvents, TableIn,
};
use crate::components::business_components::tables::TableInfo;

pub type BTable = Table;
pub type BColumnsInfo = ColumnsInfo;
pub type BColumn = Column;
pub type BDataType = DataType;
pub type BTableIn = TableIn;
pub type BTableInfo = TableInfo;
pub type BTableChangeEvents = TableChangeEvents;

pub trait BusinessComponent {
    async fn initialize_component(&mut self) {}
}

pub(super) mod repository_module {
    use crate::components::business_components::database::repository::Repository;

    pub type BRepository = Repository;
}
