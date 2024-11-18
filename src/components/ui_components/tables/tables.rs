use crate::components::business_components::{
    component::{BColumn, BDataType, BTable, BTableIn, BusinessComponent},
    components::BusinessTables,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    tables::{events::TablesMessage, table_info::TableInfoUI},
};
use iced::{
    alignment,
    border::Radius,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct TablesUI {
    pub table_filter: String,
    pub show_create_table_form: bool,
    pub create_table_input: BTableIn,
    pub tables: BusinessTables,
    pub single_table_info: Option<TableInfoUI>,
}

impl UIComponent for TablesUI {
    type EventType = TablesMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::UpdateTableFilter(input) => {
                self.table_filter = input;
                Task::none()
            }
            Self::EventType::ShowOrRemoveCreateTableForm => {
                if self.create_table_input.columns.len() == 0 {
                    for _ in 0..1 {
                        self.create_table_input.columns.push(BColumn::default());
                    }
                }
                self.show_create_table_form = !self.show_create_table_form;
                Task::none()
            }
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
            Self::EventType::UpdateTableName(input) => {
                self.create_table_input.table_name = input;
                Task::none()
            }
            Self::EventType::TableCreated(tables, table_name) => {
                self.show_create_table_form = false;
                self.create_table_input = BTableIn::default();
                self.tables = tables;
                Task::done(Self::EventType::message(
                    Self::EventType::GetSingleTableInfo(table_name),
                ))
            }
            Self::EventType::SubmitCreateTable(create_table_input) => {
                let mut tables = self.tables.clone();
                Task::perform(
                    async move {
                        tables.add_table(create_table_input.clone()).await;
                        (tables, create_table_input.table_name)
                    },
                    |table_tuple| {
                        let (tables, table_name) = table_tuple;
                        Self::EventType::message(Self::EventType::TableCreated(tables, table_name))
                    },
                )
            }
            Self::EventType::GetSingleTableInfo(table_name) => {
                let mut tables = self.tables.clone();

                Task::perform(
                    async move {
                        tables.set_table_info(table_name).await;
                        tables.table_info.unwrap()
                    },
                    |table_info| {
                        Self::EventType::message(Self::EventType::SetSingleTableInfo(table_info))
                    },
                )
            }
            Self::EventType::SetSingleTableInfo(table_info) => {
                self.tables.table_info = None; // object is no longer needed becasue logic is in
                                               // the table info ui component
                self.single_table_info = Some(TableInfoUI::new(table_info));
                Task::none()
            }
            Self::EventType::UndisplayTableInfo => {
                self.single_table_info = None;
                Task::none()
            }
            Self::EventType::SingleTableInfo(table_info_message) => {
                if let Some(table_info) = &mut self.single_table_info {
                    table_info.update(table_info_message)
                } else {
                    Task::none()
                }
            }
            Self::EventType::InitializeComponent => {
                let mut tables = self.tables.clone();
                Task::perform(
                    async move {
                        tables.initialize_component().await;
                        tables
                    },
                    |tables| TablesMessage::message(TablesMessage::ComponentInitialized(tables)),
                )
            }
            Self::EventType::ComponentInitialized(tables) => {
                self.tables = tables;
                Task::none()
            }
        }
    }
}

impl TablesUI {
    pub fn new(tables: BusinessTables) -> Self {
        Self {
            table_filter: String::default(),
            show_create_table_form: false,
            create_table_input: BTableIn::default(),
            tables,
            single_table_info: None,
        }
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut row = Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .spacing(20)
            .padding(20);

        row = row.push(self.tables_section());
        row = row.push(self.create_table_section());

        // Display single table info with an "Undisplay" button
        if let Some(table_info) = &self.single_table_info {
            let mut table_info_section = Column::new().spacing(10).padding(10);
            table_info_section = table_info_section.push(table_info.content());

            let undisplay_button = button("🔙 Back")
                .style(|_, _| button_style())
                .on_press(<TablesUI as UIComponent>::EventType::message(
                    <TablesUI as UIComponent>::EventType::UndisplayTableInfo,
                ))
                .padding(10);

            table_info_section = table_info_section.push(undisplay_button);

            row = row.push(container(table_info_section).width(Length::Fill));
        }

        container(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(20)
            .style(|_| container_style())
            .into()
    }

    // ======================== SECTION: Tables Display ========================

    fn tables_section<'a>(&'a self) -> Element<'a, Message> {
        let mut tables_display = Column::new().spacing(10).padding(10);
        tables_display = tables_display.push(self.table_filter_input());
        tables_display = tables_display.push(self.tables_container());

        let scrollable_section = scrollable(
            container(tables_display)
                .padding(10)
                .style(|_| container_style()),
        )
        .height(Length::Fill)
        .width(Length::Fill);

        let toggle_form_button = button(if self.show_create_table_form {
            "Remove create table form"
        } else {
            "Show create table form"
        })
        .style(|_, _| button_style())
        .on_press(<TablesUI as UIComponent>::EventType::message(
            <TablesUI as UIComponent>::EventType::ShowOrRemoveCreateTableForm,
        ))
        .padding(10);

        Column::new()
            .push(scrollable_section)
            .push(toggle_form_button)
            .spacing(10)
            .padding(10)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn table_filter_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Search Tables", &self.table_filter)
            .on_input(|input| {
                <TablesUI as UIComponent>::EventType::message(
                    <TablesUI as UIComponent>::EventType::UpdateTableFilter(input),
                )
            })
            .width(Length::Fill)
            .padding(10)
            .style(|_, _| text_input_style())
            .into()
    }

    fn tables_container<'a>(&'a self) -> Element<'a, Message> {
        if let Some(tables) = &self.tables.tables {
            let mut tables_column = Column::new().spacing(10).padding(10);
            let table_filter_pattern = self.get_table_filter_regex();

            for table in tables
                .iter()
                .filter(|t| table_filter_pattern.is_match(&t.table_name))
            {
                let table_button = button(text(&table.table_name)).on_press(
                    <TablesUI as UIComponent>::EventType::message(
                        <TablesUI as UIComponent>::EventType::GetSingleTableInfo(
                            table.table_name.clone(),
                        ),
                    ),
                );
                tables_column = tables_column.push(table_button);
            }

            scrollable(tables_column).height(Length::Fill).into()
        } else {
            container(text("Loading")).height(Length::Fill).into()
        }
    }

    // ======================== SECTION: Create Table ========================

    fn create_table_section<'a>(&'a self) -> Element<'a, Message> {
        let mut create_form = Column::new().spacing(20).padding(20);
        if self.show_create_table_form {
            create_form = create_form.push(self.create_table_form());
        }

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
            .on_press(<TablesUI as UIComponent>::EventType::message(
                <TablesUI as UIComponent>::EventType::AddColumn,
            ))
            .padding(10);
        form = form.push(add_column_button);

        let create_table_button = button("🛠️ Create Table")
            .style(|_, _| create_button_style())
            .on_press(<TablesUI as UIComponent>::EventType::message(
                <TablesUI as UIComponent>::EventType::SubmitCreateTable(
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
                <TablesUI as UIComponent>::EventType::message(
                    <TablesUI as UIComponent>::EventType::UpdateTableName(value),
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
                <TablesUI as UIComponent>::EventType::message(
                    <TablesUI as UIComponent>::EventType::UpdateColumnName(index, value),
                )
            })
            .width(Length::FillPortion(2))
            .style(|_, _| text_input_style());

        let datatype_input = PickList::new(
            vec![BDataType::TEXT, BDataType::INT, BDataType::TIMESTAMP],
            Some(&column.datatype),
            move |value| {
                <TablesUI as UIComponent>::EventType::message(
                    <TablesUI as UIComponent>::EventType::UpdateColumnType(index, value),
                )
            },
        )
        .width(Length::FillPortion(1));

        let remove_button = button("🗑️ Remove")
            .style(|_, _| button_style())
            .on_press(<TablesUI as UIComponent>::EventType::message(
                <TablesUI as UIComponent>::EventType::RemoveColumn(index),
            ))
            .padding(10);

        row![name_input, datatype_input, remove_button]
            .spacing(10)
            .into()
    }

    fn get_table_filter_regex(&self) -> Regex {
        Regex::new(&format!(r"(?i){}", self.table_filter))
            .unwrap_or_else(|_| Regex::new("").unwrap())
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
