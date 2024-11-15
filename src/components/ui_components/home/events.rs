use crate::components::business_components::{
    component::{BDataType, BTable, BTableIn, BTableInfo},
    components::{BusinessHome, BusinessTables},
};
use crate::components::ui_components::home::home::HomeUI;
use crate::components::ui_components::{component::Event, events::Message};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentInitialized(HomeUI),
    Tables(TablesMessage),
}

impl Event for HomeMessage {
    fn message(event: Self) -> Message {
        Message::Home(event)
    }
}

#[derive(Debug, Clone)]
pub enum TablesMessage {
    SubmitCreateTable(BTableIn),
    UpdateTableFilter(String),
    ShowCreateTableForm,
    AddColumn,                          // Event to add a new column to the form
    RemoveColumn(usize),                // Event to remove a specific column by index
    UpdateColumnName(usize, String),    // Event to update the name of a specific column
    UpdateColumnType(usize, BDataType), // Event to update the type of a specific column
    UpdateTableName(String),
    TableCreated(BusinessTables, String),
    GetSingleTableInfo(String),
    SetSingleTableInfo(BTableInfo),
    UndisplayTableInfo,
    SingleTableInfo(TableInfoMessage),
}

impl Event for TablesMessage {
    fn message(event: Self) -> Message {
        HomeMessage::message(HomeMessage::Tables(event))
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
    UpdateTableChangeEventsDisplay,
}

impl Event for TableInfoMessage {
    fn message(event: Self) -> Message {
        TablesMessage::message(TablesMessage::SingleTableInfo(event))
    }
}
