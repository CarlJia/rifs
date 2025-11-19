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

        // SQLite 不支持对现有表添加外键约束
        // 在生产环境中，应该重建表来添加外键约束
        // 这里我们只添加列，跳过外键约束的创建
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 由于我们没有创建外键约束，所以也不需要删除它
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
