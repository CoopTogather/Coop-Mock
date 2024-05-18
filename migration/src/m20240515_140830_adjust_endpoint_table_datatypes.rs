use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(EndPoints::Table)
                    .modify_column(ColumnDef::new(EndPoints::Name).string_len(32))
                    .modify_column(ColumnDef::new(EndPoints::Method).string_len(10))
                    .modify_column(ColumnDef::new(EndPoints::Description).string_len(255))
                    .modify_column(ColumnDef::new(EndPoints::CreatedAt).timestamp_with_time_zone())
                    .modify_column(ColumnDef::new(EndPoints::UpdatedAt).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(EndPoints::Table)
                    .modify_column(ColumnDef::new(EndPoints::Name).string())
                    .modify_column(ColumnDef::new(EndPoints::Method).string())
                    .modify_column(ColumnDef::new(EndPoints::Description).string())
                    .modify_column(ColumnDef::new(EndPoints::CreatedAt).date_time())
                    .modify_column(ColumnDef::new(EndPoints::UpdatedAt).date_time())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum EndPoints {
    Table,
    Name,
    Method,
    Description,
    CreatedAt,
    UpdatedAt,
}
