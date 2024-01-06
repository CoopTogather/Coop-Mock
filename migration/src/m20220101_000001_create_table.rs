use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(EndPoints::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(EndPoints::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(EndPoints::Name).string().not_null())
                    .col(ColumnDef::new(EndPoints::Path).string().not_null())
                    .col(ColumnDef::new(EndPoints::Method).string().not_null())
                    .col(ColumnDef::new(EndPoints::Description).string().null())
                    .col(ColumnDef::new(EndPoints::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(EndPoints::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(EndPoints::Enabled).boolean().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(EndPoints::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum EndPoints {
    Table,
    Id,
    Name,
    Path,
    Method,
    Description,
    CreatedAt,
    UpdatedAt,
    Enabled,
}
