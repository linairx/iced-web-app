# Iced Web 应用

一个使用 Iced 框架开发的 Rust Web 应用，可以编译为 WebAssembly 在浏览器中运行。

## 功能

- ✅ 使用 Iced GUI 框架
- ✅ 编译为 WebAssembly
- ✅ 在浏览器中运行
- ✅ 简单的计数器示例

## 项目结构

```
iced-web-app/
├── src/
│   └── main.rs          # 主程序
├── index.html           # Web 入口页面
├── Cargo.toml           # Rust 项目配置
├── build-web.sh         # 构建脚本
├── serve-web.sh         # 启动服务器脚本
└── web/                 # 构建输出目录
    ├── index.html
    ├── iced_web_app.js
    └── iced_web_app_bg.wasm
```

## 快速开始

### 1. 安装依赖

```bash
# 安装 Rust（如果还没有）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加 WebAssembly 目标
rustup target add wasm32-unknown-unknown

# 安装 wasm-bindgen
cargo install wasm-bindgen-cli
```

### 2. 构建 Web 应用

```bash
# 运行构建脚本
./build-web.sh
```

构建脚本会自动：
1. 检查并安装 wasm32 目标
2. 检查并安装 wasm-bindgen
3. 编译 Rust 代码为 WebAssembly
4. 生成 JavaScript 绑定
5. 复制文件到 web 目录
6. 优化 WebAssembly（如果安装了 wasm-opt）

### 3. 运行应用

```bash
# 运行启动脚本
./serve-web.sh
```

或手动启动：

```bash
cd web
python -m http.server 8000
```

然后在浏览器中访问：http://localhost:8000

## 手动构建步骤

如果你想手动控制构建过程：

```bash
# 1. 编译为 WebAssembly
cargo build --release --target wasm32-unknown-unknown

# 2. 生成 JavaScript 绑定
wasm-bindgen target/wasm32-unknown-unknown/release/iced_web_app.wasm \
    --out-dir web \
    --target web

# 3. 复制 HTML 文件
cp index.html web/

# 4. 启动服务器
cd web && python -m http.server 8000
```

## 可选：优化 WebAssembly

安装 Binaryen 工具包来优化 wasm 文件：

```bash
# Arch Linux
sudo pacman -S binaryen

# macOS
brew install binaryen

# Ubuntu/Debian
sudo apt install binaryen
```

优化后 wasm 文件会显著变小（通常减少 30-50%）。

## 开发

### 修改代码

编辑 `src/main.rs` 文件，然后重新运行构建脚本。

### 本地测试

```bash
# 快速测试（桌面版）
cargo run

# 构建 Web 版
./build-web.sh
./serve-web.sh
```

## 自定义

### 修改 UI

在 `src/main.rs` 中的 `view()` 方法中修改 UI：

```rust
fn view(&self) -> Element<Message> {
    column![
        text("标题"),
        button("按钮").on_press(Message::ButtonClicked),
    ]
    .into()
}
```

### 添加新功能

1. 在 `Message` 枚举中添加新消息类型
2. 在 `update()` 方法中处理消息
3. 在 `view()` 方法中添加 UI 元素

## 技术栈

- **Rust** - 系统编程语言
- **Iced** - 跨平台 GUI 框架（灵感来自 Elm）
- **WebAssembly** - 在浏览器中运行 Rust 代码
- **wasm-bindgen** - Rust 和 JavaScript 之间的桥梁

## 为什么选择 Iced？

- 🎨 **类型安全**：编译时类型检查
- 🚀 **高性能**：原生性能 + WebAssembly
- 🔄 **跨平台**：一套代码，多个平台（桌面、Web）
- 📦 **简单**：Elm 架构，易于理解
- 🎯 **现代**：活跃开发，社区支持

## 相关资源

- [Iced 官方文档](https://docs.iced.rs/)
- [Iced GitHub](https://github.com/iced-rs/iced)
- [WebAssembly Rust](https://rustwasm.github.io/)
- [wasm-bindgen](https://rustwasm.github.io/wasm-bindgen/)

## 许可证

MIT License
