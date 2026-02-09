# KTX2 纹理支持

本项目实现了 KTX2 纹理格式支持，可以从 PNG 生成 KTX2 文件并在 iced 应用中加载。

## 功能特性

- ✅ 纯 Rust KTX2 生成器
- ✅ PNG → KTX2 转换
- ✅ iced Web 应用集成
- ✅ WASM 环境支持

## 快速开始

### 1. 生成 KTX2 文件

```bash
# 编译生成器
cargo build --bin ktx2_generator --release

# 从 PNG 生成 KTX2
./target/release/ktx2_generator input.png output.ktx2

# 或使用便捷脚本
./scripts/generate_ktx2.sh input.png output.ktx2
```

### 2. 在代码中使用

```rust
use texture::TextureLoader;

let mut loader = TextureLoader::new();
let data = std::fs::read("texture.ktx2")?;
loader.load_from_ktx2_bytes(&data)?;

if let Some(handle) = loader.as_iced_handle() {
    image(handle)  // 在 iced UI 中显示
}
```

## 文件说明

| 工具 | 文件 | 说明 |
|------|------|------|
| KTX2 生成器 | `src/bin/ktx2_generator.rs` | 从 PNG 生成 KTX2 |
| 测试工具 | `src/bin/test_ktx2.rs` | 功能测试 |
| 纹理模块 | `src/texture.rs` | 纹理加载实现 |
| 生成脚本 | `scripts/generate_ktx2.sh` | 便捷生成脚本 |

## 格式说明

当前实现生成的是**未压缩的 RGBA8 KTX2**：

- **优点**: 纯 Rust，无原生依赖，WASM 友好
- **缺点**: 文件较大（~18MB for 3412×1362 图像）
- **适用**: 快速开发测试

### 压缩 KTX2

如需更小的文件，可使用以下方案：

1. **在线工具**: https://basis-universal-demo.appspot.com/
2. **安装 toktx**: `./scripts/install_ktx_software.sh`
3. **命令行**: `toktx --basis --uastc all input.png output.ktx2`

## 性能对比

| 格式 | 大小 | 加载 | GPU |
|------|------|------|-----|
| PNG | 577 KB | 慢 (需解码) | ⭐⭐⭐ |
| KTX2 (未压缩) | 18 MB | 快 | ⭐⭐⭐⭐⭐ |
| KTX2 (Basis) | ~300 KB | 最快 | ⭐⭐⭐⭐ |

## 技术细节

### KTX2 文件结构

```
+-------------------+
| Header (80 bytes) |
+-------------------+
| Data Format Desc  |
+-------------------+
| Level Index       |
+-------------------+
| Texture Data      |
+-------------------+
```

### 关键参数

- vkFormat: 0 (UNDEFINED)
- typeSize: 1
- levelCount: 1
- supercompressionScheme: 0 (无压缩)
- 颜色空间: sRGB
- 格式: RGBA8

## 参考资料

- [KTX2 规范](https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html)
- [ktx2 crate](https://docs.rs/ktx2)
- [KTX Software](https://github.com/KhronosGroup/KTX-Software)
- [Basis Universal](https://github.com/BinomialLLC/basis_universal)
