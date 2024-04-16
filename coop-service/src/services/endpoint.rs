use std::sync::Arc;

use crate::{
    domain::{
        models::endpoints::{CreateEndpointDto, EndpointDto, SearchEndpointDto, UpdateEndpointDto},
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

    async fn get_mocks(
        &self,
        search_dto: SearchEndpointDto,
    ) -> Result<Vec<EndpointDto>, errors::CustomError> {
        let result = self.settings_repository.get_mocks(search_dto).await;

        match result {
            Ok(endpoints) => Ok(endpoints),
            Err(err) => Err(errors::CustomError::ServiceError(err.to_string())),
        }
    }

    async fn get_mocks_by_scope(&self, scope: &str) -> Result<Vec<EndpointDto>, CustomError> {
        let scope_add_slash = format!("/{}", scope);
        let result = self
            .settings_repository
            .get_mocks_by_scope(scope_add_slash.as_str())
            .await;

        match result {
            Ok(endpoints) => Ok(endpoints),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }

    async fn update_mock(&self, settings: UpdateEndpointDto) -> Result<(), CustomError> {
        let result = self.settings_repository.update_mock(settings).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }

    async fn delete_mock(&self, id: i32) -> Result<(), CustomError> {
        let result = self.settings_repository.delete_mock(id).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }

    async fn toggle_mock(&self, id: i32) -> Result<(), CustomError> {
        let result = self.settings_repository.toggle_mock(id).await;

        match result {
            Ok(_) => Ok(()),
            Err(err) => Err(CustomError::ServiceError(err.to_string())),
        }
    }
}
