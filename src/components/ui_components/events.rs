use crate::components::ui_components::components::{HomeUIComponent, UIComponents};

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(UIComponents),
    InitializeHomeComponent,
    HomeComponentInitialized(HomeUIComponent),
}
