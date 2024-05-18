pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240106_032056_add_options_col_to_endpoints;
mod m20240515_140830_adjust_endpoint_table_datatypes;
mod m20240518_053654_add_collection_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240106_032056_add_options_col_to_endpoints::Migration),
            Box::new(m20240515_140830_adjust_endpoint_table_datatypes::Migration),
            Box::new(m20240518_053654_add_collection_table::Migration),
        ]
    }
}
