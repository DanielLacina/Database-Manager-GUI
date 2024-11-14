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
            BTableChangeEvents::AddColumn(column_name, data_type) => {
                self.handle_add_column(table_change_event.clone(), column_name, data_type);
            }
            _ => {
                self.table_change_events.push(table_change_event.clone());
            }
        }
        println!("{:?}", self.table_change_events);
    }

    fn handle_add_column(
        &mut self,
        table_change_event: BTableChangeEvents,
        column_name: &str,
        data_type: &BDataType,
    ) {
        if let Some(existing_remove_column_event_index) =
            self.find_existing_remove_column_event(column_name)
        {
            println!("found remove column event");
            if let BTableChangeEvents::RemoveColumn(original_column_name) =
                &self.table_change_events[existing_remove_column_event_index]
            {
                println!("found column name that was removed");
                if let Some(original_column) = self
                    .columns_info
                    .iter()
                    .find(|&column| column.name == *original_column_name)
                {
                    println!("found column datatype");
                    if *data_type != original_column.datatype {
                        self.table_change_events
                            .remove(existing_remove_column_event_index);
                        self.table_change_events
                            .push(BTableChangeEvents::ChangeColumnDataType(
                                column_name.to_string(),
                                data_type.clone(),
                            ));
                    } else {
                        self.table_change_events
                            .remove(existing_remove_column_event_index);
                    }
                }
            }
        } else {
            self.table_change_events.push(table_change_event);
        }
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

    fn handle_change_column_name(
        &mut self,
        mut table_change_event: BTableChangeEvents,
        column_name: &str,
        new_column_name: &str,
    ) {
        if column_name == new_column_name {
            return;
        }

        // Handle existing rename event
        if let Some(existing_event_index) = self.find_existing_rename_column_event(column_name) {
            self.update_existing_rename_event(
                existing_event_index,
                &mut table_change_event,
                new_column_name,
            );
        }
        // Handle existing add column event
        else if let Some(existing_add_column_event_index) =
            self.find_existing_add_column_event(column_name)
        {
            self.update_existing_add_column_event(
                existing_add_column_event_index,
                &mut table_change_event,
                column_name,
                new_column_name,
            );
        } else {
            // If no existing events, add a new change event
            self.table_change_events.push(table_change_event);
        }
        // Check if there is an existing datatype change event for this column
        self.update_existing_datatype_change_event(column_name, new_column_name);
    }

    /// Updates an existing rename column event if necessary.
    fn update_existing_rename_event(
        &mut self,
        event_index: usize,
        table_change_event: &mut BTableChangeEvents,
        new_column_name: &str,
    ) {
        if let BTableChangeEvents::ChangeColumnName(original_column_name, _) =
            &self.table_change_events[event_index]
        {
            if original_column_name != new_column_name {
                *table_change_event = BTableChangeEvents::ChangeColumnName(
                    original_column_name.clone(),
                    new_column_name.to_string(),
                );
                self.table_change_events.push(table_change_event.clone());
            }
        }
        self.table_change_events.remove(event_index);
    }

    /// Converts an existing add column event to handle renaming after addition.
    fn update_existing_add_column_event(
        &mut self,
        event_index: usize,
        table_change_event: &mut BTableChangeEvents,
        column_name: &str,
        new_column_name: &str,
    ) {
        if let BTableChangeEvents::AddColumn(_, added_data_type) =
            self.table_change_events[event_index].clone()
        {
            // Remove the old add column event
            self.table_change_events.remove(event_index);

            // Convert the event to a new AddColumn event
            *table_change_event =
                BTableChangeEvents::AddColumn(new_column_name.to_string(), added_data_type.clone());

            // Handle adding the column
            self.handle_add_column(
                table_change_event.clone(),
                new_column_name,
                &added_data_type,
            );
        }
    }

    /// Updates an existing change column data type event if necessary.
    fn update_existing_datatype_change_event(&mut self, column_name: &str, new_column_name: &str) {
        if let Some(event_index) = self.find_existing_change_data_type_column_event(column_name) {
            if let BTableChangeEvents::ChangeColumnDataType(original_column_name, data_type) =
                self.table_change_events[event_index].clone()
            {
                // Remove the old datatype change event
                self.table_change_events.remove(event_index);

                // Add a new ChangeColumnDataType event with the new column name
                println!("{}", new_column_name.to_string());
                self.table_change_events
                    .push(BTableChangeEvents::ChangeColumnDataType(
                        new_column_name.to_string(),
                        data_type.clone(),
                    ));
            }
        }
    }

    // Private helper function for handling RemoveColumn
    fn handle_remove_column(
        &mut self,
        mut table_change_event: BTableChangeEvents,
        column_name: &str,
    ) {
        if let Some(existing_add_column_event_index) =
            self.find_existing_add_column_event(column_name)
        {
            /* checks if a column has been added to the table and was not already there so it can
             * counter out that event to get rid of redundant events */
            self.table_change_events
                .remove(existing_add_column_event_index);
        } else if let Some(existing_change_column_data_type_event_index) =
            self.find_existing_change_data_type_column_event(column_name)
        {
            /* checks if a column has been added to the table and was not already there so it can
             * counter out that event to get rid of redundant events */
            self.table_change_events
                .remove(existing_change_column_data_type_event_index);
            self.table_change_events.push(table_change_event);
        } else if let Some(existing_rename_column_event_index) =
            self.find_existing_rename_column_event(column_name)
        {
            if let BTableChangeEvents::ChangeColumnName(
                original_column_name,
                modified_column_name,
            ) = &self.table_change_events[existing_rename_column_event_index]
            {
                table_change_event =
                    BTableChangeEvents::RemoveColumn(original_column_name.to_string());
                self.table_change_events
                    .remove(existing_rename_column_event_index);
                self.table_change_events.push(table_change_event);
            }
        } else {
            self.table_change_events.push(table_change_event);
        }
    }

    // Utility function to find an existing ChangeColumnName event
    fn find_existing_rename_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::ChangeColumnName(original_column_name, modified_column_name)
                if modified_column_name == column_name)
        })
    }

    fn find_existing_remove_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::RemoveColumn(original_column_name)
                if original_column_name == column_name)
        })
    }

    // Utility function to find an existing AddColumn event
    fn find_existing_add_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::AddColumn(existing_column_name, _)
                if existing_column_name == column_name)
        })
    }

    fn find_existing_change_data_type_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::ChangeColumnDataType(existing_column_name, _)
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
        table_info.initialize_component().await;

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
    async fn test_add_remove_column(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a column
        table_info.add_table_change_event(BTableChangeEvents::AddColumn(
            String::from("email"),
            BDataType::TEXT,
        ));
        table_info.alter_table().await;

        assert!(table_info
            .columns_info
            .iter()
            .any(|col| col.name == "email"));

        // Remove the column
        table_info.add_table_change_event(BTableChangeEvents::RemoveColumn(String::from("email")));
        table_info.alter_table().await;

        assert!(!table_info
            .columns_info
            .iter()
            .any(|col| col.name == "email"));
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

        // Change column name
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("name"),
            String::from("username"),
        ));
        table_info.alter_table().await;

        let expected_columns = vec![BColumn {
            name: String::from("username"),
            datatype: BDataType::TEXT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_handle_multiple_column_changes(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![
                BColumn {
                    name: String::from("id"),
                    datatype: BDataType::INT,
                },
                BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                },
            ],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a column
        table_info.add_table_change_event(BTableChangeEvents::AddColumn(
            String::from("email"),
            BDataType::TEXT,
        ));

        // Rename an existing column
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("name"),
            String::from("username"),
        ));

        // Change the datatype of an existing column
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnDataType(
            String::from("id"),
            BDataType::TEXT,
        ));

        table_info.alter_table().await;

        let mut expected_columns = vec![
            BColumn {
                name: String::from("id"),
                datatype: BDataType::TEXT,
            },
            BColumn {
                name: String::from("username"),
                datatype: BDataType::TEXT,
            },
            BColumn {
                name: String::from("email"),
                datatype: BDataType::TEXT,
            },
        ];

        let mut actual_columns = table_info.columns_info;
        actual_columns.sort_by(|a, b| a.name.cmp(&b.name));
        expected_columns.sort_by(|a, b| a.name.cmp(&b.name));

        assert_eq!(actual_columns, expected_columns);
    }

    #[sqlx::test]
    async fn test_chain_column_name_changes(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Change column name from "name" to "username"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("name"),
            String::from("username"),
        ));

        // Change column name from "username" to "user_id"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("username"),
            String::from("user_id"),
        ));

        table_info.alter_table().await;

        let expected_columns = vec![BColumn {
            name: String::from("user_id"),
            datatype: BDataType::TEXT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_remove_non_existent_column(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Attempt to remove a non-existent column
        table_info.add_table_change_event(BTableChangeEvents::RemoveColumn(String::from("email")));
        table_info.alter_table().await;

        // Ensure existing columns remain unaffected
        assert!(table_info.columns_info.iter().any(|col| col.name == "name"));
    }
    #[sqlx::test]
    async fn test_multiple_rename_and_remove_events(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![
                BColumn {
                    name: String::from("id"),
                    datatype: BDataType::INT,
                },
                BColumn {
                    name: String::from("email"),
                    datatype: BDataType::TEXT,
                },
            ],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Rename "email" to "user_email"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("email"),
            String::from("user_email"),
        ));

        // Rename "user_email" to "contact_email"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("user_email"),
            String::from("contact_email"),
        ));

        // Remove the column "contact_email"
        table_info.add_table_change_event(BTableChangeEvents::RemoveColumn(String::from(
            "contact_email",
        )));

        table_info.alter_table().await;

        // Assertions: Ensure that "contact_email" is removed and "id" remains
        let expected_columns = vec![BColumn {
            name: String::from("id"),
            datatype: BDataType::INT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_multiple_table_name_changes(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("users"),
            columns: vec![
                BColumn {
                    name: String::from("id"),
                    datatype: BDataType::INT,
                },
                BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                },
            ],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add multiple table name change events
        table_info.add_table_change_event(BTableChangeEvents::ChangeTableName(String::from(
            "customers",
        )));
        table_info
            .add_table_change_event(BTableChangeEvents::ChangeTableName(String::from("clients")));

        table_info.alter_table().await;

        // Assertions: Ensure the latest change is applied
        assert_eq!(table_info.table_name, "clients");
    }

    #[sqlx::test]
    async fn test_rename_and_change_datatype(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("products"),
            columns: vec![BColumn {
                name: String::from("price"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Rename "price" to "cost"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("price"),
            String::from("cost"),
        ));

        // Change datatype of "cost" to INT
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnDataType(
            String::from("cost"),
            BDataType::INT,
        ));

        table_info.alter_table().await;

        // Assertions: Ensure column is renamed and datatype changed
        let expected_columns = vec![BColumn {
            name: String::from("cost"),
            datatype: BDataType::INT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_add_rename_and_remove_column(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("inventory"),
            columns: vec![BColumn {
                name: String::from("item"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a new column "quantity"
        table_info.add_table_change_event(BTableChangeEvents::AddColumn(
            String::from("quantity"),
            BDataType::INT,
        ));

        // Rename "quantity" to "stock"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("quantity"),
            String::from("stock"),
        ));

        // Remove "stock"
        table_info.add_table_change_event(BTableChangeEvents::RemoveColumn(String::from("stock")));

        table_info.alter_table().await;

        // Assertions: Ensure only the original column remains
        let expected_columns = vec![BColumn {
            name: String::from("item"),
            datatype: BDataType::TEXT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_add_and_immediate_remove_column(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("sales"),
            columns: vec![BColumn {
                name: String::from("sale_id"),
                datatype: BDataType::INT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Add a new column "discount"
        table_info.add_table_change_event(BTableChangeEvents::AddColumn(
            String::from("discount"),
            BDataType::INT,
        ));

        // Immediately remove the newly added column "discount"
        table_info
            .add_table_change_event(BTableChangeEvents::RemoveColumn(String::from("discount")));

        table_info.alter_table().await;

        // Assertions: Ensure the added column is not present
        let expected_columns = vec![BColumn {
            name: String::from("sale_id"),
            datatype: BDataType::INT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }

    #[sqlx::test]
    async fn test_chain_rename_back_to_original(pool: PgPool) {
        let table_in = BTableIn {
            table_name: String::from("employees"),
            columns: vec![BColumn {
                name: String::from("full_name"),
                datatype: BDataType::TEXT,
            }],
        };

        let mut table_info = create_table_info(pool, &table_in).await;

        // Rename "full_name" to "name"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("full_name"),
            String::from("name"),
        ));

        // Rename "name" back to "full_name"
        table_info.add_table_change_event(BTableChangeEvents::ChangeColumnName(
            String::from("name"),
            String::from("full_name"),
        ));

        table_info.alter_table().await;

        // Assertions: Ensure the column name is reverted to the original
        let expected_columns = vec![BColumn {
            name: String::from("full_name"),
            datatype: BDataType::TEXT,
        }];
        assert_eq!(table_info.columns_info, expected_columns);
    }
}
