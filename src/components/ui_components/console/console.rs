use crate::components::ui_components::component::UIComponent;
use crate::components::ui_components::console::events::ConsoleMessage;
use crate::components::ui_components::events::Message;
use iced::{
    border,
    border::Radius,
    font::Font,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, Container, PickList,
        Row, Scrollable, Text, TextInput,
    },
    Alignment, Background, Border, Color, Element, Length, Shadow, Task, Theme, Vector,
};

#[derive(Debug, Clone)]
pub struct Console {
    messages: Vec<String>,
}

impl UIComponent for Console {
    type EventType = ConsoleMessage;

    fn update(&mut self, message: Self::EventType) -> Task<Message> {
        match message {
            Self::EventType::LogMessage(message) => {
                self.messages.push(message);
                Task::none()
            }
        }
    }
}

impl Console {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn content(&self) -> Scrollable<'_, Message> {
        let mut console_display = Column::new().spacing(10).padding(10);

        for message in &self.messages {
            // Create a `Text` widget with wrapping and styled
            let text_widget = Text::new(message)
                .size(16)
                .width(Length::Fill)
                .color(Color::from_rgb(0.8, 0.8, 0.8)); // Light gray text color

            // Add padding and background for each message to make it look like a console
            let message_container = Container::new(text_widget)
                .padding(10)
                .width(Length::Fill)
                .style(|_| console_message_style());

            console_display = console_display.push(message_container);
        }

        // Wrap the column in a scrollable container with a fixed height
        scrollable(
            container(console_display)
                .style(|_| console_style())
                .height(Length::Fill)
                .padding(10),
        )
        .height(Length::Fill) // Fixed height for the console window
        .width(400)
        .style(|_, _| scrollbar_style()) // Apply custom scrollbar style
    }
}

// ======================== STYLES ========================

// Style for the individual console messages
fn console_message_style() -> container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(Color::from_rgb(0.15, 0.15, 0.15))),
        text_color: Some(Color::from_rgb(0.8, 0.8, 0.8)), // Light gray text
        border: Border {
            color: Color::from_rgb(0.4, 0.4, 0.4),
            width: 1.0,
            radius: Radius::from(5.0),
        },
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 2.0),
            blur_radius: 3.0,
        },
    }
}

// Style for the overall console container
fn console_style() -> container::Style {
    iced::widget::container::Style {
        background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
        border: Border {
            color: Color::from_rgb(0.3, 0.3, 0.3),
            width: 2.0,
            radius: Radius::new(0),
        },
        text_color: Some(Color::from_rgb(0.9, 0.9, 0.9)),
        shadow: Shadow {
            color: Color::BLACK,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 6.0,
        },
    }
}

fn scrollbar_style() -> scrollable::Style {
    scrollable::Style {
        container: container::Style {
            text_color: None,
            background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: Radius::from(0.0),
            },
            shadow: Shadow {
                color: Color::TRANSPARENT,
                offset: Vector::new(0.0, 0.0),
                blur_radius: 0.0,
            },
        },
        vertical_rail: scrollable::Rail {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
            border: Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: Radius::from(3.0),
            },
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.6, 0.6, 0.6),
                border: Border {
                    color: Color::from_rgb(0.4, 0.4, 0.4),
                    width: 1.0,
                    radius: Radius::from(3.0),
                },
            },
        },
        horizontal_rail: scrollable::Rail {
            background: Some(Background::Color(Color::from_rgb(0.2, 0.2, 0.2))),
            border: Border {
                color: Color::from_rgb(0.3, 0.3, 0.3),
                width: 1.0,
                radius: Radius::from(3.0),
            },
            scroller: scrollable::Scroller {
                color: Color::from_rgb(0.6, 0.6, 0.6),
                border: Border {
                    color: Color::from_rgb(0.4, 0.4, 0.4),
                    width: 1.0,
                    radius: Radius::from(3.0),
                },
            },
        },
        gap: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
    }
}
