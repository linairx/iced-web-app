//! 查找 KTX2 数据偏移

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ktx2_data = fs::read("public/1.ktx2")?;

    println!("🔍 KTX2 Header 分析");
    println!();

    // KTX2 header 格式:
    // 0-11: identifier
    // 12-15: vkFormat
    // 16-19: typeSize
    // 20-23: pixelWidth
    // 24-27: pixelHeight
    // 28-31: pixelDepth
    // 32-35: arrayElementCount
    // 36-39: faceCount
    // 40-43: levelCount
    // 44-47: supercompressionScheme
    // 48-83: data_format_descriptor[8] (4 u64)
    // 84-99: key_value_data[2] (2 u64)
    // 100-111: supercompression_global_data[2] (2 u64)

    // 读取 data_format_descriptor 偏移和长度
    let dfd_offset = u32::from_le_bytes(ktx2_data[48..52].try_into()?);
    let dfd_length = u32::from_le_bytes(ktx2_data[52..56].try_into()?);

    println!("📦 Header 中的 DFD 信息:");
    println!("   DFD 偏移: {} 字节 (0x{:04x})", dfd_offset, dfd_offset);
    println!("   DFD 长度: {} 字节", dfd_length);
    println!();

    // Level Index 紧跟在 DFD 之后
    let level_index_offset = dfd_offset as usize + dfd_length as usize;

    // 对齐到 4 字节
    let aligned_index_offset = ((level_index_offset + 3) / 4) * 4;

    println!("📊 Level Index 位置:");
    println!("   偏移: {} 字节 (0x{:04x})", aligned_index_offset, aligned_index_offset);
    println!();

    // Level Index 包含 3 个 u64 值
    let index_data = &ktx2_data[aligned_index_offset..aligned_index_offset + 24];

    let byte_offset = u64::from_le_bytes(index_data[0..8].try_into()?);
    let byte_length = u64::from_le_bytes(index_data[8..16].try_into()?);
    let uncompressed_length = u64::from_le_bytes(index_data[16..24].try_into()?);

    println!("📊 Level Index 内容:");
    println!("   byteOffset: 0x{:08x} ({} 字节)", byte_offset, byte_offset);
    println!("   byteLength: {} 字节 ({:.2} MB)", byte_length, byte_length as f64 / 1024.0 / 1024.0);
    println!("   uncompressedByteLength: {} 字节 ({:.2} MB)", uncompressed_length, uncompressed_length as f64 / 1024.0 / 1024.0);
    println!();

    // 验证数据
    if byte_offset as usize >= ktx2_data.len() {
        println!("❌ 偏移超出文件范围");
        return Ok(());
    }

    println!("✅ 数据偏移: 0x{:08x} ({} 字节)", byte_offset, byte_offset);

    let data_start = byte_offset as usize;
    let data_end = std::cmp::min(data_start + 64, ktx2_data.len());
    let preview = &ktx2_data[data_start..data_end];

    println!("🎨 数据预览 (前 {} 字节):", preview.len());
    for (i, chunk) in preview.chunks(16).enumerate() {
        print!("   {:04x}: ", byte_offset as usize + i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }

    // 验证大小
    let width = 3412u32;
    let height = 1362u32;
    let expected = width as u64 * height as u64 * 4;

    println!();
    println!("📏 大小验证:");
    println!("   预期 (RGBA8): {} 字节 ({:.2} MB)", expected, expected as f64 / 1024.0 / 1024.0);
    println!("   Level Index 报告: {} 字节", uncompressed_length);

    if uncompressed_length == expected {
        println!("   匹配: ✅");
    } else {
        let diff = uncompressed_length as i64 - expected as i64;
        println!("   匹配: ❌ 差值: {}", diff);
    }

    Ok(())
}
