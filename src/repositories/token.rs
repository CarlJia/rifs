use async_trait::async_trait;
use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    TransactionTrait,
};
use std::sync::Arc;

use crate::entities::{api_token, ApiToken};
use crate::repositories::{BaseRepository, Repository};
use crate::utils::AppError;

/// Token 仓储
pub struct TokenRepository {
    base: BaseRepository,
}

impl TokenRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self {
            base: BaseRepository::new(connection),
        }
    }

    fn conn(&self) -> Arc<DatabaseConnection> {
        self.base.get_connection()
    }

    pub async fn list_models(&self) -> Result<Vec<api_token::Model>, AppError> {
        ApiToken::find()
            .order_by_desc(api_token::Column::CreatedAt)
            .all(&*self.conn())
            .await
            .map_err(|e| AppError::Internal(format!("查询Token失败: {}", e)))
    }

    pub async fn find_by_hash(
        &self,
        token_hash: &str,
    ) -> Result<Option<api_token::Model>, AppError> {
        ApiToken::find()
            .filter(api_token::Column::TokenHash.eq(token_hash))
            .one(&*self.conn())
            .await
            .map_err(|e| AppError::Internal(format!("查询Token失败: {}", e)))
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<api_token::Model>, AppError> {
        ApiToken::find_by_id(id)
            .one(&*self.conn())
            .await
            .map_err(|e| AppError::Internal(format!("查询Token失败: {}", e)))
    }

    pub async fn insert(
        &self,
        mut active_model: api_token::ActiveModel,
    ) -> Result<api_token::Model, AppError> {
        active_model
            .insert(&*self.conn())
            .await
            .map_err(|e| AppError::Internal(format!("创建Token失败: {}", e)))
    }

    pub async fn delete_by_id(&self, id: i32) -> Result<u64, AppError> {
        let result = ApiToken::delete_many()
            .filter(api_token::Column::Id.eq(id))
            .exec(&*self.conn())
            .await
            .map_err(|e| AppError::Internal(format!("删除Token失败: {}", e)))?;

        Ok(result.rows_affected)
    }

    pub async fn count_admins(&self) -> Result<i64, AppError> {
        ApiToken::find()
            .filter(api_token::Column::Role.eq("admin"))
            .count(&*self.conn())
            .await
            .map(|count| count as i64)
            .map_err(|e| AppError::Internal(format!("统计管理员数量失败: {}", e)))
    }

    pub async fn count_all(&self) -> Result<i64, AppError> {
        ApiToken::find()
            .count(&*self.conn())
            .await
            .map(|count| count as i64)
            .map_err(|e| AppError::Internal(format!("统计Token数量失败: {}", e)))
    }

    pub async fn update_last_used(&self, id: i32) -> Result<(), AppError> {
        if let Some(mut model) = self.find_by_id(id).await? {
            model.last_used_at = Some(Utc::now());
            model.updated_at = Utc::now();
            let mut active: api_token::ActiveModel = model.into();
            active
                .update(&*self.conn())
                .await
                .map_err(|e| AppError::Internal(format!("更新Token失败: {}", e)))?;
        }

        Ok(())
    }

    pub async fn adjust_usage(&self, token_id: i32, delta: i64) -> Result<(), AppError> {
        // ID为0的token是临时用户，不需要检查存储配额
        if token_id == 0 {
            return Ok(());
        }

        let connection = self.conn();
        let txn = connection
            .begin()
            .await
            .map_err(|e| AppError::Internal(format!("开启事务失败: {}", e)))?;

        let mut model = match ApiToken::find_by_id(token_id)
            .one(&txn)
            .await
            .map_err(|e| AppError::Internal(format!("查询Token失败: {}", e)))?
        {
            Some(model) => model,
            None => {
                txn.rollback()
                    .await
                    .ok();
                return Err(AppError::BadRequest("Token不存在".to_string()));
            }
        };

        if delta > 0 {
            if let Some(limit) = model.max_upload_size {
                if model.used_upload_size + delta > limit {
                    txn.rollback().await.ok();
                    return Err(AppError::BadRequest("已超过上传配额".to_string()));
                }
            }
        }

        model.used_upload_size += delta;
        if model.used_upload_size < 0 {
            model.used_upload_size = 0;
        }
        model.updated_at = Utc::now();

        let mut active: api_token::ActiveModel = model.into();
        active
            .update(&txn)
            .await
            .map_err(|e| AppError::Internal(format!("更新Token失败: {}", e)))?;

        txn.commit()
            .await
            .map_err(|e| AppError::Internal(format!("提交事务失败: {}", e)))
    }
}

#[async_trait::async_trait]
impl Repository for TokenRepository {
    fn get_connection(&self) -> Arc<DatabaseConnection> {
        self.base.get_connection()
    }

    async fn transaction<F, R>(&self, func: F) -> Result<R, AppError>
    where
        F: for<'c> FnOnce(
                &'c sea_orm::DatabaseTransaction,
            ) -> std::pin::Pin<
                Box<dyn std::future::Future<Output = Result<R, sea_orm::DbErr>> + Send + 'c>,
            > + Send,
        R: Send,
    {
        self.base.transaction(func).await
    }
}
