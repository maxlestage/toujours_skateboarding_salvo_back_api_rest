use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Data::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Data::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Data::UserId).integer().not_null())
                    .col(ColumnDef::new(Data::Title).string().not_null())
                    .col(ColumnDef::new(Data::Description).string().not_null())
                    .col(ColumnDef::new(Data::Path).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Data::Table).to_owned())
            .await
    }
}

/// Learn more at https://docs.rs/sea-query#iden
#[derive(Iden)]
enum Data {
    Table,
    Id,
    UserId,
    Title,
    Description,
    Path,
}
