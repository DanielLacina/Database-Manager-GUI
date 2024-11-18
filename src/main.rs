mod components;
use crate::components::ui_components::{
    component::{Event, UIComponent},
    components::{ComponentsMessage, CurrentComponent, UIComponents},
    events::Message,
};
use iced::{
    widget::{button, column, container, row, text, Column, Row, Text},
    Element, Settings, Task, Theme,
};

pub struct Crm {
    components: Option<UIComponents>,
}

impl Crm {
    pub fn setup() -> (Self, Task<Message>) {
        (
            Self { components: None },
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
            match components.current_component {
                CurrentComponent::Home => {
                    let mut display = Row::new();

                    // Add the main content
                    display = display.push(Column::new().push(components.tables_ui.content()));

                    // Add the "Show Console" button
                    display = display.push(
                        button(if components.show_console {
                            "Remove Console"
                        } else {
                            "Show Console"
                        })
                        .on_press(ComponentsMessage::message(
                            ComponentsMessage::ShowOrRemoveConsole,
                        )),
                    );

                    // Conditionally add the console content
                    if components.show_console {
                        display = display.push(components.console.content());
                    }

                    display.into()
                }
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
                            UIComponents::initialize_startup_components_message()
                        }
                        _ => Task::none(),
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
            Message::Tables(tables_message) => {
                if let Some(components) = &mut self.components {
                    components.tables_ui.update(tables_message)
                } else {
                    Task::none()
                }
            }
            Message::Console(console_message) => {
                if let Some(components) = &mut self.components {
                    components.console.update(console_message)
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
