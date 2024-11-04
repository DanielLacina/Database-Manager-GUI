use crate::components::business_components::home::Home;
use crate::components::ui_components::components::Message;
use iced::{
    widget::{button, column, container, row, text, Column, Text},
    Element, Settings, Task,
};

pub struct HomeUI {
    pub home: Home,
}

impl HomeUI {
    pub fn new(home: Home) -> Self {
        Self { home }
    }
    pub fn content(&self) -> Element<'_, Message> {
        let home = self.home;
        if !home.tables.is_none() {
            container(Column::with_children(
                home.tables
                    .unwrap_or_default()
                    .into_iter()
                    .map(|table| Text::new(table.table_name).into())
                    .collect::<Vec<_>>(),
            ))
            .into()
        } else {
            column!(text("loading")).into()
        }
    }
}
