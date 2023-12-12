use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    RunQueryDsl,
};

use crate::domain::models::settings::CreateSettings;

pub struct SettingsRepositoryImpl {
    pub pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn create_mock(&self, create_settings: CreateSettings) -> Result<usize, &str>;
}

impl<'registry> SettingsRepositoryImpl {
    pub fn new(connection_pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>) -> Self {
        Self {
            pool: connection_pool,
        }
    }
}

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn create_mock(&self, create_settings: CreateSettings) -> Result<usize, &str> {
        use crate::{
            infrastructure::models::settings::CreateEndpointSettingsDiesel,
            schema::endpoints_setting,
        };
        let mut connection = self.pool.get().unwrap();
        let result = diesel::insert_into(endpoints_setting::dsl::endpoints_setting)
            .values(CreateEndpointSettingsDiesel::from(create_settings))
            .execute(&mut connection)
            .map_err(|_| "Error creating mock");

        result
    }
}
