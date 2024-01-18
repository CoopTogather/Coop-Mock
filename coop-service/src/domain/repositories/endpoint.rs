use crate::infrastructure::models::end_points;

#[async_trait::async_trait]
pub trait EndpointRepository: Send + Sync {
    async fn create_mock(&self, add_endpoint: end_points::ActiveModel) -> Result<(), &str>;

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<end_points::Model>, &str>;

    async fn get_mocks(&self) -> Result<Vec<end_points::Model>, &str>;
}
