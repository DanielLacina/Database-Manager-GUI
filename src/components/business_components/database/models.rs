#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Table {
    pub table_name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct ColumnsInfo {
    pub column_name: String,
    pub data_type: String,
}
