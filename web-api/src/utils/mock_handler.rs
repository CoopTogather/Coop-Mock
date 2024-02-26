use std::{collections::HashMap, sync::Arc};

use coop_service::{container::AppContainer, domain::models::endpoints::EndpointDto};
use endpoint_handler::{
    caching::{TemplateCaching, TemplateCachingImpl},
    endpoint_template::TemplateImpl,
};
use poem::Endpoint;

pub struct MockEndpointsHandler {
    pub caching: Arc<TemplateCachingImpl>,
}

impl MockEndpointsHandler {
    pub async fn new(container: Arc<AppContainer>) -> Result<Self, &str> {
        let app_container = container.clone();
        let caching = Arc::new(TemplateCachingImpl::default());
        let mock_endpoint_service = app_container.services_container.settings_service.clone();

        let mock_data = mock_endpoint_service.get_mocks().await?;

        let templates: Vec<TemplateImpl> = mock_data
            .to_owned()
            .iter()
            .map(|mock| TemplateImpl::from(mock.to_owned()))
            .collect();

        let grouped_templates = Self::group_templates_by_first_scope(templates);

        for (scope, temp) in grouped_templates.iter() {
            let scope = scope.to_owned();
            let template = temp.to_owned();

            caching.add_range(scope.as_str(), template);
        }

        Ok(Self { caching })
    }

    fn group_templates_by_first_scope(
        templates: Vec<TemplateImpl>,
    ) -> HashMap<String, Vec<TemplateImpl>> {
        let mut grouped_templates: HashMap<String, Vec<TemplateImpl>> = HashMap::new();

        for template in templates {
            let first_scope = template.paths.first().unwrap();
            let entry = grouped_templates
                .entry(first_scope.scope)
                .or_insert(Vec::new());

            entry.push(template);
        }

        grouped_templates
    }
}
