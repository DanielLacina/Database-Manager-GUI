use crate::components::business_components::{
    component::{BColumn, BDataType, BTable, BTableIn, BusinessComponent},
    components::BusinessHome,
};
use crate::components::ui_components::{
    component::UIComponent, events::Message, home::events::HomeMessage,
};
use iced::{
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row, Text,
    },
    Alignment, Element, Length, Task,
};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct HomeUI {
    pub home: BusinessHome,
    pub table_filter: String,
    pub show_create_table_form: bool,
    pub create_table_input: BTableIn,
}

#[derive(Debug, Clone)]
pub enum ColumnMessage {
    NameChanged(usize, String),
    DatatypeChanged(usize, String),
    AddColumn,
    RemoveColumn(usize),
}

impl UIComponent for HomeUI {
    type EventType = HomeMessage;

    async fn initialize_component(&mut self) {
        self.home.initialize_component().await;
    }

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::InitializeComponent => {
                let mut home_ui = self.clone();
                Task::perform(
                    async move {
                        home_ui.initialize_component().await;
                        home_ui
                    },
                    |home_ui_initialized| {
                        Message::Home(Self::EventType::ComponentInitialized(home_ui_initialized))
                    },
                )
            }
            Self::EventType::ComponentInitialized(home_ui_initialized) => {
                *self = home_ui_initialized;
                Task::none()
            }
            Self::EventType::UpdateTableFilter(input) => {
                self.table_filter = input;
                Task::none()
            }
            Self::EventType::ShowCreateTableForm => {
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
            Self::EventType::SubmitCreateTable => {
                let mut home = self.home.clone();
                let create_table_input = self.create_table_input.clone();
                Task::perform(
                    async move {
                        home.add_table(create_table_input).await;
                        home
                    },
                    |home| Message::Home(Self::EventType::HomeTablesUpdated(home)),
                )
            }
            Self::EventType::HomeTablesUpdated(home) => {
                self.home = home;
                self.show_create_table_form = false;
                self.create_table_input = BTableIn::default();
                Task::none()
            }
        }
    }
}

impl HomeUI {
    pub fn new(home: BusinessHome) -> Self {
        Self {
            home,
            table_filter: String::new(),
            show_create_table_form: false,
            create_table_input: BTableIn::default(),
        }
    }

    /// Main content function that combines all UI components
    pub fn content<'a>(&'a self) -> Element<'a, Message> {
        let mut row = Row::new();
        row = row.push(self.tables());
        row = row.push(self.title());
        container(row).into()
    }

    /// Renders the title section
    fn title<'a>(&'a self) -> Element<'a, Message> {
        let title_text = if let Some(title) = &self.home.title {
            title
        } else {
            "Loading"
        };
        container(text(title_text)).into()
    }

    /// Renders the tables section with optional filtering and the create table form
    fn tables<'a>(&'a self) -> Element<'a, Message> {
        let mut tables_display = Column::new();
        tables_display = tables_display.push(self.tables_container());
        tables_display = tables_display.push(self.table_filter_input());

        let show_create_table_form_button = button(if self.show_create_table_form {
            "Remove create table form"
        } else {
            "Show create table form"
        })
        .on_press(Message::Home(HomeMessage::ShowCreateTableForm));

        tables_display = tables_display.push(show_create_table_form_button);

        if self.show_create_table_form {
            tables_display = tables_display.push(self.create_table_form());
        }

        container(tables_display).into()
    }

    /// Creates the search filter input for filtering tables
    fn table_filter_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Search", &self.table_filter)
            .on_input(|input| Message::Home(HomeMessage::UpdateTableFilter(input)))
            .width(300)
            .into()
    }

    /// Creates a container to list all tables
    fn tables_container<'a>(&'a self) -> Element<'a, Message> {
        if let Some(tables) = &self.home.tables {
            let mut tables_column = Column::new()
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10);

            let table_filter_pattern = self.get_table_filter_regex();

            let tables_filtered: Vec<_> = tables
                .iter()
                .filter(|table| table_filter_pattern.is_match(&table.table_name))
                .collect();

            for table in tables_filtered {
                tables_column = tables_column.push(text(&table.table_name));
            }

            container(tables_column).height(250).width(300).into()
        } else {
            container(text("Loading"))
                .height(Length::Fill)
                .width(Length::Fill)
                .padding(10)
                .into()
        }
    }

    /// Creates a regex pattern for filtering tables
    fn get_table_filter_regex(&self) -> Regex {
        Regex::new(&format!(r"(?i){}", &self.table_filter)).unwrap_or_else(|error| {
            eprintln!("{}", error);
            Regex::new(r"").unwrap()
        })
    }

    /// Creates the form to create a new table
    fn create_table_form<'a>(&'a self) -> Element<'a, Message> {
        let mut form = Column::new()
            .height(Length::Fill)
            .width(Length::Fill)
            .padding(10)
            .spacing(10);

        form = form.push(self.table_name_input());

        for (index, column) in self.create_table_input.columns.iter().enumerate() {
            form = form.push(self.column_input_row(index, column));
        }

        let add_column_button = button("Add Column")
            .on_press(Message::Home(HomeMessage::AddColumn))
            .padding(10);

        let create_table_button = button("Create table")
            .on_press(Message::Home(HomeMessage::SubmitCreateTable))
            .padding(10);

        form = form.push(add_column_button);
        form = form.push(row![create_table_button]);

        container(form).into()
    }

    /// Creates the input field for the table name
    fn table_name_input<'a>(&'a self) -> Element<'a, Message> {
        text_input("Table Name", &self.create_table_input.table_name)
            .on_input(|value| Message::Home(HomeMessage::UpdateTableName(value)))
            .width(400)
            .into()
    }

    /// Creates a row of inputs for a column (name, data type, and remove button)
    fn column_input_row<'a>(&'a self, index: usize, column: &'a BColumn) -> Element<'a, Message> {
        let name_input = text_input("Column Name", &column.name)
            .on_input(move |value| Message::Home(HomeMessage::UpdateColumnName(index, value)))
            .width(200);

        let datatype_input = PickList::new(
            vec![BDataType::TEXT, BDataType::INT, BDataType::DATETIME],
            Some(&column.datatype),
            move |value| Message::Home(HomeMessage::UpdateColumnType(index, value)),
        )
        .placeholder("Data Type")
        .width(200);

        let remove_button = button("Remove")
            .on_press(Message::Home(HomeMessage::RemoveColumn(index)))
            .padding(5);

        row![name_input, datatype_input, remove_button]
            .spacing(10)
            .into()
    }
}
