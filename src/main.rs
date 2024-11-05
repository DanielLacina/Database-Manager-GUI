mod components;

use crate::components::ui_components::{
    component::initialize_ui_component,
    components::{CurrentComponent, Message, UIComponents},
    home::HomeUI,
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
        if self.components.is_none() {
            column![container("loading")].into()
        } else {
            let components = self.components.clone().unwrap();
            match self.current_component {
                CurrentComponent::Home => {
                    let home_component = components.home_ui;
                    home_component.content()
                }
            }
        }
    }
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InitializeComponents(components) => {
                self.components = Some(components);
                Task::done(Message::InitializeHomeComponent)
            }
            Message::InitializeHomeComponent => {
                let home_component = self.components.clone().unwrap().home_ui;
                Task::perform(
                    async move { initialize_ui_component::<HomeUI>(home_component).await },
                    |home_ui| Message::HomeComponentInitialized(home_ui),
                )
            }
            Message::HomeComponentInitialized(home_ui) => {
                if let Some(components) = &mut self.components {
                    components.home_ui = home_ui;
                }
                Task::none()
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
