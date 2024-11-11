use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BDataType, BTable, BTableIn, BTableInfo,
    BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct Tables {
    repository: BRepository,
    pub tables: Option<Vec<BTable>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableInfo {
    pub table_name: String,
    pub columns_info: Vec<BColumnsInfo>,
}

impl BusinessComponent for Tables {
    async fn initialize_component(&mut self) {
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }
}

impl Tables {
    pub fn new(repository: BRepository) -> Self {
        Self {
            repository,
            tables: None,
        }
    }

    pub async fn add_table(&mut self, table_in: BTableIn) {
        self.repository.create_table(table_in).await;
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }

    pub async fn get_table_info(&self, table_name: String) -> TableInfo {
        let columns_info = self
            .repository
            .get_columns_info(table_name.clone())
            .await
            .unwrap();
        TableInfo {
            table_name,
            columns_info,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn tables_business_component(pool: PgPool) -> Tables {
        let repository = BRepository::new(Some(pool)).await;
        Tables::new(repository)
    }

    #[sqlx::test]
    async fn test_initialize_tables_component_component(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let mut tables_component = tables_business_component(pool).await;
        tables_component.initialize_component().await;
        let expected_tables = vec![BTable {
            table_name: String::from("users"),
        }];

        assert_eq!(tables_component.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let mut tables_component = tables_business_component(pool).await;
        tables_component.initialize_component().await;
        tables_component
            .add_table(BTableIn {
                table_name: String::from("users"),
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
        let expected_tables = vec![BTable {
            table_name: String::from("users"),
        }];

        assert_eq!(tables_component.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_get_table_info(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let tables_component = tables_business_component(pool).await;
        let table_name = String::from("users");
        let table_info = tables_component.get_table_info(table_name.clone()).await;
        let expected_table_info = TableInfo {
            table_name: table_name.clone(),
            columns_info: vec![BColumnsInfo {
                column_name: String::from("name"),
                data_type: String::from("text"),
            }],
        };

        assert_eq!(table_info, expected_table_info);
    }
}
