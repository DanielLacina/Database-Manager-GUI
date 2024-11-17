use crate::components::ui_components::component::Event;
use crate::components::ui_components::events::Message;

#[derive(Debug, Clone)]
pub enum ConsoleMessage {}

impl Event for ConsoleMessage {
    fn message(event: Self) -> Message {
        Message::Console(event)
    }
}
