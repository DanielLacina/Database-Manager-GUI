use std::fmt;

pub enum DataType {
    TEXT,
    INT,
    DATETIME,
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::TEXT => write!(f, "TEXT"),
            DataType::INT => write!(f, "INT"),
            DataType::DATETIME => write!(f, "DATETIME"),
        }
    }
}

pub struct Column {
    pub name: String,
    pub data_type: DataType,
}

pub struct TableIn {
    pub table_name: String,
    pub columns: Vec<Column>,
}
