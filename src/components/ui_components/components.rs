use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
};
use crate::components::ui_components::{
    home::{events::HomeMessage, home::HomeUI},
    tables::tables::TablesUI,
};
use iced::{futures::join, Task};

pub type HomeUIComponent = HomeUI;
pub type TablesUIComponent = TablesUI;

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
    pub tables_ui: TablesUI,
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
        /* creates repositories */
        let business_components = BusinessComponents::new().await;
        let mut home_ui = HomeUI::new(business_components.home);
        let mut tables_ui = TablesUI::new(business_components.tables);
        let (home_result, tables_result) = join!(
            home_ui.initialize_component(),
            tables_ui.initialize_component()
        );
        Self { home_ui, tables_ui }
    }
}
