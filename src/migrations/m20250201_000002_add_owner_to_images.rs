use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Images::Table)
                    .add_column(ColumnDef::new(Images::OwnerTokenId).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_images_owner_token")
                    .from(Images::Table, Images::OwnerTokenId)
                    .to(ApiTokens::Table, ApiTokens::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .on_update(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_images_owner_token")
                    .table(Images::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Images::Table)
                    .drop_column(Images::OwnerTokenId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Images {
    Table,
    OwnerTokenId,
}

#[derive(DeriveIden)]
enum ApiTokens {
    Table,
    Id,
}
