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
                    .add_column(ColumnDef::new(EndPoints::Options).json_binary().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(EndPoints::Table)
                    .drop_column(EndPoints::Options)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum EndPoints {
    Table,
    Options,
}
