use crate::components::ui_components::component::UIComponent;
use crate::components::ui_components::console::events::ConsoleMessage;
use crate::components::ui_components::events::Message;
use iced::Task;

#[derive(Debug, Clone)]
pub struct Console {
    events: Vec<String>,
}

impl UIComponent for Console {
    type EventType = ConsoleMessage;

    async fn initialize_component(&mut self) {}
    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        Task::none()
    }
}

impl Console {
    pub fn new() -> Self {
        Self { events: vec![] }
    }
}
