use crate::components::business_components::database::{
    database::create_database_pool,
    models::{ColumnsInfo, Table},
    schemas::{TableChangeEvents, TableIn},
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct Repository {
    pool: PgPool,
}

impl Repository {
    pub async fn new(existing_pool: Option<PgPool>) -> Self {
        if let Some(pool) = existing_pool {
            Self { pool }
        } else {
            let pool = create_database_pool().await;
            Self { pool }
        }
    }

    pub async fn get_tables(&self) -> Result<Vec<Table>, Box<sqlx::Error>> {
        let res = sqlx::query_as::<_, Table>(
            "SELECT table_name
      FROM information_schema.tables
     WHERE table_schema='public'
       AND table_type='BASE TABLE'",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(res)
    }

    pub async fn get_columns_info(
        &self,
        table_name: &str,
    ) -> Result<Vec<ColumnsInfo>, Box<sqlx::Error>> {
        let res = sqlx::query_as::<_, ColumnsInfo>(&format!(
            "SELECT column_name, data_type
FROM information_schema.columns
WHERE table_name = '{}'",
            table_name
        ))
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(res)
    }

    pub async fn create_table(&self, table_in: &TableIn) {
        let columns_query_list = table_in
            .columns
            .iter()
            .map(|column| format!("{} {}", &column.name, &column.datatype.to_string()))
            .collect::<Vec<_>>();
        let columns_query_joined = format!("({})", columns_query_list.join(", "));
        sqlx::query(&format!(
            "CREATE TABLE {} {}",
            &table_in.table_name, &columns_query_joined
        ))
        .execute(&self.pool)
        .await
        .unwrap();
    }

    pub async fn alter_table(
        &self,
        table_name: &str,
        table_change_events: &Vec<TableChangeEvents>,
    ) -> Result<(), sqlx::Error> {
        let mut changes = Vec::new();
        let mut new_table_name = None;

        for event in table_change_events {
            match event {
                TableChangeEvents::ChangeTableName(new_name) => {
                    new_table_name = Some(new_name);
                }
                TableChangeEvents::ChangeColumnDataType(column_name, new_data_type) => {
                    let change = format!(
                        "ALTER COLUMN \"{}\" TYPE {}",
                        column_name,
                        new_data_type.to_string()
                    );
                    changes.push(change);
                }
                TableChangeEvents::ChangeColumnName(old_name, new_name) => {
                    let change = format!("RENAME COLUMN \"{}\" TO \"{}\"", old_name, new_name);
                    changes.push(change);
                }
            }
        }
        let input_table_name;
        // If there's a table rename, we need to handle that separately
        let table_rename_query;
        if let Some(new_name) = new_table_name {
            table_rename_query =
                format!("ALTER TABLE \"{}\" RENAME TO \"{}\";", table_name, new_name);
            input_table_name = new_name;
        } else {
            table_rename_query = String::new();
            input_table_name = &table_name.to_string();
        };

        // Combine all the changes into one query
        let alter_table_query = if !changes.is_empty() {
            format!(
                "ALTER TABLE \"{}\" {};",
                input_table_name,
                changes.join(", ")
            )
        } else {
            String::new()
        };

        // Combine the table rename and column alterations into a single query
        let full_query = format!("{} {}", table_rename_query, alter_table_query);

        // Execute the query if it's not empty
        if !full_query.trim().is_empty() {
            sqlx::query(&full_query).execute(&self.pool).await?;
        }

        Ok(())
    }
}
