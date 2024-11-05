use crate::components::business_components::{
    component::{initialize_business_component, BusinessTableOut},
    home::Home,
};
use crate::components::ui_components::{component::UIComponent, events::Message};
use iced::{
    widget::{button, column, container, row, scrollable, text, Column, Text},
    Alignment, Element, Length,
};

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: Home,
}

impl UIComponent for HomeUI {
    async fn initialize_component(&mut self) {
        let home_business_component =
            initialize_business_component::<Home>(self.home.clone()).await;
        self.home = home_business_component;
    }
}

impl HomeUI {
    pub fn new(home: Home) -> Self {
        Self { home }
    }

    fn tables(tables: Option<Vec<BusinessTableOut>>) -> Element<'static, Message> {
        if !tables.is_none() {
            let mut column = Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10);
            for table in tables.unwrap_or_default() {
                column = column.push(text(table.table_name));
            }
            scrollable(container(column)).height(250).width(300).into()
        } else {
            container("Loading")
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10)
                .into()
        }
    }

    pub fn content(&self) -> Element<'static, Message> {
        HomeUI::tables(self.home.tables.clone())
    }
}
