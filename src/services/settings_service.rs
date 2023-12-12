use std::sync::Arc;

use poem::{post, Route};

use crate::{
    domain::models::settings::CreateSettings,
    infrastructure::repositories::settings_repository::SettingsRepository,
};

pub struct MockServiceImpl {
    pub mock_repository: Arc<dyn SettingsRepository>,
}

#[async_trait::async_trait]
pub trait MockService: Sync + Send {
    async fn create_mock(&self, settings: CreateSettings) -> Result<usize, &str>;
}

impl MockServiceImpl {
    pub fn new(mock_repository: Arc<dyn SettingsRepository>) -> Self {
        Self { mock_repository }
    }
}

#[async_trait::async_trait]
impl MockService for MockServiceImpl {
    async fn create_mock(&self, settings: CreateSettings) -> Result<usize, &str> {
        let result = self.mock_repository.create_mock(settings).await;

        result
    }
}
