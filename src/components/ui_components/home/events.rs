use crate::components::ui_components::component::Event;
use crate::components::ui_components::home::home::HomeUI;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentInitialized(HomeUI),
    TableFilterChanged(String),
    ShowCreateTableForm,
    AddColumn,                       // Event to add a new column to the form
    RemoveColumn(usize),             // Event to remove a specific column by index
    UpdateColumnName(usize, String), // Event to update the name of a specific column
    UpdateColumnType(usize, String), // Event to update the type of a specific column
}

impl Event for HomeMessage {}
