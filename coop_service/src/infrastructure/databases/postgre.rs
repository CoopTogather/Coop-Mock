use diesel::r2d2::{ConnectionManager, Pool};

pub fn create_db_pool(connection_string: String) -> Pool<ConnectionManager<diesel::PgConnection>> {
    let manager = ConnectionManager::<diesel::PgConnection>::new(connection_string);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    pool
}
