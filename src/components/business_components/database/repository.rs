use crate::components::business_components::database::{
    database::create_database_pool, models::TableOut,
};
use sqlx::PgPool;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new() -> Self {
        let pool = create_database_pool().await;
        Self { pool }
    }

    pub async fn get_tables(&self) -> Result<Vec<TableOut>, dyn std::error::Error> {
        let res = sqlx::query_as::<_, TableOut>(
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

    pub async fn create_table() {}
}
