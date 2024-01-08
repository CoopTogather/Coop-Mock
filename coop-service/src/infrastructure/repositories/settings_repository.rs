use std::{ops::Deref, sync::Arc};

use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait};

use crate::infrastructure::models::end_points;

pub struct SettingsRepositoryImpl {
    pub pool: Arc<DatabaseConnection>,
}

#[async_trait::async_trait]
pub trait SettingsRepository: Send + Sync {
    async fn create_mock(&self, add_endpoint: end_points::ActiveModel) -> Result<(), &str>;

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<end_points::Model>, &str>;

    async fn get_mocks(&self) -> Result<Vec<end_points::Model>, &str>;
}

impl SettingsRepositoryImpl {
    pub fn new(connection_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            pool: connection_pool,
        }
    }
}

#[async_trait::async_trait]
impl SettingsRepository for SettingsRepositoryImpl {
    async fn create_mock(&self, add_endpoint: end_points::ActiveModel) -> Result<(), &str> {
        let connection_pool = self.pool.deref();

        match add_endpoint.insert(connection_pool).await {
            Ok(endpoint) => {
                println!("Endpoint created: {:?}", endpoint);
            }
            Err(err) => {
                println!("Error creating endpoint: {:?}", err);
            }
        };

        Ok(())
    }

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<end_points::Model>, &str> {
        let connection_pool = self.pool.deref();

        let endpoint = end_points::Entity::find_by_id(endpoint_id)
            .one(connection_pool)
            .await;

        match endpoint {
            Ok(endpoint) => Ok(endpoint),
            Err(err) => {
                println!("Error getting endpoint: {:?}", err);
                Err("Error getting endpoint")
            }
        }
    }

    async fn get_mocks(&self) -> Result<Vec<end_points::Model>, &str> {
        let connection_pool = self.pool.deref();

        let endpoints = end_points::Entity::find().all(connection_pool).await;

        match endpoints {
            Ok(endpoints) => Ok(endpoints),
            Err(err) => {
                println!("Error getting endpoints: {:?}", err);
                Err("Error getting endpoints")
            }
        }
    }
}
