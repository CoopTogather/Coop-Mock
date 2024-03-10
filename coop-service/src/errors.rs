use sea_orm::error;

#[derive(thiserror::Error, Debug)]
pub enum CustomError {
    #[error("An error from serde_json, may cause by invalid json format. {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("An error from std::io")]
    IoError(#[from] std::io::Error),
    #[error("An error from sea_orm")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("An error from Service Layer: {0}")]
    ServiceError(String),
}
