use sea_orm::{Database, DatabaseConnection};

pub async fn create_db_pool(connection_string: String) -> DatabaseConnection {
    Database::connect(&connection_string)
        .await
        .expect("Failed to connect to database")
}
