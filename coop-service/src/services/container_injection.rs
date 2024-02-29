use std::sync::Arc;

use crate::{
    domain::services::endpoint::EndpointService,
    infrastructure::container_injection::InfraContainer,
};

use super::endpoint::EndpointServiceImpl;

pub struct ServiceContainer {
    pub settings_service: Arc<dyn EndpointService>,
}

impl ServiceContainer {
    pub fn new(infra_container: &Arc<InfraContainer>) -> Self {
        let settings_service = Arc::new(EndpointServiceImpl::new(
            infra_container.mock_repository.clone(),
        ));

        Self {
            settings_service: settings_service,
        }
    }
}
