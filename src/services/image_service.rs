use chrono::Utc;
use std::io::ErrorKind;

use sha2::{Digest, Sha256};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::warn;

use crate::config::AppConfig;
use crate::database::DatabasePool;
use crate::models::{ApiTokenInfo, ImageInfo, ImageQuery, ImageStats, TokenRole};
use crate::repositories::{ImageRepository, ImageRepositoryTrait};
use crate::utils::{
    detect_file_type, ensure_image_dir, ensure_upload_dir, get_extension_from_mime, get_file_path,
    get_upload_dir, validate_file_size, AppError,
};
use super::{cache_service::CacheService, token_service::TokenService};

/// 图片服务结构体
pub struct ImageService;

impl ImageService {
    /// 计算文件哈希值
    fn calculate_file_hash(data: &[u8], owner_token_id: Option<i32>) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        if let Some(owner_id) = owner_token_id {
            hasher.update(owner_id.to_le_bytes());
        }
        format!("{:x}", hasher.finalize())
    }

    /// 保存上传的图片文件
    pub async fn save_image(
        pool: &DatabasePool,
        data: &[u8],
        original_filename: Option<String>,
        owner: &ApiTokenInfo,
    ) -> Result<ImageInfo, AppError> {
        // 验证文件是否为空
        if data.is_empty() {
            return Err(AppError::InvalidFile);
        }

        // 验证文件大小
        validate_file_size(data.len() as u64)?;

        // 基于文件内容检测真实的MIME类型（安全）
        let mime_type = detect_file_type(data)?;

        let owner_token_id = Some(owner.id);
        let file_hash = Self::calculate_file_hash(data, owner_token_id);

        // 检查是否已存在相同文件
        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection.clone());
        if let Some(existing_image) = image_repo.find_by_hash(&file_hash).await? {
            return Ok(existing_image);
        }

        // 确保基础上传目录存在
        ensure_upload_dir().await?;

        // 根据真实MIME类型生成文件扩展名
        let extension = get_extension_from_mime(&mime_type)?;

        // 创建图片信息
        let image_info = ImageInfo {
            hash: file_hash.clone(),
            size: data.len() as u64,
            mime_type,
            created_at: Utc::now(),
            last_accessed: None,
            extension,
            access_count: 0,
            original_filename,
            owner_token_id,
        };

        let relative_path = format!(
            "{}/{}/{}",
            &file_hash[0..2],
            &file_hash[2..4],
            image_info.stored_name()
        );

        let reserve_amount = data.len() as i64;
        let token_service = TokenService::new(connection.clone());
        token_service.reserve_storage(owner.id, reserve_amount).await?;

        let result = async {
            ensure_image_dir(std::path::Path::new(&relative_path)).await?;
            let file_path = get_file_path(std::path::Path::new(&relative_path));

            let mut file = File::create(&file_path).await?;
            file.write_all(data).await?;
            file.sync_all().await?;

            image_repo.insert(&image_info).await?;
            Ok::<_, AppError>(image_info)
        }
        .await;

        if result.is_err() {
            let _ = token_service.release_storage(owner.id, reserve_amount).await;
        }

        result
    }

    /// 根据哈希值获取图片信息
    pub async fn get_image_info(
        pool: &DatabasePool,
        identifier: &str,
    ) -> Result<Option<ImageInfo>, AppError> {
        // 验证标识符格式
        if identifier.is_empty() {
            return Err(AppError::BadRequest("标识符不能为空".to_string()));
        }

        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection);

        // 从数据库查询
        image_repo.find_by_hash(identifier).await
    }

    /// 读取图片文件内容
    pub async fn read_image_file(
        pool: &DatabasePool,
        identifier: &str,
    ) -> Result<Vec<u8>, AppError> {
        // 获取图片信息
        let image_info = Self::get_image_info(pool, identifier)
            .await?
            .ok_or(AppError::FileNotFound)?;

        // 更新访问信息
        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection);
        let _ = image_repo.update_access(identifier).await;

        // 构建文件路径
        let stored_name = image_info.stored_name();
        let relative_path = format!(
            "{}/{}/{}",
            &image_info.hash[0..2],
            &image_info.hash[2..4],
            stored_name
        );
        let file_path = get_upload_dir().join(&relative_path);

        let data = tokio::fs::read(&file_path).await?;
        Ok(data)
    }

    /// 删除图片文件
    pub async fn delete_image(pool: &DatabasePool, identifier: &str) -> Result<(), AppError> {
        // 获取图片信息
        let image_info = Self::get_image_info(pool, identifier)
            .await?
            .ok_or(AppError::FileNotFound)?;

        // 从数据库删除记录
        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection);
        image_repo.delete_by_hash(identifier).await?;

        // 删除文件
        let stored_name = image_info.stored_name();
        let relative_path = format!(
            "{}/{}/{}",
            &image_info.hash[0..2],
            &image_info.hash[2..4],
            stored_name
        );
        let file_path = get_upload_dir().join(&relative_path);

        tokio::fs::remove_file(&file_path).await?;

        Ok(())
    }

    /// 查询图片列表
    pub async fn query_images(
        pool: &DatabasePool,
        query: &ImageQuery,
    ) -> Result<(Vec<ImageInfo>, u64), AppError> {
        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection);
        let page_result = image_repo.find_by_query(query).await?;
        Ok((page_result.items, page_result.total))
    }

    /// 获取统计信息
    pub async fn get_stats(pool: &DatabasePool, owner_token_id: Option<i32>) -> Result<ImageStats, AppError> {
        let connection = pool.get_connection();
        let image_repo = ImageRepository::new(connection);
        image_repo.get_stats(owner_token_id).await
    }
}
