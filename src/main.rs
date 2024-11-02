mod database;

use crate::database::Repository;
use iced::{
    widget::{button, column, container, text, Column},
    Task,
};

#[derive(Debug, Clone)]
enum Message {
    InitializeRepository(Repository),
}

struct Crm {
    repository: Option<Repository>,
}

impl Crm {
    fn new() -> (Self, Task<Message>) {
        (
            Self { repository: None },
            Task::perform(Repository::new(), |repository| {
                Message::InitializeRepository(repository)
            }),
        )
    }
    fn title(&self) -> String {
        String::from("CRM")
    }
    fn view(&self) -> Column<Message> {
        column![container("hello")]
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeRepository(repository) => {
                self.repository = Some(repository);
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(Crm::title, Crm::update, Crm::view).run_with(Crm::new)
}
