use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ApiTokens::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(ApiTokens::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(ApiTokens::Name).string().not_null())
                    .col(
                        ColumnDef::new(ApiTokens::TokenHash)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(ApiTokens::Role).string().not_null())
                    .col(ColumnDef::new(ApiTokens::MaxUploadSize).big_integer().null())
                    .col(
                        ColumnDef::new(ApiTokens::UsedUploadSize)
                            .big_integer()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(ApiTokens::ExpiresAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(ApiTokens::IsActive)
                            .boolean()
                            .not_null()
                            .default(true),
                    )
                    .col(
                        ColumnDef::new(ApiTokens::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ApiTokens::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(ApiTokens::LastUsedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(ApiTokens::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum ApiTokens {
    Table,
    Id,
    Name,
    TokenHash,
    Role,
    MaxUploadSize,
    UsedUploadSize,
    ExpiresAt,
    IsActive,
    CreatedAt,
    UpdatedAt,
    LastUsedAt,
}
