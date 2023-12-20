use std::sync::Arc;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    RunQueryDsl,
};

use crate::infrastructure::models::settings::CreateEndpointSettingsDiesel;

pub struct SettingsRepositoryImpl {
    pub pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>,
}

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn create_mock(
        &self,
        create_settings: CreateEndpointSettingsDiesel,
    ) -> Result<usize, &str>;
}

impl SettingsRepositoryImpl {
    pub fn new(connection_pool: Arc<Pool<ConnectionManager<diesel::PgConnection>>>) -> Self {
        Self {
            pool: connection_pool,
        }
    }
}

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn create_mock(
        &self,
        create_settings: CreateEndpointSettingsDiesel,
    ) -> Result<usize, &str> {
        use crate::infrastructure::schema::endpoints_setting;

        let mut connection = self.pool.get().unwrap();
        let result = diesel::insert_into(endpoints_setting::dsl::endpoints_setting)
            .values(create_settings)
            .execute(&mut connection)
            .map_err(|_| "Error creating mock");

        result
    }
}
