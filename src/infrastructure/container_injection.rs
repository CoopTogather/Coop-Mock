use std::sync::Arc;

use super::{
    databases::{get_connection_string, postgre::create_db_pool},
    repositories::settings_repository::{SettingsRepository, SettingsRepositoryImpl},
};

use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

pub struct InfraContainer {
    pub shared_connection_pool: Arc<Pool<ConnectionManager<PgConnection>>>,
    pub mock_repository: Arc<dyn SettingsRepository>,
}

impl InfraContainer {
    pub fn new() -> Self {
        let postgre_connection_pool =
            create_db_pool(get_connection_string("DATABASE_URL".to_string()));
        let shared_connection_pool = Arc::new(postgre_connection_pool);

        let mock_repository = Arc::new(SettingsRepositoryImpl::new(shared_connection_pool.clone()));

        Self {
            shared_connection_pool: shared_connection_pool,
            mock_repository: mock_repository,
        }
    }
}
