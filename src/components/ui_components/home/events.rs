use crate::components::ui_components::component::Event;
use crate::components::ui_components::home::home::HomeUI;

#[derive(Debug, Clone)]
pub enum HomeMessage {
    InitializeHomeComponent,
    HomeComponentInitialized(HomeUI),
}

impl Event for HomeMessage {}
