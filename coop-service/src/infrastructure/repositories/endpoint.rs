use std::{ops::Deref, sync::Arc};

use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    domain::repositories::endpoint::EndpointRepository, infrastructure::models::end_points,
};

pub struct EndpointRepositoryImpl {
    pub pool: Arc<DatabaseConnection>,
}

impl EndpointRepositoryImpl {
    pub fn new(connection_pool: Arc<DatabaseConnection>) -> Self {
        Self {
            pool: connection_pool,
        }
    }
}

#[async_trait::async_trait]
impl EndpointRepository for EndpointRepositoryImpl {
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

        let endpoints = end_points::Entity::find()
            .filter(end_points::Column::Enabled.eq(true))
            .all(connection_pool)
            .await;

        match endpoints {
            Ok(endpoints) => Ok(endpoints),
            Err(err) => {
                println!("Error getting endpoints: {:?}", err);
                Err("Error getting endpoints")
            }
        }
    }
}
