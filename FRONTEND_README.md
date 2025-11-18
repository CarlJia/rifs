# RIFS Tauri 前端

这是 RIFS（高性能 Rust 图床服务）的 Tauri 跨平台桌面应用前端。

## 特性

- **现代化 UI**：使用 shadcn/ui 组件库构建
- **简洁风格**：深色主题，专注于功能
- **跨平台**：支持 Windows、macOS、Linux
- **响应式设计**：适配各种屏幕尺寸
- **实时交互**：图片上传、浏览、缓存管理

## 技术栈

- **React 18** - UI 框架
- **TypeScript** - 类型安全
- **Tauri** - 跨平台桌面应用框架
- **Vite** - 构建工具
- **Tailwind CSS** - 样式框架
- **shadcn/ui** - 组件库
- **Lucide React** - 图标库

## 开发

### 前置要求

- Node.js 20+
- npm 11+

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri:dev
```

### 构建应用

```bash
npm run tauri:build
```

## 项目结构

```
src/
├── components/      # React 组件
│   ├── ui/         # shadcn/ui 组件
│   └── Layout.tsx  # 主布局组件
├── pages/          # 页面组件
│   ├── Home.tsx    # 首页（上传）
│   ├── Gallery.tsx # 图片库
│   ├── CacheManagement.tsx # 缓存管理
│   └── Login.tsx   # 登录页
├── services/       # API 服务
├── hooks/          # 自定义 Hooks
├── types/          # TypeScript 类型
└── styles/         # 全局样式
```

## 页面功能

### 首页（上传）
- 拖拽或点击选择图片上传
- 支持多个文件批量上传
- 显示上传进度
- 支持复制 URL、Markdown、HTML 格式
- 支持自动复制链接选项

### 图片库
- 浏览所有上传的图片
- 缩略图预览
- 无限滚动加载
- 查看原图功能

### 缓存管理
- 查看缓存统计信息
- 基于条件清理缓存
- 查看详细的缓存文件列表

### 登录
- 当服务器启用认证时显示
- 支持自定义授权头名称
- 安全的令牌存储

## 样式主题

使用深色主题，配色方案：
- 背景色：深蓝-黑色
- 主色：青蓝色 (Cyan-Blue)
- 强调色：明亮的蓝色
- 文本色：浅色

## 配置

### API 基址

在 `src/services/api.ts` 中修改 `API_BASE_URL`：

```typescript
const API_BASE_URL = 'http://localhost:3000'
```

### Tauri 配置

编辑 `tauri.conf.json` 来自定义应用行为。

## 构建输出

构建后的应用文件位于 `src-tauri/target/release/` 目录。

## 许可证

MIT
