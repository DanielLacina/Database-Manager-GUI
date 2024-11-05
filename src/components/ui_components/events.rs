use crate::components::ui_components::{components::UIComponents, home::HomeUI};

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(UIComponents),
    InitializeHomeComponent,
    HomeComponentInitialized(HomeUI),
}
