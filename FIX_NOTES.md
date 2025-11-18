# 修复说明

## 修复的问题

### 1. 登录拦截问题

**问题**：Tauri 首页没有进行登录拦截，即使启用了认证，未登录用户仍可访问。

**原因**：`useAuth.ts` 中的认证逻辑不清晰。当认证不是必需的时，即使没有令牌也会设置 `isAuthenticated` 为 `true`。

**解决方案**：
- 改进了 `useAuth.ts` 中的认证逻辑
- 现在当 `authRequired` 为 `true` 时，必须有有效的令牌才能设置 `isAuthenticated` 为 `true`
- 如果认证是必需的但没有令牌，应用会强制跳转到登录页

**改动文件**：`src/hooks/useAuth.ts`

### 2. 图片加载问题

**问题**：图片库加载不出来图片，图片 URL 生成使用硬编码的 `http://localhost:3000`。

**原因**：
- 图片 URL 在多个页面中硬编码了后端地址
- 在 Tauri 环境中，用户可能需要连接到不同的后端服务器
- 没有提供灵活的 API 地址配置

**解决方案**：
- 创建了动态 `getApiBaseUrl()` 函数，支持多种配置方式：
  1. 本地存储配置（最高优先级，用户可在设置页面修改）
  2. 环境变量 `VITE_API_BASE_URL`
  3. 默认值 `http://localhost:3000`
- 更新所有 API 调用使用动态地址
- 在 Gallery 和 Home 页面中使用动态地址生成图片 URL
- 创建了专用的设置页面允许用户配置 API 地址

**改动文件**：
- `src/services/api.ts` - 添加 `getApiBaseUrl()` 和 `setApiBaseUrl()` 函数
- `src/pages/Gallery.tsx` - 使用动态 API 地址
- `src/pages/Home.tsx` - 使用动态 API 地址
- `src/pages/Settings.tsx` - 新建设置页面
- `src/App.tsx` - 添加设置页面路由
- `src/components/Layout.tsx` - 添加设置菜单项
- `tsconfig.json` - 添加 Vite 类型支持

## 新增功能

### 系统设置页面 (`src/pages/Settings.tsx`)

新增的设置页面提供以下功能：
- **API 服务器地址配置**：允许用户修改后端服务器地址
- **URL 验证**：验证输入的 URL 格式是否正确
- **本地存储**：配置保存到浏览器本地存储
- **重置默认值**：一键重置为默认地址
- **配置信息展示**：显示当前 API 地址和应用版本

### 环境变量支持

项目现在支持通过环境变量配置 API 地址：
```bash
VITE_API_BASE_URL=http://your-backend-server:3000
```

创建 `.env` 文件或在构建时设置此环境变量。

参考 `.env.example` 文件获取更多信息。

## 配置方法

### 方法 1：使用设置页面（推荐用于 Tauri 应用）

1. 打开应用
2. 点击侧边栏的"系统设置"
3. 输入后端服务器地址（如 `http://192.168.1.100:3000`）
4. 点击"保存设置"
5. 刷新页面使配置生效

### 方法 2：使用环境变量

创建 `.env` 文件：
```
VITE_API_BASE_URL=http://your-server:3000
```

然后构建应用：
```bash
npm run build
```

### 方法 3：本地存储（开发者模式）

在浏览器开发者工具的 Console 中运行：
```javascript
localStorage.setItem('api_base_url', 'http://your-server:3000')
window.location.reload()
```

## 测试

构建后验证：
```bash
npm run build
```

应该看到成功的构建输出，无 TypeScript 错误。

## 文件修改总结

| 文件 | 修改内容 |
|------|---------|
| `src/hooks/useAuth.ts` | 改进认证逻辑 |
| `src/services/api.ts` | 添加动态 API 地址支持 |
| `src/pages/Gallery.tsx` | 使用动态 API 地址 |
| `src/pages/Home.tsx` | 使用动态 API 地址 |
| `src/pages/Settings.tsx` | 新建设置页面 |
| `src/App.tsx` | 添加设置页面路由 |
| `src/components/Layout.tsx` | 添加设置导航菜单 |
| `tsconfig.json` | 添加 Vite 类型支持 |
| `.env.example` | 新建环境变量示例 |

## 使用指南

### 首次启动

1. 运行 `npm install` 安装依赖
2. 如果后端不在 `http://localhost:3000`，先进入设置页面配置 API 地址
3. 如果后端启用了认证，在登录页面输入令牌

### 修改 API 地址

1. 打开应用
2. 点击"系统设置"
3. 修改 API 地址
4. 保存并刷新

### 常见问题

**Q: 为什么图片还是不显示？**
A: 
- 检查 API 地址是否正确（访问 `http://api-address/health` 验证）
- 确认后端服务已启动
- 检查浏览器网络标签页查看请求状态

**Q: 如何判断认证是否启用？**
A:
- 刷新页面，如果自动跳转到登录页，说明认证已启用
- 或进入设置页面查看日志信息

**Q: 配置修改后不生效？**
A:
- 确保点击了"保存设置"按钮
- 刷新页面后重试
- 清除浏览器本地存储后重新配置

## 向后兼容性

所有修改都是向后兼容的：
- 默认 API 地址仍然是 `http://localhost:3000`
- 现有的 localStorage 令牌继续有效
- 没有强制要求配置 API 地址

## 部署建议

### Web 部署

如果部署为 Web 应用：
```bash
# 设置环境变量
export VITE_API_BASE_URL=https://your-api-server
npm run build

# 部署 dist 目录
```

### Tauri 应用部署

Tauri 应用用户可以：
1. 使用默认配置 `http://localhost:3000`
2. 或在设置页面手动配置 API 地址
3. 配置会持久化到本地存储
