use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTable, BTableIn, BTableInfo,
    BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct Tables {
    repository: BRepository,
    pub tables: Option<Vec<BTable>>,
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

    async fn get_table_info(&self, table_name: String) -> Vec<BTableInfo> {
        self.repository.get_table_info(table_name).await.unwrap()
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
    async fn test_initialize_tables_component(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let mut tables = tables_business_component(pool).await;
        tables.initialize_component().await;
        let expected_tables = vec![BTable {
            table_name: String::from("users"),
        }];

        assert_eq!(tables.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let mut tables = tables_business_component(pool).await;
        tables.initialize_component().await;
        tables
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

        assert_eq!(tables.tables, Some(expected_tables));
    }

    #[sqlx::test]
    async fn test_get_table_info(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let tables = tables_business_component(pool).await;
        let table_info = tables.get_table_info(String::from("users")).await;
        let expected_table_info = vec![BTableInfo {
            column_name: String::from("name"),
            data_type: String::from("text"),
        }];

        assert_eq!(table_info, expected_table_info);
    }
}
