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
    console: Arc<BusinessConsole>,
}

impl TableDataInserter {
    pub fn new(
        repository: Arc<BRepository>,
        console: Arc<BusinessConsole>,
        table_info: Arc<BTableInfo>,
    ) -> Self {
        Self {
            repository,
            console,
            table_info,
        }
    }

    pub async fn insert_into_table(&self, table_inserted_data: BTableInsertedData) {
        self.repository.insert_into_table(table_inserted_data).await;
    }

    pub fn get_column_names(&self) -> Option<Vec<String>> {
        if let Some(table_name) = self.table_info.table_name.blocking_lock().as_ref() {
            let tables_general_info = self.table_info.tables_general_info.blocking_lock();
            let table_info_table_general_info = tables_general_info
                .iter()
                .find(|table| &table.table_name == table_name);

            if let Some(table_general_info) = table_info_table_general_info {
                Some(table_general_info.column_names.clone())
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::business_components::component::{
        repository_module::BRepositoryConsole, BTableGeneral, BTableIn,
    };
    use crate::components::business_components::tables::test_utils::{
        create_btable_general, create_repository_table_and_console, default_table_in, sort_columns,
        sort_tables_general_info,
    };
    use sqlx::PgPool;

    async fn create_table_data_inserter(
        pool: PgPool,
        table_in: &BTableIn,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> TableDataInserter {
        let (repository_result, console_result) =
            create_repository_table_and_console(pool, table_in).await;
        let table_info = BTableInfo::new(
            repository_result.clone(),
            console_result.clone(),
            tables_general_info,
        );
        let table_data_inserter =
            TableDataInserter::new(repository_result, console_result, Arc::new(table_info));
        table_data_inserter
    }

    #[sqlx::test]
    async fn test_insert_data_into_table(pool: PgPool) {
        let tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>> =
            Arc::new(AsyncMutex::new(Vec::new()));
        let table_in = default_table_in();
        let table_data_inserter =
            create_table_data_inserter(pool, &table_in, tables_general_info).await;
    }
}
