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

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new() -> Self {
        let pool = create_database_pool().await;
        Self { pool }
    }
}
