use std::sync::Arc;

use crate::{
    infrastructure::{
        databases::{get_connection_string, postgre::create_db_pool},
        repositories::settings_repository::SettingsRepositoryImpl,
    },
    services::settings_service::{MockService, MockServiceImpl},
};

pub struct AppContainer {
    pub mocks_service: Arc<dyn MockService>,
}

impl AppContainer {
    pub fn new() -> Self {
        let postgre_connection_pool =
            create_db_pool(get_connection_string("DATABASE_URL".to_string()));

        let mock_repository = Arc::new(SettingsRepositoryImpl::new(Arc::new(
            postgre_connection_pool.clone(),
        )));

        Self {
            mocks_service: Arc::new(MockServiceImpl::new(mock_repository.clone())),
        }
    }
}

impl Default for AppContainer {
    fn default() -> Self {
        Self::new()
    }
}
