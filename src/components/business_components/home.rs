use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BDataType, BTable, BTableIn, BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct Home {
    repository: BRepository,
    pub title: Option<String>,
    pub tables: Option<Vec<BTable>>,
}

impl BusinessComponent for Home {
    async fn initialize_component(&mut self) {
        self.tables = Some(self.repository.get_tables().await.unwrap());
        self.title = Some(String::from("Home Component"));
    }
}

impl Home {
    pub fn new(repository: BRepository) -> Self {
        Self {
            repository,
            title: None,
            tables: None,
        }
    }

    pub async fn add_table(&mut self, table_in: BTableIn) {
        self.repository.create_table(table_in).await;
        self.tables = Some(self.repository.get_tables().await.unwrap());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    async fn home_business_component(pool: PgPool) -> Home {
        let repository = BRepository::new(Some(pool)).await;
        Home {
            repository,
            title: None,
            tables: None,
        }
    }

    #[sqlx::test]
    async fn test_initialize_home_component(pool: PgPool) {
        sqlx::query!("CREATE TABLE users (name TEXT)")
            .execute(&pool)
            .await
            .unwrap();
        let mut home = home_business_component(pool).await;
        home.initialize_component().await;
        let expected_tables = vec![BTable {
            table_name: String::from("users"),
        }];

        assert_eq!(home.tables, Some(expected_tables));
        assert_eq!(home.title, Some(String::from("Home Component")));
    }

    #[sqlx::test]
    async fn test_add_table(pool: PgPool) {
        let mut home = home_business_component(pool).await;
        home.initialize_component().await;
        home.add_table(BTableIn {
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
            ],
        })
        .await;
        let expected_tables = vec![BTable {
            table_name: String::from("users"),
        }];

        assert_eq!(home.tables, Some(expected_tables));
    }
}
