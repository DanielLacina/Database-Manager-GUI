use crate::components::ui_components::components::{ComponentsMessage, UIComponents};
use crate::components::ui_components::{
    console::events::ConsoleMessage, home::events::HomeMessage, tables::events::TablesMessage,
};

#[derive(Debug, Clone)]
pub enum Message {
    Components(ComponentsMessage),
    Home(HomeMessage),
    Tables(TablesMessage),
    Console(ConsoleMessage),
}
