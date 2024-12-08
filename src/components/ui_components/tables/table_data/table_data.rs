use crate::components::business_components::{
    component::{
        BColumn, BConstraint, BDataType, BTableData, BTableGeneral, BTableIn, BTableInsertedData,
        BusinessComponent,
    },
    components::BusinessTables,
};
use crate::components::ui_components::component::{Event, UIComponent};
use crate::components::ui_components::{
    events::Message, tables::table_data::events::TableDataMessage,
};
use iced::{
    alignment,
    alignment::{Alignment, Vertical},
    border::Radius,
    futures::join,
    widget::{
        button, checkbox, column, container, pick_list, row, scrollable, text, text_input, Button,
        Checkbox, Column, PickList, Row, Text, TextInput,
    },
    Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};
use regex::Regex;
use std::iter::zip;
use std::sync::{Arc, Mutex};
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct TableDataUI {
    table_data: Arc<BTableData>,
    table_inserted_data: Option<BTableInsertedData>,
    selected_table_name: Option<String>,
}

impl UIComponent for TableDataUI {
    type EventType = TableDataMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::GetTableData(table_name) => {
                self.selected_table_name = Some(table_name.clone());
                let table_data = self.table_data.clone();
                Task::perform(
                    async move {
                        table_data.set_table_data(table_name).await;
                    },
                    |_| Self::EventType::SetTableData.message(),
                )
            }
            Self::EventType::SetTableData => {
                let updated_table_inserted_data =
                    self.table_data.table_inserted_data.blocking_lock();

                self.table_inserted_data = updated_table_inserted_data.clone();
                Task::none()
            }
            Self::EventType::UpdateCell(row_index, col_index, new_value) => {
                if let Some(table_inserted_data) = self.table_inserted_data.as_mut() {
                    if let Some(row_data) = table_inserted_data.rows.get_mut(row_index) {
                        if let Some(cell) = row_data.get_mut(col_index) {
                            *cell = new_value;
                        }
                    }
                }
                Task::none()
            }
        }
    }
}

impl TableDataUI {
    pub fn new(table_data: Arc<BTableData>) -> Self {
        Self {
            table_data,
            table_inserted_data: None,
            selected_table_name: None,
        }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let picklist = self.create_picklist();

        let table_content = self.create_table_content();

        // Combine the picklist and table content into a single column
        Column::new()
            .spacing(20)
            .push(text("Table Data Viewer").size(32).style(|_| text_style()))
            .push(picklist)
            .push(table_content)
            .into()
    }

    fn create_picklist<'a>(&'a self) -> Element<'a, Message> {
        let table_names: Vec<String> = self
            .table_data
            .tables_general_info
            .blocking_lock()
            .iter()
            .map(|info| info.table_name.clone())
            .collect();

        PickList::new(
            table_names.clone(),
            self.selected_table_name.clone(),
            |selected| TableDataMessage::GetTableData(selected.to_string()).message(),
        )
        .style(|_, _| picklist_style())
        .into()
    }

    fn create_table_content<'a>(&'a self) -> Element<'a, Message> {
        if let Some(ref table_inserted_data) = self.table_inserted_data {
            let column_headers = self.create_table_header(&table_inserted_data.column_names);
            let rows = self.create_table_rows(&table_inserted_data.rows);

            let table_with_header = Column::new().spacing(10).push(column_headers).push(rows);

            container(table_with_header)
                .style(|_| table_container_style())
                .padding(20)
                .width(Length::Fill)
                .into()
        } else {
            self.create_no_data_message()
        }
    }

    fn create_table_header<'a>(&'a self, column_names: &[String]) -> Row<'a, Message> {
        column_names.iter().enumerate().fold(
            Row::new().spacing(10),
            |row, (col_index, col_name)| {
                row.push(
                    container(text(col_name.clone()).size(16).style(|_| text_style()))
                        .width(Length::FillPortion(1)), // Ensure each column takes equal space
                )
            },
        )
    }
    fn create_table_rows<'a>(&'a self, rows: &[Vec<String>]) -> Column<'a, Message> {
        rows.iter()
            .enumerate()
            .fold(Column::new().spacing(10), |col, (row_index, row)| {
                let row_display = self.create_table_row(row, row_index);
                col.push(row_display)
            })
    }

    fn create_table_row<'a>(&'a self, row: &[String], row_index: usize) -> Row<'a, Message> {
        row.iter()
            .enumerate()
            .fold(Row::new().spacing(10), |r, (col_index, value)| {
                r.push(
                    container(self.create_table_column(row_index, col_index, value.as_str()))
                        .width(Length::FillPortion(1)), // Match width with header columns
                )
            })
    }
    fn create_table_column<'a>(
        &'a self,
        row_index: usize,
        col_index: usize,
        value: &str,
    ) -> TextInput<'a, Message> {
        text_input("", value)
            .on_input(move |new_value| {
                <TableDataUI as UIComponent>::EventType::UpdateCell(row_index, col_index, new_value)
                    .message()
            })
            .padding(5)
            .style(|_, _| text_input_style())
    }

    fn create_no_data_message<'a>(&'a self) -> Element<'a, Message> {
        container(text("Select a table").size(16).style(|_| text_style()))
            .padding(20)
            .into()
    }
}

// Style function for the table container using ::Style
fn table_container_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))), // Dark background
        border: Border {
            color: Color::from_rgb(0.0, 0.7, 1.0), // Neon cyan border
            width: 2.0,
            radius: Radius::from(12.0), // Rounded corners
        },
        text_color: Some(Color::from_rgb(0.9, 0.9, 1.0)), // Light neon text for readability
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.7, 1.0, 0.3), // Neon glow effect
            offset: Vector::new(0.0, 4.0),
            blur_radius: 10.0,
        },
    }
}

// Style for picklist dropdowns
fn picklist_style() -> pick_list::Style {
    pick_list::Style {
        text_color: Color::from_rgb(0.9, 0.9, 1.0), // Neon text
        background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)), // Dark background
        border: Border {
            color: Color::from_rgb(0.0, 0.7, 1.0), // Neon cyan border
            width: 1.5,
            radius: Radius::from(8.0),
        },
        placeholder_color: Color::from_rgba(0.6, 0.6, 0.7, 0.8), // Placeholder muted color
        handle_color: Color::from_rgb(0.0, 0.7, 1.0),            // Highlighted selection
    }
}

// General button styling for a futuristic look
fn create_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))), // Dark button background
        border: Border {
            color: Color::from_rgb(0.0, 0.7, 1.0), // Neon cyan border
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        text_color: Color::from_rgb(0.9, 0.9, 1.0), // Neon text
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.7, 1.0, 0.3), // Neon glow effect
            offset: Vector::new(0.0, 4.0),               // Slight vertical shadow offset
            blur_radius: 10.0,                           // Smooth shadow edges
        },
    }
} // General text style for dark themes
fn text_style() -> text::Style {
    text::Style {
        color: Some(Color::from_rgb(0.9, 0.9, 1.0)), // Neon text color
    }
}

fn text_input_style() -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from_rgb(0.15, 0.15, 0.2)), // Dark background
        border: Border {
            color: Color::from_rgb(0.0, 0.7, 1.0), // Neon cyan border
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        icon: Color::from_rgb(0.9, 0.9, 1.0), // Light neon color for the icon
        placeholder: Color::from_rgb(0.6, 0.6, 0.7), // Muted gray for placeholder text
        value: Color::from_rgb(0.9, 0.9, 1.0), // Neon text color for the input value
        selection: Color::from_rgba(0.0, 0.7, 1.0, 0.5), // Highlighted selection color
    }
}
