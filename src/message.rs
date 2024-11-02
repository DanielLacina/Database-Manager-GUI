use crate::components::Components;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(Components),
}
