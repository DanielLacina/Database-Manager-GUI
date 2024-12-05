use crate::components::business_components::database::models::ColumnsInfo;
use crate::components::business_components::database::schemas::{
    Column, ColumnForeignKey, Constraint, DataType, TableChangeEvents, TableGeneral, TableIn,
    TableInsertedData,
};
use crate::components::business_components::tables::table_info::TableInfo;

pub type BColumn = Column;
pub type BDataType = DataType;
pub type BTableIn = TableIn;
pub type BTableChangeEvents = TableChangeEvents;
pub type BTableInfo = TableInfo;
pub type BTableGeneral = TableGeneral;
pub type BConstraint = Constraint;
pub type BColumnForeignKey = ColumnForeignKey;
pub type BTableInsertedData = TableInsertedData;

pub trait BusinessComponent {
    async fn initialize_component(&self) {}
}

pub(super) mod repository_module {
    use crate::components::business_components::database::console::RepositoryConsole;
    use crate::components::business_components::database::repository::Repository;

    pub type BRepository = Repository;
    pub type BRepositoryConsole = RepositoryConsole;
}
