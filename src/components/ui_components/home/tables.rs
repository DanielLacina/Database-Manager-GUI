use crate::components::business_components::{
    component::{BColumn, BDataType, BTable, BTableIn, BusinessComponent},
    components::BusinessHome,
};
use crate::components::ui_components::{
    component::{Event, UIComponent},
    events::Message,
    home::events::{HomeMessage, TablesMessage},
};
use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
};
use regex::Regex;

#[derive(Debug, Clone, Default)]
pub struct TablesUI {
    pub table_filter: String,
    pub show_create_table_form: bool,
    pub create_table_input: BTableIn,
    pub tables: Option<Vec<BTable>>,
}

impl UIComponent for TablesUI {
    type EventType = TablesMessage;

    async fn initialize_component(&mut self) {}

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::UpdateTableFilter(input) => {
                self.table_filter = input;
                Task::none()
            }
            Self::EventType::ShowCreateTableForm => {
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
            Self::EventType::TableCreated(tables) => {
                self.show_create_table_form = false;
                self.create_table_input = BTableIn::default();
                self.tables = tables;
                Task::none()
            }
        }
    }
}

impl TablesUI {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn initialize_tables(&mut self, tables: Option<Vec<BTable>>) {
        self.tables = tables;
    }

    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut row = Row::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .spacing(20)
            .padding(20);

        row = row.push(self.tables_section());
        row = row.push(self.create_table_section());

        container(row)
            .height(Length::Fill)
            .width(Length::Fill)
            .into()
    }

    /// Renders the section for displaying tables and filtering
    fn tables_section<'a>(&'a self) -> Element<'a, Message> {
        let mut tables_display = Column::new().spacing(10).padding(10);

        // Add the tables list and filter input
        tables_display = tables_display.push(self.table_filter_input());
        tables_display = tables_display.push(self.tables_container());

        let show_create_table_form_button = button(if self.show_create_table_form {
            "Remove create table form"
        } else {
            "Show create table form"
        })
        .on_press(TablesMessage::message(TablesMessage::ShowCreateTableForm));

        tables_display = tables_display.push(show_create_table_form_button);

        // Wrap in a scrollable container
        scrollable(
            container(tables_display)
                .padding(10)
                .style(|_| container::Style {
                    background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))), // Dark Gray border color
                    ..Default::default()
                }),
        )
        .width(400)
        .height(500)
        .into()
    }

    /// Creates the search filter input for filtering tables
    fn table_filter_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Search Tables", &self.table_filter)
            .on_input(|input| TablesMessage::message(TablesMessage::UpdateTableFilter(input)))
            .width(300)
            .padding(10)
            .into()
    }

    /// Creates a container to list all tables
    fn tables_container<'a>(&'a self) -> Element<'a, Message> {
        if let Some(tables) = &self.tables {
            let mut tables_column = Column::new().spacing(10).padding(10);

            let table_filter_pattern = self.get_table_filter_regex();

            let tables_filtered: Vec<_> = tables
                .iter()
                .filter(|table| table_filter_pattern.is_match(&table.table_name))
                .collect();

            for table in tables_filtered {
                tables_column = tables_column.push(text(&table.table_name));
            }

            scrollable(container(tables_column).padding(10))
                .height(300)
                .width(350)
                .into()
        } else {
            container(text("Loading"))
                .height(300)
                .width(350)
                .padding(10)
                .into()
        }
    }

    /// Creates the form section for creating a new table
    fn create_table_section<'a>(&'a self) -> Element<'a, Message> {
        let mut create_form = Column::new().spacing(10).padding(10);

        if self.show_create_table_form {
            create_form = create_form.push(self.create_table_form());
        }

        container(create_form).padding(10).into()
    }

    /// Creates the form to create a new table
    fn create_table_form<'a>(&'a self) -> Element<'a, Message> {
        let mut form = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(10)
            .spacing(10);

        form = form.push(self.table_name_input());

        form = form.push(self.table_form_columns());

        let add_column_button = button("Add Column")
            .on_press(TablesMessage::message(TablesMessage::AddColumn))
            .padding(10);

        let create_table_button = button("Create table")
            .on_press(HomeMessage::message(HomeMessage::SubmitCreateTable(
                self.create_table_input.clone(),
            )))
            .padding(10);

        form = form.push(add_column_button);
        form = form.push(row![create_table_button]);

        container(form)
            .padding(10)
            .style(|_| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.3, 0.3, 0.3))), // Dark Gray border color
                ..Default::default()
            })
            .into()
    }

    fn table_form_columns<'a>(&'a self) -> Element<'a, Message> {
        let mut table_columns = Column::new();
        for (index, column) in self.create_table_input.columns.iter().enumerate() {
            table_columns = table_columns.push(self.column_input_row(index, column));
        }
        scrollable(table_columns).height(500).into()
    }

    /// Creates the input field for the table name
    fn table_name_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Table Name", &self.create_table_input.table_name)
            .on_input(|value| TablesMessage::message(TablesMessage::UpdateTableName(value)))
            .width(350)
            .padding(10)
            .into()
    }

    /// Creates a row of inputs for a column (name, data type, and remove button)
    fn column_input_row<'a>(&'a self, index: usize, column: &'a BColumn) -> Element<'a, Message> {
        let name_input = text_input("Column Name", &column.name)
            .on_input(move |value| {
                TablesMessage::message(TablesMessage::UpdateColumnName(index, value))
            })
            .width(200);

        let datatype_input = PickList::new(
            vec![BDataType::TEXT, BDataType::INT, BDataType::TIMESTAMP],
            Some(&column.datatype),
            move |value| TablesMessage::message(TablesMessage::UpdateColumnType(index, value)),
        )
        .placeholder("Data Type")
        .width(150);

        let remove_button = button("Remove")
            .on_press(TablesMessage::message(TablesMessage::RemoveColumn(index)))
            .padding(5);

        row![name_input, datatype_input, remove_button]
            .spacing(10)
            .padding(2)
            .into()
    }

    /// Creates a regex pattern for filtering tables
    fn get_table_filter_regex(&self) -> Regex {
        Regex::new(&format!(r"(?i){}", &self.table_filter)).unwrap_or_else(|error| {
            eprintln!("{}", error);
            Regex::new(r"").unwrap()
        })
    }
}
