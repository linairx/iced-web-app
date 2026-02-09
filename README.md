# Iced Web App - 纹理加载演示

使用 Iced 框架开发的 Rust Web 应用，支持 PNG 和 KTX2 纹理加载。

## ✨ 特性

- ✅ **Iced GUI** - 类型安全的 Rust GUI 框架
- ✅ **WebAssembly** - 在浏览器中运行
- ✅ **纹理支持** - PNG 和 KTX2 格式
- ✅ **鼠标事件** - 完整的鼠标事件处理
- ✅ **纯 Rust** - KTX2 生成和加载

## 🚀 快速开始

### 前置要求

```bash
# 安装 Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加 WASM 目标
rustup target add wasm32-unknown-unknown

# 安装 Bun
curl -fsSL https://bun.sh/install | bash
```

### 构建

```bash
# 构建 WASM
bun run build:all

# 启动开发服务器
bun run dev
```

访问: http://localhost:8080

## 📁 项目结构

```
iced-web-app/
├── src/
│   ├── main.rs              # 主应用
│   ├── texture.rs           # 纹理加载模块
│   └── bin/
│       ├── ktx2_generator.rs # KTX2 生成工具
│       └── test_ktx2.rs      # 测试工具
├── scripts/
│   ├── build-wasm.sh        # WASM 构建脚本
│   └── generate_ktx2.sh     # KTX2 生成脚本
├── public/                  # Web 静态文件
├── server.ts                # 开发服务器
└── Cargo.toml               # Rust 配置
```

## 🎨 功能演示

### 鼠标事件
- 鼠标移动跟踪
- 按键检测
- 滚轮事件
- 窗口进入/离开

### 纹理加载
- PNG 图像加载
- KTX2 纹理加载（未压缩 RGBA8）
- 动态预览

## 🔧 KTX2 工具

### 生成 KTX2 文件

```bash
# 使用便捷脚本
./scripts/generate_ktx2.sh input.png output.ktx2

# 或直接使用
cargo build --bin ktx2_generator --release
./target/release/ktx2_generator input.png output.ktx2
```

### 测试 KTX2 功能

```bash
cargo run --bin test_ktx2
```

详细说明请查看 [KTX2.md](KTX2.md)

## 📊 性能对比

| 格式 | 大小 | 加载 | 说明 |
|------|------|------|------|
| PNG | 577 KB | 需解码 | 推荐 Web 使用 |
| KTX2 (未压缩) | 18 MB | 直接上传 | 本地测试 |

## 🛠️ 开发

### 桌面测试

```bash
cargo run
```

### Web 构建

```bash
bun run build:all
bun run dev
```

## 📚 相关资源

- [Iced 文档](https://docs.iced.rs/)
- [KTX2 规范](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html)
- [WebAssembly Rust](https://rustwasm.github.io/)

## 📄 许可证

MIT License
