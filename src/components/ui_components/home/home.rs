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

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: BusinessHome,
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::InitializeComponent => {
                let mut home = self.home.clone();
                Task::perform(
                    async move {
                        home.initialize_component().await;
                        home
                    },
                    |home| Self::EventType::message(Self::EventType::ComponentInitialized(home)),
                )
            }
            Self::EventType::ComponentInitialized(home) => {
                self.home = home;
                Task::none()
            }
        }
    }
}

impl HomeUI {
    pub fn new(home: BusinessHome) -> Self {
        Self { home }
    }

    /// Main content function that combines all UI components
    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        self.title()
    }

    /// Renders the title section
    fn title<'a>(&'a self) -> Element<'a, Message> {
        let title_text = if let Some(title) = &self.home.title {
            title
        } else {
            "Loading"
        };
        container(text(title_text))
            .width(300)
            .height(50)
            .padding(10)
            .into()
    }
}
