use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
};
use crate::components::ui_components::{
    console::console::Console,
    home::{events::HomeMessage, home::HomeUI},
    tables::{events::TablesMessage, tables::TablesUI},
};
use iced::Task;

pub type HomeUIComponent = HomeUI;
pub type TablesUIComponent = TablesUI;
pub type ConsoleComponent = Console;

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
    pub tables_ui: TablesUIComponent,
    pub console: ConsoleComponent,
}

impl UIComponent for UIComponents {
    type EventType = ComponentsMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        Task::none()
    }
}

impl UIComponents {
    pub async fn new() -> Self {
        /* creates repositories */
        let business_components = BusinessComponents::new().await;
        Self {
            home_ui: HomeUI::new(business_components.home),
            tables_ui: TablesUI::new(business_components.tables),
            console: Console::new(),
        }
    }

    pub fn initialize_startup_components_message() -> Task<Message> {
        Task::done(HomeMessage::message(HomeMessage::InitializeComponent)).chain(Task::done(
            TablesMessage::message(TablesMessage::InitializeComponent),
        ))
    }
}
