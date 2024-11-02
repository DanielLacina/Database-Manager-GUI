use crate::database::Repository;
use iced::{
    widget::{button, column, container, text, Column},
    Task,
};

#[derive(Debug, Clone)]
pub struct Home {
    repository: Repository,
}

impl Home {
    pub fn new(repository: Repository) -> Self {
        Self {
            repository: repository.clone(),
        }
    }
}
