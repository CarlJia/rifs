# 用户角色限制功能测试指南

## 功能概述

本次实现为图床服务添加了基于用户角色的访问控制，普通用户只能访问首页和图片库两个菜单，管理员可以访问所有功能。

## 实现细节

### 后端改动

1. **新增 AdminGuard 提取器** (`src/middleware/auth.rs`)
   - 用于保护只允许管理员访问的接口
   - 检查请求中的 token 是否为配置文件中的管理员 token

2. **错误处理增强** (`src/utils/error.rs`)
   - 添加 `Forbidden(403)` 错误类型用于权限拒绝

3. **新增用户信息接口** (`src/handlers/auth_handler.rs`)
   - `GET /api/auth/user-info` - 返回当前用户的信息（包括 role 字段）

4. **受保护接口列表**
   - Token 管理接口（仅 Admin）：
     - `GET /api/tokens/list`
     - `POST /api/tokens/create`
     - `DELETE /api/tokens/{id}`
     - `GET /api/tokens/{id}`
   
   - 缓存管理接口（仅 Admin）：
     - `POST /api/cache/cleanup/auto`
     - `POST /api/cache/decay`
     - `DELETE /api/cache/clear`

### 前端改动

1. **useAuth Hook 增强** (`src-tauri/hooks/useAuth.ts`)
   - 新增 `userRole` 状态（'admin' | 'user'）
   - 新增 `userInfo` 状态
   - 登录和初始化时自动获取用户角色

2. **API 服务扩展** (`src-tauri/services/api.ts`)
   - 新增 `getUserInfo()` 函数

3. **菜单动态过滤** (`src-tauri/components/Layout.tsx`)
   - 根据 `userRole` 动态过滤菜单项
   - 普通用户只看到："首页"、"图片库"
   - 管理员看到全部菜单

4. **路由保护** (`src-tauri/App.tsx`)
   - 普通用户尝试访问受限页面时自动重定向到首页

## 测试场景

### 场景 1：管理员用户
1. 使用管理员 token 登录
2. 验证可见所有菜单项：首页、图片库、缓存管理、用户管理、系统设置
3. 验证所有接口都可访问

### 场景 2：普通用户
1. 使用普通用户 token 登录
2. 验证只看到菜单：首页、图片库
3. 验证缓存管理、用户管理、系统设置菜单不可见
4. 手动访问受限页面时自动重定向到首页
5. 直接调用受限 API 时返回 403 Forbidden

### 场景 3：无认证要求
1. 禁用认证（auth.enabled = false）
2. 验证所有用户都默认为 admin 角色
3. 所有菜单都可见

## API 响应示例

### 获取用户信息 - 管理员
```
GET /api/auth/user-info
Authorization: Bearer <admin_token>

Response:
{
  "success": true,
  "data": {
    "name": "管理员",
    "role": "admin"
  }
}
```

### 获取用户信息 - 普通用户
```
GET /api/auth/user-info
Authorization: Bearer <user_token>

Response:
{
  "success": true,
  "data": {
    "name": "user_token_name",
    "role": "user"
  }
}
```

### 普通用户访问受限接口
```
GET /api/tokens/list
Authorization: Bearer <user_token>

Response (403):
{
  "success": false,
  "message": "只有管理员才能访问此资源",
  "code": 403
}
```

## 配置说明

系统会自动检测 token 的角色：
- 配置文件中的 token（auth.token）- 管理员角色
- 数据库中创建的 token - 根据创建时指定的 role 字段

## 部署注意事项

1. 确保认证已启用（auth.enabled = true）
2. 配置管理员 token（auth.token）
3. 前端会自动从 /api/auth/user-info 获取用户角色
4. 菜单会根据角色自动隐藏
