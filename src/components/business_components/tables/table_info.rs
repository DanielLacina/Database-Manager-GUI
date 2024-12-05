use crate::components::business_components::component::{
    repository_module::BRepository, BColumn, BColumnForeignKey, BConstraint, BDataType,
    BTableChangeEvents, BTableGeneral, BTableIn, BusinessComponent,
};
use crate::components::business_components::components::BusinessConsole;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;
use tokio::task;

#[derive(Debug, Clone)]
pub struct TableInfo {
    repository: Arc<BRepository>,
    pub table_name: Arc<AsyncMutex<Option<String>>>,
    pub columns_info: Arc<AsyncMutex<Vec<BColumn>>>,
    pub tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    table_change_events: Arc<AsyncMutex<Vec<BTableChangeEvents>>>,
    console: Arc<BusinessConsole>,
}

impl TableInfo {
    pub fn new(
        repository: Arc<BRepository>,
        console: Arc<BusinessConsole>,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> Self {
        Self {
            repository,
            table_name: Arc::new(AsyncMutex::new(None)),
            columns_info: Arc::new(AsyncMutex::new(vec![])),
            table_change_events: Arc::new(AsyncMutex::new(vec![])),
            console,
            tables_general_info,
        }
    }

    pub fn get_table_change_events(&self) -> Vec<BTableChangeEvents> {
        self.table_change_events.blocking_lock().clone()
    }

    pub async fn set_table_info(&self, table_name: String) {
        let columns_info = self.repository.get_columns_info(&table_name).await.unwrap();
        let columns_info_with_enums = columns_info
            .into_iter()
            .map(|column_info| BColumn::to_column(column_info))
            .collect();

        // Lock the async mutex and update the columns_info
        let mut locked_columns_info = self.columns_info.lock().await;
        *locked_columns_info = columns_info_with_enums;
        let mut locked_table_name = self.table_name.lock().await;
        *locked_table_name = Some(table_name);
    }

    pub fn reset_table_info(&self) {
        let mut locked_table_name = self.table_name.blocking_lock();
        *locked_table_name = None;
        let mut columns_info = self.columns_info.blocking_lock();
        *columns_info = vec![];
        let mut table_change_events = self.table_change_events.blocking_lock();
        *table_change_events = vec![];
    }
    pub fn add_table_change_event(&self, table_change_event: BTableChangeEvents) {
        let mut locked_table_change_events = self.table_change_events.blocking_lock();

        match table_change_event {
            BTableChangeEvents::ChangeTableName(new_table_name) => {
                self.handle_change_table_name(new_table_name, &mut locked_table_change_events);
            }
            BTableChangeEvents::ChangeColumnDataType(column_name, data_type) => {
                self.handle_change_column_datatype(
                    column_name,
                    data_type,
                    &mut locked_table_change_events,
                );
            }
            BTableChangeEvents::ChangeColumnName(column_name, new_column_name) => {
                self.handle_change_column_name(
                    column_name,
                    new_column_name,
                    &mut locked_table_change_events,
                );
            }
            BTableChangeEvents::RemoveColumn(column_name) => {
                self.handle_remove_column(column_name, &mut locked_table_change_events);
            }
            BTableChangeEvents::AddColumn(column_name, data_type) => {
                self.handle_add_column(column_name, data_type, &mut locked_table_change_events);
            }
            BTableChangeEvents::AddForeignKey(column_foreign_key) => {
                self.handle_add_foreign_key(column_foreign_key, &mut locked_table_change_events);
            }
            BTableChangeEvents::RemoveForeignKey(column_name) => {
                self.handle_remove_foreign_key(column_name, &mut locked_table_change_events);
            }
            BTableChangeEvents::AddPrimaryKey(column_name) => {
                self.handle_add_primary_key(column_name, &mut locked_table_change_events);
            }
            BTableChangeEvents::RemovePrimaryKey(column_name) => {
                self.handle_remove_primary_key(column_name, &mut locked_table_change_events);
            }
        }

        self.console
            .write(format!("{:?}", *locked_table_change_events));
    }

    fn handle_change_table_name(
        &self,
        table_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        let locked_table_name = self.table_name.blocking_lock();
        if let Some(existing_event_index) =
            self.find_existing_change_table_name_event_locked(locked_table_change_events)
        {
            if table_name == *locked_table_name.as_ref().unwrap() {
                locked_table_change_events.remove(existing_event_index);
            } else {
                locked_table_change_events.remove(existing_event_index);
                locked_table_change_events.push(BTableChangeEvents::ChangeTableName(table_name));
            }
        } else {
            locked_table_change_events.push(BTableChangeEvents::ChangeTableName(table_name));
        }
    }

    fn handle_add_column(
        &self,
        column_name: String,
        data_type: BDataType,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) =
            self.find_existing_remove_column_event_locked(&column_name, locked_table_change_events)
        {
            if let BTableChangeEvents::RemoveColumn(original_column_name) =
                &locked_table_change_events[existing_event_index]
            {
                let locked_columns_info = self.columns_info.blocking_lock();
                if let Some(original_column) = locked_columns_info
                    .iter()
                    .find(|&column| column.name == *original_column_name)
                {
                    if data_type == original_column.datatype {
                        locked_table_change_events.remove(existing_event_index);
                    } else {
                        locked_table_change_events.remove(existing_event_index);
                        locked_table_change_events.push(BTableChangeEvents::ChangeColumnDataType(
                            column_name,
                            data_type,
                        ));
                    }
                }
            }
        } else {
            locked_table_change_events.push(BTableChangeEvents::AddColumn(column_name, data_type));
        }
    }

    fn handle_change_column_datatype(
        &self,
        column_name: String,
        data_type: BDataType,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        let locked_columns_info = self.columns_info.blocking_lock();

        if let Some(existing_event_index) = self.find_existing_change_data_type_column_event_locked(
            &column_name,
            locked_table_change_events,
        ) {
            if let Some(column) = locked_columns_info
                .iter()
                .find(|&column| column.name == column_name)
            {
                if column.datatype == data_type {
                    locked_table_change_events.remove(existing_event_index);
                } else {
                    locked_table_change_events.remove(existing_event_index);
                    locked_table_change_events.push(BTableChangeEvents::ChangeColumnDataType(
                        column_name,
                        data_type,
                    ));
                }
            } else {
                locked_table_change_events.remove(existing_event_index);
                locked_table_change_events.push(BTableChangeEvents::ChangeColumnDataType(
                    column_name,
                    data_type,
                ));
            }
        } else if let Some(existing_event_index) =
            self.find_existing_add_column_event_locked(&column_name, locked_table_change_events)
        {
            if let BTableChangeEvents::AddColumn(_, added_column_data_type) =
                &locked_table_change_events[existing_event_index]
            {
                if *added_column_data_type != data_type {
                    locked_table_change_events.remove(existing_event_index);
                    locked_table_change_events
                        .push(BTableChangeEvents::AddColumn(column_name, data_type));
                }
            }
        } else {
            locked_table_change_events.push(BTableChangeEvents::ChangeColumnDataType(
                column_name,
                data_type,
            ));
        }
    }

    fn handle_change_column_name(
        &self,
        column_name: String,
        new_column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if column_name == new_column_name {
            return;
        }

        self.rename_existing_datatype_change_event_locked(
            &column_name,
            &new_column_name,
            locked_table_change_events,
        );

        if let Some(existing_event_index) =
            self.find_existing_rename_column_event_locked(&column_name, locked_table_change_events)
        {
            self.update_existing_rename_event_locked(
                existing_event_index,
                new_column_name.clone(),
                locked_table_change_events,
            );
        } else if let Some(existing_event_index) =
            self.find_existing_add_column_event_locked(&column_name, locked_table_change_events)
        {
            self.update_existing_add_column_event_locked(
                existing_event_index,
                column_name.clone(),
                new_column_name.clone(),
                locked_table_change_events,
            );
        } else {
            locked_table_change_events.push(BTableChangeEvents::ChangeColumnName(
                column_name.clone(),
                new_column_name.clone(),
            ));
        }

        if let Some(existing_event_index) = self
            .find_existing_add_primary_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
            locked_table_change_events
                .push(BTableChangeEvents::AddPrimaryKey(new_column_name.clone()));
        }
    }

    fn handle_remove_column(
        &self,
        column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) = self
            .find_existing_add_primary_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        }
        if let Some(existing_event_index) = self
            .find_existing_add_foreign_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        }
        if let Some(existing_event_index) =
            self.find_existing_add_column_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        } else if let Some(existing_event_index) = self
            .find_existing_change_data_type_column_event_locked(
                &column_name,
                locked_table_change_events,
            )
        {
            locked_table_change_events.remove(existing_event_index);
            locked_table_change_events.push(BTableChangeEvents::RemoveColumn(column_name));
        } else if let Some(existing_event_index) =
            self.find_existing_rename_column_event_locked(&column_name, locked_table_change_events)
        {
            if let BTableChangeEvents::ChangeColumnName(original_column_name, _) =
                locked_table_change_events[existing_event_index].clone()
            {
                locked_table_change_events.remove(existing_event_index);
                locked_table_change_events
                    .push(BTableChangeEvents::RemoveColumn(original_column_name));
            }
        } else {
            locked_table_change_events.push(BTableChangeEvents::RemoveColumn(column_name));
        }
    }
    fn rename_existing_datatype_change_event_locked(
        &self,
        column_name: &str,
        new_column_name: &str,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(event_index) = self.find_existing_change_data_type_column_event_locked(
            column_name,
            locked_table_change_events,
        ) {
            if let BTableChangeEvents::ChangeColumnDataType(_, data_type) =
                locked_table_change_events[event_index].clone()
            {
                locked_table_change_events.remove(event_index);
                locked_table_change_events.push(BTableChangeEvents::ChangeColumnDataType(
                    new_column_name.to_string(),
                    data_type,
                ));
            }
        }
    }

    fn update_existing_rename_event_locked(
        &self,
        event_index: usize,
        new_column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let BTableChangeEvents::ChangeColumnName(original_column_name, _) =
            locked_table_change_events[event_index].clone()
        {
            if original_column_name != new_column_name {
                locked_table_change_events.push(BTableChangeEvents::ChangeColumnName(
                    original_column_name,
                    new_column_name,
                ));
            }
        }
        locked_table_change_events.remove(event_index);
    }

    fn update_existing_add_column_event_locked(
        &self,
        event_index: usize,
        column_name: String,
        new_column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let BTableChangeEvents::AddColumn(_, added_data_type) =
            locked_table_change_events[event_index].clone()
        {
            locked_table_change_events.remove(event_index);
            self.handle_add_column(new_column_name, added_data_type, locked_table_change_events);
        }
    }
    fn handle_add_primary_key(
        &self,
        column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) = self
            .find_existing_remove_primary_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        } else {
            locked_table_change_events.push(BTableChangeEvents::AddPrimaryKey(column_name));
        }
    }

    fn handle_remove_primary_key(
        &self,
        column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) = self
            .find_existing_add_primary_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        } else {
            locked_table_change_events.push(BTableChangeEvents::RemovePrimaryKey(column_name));
        }
    }

    fn handle_add_foreign_key(
        &self,
        column_foreign_key: BColumnForeignKey,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) = self.find_existing_add_foreign_key_event_locked(
            &column_foreign_key.column_name,
            locked_table_change_events,
        ) {
            locked_table_change_events.remove(existing_event_index);
            locked_table_change_events.push(BTableChangeEvents::AddForeignKey(column_foreign_key));
        } else if let Some(existing_event_index) = self
            .find_existing_remove_foreign_key_event_locked(
                &column_foreign_key.column_name,
                locked_table_change_events,
            )
        {
            locked_table_change_events.remove(existing_event_index);
        } else {
            locked_table_change_events.push(BTableChangeEvents::AddForeignKey(column_foreign_key));
        }
    }

    fn handle_remove_foreign_key(
        &self,
        column_name: String,
        locked_table_change_events: &mut Vec<BTableChangeEvents>,
    ) {
        if let Some(existing_event_index) = self
            .find_existing_add_foreign_key_event_locked(&column_name, locked_table_change_events)
        {
            locked_table_change_events.remove(existing_event_index);
        } else {
            locked_table_change_events.push(BTableChangeEvents::RemoveForeignKey(column_name));
        }
    }

    fn find_existing_remove_primary_key_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
            matches!(event, BTableChangeEvents::RemovePrimaryKey(existing_column_name)
            if existing_column_name == column_name)
        })
    }

    fn find_existing_add_primary_key_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::AddPrimaryKey(existing_column_name) if existing_column_name == column_name)
    })
    }

    fn find_existing_add_foreign_key_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::AddForeignKey(existing_column_foreign_key) if existing_column_foreign_key.column_name == column_name)
    })
    }

    fn find_existing_remove_foreign_key_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::RemoveForeignKey(existing_column_name) if existing_column_name == column_name)
    })
    }

    fn find_existing_rename_column_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::ChangeColumnName(_, modified_column_name) if modified_column_name == column_name)
    })
    }

    fn find_existing_remove_column_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::RemoveColumn(existing_column_name) if existing_column_name == column_name)
    })
    }

    fn find_existing_add_column_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::AddColumn(existing_column_name, _) if existing_column_name == column_name)
    })
    }

    fn find_existing_change_data_type_column_event_locked(
        &self,
        column_name: &str,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events.iter().position(|event| {
        matches!(event, BTableChangeEvents::ChangeColumnDataType(existing_column_name, _) if existing_column_name == column_name)
    })
    }

    fn find_existing_change_table_name_event_locked(
        &self,
        locked_table_change_events: &Vec<BTableChangeEvents>,
    ) -> Option<usize> {
        locked_table_change_events
            .iter()
            .position(|event| matches!(event, BTableChangeEvents::ChangeTableName(_)))
    }

    async fn alter_table(&self) {
        let mut locked_table_change_events = self.table_change_events.lock().await;
        let mut locked_table_name = self.table_name.lock().await;

        if !locked_table_change_events.is_empty() {
            let primary_key_column_names: Vec<String> = {
                let locked_columns = self.columns_info.lock().await;
                locked_columns
                    .iter()
                    .filter(|&column| {
                        column
                            .constraints
                            .iter()
                            .any(|constraint| matches!(constraint, BConstraint::PrimaryKey))
                    })
                    .map(|column| column.name.clone())
                    .collect()
            };

            let res = self
                .repository
                .alter_table(
                    locked_table_name.as_ref().unwrap(),
                    &*locked_table_change_events,
                    &primary_key_column_names,
                )
                .await;
            println!("Alter table result: {:?}", res);
        }

        for event in locked_table_change_events.iter() {
            if let BTableChangeEvents::ChangeTableName(updated_table_name) = event {
                *locked_table_name = Some(updated_table_name.clone());
            }
        }

        // Clear events
        locked_table_change_events.clear();
    }
    pub async fn update_table(&self) {
        self.alter_table().await;
        let current_table_name = { self.table_name.lock().await.as_ref().unwrap().clone() };

        self.set_table_info(current_table_name).await;
        self.set_general_tables_info().await;
    }

    pub async fn set_general_tables_info(&self) {
        let mut locked_tables = self.tables_general_info.lock().await;
        let tables_general_info = self.repository.get_general_tables_info().await.unwrap();
        let tables = tables_general_info
            .into_iter()
            .map(|table| BTableGeneral::to_table(table))
            .collect();
        *locked_tables = tables;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::business_components::component::repository_module::BRepositoryConsole;
    use sqlx::PgPool;

    async fn create_table_info(
        pool: PgPool,
        table_in: &BTableIn,
        tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
    ) -> TableInfo {
        let database_console = Arc::new(BRepositoryConsole::new());
        let repository = Arc::new(BRepository::new(Some(pool), database_console.clone()).await);
        let console = Arc::new(BusinessConsole::new(database_console));
        repository.create_table(table_in).await;

        let table_info = TableInfo::new(repository, console, tables_general_info);
        table_info.set_table_info(table_in.table_name.clone()).await;
        table_info.set_general_tables_info().await; // Initialize tables_general_info
        table_info
    }

    fn default_table_in() -> BTableIn {
        BTableIn {
            table_name: String::from("users"),
            columns: vec![
                BColumn {
                    name: String::from("id"),
                    datatype: BDataType::INTEGER,
                    constraints: vec![BConstraint::PrimaryKey],
                },
                BColumn {
                    name: String::from("name"),
                    datatype: BDataType::TEXT,
                    constraints: vec![],
                },
            ],
        }
    }

    fn create_btable_general(table_in: &BTableIn) -> BTableGeneral {
        BTableGeneral {
            table_name: table_in.table_name.clone(),
            column_names: table_in
                .columns
                .iter()
                .map(|col| col.name.clone())
                .collect(),
            data_types: table_in
                .columns
                .iter()
                .map(|col| col.datatype.clone())
                .collect(),
        }
    }

    fn sort_columns(columns: &mut Vec<BColumn>) {
        columns.sort_by(|a, b| a.name.cmp(&b.name));
    }

    fn sort_tables_general_info(tables: &mut Vec<BTableGeneral>) {
        tables.sort_by(|a, b| a.table_name.cmp(&b.table_name));
    }

    #[sqlx::test]
    async fn test_initialize_component(pool: PgPool) {
        let table_in = default_table_in();
        let tables_general_info = Arc::new(AsyncMutex::new(Vec::new()));

        let table_info = create_table_info(pool, &table_in, tables_general_info).await;

        let mut expected_columns = table_in.columns.clone();
        sort_columns(&mut expected_columns);

        // Verify the initialized state
        let columns_info = table_info.columns_info.lock().await;
        assert_eq!(*columns_info, expected_columns);
        assert_eq!(
            table_info.table_name.lock().await.as_ref().unwrap(),
            &table_in.table_name
        );
    }

    #[sqlx::test]
    async fn test_alter_table(pool: PgPool) {
        let tables_general_info = Arc::new(AsyncMutex::new(Vec::new()));

        let remote_table = BTableIn {
            table_name: String::from("registrations"),
            columns: vec![BColumn {
                name: String::from("id"),
                datatype: BDataType::INTEGER,
                constraints: vec![BConstraint::PrimaryKey],
            }],
        };

        let repository =
            Arc::new(BRepository::new(Some(pool), Arc::new(BRepositoryConsole::new())).await);
        repository.create_table(&remote_table).await;

        let table_in = default_table_in();
        repository.create_table(&table_in).await;

        let console = Arc::new(BusinessConsole::new(Arc::new(BRepositoryConsole::new())));
        let table_info = Arc::new(TableInfo::new(
            repository.clone(),
            console.clone(),
            tables_general_info.clone(),
        ));
        table_info.set_table_info(table_in.table_name.clone()).await;

        let table_change_events = vec![
            BTableChangeEvents::AddColumn(String::from("email"), BDataType::TEXT),
            BTableChangeEvents::ChangeColumnName(String::from("name"), String::from("username")),
            BTableChangeEvents::ChangeColumnDataType(String::from("username"), BDataType::INTEGER),
            BTableChangeEvents::AddColumn(String::from("age"), BDataType::INTEGER),
            BTableChangeEvents::RemoveColumn(String::from("age")),
            BTableChangeEvents::ChangeTableName(String::from("customers")),
            BTableChangeEvents::AddColumn(String::from("created_at"), BDataType::TIMESTAMP),
            BTableChangeEvents::ChangeColumnName(
                String::from("created_at"),
                String::from("registration_date"),
            ),
            BTableChangeEvents::RemoveColumn(String::from("registration_date")),
            BTableChangeEvents::AddColumn(String::from("is_active"), BDataType::BOOLEAN),
            BTableChangeEvents::ChangeColumnName(
                String::from("is_active"),
                String::from("active_status"),
            ),
            BTableChangeEvents::AddColumn(String::from("last_login"), BDataType::TIMESTAMP),
            BTableChangeEvents::ChangeColumnDataType(
                String::from("last_login"),
                BDataType::TIMESTAMP,
            ),
            BTableChangeEvents::AddColumn(String::from("country"), BDataType::TEXT),
            BTableChangeEvents::AddPrimaryKey(String::from("country")),
            BTableChangeEvents::ChangeColumnName(String::from("country"), String::from("region")),
            BTableChangeEvents::ChangeTableName(String::from("clients")),
            BTableChangeEvents::AddColumn(String::from("phone_number"), BDataType::TEXT),
            BTableChangeEvents::AddPrimaryKey(String::from("phone_number")),
            BTableChangeEvents::RemoveColumn(String::from("phone_number")),
            BTableChangeEvents::ChangeColumnName(String::from("username"), String::from("name")),
            BTableChangeEvents::AddColumn(String::from("registration_id"), BDataType::INTEGER),
            BTableChangeEvents::AddForeignKey(BColumnForeignKey {
                column_name: String::from("registration_id"),
                referenced_table: String::from("registrations"),
                referenced_column: String::from("id"),
            }),
            BTableChangeEvents::RemoveForeignKey(String::from("registration_id")),
            BTableChangeEvents::AddForeignKey(BColumnForeignKey {
                column_name: String::from("registration_id"),
                referenced_table: String::from("registrations"),
                referenced_column: String::from("id"),
            }),
        ];

        let table_info_copy = table_info.clone();
        task::spawn_blocking(move || {
            for event in table_change_events {
                table_info_copy.add_table_change_event(event);
            }
        })
        .await;

        table_info.update_table().await;

        let mut expected_columns = vec![
            BColumn {
                name: String::from("id"),
                datatype: BDataType::INTEGER,
                constraints: vec![BConstraint::PrimaryKey],
            },
            BColumn {
                name: String::from("name"),
                datatype: BDataType::INTEGER,
                constraints: vec![],
            },
            BColumn {
                name: String::from("email"),
                datatype: BDataType::TEXT,
                constraints: vec![],
            },
            BColumn {
                name: String::from("active_status"),
                datatype: BDataType::BOOLEAN,
                constraints: vec![],
            },
            BColumn {
                name: String::from("last_login"),
                datatype: BDataType::TIMESTAMP,
                constraints: vec![],
            },
            BColumn {
                name: String::from("region"),
                datatype: BDataType::TEXT,
                constraints: vec![BConstraint::PrimaryKey],
            },
            BColumn {
                name: String::from("registration_id"),
                datatype: BDataType::INTEGER,
                constraints: vec![BConstraint::ForeignKey(
                    String::from("registrations"),
                    String::from("id"),
                )],
            },
        ];
        sort_columns(&mut expected_columns);

        let columns_info = table_info.columns_info.lock().await;
        assert_eq!(*columns_info, expected_columns);

        let expected_table_name = String::from("clients");
        assert_eq!(
            table_info.table_name.lock().await.as_ref().unwrap(),
            &expected_table_name
        );

        let table_change_events = table_info.table_change_events.lock().await;
        assert!(table_change_events.is_empty());
    }

    #[sqlx::test]
    async fn test_set_general_tables_info(pool: PgPool) {
        let table_in = default_table_in();
        let tables_general_info = Arc::new(AsyncMutex::new(Vec::new()));

        let table_info = create_table_info(pool, &table_in, tables_general_info.clone()).await;

        let new_table = BTableIn {
            table_name: String::from("products"),
            columns: vec![BColumn {
                name: String::from("product_name"),
                datatype: BDataType::TEXT,
                constraints: vec![],
            }],
        };
        table_info.repository.create_table(&new_table).await;

        table_info.set_general_tables_info().await;

        let mut expected_tables_general_info = vec![
            create_btable_general(&table_in),
            create_btable_general(&new_table),
        ];

        sort_tables_general_info(&mut expected_tables_general_info);

        let mut general_info = tables_general_info.lock().await.clone();
        sort_tables_general_info(&mut general_info);

        assert_eq!(general_info, expected_tables_general_info);
    }
}
