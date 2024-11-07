mod components;

use crate::components::ui_components::{
    component::UIComponent,
    components::{CurrentComponent, UIComponents},
    events::Message,
};
use iced::{
    widget::{button, column, container, row, text, Column, Text},
    Element, Settings, Task, Theme,
};

struct Crm {
    current_component: CurrentComponent,
    components: Option<UIComponents>,
}

impl Crm {
    fn setup() -> (Self, Task<Message>) {
        (
            Self {
                current_component: CurrentComponent::Home,
                components: None,
            },
            Task::perform(UIComponents::new(), |components| {
                Message::InitializeComponents(components)
            }),
        )
    }
    fn title(&self) -> String {
        String::from("CRM")
    }
    fn theme(&self) -> Theme {
        Theme::Dark
    }
    fn view(&self) -> Element<'_, Message> {
        if let Some(components) = &self.components {
            match self.current_component {
                CurrentComponent::Home => components.home_ui.content(),
            }
        } else {
            column![container("loading")].into()
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeComponents(components) => {
                self.components = Some(components);
                Task::done(Message::InitializeHomeComponent)
            }
            Message::InitializeHomeComponent => {
                if let Some(components) = &self.components {
                    let mut home_ui = components.home_ui.clone();
                    Task::perform(
                        async move {
                            home_ui.initialize_component().await;
                            home_ui
                        },
                        |home_ui| Message::HomeComponentInitialized(home_ui),
                    )
                } else {
                    Task::none()
                }
            }

            Message::HomeComponentInitialized(home_ui) => {
                if let Some(components) = &mut self.components {
                    components.home_ui = home_ui;
                    Task::none()
                } else {
                    Task::none()
                }
            }
        }
    }
}

fn main() -> iced::Result {
    iced::application(Crm::title, Crm::update, Crm::view)
        .settings(Settings::default())
        .theme(Crm::theme)
        .run_with(Crm::setup)
}
