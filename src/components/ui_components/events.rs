use crate::components::ui_components::components::{ComponentsMessage, UIComponents};
use crate::components::ui_components::home::events::HomeMessage;

#[derive(Debug, Clone)]
pub enum Message {
    Components(ComponentsMessage),
    Home(HomeMessage),
}
