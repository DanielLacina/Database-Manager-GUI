use crate::message::Message;
use iced::{
    widget::{button, column, container, row, text, Column, Text},
    Element, Settings, Task,
};

pub struct HomeUI;

impl HomeUI {
    pub fn new() -> Self {
        Self {}
    }
    pub fn content(&self) -> Element<'_, Message> {
        if !home_component.tables.is_none() {
            container(Column::with_children(
                home_component
                    .tables
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
