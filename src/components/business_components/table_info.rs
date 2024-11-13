use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BDataType, BTable, BTableChangeEvents,
    BTableIn, BusinessComponent,
};

#[derive(Debug, Clone)]
pub struct TableInfo {
    repository: BRepository,
    pub table_name: String,
    pub columns_info: Vec<BColumn>,
    table_change_events: Vec<BTableChangeEvents>,
}

impl BusinessComponent for TableInfo {
    async fn initialize_component(&mut self) {
        self.set_table_info(&self.table_name.clone()).await;
    }
}

impl TableInfo {
    pub fn new(repository: BRepository, table_name: String) -> Self {
        Self {
            repository,
            table_name,
            columns_info: vec![],
            table_change_events: vec![],
        }
    }

    async fn set_table_info(&mut self, table_name: &str) {
        let columns_info = self.repository.get_columns_info(table_name).await.unwrap();
        let columns_info_with_enum_datatype = columns_info
            .into_iter()
            .map(|column| BColumn {
                name: column.column_name,
                datatype: BDataType::to_datatype(column.data_type),
            })
            .collect();
        self.table_name = table_name.to_string();
        self.columns_info = columns_info_with_enum_datatype;
    }

    pub fn add_table_change_event(&mut self, mut table_change_event: BTableChangeEvents) {
        if let BTableChangeEvents::ChangeTableName(new_table_name) = &table_change_event {
            // Remove any existing ChangeTableName events before adding the new one
            self.table_change_events
                .retain(|event| !matches!(event, BTableChangeEvents::ChangeTableName(_)));
        }

        if let BTableChangeEvents::ChangeColumnDataType(column_name, data_type) =
            &table_change_event
        {
            self.table_change_events.retain(|event| {
                // Only retain events that are not ChangeColumnDataType for the same column_name
                match event {
                    BTableChangeEvents::ChangeColumnDataType(existing_column_name, _) => {
                        existing_column_name != column_name
                    }
                    _ => true, // Retain all other event types
                }
            });
        }
        if let BTableChangeEvents::ChangeColumnName(column_name, new_column_name) =
            &table_change_event
        {
            // Step 1: Check for existing events with the same target column name
            if let Some(existing_event_index) =
                self.table_change_events
                    .iter()
                    .position(|event| match event {
                        BTableChangeEvents::ChangeColumnName(original_column, modified_column) => {
                            modified_column == column_name
                        }
                        _ => false,
                    })
            {
                // Step 2: Update the new event with the original column name from the existing event
                if let BTableChangeEvents::ChangeColumnName(original_column, _) =
                    &self.table_change_events[existing_event_index]
                {
                    table_change_event = BTableChangeEvents::ChangeColumnName(
                        original_column.clone(),
                        new_column_name.clone(),
                    );
                }

                // Step 3: Remove the existing event
                self.table_change_events.remove(existing_event_index);
            }
        }

        // Add the new table change event
        self.table_change_events.push(table_change_event);
    }

    pub async fn alter_table(&mut self) {
        // Apply the stored table change events
        if !self.table_change_events.is_empty() {
            let res = self
                .repository
                .alter_table(&self.table_name, &self.table_change_events)
                .await;
            println!("Alter table result: {:?}", res);
        }

        // Process the stored events to update the internal state
        let mut new_table_name = self.table_name.clone(); // Copy the current table name

        for event in &self.table_change_events {
            match event {
                BTableChangeEvents::ChangeTableName(updated_name) => {
                    new_table_name = updated_name.clone();
                }
                _ => {}
            }
        }

        // Update the internal table name and clear events
        self.table_name = new_table_name;
        self.table_change_events.clear();

        // Now call `set_table_info` with the updated table name
        let updated_table_name = self.table_name.clone();
        self.set_table_info(&updated_table_name).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;

    /// Helper function to create a `TableInfo` instance.
    async fn create_table_info(pool: PgPool, table_in: &BTableIn) -> TableInfo {
        let repository = BRepository::new(Some(pool)).await;
        repository.create_table(table_in).await;
        let mut table_info = TableInfo::new(repository, table_in.table_name.clone());
        table_info.set_table_info(&table_in.table_name).await;
        table_info
    }

    #[sqlx::test]
    async fn test_initialize_component(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Call initialize_component to set columns_info
        table_info.initialize_component().await;

        // Assertions
        assert_eq!(table_info.table_name, table_in.table_name);
        assert_eq!(
            table_info.columns_info,
            vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }]
        );
    }

    #[sqlx::test]
    async fn test_alter_table_change_table_name(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a table change event to change the table name
        table_info
            .table_change_events
            .push(BTableChangeEvents::ChangeTableName(String::from(
                "accounts",
            )));

        // Alter the table based on the events
        table_info.alter_table().await;

        // Assertions
        assert_eq!(table_info.table_name, "accounts");
        assert!(table_info.columns_info.len() > 0);
    }

    #[sqlx::test]
    async fn test_alter_table_change_column_name(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a table change event to change a column name
        table_info
            .table_change_events
            .push(BTableChangeEvents::ChangeColumnName(
                String::from("name"),
                String::from("username"),
            ));

        // Alter the table based on the events
        table_info.alter_table().await;

        // Assertions
        let expected_columns = vec![BColumn {
            name: String::from("username"),
            datatype: BDataType::TEXT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_alter_table_change_column_datatype(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("age"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a table change event to change the column datatype
        table_info
            .table_change_events
            .push(BTableChangeEvents::ChangeColumnDataType(
                String::from("age"),
                BDataType::INT,
            ));

        // Alter the table based on the events
        table_info.alter_table().await;

        // Assertions
        let expected_columns = vec![BColumn {
            name: String::from("age"),
            datatype: BDataType::INT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }
    #[cfg(test)]
    mod tests {
        use super::*;
        use sqlx::PgPool;

        /// Helper function to create a `TableInfo` instance.
        async fn create_table_info(pool: PgPool, table_in: &BTableIn) -> TableInfo {
            let repository = BRepository::new(Some(pool)).await;
            repository.create_table(table_in).await;
            let mut table_info = TableInfo::new(repository, table_in.table_name.clone());
            table_info.set_table_info(&table_in.table_name).await;
            table_info
        }

        #[sqlx::test]
        async fn test_initialize_component(pool: PgPool) {
            let table_in = BTableIn {
                table_name: String::from("users"),
                columns: vec![BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                }],
            };

            let mut table_info = create_table_info(pool, &table_in).await;

            // Call initialize_component to set columns_info
            table_info.initialize_component().await;

            // Assertions
            assert_eq!(table_info.table_name, table_in.table_name);
            assert_eq!(
                table_info.columns_info,
                vec![BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                }]
            );
        }

        #[sqlx::test]
        async fn test_alter_table_change_table_name(pool: PgPool) {
            let table_in = BTableIn {
                table_name: String::from("users"),
                columns: vec![BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                }],
            };

            let mut table_info = create_table_info(pool, &table_in).await;

            // Add a change event to change the table name
            table_info.add_table_change_event(BTableChangeEvents::ChangeTableName(String::from(
                "accounts",
            )));

            // Alter the table based on the events
            table_info.alter_table().await;

            // Assertions
            assert_eq!(table_info.table_name, "accounts");
            assert!(table_info.columns_info.len() > 0);
        }

        #[sqlx::test]
        async fn test_alter_table_change_column_name(pool: PgPool) {
            let table_in = BTableIn {
                table_name: String::from("users"),
                columns: vec![BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                }],
            };

            let mut table_info = create_table_info(pool, &table_in).await;

            // Add a change event to change the column name
            table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
                String::from("name"),
                String::from("username"),
            ));

            // Alter the table based on the events
            table_info.alter_table().await;

            // Assertions
            let expected_columns = vec![BColumn {
                name: String::from("username"),
                datatype: BDataType::TEXT,
            }];
            assert_eq!(table_info.columns_info, expected_columns);
        }

        #[sqlx::test]
        async fn test_alter_table_change_column_datatype(pool: PgPool) {
            let table_in = BTableIn {
                table_name: String::from("users"),
                columns: vec![BColumn {
                    name: String::from("age"),
                    datatype: BDataType::TEXT,
                }],
            };

            let mut table_info = create_table_info(pool, &table_in).await;

            // Add a change event to change the column datatype
            table_info.add_table_change_event(BTableChangeEvents::ChangeColumnDataType(
                String::from("age"),
                BDataType::INT,
            ));

            // Alter the table based on the events
            table_info.alter_table().await;

            // Assertions
            let expected_columns = vec![BColumn {
                name: String::from("age"),
                datatype: BDataType::INT,
            }];
            assert_eq!(table_info.columns_info, expected_columns);
        }

        #[sqlx::test]
        async fn test_multiple_events(pool: PgPool) {
            let table_in = BTableIn {
                table_name: String::from("users"),
                columns: vec![
                    BColumn {
                        name: String::from("name"),
                        datatype: BDataType::TEXT,
                    },
                    BColumn {
                        name: String::from("age"),
                        datatype: BDataType::TEXT,
                    },
                ],
            };

            let mut table_info = create_table_info(pool, &table_in).await;

            // Add multiple change events
            table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
                String::from("name"),
                String::from("username"),
            ));
            table_info.add_table_change_event(BTableChangeEvents::ChangeColumnDataType(
                String::from("age"),
                BDataType::INT,
            ));
            table_info.add_table_change_event(BTableChangeEvents::ChangeTableName(String::from(
                "accounts",
            )));

            // Alter the table based on the events
            table_info.alter_table().await;

            // Assertions
            assert_eq!(table_info.table_name, "accounts");

            let mut actual_columns = table_info.columns_info;
            let mut expected_columns = vec![
                BColumn {
                    name: String::from("username"),
                    datatype: BDataType::TEXT,
                },
                BColumn {
                    name: String::from("age"),
                    datatype: BDataType::INT,
                },
            ];

            // Sort both vectors before comparing
            actual_columns.sort_by(|a, b| a.name.cmp(&b.name));
            expected_columns.sort_by(|a, b| a.name.cmp(&b.name));

            assert_eq!(actual_columns, expected_columns);
        }
    }
}
