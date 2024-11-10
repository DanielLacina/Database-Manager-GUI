use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DataType {
    TEXT,
    INT,
    TIMESTAMP,
}

impl Default for DataType {
    fn default() -> Self {
        DataType::TEXT
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::TEXT => write!(f, "TEXT"),
            DataType::INT => write!(f, "INT"),
            DataType::TIMESTAMP => write!(f, "TIMESTAMP"),
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
}

#[derive(Default, Debug, Clone)]
pub struct TableIn {
    pub table_name: String,
    pub columns: Vec<Column>,
}
