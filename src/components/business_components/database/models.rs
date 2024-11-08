#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct TableOut {
    pub table_name: String,
}
