use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnForeignKey, BConstraint, BDataType,
    BRowColumnValue, BTableChangeEvents, BTableDataChangeEvents, BTableGeneral, BTableIn,
    BTableInfo, BTableInsertedData, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use sqlx::Row;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tokio::task;
use std::iter::zip;

#[derive(Debug, Clone)]
pub struct TableData {
    repository: Arc<BRepository>,
    console: Arc<BusinessConsole>,
    pub tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    pub table_inserted_data: Arc<AsyncMutex<Option<BTableInsertedData>>>,
    pub table_data_change_events: Arc<AsyncMutex<Vec<BTableDataChangeEvents>>>,
    primary_key_column_names: Arc<AsyncMutex<Vec<String>>> 
}

pub struct TableDataChangeEventsByRowIndex {
   pub row_index: usize,  
   pub new_value: String
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
            table_data_change_events: Arc::new(AsyncMutex::new(vec![])),
        }
    }


    pub fn add_table_data_change_event(&self, table_data_change_event: BTableDataChangeEventsByRowNumber) {
        let mut locked_table_data_change_events = self.table_data_change_events.blocking_lock();

        match &table_data_change_event {
            BTableDataChangeEvents::ModifyRowColumnValue(row_column_value) => {
                if let Some(existing_event_index) = locked_table_data_change_events.iter().position(
                    |existing_event| matches!(existing_event, 
                        BTableDataChangeEvents::ModifyRowColumnValue(existing_row_column_value)
                        if zip(&row_column_value.conditions, &existing_row_column_value.conditions).all(|(condition, existing_condition)| condition.value == existing_condition.value) )
                    
                ) {
                    // Replace the existing event
                    locked_table_data_change_events.remove(existing_event_index);
                    locked_table_data_change_events.push(table_data_change_event);

                } else {
                            
                    locked_table_data_change_events.push(table_data_change_event);
                        }
            }
        }
        self.console.write(format!("{:?}", locked_table_data_change_events));

    // Add the new event if no matching event was found
    }

    async fn get_primary_key_columns_and_values_by_row_index(&self, row_index: usize) {
        let locked_table_inserted_data = self.table_inserted_data.blocking_lock();
        if let Some(row) = locked_table_inserted_data.get(row_index) {
            tables_general_info
        }
    }

    pub async fn insert_into_table(&self, table_inserted_data: BTableInsertedData) {
        self.repository.insert_into_table(table_inserted_data).await;
    }

    pub async fn update_table_data(&self) {
    // Extract and drop the lock on `table_inserted_data`
    let (table_name, table_data_change_events) = {
        let table_inserted_data_guard = self.table_inserted_data.lock().await;
        if let Some(ref table_inserted_data) = *table_inserted_data_guard {
            let table_name = table_inserted_data.table_name.clone();
            let table_data_change_events_guard = self.table_data_change_events.lock().await;
            let table_data_change_events = table_data_change_events_guard.clone();
            (table_name, table_data_change_events)
        } else {
            return; // If there's no table_inserted_data, exit the function
        }
    };
    // Use the extracted values without holding the locks
    self.repository.update_table_data(&table_name, &table_data_change_events).await;
    self.set_table_data(table_name).await;
}
    pub async fn set_table_data(&self, table_name: String) {
        // Lock the general info table
        let tables_general_info = self.tables_general_info.lock().await;
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
            let primary_key_column_names = self.repository.get_primary_key_column_names(&table_name).await.unwrap();

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
                                row.get::<String, _>(column_name.as_str())
                            })
                            .collect::<Vec<String>>()
                    })
                    .collect::<Vec<Vec<String>>>(),
            };
            // Update the shared table inserted data
            *self.table_inserted_data.lock().await = Some(table_inserted_data);
            *self.table_data_change_events.lock().await = vec![];
            *self.primary_key_column_names.lock().await = primary_key_column_names;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::business_components::component::{
        repository_module::BRepositoryConsole, BTableGeneral, BTableIn,
    };
    use crate::components::business_components::tables::{test_utils::{
        create_btable_general, create_repository_table_and_console, default_table_in, sort_columns,
        sort_tables_general_info}, utils::set_tables_general_info
    };
    use sqlx::PgPool;

    async fn create_table_data(
        pool: PgPool,
        table_in: &BTableIn,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> TableData {
        let (repository_result, console_result) =
            create_repository_table_and_console(pool, table_in).await;
        let table_general_info = Arc::new(AsyncMutex::new(Vec::<BTableGeneral>::new()));
        set_tables_general_info(repository_result.clone(), tables_general_info.clone()).await;
        let table_data = TableData::new(repository_result, console_result, tables_general_info);
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
