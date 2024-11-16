use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BDataType, BTable, BTableChangeEvents,
    BTableIn, BTableInfo, BusinessComponent,
};
use crate::components::business_components::tables::table_info::TableInfo;

#[derive(Debug, Clone)]
pub struct Tables {
    repository: BRepository,
    pub tables: Option<Vec<BTable>>,
    pub table_info: Option<TableInfo>,
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
}
