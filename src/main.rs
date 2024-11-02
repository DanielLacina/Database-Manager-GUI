mod database;

use iced::widget::{button, column, container, text, Column};

#[derive(Debug, Clone, Copy)]
enum Message {
    Increment,
    Decrement,
}

#[derive(Default)]
struct Crm;

impl Crm {
    fn view(&self) -> Column<Message> {
        column![container("hello")]
    }
    fn update(&mut self, message: Message) {}
}

fn main() -> iced::Result {
    iced::application("A cool counter", Crm::update, Crm::view).run()
}
