use crate::components::business_components::{
    component::{BusinessComponent, BusinessTableOut},
    components::BusinessHome,
};
use crate::components::ui_components::{
    component::UIComponent, events::Message, home::events::HomeMessage,
};
use iced::{
    widget::{button, column, container, scrollable, text, text_input, Column, Row, Text},
    Alignment, Element, Length, Task,
};

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: BusinessHome,
    pub table_search: String,
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    async fn initialize_component(&mut self) {
        self.home.initialize_component().await;
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
                        Message::Home(Self::EventType::ComponentInitialized(home_ui_initialized))
                    },
                )
            }
            Self::EventType::ComponentInitialized(home_ui_initialized) => {
                *self = home_ui_initialized;
                Task::none()
            }
        }
    }
}

impl HomeUI {
    pub fn new(home: BusinessHome) -> Self {
        Self {
            home,
            table_search: String::from(""),
        }
    }

    fn tables<'a>(&'a self) -> Element<'a, Message> {
        let tables_container = if let Some(tables) = &self.home.tables {
            let mut tables_column = Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10);

            for table in tables {
                tables_column = tables_column.push(text(&table.table_name));
            }
            container(tables_column).height(250).width(300)
        } else {
            container(text("Loading"))
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10)
        };

        let text_input = text_input("Search", &self.table_search);
        let mut tables_display = Column::new();
        tables_display = tables_display.push(tables_container);
        tables_display = tables_display.push(text_input);
        container(tables_display).into()
    }

    fn title<'a>(&'a self) -> Element<'a, Message> {
        if let Some(title) = &self.home.title {
            container(text(title)).into()
        } else {
            container(text("Loading")).into()
        }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut row = Row::new();
        row = row.push(self.tables());
        row = row.push(self.title());
        container(row).into()
    }
}
