use actix_web::Error;
use std::sync::Arc;

use crate::infrastructure::repositories::mock_repository::MockRepository;

pub struct MockServiceImpl {
    pub mock_repository: Arc<dyn MockRepository>,
}

#[async_trait::async_trait]
pub trait MockService: Sync + Send {
    async fn create_mock(&self) -> Result<(), Error>;
}

impl MockServiceImpl {
    pub fn new(mock_repository: Arc<dyn MockRepository>) -> Self {
        Self { mock_repository }
    }
}

#[async_trait::async_trait]
impl MockService for MockServiceImpl {
    async fn create_mock(&self) -> Result<(), Error> {
        Ok(())
    }
}
