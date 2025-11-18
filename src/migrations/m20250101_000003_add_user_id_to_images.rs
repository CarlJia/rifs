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
                    .table(Image::Table)
                    .add_column(
                        ColumnDef::new(Image::UserId)
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
                    .name("fk_images_user_id")
                    .from(Image::Table, Image::UserId)
                    .to(User::Table, User::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建索引以提高查询性能
        manager
            .create_index(
                Index::create()
                    .name("idx_images_user_id")
                    .table(Image::Table)
                    .col(Image::UserId)
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
                    .name("idx_images_user_id")
                    .to_owned(),
            )
            .await?;

        // 删除外键约束
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_images_user_id")
                    .to_owned(),
            )
            .await?;

        // 删除user_id列
        manager
            .alter_table(
                Table::alter()
                    .table(Image::Table)
                    .drop_column(Image::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Image {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
}