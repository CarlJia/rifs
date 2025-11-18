use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 首先添加user_id列
        manager
            .alter_table(
                Table::alter()
                    .table(Cache::Table)
                    .add_column(
                        ColumnDef::new(Cache::UserId)
                            .integer()
                            .not_null()
                            .default_value(1), // 默认指向管理员用户
                    )
                    .to_owned(),
            )
            .await?;

        // 创建外键约束
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_cache_user_id")
                    .from(Cache::Table, Cache::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建索引以提高查询性能
        manager
            .create_index(
                Index::create()
                    .name("idx_cache_user_id")
                    .table(Cache::Table)
                    .col(Cache::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 删除索引
        manager
            .drop_index(
                Index::drop()
                    .name("idx_cache_user_id")
                    .to_owned(),
            )
            .await?;

        // 删除外键约束
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_cache_user_id")
                    .to_owned(),
            )
            .await?;

        // 删除user_id列
        manager
            .alter_table(
                Table::alter()
                    .table(Cache::Table)
                    .drop_column(Cache::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Cache {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}