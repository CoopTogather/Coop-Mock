use std::ops::Deref;

use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .create_table(
                Table::create()
                    .table(Collection::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Collection::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Collection::Name)
                            .string()
                            .string_len(32)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Collection::Description)
                            .string()
                            .string_len(255)
                            .null(),
                    )
                    .to_owned(),
            )
            .await;

        manager
            .alter_table(
                Table::alter()
                    .table(EndPoints::Table)
                    .add_column(ColumnDef::new(EndPoints::CollectionId).integer().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _ = manager
            .drop_table(Table::drop().table(Collection::Table).to_owned())
            .await;

        manager
            .alter_table(
                Table::alter()
                    .table(EndPoints::Table)
                    .drop_column(EndPoints::CollectionId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum EndPoints {
    Table,
    Id,
    CollectionId,
}

#[derive(DeriveIden)]
enum Collection {
    Table,
    Id,
    Name,
    Description,
}
