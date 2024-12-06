use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnForeignKey, BConstraint, BDataType,
    BTableChangeEvents, BTableDataInserter, BTableGeneral, BTableIn, BTableInfo,
    BTableInsertedData, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tokio::task;

#[derive(Debug, Clone)]
pub struct TableDataInserter {
    repository: Arc<BRepository>,
    table_info: Arc<BTableInfo>,
}

impl TableDataInserter {
    pub fn new(repository: Arc<BRepository>, table_info: Arc<BTableInfo>) -> Self {
        Self {
            repository,
            table_info,
        }
    }

    pub async fn insert_into_table(&self, table_inserted_data: BTableInsertedData) {
        self.repository.insert_into_table(table_inserted_data).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::business_components::component::{
        repository_module::BRepositoryConsole, BTableGeneral, BTableIn,
    };
    use crate::components::business_components::tables::test_utils::{
        create_btable_general, default_table_in, sort_columns, sort_tables_general_info,
    };
    use sqlx::PgPool;

    //    async fn create_table_data_inserter(
    //        pool: PgPool,
    //        table_in: &BTableIn,
    //        tables_general_info: Arc<Mutex<Vec<BTableGeneral>>>,
    //    ) -> TableDataInserter {
    //        let repository =
    //        let table_info = (BTableInfo::new);

    //    }
}
