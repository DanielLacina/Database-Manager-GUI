use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::home::HomeUI;

#[derive(Debug, Clone)]
pub enum Message {
    InitializeComponents(UIComponents),
    InitializeHomeComponent,
    HomeComponentInitialized(HomeUI),
}

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

pub struct UIComponents {
    home_ui: HomeUI,
}

impl UIComponents {
    pub async fn new() -> Self {
        let business_components = BusinessComponents::new().await;
        Self {
            home_ui: HomeUI::new(business_components.home),
        }
    }
}

pub async fn initialize_component<T: BusinessComponent>(mut business_component: T) -> T {
    business_component.initialize_component().await;
    business_component
}
