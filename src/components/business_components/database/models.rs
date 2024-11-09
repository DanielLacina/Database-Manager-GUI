#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Table {
    pub table_name: String,
}
