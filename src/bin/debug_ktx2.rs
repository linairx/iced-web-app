//! KTX2 调试工具
//!
//! 检查 KTX2 文件的实际数据内容

use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("用法: {} <ktx2_file>", args[0]);
        eprintln!("示例: {} public/1.ktx2", args[0]);
        std::process::exit(1);
    }

    let ktx2_path = &args[1];

    println!("🔍 KTX2 调试工具");
    println!("文件: {}", ktx2_path);
    println!();

    // 读取 KTX2 文件
    let ktx2_data = fs::read(ktx2_path)?;
    let size_mb = ktx2_data.len() as f64 / 1024.0 / 1024.0;
    println!("✅ 文件大小: {} 字节 ({:.2} MB)", ktx2_data.len(), size_mb);
    println!();

    // 使用 ktx2 crate 读取
    println!("📦 解析 KTX2 文件...");
    let reader = ktx2::Reader::new(&ktx2_data)?;
    let header = reader.header();

    println!("   格式: {:?}", header.format);
    println!("   类型大小: {}", header.type_size);
    println!("   尺寸: {}x{}", header.pixel_width, header.pixel_height);
    println!("   深度: {}", header.pixel_depth);
    println!("   层级数: {}", header.level_count);
    println!();

    // 读取层级数据
    let levels: Vec<_> = reader.levels().collect();
    println!("📊 层级数据:");
    println!("   层级数量: {}", levels.len());
    println!();

    for (i, level) in levels.iter().enumerate() {
        println!("   层级 {}:", i);
        println!("     数据长度: {} 字节", level.data.len());
        println!("     未压缩长度: {} 字节", level.uncompressed_byte_length);

        // 检查数据内容
        if level.data.len() > 0 {
            println!("     前 16 字节:");
            for (j, byte) in level.data.iter().take(16).enumerate() {
                print!("{:02x} ", byte);
                if (j + 1) % 8 == 0 {
                    println!();
                    print!("                  ");
                }
            }
            println!();

            // 计算预期数据大小
            let expected_size = header.pixel_width * header.pixel_height * 4; // RGBA8
            println!("     预期大小 (RGBA8): {} 字节", expected_size);
            println!("     实际大小: {} 字节", level.data.len());
            println!("     匹配: {}", if level.data.len() == expected_size as usize { "✅" } else { "❌" });
        } else {
            println!("     ⚠️  数据为空！");
        }
        println!();
    }

    // 直接从文件读取原始数据部分
    println!("🔍 直接读取文件数据部分...");

    // KTX2 文件结构:
    // - Header: 80 bytes
    // - Level Index: 24 bytes per level
    // - Data starts after header + index + alignment

    let header_size = 80;
    let level_index_size = 24; // 3 * u64
    let total_before_data = header_size + level_index_size;
    let aligned_offset = ((total_before_data + 7) / 8) * 8;

    println!("   Header 大小: {} 字节", header_size);
    println!("   Level Index 大小: {} 字节", level_index_size);
    println!("   对齐后偏移: {} 字节", aligned_offset);

    if ktx2_data.len() > aligned_offset {
        let raw_data = &ktx2_data[aligned_offset..];
        let raw_data_size = std::cmp::min(100, raw_data.len());
        println!("   原始数据前 {} 字节:", raw_data_size);
        for (j, byte) in raw_data.iter().take(raw_data_size).enumerate() {
            print!("{:02x} ", byte);
            if (j + 1) % 16 == 0 {
                println!();
            }
        }
        println!();
    }

    // 对比分析
    println!("📊 问题分析:");
    if levels.is_empty() {
        println!("   ❌ 没有层级数据");
    } else if levels[0].data.is_empty() {
        println!("   ❌ 第一层级数据为空");
        println!("   🔧 原因: ktx2 crate 可能未正确读取数据");
        println!("   💡 解决方案: 直接从文件偏移读取数据");
    } else if levels[0].data.len() != (header.pixel_width * header.pixel_height * 4) as usize {
        println!("   ⚠️  数据大小不匹配");
        println!("   💡 可能是格式问题或需要解压缩");
    } else {
        println!("   ✅ 数据看起来正确");
    }

    Ok(())
}
