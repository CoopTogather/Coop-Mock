use std::{collections::HashMap, ops::Deref, sync::Arc};

use coop_service::{container::AppContainer, errors::CustomError};
use endpoint_handler::{
    caching::{TemplateCaching, TemplateCachingImpl},
    endpoint_template::{Template, TemplateImpl},
};
use poem::Request;

pub struct DatabaseMockHandlerImpl {
    caching: Arc<TemplateCachingImpl>,
    app_container: Arc<AppContainer>,
}

pub trait MockEndpointHandler {
    fn handle_mock_request(&self, request: &Request) -> Result<impl Template, CustomError>;
}

impl DatabaseMockHandlerImpl {
    pub async fn new(container: Arc<AppContainer>) -> Result<Self, CustomError> {
        let caching = Arc::new(TemplateCachingImpl::default());
        let app_container = container.clone();

        Ok(Self {
            caching,
            app_container,
        })
    }

    async fn retreive_templates_from_datasource(
        &self,
        request: &Request,
    ) -> Result<dyn Template, CustomError> {
        let path = request.uri().path().to_owned();
        let container = self.app_container.clone().deref();
        let mock_endpoint_service = container
            .services_container
            .clone()
            .settings_service
            .clone();

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

            self.caching.add_template_vec(scope.as_str(), template);
        }

        
    }

    fn group_templates_by_first_scope(
        templates: Vec<TemplateImpl>,
    ) -> HashMap<String, Vec<TemplateImpl>> {
        let mut grouped_templates: HashMap<String, Vec<TemplateImpl>> = HashMap::new();

        for template in templates {
            let first_scope = template.paths.first().unwrap();
            let entry = grouped_templates
                .entry(first_scope.scope.to_owned())
                .or_insert(Vec::new());

            entry.push(template);
        }

        grouped_templates
    }
}

impl MockEndpointHandler for DatabaseMockHandlerImpl {
    fn handle_mock_request(&self, request: &Request) -> Result<impl Template, CustomError> {
        let path = request.uri().path().to_owned();
        let method = request.method().to_string();

        let templates = self.caching.get_templates(path.as_str());

        if templates.is_empty() {
            self.retreive_templates_from_datasource(request)?;
        }

        let templates = self.caching.get_templates(path.as_str());

        for template in templates {
            if template.matches(path.as_str(), method.as_str()) {
                return Ok(template);
            }
        }

        Err(CustomError::NotFound)
    }
}
