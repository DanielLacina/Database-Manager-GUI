use crate::components::business_components::{
    component::{BDataType, BTable, BTableGeneralInfo, BTableIn, BTableInfo},
    components::BusinessTables,
};
use crate::components::ui_components::{component::Event, events::Message};

#[derive(Debug, Clone)]
pub enum TablesMessage {
    UpdateTableFilter(String),
    UpdateTables,
    SetTables(BusinessTables),
    ShowOrRemoveCreateTableForm,
    GetSingleTableInfo(String),
    SetSingleTableInfo(BTableInfo),
    UndisplayTableInfo,
    SingleTableInfo(TableInfoMessage),
    CreateTableForm(CreateTableFormMessage),
    InitializeComponent,
    ComponentInitialized(BusinessTables),
    RequestDeleteTable(String),
    ConfirmDeleteTable,
    CancelDeleteTable,
}

impl Event for TablesMessage {
    fn message(event: Self) -> Message {
        Message::Tables(event)
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
    TableCreated(BusinessTables, String),
    SetOrRemovePrimaryKey(usize),
    AddForeignKey(usize, String, String),
    RemoveForeignKey(usize),
    ShowOrRemoveCreateTableForm,
    ToggleForeignKeyDropdown(usize),
    ToggleForeignKeyTable(usize, String),
}

impl Event for CreateTableFormMessage {
    fn message(event: Self) -> Message {
        TablesMessage::message(TablesMessage::CreateTableForm(event))
    }
}

#[derive(Debug, Clone)]
pub enum TableInfoMessage {
    AddColumn,                          // Event to add a new column to the form
    RemoveColumn(usize),                // Event to remove a specific column by index
    UpdateColumnName(usize, String),    // Event to update the name of a specific column
    UpdateColumnType(usize, BDataType), // Event to update the type of a specific column
    UpdateTableName(String),
    SubmitUpdateTable,
    UpdateTableInfo(BTableInfo),
    ToggleForeignKeyDropdown(usize),
    ToggleForeignKeyTable(usize, String),
    AddForeignKey(usize, String, String),
    RemoveForeignKey(usize),
}

impl Event for TableInfoMessage {
    fn message(event: Self) -> Message {
        TablesMessage::message(TablesMessage::SingleTableInfo(event))
    }
}
