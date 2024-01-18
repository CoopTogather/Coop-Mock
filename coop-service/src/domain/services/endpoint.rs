use crate::domain::models::endpoints::{CreateEndpointDto, EndpointDto};

#[async_trait::async_trait]
pub trait EndpointService: Sync + Send {
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), &str>;
    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, &str>;
    async fn get_mocks(&self) -> Result<Vec<Option<EndpointDto>>, &str>;
}
