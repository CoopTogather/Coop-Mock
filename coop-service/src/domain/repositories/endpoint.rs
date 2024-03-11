use crate::{
    domain::models::endpoints::{
        CreateEndpointDto, EndpointDto, SearchEndpointDto, UpdateEndpointDto,
    },
    errors::CustomError,
};

#[async_trait::async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn create_mock(&self, add_endpoint: CreateEndpointDto) -> Result<(), CustomError>;

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<EndpointDto>, CustomError>;

    async fn get_mocks(
        &self,
        search_params: SearchEndpointDto,
    ) -> Result<Vec<EndpointDto>, CustomError>;

    async fn get_mocks_by_scope(&self, scope: &str) -> Result<Vec<EndpointDto>, CustomError>;

    async fn update_mock(&self, update_dto: UpdateEndpointDto) -> Result<(), CustomError>;

    async fn delete_mock(&self, endpoint_id: i32) -> Result<(), CustomError>;

    async fn toggle_mock(&self, endpoint_id: i32) -> Result<(), CustomError>;
}
