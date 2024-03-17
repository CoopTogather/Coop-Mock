use poem::{Body, IntoResponse, Response};
use serde::{de::DeserializeOwned, Serialize};

pub struct GeneralApiResponse<T>
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    pub status: poem::http::StatusCode,
    pub message: String,
    pub data: Option<T>,
    pub success: bool,
}

impl<T> GeneralApiResponse<T>
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    pub fn new(
        status: poem::http::StatusCode,
        message: String,
        data: Option<T>,
        success: bool,
    ) -> Self {
        Self {
            status,
            message,
            data,
            success,
        }
    }
}

impl<T> IntoResponse for GeneralApiResponse<T>
where
    T: Send + Sync + Serialize + DeserializeOwned,
{
    fn into_response(self) -> Response {
        if self.data.is_some() {
            Response::builder()
                .status(self.status)
                .content_type("application/json")
                .body(Body::from_string(
                    serde_json::to_string(&self.data).unwrap(),
                ))
        } else {
            Response::builder().status(self.status).finish()
        }
    }
}
