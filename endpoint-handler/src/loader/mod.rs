use async_trait::async_trait;
use coop_service::domain::models::endpoints::EndpointDto;

#[async_trait]
pub trait TemplateLoader: Send + Sync {
    async fn load_templates() -> Result<(), Vec<EndpointDto>>;
}

pub struct TemplateLoaderImpl {}
