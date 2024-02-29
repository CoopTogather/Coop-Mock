use crate::{
    domain::models::endpoints::{CreateEndpointDto, EndpointDto},
    errors::CustomError,
};

#[async_trait::async_trait]
pub trait EndpointService: Sync + Send {
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), CustomError>;
    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, CustomError>;
    async fn get_mocks(&self) -> Result<Vec<EndpointDto>, CustomError>;
}