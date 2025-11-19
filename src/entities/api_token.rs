use chrono::{DateTime, Utc};
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

use crate::models::{ApiTokenInfo, TokenRole};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "api_tokens")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub token_hash: String,
    pub role: String,
    pub max_upload_size: Option<i64>,
    pub used_upload_size: i64,
    pub expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for ApiTokenInfo {
    fn from(model: Model) -> Self {
        Self {
            id: model.id,
            name: model.name,
            role: TokenRole::from(model.role.as_str()),
            max_upload_size: model.max_upload_size,
            used_upload_size: model.used_upload_size,
            expires_at: model.expires_at,
            is_active: model.is_active,
            created_at: model.created_at,
            updated_at: model.updated_at,
            last_used_at: model.last_used_at,
        }
    }
}
