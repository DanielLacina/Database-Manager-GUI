use crate::components::ui_components::{component::Event, events::Message};

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeComponent,
    ComponentInitialized,
}

impl Event for HomeMessage {
    fn message(self) -> Message {
        Message::Home(self)
    }
}
