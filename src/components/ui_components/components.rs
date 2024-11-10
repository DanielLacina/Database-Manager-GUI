use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::home::{events::HomeMessage, home::HomeUI};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
};
use iced::Task;

pub type HomeUIComponent = HomeUI;

#[derive(Debug, Clone)]
pub enum ComponentsMessage {
    InitializeComponents(UIComponents),
}

impl Event for ComponentsMessage {
    fn message(event: Self) -> Message {
        Message::Components(event)
    }
}

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

#[derive(Debug, Clone)]
pub struct UIComponents {
    pub home_ui: HomeUIComponent,
}

impl UIComponent for UIComponents {
    type EventType = ComponentsMessage;

    async fn initialize_component(&mut self) {}
    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        Task::none()
    }
}

impl UIComponents {
    pub async fn new() -> Self {
        let business_components = BusinessComponents::new().await;
        Self {
            home_ui: HomeUI::new(business_components.home, business_components.tables),
        }
    }

    pub fn initialized_task_message() -> Task<Message> {
        Task::done(Message::Home(HomeMessage::InitializeComponent))
    }
}
