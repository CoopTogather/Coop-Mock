use std::{result, sync::Arc};

use crate::{
    domain::{
        models::endpoints::{CreateEndpointDto, EndpointDto},
        repositories::endpoint::EndpointRepository,
        services::endpoint::EndpointService,
    },
    errors::{self, CustomError},
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
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), CustomError> {
        let result = self.settings_repository.create_mock(settings).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }

    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, CustomError> {
        let result = self.settings_repository.get_mock(id).await;

        match result {
            Ok(endpoint) => Ok(endpoint),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }

    async fn get_mocks(&self) -> Result<Vec<EndpointDto>, errors::CustomError> {
        let result = self.settings_repository.get_mocks().await;

        match result {
            Ok(endpoints) => Ok(endpoints),
            Err(err) => Err(errors::CustomError::ServiceError(err.to_string())),
        }
    }
}
