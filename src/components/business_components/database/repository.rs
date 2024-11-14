use crate::components::business_components::database::{
    database::create_database_pool,
    models::{ColumnsInfo, Table},
    schemas::{TableChangeEvents, TableIn},
};
use sqlx::{Executor, PgPool, Postgres, Transaction};

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
        // Begin a transaction
        let mut transaction: Transaction<'_, Postgres> = self.pool.begin().await?;
        let mut current_table_name = table_name.to_string();

        // Collect changes and detect table rename
        for event in table_change_events {
            let query = match event {
                TableChangeEvents::ChangeTableName(new_name) => {
                    let query = format!(
                        "ALTER TABLE \"{}\" RENAME TO \"{}\"",
                        current_table_name, new_name
                    );
                    current_table_name = new_name.clone();
                    query
                }
                TableChangeEvents::ChangeColumnDataType(column_name, new_data_type) => {
                    format!(
                        "ALTER TABLE \"{}\" ALTER COLUMN \"{}\" TYPE {} USING \"{}\"::{}",
                        current_table_name, column_name, new_data_type, column_name, new_data_type
                    )
                }
                TableChangeEvents::ChangeColumnName(old_name, new_name) => {
                    format!(
                        "ALTER TABLE \"{}\" RENAME COLUMN \"{}\" TO \"{}\"",
                        current_table_name, old_name, new_name
                    )
                }
                TableChangeEvents::AddColumn(column_name, data_type) => {
                    format!(
                        "ALTER TABLE \"{}\" ADD COLUMN \"{}\" {}",
                        current_table_name, column_name, data_type
                    )
                }
                TableChangeEvents::RemoveColumn(column_name) => {
                    format!(
                        "ALTER TABLE \"{}\" DROP COLUMN \"{}\"",
                        current_table_name, column_name
                    )
                }
            };

            // Execute the query within the transaction
            println!("Executing query: {}", query);
            sqlx::query(&query).execute(&mut *transaction).await?;
        }

        // Commit the transaction if all queries succeed
        transaction.commit().await?;

        Ok(())
    }
}
