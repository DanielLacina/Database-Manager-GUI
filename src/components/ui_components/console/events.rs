use crate::components::ui_components::component::Event;
use crate::components::ui_components::events::Message;

#[derive(Debug, Clone)]
pub enum ConsoleMessage {
    LogMessage(String),
    ClearMessages,
}

impl Event for ConsoleMessage {
    fn message(self) -> Message {
        Message::Console(self)
    }
}
