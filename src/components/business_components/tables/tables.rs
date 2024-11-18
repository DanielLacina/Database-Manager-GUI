use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BDataType, BTable, BTableChangeEvents,
    BTableIn, BTableInfo, BusinessComponent,
};
use crate::components::business_components::tables::table_info::TableInfo;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Tables {
    repository: Arc<BRepository>,
    pub tables: Option<Vec<BTable>>,
    pub table_info: Option<TableInfo>,
}

impl BusinessComponent for Tables {
    async fn initialize_component(&mut self) {
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }
}

impl Tables {
    pub fn new(repository: Arc<BRepository>) -> Self {
        Self {
            repository,
            tables: None,
            table_info: None,
        }
    }

    pub async fn set_table_info(&mut self, table_name: String) {
        let mut table_info = TableInfo::new(self.repository.clone(), table_name);
        table_info.initialize_component().await;
        self.table_info = Some(table_info);
    }

    pub async fn add_table(&mut self, table_in: BTableIn) {
        self.repository.create_table(&table_in).await;
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }

    pub async fn delete_table(&mut self, table_name: String) {
        self.repository.delete_table(&table_name).await;
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn tables_component(pool: PgPool, table_in: &BTableIn) -> Tables {
        let repository = BRepository::new(Some(pool)).await;
        repository.create_table(table_in).await;
        Tables::new(repository.into())
    }

    /// Helper function to initialize the `Tables` component.
    async fn initialized_tables_component(pool: PgPool, table_in: &BTableIn) -> Tables {
        let mut tables_component = tables_component(pool, table_in).await;
        tables_component.initialize_component().await;
        tables_component
    }

    fn default_table_in() -> BTableIn {
        BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        }
    }

    #[sqlx::test]
    async fn test_initialize_tables_component(pool: PgPool) {
        let table_in = default_table_in();

        // Initialize the tables component
        let mut tables = initialized_tables_component(pool, &table_in).await;

        // Expected result
        let expected_tables = vec![BTable {
            table_name: table_in.table_name,
        }];

        // Assert that the initialized component matches the expected output
        assert_eq!(tables.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool, &table_in).await;
        let create_table_name = String::from("account");
        tables
            .add_table(BTableIn {
                table_name: create_table_name.clone(),
                columns: vec![
                    BColumn {
                        name: String::from("username"),
                        datatype: BDataType::TEXT,
                    },
                    BColumn {
                        name: String::from("password"),
                        datatype: BDataType::TEXT,
                    },
                    BColumn {
                        name: String::from("balance"),
                        datatype: BDataType::INT,
                    },
                    BColumn {
                        name: String::from("join_date"),
                        datatype: BDataType::TIMESTAMP,
                    },
                ],
            })
            .await;
        let mut expected_tables = vec![
            BTable {
                table_name: table_in.table_name,
            },
            BTable {
                table_name: create_table_name,
            },
        ];
        expected_tables.sort_by(|a, b| b.table_name.cmp(&a.table_name));
        tables
            .tables
            .as_mut()
            .unwrap()
            .sort_by(|a, b| b.table_name.cmp(&a.table_name));

        assert_eq!(tables.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_delete_table(pool: PgPool) {
        let table_in = default_table_in();
        let mut tables = initialized_tables_component(pool, &table_in).await;
        tables.delete_table(table_in.table_name).await;
        let mut expected_tables = vec![];
        assert_eq!(tables.tables, Some(expected_tables));
    }
}
