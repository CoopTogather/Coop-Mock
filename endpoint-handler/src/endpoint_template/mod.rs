use coop_service::domain::models::endpoints::EndpointDto;
use poem::{IntoResponse, Response};

use self::path::{TemplatePath, TemplatePathImpl};

pub mod options;
pub mod parameter;
pub mod path;
pub mod response;

#[derive(Clone)]
pub struct TemplateImpl {
    pub paths: Vec<TemplatePathImpl>,
    pub method: HttpMethod,
    pub options: options::MockOptions,
}

/// A trait representing a template for an endpoint.
pub trait Template {
    /// Checks if the template matches the given path and method.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to match against.
    /// * `method` - The HTTP method to match against.
    ///
    /// # Returns
    ///
    /// Returns `true` if the template matches the path and method, `false` otherwise.
    fn matches(&self, path: &str, method: &str) -> bool;

    /// Converts the template into a response.
    ///
    /// # Returns
    ///
    /// Returns an implementation of the `IntoResponse` trait that represents the response.
    fn into_response(&self) -> Response;
}

#[derive(Debug, PartialEq, Clone)]
pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}

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

    fn into_response(&self) -> Response {
        let response = self.options.response.clone();

        match response {
            Some(res) => res.into_response(),
            None => poem::Response::builder()
                .status(poem::http::StatusCode::OK)
                .body("".to_string())
                .into_response(),
        }
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
