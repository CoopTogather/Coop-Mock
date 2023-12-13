use std::sync::Arc;

use crate::{
    domain::models::settings::CreateSettings,
    infrastructure::repositories::settings_repository::SettingsRepository,
};

pub struct SettingsServiceImpl {
    pub settings_repository: Arc<dyn SettingsRepository>,
}

#[async_trait::async_trait]
pub trait SettingsService: Sync + Send {
    async fn create_mock(&self, settings: CreateSettings) -> Result<usize, &str>;
}

impl SettingsServiceImpl {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self { settings_repository }
    }
}

#[async_trait::async_trait]
impl SettingsService for SettingsServiceImpl {
    async fn create_mock(&self, settings: CreateSettings) -> Result<usize, &str> {
        let result = self.settings_repository.create_mock(settings).await;

        result
    }
}
