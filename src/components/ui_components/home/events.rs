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
}

impl Event for HomeMessage {
    fn message(event: Self) -> Message {
        Message::Home(event)
    }
}
