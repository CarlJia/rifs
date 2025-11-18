use chrono::Utc;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect, Set,
};
use tracing::{debug, info, warn};

use crate::{
    entities::image,
    entities::user::{UserRole as DbUserRole},
    models::{CreateUserRequest, UserQuery, UserInfo, UserStats},
    utils::AppError,
};

/// 用户服务
pub struct UserService {
    db: Arc<DatabaseConnection>,
}

impl UserService {
    /// 创建新的用户服务实例
    pub fn new(db: Arc<DatabaseConnection>) -> Result<Self, AppError> {
        Ok(Self { db })
    }

    /// 根据token获取用户
    pub async fn get_user_by_token(&self, token: &str) -> Result<user::Model, AppError> {
        debug!("根据token查询用户: {}", token);

        let user = user::Entity::find()
            .filter(user::Column::Token.eq(token))
            .one(&*self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询用户失败: {}", e)))?;

        // 更新最后活跃时间
        self.update_last_active(user.id).await?;

        Ok(user)
    }

    /// 根据ID获取用户
    pub async fn get_user_by_id(&self, id: i64) -> Result<user::Model, AppError> {
        user::Entity::find_by_id(id)
            .one(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询用户失败: {}", e)))
    }

    /// 创建用户
    pub async fn create_user(&self, request: CreateUserRequest) -> Result<UserInfo, AppError> {
        debug!("创建用户: {:?}", request);

        // 检查token是否已存在
        if let Some(_) = user::Entity::find()
            .filter(user::Column::Token.eq(&request.token))
            .one(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("检查token唯一性失败: {}", e)))?
        {
            return Err(AppError::Conflict("Token已存在".to_string()));
        }

        let now = Utc::now();
        let user_model = user::ActiveModel {
            token: Set(request.token),
            role: Set(request.role.unwrap_or_default().into()),
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
            .await
            .map_err(|e| AppError::Internal(format!("查询用户图片失败: {}", e)))?;

        for image in images {
            // 删除图片文件
            if let Err(e) = tokio::fs::remove_file(&image.stored_name()).await {
                warn!("删除图片文件失败: {} - {}", image.stored_name(), e);
            }

            // 删除数据库记录
            image::Entity::delete_by_id(image.id).exec(&self.db).await?;
        }

        // 删除用户的所有缓存
        use crate::entities::cache;
        let caches = cache::Entity::find()
            .filter(cache::Column::UserId.eq(id))
            .all(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询用户缓存失败: {}", e)))?;

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
            select = select.filter(user::Column::Role.eq(role.into()));
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

        // 分页
        let limit = query.limit.unwrap_or(50).min(100);
        let offset = query.offset.unwrap_or(0);

        let paginator = select.paginate(&self.db, limit);
        let total = paginator
            .num_items()
            .await
            .map_err(|e| AppError::Internal(format!("查询用户总数失败: {}", e)))?;

        let models = paginator
            .fetch_page(offset / limit)
            .await
            .map_err(|e| AppError::Internal(format!("查询用户列表失败: {}", e)))?;

        let user_infos: Vec<UserInfo> = models.into_iter().map(UserInfo::from).collect();
        let _page = offset / limit;

        Ok((user_infos, total))
    }

    /// 更新用户配额使用量
    pub async fn update_used_quota(&self, id: i64, delta: i64) -> Result<(), AppError> {
        let user = user::ActiveModel {
            id: Set(id),
            used_quota: Set(delta),
            ..Default::default()
        };

        user.update(&self.db).await?;
        Ok(())
    }

    /// 获取用户统计信息
    pub async fn get_user_stats(&self) -> Result<UserStats, AppError> {
        let connection = &self.db;
        let db_backend = connection.get_database_backend();

        // 基本统计
        let basic_stats = connection
            .query_one(Statement::from_string(
                db_backend,
                "SELECT COUNT(*) as total_count, COALESCE(SUM(upload_quota), 0) as total_quota, COALESCE(SUM(used_quota), 0) as total_used_quota FROM users".to_string(),
            ))
            .await
            .map_err(|e| AppError::Internal(format!("查询基本统计失败: {}", e)))?;

        let (total_count, total_quota, total_used_quota) = if let Some(row) = basic_stats {
            (
                row.try_get("", "total_count").unwrap_or(0u64) as i64,
                row.try_get("", "total_quota").unwrap_or(0u64) as i64,
                row.try_get("", "total_used_quota").unwrap_or(0u64) as i64,
            )
        } else {
            (0i64, 0i64, 0i64)
        };

        // 按角色统计
        let admin_count = user::Entity::find()
            .filter(user::Column::Role.eq(DbUserRole::Admin))
            .count(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询管理员数量失败: {}", e)))?;

        let user_count = user::Entity::find()
            .filter(user::Column::Role.eq(DbUserRole::User))
            .count(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询用户数量失败: {}", e)))?;

        let thirty_days_ago = Utc::now() - chrono::Duration::days(30);
        let active_count = user::Entity::find()
            .filter(user::Column::LastActive.gte(thirty_days_ago))
            .count(&self.db)
            .await
            .map_err(|e| AppError::Internal(format!("查询活跃用户数量失败: {}", e)))?;

        let quota_usage_rate = if total_quota > 0 {
            total_used_quota as f64 / total_quota as f64
        } else {
            0.0
        };

        Ok(UserStats {
            total_count,
            admin_count,
            user_count,
            active_count,
            total_quota: total_quota as u64,
            used_quota: total_used_quota as u64,
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
            role: Set(DbUserRole::Admin),
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