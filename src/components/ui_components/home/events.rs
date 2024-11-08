use crate::components::ui_components::component::Event;
use crate::components::ui_components::home::home::HomeUI;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentInitialized(HomeUI),
    TableFilterChanged(String),
}

impl Event for HomeMessage {}
