use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnForeignKey, BConstraint, BDataType,
    BTableChangeEvents, BTableGeneral, BTableIn, BTableInfo, BTableInsertedData, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use sqlx::Row;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tokio::task;

#[derive(Debug, Clone)]
pub struct TableData {
    repository: Arc<BRepository>,
    console: Arc<BusinessConsole>,
    pub tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    pub table_inserted_data: Arc<AsyncMutex<Option<BTableInsertedData>>>,
}

impl TableData {
    pub fn new(
        repository: Arc<BRepository>,
        console: Arc<BusinessConsole>,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> Self {
        Self {
            repository,
            console,
            tables_general_info,
            table_inserted_data: Arc::new(AsyncMutex::new(None)),
        }
    }

    pub async fn insert_into_table(&self, table_inserted_data: BTableInsertedData) {
        self.repository.insert_into_table(table_inserted_data).await;
    }

    pub async fn set_table_data(&self, table_name: String) {
        // Lock the general info table
        let tables_general_info = self.tables_general_info.lock().await;

        // Find the general info for the specified table
        if let Some(table_general_info) = tables_general_info
            .iter()
            .find(|info| info.table_name == table_name)
        {
            // Fetch rows for the table
            let table_data_rows = self
                .repository
                .get_table_data_rows(&table_name, &table_general_info.column_names)
                .await
                .unwrap();

            // Construct the inserted data
            let table_inserted_data = BTableInsertedData {
                table_name: table_name.clone(),
                column_names: table_general_info.column_names.clone(),
                data_types: table_general_info.data_types.clone(),
                rows: table_data_rows
                    .iter()
                    .map(|row| {
                        table_general_info
                            .column_names
                            .iter()
                            .map(|column_name| {
                                row.try_get::<String, _>(column_name.as_str()).unwrap()
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>(),
            };
            // Update the shared table inserted data
            *self.table_inserted_data.lock().await = Some(table_inserted_data);
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

    async fn create_table_data(
        pool: PgPool,
        table_in: &BTableIn,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> TableData {
        let (repository_result, console_result) =
            create_repository_table_and_console(pool, table_in).await;
        let table_info = BTableInfo::new(
            repository_result.clone(),
            console_result.clone(),
            tables_general_info,
        );
        let table_data = TableData::new(repository_result, console_result, Arc::new(table_info));
        table_data
    }

    #[sqlx::test]
    async fn test_insert_data_into_table(pool: PgPool) {
        let tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>> =
            Arc::new(AsyncMutex::new(Vec::new()));
        let table_in = default_table_in();
        let table_data = create_table_data(pool, &table_in, tables_general_info).await;
    }
}
