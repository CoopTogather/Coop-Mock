use std::{collections::HashMap, ops::Deref, sync::Arc};

use coop_service::{container::AppContainer, errors::CustomError};
use endpoint_handler::{
    caching::{TemplateCaching, TemplateCachingImpl},
    endpoint_template::{Template, TemplateImpl},
    utils::path::first_scope,
};
use poem::{Request, Response};

pub struct DatabaseMockHandlerImpl {
    caching: Arc<dyn TemplateCaching>,
    app_container: Arc<AppContainer>,
}

#[async_trait::async_trait]
pub trait MockEndpointHandler: Send + Sync {
    async fn handle_mock_request(&self, request: &Request) -> Option<Response>;
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

    async fn try_get_template(&self, request: &Request) -> Option<impl Template> {
        let template = self.try_get_template_from_cache(request);

        if template.is_some() {
            return template;
        }

        let template = self
            .try_get_template_from_datasource_and_update_cache(request)
            .await
            .ok();

        template
    }

    fn try_get_template_from_cache(&self, request: &Request) -> Option<TemplateImpl> {
        let path = request.uri().path().to_owned();
        let method = request.method().to_string();

        self.caching.find_template(path.as_str(), method.as_str())
    }

    async fn try_get_template_from_datasource_and_update_cache(
        &self,
        request: &Request,
    ) -> Result<TemplateImpl, CustomError> {
        let path = request.uri().path().to_owned();
        let first_scope = match first_scope(path.as_str()) {
            Some(scope) => scope,
            None => return Err(CustomError::ServiceError("No scope found".to_owned())),
        };

        let container = self.app_container.deref();
        let endpoints_result = container
            .services_container
            .settings_service
            .get_mocks_by_scope(first_scope)
            .await?;

        let templates: Vec<TemplateImpl> = endpoints_result
            .to_owned()
            .iter()
            .map(|mock| TemplateImpl::from(mock.to_owned()))
            .collect();

        let grouped_templates = Self::group_templates_by_first_scope(&templates);

        for (scope, temp) in grouped_templates.iter() {
            let scope = scope.to_owned();
            let template = temp.to_owned();

            self.caching.add_template_vec(scope.as_str(), template);
        }

        let target_template = templates
            .iter()
            .find(|t| t.matches(path.as_str(), request.method().as_str()))
            .cloned();

        match target_template {
            Some(template) => Ok(template),
            None => Err(CustomError::ServiceError("No template found".to_owned())),
        }
    }

    fn group_templates_by_first_scope(
        templates: &Vec<TemplateImpl>,
    ) -> HashMap<String, Vec<TemplateImpl>> {
        let mut grouped_templates: HashMap<String, Vec<TemplateImpl>> = HashMap::new();

        for template in templates {
            let first_scope = template.paths.first().unwrap();
            let entry = grouped_templates
                .entry(first_scope.scope.to_owned())
                .or_insert(Vec::new());

            entry.push(template.to_owned());
        }

        grouped_templates
    }
}

#[async_trait::async_trait]
impl MockEndpointHandler for DatabaseMockHandlerImpl {
    async fn handle_mock_request(&self, request: &Request) -> Option<Response> {
        let template = self.try_get_template(request).await;

        if template.is_some() {
            let template = template.unwrap();

            return Some(template.into_response());
        }

        None
    }
}
