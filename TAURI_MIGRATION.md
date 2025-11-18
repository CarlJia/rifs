# RIFS Tauri 迁移指南

## 概述

本项目已成功迁移到 Tauri 跨平台桌面应用框架。新的前端使用现代化的技术栈，采用 shadcn/ui 组件库实现简洁干净的用户界面。

## 文件结构变化

### 新增文件和目录

```
/
├── src/                          # 前端源代码（新增）
│   ├── components/              # React 组件
│   │   ├── ui/                 # shadcn/ui 组件库
│   │   └── Layout.tsx
│   ├── pages/                   # 页面组件
│   │   ├── Home.tsx            # 首页（上传功能）
│   │   ├── Gallery.tsx         # 图片库（浏览）
│   │   ├── CacheManagement.tsx # 缓存管理
│   │   └── Login.tsx           # 登录页
│   ├── services/                # API 服务
│   │   └── api.ts              # 后端 API 调用
│   ├── hooks/                   # 自定义 Hooks
│   │   └── useAuth.ts          # 认证 Hook
│   ├── types/                   # TypeScript 类型定义
│   │   └── index.ts
│   ├── styles/                  # 全局样式
│   │   └── globals.css
│   ├── App.tsx                  # 主应用组件
│   └── main.tsx                 # 入口文件
├── index.html                   # HTML 模板（新增）
├── package.json                 # Node.js 项目配置（新增）
├── tsconfig.json                # TypeScript 配置（新增）
├── vite.config.ts               # Vite 构建配置（新增）
├── tailwind.config.js           # Tailwind CSS 配置（新增）
├── postcss.config.js            # PostCSS 配置（新增）
├── tauri.conf.json              # Tauri 应用配置（新增）
├── .prettierrc                  # 代码格式化配置（新增）
└── FRONTEND_README.md           # 前端文档（新增）
```

### 保留的文件

```
/web/                           # 原始 web 文件（保留用于参考）
├── index.html
├── gallery.html
├── cache_management.html
├── login.html
├── app.js
├── gallery.js
├── cache_management.js
├── login.js
├── style.css
└── README.md
```

## 技术栈升级

### 前端栈
- **React 18** - UI 框架
- **TypeScript 5** - 类型安全的 JavaScript
- **Vite 5** - 现代化构建工具
- **Tailwind CSS 3** - 实用型 CSS 框架
- **shadcn/ui** - 高质量 UI 组件库
- **Lucide React** - 现代 SVG 图标库
- **Tauri** - 跨平台桌面应用框架

### 后端（无变化）
- **Rust** + **Axum** - Web 框架
- **SeaORM** - 数据库 ORM
- **Tokio** - 异步运行时

## 主要特性

### 1. 现代化界面
- 深色主题设计
- 简洁干净的布局
- 响应式设计
- 平滑的动画和过渡

### 2. 图片上传
- 拖拽上传支持
- 多文件批量上传
- 实时上传进度显示
- 上传结果支持多种格式（URL、Markdown、HTML）
- 一键复制功能
- 自动复制选项

### 3. 图片库
- 缩略图预览
- 无限滚动加载
- 图片查看原图功能
- 总数统计

### 4. 缓存管理
- 实时缓存统计
- 灵活的清理策略（按年龄或大小）
- 缓存详情表格展示

### 5. 认证系统
- 支持令牌认证
- 自定义认证头支持
- 安全的本地存储
- 登出功能

## 构建和运行

### 开发模式
```bash
npm install
npm run tauri:dev
```

### 生产构建
```bash
npm run tauri:build
```

### 纯前端构建（用于 Web 服务）
```bash
npm run build
```

输出在 `dist/` 目录。

## 配置

### API 基址配置
编辑 `src/services/api.ts`：
```typescript
const API_BASE_URL = 'http://localhost:3000'  // 修改为实际后端地址
```

### Tauri 应用配置
编辑 `tauri.conf.json` 来自定义：
- 窗口大小
- 应用标题
- 最小/最大尺寸
- 打包选项

## 样式指南

### 颜色方案
- **背景**：深蓝-黑色 (#0f172a)
- **前景**：浅色文本 (#e2e8f0)
- **主色**：青蓝色 (#06b6d4)
- **强调**：蓝色 (#3b82f6)
- **成功**：绿色 (#10b981)
- **错误**：红色 (#ef4444)

### 间距
- 使用 Tailwind CSS 的标准间距单位
- 卡片间距：20px
- 内部填充：16-20px

### 排版
- 字体系列：System fonts
- 标题：2.5rem (h1)、2rem (h2)、1.5rem (h3)
- 正文：0.875-1rem

## 性能优化

1. **代码分割** - Vite 自动处理
2. **懒加载** - 图片使用 loading="lazy"
3. **Intersection Observer** - 图片库无限滚动
4. **防抖** - 滚动事件处理
5. **图片优化** - 缩略图生成 (w400_h200_jpeg_q80)

## 跨平台支持

### Windows
- 原生 Windows 应用
- 可执行文件：`RIFS.exe`

### macOS
- 原生 macOS 应用
- 打包格式：`.app`

### Linux
- AppImage 或 DEB 包
- 原生系统集成

## 迁移注意事项

1. **API 端点**：Tauri 应用需要正确的 CORS 配置或使用本地代理
2. **认证**：令牌存储在浏览器本地存储中
3. **开发服务器**：开发模式使用 Vite 开发服务器（端口 5173）
4. **生产构建**：生产构建会生成独立的 HTML/JS/CSS 文件

## 故障排除

### 构建失败
检查：
- Node.js 版本（需要 20+）
- npm 版本（需要 11+）
- 依赖安装是否完整

### API 连接失败
检查：
- 后端服务是否运行
- API_BASE_URL 配置是否正确
- CORS 是否正确配置
- 防火墙/网络设置

### 类型错误
运行：
```bash
npm run build
```
确保所有 TypeScript 错误都已解决。

## 未来改进方向

1. 添加图片编辑功能
2. 支持文件夹管理
3. 搜索和过滤功能
4. 图片标签和分类
5. 分享链接生成
6. 离线功能支持
7. 同步功能

## 贡献指南

当修改前端代码时：
1. 遵循现有的代码风格
2. 使用 shadcn/ui 组件
3. 使用 Tailwind CSS 进行样式
4. 添加适当的类型定义
5. 运行 `npm run build` 验证没有错误

## 许可证

MIT
