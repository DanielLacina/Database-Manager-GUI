use crate::components::business_components::component::{
    BColumn, BConstraint, BDataType, BTableGeneralInfo, BTableIn, BusinessComponent,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    tables::events::CreateTableFormMessage,
    tables::foreign_key_dropdown::{ForeignKeyDropDownUI, ForeignKeyDropdownEvents},
};
use iced::{
    alignment,
    alignment::{Alignment, Vertical},
    border::Radius,
    widget::{
        button, checkbox, column, container, row, scrollable, text, text_input, Button, Checkbox,
        Column, PickList, Row, Text, TextInput,
    },
    Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};
use std::iter::zip;

#[derive(Debug, Clone)]
pub struct CreateTableFormUI {
    create_table_input: BTableIn,
    pub tables_general_info: Option<Vec<BTableGeneralInfo>>,
    active_foreign_key_table_within_dropdown: Option<String>, // table in foreign key dropdown that has its columns displayed
    active_foreign_key_dropdown_column: Option<usize>, // column index that wants the foreign key dropdown
                                                       // activated
}

impl UIComponent for CreateTableFormUI {
    type EventType = CreateTableFormMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::AddColumn => {
                self.create_table_input.columns.push(BColumn::default());
                Task::none()
            }
            Self::EventType::RemoveColumn(index) => {
                if index < self.create_table_input.columns.len() {
                    self.create_table_input.columns.remove(index);
                }
                Task::none()
            }
            Self::EventType::UpdateColumnName(index, input) => {
                if let Some(column) = self.create_table_input.columns.get_mut(index) {
                    column.name = input;
                }
                Task::none()
            }
            Self::EventType::UpdateColumnType(index, input) => {
                if let Some(column) = self.create_table_input.columns.get_mut(index) {
                    column.datatype = input;
                }
                Task::none()
            }
            Self::EventType::SetOrRemovePrimaryKey(index) => {
                if let Some(column) = self.create_table_input.columns.get_mut(index) {
                    if let Some(existing_index) = column
                        .constraints
                        .iter()
                        .position(|constraint| matches!(constraint, BConstraint::PrimaryKey))
                    {
                        column.constraints.remove(existing_index);
                    } else {
                        column.constraints.push(BConstraint::PrimaryKey);
                    }
                }
                Task::none()
            }
            Self::EventType::AddForeignKey(
                index,
                referenced_table_name,
                referenced_column_name,
            ) => {
                if let Some(column) = self.create_table_input.columns.get_mut(index) {
                    if let Some(existing_index) = column.constraints.iter().position(|constraint| {
                        matches!(
                            constraint,
                            BConstraint::ForeignKey(existing_table_name, existing_column_name)
                        )
                    }) {
                        // Remove the foreign key constraint if it exists
                        column.constraints.remove(existing_index);
                        column.constraints.push(BConstraint::ForeignKey(
                            referenced_table_name,
                            referenced_column_name,
                        ));
                    } else {
                        // Add the foreign key constraint if it does not exist
                        column.constraints.push(BConstraint::ForeignKey(
                            referenced_table_name,
                            referenced_column_name,
                        ));
                    }
                }

                self.active_foreign_key_dropdown_column = None;
                self.active_foreign_key_table_within_dropdown = None;
                Task::none()
            }
            Self::EventType::RemoveForeignKey(index) => {
                if let Some(column) = self.create_table_input.columns.get_mut(index) {
                    if let Some(existing_index) = column.constraints.iter().position(|constraint| {
                        matches!(
                            constraint,
                            BConstraint::ForeignKey(existing_table_name, existing_column_name)
                        )
                    }) {
                        column.constraints.remove(existing_index);
                    }
                }
                self.active_foreign_key_dropdown_column = None;
                self.active_foreign_key_table_within_dropdown = None;

                Task::none()
            }
            Self::EventType::UpdateTableName(input) => {
                self.create_table_input.table_name = input;
                Task::none()
            }
            Self::EventType::TableCreated(tables, table_name) => {
                self.create_table_input = BTableIn::default();
                Task::none()
            }
            Self::EventType::SubmitCreateTable(create_table_input) => Task::none(),
            Self::EventType::ShowOrRemoveCreateTableForm => {
                if self.create_table_input.columns.len() == 0 {
                    for _ in 0..1 {
                        self.create_table_input.columns.push(BColumn {
                            name: String::from("id"),
                            datatype: BDataType::INTEGER,
                            constraints: vec![BConstraint::PrimaryKey],
                        });
                    }
                }
                Task::none()
            }
            Self::EventType::ToggleForeignKeyDropdown(index) => {
                // Toggle the dropdown for the specified column
                if self.active_foreign_key_dropdown_column == Some(index) {
                    self.active_foreign_key_dropdown_column = None;
                } else {
                    self.active_foreign_key_dropdown_column = Some(index);
                }
                Task::none()
            }
            Self::EventType::ToggleForeignKeyTable(_, table_name) => {
                // Toggle the column list for the specified table
                if self.active_foreign_key_table_within_dropdown == Some(table_name.clone()) {
                    self.active_foreign_key_table_within_dropdown = None;
                } else {
                    self.active_foreign_key_table_within_dropdown = Some(table_name);
                }
                Task::none()
            }
        }
    }
}

impl CreateTableFormUI {
    pub fn new(tables_general_info: Option<Vec<BTableGeneralInfo>>) -> Self {
        Self {
            create_table_input: BTableIn::default(),
            tables_general_info,
            active_foreign_key_dropdown_column: None,
            active_foreign_key_table_within_dropdown: None,
        }
    }

    // ======================== SECTION: Create Table ========================

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let create_form = Column::new()
            .spacing(20)
            .padding(20)
            .push(self.create_table_form());

        container(create_form)
            .padding(20)
            .style(|_| container_style())
            .into()
    }

    fn create_table_form<'a>(&'a self) -> Element<'a, Message> {
        let mut form = Column::new().spacing(15).padding(15);
        form = form
            .push(self.table_name_input())
            .push(self.table_form_columns())
            .push(self.add_column_button())
            .push(self.create_table_button());

        form.into()
    }

    // ======================== COMPONENTS ========================

    fn table_name_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Enter Table Name", &self.create_table_input.table_name)
            .on_input(|value| {
                <CreateTableFormUI as UIComponent>::EventType::UpdateTableName(value).message()
            })
            .width(Length::Fill)
            .padding(10)
            .style(|_, _| text_input_style())
            .into()
    }

    fn table_form_columns<'a>(&'a self) -> Element<'a, Message> {
        let columns_list = self.create_table_input.columns.iter().enumerate().fold(
            Column::new().spacing(10),
            |columns_list, (index, column)| columns_list.push(self.column_input_row(index, column)),
        );

        scrollable(columns_list)
            .height(Length::Fill)
            .direction(scrollable::Direction::Both {
                vertical: scrollable::Scrollbar::new(),
                horizontal: scrollable::Scrollbar::new(),
            })
            .into()
    }

    fn column_input_row<'a>(&'a self, index: usize, column: &'a BColumn) -> Element<'a, Message> {
        Row::new()
            .spacing(10)
            .align_y(Vertical::Center)
            .push(self.column_name_input(index, &column.name))
            .push(self.data_type_picker(index, &column.datatype))
            .push(self.primary_key_checkbox(index, column))
            .push(self.foreign_key_button(index))
            .push(self.remove_column_button(index))
            .into()
    }

    fn foreign_key_button<'a>(&self, index: usize, column: &'a BColumn) -> Element<'a, Message> {
        let foreign_key_dropdown_events = ForeignKeyDropdownEvents {
            add_foreign_key: Box::new(move |referenced_table, referenced_column| {
                <CreateTableFormUI as UIComponent>::EventType::AddForeignKey(
                    index,
                    referenced_table,
                    referenced_column,
                )
                .message()
            }),
            remove_foreign_key: <CreateTableFormUI as UIComponent>::EventType::RemoveForeignKey(
                index,
            )
            .message(),
            toggle_foreign_key_dropdown:
                <CreateTableFormUI as UIComponent>::EventType::ToggleForeignKeyDropdown(index)
                    .message(),
            toggle_foreign_key_table: Box::new(move |table_name| {
                <CreateTableFormUI as UIComponent>::EventType::ToggleForeignKeyTable(
                    index, table_name,
                )
                .message()
            }),
        };

        let foreign_key_dropdown = ForeignKeyDropDownUI::new(
            column.clone(),
            self.tables_general_info.clone(),
            foreign_key_dropdown_events,
        );

        foreign_key_dropdown.content()
    }
    fn column_name_input<'a>(&'a self, index: usize, name: &str) -> TextInput<'a, Message> {
        text_input("Column Name", name)
            .on_input(move |value| {
                <CreateTableFormUI as UIComponent>::EventType::UpdateColumnName(index, value)
                    .message()
            })
            .width(200)
            .style(|_, _| text_input_style())
    }

    fn data_type_picker<'a>(&'a self, index: usize, datatype: &BDataType) -> Element<'a, Message> {
        PickList::new(
            vec![BDataType::TEXT, BDataType::INTEGER, BDataType::TIMESTAMP],
            Some(datatype.clone()),
            move |value| {
                <CreateTableFormUI as UIComponent>::EventType::UpdateColumnType(index, value)
                    .message()
            },
        )
        .width(150)
        .into()
    }

    fn primary_key_checkbox<'a>(
        &'a self,
        index: usize,
        column: &'a BColumn,
    ) -> Checkbox<'a, Message> {
        checkbox(
            "Primary Key",
            column.constraints.contains(&BConstraint::PrimaryKey),
        )
        .on_toggle(move |_| {
            <CreateTableFormUI as UIComponent>::EventType::SetOrRemovePrimaryKey(index).message()
        })
    }

    fn remove_column_button<'a>(&'a self, index: usize) -> Button<'a, Message> {
        button("Remove")
            .style(|_, _| delete_button_style())
            .padding(10)
            .on_press(<CreateTableFormUI as UIComponent>::EventType::RemoveColumn(index).message())
    }

    fn add_column_button(&self) -> Button<'_, Message> {
        button("➕ Add Column")
            .style(|_, _| button_style())
            .padding(10)
            .on_press(<CreateTableFormUI as UIComponent>::EventType::AddColumn.message())
    }

    fn create_table_button(&self) -> Button<'_, Message> {
        button("🛠️ Create Table")
            .style(|_, _| create_button_style())
            .padding(15)
            .on_press(
                <CreateTableFormUI as UIComponent>::EventType::SubmitCreateTable(
                    self.create_table_input.clone(),
                )
                .message(),
            )
    }
}
// ======================== STYLES ========================
fn container_style() -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))), // Background color
        border: Border {
            color: Color::TRANSPARENT,
            width: 1.5,
            radius: Radius::from(5.0),
        },
        text_color: Some(Color::WHITE), // Text color for the content inside the container
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 5.0,
        },
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
fn button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.75, 0.65))),
        border: Border {
            color: Color::from_rgb(0.0, 0.6, 0.5),
            width: 2.0,
            radius: Radius::from(5.0),
        },
        text_color: Color::WHITE,
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 3.0),
            blur_radius: 5.0,
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

fn create_button_style() -> button::Style {
    button::Style {
        background: Some(Background::Color(Color::from_rgb(0.0, 0.5, 0.9))),
        border: Border {
            color: Color::from_rgb(0.0, 0.4, 0.7),
            width: 2.0,
            radius: Radius::from(8.0),
        },
        text_color: Color::WHITE,
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 3.0),
            blur_radius: 7.0,
        },
    }
}

fn text_input_style() -> text_input::Style {
    text_input::Style {
        background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)), // Darker input background
        border: Border {
            width: 1.5,
            color: Color::from_rgb(0.0, 0.74, 0.84),
            radius: Radius::from(5.0),
        },
        placeholder: Color::from_rgb(0.6, 0.6, 0.6), // Color for placeholder text
        value: Color::WHITE,                         // Color for input text
        selection: Color::from_rgb(0.0, 0.74, 0.84), // Color for selected text
        icon: Color::from_rgb(0.8, 0.8, 0.8),        // Color for any input icons
    }
}
