# Tauri 前端快速开始指南

## 项目概述

RIFS 已成功迁移到 Tauri 框架，提供现代化、跨平台的桌面应用体验。前端使用 React 18 + TypeScript + shadcn/ui 组件库构建，提供简洁干净的用户界面。

## 安装和运行

### 1. 安装依赖

```bash
npm install
```

### 2. 开发模式

```bash
npm run tauri:dev
```

这将启动：
- Vite 开发服务器（http://localhost:5173）
- Tauri 开发窗口

### 3. 生产构建

```bash
npm run tauri:build
```

生成的应用位于 `src-tauri/target/release/`

## 项目结构

```
src/
├── App.tsx                     # 主应用组件，页面路由
├── main.tsx                    # React 入口点
├── components/
│   ├── Layout.tsx              # 侧边栏导航和主布局
│   └── ui/                     # shadcn/ui 组件库
│       ├── button.tsx          # 按钮组件
│       ├── card.tsx            # 卡片组件
│       ├── input.tsx           # 输入框
│       ├── label.tsx           # 标签
│       ├── dialog.tsx          # 对话框
│       ├── progress.tsx        # 进度条
│       └── dropdown-menu.tsx   # 下拉菜单
├── pages/
│   ├── Home.tsx                # 首页（图片上传）
│   ├── Gallery.tsx             # 图片库（浏览）
│   ├── CacheManagement.tsx     # 缓存管理
│   └── Login.tsx               # 登录页
├── services/
│   └── api.ts                  # 后端 API 调用
├── hooks/
│   └── useAuth.ts              # 认证状态管理
├── types/
│   └── index.ts                # TypeScript 类型定义
└── styles/
    └── globals.css             # 全局 Tailwind 样式
```

## 页面功能说明

### 首页（Home）
- **图片上传**：支持拖拽或点击选择
- **批量上传**：支持多个文件同时上传
- **进度显示**：实时显示上传进度百分比
- **多格式输出**：支持复制 URL、Markdown、HTML 三种格式
- **一键复制**：每种格式都有单独的复制按钮
- **自动复制**：可选项可在单个文件上传时自动复制链接

### 图片库（Gallery）
- **缩略图浏览**：展示已上传的所有图片
- **无限滚动**：自动加载更多图片
- **原图查看**：点击查看按钮可在新标签页打开原图
- **图片统计**：显示总图片数量

### 缓存管理（CacheManagement）
- **缓存统计**：显示总缓存大小和文件数量
- **智能清理**：按时间或大小进行缓存清理
- **详情查看**：表格显示最近的缓存文件
- **手动刷新**：随时更新统计信息

### 登录页（Login）
- **令牌认证**：输入授权令牌
- **自定义头**：支持配置自定义的认证头名称
- **安全存储**：令牌存储在浏览器本地

## 配置

### 修改 API 地址

编辑 `src/services/api.ts`：

```typescript
const API_BASE_URL = 'http://localhost:3000'  // 修改为实际地址
```

### 修改应用窗口配置

编辑 `tauri.conf.json`：

```json
"windows": [
  {
    "title": "RIFS - 高性能图床服务",
    "width": 1200,      // 窗口宽度
    "height": 800,      // 窗口高度
    "minWidth": 800,    // 最小宽度
    "minHeight": 600    // 最小高度
  }
]
```

## 样式定制

### 颜色主题

编辑 `src/styles/globals.css` 中的 CSS 变量：

```css
:root {
  --background: 0 0% 3%;        /* 背景色 */
  --foreground: 0 0% 98%;       /* 文本色 */
  --primary: 186 100% 50%;      /* 主色（青蓝） */
  --accent: 0 0% 20%;           /* 强调色 */
  /* ... 其他颜色 ... */
}
```

### Tailwind 定制

编辑 `tailwind.config.js` 来修改主题配置。

## 常见任务

### 添加新页面

1. 在 `src/pages/` 创建新组件 `NewPage.tsx`
2. 在 `src/App.tsx` 中导入并添加到路由
3. 在 `Layout.tsx` 中添加导航菜单项

### 添加 API 调用

1. 在 `src/services/api.ts` 添加函数
2. 在页面中导入使用
3. 确保处理错误和加载状态

### 添加新的 UI 组件

1. 从 shadcn/ui 或 Radix UI 复制组件到 `src/components/ui/`
2. 在页面中导入使用
3. 使用 Tailwind CSS 类进行样式定制

## 构建输出

### Web 输出
```bash
npm run build
```
输出到 `dist/` 目录，包含：
- `index.html` - HTML 模板
- `assets/` - CSS 和 JavaScript 文件

### Tauri 应用输出
```bash
npm run tauri:build
```
输出到 `src-tauri/target/release/`，包含：
- **Windows**: `RIFS.exe` + 依赖文件
- **macOS**: `RIFS.app`
- **Linux**: AppImage 或 DEB 包

## 调试

### 在浏览器中调试

```bash
npm run dev
```

打开 `http://localhost:5173` 即可使用浏览器开发者工具。

### 查看控制台日志

```javascript
console.log('Debug message')
```

在浏览器开发者工具的 Console 标签查看。

## 性能优化

- **代码分割**：Vite 自动处理
- **懒加载**：图片使用 `loading="lazy"`
- **无限滚动**：使用 Intersection Observer API
- **防抖**：滚动事件处理使用防抖
- **缓存**：HTTP 缓存和浏览器缓存

## 依赖管理

### 主要依赖

- `react` - UI 框架
- `typescript` - 类型系统
- `vite` - 构建工具
- `tailwindcss` - 样式框架
- `@radix-ui/*` - UI 组件基础库
- `lucide-react` - 图标库
- `@tauri-apps/*` - Tauri API

### 更新依赖

```bash
npm update              # 更新所有依赖
npm install <package>  # 安装特定包
npm audit fix          # 修复安全漏洞
```

## 故障排除

### Q: 构建失败
A: 检查 Node.js 版本（需要 20+）和 npm 版本（需要 11+）

### Q: API 连接失败
A: 检查 `API_BASE_URL` 配置和后端服务是否运行

### Q: 样式不正确
A: 确保 Tailwind CSS 类名正确，运行 `npm run build` 重建

### Q: 无法启动开发服务器
A: 尝试删除 `node_modules` 和 `package-lock.json`，然后重新安装

## 最佳实践

1. **命名规范**
   - 组件：PascalCase（如 `HomePage.tsx`）
   - 函数：camelCase（如 `handleSubmit`）
   - 常量：UPPER_SNAKE_CASE（如 `API_BASE_URL`）

2. **代码组织**
   - 将相关的逻辑分组到自定义 hooks
   - 使用 TypeScript 类型定义所有 API 响应
   - 将魔法数字提取到常量

3. **性能**
   - 使用 `React.memo` 防止不必要的重新渲染
   - 使用 `useCallback` 优化事件处理器
   - 对大列表使用虚拟滚动

4. **可维护性**
   - 添加适当的代码注释
   - 遵循现有的代码风格
   - 定期运行 `npm run build` 检查错误

## 进一步阅读

- [FRONTEND_README.md](./FRONTEND_README.md) - 前端详细文档
- [TAURI_MIGRATION.md](./TAURI_MIGRATION.md) - 迁移指南和参考
- [Tauri 官方文档](https://tauri.app/v1/guides/)
- [React 官方文档](https://react.dev/)
- [Tailwind CSS 文档](https://tailwindcss.com/)

## 获取帮助

遇到问题时：
1. 检查控制台错误信息
2. 查看相关的源代码和类型定义
3. 参考上述文档
4. 查看项目提交历史了解之前的修复

---

祝开发愉快！🚀
