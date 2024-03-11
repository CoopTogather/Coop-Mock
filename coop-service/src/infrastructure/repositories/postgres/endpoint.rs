use std::{ops::Deref, sync::Arc};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    domain::{
        models::{
            endpoints::{CreateEndpointDto, EndpointDto, SearchEndpointDto, UpdateEndpointDto},
            InsertCommandModel, UpdateCommandModel,
        },
        repositories::endpoint::EndpointRepository,
    },
    errors::CustomError,
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
    async fn create_mock(&self, add_endpoint: CreateEndpointDto) -> Result<(), CustomError> {
        let active_model = add_endpoint.to_entity_model();

        let connection_pool = self.pool.deref();

        active_model.insert(connection_pool).await?;

        Ok(())
    }

    async fn get_mock(&self, endpoint_id: i32) -> Result<Option<EndpointDto>, CustomError> {
        let connection_pool = self.pool.deref();

        let endpoint = end_points::Entity::find_by_id(endpoint_id)
            .one(connection_pool)
            .await?;

        Ok(EndpointDto::from_option(endpoint))
    }

    async fn get_mocks(
        &self,
        search_params: SearchEndpointDto,
    ) -> Result<Vec<EndpointDto>, CustomError> {
        let connection_pool = self.pool.deref();
        let mut filter = end_points::Column::Enabled.eq(true);

        if search_params.name.is_some() {
            filter = filter.and(end_points::Column::Name.contains(search_params.name.unwrap()));
        }

        if search_params.path.is_some() {
            filter = filter.and(end_points::Column::Path.contains(search_params.path.unwrap()));
        }

        let endpoints = end_points::Entity::find()
            .filter(filter)
            .all(connection_pool)
            .await?;

        let endpoints_dto = endpoints
            .into_iter()
            .map(|model| EndpointDto::from(model))
            .collect();

        Ok(endpoints_dto)
    }

    async fn get_mocks_by_scope(&self, scope: &str) -> Result<Vec<EndpointDto>, CustomError> {
        let connection_pool = self.pool.deref();

        let endpoints = end_points::Entity::find()
            .filter(end_points::Column::Path.starts_with(scope))
            .all(connection_pool)
            .await?;

        let endpoints_dto = endpoints
            .into_iter()
            .map(|e| EndpointDto::from(e))
            .collect();

        Ok(endpoints_dto)
    }

    async fn update_mock(&self, update_dto: UpdateEndpointDto) -> Result<(), CustomError> {
        let connection_pool = self.pool.deref();

        let original_endpoint = end_points::Entity::find_by_id(update_dto.id)
            .one(connection_pool)
            .await?;

        if original_endpoint.is_none() {
            return Err(CustomError::ServiceError("Endpoint not found.".to_string()));
        }

        let original_endpoint = original_endpoint.unwrap().into_active_model();

        let update_endpoint = update_dto.set_update_active_model(original_endpoint);

        update_endpoint.update(connection_pool).await?;

        Ok(())
    }

    async fn delete_mock(&self, endpoint_id: i32) -> Result<(), CustomError> {
        let connection_pool = self.pool.deref();

        let endpoint = end_points::Entity::find_by_id(endpoint_id)
            .one(connection_pool)
            .await?;

        if endpoint.is_none() {
            return Err(CustomError::ServiceError("Endpoint not found.".to_string()));
        }

        let endpoint = endpoint.unwrap().into_active_model();

        endpoint.delete(connection_pool).await?;

        Ok(())
    }

    async fn toggle_mock(&self, endpoint_id: i32) -> Result<(), CustomError> {
        let connection_pool = self.pool.deref();

        let endpoint = end_points::Entity::find_by_id(endpoint_id)
            .one(connection_pool)
            .await?;

        if endpoint.is_none() {
            return Err(CustomError::ServiceError("Endpoint not found.".to_string()));
        }

        let mut endpoint = endpoint.unwrap().into_active_model();

        let enabled = endpoint.enabled.clone().unwrap();

        endpoint.enabled = Set(!enabled);

        endpoint.update(connection_pool).await?;

        Ok(())
    }
}
