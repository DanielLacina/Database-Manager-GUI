use crate::database::Repository;
use crate::message::Message;
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

    pub fn content(&self) -> Column<Message> {
        column!(container("home screen"))
    }
}
