use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnsInfo, BConstraint, BDataType, BTable,
    BTableChangeEvents, BTableIn, BusinessComponent,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct TableInfo {
    repository: Arc<BRepository>,
    pub table_name: String,
    pub columns_info: Vec<BColumn>,
    table_change_events: Vec<BTableChangeEvents>,
}

impl BusinessComponent for TableInfo {
    async fn initialize_component(&mut self) {
        self.set_table_info().await;
    }
}

impl TableInfo {
    pub fn new(repository: Arc<BRepository>, table_name: String) -> Self {
        Self {
            repository,
            table_name,
            columns_info: vec![],
            table_change_events: vec![],
        }
    }

    pub fn get_table_change_events(&self) -> Vec<BTableChangeEvents> {
        self.table_change_events.clone()
    }

    async fn set_table_info(&mut self) {
        let columns_info = self
            .repository
            .get_columns_info(&self.table_name)
            .await
            .unwrap();
        let columns_info_with_enums = columns_info
            .into_iter()
            .map(|column_info| BColumn::to_column(column_info))
            .collect();
        self.columns_info = columns_info_with_enums;
    }

    pub fn add_table_change_event(&mut self, table_change_event: BTableChangeEvents) {
        match table_change_event {
            BTableChangeEvents::ChangeTableName(new_table_name) => {
                self.handle_change_table_name(new_table_name);
            }
            BTableChangeEvents::ChangeColumnDataType(column_name, data_type) => {
                self.handle_change_column_datatype(column_name, data_type);
            }
            BTableChangeEvents::ChangeColumnName(column_name, new_column_name) => {
                self.handle_change_column_name(column_name, new_column_name);
            }
            BTableChangeEvents::RemoveColumn(column_name) => {
                self.handle_remove_column(column_name);
            }
            BTableChangeEvents::AddColumn(column_name, data_type) => {
                self.handle_add_column(column_name, data_type);
            }
            BTableChangeEvents::AddForeignKey(column_name, referenced_table, referenced_column) => {
                self.handle_add_foreign_key(column_name, referenced_table, referenced_column);
            }
            BTableChangeEvents::RemoveForeignKey(column_name) => {
                self.handle_remove_foreign_key(column_name);
            }
            BTableChangeEvents::AddPrimaryKey(column_name) => {
                self.handle_add_primary_key(column_name);
            }
            BTableChangeEvents::RemovePrimaryKey(column_name) => {
                self.handle_remove_primary_key(column_name);
            }
        }
        println!("{:?}", self.table_change_events);
    }

    fn handle_add_column(&mut self, column_name: String, data_type: BDataType) {
        if let Some(existing_event_index) = self.find_existing_remove_column_event(&column_name) {
            if let BTableChangeEvents::RemoveColumn(original_column_name) =
                &self.table_change_events[existing_event_index]
            {
                if let Some(original_column) = self
                    .columns_info
                    .iter()
                    .find(|&column| column.name == *original_column_name)
                {
                    if data_type == original_column.datatype {
                        self.table_change_events.remove(existing_event_index);
                    } else {
                        self.table_change_events.remove(existing_event_index);
                        self.table_change_events
                            .push(BTableChangeEvents::ChangeColumnDataType(
                                column_name,
                                data_type,
                            ));
                    }
                }
            }
        } else {
            self.table_change_events
                .push(BTableChangeEvents::AddColumn(column_name, data_type));
        }
    }

    fn handle_change_table_name(&mut self, table_name: String) {
        if let Some(existing_event_index) = self.find_existing_change_table_name_event() {
            if table_name == self.table_name {
                self.table_change_events.remove(existing_event_index);
            } else {
                self.table_change_events.remove(existing_event_index);
                self.table_change_events
                    .push(BTableChangeEvents::ChangeTableName(table_name));
            }
        } else {
            self.table_change_events
                .push(BTableChangeEvents::ChangeTableName(table_name));
        }
    }

    fn handle_change_column_datatype(&mut self, column_name: String, data_type: BDataType) {
        if let Some(existing_event_index) =
            self.find_existing_change_data_type_column_event(&column_name)
        {
            if let Some(column) = self
                .columns_info
                .iter()
                .find(|&column| column.name == column_name)
            {
                if column.datatype == data_type {
                    self.table_change_events.remove(existing_event_index);
                } else {
                    self.table_change_events.remove(existing_event_index);
                    self.table_change_events
                        .push(BTableChangeEvents::ChangeColumnDataType(
                            column_name,
                            data_type,
                        ));
                }
            } else {
                self.table_change_events.remove(existing_event_index);
                self.table_change_events
                    .push(BTableChangeEvents::ChangeColumnDataType(
                        column_name,
                        data_type,
                    ));
            }
        } else if let Some(existing_event_index) = self.find_existing_add_column_event(&column_name)
        {
            if let BTableChangeEvents::AddColumn(_, added_column_data_type) =
                &self.table_change_events[existing_event_index]
            {
                if *added_column_data_type != data_type {
                    self.table_change_events.remove(existing_event_index);
                    self.table_change_events
                        .push(BTableChangeEvents::AddColumn(column_name, data_type));
                }
            }
        } else {
            self.table_change_events
                .push(BTableChangeEvents::ChangeColumnDataType(
                    column_name,
                    data_type,
                ));
        }
    }

    fn handle_change_column_name(&mut self, column_name: String, new_column_name: String) {
        self.rename_existing_datatype_change_event(&column_name, &new_column_name);

        if column_name == new_column_name {
            return;
        } else if let Some(existing_event_index) =
            self.find_existing_rename_column_event(&column_name)
        {
            self.update_existing_rename_event(existing_event_index, new_column_name);
        } else if let Some(existing_event_index) = self.find_existing_add_column_event(&column_name)
        {
            self.update_existing_add_column_event(
                existing_event_index,
                column_name,
                new_column_name,
            );
        } else {
            self.table_change_events
                .push(BTableChangeEvents::ChangeColumnName(
                    column_name,
                    new_column_name,
                ));
        }
    }

    fn handle_remove_column(&mut self, column_name: String) {
        if let Some(existing_event_index) = self.find_existing_add_primary_key_event(&column_name) {
            self.table_change_events.remove(existing_event_index);
        }
        if let Some(existing_event_index) = self.find_existing_add_foreign_key_event(&column_name) {
            self.table_change_events.remove(existing_event_index);
        }
        if let Some(existing_event_index) = self.find_existing_add_column_event(&column_name) {
            self.table_change_events.remove(existing_event_index);
        } else if let Some(existing_event_index) =
            self.find_existing_change_data_type_column_event(&column_name)
        {
            self.table_change_events.remove(existing_event_index);
            self.table_change_events
                .push(BTableChangeEvents::RemoveColumn(column_name));
        } else if let Some(existing_event_index) =
            self.find_existing_rename_column_event(&column_name)
        {
            if let BTableChangeEvents::ChangeColumnName(
                original_column_name,
                modified_column_name,
            ) = self.table_change_events[existing_event_index].clone()
            {
                self.table_change_events.remove(existing_event_index);
                self.table_change_events
                    .push(BTableChangeEvents::RemoveColumn(original_column_name));
            }
        } else {
            self.table_change_events
                .push(BTableChangeEvents::RemoveColumn(column_name));
        }
    }

    fn handle_add_primary_key(&mut self, column_name: String) {
        if let Some(existing_event_index) =
            self.find_existing_remove_primary_key_event(&column_name)
        {
            self.table_change_events.remove(existing_event_index);
        } else {
            self.table_change_events
                .push(BTableChangeEvents::AddPrimaryKey(column_name));
        }
    }

    fn handle_remove_primary_key(&mut self, column_name: String) {
        if let Some(existing_event_index) = self.find_existing_add_primary_key_event(&column_name) {
            self.table_change_events.remove(existing_event_index);
        } else {
            self.table_change_events
                .push(BTableChangeEvents::RemovePrimaryKey(column_name));
        }
    }

    fn handle_add_foreign_key(
        &mut self,
        column_name: String,
        referenced_table: String,
        referenced_column: String,
    ) {
        if let Some(existing_event_index) =
            self.find_existing_remove_foreign_key_event(&column_name)
        {
            self.table_change_events.remove(existing_event_index);
        } else {
            self.table_change_events
                .push(BTableChangeEvents::AddForeignKey(
                    column_name,
                    referenced_table,
                    referenced_column,
                ));
        }
    }

    fn handle_remove_foreign_key(&mut self, column_name: String) {
        if let Some(existing_event_index) = self.find_existing_add_foreign_key_event(&column_name) {
            self.table_change_events.remove(existing_event_index);
        } else {
            self.table_change_events
                .push(BTableChangeEvents::RemoveForeignKey(column_name));
        }
    }

    fn update_existing_rename_event(&mut self, event_index: usize, new_column_name: String) {
        if let BTableChangeEvents::ChangeColumnName(original_column_name, _) =
            self.table_change_events[event_index].clone()
        {
            if original_column_name != new_column_name {
                self.table_change_events
                    .push(BTableChangeEvents::ChangeColumnName(
                        original_column_name,
                        new_column_name,
                    ));
            }
        }
        self.table_change_events.remove(event_index);
    }

    fn update_existing_add_column_event(
        &mut self,
        event_index: usize,
        column_name: String,
        new_column_name: String,
    ) {
        if let BTableChangeEvents::AddColumn(_, added_data_type) =
            self.table_change_events[event_index].clone()
        {
            self.table_change_events.remove(event_index);
            self.handle_add_column(new_column_name, added_data_type);
        }
    }

    fn rename_existing_datatype_change_event(&mut self, column_name: &str, new_column_name: &str) {
        if let Some(event_index) = self.find_existing_change_data_type_column_event(column_name) {
            if let BTableChangeEvents::ChangeColumnDataType(original_column_name, data_type) =
                self.table_change_events[event_index].clone()
            {
                self.table_change_events.remove(event_index);
                self.table_change_events
                    .push(BTableChangeEvents::ChangeColumnDataType(
                        new_column_name.to_string(),
                        data_type,
                    ));
            }
        }
    }

    fn find_existing_remove_primary_key_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::RemovePrimaryKey(existing_column_name)
                if existing_column_name == column_name)
        })
    }

    fn find_existing_add_primary_key_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::AddPrimaryKey(existing_column_name)
                if existing_column_name == column_name)
        })
    }

    fn find_existing_add_foreign_key_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::AddForeignKey(existing_column_name, _, _)
                if existing_column_name == column_name)
        })
    }

    fn find_existing_remove_foreign_key_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::RemoveForeignKey(existing_column_name)
                if existing_column_name == column_name)
        })
    }

    fn find_existing_rename_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::ChangeColumnName(_, modified_column_name)
                if modified_column_name == column_name)
        })
    }

    fn find_existing_remove_column_event(&self, column_name: &str) -> Option<usize> {
        self.table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::RemoveColumn(existing_column_name)
                if existing_column_name == column_name)
        })
    }

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

    fn find_existing_change_table_name_event(&self) -> Option<usize> {
        self.table_change_events
            .iter()
            .position(|event| matches!(event, BTableChangeEvents::ChangeTableName(_)))
    }

    pub async fn alter_table(&mut self) {
        if !self.table_change_events.is_empty() {
            let res = self
                .repository
                .alter_table(&self.table_name, &self.table_change_events)
                .await;
            println!("Alter table result: {:?}", res);
        }

        for event in &self.table_change_events {
            if let BTableChangeEvents::ChangeTableName(updated_table_name) = event {
                self.table_name = updated_table_name.clone();
            }
        }

        self.table_change_events.clear();
        self.set_table_info().await;
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
        let mut table_info = TableInfo::new(repository.into(), table_in.table_name.clone());
        table_info.set_table_info().await;
        table_info
    }

    fn default_table_in() -> BTableIn {
        BTableIn {
            table_name: String::from("users"),
            columns: vec![BColumn {
                name: String::from("name"),
                datatype: BDataType::TEXT,
                constraints: vec![],
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
                constraints: vec![],
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
            BTableChangeEvents::ChangeColumnDataType(String::from("username"), BDataType::INTEGER),
            // 4. Add a new column "age" with datatype INT
            BTableChangeEvents::AddColumn(String::from("age"), BDataType::INTEGER),
            // 5. Remove the column "age"
            BTableChangeEvents::RemoveColumn(String::from("age")),
            // 6. Change the table name from "users" to "customers"
            BTableChangeEvents::ChangeTableName(String::from("customers")),
            // 7. Add a new column "created_at" with datatype TIMESTAMP
            BTableChangeEvents::AddColumn(String::from("created_at"), BDataType::TIMESTAMP),
            // 8. Change the datatype of the "email" column from TEXT to TEXT
            BTableChangeEvents::ChangeColumnDataType(String::from("email"), BDataType::TEXT),
            // 9. Rename the column "created_at" to "regiStringation_date"
            BTableChangeEvents::ChangeColumnName(
                String::from("created_at"),
                String::from("registration_date"),
            ),
            // 10. Remove the column "regiStringation_date"
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
            BTableChangeEvents::AddPrimaryKey(String::from("phone_number")),
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
            BTableChangeEvents::ChangeColumnDataType(String::from("name"), BDataType::INTEGER),
        ];
        assert_eq!(table_info.table_change_events, expected_events);
        table_info.alter_table().await;
        let mut expected_columns = vec![
            BColumn {
                constraints: vec![],
                name: String::from("name"),
                datatype: BDataType::INTEGER,
            },
            BColumn {
                constraints: vec![],
                name: String::from("email"),
                datatype: BDataType::TEXT,
            },
            BColumn {
                constraints: vec![],
                name: String::from("active_status"),
                datatype: BDataType::TEXT,
            },
            BColumn {
                constraints: vec![],
                name: String::from("last_login"),
                datatype: BDataType::TIMESTAMP,
            },
            BColumn {
                constraints: vec![],
                name: String::from("country"),
                datatype: BDataType::TEXT,
            },
        ];
        expected_columns.sort_by(|a, b| b.name.cmp(&a.name));
        table_info.columns_info.sort_by(|a, b| b.name.cmp(&a.name));
        let expected_table_name = String::from("clients");
        assert!(table_info.table_change_events.is_empty());
        assert_eq!(table_info.columns_info, expected_columns);
        assert_eq!(table_info.table_name, expected_table_name);
    }
}
