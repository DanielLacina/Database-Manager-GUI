use crate::components::business_components::database::{
    database::create_database_pool,
    models::{ColumnsInfo, Table},
    schemas::{Constraint, TableChangeEvents, TableIn},
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
        let res = sqlx::query_as::<_, ColumnsInfo>(
                        "SELECT
                        c.column_name,
                        c.data_type,
                        ARRAY_AGG(tc.constraint_type::TEXT) AS constraint_types,
                        ARRAY_AGG(ccu.table_name::TEXT) AS referenced_tables,
                        ARRAY_AGG(ccu.column_name::TEXT) AS referenced_columns
                    FROM
                        information_schema.columns AS c
                    LEFT JOIN
                        information_schema.key_column_usage AS kcu
                        ON c.table_name = kcu.table_name
                        AND c.column_name = kcu.column_name
                    LEFT JOIN
                        information_schema.table_constraints AS tc
                        ON tc.constraint_name = kcu.constraint_name
                        AND tc.table_name = c.table_name
                    LEFT JOIN
                        information_schema.referential_constraints AS rc
                        ON rc.constraint_name = tc.constraint_name
                    LEFT JOIN
                        information_schema.constraint_column_usage AS ccu
                        ON ccu.constraint_name = rc.unique_constraint_name
                    WHERE
                        c.table_name = $1 
                    GROUP BY c.column_name, c.data_type",
        )
        .bind(&table_name)
        .fetch_all(&self.pool)
        .await
        .unwrap();
        Ok(res)
    }

    pub async fn create_table(&self, table_in: &TableIn) {
        let mut primary_key_columns = vec![];

        let columns_query_list: Vec<String> = table_in
            .columns
            .iter()
            .map(|column| {
                let mut column_configuration = vec![format!("\"{}\" {}", column.name, column.datatype)];
                for constraint in &column.constraints {
                    match constraint {
                        Constraint::ForeignKey(referenced_table, referenced_column) => {
                            column_configuration.push(format!(
                                "REFERENCES \"{}\"(\"{}\")",
                                referenced_table, referenced_column
                            ));
                        }
                        Constraint::PrimaryKey => {
                            primary_key_columns.push(column.name.clone());
                        }
                    }
                }
                column_configuration.join(" ")
            })
            .collect();

        // If there are primary keys, append the PRIMARY KEY constraint
        let mut full_query_list = columns_query_list.clone();
        if !primary_key_columns.is_empty() {
            full_query_list.push(format!(
                "PRIMARY KEY ({})",
                primary_key_columns
                    .iter()
                    .map(|col| format!("\"{}\"", col))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        let columns_query_joined = format!("({})", full_query_list.join(", "));

        // Construct the full SQL query
        let query = format!(
            "CREATE TABLE \"{}\" {}",
            table_in.table_name, columns_query_joined
        );

        // Print the query for debugging
        println!("Generated Query: {}", query);

        // Execute the query
        sqlx::query(&query)
            .execute(&self.pool)
            .await
            .unwrap();
    }

    pub async fn delete_table(&self, table_name: &str) {
        sqlx::query(&format!("DROP TABLE {}", &table_name))
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
                TableChangeEvents::AddForeignKey(column_name,referenced_table, referenced_column) => {
                     format!(
                        "ALTER TABLE \"{}\" ADD CONSTRAINT FOREGIN KEY(\"{}\") REFERENCES \"{}\"(\"{}\")",
                        current_table_name, column_name, referenced_table, referenced_column 
                    )
 
                }

                TableChangeEvents::AddPrimaryKey(column_name) => {
                     format!(
                        "ALTER TABLE \"{}\" ADD PRIMARY KEY (\"{}\")",
                         current_table_name, column_name )
 
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
