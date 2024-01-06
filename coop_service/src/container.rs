use std::sync::Arc;

use crate::{
    infrastructure::container_injection::InfraContainer,
    services::container_injection::ServiceContainer,
};

pub struct AppContainer {
    pub infra_container: Arc<InfraContainer>,
    pub services_container: Arc<ServiceContainer>,
}

impl AppContainer {
    pub async fn new() -> Self {
        let infra_container = Arc::new(InfraContainer::new().await);
        let services_container = Arc::new(ServiceContainer::new(&infra_container));

        Self {
            infra_container,
            services_container,
        }
    }
}
