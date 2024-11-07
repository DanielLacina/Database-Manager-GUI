use crate::components::ui_components::events::Message;

pub trait UIComponent {
    type EventType: Event;

    async fn initialize_component(&mut self);
    fn update(message: Self::EventType) -> Option<Self::EventType>;
}

pub trait Event {}
