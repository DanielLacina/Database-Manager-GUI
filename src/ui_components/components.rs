use crate::business_components::component::BusinessComponent;
use crate::ui_components::home::HomeUI;

#[derive(Debug, Clone)]
pub enum CurrentComponent {
    Home,
}

pub struct UIComponents {
    home_ui: HomeUI,
}

pub async fn initialize_component<T: BusinessComponent>(mut business_component: T) -> T {
    business_component.initialize_component().await;
    business_component
}
