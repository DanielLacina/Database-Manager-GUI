mod components;

use crate::components::ui_components::{
    component::UIComponent,
    components::{ComponentsMessage, CurrentComponent, UIComponents},
    events::Message,
};
use iced::{
    widget::{button, column, container, row, text, Column, Text},
    Element, Settings, Task, Theme,
};

pub struct Crm {
    current_component: CurrentComponent,
    components: Option<UIComponents>,
}

impl Crm {
    pub fn setup() -> (Self, Task<Message>) {
        (
            Self {
                current_component: CurrentComponent::Home,
                components: None,
            },
            Task::perform(UIComponents::new(), |components| {
                Message::Components(ComponentsMessage::InitializeComponents(components))
            }),
        )
    }
    pub fn title(&self) -> String {
        String::from("CRM")
    }
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
    pub fn view(&self) -> Element<'_, Message> {
        if let Some(components) = &self.components {
            match self.current_component {
                CurrentComponent::Home => components.home_ui.content(),
            }
        } else {
            column![container("loading")].into()
        }
    }
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Components(components_message) => {
                if let Some(components) = &mut self.components {
                    components.update(components_message)
                } else {
                    match components_message {
                        ComponentsMessage::InitializeComponents(ui_components) => {
                            self.components = Some(ui_components);
                            UIComponents::initialized_task_message()
                        }
                    }
                }
            }
            Message::Home(home_message) => {
                if let Some(components) = &mut self.components {
                    components.home_ui.update(home_message)
                } else {
                    Task::none()
                }
            }
        }
    }
}

pub fn main() -> iced::Result {
    iced::application(Crm::title, Crm::update, Crm::view)
        .settings(Settings::default())
        .theme(Crm::theme)
        .run_with(Crm::setup)
}
