use std::sync::Arc;

use crate::infrastructure::container_injection::InfraContainer;

use super::settings_service::{SettingsService, SettingsServiceImpl};

pub struct ServiceContainer {
    pub settings_service: Arc<dyn SettingsService>,
}

impl ServiceContainer {
    pub fn new(infra_container: &Arc<InfraContainer>) -> Self {
        let settings_service = Arc::new(SettingsServiceImpl::new(
            infra_container.mock_repository.clone(),
        ));

        Self {
            settings_service: settings_service,
        }
    }
}
