use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(User::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(User::Token).string().not_null().unique_key())
                    .col(ColumnDef::new(User::Role).string().not_null().default("user"))
                    .col(ColumnDef::new(User::ExpiresAt).timestamp_with_time_zone())
                    .col(ColumnDef::new(User::UploadQuota).big_integer().not_null())
                    .col(ColumnDef::new(User::UsedQuota).big_integer().not_null().default(0))
                    .col(
                        ColumnDef::new(User::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(ColumnDef::new(User::LastActive).timestamp_with_time_zone())
                    .to_owned(),
            )
            .await?;

        // 创建默认管理员用户
        let admin_token = generate_admin_token();
        manager
            .exec_stmt(
                sea_orm::Statement::from_sql_and_values(
                    manager.get_database_backend(),
                    r#"
                    INSERT INTO users (token, role, upload_quota, created_at) 
                    VALUES ($1, 'admin', $2, $3)
                    "#,
                    vec![
                        admin_token.clone().into(),
                        (10i64.pow(9) * 1024 * 1024).into(), // 1TB
                        chrono::Utc::now().into(),
                    ],
                )
            )
            .await?;

        // 打印管理员token到日志
        println!("🔑 超级管理员Token已生成: {}", admin_token);
        println!("⚠️  请妥善保存此Token，用于系统管理");
        println!("📝 登录地址: http://localhost:3000/login");

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    Token,
    Role,
    ExpiresAt,
    UploadQuota,
    UsedQuota,
    CreatedAt,
    LastActive,
}

/// 生成管理员token
fn generate_admin_token() -> String {
    use rand::distributions::Alphanumeric;
    use rand::{thread_rng, Rng};

    let mut rng = thread_rng();
    (0..64)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}