#[derive(Debug, Clone)]
pub struct TemplateScopeParameterImpl {
    pub name: String,
    pub scope_type: TemplateScopeTypesEnum,
}

pub trait TemplateScopeParameter {
    fn matches(&self, path_scope: &str) -> bool;
}

#[derive(Debug, PartialEq, Clone)]
pub enum TemplateScopeTypesEnum {
    String,
    Number,
    Boolean,
}

impl TemplateScopeParameterImpl {
    pub fn new(scope: &str) -> Option<Self> {
        let mut split = scope[1..scope.len() - 1].split(':');
        let name = match split.next() {
            Some(name) => name,
            None => return None,
        };
        let scope_type_str = match split.next() {
            Some(scope_type) => scope_type,
            None => return None,
        };

        let scope_type = match str_to_scope_type(scope_type_str) {
            Some(scope_type) => scope_type,
            None => return None,
        };

        Some(Self {
            name: name.to_string(),
            scope_type,
        })
    }
}

impl TemplateScopeParameter for TemplateScopeParameterImpl {
    fn matches(&self, path_scope: &str) -> bool {
        match self.scope_type {
            TemplateScopeTypesEnum::String => path_scope.parse::<String>().is_ok(),
            TemplateScopeTypesEnum::Number => path_scope.parse::<i32>().is_ok(),
            TemplateScopeTypesEnum::Boolean => path_scope.parse::<bool>().is_ok(),
        }
    }
}

fn str_to_scope_type(scope_type: &str) -> Option<TemplateScopeTypesEnum> {
    let scope_lowercase = scope_type.to_lowercase();

    match scope_lowercase.as_str() {
        "number" => Some(TemplateScopeTypesEnum::Number),
        "string" => Some(TemplateScopeTypesEnum::String),
        "boolean" => Some(TemplateScopeTypesEnum::Boolean),
        _ => None,
    }
}
