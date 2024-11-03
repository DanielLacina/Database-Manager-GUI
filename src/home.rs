use crate::database::Repository;

#[derive(Debug, Clone)]
pub struct Home {
    repository: Repository,
}

pub struct HomeContent {
    pub title: String,
}

impl Home {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }

    pub fn content(&self) -> HomeContent {
        HomeContent {
            title: String::from("Home Component"),
        }
    }
}
