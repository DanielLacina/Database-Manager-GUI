use crate::components::ui_components::components::UIComponents;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(UIComponents),
    InitializeHomeComponent,
    HomeComponentInitialized,
}
