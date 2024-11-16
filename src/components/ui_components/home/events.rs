use crate::components::business_components::components::BusinessHome;
use crate::components::ui_components::{component::Event, events::Message};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentInitialized(BusinessHome),
}

impl Event for HomeMessage {
    fn message(event: Self) -> Message {
        Message::Home(event)
    }
}
