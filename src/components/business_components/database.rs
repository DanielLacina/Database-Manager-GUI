use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

async fn create_database_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Env variable: DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap()
}
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Table {
    pub table_name: String,
}

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new() -> Self {
        let pool = create_database_pool().await;
        Self { pool }
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
}
