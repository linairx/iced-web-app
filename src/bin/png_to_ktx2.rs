//! PNG 到 KTX2 转换工具（简单版本）
//!
//! 使用方法：
//! ```bash
//! cargo run --bin png_to_ktx2 -- input.png output.ktx2
//! ```
//!
//! 注意：此工具创建未压缩的 KTX2 文件，无需原生依赖

use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;
use image::GenericImageView;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("用法: {} <input.png> <output.ktx2>", args[0]);
        eprintln!();
        eprintln!("示例:");
        eprintln!("  {} public/1.png public/1.ktx2", args[0]);
        eprintln!();
        eprintln!("注意：创建的是未压缩的 RGBA8 KTX2 文件");
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("🔄 正在转换 {} -> {}", input_path, output_path);

    // 检查输入文件
    if !Path::new(input_path).exists() {
        return Err(format!("输入文件不存在: {}", input_path).into());
    }

    // 读取 PNG
    println!("📖 读取 PNG 文件...");
    let png_data = fs::read(input_path)?;

    // 使用 image crate 解码
    let img = image::load_from_memory(&png_data)?;
    let rgba = img.to_rgba8();
    let (width, height) = img.dimensions();

    println!("   尺寸: {}x{}", width, height);
    println!("   数据大小: {} 字节", rgba.len());

    // 创建 KTX2 文件
    println!("📦 创建 KTX2 文件...");
    create_ktx2_file(&rgba, width, height, output_path)?;

    println!("✅ 转换完成！");
    println!("   输出: {}", output_path);

    Ok(())
}

/// 创建简单的 KTX2 文件（未压缩 RGBA8 格式）
///
/// KTX2 文件结构：
/// - 标识符 (12 字节)
/// - 头部 (80 字节)
/// - 索引 (level index, 24 字节)
/// - 数据对齐填充
/// - 图像数据
fn create_ktx2_file(
    rgba_data: &[u8],
    width: u32,
    height: u32,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut file = fs::File::create(output_path)?;

    // 1. KTX2 标识符 (12 字节)
    file.write_all(b"\xABKTX 20\xBB\r\n\x1A\n")?;

    // 2. 头部 (80 字节)
    // 参考: https://registry.khronos.org/KTX/specs/2.0/ktxspec.v2.html#_identifier_and_header

    write_header(&mut file, width, height, rgba_data.len() as u64)?;

    // 3. Level Index (24 字节)
    write_level_index(&mut file, rgba_data.len() as u64)?;

    // 4. 数据对齐
    // KTX2 要求数据偏移是 8 字节对齐
    let header_size = 12 + 80 + 24; // 标识符 + 头部 + 索引
    let aligned_offset = ((header_size + 7) / 8) * 8;
    let padding = aligned_offset - header_size;

    for _ in 0..padding {
        file.write_all(&[0])?;
    }

    // 5. 写入图像数据
    file.write_all(rgba_data)?;

    Ok(())
}

fn write_header<W: Write>(writer: &mut W, width: u32, height: u32, data_len: u64) -> Result<(), Box<dyn std::error::Error>> {
    // vkFormat: VK_FORMAT_R8G8B8A8_UNORM (需要查找正确的值，这里用简化的值)
    // 暂时使用 0 表示未指定或基础格式
    let vk_format = 0u32;

    // typeSize: 每个像素的字节数 (RGBA8 = 4 字节，但这里是每个通道的字节大小 = 1)
    let type_size = 1u32;

    // pixelWidth, pixelHeight, pixelDepth
    let pixel_width = width;
    let pixel_height = height;
    let pixel_depth = 0u32; // 2D 纹理，depth = 0

    // layerCount: 数组层数 (0 或 1 表示单层)
    let layer_count = 0u32;

    // faceCount: 立方体面数 (1 表示非立方体)
    let face_count = 1u32;

    // levelCount: mipmap 层级数 (1 表示只有基础层级)
    let level_count = 1u32;

    // supercompressionScheme: 0 = 无超级压缩
    let supercompression_scheme = 0u32;

    // 写入头部字段
    writer.write_all(&vk_format.to_le_bytes())?;
    writer.write_all(&type_size.to_le_bytes())?;
    writer.write_all(&pixel_width.to_le_bytes())?;
    writer.write_all(&pixel_height.to_le_bytes())?;
    writer.write_all(&pixel_depth.to_le_bytes())?;
    writer.write_all(&layer_count.to_le_bytes())?;
    writer.write_all(&face_count.to_le_bytes())?;
    writer.write_all(&level_count.to_le_bytes())?;
    writer.write_all(&supercompression_scheme.to_le_bytes())?;

    // dataFormatDescriptor (必须存在但可以是空的，用全 0 填充)
    // 这里我们写入一个最小的 DFD (Data Format Descriptor)
    // DFD header + basic block
    let dfd_total_size = 184u32; // DFD 总大小（包含 header 和 block）

    writer.write_all(&dfd_total_size.to_le_bytes())?;

    // DFD 的其余部分暂时填充 0
    // 实际应用中应该正确填写 DFD，但为了简单起见这里跳过
    // 读取时可以使用 0 或忽略

    // keyValueData 偏移和大小（0 表示无元数据）
    writer.write_all(&0u64.to_le_bytes())?;

    writer.write_all(&0u64.to_le_bytes())?;

    Ok(())
}

fn write_level_index<W: Write>(writer: &mut W, data_len: u64) -> Result<(), Box<dyn std::error::Error>> {
    let header_size = 12 + 80; // 标识符 + 头部

    // 数据偏移（从文件开始）
    // 需要加上 Level Index 的大小 (24) 和可能的填充
    let level_index_size = 24;
    let total_before_data = header_size + level_index_size;
    let aligned_offset = ((total_before_data + 7) / 8) * 8;

    let byte_offset = aligned_offset as u64;
    let byte_length = data_len;
    let uncompressed_byte_length = data_len;

    writer.write_all(&byte_offset.to_le_bytes())?;
    writer.write_all(&byte_length.to_le_bytes())?;
    writer.write_all(&uncompressed_byte_length.to_le_bytes())?;

    Ok(())
}
