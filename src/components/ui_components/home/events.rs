use crate::components::business_components::component::BDataType;
use crate::components::ui_components::component::Event;
use crate::components::ui_components::home::home::HomeUI;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentUpdated(HomeUI),
    UpdateTableFilter(String),
    ShowCreateTableForm,
    AddColumn,                          // Event to add a new column to the form
    RemoveColumn(usize),                // Event to remove a specific column by index
    UpdateColumnName(usize, String),    // Event to update the name of a specific column
    UpdateColumnType(usize, BDataType), // Event to update the type of a specific column
    UpdateTableName(String),
    SubmitCreateTable,
}

impl Event for HomeMessage {}
