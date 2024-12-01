use crate::components::business_components::{
    component::BusinessComponent, components::BusinessHome,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    home::events::HomeMessage,
};
use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
};
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: Arc<AsyncMutex<BusinessHome>>,
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::InitializeComponent => {
                let home = self.home.clone();
                Task::perform(
                    async move {
                        let mut locked_home = home.lock().await;
                        locked_home.initialize_component().await;
                    },
                    |_| Self::EventType::ComponentInitialized.message(),
                )
            }
            Self::EventType::ComponentInitialized => Task::none(),
        }
    }
}

impl HomeUI {
    pub fn new(home: Arc<AsyncMutex<BusinessHome>>) -> Self {
        Self { home }
    }

    /// Main content function that combines all UI components
    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        self.title()
    }

    /// Renders the title section
    fn title<'a>(&'a self) -> Element<'a, Message> {
        let title_text = if let Ok(home) = self.home.try_lock() {
            if let Some(ref title) = home.title {
                title.clone()
            } else {
                "Loading".to_string()
            }
        } else {
            "Loading".to_string()
        };
        container(text(title_text))
            .width(300)
            .height(50)
            .padding(10)
            .into()
    }
}
