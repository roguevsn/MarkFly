# 🚀 MarkFly - 快到起飞的 Markdown 编辑器

**MarkFly** 是一款基于 Tauri + Vue + ByteMD 构建的现代化跨平台 Markdown 编辑器，专为追求极致性能和流畅体验的用户设计。

> ⚡ **快到起飞** - 启动速度 < 2秒，内存占用 < 100MB，让你专注于创作而非等待

![MarkFly Logo](src-tauri/icons/icon.png)

## ✨ 核心特性

- 🚀 **极致性能**: 基于 Rust + Tauri，启动快速，内存占用极低
- 🖥️ **跨平台支持**: 完美支持 Windows、macOS、Linux 三大操作系统
- 📝 **实时预览**: 分屏实时预览，所见即所得的写作体验
- 🎨 **语法高亮**: 支持 180+ 编程语言的代码高亮显示
- 📊 **图表支持**: 内置 Mermaid 流程图、时序图等图表渲染
- 🧮 **数学公式**: 完整的 LaTeX 数学公式支持
- 📋 **GFM 扩展**: 完整支持 GitHub Flavored Markdown 规范
- 🔍 **图片缩放**: 点击图片即可放大查看细节
- 💾 **文件管理**: 支持新建、打开、保存 Markdown 文件
- 🌙 **主题切换**: 支持亮色和暗色两种主题模式
- 🎯 **专注写作**: 简洁直观的界面设计，让你专注于内容创作

## 🛠️ 技术栈

- **前端框架**: Vue 3 + TypeScript
- **编辑器核心**: ByteMD (基于 CodeMirror 6)
- **桌面框架**: Tauri (Rust)
- **构建工具**: Vite
- **插件系统**: 
  - `@bytemd/plugin-gfm` - GitHub Flavored Markdown
  - `@bytemd/plugin-highlight` - 代码语法高亮
  - `@bytemd/plugin-math` - 数学公式支持
  - `@bytemd/plugin-mermaid` - Mermaid 图表
  - `@bytemd/plugin-medium-zoom` - 图片缩放
  - `@bytemd/plugin-frontmatter` - Front Matter 支持

## 🚀 快速开始

### 环境要求

- Node.js 16+ 
- Rust 1.70+
- 系统要求：Windows 10+, macOS 10.15+, Linux (Ubuntu 18.04+)

### 安装依赖

```bash
# 安装前端依赖
npm install

# 安装 Tauri CLI (如果还没有安装)
npm install -g @tauri-apps/cli
```

### 开发模式

```bash
# 启动开发服务器
npm run tauri:dev
```

### 构建应用

```bash
# 构建生产版本
npm run tauri:build
```

## 📖 使用指南

### 快捷键

- `Ctrl/Cmd + N` - 新建文件
- `Ctrl/Cmd + O` - 打开文件
- `Ctrl/Cmd + S` - 保存文件
- `Ctrl/Cmd + Shift + P` - 切换预览模式

### 支持的文件格式

- `.md` - Markdown 文件
- `.markdown` - Markdown 文件
- `.txt` - 纯文本文件

## 🏗️ 项目结构

```
MarkFly/
├── src/                    # Vue 前端源码
│   ├── components/         # Vue 组件
│   ├── stores/             # 状态管理
│   ├── styles/             # 样式文件
│   ├── data/               # 示例数据
│   ├── App.vue             # 主应用组件
│   └── main.ts             # 应用入口
├── src-tauri/              # Tauri 后端源码
│   ├── src/                # Rust 源码
│   ├── icons/              # 应用图标
│   ├── Cargo.toml          # Rust 依赖配置
│   └── tauri.conf.json     # Tauri 配置
├── package.json            # Node.js 依赖配置
├── vite.config.ts          # Vite 构建配置
└── README.md               # 项目说明
```

## 🤝 贡献指南

欢迎贡献代码！请遵循以下步骤：

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情。

## 🙏 致谢

- [Tauri](https://tauri.app/) - 跨平台桌面应用框架
- [Vue.js](https://vuejs.org/) - 渐进式 JavaScript 框架
- [ByteMD](https://bytemd.js.org/) - 可扩展的 Markdown 编辑器
- [Vite](https://vitejs.dev/) - 下一代前端构建工具

## 📞 联系我们

- 项目主页: [GitHub](https://github.com/jwangkun/MarkFly.git)
- 问题反馈: [Issues](https://github.com/jwangkun/MarkFly.git/issues)
- 功能建议: [Discussions](https://github.com/jwangkun/MarkFly.git/discussions)

---

⭐ 如果这个项目对你有帮助，请给我们一个 Star！