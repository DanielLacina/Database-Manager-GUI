use crate::components::business_components::database::{
    database::create_database_pool,
    models::{Table, TableInfo},
    schemas::TableIn,
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new(existing_pool: Option<PgPool>) -> Self {
        if let Some(pool) = existing_pool {
            Self { pool }
        } else {
            let pool = create_database_pool().await;
            Self { pool }
        }
    }

    pub async fn get_tables(&self) -> Result<Vec<Table>, Box<sqlx::Error>> {
        let res = sqlx::query_as::<_, Table>(
            "SELECT table_name
      FROM information_schema.tables
     WHERE table_schema='public'
       AND table_type='BASE TABLE'",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(res)
    }

    pub async fn get_table_info(
        &self,
        table_name: String,
    ) -> Result<Vec<TableInfo>, Box<sqlx::Error>> {
        let res = sqlx::query_as::<_, TableInfo>(&format!(
            "SELECT column_name, data_type
FROM information_schema.columns
WHERE table_name = '{}'",
            table_name
        ))
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(res)
    }

    pub async fn create_table(&self, table_in: TableIn) {
        let columns_query_list = table_in
            .columns
            .into_iter()
            .map(|column| format!("{} {}", &column.name, &column.datatype.to_string()))
            .collect::<Vec<_>>();
        let columns_query_joined = format!("({})", columns_query_list.join(", "));
        sqlx::query(&format!(
            "CREATE TABLE {} {}",
            &table_in.table_name, &columns_query_joined
        ))
        .execute(&self.pool)
        .await
        .unwrap();
    }
}
