# 用户管理功能实现文档

## 概述
完善了用户管理功能，包括前端用户管理页面、后端API接口、以及应用启动时的默认管理员初始化。

## 功能特性

### 1. 应用启动时自动初始化管理员账户
- **位置**: `src/app_state.rs`
- **行为**: 
  - 应用启动时检查数据库中是否存在Token
  - 如果数据库为空（首次启动），自动创建一个默认超级管理员Token
  - 生成随机的Token密钥并打印到控制台（可供用户复制保存）
  - 包含以下日志输出：
    - `🔐 默认管理员令牌: <随机生成的token>`
    - `⚠️ 请妥善保存该令牌，之后无法再次查询`
- **代码**: 调用 `TokenService::ensure_default_admin()` 方法

### 2. 后端 Token 管理 API
- **位置**: `src/handlers/token_handler.rs`
- **新增端点**:
  - `GET /api/tokens/list` - 获取所有Token列表
  - `POST /api/tokens/create` - 创建新Token
  - `GET /api/tokens/{id}` - 获取单个Token信息
  - `DELETE /api/tokens/{id}` - 删除Token并清理相关数据
- **认证**: 所有端点都需要通过 `AuthGuard` 认证
- **功能**:
  - 支持创建不同角色的Token（admin/user）
  - 支持设置Token的过期时间
  - 支持设置Token的上传配额
  - 删除Token时自动删除该Token拥有的所有图片和缓存

### 3. 前端用户管理页面
- **位置**: `src/pages/UserManagement.tsx`
- **功能**:
  - 显示所有Token列表
  - 创建新Token的对话框
  - 新Token生成后显示一次性密钥（支持复制和显示/隐藏）
  - 删除Token的确认对话框
  - 实时显示Token的以下信息：
    - Token名称
    - 角色（管理员/普通用户）
    - 是否活跃
    - 创建时间
    - 最后使用时间
    - 上传配额使用情况
  - 响应式设计，支持移动端

### 4. 前端路由和导航集成
- **位置**: 
  - `src/App.tsx` - 添加用户管理页面路由
  - `src/components/Layout.tsx` - 添加侧边栏菜单项
- **菜单项**: 在侧边栏中添加"用户管理"菜单，点击可切换到用户管理页面

### 5. API 客户端增强
- **位置**: `src/services/api.ts`
- **新增**: 通用 API 客户端对象 `api`，提供以下方法：
  - `api.get(endpoint)` - 发送GET请求
  - `api.post(endpoint, data)` - 发送POST请求
  - `api.delete(endpoint)` - 发送DELETE请求
  - 所有请求自动包含认证头和正确的Content-Type

### 6. 路由配置更新
- **位置**: `src/routes/mod.rs`
- **新增路由**:
  - `/user-management` - 用户管理页面
  - `/api/tokens/list` - Token列表
  - `/api/tokens/create` - 创建Token
  - `/api/tokens/{id}` - Token详情和删除

## 文件变更总结

### 新增文件
- `src/pages/UserManagement.tsx` - 前端用户管理页面
- `src/handlers/token_handler.rs` - Token管理处理器
- `USER_MANAGEMENT_FEATURE.md` - 本文档

### 修改文件
- `src/app_state.rs` - 添加默认管理员初始化逻辑
- `src/handlers/mod.rs` - 导出新的Token处理器
- `src/routes/mod.rs` - 添加Token管理路由
- `src/App.tsx` - 添加用户管理页面支持
- `src/components/Layout.tsx` - 添加用户管理菜单项
- `src/services/api.ts` - 添加通用API客户端

## 使用说明

### 首次启动
1. 运行应用
2. 应用会检查数据库中是否有Token
3. 如果没有Token，会自动创建管理员账户并输出：
   ```
   🔐 默认管理员令牌: <随机token>
   ⚠️ 请妥善保存该令牌，之后无法再次查询
   ```
4. 用户需要保存这个token用于后续登录和管理

### 访问用户管理页面
1. 登录应用（使用管理员token或其他token）
2. 在侧边栏菜单中点击"用户管理"
3. 在用户管理页面可以：
   - 查看所有Token列表
   - 创建新的Token
   - 删除Token

### 创建新Token
1. 点击"创建Token"按钮
2. 在对话框中填写：
   - Token名称（必填）
   - 选择角色（admin或user）
3. 点击"创建"
4. 新Token会显示一次，用户需要复制保存

## 代码精简/优化
- 精简了不必要的注释
- 优化了API路由的组织
- 使用通用API客户端避免代码重复
- 合理使用TypeScript类型约束

## 注意事项
- 管理员Token在首次启动时只会生成一次，之后无法再获取
- 删除Token会同时删除该Token拥有的所有图片和缓存
- Token API的所有端点都需要认证
- Token创建后显示的明文只能查看一次，之后系统只保存hash值
