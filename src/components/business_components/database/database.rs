use dotenvy::dotenv;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn create_database_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("Env variable: DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url.as_str())
        .await
        .unwrap()
}
