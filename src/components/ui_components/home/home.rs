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
    pub home: Arc<BusinessHome>,
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::InitializeComponent => {
                let home = self.home.clone();
                Task::perform(
                    async move {
                        home.initialize_component().await;
                    },
                    |_| Self::EventType::ComponentInitialized.message(),
                )
            }
            Self::EventType::ComponentInitialized => Task::none(),
        }
    }
}

impl HomeUI {
    pub fn new(home: Arc<BusinessHome>) -> Self {
        Self { home }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        self.title()
    }

    fn title<'a>(&'a self) -> Element<'a, Message> {
        // Acquire the lock
        let locked_title = self.home.title.blocking_lock();

        // Extract the title text
        let title_text = if let Some(title) = locked_title.as_ref() {
            title.clone()
        } else {
            "Loading".to_string()
        };

        // Create the container with the title
        container(text(title_text))
            .width(300)
            .height(50)
            .padding(10)
            .into()
    }
}
