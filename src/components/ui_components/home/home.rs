use crate::components::business_components::{
    component::{BColumn, BDataType, BTable, BTableIn, BusinessComponent},
    components::{BusinessHome, BusinessTables},
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    home::{
        events::{HomeMessage, TablesMessage},
        tables::TablesUI,
    },
};
use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: BusinessHome,
    pub tables_ui: TablesUI,
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    async fn initialize_component(&mut self) {
        self.home.initialize_component().await;
        self.tables_ui.initialize_component().await;
    }

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::InitializeComponent => {
                let mut home_ui = self.clone();
                Task::perform(
                    async move {
                        home_ui.initialize_component().await;
                        home_ui
                    },
                    |home_ui_initialized| {
                        Self::EventType::message(Self::EventType::ComponentInitialized(
                            home_ui_initialized,
                        ))
                    },
                )
            }
            Self::EventType::ComponentInitialized(home_ui_initialized) => {
                *self = home_ui_initialized;
                Task::none()
            }

            Self::EventType::Tables(tables_message) => self.tables_ui.update(tables_message),
        }
    }
}

impl HomeUI {
    pub fn new(home: BusinessHome, tables: BusinessTables) -> Self {
        Self {
            home,
            tables_ui: TablesUI::new(tables),
        }
    }

    /// Main content function that combines all UI components
    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        self.tables_ui.content()
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
