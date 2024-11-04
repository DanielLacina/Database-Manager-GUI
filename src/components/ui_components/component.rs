pub trait UIComponent {
    async fn initialize_component(&mut self);
}

pub async fn initialize_ui_component<T: UIComponent>(mut ui_component: T) -> T {
    ui_component.initialize_component().await;
    ui_component
}
