use crate::components::business_components::{
    component::{
        BColumn, BColumnForeignKey, BConstraint, BDataType, BTableChangeEvents, BTableGeneralInfo,
        BTableIn, BTableInfo, BusinessComponent,
    },
    components::BusinessTables,
};
use crate::components::ui_components::console::events::ConsoleMessage;
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    tables::events::TableInfoMessage,
    tables::foreign_key_dropdown::{ForeignKeyDropDownUI, ForeignKeyDropdownEvents},
};
use iced::{
    alignment::Vertical,
    border,
    border::Radius,
    font::Font,
    widget::{
        button, checkbox, column, container, row, scrollable, text, text_input, Button, Column,
        PickList, Row, Scrollable, Text, TextInput,
    },
    Alignment, Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};
use std::iter::zip;
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

#[derive(Debug, Clone)]
pub struct TableInfoForeignKeyDropdown;

impl ForeignKeyDropdownEvents for TableInfoForeignKeyDropdown {
    fn add_foreign_key(
        &self,
        index: usize,
        referenced_table_name: String,
        referenced_column_name: String,
    ) -> Message {
        TableInfoMessage::AddForeignKey(index, referenced_table_name, referenced_column_name)
            .message()
    }
    fn remove_foreign_key(&self, index: usize) -> Message {
        TableInfoMessage::RemoveForeignKey(index).message()
    }
    fn toggle_foreign_key_table(&self, index: usize, table_name: String) -> Message {
        TableInfoMessage::ToggleForeignKeyTable(index, table_name).message()
    }
}

#[derive(Debug, Clone)]
pub struct TableInfoUI {
    table_info: Arc<BTableInfo>,
    table_name_display: String,
    columns_display: Vec<BColumn>,
    active_foreign_key_dropdown: Option<ForeignKeyDropDownUI<TableInfoForeignKeyDropdown>>,
}

impl UIComponent for TableInfoUI {
    type EventType = TableInfoMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::AddColumn => {
                let new_column = BColumn::default();
                self.columns_display.push(new_column.clone());
                Task::done(
                    Self::EventType::AddTableChangeEvent(BTableChangeEvents::AddColumn(
                        new_column.name,
                        new_column.datatype,
                    ))
                    .message(),
                )
            }
            Self::EventType::RemoveColumn(index) => {
                if index < self.columns_display.len() {
                    if let Some(column) = self.columns_display.get_mut(index) {
                        let column_name = column.name.clone();
                        self.columns_display.remove(index);
                        return Task::done(
                            Self::EventType::AddTableChangeEvent(BTableChangeEvents::RemoveColumn(
                                column_name,
                            ))
                            .message(),
                        );
                    }
                }
                Task::none()
            }
            Self::EventType::UpdateColumnName(index, new_column_name) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    let original_column_name = column.name.clone();
                    column.name = new_column_name.clone();
                    Task::done(
                        Self::EventType::AddTableChangeEvent(BTableChangeEvents::ChangeColumnName(
                            original_column_name,
                            new_column_name,
                        ))
                        .message(),
                    )
                } else {
                    Task::none()
                }
            }
            Self::EventType::UpdateColumnType(index, new_datatype) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    column.datatype = new_datatype.clone();
                    Task::done(
                        Self::EventType::AddTableChangeEvent(
                            BTableChangeEvents::ChangeColumnDataType(
                                column.name.clone(),
                                new_datatype,
                            ),
                        )
                        .message(),
                    )
                } else {
                    Task::none()
                }
            }
            Self::EventType::UpdateTableName(new_table_name) => {
                self.table_name_display = new_table_name.clone();
                Task::done(
                    Self::EventType::AddTableChangeEvent(BTableChangeEvents::ChangeTableName(
                        new_table_name,
                    ))
                    .message(),
                )
            }
            Self::EventType::SubmitUpdateTable => {
                let table_info = self.table_info.clone();
                Task::perform(
                    async move {
                        table_info.update_table().await;
                    },
                    |_| Self::EventType::UpdateTableInfoUI.message(),
                )
            }
            Self::EventType::UpdateTableInfoUI => {
                self.columns_display = self.table_info.columns_info.blocking_lock().clone();
                self.table_name_display = self
                    .table_info
                    .table_name
                    .blocking_lock()
                    .as_ref()
                    .unwrap()
                    .clone();
                Task::none()
            }
            Self::EventType::AddForeignKey(
                index,
                referenced_table_name,
                referenced_column_name,
            ) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    if let Some(existing_index) = column.constraints.iter().position(|constraint| {
                        matches!(
                            constraint,
                            BConstraint::ForeignKey(existing_table_name, existing_column_name)
                        )
                    }) {
                        // Replace the foreign key constraint if it exists
                        column.constraints.remove(existing_index);
                    }
                    column.constraints.push(BConstraint::ForeignKey(
                        referenced_table_name.clone(),
                        referenced_column_name.clone(),
                    ));
                    Task::done(
                        Self::EventType::AddTableChangeEvent(BTableChangeEvents::AddForeignKey(
                            BColumnForeignKey {
                                column_name: column.name.clone(),
                                referenced_table: referenced_table_name,
                                referenced_column: referenced_column_name,
                            },
                        ))
                        .message(),
                    )
                } else {
                    Task::none()
                }
            }
            Self::EventType::RemoveForeignKey(index) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    if let Some(existing_index) = column.constraints.iter().position(|constraint| {
                        matches!(
                            constraint,
                            BConstraint::ForeignKey(existing_table_name, existing_column_name)
                        )
                    }) {
                        column.constraints.remove(existing_index);
                    }
                    Task::done(
                        Self::EventType::AddTableChangeEvent(BTableChangeEvents::RemoveForeignKey(
                            column.name.clone(),
                        ))
                        .message(),
                    )
                } else {
                    Task::none()
                }
            }
            Self::EventType::SetOrRemovePrimaryKey(index) => {
                if let Some(column) = self.columns_display.get_mut(index) {
                    if let Some(existing_index) = column
                        .constraints
                        .iter()
                        .position(|constraint| matches!(constraint, BConstraint::PrimaryKey))
                    {
                        column.constraints.remove(existing_index);
                        Task::done(
                            Self::EventType::AddTableChangeEvent(
                                BTableChangeEvents::RemovePrimaryKey(column.name.clone()),
                            )
                            .message(),
                        )
                    } else {
                        column.constraints.push(BConstraint::PrimaryKey);
                        Task::done(
                            Self::EventType::AddTableChangeEvent(
                                BTableChangeEvents::AddPrimaryKey(column.name.clone()),
                            )
                            .message(),
                        )
                    }
                } else {
                    Task::none()
                }
            }
            Self::EventType::AddTableChangeEvent(table_change_event) => {
                self.table_info.add_table_change_event(table_change_event);
                Task::none()
            }
            Self::EventType::ToggleForeignKeyDropdown(index) => {
                if let Some(column) = self.columns_display.get(index) {
                    if let Some(foreign_key_dropdown) = &self.active_foreign_key_dropdown {
                        if foreign_key_dropdown.index == index {
                            self.active_foreign_key_dropdown = None;
                        } else {
                            self.active_foreign_key_dropdown = Some(ForeignKeyDropDownUI::new(
                                column.clone(),
                                self.table_info.tables_general_info.blocking_lock().clone(),
                                TableInfoForeignKeyDropdown,
                                None,
                                index,
                            ));
                        }
                    } else {
                        self.active_foreign_key_dropdown = Some(ForeignKeyDropDownUI::new(
                            column.clone(),
                            self.table_info.tables_general_info.blocking_lock().clone(),
                            TableInfoForeignKeyDropdown,
                            None,
                            index,
                        ));
                    }
                }
                Task::none()
            }
            Self::EventType::ToggleForeignKeyTable(_, table_name) => {
                if let Some(foreign_key_dropdown) = &mut self.active_foreign_key_dropdown {
                    foreign_key_dropdown.active_foreign_key_table_within_dropdown =
                        Some(table_name);
                }
                Task::none()
            }
            Self::EventType::TableChangeEventDone => Task::none(),
        }
    }
}

impl TableInfoUI {
    pub fn new(table_info: Arc<BTableInfo>) -> Self {
        Self {
            table_info: table_info.clone(),
            table_name_display: table_info
                .table_name
                .blocking_lock()
                .as_ref()
                .unwrap()
                .clone(),
            columns_display: table_info.columns_info.blocking_lock().clone(),
            active_foreign_key_dropdown: None,
        }
    }

    pub fn get_table_name(&self) -> String {
        self.table_info
            .table_name
            .blocking_lock()
            .as_ref()
            .unwrap()
            .clone()
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut table_info_column = Column::new().spacing(20).padding(20);

        table_info_column = table_info_column
            .push(self.build_table_name_input())
            .push(self.build_column_headers())
            .push(self.separator_line())
            .push(self.scrollable_columns_info())
            .push(self.add_column_button())
            .push(self.update_table_button());

        container(table_info_column)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .style(|_| container_style())
            .into()
    }

    // ============== Smaller Reusable Methods ==============

    fn build_table_name_input(&self) -> TextInput<'_, Message> {
        text_input("📝 Table Name", &self.table_name_display)
            .on_input(|value| TableInfoMessage::UpdateTableName(value).message())
            .size(30)
            .padding(10)
            .width(Length::Fill)
            .style(|_, _| text_input_style())
    }

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

    fn separator_line(&self) -> Element<'_, Message> {
        text("------------------------------")
            .color(Color::from_rgb(0.6, 0.6, 0.6))
            .size(16)
            .into()
    }

    fn scrollable_columns_info(&self) -> Element<'_, Message> {
        let columns_info_column = self.build_columns_info();
        scrollable(container(columns_info_column.spacing(10)).padding(10))
            .height(Length::Fill)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new(),
            })
            .into()
    }

    fn build_columns_info(&self) -> Column<'_, Message> {
        self.columns_display
            .iter()
            .enumerate()
            .fold(
                Column::new().spacing(10),
                |columns_info_column, (index, column_info)| {
                    columns_info_column.push(self.build_column_row(index, column_info))
                },
            )
            .into()
    }

    fn build_column_row<'a>(&'a self, index: usize, column_info: &'a BColumn) -> Row<'a, Message> {
        Row::new()
            .spacing(20)
            .push(self.column_name_input(index, &column_info.name))
            .push(self.data_type_picker(index, &column_info.datatype))
            .push(self.primary_key_checkbox(index, &column_info))
            .push(self.render_foreign_key_button(index, &column_info))
            .push(self.remove_column_button(index))
            .align_y(Vertical::Center)
    }

    fn column_name_input<'a>(&'a self, index: usize, name: &str) -> TextInput<'a, Message> {
        text_input("Column Name", name)
            .on_input(move |value| TableInfoMessage::UpdateColumnName(index, value).message())
            .width(200)
            .padding(5)
            .style(|_, _| text_input_style())
    }

    fn data_type_picker<'a>(&'a self, index: usize, datatype: &BDataType) -> Element<'a, Message> {
        PickList::new(
            vec![BDataType::TEXT, BDataType::INTEGER, BDataType::TIMESTAMP],
            Some(datatype.clone()),
            move |value| {
                <TableInfoUI as UIComponent>::EventType::UpdateColumnType(index, value).message()
            },
        )
        .width(150)
        .padding(5)
        .into()
    }

    fn primary_key_checkbox<'a>(&'a self, index: usize, column: &BColumn) -> Element<'a, Message> {
        checkbox(
            "Primary Key",
            column.constraints.contains(&BConstraint::PrimaryKey),
        )
        .on_toggle(move |_| {
            <TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::SetOrRemovePrimaryKey(index),
            )
        })
        .into()
    }

    fn render_foreign_key_button<'a>(
        &'a self,
        index: usize,
        column: &BColumn,
    ) -> Element<'a, Message> {
        // Button to show the foreign key tables
        let button_text = if let Some(foreign_key_constraint) = column
            .constraints
            .iter()
            .find(|constraint| matches!(constraint, BConstraint::ForeignKey(_, _)))
        {
            if let BConstraint::ForeignKey(referenced_table_name, referenced_column_name) =
                foreign_key_constraint
            {
                text(format!(
                    "{}.{}",
                    referenced_table_name, referenced_column_name
                ))
            } else {
                text("Set Foreign Key")
            }
        } else {
            text("Set Foreign Key")
        };

        let button = button(button_text).style(|_, _| button_style()).on_press(
            <TableInfoUI as UIComponent>::EventType::message(
                <TableInfoUI as UIComponent>::EventType::ToggleForeignKeyDropdown(index),
            ),
        );

        // Check if the current column's foreign key dropdown is active
        if let Some(active_foreign_key_dropdown) = &self.active_foreign_key_dropdown {
            if active_foreign_key_dropdown.index == index {
                Column::new()
                    .push(button)
                    .push(active_foreign_key_dropdown.content())
                    .spacing(5)
                    .into()
            } else {
                // Render just the button
                button.into()
            }
        } else {
            button.into()
        }
    }

    fn add_column_button(&self) -> Button<'_, Message> {
        button("➕ Add Column")
            .style(|_, _| button_style())
            .padding(10)
            .on_press(TableInfoMessage::AddColumn.message())
    }

    fn remove_column_button<'a>(&'a self, index: usize) -> Button<'a, Message> {
        button("🗑️ Remove")
            .style(|_, _| delete_button_style())
            .padding(10)
            .on_press(TableInfoMessage::RemoveColumn(index).message())
    }

    fn update_table_button(&self) -> Button<'_, Message> {
        button("🛠️ Update Table")
            .style(|_, _| button_style())
            .padding(10)
            .on_press(TableInfoMessage::SubmitUpdateTable.message())
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

fn constraints_container_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.95, 0.95, 0.95))),
        border: Border {
            color: Color::from_rgb(0.85, 0.85, 0.85),
            width: 1.0,
            radius: Radius::from(5.0),
        },
        text_color: Some(Color::BLACK),
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.05),
            offset: Vector::new(0.0, 1.0),
            blur_radius: 2.0,
        },
    }
}
fn table_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.4, 0.8))), // Blue background
        border: Border {
            color: Color::from_rgb(0.1, 0.3, 0.6), // Darker blue border
            width: 2.0,
            radius: Radius::from(6.0),
        },
        text_color: Color::WHITE, // White text for contrast
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5), // Slight shadow for depth
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
    }
}

fn column_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.4, 0.8, 0.2))), // Green background
        border: Border {
            color: Color::from_rgb(0.3, 0.6, 0.1), // Darker green border
            width: 1.5,
            radius: Radius::from(5.0),
        },
        text_color: Color::BLACK, // Black text for contrast
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.3), // Subtle shadow
            offset: Vector::new(0.0, 1.0),
            blur_radius: 5.0,
        },
    }
}

fn dropdown_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))), // Dark background
        border: Border {
            color: Color::from_rgb(0.0, 0.6, 0.6), // Aqua border color
            width: 2.0,
            radius: Radius::from(5.0),
        },
        text_color: Some(Color::WHITE), // White text color
        shadow: Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5), // Slight shadow for depth
            offset: Vector::new(0.0, 2.0),
            blur_radius: 10.0,
        },
    }
}

fn delete_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.8, 0.2, 0.2))), // Soft red background
        border: Border {
            color: Color::from_rgb(0.6, 0.1, 0.1), // Dark red border
            width: 2.0,
            radius: Radius::from(5.0),
        },
        text_color: Color::WHITE, // White text for contrast
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 3.0),
            blur_radius: 5.0,
        },
    }
}
