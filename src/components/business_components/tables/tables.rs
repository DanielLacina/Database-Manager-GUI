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
    pub table_info: Arc<TableInfo>,
    pub tables_general_info: Arc<AsyncMutex<Vec<BTableGeneralInfo>>>,
    console: Arc<BusinessConsole>,
}

impl BusinessComponent for Tables {
    async fn initialize_component(&self) {
        self.update_tables().await;
    }
}

impl Tables {
    pub fn new(repository: Arc<BRepository>, console: Arc<BusinessConsole>) -> Self {
        let tables_general_info = Arc::new(AsyncMutex::new(vec![]));
        Self {
            table_info: Arc::new(BTableInfo::new(
                repository.clone(),
                console.clone(),
                tables_general_info.clone(),
            )),
            repository,
            tables_general_info,
            console,
        }
    }

    pub async fn set_general_tables_info(&self) {
        let mut locked_tables = self.tables_general_info.lock().await;
        *locked_tables = self.repository.get_general_tables_info().await.unwrap();
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

    async fn tables_component(pool: PgPool, table_in: &BTableIn) -> Tables {
        let database_console = Arc::new(BRepositoryConsole::new());

        let repository = Arc::new(BRepository::new(Some(pool), database_console.clone()).await);
        repository.create_table(table_in).await;

        let console = Arc::new(BusinessConsole::new(database_console));
        Tables::new(repository, console)
    }

    async fn initialized_tables_component(pool: PgPool, table_in: &BTableIn) -> Tables {
        let tables = tables_component(pool, table_in).await;
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
        let table_in = default_table_in();
        let tables = initialized_tables_component(pool, &table_in).await;

        let expected_tables_general_info = vec![BTableGeneralInfo {
            table_name: table_in.table_name.clone(),
            column_names: table_in
                .columns
                .iter()
                .map(|column| column.name.clone())
                .collect(),
            data_types: table_in
                .columns
                .iter()
                .map(|column| column.datatype.clone())
                .collect(),
        }];

        let tables_general_info = tables.tables_general_info.lock().await;
        assert_eq!(*tables_general_info, expected_tables_general_info);
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let initial_table_in = default_table_in();
        let tables = initialized_tables_component(pool, &initial_table_in).await;

        let new_table_in = BTableIn {
            table_name: String::from("products"),
            columns: vec![BColumn {
                name: String::from("product_name"),
                datatype: BDataType::TEXT,
                constraints: vec![],
            }],
        };

        // Add a new table
        tables.add_table(new_table_in.clone()).await;

        // Verify both tables exist in `tables_general_info`
        let tables_general_info = tables.tables_general_info.lock().await;
        assert_eq!(tables_general_info.len(), 2);
        assert_eq!(tables_general_info[1].table_name, new_table_in.table_name);

        // Verify the columns info
        assert_eq!(
            tables_general_info[1]
                .column_names
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
            new_table_in
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect::<Vec<_>>()
        );
    }

    #[sqlx::test]
    async fn test_delete_table(pool: PgPool) {
        let table_in = default_table_in();
        let tables = initialized_tables_component(pool, &table_in).await;

        // Delete the initial table
        tables.delete_table(table_in.table_name.clone()).await;

        // Verify no tables exist in `tables_general_info`
        let tables_general_info = tables.tables_general_info.lock().await;
        assert!(tables_general_info.is_empty());
    }

    #[sqlx::test]
    async fn test_set_general_tables_info(pool: PgPool) {
        let initial_table_in = default_table_in();
        let tables = initialized_tables_component(pool, &initial_table_in).await;

        let new_table_in = BTableIn {
            table_name: String::from("orders"),
            columns: vec![BColumn {
                name: String::from("order_id"),
                datatype: BDataType::INTEGER,
                constraints: vec![BConstraint::PrimaryKey],
            }],
        };

        // Add a new table and update general table info
        tables.add_table(new_table_in.clone()).await;

        let tables_general_info = tables.tables_general_info.lock().await;
        assert_eq!(tables_general_info.len(), 2);
        assert_eq!(tables_general_info[1].table_name, new_table_in.table_name);
    }

    #[sqlx::test]
    async fn test_get_general_table_info(pool: PgPool) {
        let initial_table_in = default_table_in();
        let tables = initialized_tables_component(pool, &initial_table_in).await;

        let new_table_in = BTableIn {
            table_name: String::from("inventory"),
            columns: vec![BColumn {
                name: String::from("item_name"),
                datatype: BDataType::TEXT,
                constraints: vec![],
            }],
        };

        // Add a table and fetch general table info
        tables.add_table(new_table_in.clone()).await;
        tables.set_general_tables_info().await;

        let tables_general_info = tables.tables_general_info.lock().await;
        assert_eq!(tables_general_info.len(), 2);
        assert_eq!(tables_general_info[1].table_name, new_table_in.table_name);
    }
}
