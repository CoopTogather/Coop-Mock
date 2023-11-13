use std::sync::Arc;

use diesel::r2d2::{ConnectionManager, Pool};

pub struct MockRepositoryImpl {
    pub pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

#[async_trait::async_trait]
pub trait MockRepository: Send + Sync {
    async fn create_mock(&self) -> Result<(), &str>;
}

impl MockRepositoryImpl {
    pub fn new(connection_pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>) -> Self {
        Self {
            pool: connection_pool,
        }
    }
}

#[async_trait::async_trait]
impl MockRepository for MockRepositoryImpl {
    async fn create_mock(&self) -> Result<(), &str> {
        let mut connection = self.pool.get().unwrap();

        run()
        Ok(())
    }
}
