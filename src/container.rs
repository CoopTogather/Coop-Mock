use crate::services::mocks_service::MockService;

pub struct AppContainer {
    pub mocks_service: MockService,
}

impl AppContainer {
    pub fn new() -> Self {
        Self {
            mocks_service: MockService,
        }
    }
}

impl Default for AppContainer {
    fn default() -> Self {
        Self::new()
    }
}
