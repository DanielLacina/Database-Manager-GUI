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
            //println!("found remove column event");
            if let BTableChangeEvents::RemoveColumn(original_column_name) =
                &self.table_change_events[existing_remove_column_event_index]
            {
                //println!("found column name that was removed");
                if let Some(original_column) = self
                    .columns_info
                    .iter()
                    .find(|&column| column.name == *original_column_name)
                {
                    //  println!("found column datatype");
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
        if let Some(existing_event_index) =
            self.find_existing_change_data_type_column_event(&column_name)
        {
            if let BTableChangeEvents::ChangeColumnDataType(existing_column_name, data_type) =
                table_change_event.clone()
            {
                if let Some(column) = self
                    .columns_info
                    .iter()
                    .find(|&column| column.name == column_name)
                {
                    if column.datatype != data_type {
                        self.table_change_events.remove(existing_event_index);
                        self.table_change_events.push(table_change_event);
                    } else {
                        self.table_change_events.remove(existing_event_index);
                    }
                } else {
                    self.table_change_events.remove(existing_event_index);
                    self.table_change_events.push(table_change_event);
                }
            }
        } else if let Some(existing_event_index) = self.find_existing_add_column_event(column_name)
        {
            if let BTableChangeEvents::AddColumn(add_column_name, add_data_type) =
                &self.table_change_events[existing_event_index]
            {
                if let BTableChangeEvents::ChangeColumnDataType(column_name, data_type_to_update) =
                    table_change_event.clone()
                {
                    if *add_data_type != data_type_to_update {
                        self.table_change_events.remove(existing_event_index);
                        self.table_change_events.push(BTableChangeEvents::AddColumn(
                            column_name,
                            data_type_to_update,
                        ));
                    }
                }
            }
        } else {
            self.table_change_events.push(table_change_event);
        }
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
                // column name should be the same as the original column name
                // in the actual database
                //  println!("{}", new_column_name.to_string());
                if let Some(column) = self
                    .columns_info
                    .iter()
                    .find(|&column| column.name == new_column_name)
                {
                    if column.datatype != data_type {
                        self.table_change_events
                            .push(BTableChangeEvents::ChangeColumnDataType(
                                new_column_name.to_string(),
                                data_type.clone(),
                            ));
                    }
                } else {
                    self.table_change_events
                        .push(BTableChangeEvents::ChangeColumnDataType(
                            new_column_name.to_string(),
                            data_type.clone(),
                        ));
                }
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

    fn default_table_in() -> BTableIn {
        BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
            }],
        }
    }

    async fn initialized_table_info(pool: PgPool, table_in: &BTableIn) -> TableInfo {
        // default configuration to prevent redundant code
        let mut table_info = create_table_info(pool, &table_in).await;
        table_info.initialize_component().await;
        table_info
    }

    #[sqlx::test]
    async fn test_initialize_component(pool: PgPool) {
        let table_in = default_table_in();
        let mut table_info = initialized_table_info(pool, &table_in).await;

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
    async fn test_alter_table(pool: PgPool) {
        let table_in = default_table_in();
        let mut table_info = initialized_table_info(pool, &table_in).await;
        let table_change_events = vec![
            // 1. Add a new column "email" with datatype TEXT
            BTableChangeEvents::AddColumn(String::from("email"), BDataType::TEXT),
            // 2. Rename the existing column "name" to "username"
            BTableChangeEvents::ChangeColumnName(String::from("name"), String::from("username")),
            // 3. Change datatype of the column "username" from TEXT to INT
            BTableChangeEvents::ChangeColumnDataType(String::from("username"), BDataType::INT),
            // 4. Add a new column "age" with datatype INT
            BTableChangeEvents::AddColumn(String::from("age"), BDataType::INT),
            // 5. Remove the column "age"
            BTableChangeEvents::RemoveColumn(String::from("age")),
            // 6. Change the table name from "users" to "customers"
            BTableChangeEvents::ChangeTableName(String::from("customers")),
            // 7. Add a new column "created_at" with datatype TIMESTAMP
            BTableChangeEvents::AddColumn(String::from("created_at"), BDataType::TIMESTAMP),
            // 8. Change the datatype of the "email" column from TEXT to TEXT
            BTableChangeEvents::ChangeColumnDataType(String::from("email"), BDataType::TEXT),
            // 9. Rename the column "created_at" to "registration_date"
            BTableChangeEvents::ChangeColumnName(
                String::from("created_at"),
                String::from("registration_date"),
            ),
            // 10. Remove the column "registration_date"
            BTableChangeEvents::RemoveColumn(String::from("registration_date")),
            // 11. Add a new column "is_active" with datatype BOOLEAN
            BTableChangeEvents::AddColumn(String::from("is_active"), BDataType::TEXT),
            // 12. Rename the column "is_active" to "active_status"
            BTableChangeEvents::ChangeColumnName(
                String::from("is_active"),
                String::from("active_status"),
            ),
            // 13. Add a new column "last_login" with datatype TIMESTAMP
            BTableChangeEvents::AddColumn(String::from("last_login"), BDataType::TIMESTAMP),
            // 14. Change the datatype of "last_login" to DATE
            BTableChangeEvents::ChangeColumnDataType(
                String::from("last_login"),
                BDataType::TIMESTAMP,
            ),
            // 15. Add a new column "country" with datatype TEXT
            BTableChangeEvents::AddColumn(String::from("country"), BDataType::TEXT),
            // 16. Change the table name from "customers" to "clients"
            BTableChangeEvents::ChangeTableName(String::from("clients")),
            // 17. Add a new column "phone_number" with datatype TEXT
            BTableChangeEvents::AddColumn(String::from("phone_number"), BDataType::TEXT),
            // 18. Remove the column "phone_number"
            BTableChangeEvents::RemoveColumn(String::from("phone_number")),
            // 19. Change the datatype of the column "country" from TEXT to TEXT
            BTableChangeEvents::ChangeColumnDataType(String::from("country"), BDataType::TEXT),
            // 20. Rename the column "username" back to "name"
            BTableChangeEvents::ChangeColumnName(String::from("username"), String::from("name")),
        ];
        for event in table_change_events {
            table_info.add_table_change_event(event);
        }
        let expected_events = vec![
            BTableChangeEvents::AddColumn(String::from("email"), BDataType::TEXT),
            BTableChangeEvents::AddColumn(String::from("active_status"), BDataType::TEXT),
            BTableChangeEvents::AddColumn(String::from("last_login"), BDataType::TIMESTAMP),
            BTableChangeEvents::AddColumn(String::from("country"), BDataType::TEXT),
            BTableChangeEvents::ChangeTableName(String::from("clients")),
            BTableChangeEvents::ChangeColumnDataType(String::from("name"), BDataType::INT),
        ];
        assert_eq!(table_info.table_change_events, expected_events);
    }
}
