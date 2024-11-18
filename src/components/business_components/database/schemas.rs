use crate::components::business_components::database::models::ColumnsInfo;
use std::fmt;
use std::iter::zip;

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

impl DataType {
    pub fn to_datatype(value: String) -> Self {
        match value.as_str() {
            "text" => Self::TEXT,
            "integer" => Self::INT,
            "timestamp without time zone" => Self::TIMESTAMP,
            _ => panic!("Invalid datatype"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Constraint {
    ForeignKey(String, String),
    PrimaryKey,
}

impl fmt::Display for Constraint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Constraint::ForeignKey(referenced_table, referenced_column) => {
                write!(f, "REFERENCES {}({})", referenced_table, referenced_column)
            }
            Constraint::PrimaryKey => write!(f, "PRIMARY KEY"),
        }
    }
}

impl Constraint {
    pub fn to_constraint(
        constraint_type: String,
        referenced_table: Option<String>,
        referenced_column: Option<String>,
    ) -> Self {
        match constraint_type.as_str() {
            "PRIMARY KEY" => Self::PrimaryKey,
            "FOREIGN KEY" => {
                Self::ForeignKey(referenced_table.unwrap(), referenced_column.unwrap())
            }
            _ => panic!("Invalid Constraint"),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct Column {
    pub name: String,
    pub datatype: DataType,
    pub constraints: Vec<Constraint>,
}

impl Column {
    pub fn to_column(column_info: ColumnsInfo) -> Self {
        Self {
            name: column_info.column_name,
            datatype: DataType::to_datatype(column_info.data_type),
            constraints: zip(
                zip(column_info.constraint_types, column_info.referenced_tables),
                column_info.referenced_columns,
            )
            .map(|((constraint_type, referenced_table), referenced_column)| {
                Constraint::to_constraint(constraint_type, referenced_table, referenced_column)
            })
            .collect(),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq)]
pub struct TableIn {
    pub table_name: String,
    pub columns: Vec<Column>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TableChangeEvents {
    ChangeTableName(String),
    ChangeColumnDataType(String, DataType),
    ChangeColumnName(String, String),
    AddColumn(String, DataType),
    RemoveColumn(String),
    AddForeignKey(String, String, String),
    AddPrimaryKey(String),
}
