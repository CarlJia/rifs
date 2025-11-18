use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use tracing::{debug, info, warn};

use crate::{
    entities::user,
    models::{CreateUserRequest, UserQuery, UserInfo, UserStats, UserRole},
    utils::AppError,
};

// 导入数据库用户角色类型以避免冲突
use user::UserRole as DbUserRole;

/// 用户服务
pub struct UserService {
    db: DatabaseConnection,
}

impl UserService {
    /// 创建新的用户服务实例
    pub fn new(db: DatabaseConnection) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    /// 根据token获取用户
    pub async fn get_user_by_token(&self, token: &str) -> Result<user::Model, AppError> {
        let user = user::Entity::find()
            .filter(user::Column::Token.eq(token))
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户不存在: {}", token)))?;

        // 更新最后活跃时间
        self.update_last_active(user.id).await?;

        Ok(user)
    }

    /// 根据ID获取用户
    pub async fn get_user_by_id(&self, id: i64) -> Result<user::Model, AppError> {
        user::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound(format!("用户不存在: {}", id)))
    }

    /// 创建用户
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserInfo, AppError> {
        // 检查token是否已存在
        if let Some(_) = user::Entity::find()
            .filter(user::Column::Token.eq(&request.token))
            .one(&self.db)
            .await?
        {
            return Err(AppError::Conflict("Token已存在".to_string()));
        }

        let now = Utc::now();
        let user_model = user::ActiveModel {
            token: Set(request.token),
            role: Set(request.role.unwrap_or_default()),
            expires_at: Set(request.expires_at),
            upload_quota: Set(request.upload_quota as i64),
            used_quota: Set(0),
            created_at: Set(now),
            last_active: Set(None),
            ..Default::default()
        };

        let user = user_model.insert(&self.db).await?;
        info!("创建用户成功: ID={}, Token={}", user.id, user.token);

        Ok(UserInfo::from(user))
    }

    /// 删除用户及其相关数据
    pub async fn delete_user(&self, id: i64) -> Result<(), AppError> {
        let user = self.get_user_by_id(id).await?;

        // 删除用户的所有图片
        use crate::entities::image;
        let images = image::Entity::find()
            .filter(image::Column::UserId.eq(id))
            .all(&self.db)
            .await?;

        for image in images {
            // 删除图片文件
            if let Err(e) = tokio::fs::remove_file(&image.stored_name()).await {
                warn!("删除图片文件失败: {} - {}", image.stored_name(), e);
            }

            // 删除数据库记录
            image::Entity::delete_by_id(image.hash).exec(&self.db).await?;
        }

        // 删除用户的所有缓存
        use crate::entities::cache;
        let caches = cache::Entity::find()
            .filter(cache::Column::UserId.eq(id))
            .all(&self.db)
            .await?;

        for cache in caches {
            // 删除缓存文件
            if let Err(e) = tokio::fs::remove_file(&cache.file_path).await {
                warn!("删除缓存文件失败: {} - {}", cache.file_path, e);
            }

            // 删除数据库记录
            cache::Entity::delete_by_id(cache.cache_key).exec(&self.db).await?;
        }

        // 删除用户
        user::Entity::delete_by_id(id).exec(&self.db).await?;

        info!("删除用户成功: ID={}, Token={}", user.id, user.token);
        Ok(())
    }

    /// 查询用户列表
    pub async fn list_users(&self, query: UserQuery) -> Result<(Vec<UserInfo>, u64), AppError> {
        let mut select = user::Entity::find();

        // 应用过滤条件
        if let Some(role) = query.role {
            select = select.filter(user::Column::Role.eq(role));
        }

        if let Some(search) = query.search {
            select = select.filter(user::Column::Token.like(&format!("%{}%", search)));
        }

        // 获取总数
        let total = select.clone().count(&self.db).await?;

        // 应用排序
        if let Some(order_by) = query.order_by {
            let order_dir = query.order_dir.unwrap_or_else(|| "asc".to_string());
            let is_desc = order_dir.to_lowercase() == "desc";

            match order_by.as_str() {
                "id" => {
                    select = if is_desc {
                        select.order_by_desc(user::Column::Id)
                    } else {
                        select.order_by_asc(user::Column::Id)
                    };
                }
                "created_at" => {
                    select = if is_desc {
                        select.order_by_desc(user::Column::CreatedAt)
                    } else {
                        select.order_by_asc(user::Column::CreatedAt)
                    };
                }
                "used_quota" => {
                    select = if is_desc {
                        select.order_by_desc(user::Column::UsedQuota)
                    } else {
                        select.order_by_asc(user::Column::UsedQuota)
                    };
                }
                "upload_quota" => {
                    select = if is_desc {
                        select.order_by_desc(user::Column::UploadQuota)
                    } else {
                        select.order_by_asc(user::Column::UploadQuota)
                    };
                }
                _ => {
                    select = select.order_by_asc(user::Column::Id);
                }
            }
        } else {
            select = select.order_by_asc(user::Column::Id);
        }

        // 应用分页
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);
        select = select.limit(limit).offset(offset);

        let users = select.all(&self.db).await?;
        let user_infos: Vec<UserInfo> = users.into_iter().map(UserInfo::from).collect();

        Ok((user_infos, total))
    }

    /// 更新用户配额使用量
    pub async fn update_used_quota(&self, id: i64, delta: i64) -> Result<(), AppError> {
        let user = self.get_user_by_id(id).await?;
        let new_used_quota = (user.used_quota + delta).max(0);

        user::ActiveModel {
            id: Set(id),
            used_quota: Set(new_used_quota),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        debug!("更新用户配额: ID={}, Used={}, Delta={}", id, new_used_quota, delta);
        Ok(())
    }

    /// 获取用户统计信息
    pub async fn get_user_stats(&self) -> Result<UserStats, AppError> {
        let total_count = user::Entity::find().count(&self.db).await?;
        
        let admin_count = user::Entity::find()
            .filter(user::Column::Role.eq(UserRole::Admin))
            .count(&self.db).await?;
        
        let user_count = user::Entity::find()
            .filter(user::Column::Role.eq(UserRole::User))
            .count(&self.db).await?;

        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        let active_count = user::Entity::find()
            .filter(user::Column::LastActive.gte(thirty_days_ago))
            .count(&self.db).await?;

        let quota_stats = user::Entity::find()
            .select_only([
                user::Column::UploadQuota,
                user::Column::UsedQuota,
            ])
            .into_tuple::<(i64, i64)>()
            .all(&self.db)
            .await?;

        let total_quota: u64 = quota_stats.iter().map(|(quota, _)| *quota as u64).sum();
        let used_quota: u64 = quota_stats.iter().map(|(_, used)| *used as u64).sum();
        let quota_usage_rate = if total_quota > 0 {
            used_quota as f64 / total_quota as f64
        } else {
            0.0
        };

        Ok(UserStats {
            total_count,
            admin_count,
            user_count,
            active_count,
            total_quota,
            used_quota,
            quota_usage_rate,
        })
    }

    /// 更新用户最后活跃时间
    async fn update_last_active(&self, id: i64) -> Result<(), AppError> {
        user::ActiveModel {
            id: Set(id),
            last_active: Set(Some(Utc::now())),
            ..Default::default()
        }
        .update(&self.db)
        .await?;

        Ok(())
    }

    /// 初始化管理员用户
    pub async fn init_admin_user(&self, token: &str) -> Result<UserInfo, AppError> {
        let now = Utc::now();
        let admin_quota = 1024i64.pow(3) * 1024; // 1TB

        let user_model = user::ActiveModel {
            token: Set(token.to_string()),
            role: Set(UserRole::Admin),
            expires_at: Set(None),
            upload_quota: Set(admin_quota),
            used_quota: Set(0),
            created_at: Set(now),
            last_active: Set(None),
            ..Default::default()
        };

        let user = user_model.insert(&self.db).await?;
        info!("初始化管理员用户成功: ID={}, Token={}", user.id, user.token);

        Ok(UserInfo::from(user))
    }
}