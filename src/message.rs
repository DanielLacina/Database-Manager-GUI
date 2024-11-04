use crate::home::Home;
use crate::components::Components;


#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(Components),
    InitializeHomeComponent,
    HomeComponentInitialized(Home),
}
