use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BConstraint, BDataType,
    BTableChangeEvents, BTableGeneralInfo, BTableIn, BTableInfo, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use crate::components::business_components::tables::table_info::TableInfo;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct Tables {
    repository: Arc<BRepository>,
    pub table_info: Arc<AsyncMutex<Option<TableInfo>>>,
    pub tables_general_info: Arc<AsyncMutex<Option<Vec<BTableGeneralInfo>>>>,
    pub console: Arc<Mutex<BusinessConsole>>,
}

impl BusinessComponent for Tables {
    async fn initialize_component(&mut self) {
        self.update_tables().await;
    }
}

impl Tables {
    pub fn new(repository: Arc<BRepository>, console: Arc<Mutex<BusinessConsole>>) -> Self {
        Self {
            repository,
            table_info: Arc::new(AsyncMutex::new(None)),
            tables_general_info: Arc::new(AsyncMutex::new(None)),
            console,
        }
    }

    pub async fn set_general_tables_info(&self) {
        let mut locked_tables = self.tables_general_info.lock().await;
        *locked_tables = Some(self.repository.get_general_tables_info().await.unwrap());
    }

    pub async fn set_table_info(&self, table_name: String) {
        let mut table_info = TableInfo::new(
            self.repository.clone(),
            self.console.clone(),
            self.tables_general_info.clone(),
            table_name,
        );
        table_info.initialize_component().await;
        let mut locked_table_info = self.table_info.lock().await;
        *locked_table_info = Some(table_info);
    }

    pub async fn add_table(&self, table_in: BTableIn) {
        self.repository.create_table(&table_in).await;
        self.set_general_tables_info().await;
    }

    pub async fn update_tables(&self) {
        self.set_general_tables_info().await;
    }

    pub async fn delete_table(&self, table_name: String) {
        self.repository.delete_table(&table_name).await;
        self.set_general_tables_info().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::business_components::component::repository_module::BRepositoryConsole;
    use sqlx::PgPool;

    async fn tables_component(pool: PgPool) -> Tables {
        let database_console = Arc::new(AsyncMutex::new(BRepositoryConsole::new()));

        let repository = Arc::new(BRepository::new(Some(pool), database_console.clone()).await);
        let console = Arc::new(Mutex::new(BusinessConsole::new(database_console)));
        Tables::new(repository, console)
    }

    async fn initialized_tables_component(pool: PgPool) -> Tables {
        let mut tables = tables_component(pool).await;
        tables.initialize_component().await;
        tables
    }

    fn default_table_in() -> BTableIn {
        BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
                constraints: vec![BConstraint::PrimaryKey],
            }],
        }
    }

    #[sqlx::test]
    async fn test_initialize_tables_component(pool: PgPool) {
        let mut tables = initialized_tables_component(pool).await;

        // Ensure `tables_general_info` is initialized
        assert!(tables.tables_general_info.lock().await.is_some());

        let general_info = tables.tables_general_info.lock().await.unwrap();
        assert!(general_info.is_empty(), "Expected no tables initially");
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool).await;

        // Add a new table
        tables.add_table(table_in.clone()).await;

        // Verify the table exists in `tables_general_info`
        let general_info = tables.tables_general_info.lock().await.unwrap();
        assert_eq!(general_info.len(), 1);
        assert_eq!(general_info[0].table_name, table_in.table_name);

        // Verify the columns info
        assert_eq!(
            general_info[0]
                .column_names
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            table_in
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect::<Vec<_>>()
        );
    }

    #[sqlx::test]
    async fn test_delete_table(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool).await;

        // Add a table and then delete it
        tables.add_table(table_in.clone()).await;
        tables.delete_table(table_in.table_name.clone()).await;

        // Verify the table no longer exists
        let general_info = tables.tables_general_info.lock().await.unwrap();
        assert!(general_info.is_empty());
    }

    #[sqlx::test]
    async fn test_set_table_info(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool).await;

        // Add a table and set its info
        tables.add_table(table_in.clone()).await;
        tables.set_table_info(table_in.table_name.clone()).await;

        // Verify `table_info` is set correctly
        let table_info = tables.table_info.lock().await.unwrap();
        assert_eq!(table_info.table_name, table_in.table_name);
        assert_eq!(
            table_info
                .columns_info
                .iter()
                .map(|col| col.name.clone())
                .collect::<Vec<_>>(),
            table_in
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect::<Vec<_>>()
        );
    }

    #[sqlx::test]
    async fn test_get_general_table_info(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool).await;

        // Add a table and fetch general table info
        tables.add_table(table_in.clone()).await;
        tables.set_general_tables_info().await;

        let general_info = tables.tables_general_info.lock().await.unwrap();

        // Verify general table info
        assert_eq!(general_info.len(), 1);
        assert_eq!(general_info[0].table_name, table_in.table_name);
        assert_eq!(
            general_info[0]
                .column_names
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            table_in
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect::<Vec<_>>()
        );
    }
}
