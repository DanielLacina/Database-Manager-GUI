use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

struct Database {
    database_url: String,
    connection_pool: PgPoolOptions,
}

impl Database {
    async fn new() {
        dotenv().ok();
        let database_url =
            env::var("DATABASE_URL").expect("Env variable: DATABASE_URL must be set");
        let connection_pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(self.database_url)
            .await
             unwrap(); 
        Self {
            database_url,
            connection_pool,
        }
    }
}
