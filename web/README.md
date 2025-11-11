# RIFS 前端界面

这是 RIFS (Rust Image File Server) 的独立前端界面，采用紧凑型设计风格。

## 文件结构

```
web/
├── index.html          # 主页 - 图片上传和API文档
├── gallery.html        # 图片库页面
├── style.css           # 紧凑型样式表
├── app.js             # 主页JavaScript功能
├── gallery.js         # 图片库JavaScript功能
└── README.md          # 本文档
```

## 特性

### 紧凑型设计
- **极简布局**: 减少不必要的空白和装饰
- **信息密度**: 在有限空间内展示更多有用信息
- **快速响应**: 优化的CSS和JavaScript，确保快速加载
- **移动友好**: 响应式设计，适配各种屏幕尺寸

### 主要功能

#### 主页 (index.html)
- **图片上传**: 支持拖拽上传、多文件选择
- **API文档**: 清晰的接口说明和示例
- **转换参数**: 详细的图片转换参数说明
- **存储信息**: 系统存储策略介绍

#### 图片库 (gallery.html)
- **瀑布流展示**: 响应式图片网格布局
- **分页加载**: 支持无限滚动或分页浏览
- **图片预览**: 点击图片查看大图
- **统计信息**: 实时显示图片数量和存储统计

### 技术特点

#### CSS优化
```css
/* 紧凑型样式示例 */
.container { max-width: 900px; margin: 0 auto; }
.card { margin-bottom: 20px; }  /* 减少间距 */
.btn { padding: 12px 24px; }    /* 适中的按钮尺寸 */
```

#### JavaScript功能
- **异步上传**: 非阻塞的文件上传体验
- **实时反馈**: 上传进度和结果即时显示
- **错误处理**: 友好的错误提示信息
- **自动复制**: 可选的链接自动复制功能

## 部署说明

### 静态文件服务
前端文件通过 `/static/*` 路径提供服务：

- `/static/style.css` - 样式文件
- `/static/app.js` - 主页脚本
- `/static/gallery.js` - 图片库脚本

### 页面路由
- `/` - 主页 (api_docs 处理器)
- `/gallery` - 图片库页面 (gallery_page 处理器)

## 自定义配置

### 修改样式
编辑 `style.css` 文件来自定义界面外观：

```css
/* 修改主题色 */
:root {
    --primary-color: #06b6d4;
    --secondary-color: #3b82f6;
}
```

### 添加功能
在对应的 JavaScript 文件中添加新功能：

```javascript
// app.js - 添加上传前验证
function validateFile(file) {
    // 文件验证逻辑
    return true;
}
```

## 浏览器兼容性

- **现代浏览器**: Chrome 60+, Firefox 55+, Safari 12+, Edge 79+
- **移动浏览器**: iOS Safari 12+, Chrome Mobile 60+
- **必需特性**: ES6, Fetch API, CSS Grid, Flexbox

## 性能优化

1. **CSS压缩**: 生产环境建议压缩CSS文件
2. **JavaScript压缩**: 使用工具压缩JS代码
3. **图片优化**: 使用适当的图片格式和尺寸
4. **缓存策略**: 设置合适的HTTP缓存头

## 开发调试

### 本地开发
```bash
# 启动RIFS服务器
cargo run

# 访问前端页面
open http://localhost:3000
```

### 调试技巧
1. 使用浏览器开发者工具调试JavaScript
2. 检查网络请求确保API调用正常
3. 验证响应式设计在不同屏幕尺寸下的表现

## 更新日志

### v1.0.0 (当前版本)
- ✅ 独立HTML文件结构
- ✅ 紧凑型CSS设计
- ✅ 完整的上传功能
- ✅ 图片库展示
- ✅ 响应式布局
- ✅ API文档集成

## 贡献指南

1. 保持代码风格一致
2. 添加适当的注释
3. 确保移动端兼容性
4. 测试所有功能正常工作
5. 更新相关文档

## 许可证

与主项目保持一致的 MIT 许可证。
