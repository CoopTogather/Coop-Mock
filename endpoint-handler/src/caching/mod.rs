use std::collections::HashMap;

use coop_service::domain::models::endpoints::EndpointDto;

use crate::endpoint_template::TemplateImpl;

pub struct TemplatesCaching {
    pub templates: HashMap<String, TemplateImpl>,
}

impl TemplatesCaching {
    pub fn new(endpoints: Vec<EndpointDto>) -> Self {
        let mut templates = HashMap::new();

        for endpoint in endpoints {
            templates.insert(endpoint.path.clone(), TemplateImpl::from(endpoint));
        }

        Self { templates }
    }
}
