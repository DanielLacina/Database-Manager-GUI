use crate::components::business_components::{
    component::{
        BColumn, BColumnsInfoWithEnumDataType, BDataType, BTable, BTableIn, BTableInfo,
        BusinessComponent,
    },
    components::BusinessTables,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    home::events::TableInfoMessage,
};
use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
};

pub struct TableInfoUI {
    table_name: String,
    columns: Vec<BColumnsInfoWithEnumDataType>,
}

impl UIComponent for TableInfoUI {
    type EventType = TableInfoMessage;

    async fn initialize_component(&mut self) {}

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::AddColumn => {
                self.columns.push(BColumn::default());
                Task::none()
            }
            Self::EventType::RemoveColumn(index) => {
                if index < self.columns.len() {
                    self.columns.remove(index);
                }
                Task::none()
            }
            Self::EventType::UpdateColumnName(index, input) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.name = input;
                }
                Task::none()
            }
            Self::EventType::UpdateColumnType(index, input) => {
                if let Some(column) = self.columns.get_mut(index) {
                    column.datatype = input;
                }
                Task::none()
            }
            Self::EventType::UpdateTableName(input) => {
                self.table_name = input;
                Task::none()
            }
        }
    }
}

impl TableInfoUI {
    fn new(table_info: BTableInfo) -> Self {
        Self {
            table_name: table_info.table_name,
            columns: table_info.columns_info,
        }
    }

    fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut table_info_column = Column::new().spacing(10);

        table_info_column = table_info_column.push(
            container(text(&self.table_name).size(35))
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(iced::Background::Color([0.2, 0.5, 0.7].into())), // Background color
                    text_color: Some([1.0, 1.0, 1.0].into()), // Text color (white)
                    ..Default::default()
                }),
        );

        // Add headers for columns
        table_info_column = table_info_column.push(
            Row::new()
                .spacing(20)
                .push(text("Column Name").size(20).width(Length::Fill))
                .push(text("Data Type").size(20).width(Length::Fill)),
        );

        // Add a horizontal line to separate headers from data
        table_info_column = table_info_column.push(text("------------------------------"));

        // Add rows of data
        for column_info in &self.columns {
            table_info_column = table_info_column.push(
                Row::new()
                    .spacing(20)
                    .push(text(&column_info.column_name).width(Length::Fill))
                    .push(text(&column_info.data_type).width(Length::Fill)),
            );
        }
        let undisplay_button =
            button(text("undisplay")).on_press(<TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::UndisplayTableInfo,
            ));
        table_info_column = table_info_column.push(undisplay_button);
        container(table_info_column).padding(20).into()
    }
}
