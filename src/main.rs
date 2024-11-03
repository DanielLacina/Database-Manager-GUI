mod component;
mod components;
mod database;
mod home;

use crate::components::{Components, CurrentComponent};
use iced::{
    widget::{button, column, container, text, Column},
    Task,
};

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(Components),
}

struct Crm {
    current_component: CurrentComponent,
    components: Option<Components>,
}

impl Crm {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                current_component: CurrentComponent::Home,
                components: None,
            },
            Task::perform(Components::new(), |components| {
                Message::InitializeComponents(components)
            }),
        )
    }
    fn title(&self) -> String {
        String::from("CRM")
    }
    fn view(&self) -> Column<Message> {
        if self.components.is_none() {
            column![container("loading")].into()
        } else {
            let components = self.components.clone().unwrap();
            match self.current_component {
                CurrentComponent::Home => {
                    let home_component = components.home;
                }
            }
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeComponents(components) => {
                self.components = Some(components);
                Task::none()
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(Crm::title, Crm::update, Crm::view).run_with(Crm::new)
}
