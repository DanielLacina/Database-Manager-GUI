#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct Table {
    pub table_name: String,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct ColumnsInfo {
    pub column_name: String,
    pub data_type: String,
    pub constraint_types: Vec<Option<String>>,
    pub referenced_tables: Vec<Option<String>>,
    pub referenced_columns: Vec<Option<String>>,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct TableGeneralInfo {
    pub table_name: String,
    pub column_names: Vec<String>,
    pub data_types: Vec<String>,
}

#[derive(sqlx::FromRow, Debug, Clone, PartialEq)]
pub struct PrimaryKeyConstraint {
    pub conname: String,
}
