# Iced Web 应用 - 快速开始指南

## ✅ 项目已创建成功！

你的 Iced Web 应用已经准备就绪，可以在浏览器中运行。

---

## 📁 项目位置

```bash
/home/lin/project/iced-web-app
```

---

## 🌐 在浏览器中访问

### 方法 1：使用本地服务器（推荐）

服务器已启动！访问：

```
http://localhost:8000
```

### 方法 2：手动启动

```bash
cd /home/lin/project/iced-web-app/web
python -m http.server 8000
```

然后访问：http://localhost:8000

---

## 📂 项目结构

```
iced-web-app/
├── src/
│   └── main.rs              # Rust 源代码（计数器应用）
├── web/                     # Web 构建输出
│   ├── index.html           # 入口页面
│   ├── iced_web_app.js      # JavaScript 绑定
│   └── iced_web_app_bg.wasm # WebAssembly 文件
├── Cargo.toml               # Rust 项目配置
├── build-web.sh             # 构建脚本
├── serve-web.sh             # 启动服务器脚本
└── README.md                # 详细文档
```

---

## 🎯 功能

一个简单的计数器应用：
- ✅ 增加计数（+ 按钮）
- ✅ 减少计数（- 按钮）
- ✅ 实时显示当前值
- ✅ 完全在浏览器中运行（通过 WebAssembly）

---

## 🔄 重新构建

如果你修改了代码，运行以下命令重新构建：

```bash
cd /home/lin/project/iced-web-app

# 方法 1：使用构建脚本
./build-web.sh

# 方法 2：手动构建
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/iced_web_app.wasm \
    --out-dir web \
    --target web
cp index.html web/
```

---

## 🛠️ 自定义应用

### 修改 UI

编辑 `src/main.rs` 中的 `view()` 方法：

```rust
fn view(&self) -> Element<Message> {
    column![
        text("你的标题").size(32),
        text(format!("计数: {}", self.value)).size(48),
        button("增加")
            .on_press(Message::Increment)
            .padding(10),
        button("减少")
            .on_press(Message::Decrement)
            .padding(10),
    ]
    .spacing(20)
    .into()
}
```

### 添加新功能

1. 在 `Message` 枚举中添加新消息：
```rust
enum Message {
    Increment,
    Decrement,
    Reset,  // 新增
}
```

2. 在 `update()` 方法中处理：
```rust
match message {
    Message::Increment => self.value += 1,
    Message::Decrement => self.value -= 1,
    Message::Reset => self.value = 0,  // 新增
}
```

3. 在 `view()` 方法中添加 UI：
```rust
button("重置")
    .on_press(Message::Reset)
    .padding(10),
```

---

## 📊 性能信息

- **WebAssembly 文件大小**：2.7 MB
- **JavaScript 绑定**：65 KB
- **加载时间**：通常 < 2 秒

### 优化建议

如果要减小文件大小：

1. **使用 wasm-opt**（需要安装 Binaryen）：
```bash
wasm-opt -O3 -o web/iced_web_app_bg_opt.wasm web/iced_web_app_bg.wasm
mv web/iced_web_app_bg_opt.wasm web/iced_web_app_bg.wasm
```

2. **启用压缩**：
在 `Cargo.toml` 中已配置：
```toml
[profile.release]
opt-level = "z"      # 优化大小
lto = true           # 链接时优化
codegen-units = 1    # 单个编译单元
```

---

## 🐛 故障排查

### 问题：浏览器控制台有错误

**解决方案**：
1. 打开浏览器开发者工具（F12）
2. 查看 Console 标签页
3. 检查具体错误信息

常见错误：
- **CORS 错误**：确保使用 HTTP 服务器，而不是直接打开 HTML 文件
- **MIME 类型错误**：确保服务器正确处理 `.wasm` 文件

### 问题：应用无法加载

**检查清单**：
- ✅ 确认服务器正在运行
- ✅ 确认访问 http://localhost:8000（不是 file://）
- ✅ 打开浏览器控制台查看错误
- ✅ 重新构建应用

### 问题：编译错误

**解决方案**：
```bash
# 清理构建缓存
cargo clean

# 重新构建
./build-web.sh
```

---

## 📚 学习资源

- [Iced 官方文档](https://docs.iced.rs/)
- [Iced GitHub](https://github.com/iced-rs/iced)
- [Iced 示例](https://github.com/iced-rs/iced/tree/master/examples)
- [WebAssembly Rust](https://rustwasm.github.io/)

---

## 🎉 下一步

1. **修改代码**：编辑 `src/main.rs`
2. **重新构建**：运行 `./build-web.sh`
3. **刷新浏览器**：按 F5 刷新页面

---

## 📞 获取帮助

遇到问题？

1. 查看浏览器控制台（F12）
2. 检查服务器日志
3. 查看详细文档：`README.md`

---

*项目创建时间: 2026-02-08*
*Iced 版本: 0.12*
*Rust 版本: 2021*
