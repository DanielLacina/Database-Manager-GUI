use crate::components::business_components::{
    component::{BColumn, BDataType, BTable, BTableIn, BTableInfo, BusinessComponent},
    components::BusinessTables,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    home::events::TableInfoMessage,
};
use iced::{
    border,
    font::Font,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
        TextInput,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
};

#[derive(Debug, Clone)]
pub struct TableInfoUI {
    table_name: String,
    columns: Vec<BColumn>,
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
    pub fn new(table_info: BTableInfo) -> Self {
        Self {
            table_name: table_info.table_name,
            columns: table_info.columns_info,
        }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        // Create the main column for table information
        let mut table_info_column = Column::new().spacing(10);

        // Table name input field
        let table_name_input = self.build_table_name_input();
        table_info_column = table_info_column.push(container(table_name_input).width(Length::Fill));

        // Add headers for the columns section
        table_info_column = table_info_column.push(self.build_column_headers());

        // Add a separator line
        table_info_column = table_info_column.push(text("------------------------------"));

        // Add column data inputs
        let columns_info_column = self.build_columns_info();
        table_info_column = table_info_column
            .push(scrollable(container(columns_info_column.spacing(10)).padding(10)).height(400));

        // Add "Add Column" button
        let add_column_button = button("Add Column")
            .on_press(<TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::AddColumn,
            ))
            .padding(10);
        table_info_column = table_info_column.push(add_column_button);

        // Wrap everything in a container and return as an Element
        container(table_info_column).padding(20).into()
    }

    /// Builds the input for the table name with styling
    fn build_table_name_input(&self) -> TextInput<'_, Message> {
        text_input("Table Name", &self.table_name)
            .on_input(|value| {
                <TableInfoUI as UIComponent>::EventType::message(
                    <TableInfoUI as UIComponent>::EventType::UpdateTableName(value),
                )
            })
            .size(30)
            .padding(20)
            .style(|_, _| text_input::Style {
                background: Background::Color(Color::from_rgb(0.2, 0.5, 0.7)),
                border: Border {
                    width: 2.0,
                    color: Color::from_rgb(0.1, 0.5, 0.8),
                    radius: border::Radius::new(1),
                },
                icon: Color::from_rgb(1.0, 1.0, 1.0),
                placeholder: Color::from_rgb(0.8, 0.8, 0.8),
                value: Color::WHITE,
                selection: Color::from_rgb(0.1, 0.5, 0.8),
            })
    }

    /// Builds headers for columns
    fn build_column_headers(&self) -> Row<'_, Message> {
        Row::new()
            .spacing(20)
            .push(text("Column Name").size(20).width(Length::Fill))
            .push(text("Data Type").size(20).width(Length::Fill))
    }

    /// Builds the input fields for the columns information
    fn build_columns_info(&self) -> Column<'_, Message> {
        let mut columns_info_column = Column::new();

        for (index, column_info) in self.columns.iter().enumerate() {
            // Input for column name
            let name_input = text_input("Column Name", &column_info.name)
                .on_input(move |value| {
                    <TableInfoUI as UIComponent>::EventType::message(
                        <TableInfoUI as UIComponent>::EventType::UpdateColumnName(index, value),
                    )
                })
                .width(200);

            // Dropdown for selecting data type
            let datatype_input = PickList::new(
                vec![BDataType::TEXT, BDataType::INT, BDataType::TIMESTAMP],
                Some(&column_info.datatype),
                move |value| {
                    <TableInfoUI as UIComponent>::EventType::message(
                        <TableInfoUI as UIComponent>::EventType::UpdateColumnType(index, value),
                    )
                },
            )
            .width(150);

            // Button to remove the column
            let remove_button = button("Remove")
                .on_press(<TableInfoUI as UIComponent>::EventType::message(
                    <TableInfoUI as UIComponent>::EventType::RemoveColumn(index),
                ))
                .padding(5);

            // Add row with inputs and button
            columns_info_column = columns_info_column.push(
                Row::new()
                    .spacing(20)
                    .push(name_input)
                    .push(datatype_input)
                    .push(remove_button)
                    .width(Length::Fill),
            );
        }

        columns_info_column
    }
}
