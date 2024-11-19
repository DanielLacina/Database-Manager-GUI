use crate::components::business_components::component::{
    BColumn, BConstraint, BDataType, BTable, BTableIn, BusinessComponent,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    tables::events::CreateTableFormMessage,
};
use iced::{
    alignment,
    alignment::Vertical,
    border::Radius,
    widget::{
        button, checkbox, column, container, row, scrollable, text, text_input, Button, Checkbox,
        Column, PickList, Row, Text,
    },
    Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct CreateTableFormUI {
    create_table_input: BTableIn,
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
                        self.create_table_input.columns.push(BColumn::default());
                    }
                }
                Task::none()
            }
        }
    }
}

impl CreateTableFormUI {
    pub fn new() -> Self {
        Self {
            create_table_input: BTableIn::default(),
        }
    }

    // ======================== SECTION: Create Table ========================

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut create_form = Column::new().spacing(20).padding(20);
        create_form = create_form.push(self.create_table_form());

        container(create_form)
            .padding(20)
            .style(|_| container_style())
            .into()
    }

    fn create_table_form<'a>(&'a self) -> Element<'a, Message> {
        let mut form = Column::new().spacing(15).padding(15);
        form = form.push(self.table_name_input());
        form = form.push(self.table_form_columns());

        let add_column_button = button("➕ Add Column")
            .style(|_, _| button_style())
            .on_press(<CreateTableFormUI as UIComponent>::EventType::message(
                <CreateTableFormUI as UIComponent>::EventType::AddColumn,
            ))
            .padding(10);
        form = form.push(add_column_button);

        let create_table_button = button("🛠️ Create Table")
            .style(|_, _| create_button_style())
            .on_press(<CreateTableFormUI as UIComponent>::EventType::message(
                <CreateTableFormUI as UIComponent>::EventType::SubmitCreateTable(
                    self.create_table_input.clone(),
                ),
            ))
            .padding(15);

        form.push(
            Row::new()
                .push(
                    container(create_table_button)
                        .width(Length::Fill)
                        .align_x(alignment::Horizontal::Center), // Center the button horizontally
                )
                .width(Length::Fill),
        )
        .into()
    }

    fn table_name_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Enter Table Name", &self.create_table_input.table_name)
            .on_input(|value| {
                <CreateTableFormUI as UIComponent>::EventType::message(
                    <CreateTableFormUI as UIComponent>::EventType::UpdateTableName(value),
                )
            })
            .width(Length::Fill)
            .padding(10)
            .style(|_, _| text_input_style())
            .into()
    }

    fn table_form_columns<'a>(&'a self) -> Element<'a, Message> {
        let mut columns_list = Column::new().spacing(10);
        for (index, column) in self.create_table_input.columns.iter().enumerate() {
            columns_list = columns_list.push(self.column_input_row(index, column));
        }
        scrollable(columns_list).height(Length::Fill).into()
    }

    fn column_input_row<'a>(&'a self, index: usize, column: &'a BColumn) -> Element<'a, Message> {
        let name_input = text_input("Column Name", &column.name)
            .on_input(move |value| {
                <CreateTableFormUI as UIComponent>::EventType::message(
                    <CreateTableFormUI as UIComponent>::EventType::UpdateColumnName(index, value),
                )
            })
            .width(Length::FillPortion(2))
            .style(|_, _| text_input_style());

        let datatype_input = PickList::new(
            vec![BDataType::TEXT, BDataType::INT, BDataType::TIMESTAMP],
            Some(&column.datatype),
            move |value| {
                <CreateTableFormUI as UIComponent>::EventType::message(
                    <CreateTableFormUI as UIComponent>::EventType::UpdateColumnType(index, value),
                )
            },
        )
        .width(Length::FillPortion(1));

        let primary_key_checkbox: Checkbox<'_, Message, Theme, iced::Renderer> = checkbox(
            "Primary Key",
            column.constraints.contains(&BConstraint::PrimaryKey),
        )
        .on_toggle(move |_| {
            <CreateTableFormUI as UIComponent>::EventType::message(
                <CreateTableFormUI as UIComponent>::EventType::SetOrRemovePrimaryKey(index),
            )
        });

        let remove_button = button("🗑️ Remove")
            .style(|_, _| button_style())
            .on_press(<CreateTableFormUI as UIComponent>::EventType::message(
                <CreateTableFormUI as UIComponent>::EventType::RemoveColumn(index),
            ))
            .padding(10);

        row![
            name_input,
            datatype_input,
            primary_key_checkbox,
            remove_button
        ]
        .spacing(10)
        .into()
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
