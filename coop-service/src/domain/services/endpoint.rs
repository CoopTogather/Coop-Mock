use crate::{
    domain::models::endpoints::{
        CreateEndpointDto, EndpointDto, SearchEndpointDto, UpdateEndpointDto,
    },
    errors::CustomError,
};

#[async_trait::async_trait]
pub trait EndpointService: Sync + Send {
    async fn create_mock(&self, settings: CreateEndpointDto) -> Result<(), CustomError>;
    async fn get_mock(&self, id: i32) -> Result<Option<EndpointDto>, CustomError>;
    async fn get_mocks(
        &self,
        search_dto: SearchEndpointDto,
    ) -> Result<Vec<EndpointDto>, CustomError>;
    async fn get_mocks_by_scope(&self, scope: &str) -> Result<Vec<EndpointDto>, CustomError>;
    async fn update_mock(&self, settings: UpdateEndpointDto) -> Result<(), CustomError>;
    async fn delete_mock(&self, id: i32) -> Result<(), CustomError>;
    async fn toggle_mock(&self, id: i32) -> Result<(), CustomError>;
}
