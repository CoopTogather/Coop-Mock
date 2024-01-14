use coop_service::domain::models::endpoints::EndpointDto;

use self::path::{TemplatePath, TemplatePathImpl};

pub mod options;
pub mod parameter;
pub mod path;

pub struct TemplateImpl {
    pub paths: Vec<TemplatePathImpl>,
    pub method: HttpMethod,
    pub options: options::MockOptions,
}

pub trait Template {
    fn matches(&self, path: &str, method: &str) -> bool;
}

#[derive(Debug, PartialEq)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

/// Template is a struct that represents a mock endpoint template.
/// It is used to match incoming requests to a mock endpoint.
/// # Example
/// ```
/// use endpoint_handler::endpoint_template::TemplateImpl;
///
/// let path = String::from("/mock/{id:number}");
///
/// let template_path = TemplateImpl::new(path, String::from("POST"), None);
///
/// assert_eq!(template_path.paths.len(), 2);
/// ```
impl TemplateImpl {
    pub fn new(path: String, method: String, options: Option<serde_json::Value>) -> Self {
        Self {
            paths: match_path(path.as_str()),
            method: match_method(method.as_str()),
            options: match options {
                Some(opt) => serde_json::from_value(opt).expect("Failed to parse mock options"),
                None => options::MockOptions::default(),
            },
        }
    }

    pub fn from(endpoint_setting: EndpointDto) -> Self {
        Self {
            paths: match_path(endpoint_setting.path.as_str()),
            method: match_method(endpoint_setting.method.as_str()),
            options: match endpoint_setting.options {
                Some(opt) => serde_json::from_value(opt).expect("Failed to parse mock options"),
                None => options::MockOptions::default(),
            },
        }
    }
}

impl Template for TemplateImpl {
    fn matches(&self, path: &str, method: &str) -> bool {
        let path_split = path.split('/').collect::<Vec<&str>>();

        if path_split.len() != self.paths.len() + 1 {
            return false;
        }

        if self.method != match_method(method) {
            return false;
        }

        for (i, path) in self.paths.iter().enumerate() {
            if !path.matches(path_split[i + 1]) {
                return false;
            }
        }

        true
    }
}

fn match_path(path: &str) -> Vec<TemplatePathImpl> {
    let mut path_split = path.split('/').collect::<Vec<&str>>();

    path_split.remove(0);

    path_split
        .into_iter()
        .map(|pt| TemplatePathImpl::new(pt))
        .collect()
}

fn match_method(method: &str) -> HttpMethod {
    match method {
        "GET" => HttpMethod::GET,
        "POST" => HttpMethod::POST,
        "PUT" => HttpMethod::PUT,
        "DELETE" => HttpMethod::DELETE,
        "PATCH" => HttpMethod::PATCH,
        _ => HttpMethod::GET,
    }
}
