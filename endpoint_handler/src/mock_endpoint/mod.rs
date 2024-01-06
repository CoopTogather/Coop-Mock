pub mod options;

pub struct MockEndpoint {
    pub paths: Vec<String>,
    pub method: HttpMethod,
    pub options: options::MockOptions,
}

pub enum HttpMethod {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
}
