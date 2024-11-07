use crate::components::business_components::{
    component::BusinessComponent, components::BusinessComponents,
};
use crate::components::ui_components::home::home::HomeUI;

pub type HomeUIComponent = HomeUI;

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

#[derive(Debug, Clone)]
pub struct UIComponents {
    pub home_ui: HomeUIComponent,
}

impl UIComponents {
    pub async fn new() -> Self {
        let business_components = BusinessComponents::new().await;
        Self {
            home_ui: HomeUI::new(business_components.home),
        }
    }
}
