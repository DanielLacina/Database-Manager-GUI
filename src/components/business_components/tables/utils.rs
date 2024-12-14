use crate::components::business_components::component::{
    repository_module::BRepository, BTableGeneral,
};
use std::sync::Arc;
use tokio::sync::Mutex as AsyncMutex;

pub async fn set_tables_general_info(
    repository: Arc<BRepository>,
    tables_general_info: Arc<AsyncMutex<Vec<BTableGeneral>>>,
) {
    let mut locked_tables_general_info = tables_general_info.lock().await;
    let new_tables_general_info = repository.get_general_tables_info().await.unwrap();
    let new_tables_general_info_structured = new_tables_general_info
        .into_iter()
        .map(|table| BTableGeneral::to_table(table))
        .collect();
    *locked_tables_general_info = new_tables_general_info_structured;
}
