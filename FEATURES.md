# RIFS 功能特性

## 核心功能

### 🖼️ 图片管理
- **上传图片**：支持拖拽上传、批量上传
- **图片浏览**：网格视图、详情查看
- **图片转换**：自动格式转换、尺寸调整
- **缓存管理**：智能缓存策略、自动清理

### 👥 用户管理
- **角色权限**：管理员和普通用户两种角色
- **访问控制**：基于角色的页面和API访问限制
- **资源隔离**：用户只能访问自己的图片
- **Token管理**：安全的API令牌认证

### 🔧 系统配置
- **动态API地址**：支持运行时配置后端地址
- **环境变量**：支持通过环境变量配置
- **本地存储**：用户配置持久化保存

## 技术特性

### 🚀 高性能
- **Rust后端**：内存安全、高性能
- **异步处理**：基于Tokio的异步架构
- **图片优化**：自动格式优化和压缩
- **智能缓存**：多级缓存策略

### 🔒 安全性
- **Token认证**：安全的API令牌机制
- **权限控制**：细粒度的角色权限管理
- **数据隔离**：用户数据完全隔离
- **输入验证**：严格的参数验证

### 🌐 跨平台
- **Tauri应用**：支持Windows、macOS、Linux
- **Web应用**：支持现代浏览器
- **响应式设计**：适配各种屏幕尺寸
- **PWA支持**：渐进式Web应用

## 用户角色

### 管理员 (Admin)
- ✅ 访问所有功能
- ✅ 用户管理
- ✅ 缓存管理
- ✅ 系统设置
- ✅ 查看所有图片

### 普通用户 (User)
- ✅ 上传图片
- ✅ 查看自己的图片
- ✅ 基本图片操作
- ❌ 用户管理
- ❌ 缓存管理
- ❌ 系统设置

## API接口

### 认证相关
- `POST /api/auth/verify` - 验证令牌
- `GET /api/tokens/list` - 列出令牌（管理员）
- `POST /api/tokens/create` - 创建令牌（管理员）
- `DELETE /api/tokens/{id}` - 删除令牌（管理员）

### 图片相关
- `POST /api/upload` - 上传图片
- `GET /api/images/query` - 查询图片
- `GET /api/images/stats` - 获取统计
- `DELETE /api/images/{id}` - 删除图片

### 缓存相关
- `GET /api/cache/stats` - 缓存统计
- `POST /api/cache/clear` - 清理缓存
- `POST /api/cache/cleanup` - 自动清理

## 配置选项

### 环境变量
```bash
VITE_API_BASE_URL=http://your-server:3000  # API服务器地址
```

### 配置文件
- `config.toml` - 主配置文件
- `config_test.toml` - 测试环境配置

### 主要配置项
- `[server]` - 服务器设置
- `[database]` - 数据库配置
- `[storage]` - 存储设置
- `[cache]` - 缓存策略
- `[logging]` - 日志配置

## 部署方式

### Docker部署
```bash
docker-compose up -d
```

### 直接运行
```bash
cargo run --release
```

### Tauri应用
```bash
npm run tauri:dev  # 开发模式
npm run tauri build # 构建应用
```

## 开发信息

### 技术栈
- **后端**: Rust + Axum + SeaORM
- **前端**: React + TypeScript + Tauri
- **数据库**: SQLite/PostgreSQL/MySQL
- **样式**: Tailwind CSS + shadcn/ui

### 项目结构
```
src/                    # Rust后端代码
├── handlers/          # API处理器
├── services/          # 业务逻辑
├── repositories/      # 数据访问层
├── middleware/        # 中间件
├── models/           # 数据模型
└── entities/         # 数据库实体

src-tauri/            # 前端代码
├── components/       # React组件
├── pages/           # 页面组件
├── services/        # API服务
├── hooks/           # 自定义Hook
└── types/           # TypeScript类型
```

## 更新日志

详细的更新记录请参考 [PROJECT_CHANGES.md](./PROJECT_CHANGES.md)