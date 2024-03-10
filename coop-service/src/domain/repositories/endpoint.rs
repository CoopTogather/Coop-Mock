use crate::domain::models::endpoints::{CreateEndpointDto, EndpointDto};

#[async_trait::async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn create_mock(&self, add_endpoint: CreateEndpointDto) -> Result<(), &str>;

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<EndpointDto>, &str>;

    async fn get_mocks(&self) -> Result<Vec<EndpointDto>, &str>;

    async fn get_mocks_by_scope(&self, scope: &str) -> Result<Vec<EndpointDto>, &str>;
}
