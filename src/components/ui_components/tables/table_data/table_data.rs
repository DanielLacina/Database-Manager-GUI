use crate::components::business_components::{
    component::{
        BColumn, BConstraint, BDataType, BRowColumnValue, BTableData, BTableDataChangeEvents,
        BTableGeneral, BTableIn, BTableInsertedData, BusinessComponent,
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
        Checkbox, Column, PickList, Row, Scrollable, Text, TextInput,
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
            Self::EventType::UpdateTableData => {
                let table_data = self.table_data.clone();
                Task::perform(
                    async move {
                        table_data.update_table_data().await;
                    },
                    |_| Self::EventType::SetTableData.message(),
                )
            }
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
            Self::EventType::ResetTableData => {
                if let Some(table_name) = self.selected_table_name.as_ref() {
                    Task::done(Self::EventType::GetTableData(table_name.clone()).message())
                } else {
                    Task::none()
                }
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
                            let column_name = table_inserted_data.column_names[col_index].clone();

                            self.table_data.add_modify_row_column_value_event(
                                row_index,
                                column_name,
                                new_value.clone(),
                            );

                            *cell = new_value;
                        }
                    }
                }
                Task::none()
            }
            Self::EventType::DeleteRow(row_index) => {
                if let Some(table_inserted_data) = self.table_inserted_data.as_mut() {
                    self.table_data.add_delete_row_event(row_index);
                    table_inserted_data.rows.remove(row_index);
                }
                Task::none()
            }
            Self::EventType::AddRow => {
                if let Some(table_inserted_data) = self.table_inserted_data.as_mut() {
                    let values: Vec<String> = table_inserted_data
                        .column_names
                        .iter()
                        .map(|col_name| String::new())
                        .collect();

                    self.table_data.add_insert_row_event(values.clone());

                    table_inserted_data.rows.push(values);
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

    pub fn get_table_name(&self) -> Option<String> {
        if let Some(table_inserted_data) = self.table_inserted_data.as_ref() {
            Some(table_inserted_data.table_name.clone())
        } else {
            None
        }
    }
    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        // Combine the picklist, table content, and update button into a single column
        Column::new()
            .spacing(20)
            .push(text("Table Data Viewer").size(32).style(|_| text_style()))
            .push(self.create_picklist())
            .push(self.create_table_content())
            .push(self.update_table_data_button()) // Add the button at the bottom
            .push(self.reset_table_data_button())
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
            let table_with_header = Column::new()
                .spacing(10)
                .push(self.table_column_names_and_rows(
                    &table_inserted_data.column_names,
                    &table_inserted_data.rows,
                ))
                .push(self.add_row_button());

            container(table_with_header)
                .style(|_| table_container_style())
                .padding(20)
                .width(Length::Fill)
                .into()
        } else {
            self.create_no_data_message()
        }
    }

    fn table_column_names_and_rows<'a>(
        &'a self,
        column_names: &Vec<String>,
        rows: &[Vec<String>],
    ) -> Scrollable<'a, Message> {
        let mut table_column_names_and_rows = Column::new();

        let column_names = column_names.iter().enumerate().fold(
            Row::new().spacing(10),
            |row, (col_index, col_name)| {
                row.push(
                    container(text(col_name.clone()).size(16).style(|_| text_style())).width(100), // Ensure each column takes equal space
                )
            },
        );
        table_column_names_and_rows = table_column_names_and_rows.push(column_names);

        for (row_index, row) in rows.iter().enumerate() {
            table_column_names_and_rows =
                table_column_names_and_rows.push(self.create_table_row(row, row_index));
        }

        scrollable(table_column_names_and_rows)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new(),
            })
            .height(Length::Fill)
    }

    fn create_table_row<'a>(&'a self, row: &[String], row_index: usize) -> Row<'a, Message> {
        let mut table_row = Row::new().spacing(10).align_y(Vertical::Center);
        for (col_index, value) in row.iter().enumerate() {
            table_row = table_row.push(
                container(self.create_table_column_value(row_index, col_index, value.as_str()))
                    .width(100) // Match width with header columns
                    .align_y(Vertical::Center),
            );
        }
        table_row.push(self.delete_row_button(row_index))
    }

    fn delete_row_button<'a>(&'a self, row_index: usize) -> Button<'a, Message> {
        button(
            text("Delete Row").size(16).style(|_| text_style()), // Style the button text
        )
        .on_press(<TableDataUI as UIComponent>::EventType::DeleteRow(row_index).message()) // Trigger the event
        .padding(10)
        .style(|_, _| delete_table_row_button_style()) // App
    }

    fn add_row_button<'a>(&'a self) -> Button<'a, Message> {
        button(
            text("Add Row").size(16).style(|_| text_style()), // Style the button text
        )
        .on_press(<TableDataUI as UIComponent>::EventType::AddRow.message()) // Trigger the event
        .padding(10)
        .style(|_, _| add_table_row_button_style()) // App
    }

    fn reset_table_data_button<'a>(&'a self) -> Button<'a, Message> {
        button(
            text("Reset Table Data").size(16).style(|_| text_style()), // Style the button text
        )
        .on_press(<TableDataUI as UIComponent>::EventType::ResetTableData.message()) // Trigger the event
        .padding(10)
        .style(|_, _| reset_table_data_button_style()) // App
    }

    fn update_table_data_button<'a>(&'a self) -> Button<'a, Message> {
        button(
            text("Update Table").size(16).style(|_| text_style()), // Style the button text
        )
        .on_press(<TableDataUI as UIComponent>::EventType::UpdateTableData.message()) // Trigger the event
        .padding(10)
        .style(|_, _| update_table_data_button_style()) // Apply button styling
    }

    fn create_table_column_value<'a>(
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
            blur_radius: 5.0,
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
fn reset_table_data_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.2))), // Dark button background
        border: Border {
            color: Color::from_rgb(0.0, 0.7, 1.0), // Neon cyan border
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        text_color: Color::from_rgb(0.9, 0.9, 1.0),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.7, 1.0, 0.3),
            offset: Vector::new(0.0, 4.0), // Slight vertical shadow offset
            blur_radius: 5.0,              // Smooth shadow edges
        },
    }
}

fn update_table_data_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.15))), // Dark button background
        border: Border {
            color: Color::from_rgba(1.0, 0.0, 0.0, 1.0),
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        text_color: Color::from_rgb(0.9, 0.9, 1.0),
        shadow: Shadow {
            color: Color::from_rgba(1.0, 0.2, 0.2, 0.3), // Neon glow effect
            offset: Vector::new(0.0, 4.0),               // Slight vertical shadow offset
            blur_radius: 5.0,                            // Smooth shadow edges
        },
    }
}

fn add_table_row_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.8, 1.0))), // Vibrant turquoise-blue background
        border: Border {
            color: Color::from_rgb(0.0, 1.0, 1.0), // Bright cyan border
            width: 3.0,                            // Slightly thicker border for emphasis
            radius: Radius::from(12.0),            // More rounded corners
        },
        text_color: Color::from_rgb(1.0, 1.0, 1.0), // Pure white text for maximum contrast
        shadow: Shadow {
            color: Color::from_rgba(0.0, 1.0, 1.0, 0.5), // Glow effect with a vibrant cyan color
            offset: Vector::new(0.0, 6.0),               // Increased vertical shadow offset
            blur_radius: 10.0,                           // More prominent shadow blur
        },
    }
}

fn delete_table_row_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.0, 0.0))), // Dark red background
        border: Border {
            color: Color::from_rgb(1.0, 0.2, 0.2), // Bright red border
            width: 2.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        text_color: Color::WHITE,
        shadow: Shadow {
            color: Color::from_rgb(0.0, 0.0, 0.0),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}

// General text style for dark themes
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
