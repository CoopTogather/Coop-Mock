use std::{ops::Deref, sync::Arc};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, ActiveModelTrait};

use crate::{
    domain::{
        models::{
            endpoints::{CreateEndpointDto, EndpointDto},
            CommandModel,
        },
        repositories::endpoint::EndpointRepository,
    },
    infrastructure::models::end_points,
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
    async fn create_mock(&self, add_endpoint: CreateEndpointDto) -> Result<(), &str> {
        let active_model = add_endpoint.to_entity_model();

        let connection_pool = self.pool.deref();

        match active_model.insert(connection_pool).await {
            Ok(endpoint) => {
                println!("Endpoint created: {:?}", endpoint);
            }
            Err(err) => {
                println!("Error creating endpoint: {:?}", err);
            }
        };

        Ok(())
    }

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<EndpointDto>, &str> {
        let connection_pool = self.pool.deref();

        let endpoint = end_points::Entity::find_by_id(endpoint_id)
            .one(connection_pool)
            .await;

        match endpoint {
            Ok(endpoint) => Ok(EndpointDto::from_option(endpoint)),
            Err(err) => {
                println!("Error getting endpoint: {:?}", err);
                Err("Error getting endpoint")
            }
        }
    }

    async fn get_mocks(&self) -> Result<Vec<EndpointDto>, &str> {
        let connection_pool = self.pool.deref();

        let endpoints = end_points::Entity::find()
            .filter(end_points::Column::Enabled.eq(true))
            .all(connection_pool)
            .await;

        match endpoints {
            Ok(endpoints) => Ok(endpoints
                .into_iter()
                .map(|e| EndpointDto::from(e))
                .collect()),
            Err(err) => {
                println!("Error getting endpoints: {:?}", err);
                Err("Error getting endpoints")
            }
        }
    }
}
