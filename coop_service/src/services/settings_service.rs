use std::sync::Arc;

use crate::{
    domain::models::{
        endpoints::{CreateEndpointDto, EndpointDto},
        CommandModel,
    },
    infrastructure::repositories::settings_repository::SettingsRepository,
};

pub struct SettingsServiceImpl {
    pub settings_repository: Arc<dyn SettingsRepository>,
}

#[async_trait::async_trait]
pub trait SettingsService: Sync + Send {
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), &str>;
    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, &str>;
}

impl SettingsServiceImpl {
    pub fn new(settings_repository: Arc<dyn SettingsRepository>) -> Self {
        Self {
            settings_repository,
        }
    }
}

#[async_trait::async_trait]
impl SettingsService for SettingsServiceImpl {
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), &str> {
        let result = self
            .settings_repository
            .create_mock(settings.to_active_model())
            .await;

        result
    }

    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, &str> {
        let result = self.settings_repository.get_mock(id).await;

        match result {
            Ok(endpoint) => Ok(EndpointDto::from(endpoint)),
            Err(err) => Err(err),
        }
    }
}
