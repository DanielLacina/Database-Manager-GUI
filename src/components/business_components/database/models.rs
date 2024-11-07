#[derive(sqlx::FromRow, Debug, Clone)]
pub struct TableOut {
    pub table_name: String,
}
