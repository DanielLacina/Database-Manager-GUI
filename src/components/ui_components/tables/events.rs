use crate::components::business_components::{
    component::{BDataType, BTableChangeEvents, BTableGeneral, BTableIn, BTableInfo},
    components::BusinessTables,
};
use crate::components::ui_components::{
    component::Event,
    events::Message,
    tables::{table_data::events::TableDataMessage, table_info::events::TableInfoMessage},
};

#[derive(Debug, Clone)]
pub enum TablesMessage {
    UpdateTableFilter(String),
    ShowOrRemoveCreateTableForm,
    ShowOrRemoveTableData,
    GetSingleTableInfo(String),
    SetSingleTableInfo,
    UndisplayTableInfo,
    SingleTableInfo(TableInfoMessage),
    SingleTableData(TableDataMessage),
    CreateTableForm(CreateTableFormMessage),
    InitializeComponent,
    SetTables,
    ComponentInitialized,
    RequestDeleteTable(String),
    ConfirmDeleteTable,
    CancelDeleteTable,
}

impl Event for TablesMessage {
    fn message(self) -> Message {
        Message::Tables(self)
    }
}

#[derive(Debug, Clone)]
pub enum CreateTableFormMessage {
    SubmitCreateTable(BTableIn),
    AddColumn,                          // Event to add a new column to the form
    RemoveColumn(usize),                // Event to remove a specific column by index
    UpdateColumnName(usize, String),    // Event to update the name of a specific column
    UpdateColumnType(usize, BDataType), // Event to update the type of a specific column
    UpdateTableName(String),
    TableCreated(String),
    SetOrRemovePrimaryKey(usize),
    AddForeignKey(usize, String, String),
    RemoveForeignKey(usize),
    ShowOrRemoveCreateTableForm,
    ToggleForeignKeyDropdown(usize),
    ToggleForeignKeyTable(usize, String),
}

impl Event for CreateTableFormMessage {
    fn message(self) -> Message {
        TablesMessage::CreateTableForm(self).message()
    }
}
