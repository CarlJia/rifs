# 移动端样式适配文档

## 概述

Tauri 应用已完全适配移动设备和平板，提供流畅的跨设备体验。采用"Mobile First"响应式设计原则，从小屏幕开始优化，逐步扩展到大屏幕。

## 适配断点

采用 Tailwind CSS 的标准断点：

| 断点 | 宽度 | 设备 |
|------|------|------|
| `xs` | <640px | 手机纵向 |
| `sm` | ≥640px | 手机横向/小平板 |
| `md` | ≥768px | 平板 |
| `lg` | ≥1024px | 桌面 |
| `xl` | ≥1280px | 大屏桌面 |

## 响应式设计策略

### 1. Layout 组件 (src/components/Layout.tsx)

#### 侧边栏适配
- **移动设备(<768px)**：
  - 侧边栏默认隐藏（通过 `hidden md:flex`）
  - 可通过菜单按钮切换显示/隐藏
  - 显示时覆盖主内容（fixed 定位）
  - 添加背景层(overlay)提高交互体验
  - 菜单选项后自动关闭侧边栏

- **平板/桌面(≥768px)**：
  - 侧边栏常显示
  - 无需切换按钮
  - 正常布局

#### 头部优化
- **移动设备**：
  - 只显示菜单按钮
  - 标题和账户信息隐藏
  - 间距减小（px-3 py-3）

- **桌面**：
  - 显示完整标题
  - 显示账户菜单
  - 间距增大（px-6 py-4）

#### 菜单项优化
```
移动: px-3 py-2 text-sm
桌面: px-4 py-2 text-base
```

### 2. 首页 (Home.tsx) 

#### 上传表单
- **移动设备**：
  - 卡片内间距减小
  - 按钮全宽显示
  - 文本大小减小

#### 上传结果卡片
- **移动设备**：
  - 单列布局
  - 输入框下方显示按钮（flex-col）
  - 按钮文本隐藏，仅显示图标（hidden sm:inline）

- **小屏幕(sm)**：
  - 输入框和按钮行显示（flex-row）
  - 文本按钮可见

- **平板+**：
  - 完整按钮文本显示
  - 更大的间距

#### API 端点卡片
- **移动设备**：
  - 代码块转换参数为2列网格
  - 文本大小减小

- **平板+**：
  - 4列网格
  - 更大字体

### 3. 图片库 (Gallery.tsx)

#### 响应式网格

```
移动: grid-cols-2     (2列)
sm:   sm:grid-cols-3  (3列)
lg:   lg:grid-cols-4  (4列)
xl:   xl:grid-cols-5  (5列)
```

#### 图片卡片优化
- **移动设备**：
  - 较小的间距（gap-2）
  - 按钮文本缩短("看" 代替 "查看")
  - 哈希显示长度减少

- **触摸优化**：
  - 添加 `group-active:scale-110` 支持触摸反馈
  - 按钮高度适应 (h-7 md:h-8)
  - 触摸友好的按钮大小

### 4. 缓存管理 (CacheManagement.tsx)

#### 统计卡片
```
移动: grid-cols-2 gap-2     (2列，小间距)
平板+: grid-cols-2 gap-4     (2列，大间距)
```

#### 表格适配
- **移动设备**：
  - 隐藏"大小"列（hidden sm:table-cell）
  - 隐藏"最后访问"列（hidden md:table-cell）
  - 只显示哈希和格式
  - 减小padding和文本大小

- **平板(sm)**：
  - 显示大小列

- **桌面(md)**：
  - 显示完整表格
  - 全部4列可见

- **提示信息**：
  - 移动端显示"在平板或桌面设备上查看完整详情"

### 5. 设置页面 (Settings.tsx)

#### 表单适配
- **移动设备**：
  - 按钮堆叠（flex-col）
  - 标签和输入框紧凑

- **小屏幕+**：
  - 按钮并排（sm:flex-row）

#### 配置信息显示
- **移动设备**：
  - 竖直排列显示（flex-col）
  - 文本可换行

- **小屏幕+**：
  - 水平排列（sm:flex-row sm:justify-between）

### 6. 登录页 (Login.tsx)

#### 卡片响应
- **移动设备**：
  - 全屏显示，边距 p-3
  - 紧凑的表单间距

- **平板+**：
  - 居中显示，最大宽度 md
  - 更大的间距

## 排版适配

### 字体大小缩放

```
移动:           桌面:
text-xs (12px) → text-sm (14px)
text-sm (14px) → text-base (16px)
text-base      → text-lg
text-lg        → text-xl
text-xl        → text-2xl
```

### 间距缩放

```
移动:    桌面:
p-3  → p-6
gap-2 → gap-4
my-3  → my-6
```

## 触摸友好性

### 按钮和交互元素

#### 最小点击区域
- 移动设备上所有可交互元素最小高度: 44px (iOS) / 48px (Android)
- 按钮内间距: `px-2 md:px-3`、`py-2 md:py-3`

#### 使用 `group-active` 进行触摸反馈
```tsx
// 例: 图片库中的图片卡片
className="group-hover:scale-110 group-active:scale-110"
```

### 输入框优化
- 字体大小 ≥ 16px（防止 iOS 自动缩放）
- 充足的 padding
- 清晰的 focus 状态

## 性能考虑

### 移动端优化

1. **图片优化**：
   - 使用缩略图（w400_h200_jpeg_q80）
   - 懒加载图片（loading="lazy"）

2. **布局抖动防止**：
   - 为动态内容设置 min-height
   - 使用 aspect-ratio 保持比例

3. **减少重排**：
   - 使用 CSS Grid 和 Flexbox
   - 避免频繁的宽度计算

4. **事件优化**：
   - 使用防抖和节流
   - 避免在移动端使用 hover（改用 active）

## 测试检查清单

### 移动设备 (320px-640px)
- [ ] 侧边栏可切换显示/隐藏
- [ ] 所有按钮可点击（≥44px）
- [ ] 文本可读（无过小字体）
- [ ] 表单输入可用
- [ ] 图片正确加载和显示
- [ ] 没有水平滚动（除非必要）

### 平板 (640px-1024px)
- [ ] 1-2 栏布局清晰
- [ ] 表格显示必要信息
- [ ] 侧边栏自动显示

### 桌面 (≥1024px)
- [ ] 完整功能展示
- [ ] 所有列都显示
- [ ] 最优用户体验

## 常见问题

### Q: 如何在新页面中应用移动端适配？
A: 遵循以下模式：
```tsx
// 侧边栏容器
<div className="p-3 md:p-6">...</div>

// 网格布局
<div className="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4">...</div>

// 按钮组
<div className="flex flex-col sm:flex-row gap-2 md:gap-3">...</div>

// 文本大小
<h1 className="text-base md:text-xl">标题</h1>
```

### Q: 为什么隐藏某些列而不是滚动？
A: 移动设备水平滚动用户体验差，我们只显示最关键的信息，完整数据在更大屏幕上可见。

### Q: 如何测试移动端？
A: 
1. 使用浏览器开发者工具的设备模拟
2. 实际设备测试
3. 检查触摸交互和姿态变化

## 未来改进

- [ ] 添加更多平板特定的优化
- [ ] 实现竖屏/横屏自适应
- [ ] 添加手势识别（滑动导航）
- [ ] PWA 支持
- [ ] 离线功能

## 参考资源

- [Tailwind CSS 响应式设计](https://tailwindcss.com/docs/responsive-design)
- [Mobile Web Best Practices](https://web.dev/mobile/)
- [Apple Human Interface Guidelines](https://developer.apple.com/design/human-interface-guidelines/)
- [Material Design 3](https://m3.material.io/)
