use crate::components::business_components::component::{
    BDataType, BTableChangeEvents, BTableGeneral, BTableIn, BTableInfo,
};
use crate::components::ui_components::{
    component::Event, events::Message, tables::events::TablesMessage,
};

#[derive(Debug, Clone)]
pub enum TableInfoMessage {
    AddColumn,                          // Event to add a new column to the form
    RemoveColumn(usize),                // Event to remove a specific column by index
    UpdateColumnName(usize, String),    // Event to update the name of a specific column
    UpdateColumnType(usize, BDataType), // Event to update the type of a specific column
    UpdateTableName(String),
    SubmitUpdateTable,
    UpdateTableInfoUI,
    ToggleForeignKeyDropdown(usize),
    ToggleForeignKeyTable(usize, String),
    AddForeignKey(usize, String, String),
    RemoveForeignKey(usize),
    SetOrRemovePrimaryKey(usize),
    AddTableChangeEvent(BTableChangeEvents),
    TableChangeEventDone,
}

impl Event for TableInfoMessage {
    fn message(self) -> Message {
        TablesMessage::SingleTableInfo(self).message()
    }
}
