use super::parameter::{TemplateScopeParameter, TemplateScopeParameterImpl};

pub struct TemplatePathImpl {
    pub scope: String,
    pub parameter: Option<TemplateScopeParameterImpl>,
}

pub trait TemplatePath {
    fn matches(&self, path_scope: &str) -> bool;
}

impl TemplatePathImpl {
    pub fn new(scope: &str) -> Self {
        let mut scope = scope.to_string();
        let mut parameter: Option<TemplateScopeParameterImpl> = None;

        if scope.starts_with('{') && scope.ends_with('}') {
            parameter = TemplateScopeParameterImpl::new(scope.as_str());
        } else {
            scope = scope;
        }

        Self { scope, parameter }
    }
}

impl TemplatePath for TemplatePathImpl {
    fn matches(&self, path_scope: &str) -> bool {
        if self.parameter.is_some() {
            self.parameter.as_ref().unwrap().matches(path_scope)
        } else {
            self.scope == path_scope
        }
    }
}
