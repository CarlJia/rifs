use std::sync::Arc;

use chrono::Utc;
use rand::{distributions::Alphanumeric, Rng};
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use sha2::{Digest, Sha256};
use tracing::info;

use crate::database::DatabasePool;
use crate::entities::api_token;
use crate::models::{ApiTokenInfo, CreateTokenPayload, CreateTokenResponse, TokenRole};
use crate::repositories::{ImageRepository, TokenRepository};
use crate::services::{CacheService, ImageService};
use crate::utils::AppError;

/// Token 业务逻辑
pub struct TokenService {
    repo: TokenRepository,
}

impl TokenService {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self {
            repo: TokenRepository::new(connection),
        }
    }

    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn generate_token() -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect()
    }

    pub async fn ensure_default_admin(&self) -> Result<Option<String>, AppError> {
        if self.repo.count_all().await? > 0 {
            return Ok(None);
        }

        let plaintext = Self::generate_token();
        let now = Utc::now();
        let active = api_token::ActiveModel {
            name: Set("超级管理员".to_string()),
            token_hash: Set(Self::hash_token(&plaintext)),
            role: Set(TokenRole::Admin.as_str().to_string()),
            max_upload_size: Set(None),
            used_upload_size: Set(0),
            expires_at: Set(None),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            last_used_at: Set(None),
            ..Default::default()
        };

        let _ = self.repo.insert(active).await?;
        info!("已创建默认超级管理员令牌");
        println!("🔐 默认管理员令牌: {}", plaintext);
        println!("⚠️ 请妥善保存该令牌，之后无法再次查询");

        Ok(Some(plaintext))
    }

    pub async fn verify_plain_token(&self, token: &str) -> Result<ApiTokenInfo, AppError> {
        if token.trim().is_empty() {
            return Err(AppError::BadRequest("令牌不能为空".to_string()));
        }

        let token_hash = Self::hash_token(token.trim());
        let model = self
            .repo
            .find_by_hash(&token_hash)
            .await?
            .ok_or_else(|| AppError::Unauthorized("认证失败，令牌不存在".to_string()))?;

        if !model.is_active {
            return Err(AppError::Unauthorized("该令牌已被禁用".to_string()));
        }

        if let Some(expired_at) = model.expires_at {
            if expired_at < Utc::now() {
                return Err(AppError::Unauthorized("该令牌已过期".to_string()));
            }
        }

        self.repo.update_last_used(model.id).await?;
        Ok(model.into())
    }

    pub async fn list_tokens(&self) -> Result<Vec<ApiTokenInfo>, AppError> {
        let models = self.repo.list_models().await?;
        Ok(models.into_iter().map(ApiTokenInfo::from).collect())
    }

    pub async fn create_token(
        &self,
        payload: CreateTokenPayload,
    ) -> Result<CreateTokenResponse, AppError> {
        if payload.name.trim().is_empty() {
            return Err(AppError::BadRequest("名称不能为空".to_string()));
        }

        let plaintext = Self::generate_token();
        let max_upload_size = payload
            .max_upload_size
            .map(|value| value as i64);
        let now = Utc::now();
        let active = api_token::ActiveModel {
            name: Set(payload.name.trim().to_string()),
            token_hash: Set(Self::hash_token(&plaintext)),
            role: Set(payload.role.as_str().to_string()),
            max_upload_size: Set(max_upload_size),
            used_upload_size: Set(0),
            expires_at: Set(payload.expires_at),
            is_active: Set(true),
            created_at: Set(now),
            updated_at: Set(now),
            last_used_at: Set(None),
            ..Default::default()
        };

        let model = self.repo.insert(active).await?;
        Ok(CreateTokenResponse {
            token: model.into(),
            plaintext,
        })
    }

    pub async fn delete_token_with_data(
        &self,
        pool: &DatabasePool,
        token_id: i32,
    ) -> Result<(), AppError> {
        let token = self
            .repo
            .find_by_id(token_id)
            .await?
            .ok_or_else(|| AppError::BadRequest("Token不存在".to_string()))?;

        if matches!(TokenRole::from(token.role.as_str()), TokenRole::Admin)
            && self.repo.count_admins().await? <= 1
        {
            return Err(AppError::BadRequest(
                "系统至少需要一个管理员令牌".to_string(),
            ));
        }

        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection.clone());
        let cache_service = CacheService::new(connection.clone())?;
        let images = {
            use crate::repositories::ImageRepositoryTrait;
            image_repo.find_by_owner(token_id).await?
        };
        let total = images.len();
        let mut cleaned_cache = 0u64;

        for image in images {
            // 删除图片文件
            ImageService::delete_image(pool, &image.hash).await?;
            // 清理相关缓存
            cleaned_cache += cache_service.remove_by_original_hash(&image.hash).await?;
        }

        self.repo.delete_by_id(token_id).await?;
        info!(
            "删除Token {} 完成，移除{}张图片，清理{}个缓存",
            token_id,
            total,
            cleaned_cache
        );
        Ok(())
    }

    pub async fn reserve_storage(&self, token_id: i32, bytes: i64) -> Result<(), AppError> {
        if bytes <= 0 {
            return Ok(());
        }
        self.repo.adjust_usage(token_id, bytes).await
    }

    pub async fn release_storage(&self, token_id: i32, bytes: i64) -> Result<(), AppError> {
        if bytes <= 0 {
            return Ok(());
        }
        self.repo.adjust_usage(token_id, -bytes).await
    }

    pub async fn get_token(&self, token_id: i32) -> Result<ApiTokenInfo, AppError> {
        self.repo
            .find_by_id(token_id)
            .await?
            .map(ApiTokenInfo::from)
            .ok_or_else(|| AppError::BadRequest("Token不存在".to_string()))
    }

    /// 根据token hash查找token信息
    pub async fn find_by_token_hash(&self, token: &str) -> Result<Option<ApiTokenInfo>, AppError> {
        use sha2::{Digest, Sha256};
        
        // 计算token的hash
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let token_hash = format!("{:x}", hasher.finalize());

        let token_model = self
            .repo
            .find_by_hash(&token_hash)
            .await?;
            
        Ok(token_model.map(ApiTokenInfo::from))
    }
}
