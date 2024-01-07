pub struct TemplatePathImpl {
    pub scope: String,
    pub is_parameter: bool,
    pub parameter_type: Option<String>,
    pub parameter_name: Option<String>,
}

pub trait TemplatePath {
    fn matches(&self, path_scope: &str) -> bool;
}

impl TemplatePathImpl {
    pub fn new(scope: &str) -> Self {
        let mut scope = scope.to_string();
        let mut is_parameter = false;
        let mut parameter_name = None;
        let mut parameter_type = None;

        if scope.starts_with('{') && scope.ends_with('}') {
            is_parameter = true;
            let mut split = scope[1..scope.len() - 1].split(':');
            parameter_name = split.next().map(|s| s.to_string());
            parameter_type = split.next().map(|s| s.to_string());
        } else {
            scope = scope.to_string();
        }

        Self {
            scope,
            is_parameter,
            parameter_name,
            parameter_type,
        }
    }
}

impl TemplatePath for TemplatePathImpl {
    fn matches(&self, path_scope: &str) -> bool {
        if self.is_parameter {
            match self.parameter_type.as_ref().unwrap().as_str() {
                "i32" => path_scope.parse::<i32>().is_ok(),
                "i64" => path_scope.parse::<i64>().is_ok(),
                "f32" => path_scope.parse::<f32>().is_ok(),
                "f64" => path_scope.parse::<f64>().is_ok(),
                "u32" => path_scope.parse::<u32>().is_ok(),
                "u64" => path_scope.parse::<u64>().is_ok(),
                _ => false,
            }
        } else {
            self.scope == path_scope
        }
    }
}
