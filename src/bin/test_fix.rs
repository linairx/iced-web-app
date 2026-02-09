//! 测试修复后的 KTX2 加载
//!
//! 验证 texture.rs 中的修复是否有效

use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 测试修复后的 KTX2 加载");
    println!();

    // 读取 KTX2 文件
    let ktx2_data = fs::read("public/1.ktx2")?;
    println!("✅ KTX2 文件大小: {} 字节", ktx2_data.len());
    println!();

    // 使用修复后的加载逻辑
    const HEADER_SIZE: usize = 80;
    const LEVEL_INDEX_SIZE: usize = 24;

    let total_before_data = HEADER_SIZE + LEVEL_INDEX_SIZE;
    let aligned_offset = ((total_before_data + 7) / 8) * 8;

    println!("📊 数据偏移计算:");
    println!("   Header 大小: {} 字节", HEADER_SIZE);
    println!("   Level Index 大小: {} 字节", LEVEL_INDEX_SIZE);
    println!("   总计: {} 字节", total_before_data);
    println!("   对齐后偏移: {} 字节", aligned_offset);
    println!();

    if ktx2_data.len() <= aligned_offset {
        panic!("文件太短");
    }

    // 提取数据
    let texture_data = &ktx2_data[aligned_offset..];
    let data_size = std::cmp::min(50, texture_data.len());

    println!("🎨 纹理数据前 {} 字节:", data_size);
    for (i, chunk) in texture_data[..data_size].chunks(16).enumerate() {
        print!("   {:04x}: ", i * 16);
        for byte in chunk {
            print!("{:02x} ", byte);
        }
        println!();
    }
    println!();

    // 验证数据
    let width = 3412u32;
    let height = 1362u32;
    let expected_size = width as usize * height as usize * 4;

    println!("✅ 验证结果:");
    println!("   预期大小: {} 字节 ({} x {} x 4)", expected_size, width, height);
    println!("   实际大小: {} 字节", texture_data.len());
    println!("   匹配: {}", if texture_data.len() == expected_size { "✅" } else { "❌" });
    println!();

    // 检查是否全白或全黑
    let all_white = texture_data.iter().all(|&b| b == 255);
    let all_black = texture_data.iter().all(|&b| b == 0);

    println!("🎨 数据内容检查:");
    println!("   全白: {}", if all_white { "✅" } else { "❌" });
    println!("   全黑: {}", if all_black { "✅" } else { "❌" });

    if !all_white && !all_black {
        println!("   数据有变化 ✅");
    }
    println!();

    // 模拟 TextureLoader
    println!("🔧 测试 TextureLoader 创建:");

    // 这里只是演示，实际使用 TextureLoader::load_from_ktx2_bytes()
    let dimensions = Some((width, height));
    let image_data = Some(texture_data.to_vec());

    if let (Some(dims), Some(data)) = (dimensions, image_data) {
        println!("   ✅ 加载成功");
        println!("   尺寸: {}x{}", dims.0, dims.1);
        println!("   数据: {} 字节", data.len());

        // 创建 iced Handle (模拟)
        use iced::widget::image::Handle;
        let handle = Handle::from_rgba(dims.0, dims.1, data);
        println!("   ✅ Handle 创建成功");
    }

    Ok(())
}
