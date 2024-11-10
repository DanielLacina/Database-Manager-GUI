use crate::components::ui_components::events::Message;
use iced::Task;

pub trait UIComponent {
    type EventType: Event;

    async fn initialize_component(&mut self);
    fn update(&mut self, message: Self::EventType) -> Task<Message>;
}

pub trait Event {
    fn message(event: Self) -> Message;
}
