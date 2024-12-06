use crate::components::business_components::{
    component::{
        repository_module::{BRepository, BRepositoryConsole},
        BColumn, BConstraint, BDataType, BTableChangeEvents, BTableData, BTableGeneral, BTableIn,
        BTableInfo, BTableInsertedData, BusinessComponent,
    },
    components::BusinessConsole,
};
use sqlx::PgPool;
use std::sync::Arc;

pub fn create_database_console() -> Arc<BRepositoryConsole> {
    Arc::new(BRepositoryConsole::new())
}
pub fn create_console(database_console: Arc<BRepositoryConsole>) -> Arc<BusinessConsole> {
    Arc::new(BusinessConsole::new(database_console))
}

pub async fn create_repository(
    pool: PgPool,
    database_console: Arc<BRepositoryConsole>,
) -> Arc<BRepository> {
    Arc::new(BRepository::new(Some(pool), database_console).await)
}

pub async fn create_repository_table_and_console(
    pool: PgPool,
    table_in: &BTableIn,
) -> (Arc<BRepository>, Arc<BusinessConsole>) {
    let database_console = create_database_console();
    let business_console = create_console(database_console.clone());
    let repository = create_repository(pool, database_console).await;
    repository.create_table(table_in).await;
    (repository, business_console)
}

pub fn default_table_in() -> BTableIn {
    BTableIn {
        table_name: String::from("users"),
        columns: vec![
            BColumn {
                name: String::from("id"),
                datatype: BDataType::INTEGER,
                constraints: vec![BConstraint::PrimaryKey],
            },
            BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
                constraints: vec![],
            },
        ],
    }
}

pub fn create_btable_general(table_in: &BTableIn) -> BTableGeneral {
    BTableGeneral {
        table_name: table_in.table_name.clone(),
        column_names: table_in
            .columns
            .iter()
            .map(|col| col.name.clone())
            .collect(),
        data_types: table_in
            .columns
            .iter()
            .map(|col| col.datatype.clone())
            .collect(),
    }
}

pub fn sort_by_table_name(tables: &mut Vec<BTableGeneral>) {
    tables.sort_by(|a, b| a.table_name.cmp(&b.table_name));
}

pub fn sort_tables_general_info(tables: &mut Vec<BTableGeneral>) {
    tables.sort_by(|a, b| a.table_name.cmp(&b.table_name));
}

pub fn sort_columns(columns: &mut Vec<BColumn>) {
    columns.sort_by(|a, b| a.name.cmp(&b.name));
}
