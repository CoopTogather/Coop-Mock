use std::sync::Arc;

use crate::domain::{
    models::{
        endpoints::{CreateEndpointDto, EndpointDto},
        CommandModel,
    },
    repositories::endpoint::EndpointRepository,
    services::endpoint::EndpointService,
};

pub struct EndpointServiceImpl {
    pub settings_repository: Arc<dyn EndpointRepository>,
}

impl EndpointServiceImpl {
    pub fn new(settings_repository: Arc<dyn EndpointRepository>) -> Self {
        Self {
            settings_repository,
        }
    }
}

#[async_trait::async_trait]
impl EndpointService for EndpointServiceImpl {
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

    async fn get_mocks(&self) -> Result<Vec<Option<EndpointDto>>, &str> {
        let result = self.settings_repository.get_mocks().await;

        match result {
            Ok(endpoints) => Ok(endpoints
                .into_iter()
                .map(|e| EndpointDto::from(Some(e)))
                .collect()),
            Err(err) => Err(err),
        }
    }
}
