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
        button, checkbox, column, container, row, scrollable, text, text_input, Button, Checkbox,
        Column, PickList, Row, Text,
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
        // Get the table names for the picklist
        let table_names: Vec<String> = self
            .table_data
            .tables_general_info
            .blocking_lock()
            .iter()
            .map(|info| info.table_name.clone())
            .collect();

        // Display the picklist to select a table, ensuring it's correctly selected
        let picklist = PickList::new(
            table_names.clone(),
            self.selected_table_name.clone(),
            |selected| TableDataMessage::GetTableData(selected.to_string()).message(),
        );

        // Display the table data if available
        let table_content: Element<'a, Message> =
            if let Some(ref table_inserted_data) = self.table_inserted_data {
                // Get the column names
                let column_names: Vec<String> = table_inserted_data
                    .column_names
                    .iter()
                    .map(|col| col.clone())
                    .collect();

                // Generate rows for each data row
                let data_rows: Element<Message> = table_inserted_data
                    .rows
                    .iter()
                    .enumerate()
                    .fold(Column::new().spacing(10), |col, (i, row)| {
                        let row_display = row
                            .iter()
                            .fold(Row::new().spacing(10), |r, data| r.push(text(data.clone())));
                        col.push(row_display)
                    })
                    .into();

                // Create a container with the table header and rows
                let table_with_header = Column::new()
                    .spacing(10)
                    .push(Row::new().spacing(10).push(
                        column_names.iter().fold(Row::new(), |r, col_name| {
                            r.push(text(col_name.clone()).size(16))
                        }),
                    ))
                    .push(data_rows);

                // Wrap the table in a container with custom styling
                container(table_with_header)
                    .style(|_| table_container_style()) // Apply table container style
                    .padding(20)
                    .width(Length::Fill)
                    .into()
            } else {
                // If no data is available, show a message
                container(text("Select a table").size(16))
                    .style(|_| table_container_style()) // Apply container style
                    .into()
            };

        // Combine the picklist and table content into a single column
        Column::new()
            .spacing(20)
            .push(text("Table Data Viewer").size(32))
            .push(picklist)
            .push(table_content)
            .into()
    }
}

// Style function for the table container using ::Style
fn table_container_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))), // Light gray background
        border: Border {
            color: Color::from_rgb(0.7, 0.7, 0.7), // Light border
            width: 1.0,
            radius: Radius::from(8.0), // Rounded corners
        },
        text_color: Some(Color::BLACK), // Black text for contrast
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.1), // Slight shadow for depth
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}
