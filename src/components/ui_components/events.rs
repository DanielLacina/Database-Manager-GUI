use crate::components::ui_components::components::UIComponents;
use crate::components::ui_components::home::events::HomeMessage;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(UIComponents),
    Home(HomeMessage),
}
