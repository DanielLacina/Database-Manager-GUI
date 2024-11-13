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

    // Public function to add a table change event
    pub fn add_table_change_event(&mut self, table_change_event: BTableChangeEvents) {
        match &table_change_event {
            BTableChangeEvents::ChangeTableName(new_table_name) => {
                self.handle_change_table_name(table_change_event.clone());
            }
            BTableChangeEvents::ChangeColumnDataType(column_name, _) => {
                self.handle_change_column_datatype(table_change_event.clone(), column_name);
            }
            BTableChangeEvents::ChangeColumnName(column_name, new_column_name) => {
                self.handle_change_column_name(
                    table_change_event.clone(),
                    column_name,
                    new_column_name,
                );
            }
            BTableChangeEvents::RemoveColumn(column_name) => {
                self.handle_remove_column(table_change_event.clone(), column_name);
            }
            _ => {
                self.table_change_events.push(table_change_event.clone());
            }
        }
        println!("{:?}", self.table_change_events);
    }

    // Private helper function for handling ChangeTableName
    fn handle_change_table_name(&mut self, table_change_event: BTableChangeEvents) {
        self.table_change_events
            .retain(|event| !matches!(event, BTableChangeEvents::ChangeTableName(_)));
        self.table_change_events.push(table_change_event);
    }

    // Private helper function for handling ChangeColumnDataType
    fn handle_change_column_datatype(
        &mut self,
        table_change_event: BTableChangeEvents,
        column_name: &str,
    ) {
        self.table_change_events.retain(|event| {
            !matches!(event, BTableChangeEvents::ChangeColumnDataType(existing_column_name, _)
                if existing_column_name == column_name)
        });
        self.table_change_events.push(table_change_event);
    }

    // Private helper function for handling ChangeColumnName
    fn handle_change_column_name(
        &mut self,
        mut table_change_event: BTableChangeEvents,
        column_name: &str,
        new_column_name: &str,
    ) {
        if let Some(existing_event_index) = self.find_existing_change_column_event(column_name) {
            println!("found column name that was the result of another column name change");
            if let BTableChangeEvents::ChangeColumnName(
                original_column_name,
                modified_column_name,
            ) = &self.table_change_events[existing_event_index]
            {
                table_change_event = BTableChangeEvents::ChangeColumnName(
                    original_column_name.clone(),
                    new_column_name.to_string(),
                );
            }
            self.table_change_events.remove(existing_event_index);
        } else if let Some(existing_event_index) = self.find_existing_add_column_event(column_name)
        {
            if let BTableChangeEvents::AddColumn(_, added_data_type) =
                &self.table_change_events[existing_event_index]
            {
                table_change_event = BTableChangeEvents::AddColumn(
                    new_column_name.to_string(),
                    added_data_type.clone(),
                );
            }
            self.table_change_events.remove(existing_event_index);
        }
        self.table_change_events.push(table_change_event);
    }

    // Private helper function for handling RemoveColumn
    fn handle_remove_column(&mut self, table_change_event: BTableChangeEvents, column_name: &str) {
        if let Some(existing_event_index) = self.find_existing_add_column_event(column_name) {
            self.table_change_events.remove(existing_event_index);
        } else {
            self.table_change_events.push(table_change_event);
        }
    }

    // Utility function to find an existing ChangeColumnName event
    fn find_existing_change_column_event(&self, column_name: &str) -> Option<usize> {
        /* checks if the column name thats name is being changed wasnt the result of another column
         * name being changed*/
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::ChangeColumnName(original_column_name, modified_column_name)
                if modified_column_name == column_name)
        })
    }

    // Utility function to find an existing AddColumn event
    fn find_existing_add_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::AddColumn(existing_column_name, _)
                if existing_column_name == column_name)
        })
    }

    // Method to apply the stored change events to the table
    pub async fn alter_table(&mut self) {
        if !self.table_change_events.is_empty() {
            let res = self
                .repository
                .alter_table(&self.table_name, &self.table_change_events)
                .await;
            println!("Alter table result: {:?}", res);
        }

        let mut new_table_name = self.table_name.clone();
        for event in &self.table_change_events {
            if let BTableChangeEvents::ChangeTableName(updated_name) = event {
                new_table_name = updated_name.clone();
            }
        }

        self.table_change_events.clear();
        self.set_table_info(&new_table_name).await;
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
