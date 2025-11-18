use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use sea_orm::Set;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "users")]
pub struct Model {
    /// 用户ID（主键）
    #[sea_orm(primary_key)]
    pub id: i64,

    /// 用户Token（唯一）
    #[sea_orm(unique)]
    pub token: String,

    /// 用户角色
    pub role: UserRole,

    /// Token过期时间（可选）
    pub expires_at: Option<DateTime<Utc>>,

    /// 上传文件总大小限制（字节）
    pub upload_quota: i64,

    /// 已使用上传大小（字节）
    #[sea_orm(default_value = 0)]
    pub used_quota: i64,

    /// 创建时间
    pub created_at: DateTime<Utc>,

    /// 最后活跃时间
    pub last_active: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::image::Entity")]
    Image,
    #[sea_orm(has_many = "super::cache::Entity")]
    Cache,
}

impl Related<super::image::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Image.def()
    }
}

impl Related<super::cache::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Cache.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

/// 用户角色
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, EnumIter, DeriveActiveEnum)]
#[sea_orm(rs_type = "String", db_type = "String(Some(10))")]
pub enum UserRole {
    /// 超级管理员
    #[sea_orm(string_value = "admin")]
    Admin,
    /// 普通用户
    #[sea_orm(string_value = "user")]
    User,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::User
    }
}

impl From<Model> for crate::models::UserInfo {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            token: model.token,
            role: model.role,
            expires_at: model.expires_at,
            upload_quota: model.upload_quota as u64,
            used_quota: model.used_quota as u64,
            created_at: model.created_at,
            last_active: model.last_active,
        }
    }
}

impl From<&crate::models::UserInfo> for ActiveModel {
    fn from(info: &crate::models::UserInfo) -> Self {
        Self {
            id: Set(info.id),
            token: Set(info.token.clone()),
            role: Set(info.role.clone()),
            expires_at: Set(info.expires_at),
            upload_quota: Set(info.upload_quota as i64),
            used_quota: Set(info.used_quota as i64),
            created_at: Set(info.created_at),
            last_active: Set(info.last_active),
        }
    }
}