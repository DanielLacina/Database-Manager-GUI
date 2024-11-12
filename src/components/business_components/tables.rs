use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BDataType, BTable, BTableChangeEvents,
    BTableIn, BTableInfo, BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct Tables {
    repository: BRepository,
    pub tables: Option<Vec<BTable>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableInfo {
    pub table_name: String,
    pub columns_info: Vec<BColumn>,
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
        self.repository.create_table(&table_in).await;
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }

    pub async fn get_table_info(&self, table_name: String) -> TableInfo {
        let columns_info = self.repository.get_columns_info(&table_name).await.unwrap();
        let columns_info_with_enum_datatype = columns_info
            .into_iter()
            .map(|column| BColumn {
                name: column.column_name,
                datatype: BDataType::to_datatype(column.data_type),
            })
            .collect();
        TableInfo {
            table_name,
            columns_info: columns_info_with_enum_datatype,
        }
    }

    pub async fn alter_table(
        &self,
        table_name: String,
        table_change_events: Vec<BTableChangeEvents>,
    ) -> TableInfo {
        let res = self
            .repository
            .alter_table(&table_name, &table_change_events)
            .await;
        eprintln!("{:?}", res);
        let mut input_table_name = table_name;
        for event in table_change_events {
            match event {
                BTableChangeEvents::ChangeTableName(new_table_name) => {
                    input_table_name = new_table_name;
                }
                _ => {}
            }
        }
        self.get_table_info(input_table_name).await
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
            columns_info: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        assert_eq!(table_info, expected_table_info);
    }

    #[sqlx::test]
    async fn test_alter_table(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let tables_component = tables_business_component(pool).await;
        let new_table_name = String::from("accounts");
        let new_column_name = String::from("username");
        let new_column_datatype = BDataType::INT;
        let table_change_events = vec![
            BTableChangeEvents::ChangeTableName(new_table_name.clone()),
            BTableChangeEvents::ChangeColumnName(String::from("name"), new_column_name.clone()),
            BTableChangeEvents::ChangeColumnDataType(
                new_column_name.clone(),
                new_column_datatype.clone(),
            ),
        ];
        let altered_table_info = tables_component
            .alter_table(String::from("users"), table_change_events)
            .await;
        let expected_altered_table_info = TableInfo {
            table_name: new_table_name,
            columns_info: vec![BColumn {
                name: new_column_name,
                datatype: new_column_datatype,
            }],
        };

        assert_eq!(altered_table_info, expected_altered_table_info);
    }
}
