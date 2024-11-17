use crate::components::ui_components::component::UIComponent;
use crate::components::ui_components::console::events::ConsoleMessage;
use crate::components::ui_components::events::Message;
use iced::{
    border,
    font::Font,
    widget::{
        button, column, container, row, scrollable, text, text_input, Column, PickList, Row,
        Scrollable, Text, TextInput,
    },
    Alignment, Background, Border, Color, Element, Length, Task, Theme,
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
        let mut console_display = Column::new();

        for message in &self.messages {
            // Create a `Text` widget with wrapping
            let text_widget = Text::new(message)
                .width(300) // Set the maximum width before wrapping
                .size(16); // Set the font size if needed

            console_display = console_display.push(text_widget);
        }

        scrollable(console_display).height(400)
    }
}
