# RIFS - Rust图床服务

<div align="center">

![RIFS Logo](https://img.shields.io/badge/RIFS-Rust%20Image%20File%20Server-blue?style=for-the-badge&logo=rust)

<p>
  <img src="https://img.shields.io/badge/Rust-1.85+-orange.svg?style=flat-square" alt="Rust Version">
  <img src="https://img.shields.io/badge/License-MIT-blue.svg?style=flat-square" alt="License">
  <img src="https://img.shields.io/badge/Status-Production%20Ready-green.svg?style=flat-square" alt="Status">
  <img src="https://img.shields.io/badge/Platform-Linux%20%7C%20macOS%20%7C%20Windows-lightgrey.svg?style=flat-square" alt="Platform">
</p>

**🚀 高性能 Rust 图床服务 • 支持实时转换和智能缓存**

</div>

---

## ⚠️ 重要声明

<div align="center">

**🤖 本项目完全由 AI (Claude) 生成和编写 🤖**

**此项目包括所有代码、文档、配置文件等均为人工智能自动生成**  
**请在使用前仔细检查和测试，AI生成的代码可能存在潜在问题**  
**不建议直接用于生产环境，需要经过充分的测试和验证**

</div>

---

## 📖 完整文档

👉 **[查看完整文档](./DOCUMENTATION.md)**

完整文档包含以下内容：
- 项目概述和系统架构
- 快速开始指南
- 功能特性详解
- API接口文档
- 配置说明
- 前端开发指南
- 移动端适配
- 部署指南
- 更新日志

---

## 🚀 快速开始

### 本地运行

```bash
# 克隆项目
git clone https://github.com/djkcyl/rifs.git
cd rifs

# 运行
cargo run --release
```

### Docker 运行

```bash
docker run --rm --pull always -d \
  -p 3000:3000 \
  -v ./uploads:/app/uploads \
  -v ./cache:/app/cache \
  -v ./data:/app/data \
  -v ./config.toml:/app/config.toml \
  djkcyl/rifs:latest
```

### Tauri 桌面应用

```bash
# 安装依赖
npm install

# 开发模式
npm run tauri:dev

# 生产构建
npm run tauri:build
```

---

## ✨ 核心特性

- **高性能** - Rust编写，内存安全，高并发处理
- **多格式支持** - 支持JPEG、PNG、GIF、WebP、AVIF、ICO 6种主流图片格式
- **实时转换** - 通过URL参数进行图片尺寸、格式、质量转换
- **智能缓存** - 自动缓存转换结果，支持LRU清理策略
- **去重存储** - SHA256哈希去重，避免重复存储
- **管理面板** - 内置Web管理界面，支持缓存管理和系统监控
- **用户管理** - 基于角色的访问控制，支持管理员和普通用户
- **跨平台** - 支持Web、Windows、macOS、Linux

---

## 📊 管理面板

- **API文档**: http://localhost:3000/
- **图片画廊**: http://localhost:3000/gallery
- **缓存管理**: http://localhost:3000/cache/management
- **登录页面**: http://localhost:3000/login
- **用户管理**: http://localhost:3000/user-management

---

## 🔗 相关链接

- [完整文档](./DOCUMENTATION.md)
- [GitHub 仓库](https://github.com/djkcyl/rifs)
- [Docker Hub](https://hub.docker.com/r/djkcyl/rifs)

---

<div align="center">

Made with ❤️ and 🦀

</div>