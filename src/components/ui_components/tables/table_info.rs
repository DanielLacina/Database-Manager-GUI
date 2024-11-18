use crate::components::business_components::{
    component::{
        BColumn, BDataType, BTable, BTableChangeEvents, BTableIn, BTableInfo, BusinessComponent,
    },
    components::BusinessTables,
};
use crate::components::ui_components::console::events::ConsoleMessage;
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    tables::events::TableInfoMessage,
};
use iced::{
    border,
    border::Radius,
    font::Font,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row,
        Scrollable, Text, TextInput,
    },
    Alignment, Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};

#[derive(Debug, Clone)]
pub struct TableInfoUI {
    table_info: BTableInfo,
    table_name_display: String,
    columns_display: Vec<BColumn>,
}

impl UIComponent for TableInfoUI {
    type EventType = TableInfoMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::AddColumn => {
                let new_column = BColumn::default();
                self.columns_display.push(new_column.clone());
                self.table_info
                    .add_table_change_event(BTableChangeEvents::AddColumn(
                        new_column.name,
                        new_column.datatype,
                    ));
                Task::done(ConsoleMessage::message(ConsoleMessage::LogMessage(
                    self.formated_table_change_events(),
                )))
            }
            Self::EventType::RemoveColumn(index) => {
                if index < self.columns_display.len() {
                    if let Some(column) = self.columns_display.get_mut(index) {
                        self.table_info
                            .add_table_change_event(BTableChangeEvents::RemoveColumn(
                                column.name.clone(),
                            ));
                        self.columns_display.remove(index);
                    }
                }
                Task::done(ConsoleMessage::message(ConsoleMessage::LogMessage(
                    self.formated_table_change_events(),
                )))
            }
            Self::EventType::UpdateColumnName(index, new_column_name) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    let original_column_name = column.name.clone();
                    column.name = new_column_name.clone();
                    self.table_info
                        .add_table_change_event(BTableChangeEvents::ChangeColumnName(
                            original_column_name,
                            new_column_name,
                        ));
                }

                Task::done(ConsoleMessage::message(ConsoleMessage::LogMessage(
                    self.formated_table_change_events(),
                )))
            }
            Self::EventType::UpdateColumnType(index, new_datatype) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    column.datatype = new_datatype.clone();
                    self.table_info.add_table_change_event(
                        BTableChangeEvents::ChangeColumnDataType(column.name.clone(), new_datatype),
                    );
                }
                Task::done(ConsoleMessage::message(ConsoleMessage::LogMessage(
                    self.formated_table_change_events(),
                )))
            }
            Self::EventType::UpdateTableName(new_table_name) => {
                self.table_name_display = new_table_name.clone();
                self.table_info
                    .add_table_change_event(BTableChangeEvents::ChangeTableName(new_table_name));
                Task::done(ConsoleMessage::message(ConsoleMessage::LogMessage(
                    self.formated_table_change_events(),
                )))
            }
            Self::EventType::SubmitUpdateTable => {
                let mut table_info = self.table_info.clone();
                Task::perform(
                    async move {
                        table_info.alter_table().await;
                        table_info
                    },
                    |updated_table_info| {
                        Self::EventType::message(Self::EventType::UpdateTableInfo(
                            updated_table_info,
                        ))
                    },
                )
            }

            Self::EventType::UpdateTableInfo(updated_table_info) => {
                self.columns_display = updated_table_info.columns_info.clone();
                self.table_name_display = updated_table_info.table_name.clone();
                self.table_info = updated_table_info;
                Task::none()
            }
        }
    }
}

impl TableInfoUI {
    pub fn new(table_info: BTableInfo) -> Self {
        Self {
            table_info: table_info.clone(),
            table_name_display: table_info.table_name,
            columns_display: table_info.columns_info,
        }
    }

    fn formated_table_change_events(&self) -> String {
        format!("{:?}", self.table_info.get_table_change_events())
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        // Main layout column without excessive nested containers
        let mut table_info_column = Column::new().spacing(20).padding(20);

        // Table name input field
        table_info_column = table_info_column.push(self.build_table_name_input());

        // Add headers for columns
        table_info_column = table_info_column.push(self.build_column_headers());

        // Add a separator line
        table_info_column = table_info_column.push(
            text("------------------------------")
                .color(Color::from_rgb(0.6, 0.6, 0.6))
                .size(16),
        );

        // Add column data inputs with scrollable area
        let columns_info_column = self.build_columns_info();
        table_info_column = table_info_column.push(
            scrollable(container(columns_info_column.spacing(10)).padding(10))
                .height(Length::FillPortion(3)),
        );

        // Add "Add Column" button
        let add_column_button = button("➕ Add Column")
            .style(|_, _| button_style())
            .padding(10)
            .on_press(<TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::AddColumn,
            ));
        table_info_column = table_info_column.push(add_column_button);

        // Add "Update Table" button
        let submit_update_table_button = button("🛠️ Update Table")
            .style(|_, _| button_style())
            .padding(10)
            .on_press(<TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::SubmitUpdateTable,
            ));
        table_info_column = table_info_column.push(submit_update_table_button);

        // Apply the main border to the whole section only
        container(table_info_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(|_| container_style())
            .into()
    }

    /// Builds the input for the table name with simplified styling
    fn build_table_name_input(&self) -> TextInput<'_, Message> {
        text_input("📝 Table Name", &self.table_name_display)
            .on_input(|value| {
                <TableInfoUI as UIComponent>::EventType::message(
                    <TableInfoUI as UIComponent>::EventType::UpdateTableName(value),
                )
            })
            .size(30)
            .padding(10)
            .width(Length::Fill)
            .style(|_, _| text_input_style())
    }

    /// Builds headers for columns without unnecessary borders
    fn build_column_headers(&self) -> Row<'_, Message> {
        Row::new()
            .spacing(20)
            .push(
                text("📋 Column Name")
                    .size(20)
                    .color(Color::WHITE)
                    .width(Length::FillPortion(2)),
            )
            .push(
                text("🔧 Data Type")
                    .size(20)
                    .color(Color::WHITE)
                    .width(Length::FillPortion(1)),
            )
    }

    /// Builds the input fields for the columns information
    fn build_columns_info(&self) -> Column<'_, Message> {
        let mut columns_info_column = Column::new().spacing(10);

        for (index, column_info) in self.columns_display.iter().enumerate() {
            let name_input = text_input("📝 Column Name", &column_info.name)
                .on_input(move |value| {
                    <TableInfoUI as UIComponent>::EventType::message(
                        <TableInfoUI as UIComponent>::EventType::UpdateColumnName(index, value),
                    )
                })
                .width(Length::FillPortion(2))
                .padding(5)
                .style(|_, _| text_input_style());

            let datatype_input = PickList::new(
                vec![BDataType::TEXT, BDataType::INT, BDataType::TIMESTAMP],
                Some(&column_info.datatype),
                move |value| {
                    <TableInfoUI as UIComponent>::EventType::message(
                        <TableInfoUI as UIComponent>::EventType::UpdateColumnType(index, value),
                    )
                },
            )
            .width(Length::FillPortion(1))
            .padding(5);

            let remove_button = button("🗑️ Remove")
                .style(|_, _| button_style())
                .on_press(<TableInfoUI as UIComponent>::EventType::message(
                    <TableInfoUI as UIComponent>::EventType::RemoveColumn(index),
                ))
                .padding(10);

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
fn container_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.12, 0.15, 0.20))), // Darker background for a CRM feel
        border: Border {
            color: Color::from_rgb(0.1, 0.4, 0.6),
            width: 1.5,
            radius: Radius::from(8.0),
        },
        text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
        shadow: Shadow {
            color: Color::from_rgb(0.0, 0.0, 0.0),
            offset: Vector::new(0.0, 3.0),
            blur_radius: 7.0,
        },
    }
}

fn button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.6, 0.9))), // CRM blue button
        border: Border {
            color: Color::from_rgb(0.0, 0.4, 0.7),
            width: 2.0,
            radius: Radius::from(5.0),
        },
        text_color: Color::WHITE,
        shadow: Shadow {
            color: Color::from_rgb(0.0, 0.0, 0.0),
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
    }
}

fn text_input_style() -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from_rgb(0.18, 0.22, 0.28)), // Darker input background
        border: Border {
            width: 2.0,
            color: Color::from_rgb(0.0, 0.6, 0.9),
            radius: Radius::from(6.0),
        },
        placeholder: Color::from_rgb(0.6, 0.6, 0.6),
        value: Color::WHITE,
        selection: Color::from_rgb(0.0, 0.6, 0.9),
        icon: Color::from_rgb(0.8, 0.8, 0.8),
    }
}
