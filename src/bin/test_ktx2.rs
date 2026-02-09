//! KTX2 加载测试
//!
//! 测试 KTX2 文件的生成和加载

use std::fs;

// 简单的 TextureLoader 定义用于测试
struct TextureLoader {
    image_data: Option<Vec<u8>>,
    dimensions: Option<(u32, u32)>,
}

impl TextureLoader {
    fn new() -> Self {
        Self {
            image_data: None,
            dimensions: None,
        }
    }

    fn load_from_ktx2_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let reader = ktx2::Reader::new(bytes)
            .map_err(|e| format!("Failed to create KTX2 reader: {:?}", e))?;

        let header = reader.header();
        let width = header.pixel_width;
        let height = if header.pixel_height > 0 {
            header.pixel_height
        } else {
            1
        };

        let levels: Vec<_> = reader.levels().collect();

        if levels.is_empty() {
            return Err("No texture data found in KTX2 file".to_string());
        }

        let first_level = &levels[0];
        let texture_data = first_level.data.to_vec();

        self.dimensions = Some((width, height));
        self.image_data = Some(texture_data);

        Ok(())
    }

    fn dimensions(&self) -> Option<(u32, u32)> {
        self.dimensions
    }

    fn data(&self) -> Option<&[u8]> {
        self.image_data.as_deref()
    }
}

fn main() {
    println!("🧪 KTX2 功能测试\n");

    // 测试 1: 生成 KTX2 文件
    println!("📝 测试 1: 生成 KTX2 文件");
    println!("   输入: public/1.png");
    println!("   输出: public/test.ktx2");

    let img = image::open("public/1.png").expect("无法加载 PNG");
    let rgba = img.to_rgba8();

    // 简单的 KTX2 头部创建（从 ktx2_generator.rs 复制）
    let ktx2_data = create_simple_ktx2(img.width(), img.height(), &rgba);

    fs::write("public/test.ktx2", &ktx2_data).expect("无法写入 KTX2");
    println!("   ✅ KTX2 文件已生成\n");

    // 测试 2: 验证 KTX2 文件
    println!("🔍 测试 2: 验证 KTX2 文件");
    match ktx2::Reader::new(&ktx2_data) {
        Ok(reader) => {
            let header = reader.header();
            println!("   ✅ 验证通过!");
            println!("   - 尺寸: {}x{}", header.pixel_width, header.pixel_height);
            println!("   - 层级: {}", header.level_count);
            println!("   - 格式: {:?}\n", header.format);
        }
        Err(e) => {
            println!("   ❌ 验证失败: {:?}\n", e);
            return;
        }
    }

    // 测试 3: 使用 TextureLoader 加载
    println!("📦 测试 3: 使用 TextureLoader 加载");
    let mut loader = TextureLoader::new();

    match loader.load_from_ktx2_bytes(&ktx2_data) {
        Ok(_) => {
            println!("   ✅ 加载成功!");
            if let Some(dims) = loader.dimensions() {
                println!("   - 尺寸: {}x{}", dims.0, dims.1);
            }
            if let Some(data) = loader.data() {
                println!("   - 数据大小: {} 字节", data.len());
                println!("   - 预期大小: {} 字节", rgba.len());
            }
        }
        Err(e) => {
            println!("   ❌ 加载失败: {}", e);
        }
    }

    println!("\n✅ 所有测试完成!");
}

// 简化的 KTX2 生成函数
fn create_simple_ktx2(width: u32, height: u32, rgba_data: &[u8]) -> Vec<u8> {
    use std::mem;

    #[repr(C)]
    struct Ktx2Header {
        identifier: [u8; 12],
        vk_format: u32,
        type_size: u32,
        pixel_width: u32,
        pixel_height: u32,
        pixel_depth: u32,
        array_element_count: u32,
        face_count: u32,
        level_count: u32,
        supercompression_scheme: u32,
        data_format_descriptor: [u32; 8],
        key_value_data: [u32; 2],
        supercompression_global_data: [u32; 2],
    }

    let dfd_offset = mem::size_of::<Ktx2Header>() as u32;
    let level_index_offset = ((dfd_offset + 28 + 3) / 4) * 4;
    let level_data_size = rgba_data.len() as u64;
    let level_data_start = level_index_offset as u64 + 24;

    let mut header = Ktx2Header {
        identifier: [0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, b'\r', b'\n', 0x1A, b'\n'],
        vk_format: 0,
        type_size: 1,
        pixel_width: width,
        pixel_height: height,
        pixel_depth: 0,
        array_element_count: 1,
        face_count: 1,
        level_count: 1,
        supercompression_scheme: 0,
        data_format_descriptor: [0, 0, 0, 0, 0, 0, 0, 0],
        key_value_data: [0, 0],
        supercompression_global_data: [0, 0],
    };

    header.data_format_descriptor[0] = dfd_offset;
    header.data_format_descriptor[1] = 28;

    let mut buffer = Vec::new();

    // Header
    buffer.extend_from_slice(unsafe {
        std::slice::from_raw_parts(
            &header as *const Ktx2Header as *const u8,
            mem::size_of::<Ktx2Header>(),
        )
    });

    // DFD
    let mut dfd = vec![0u8; 28];
    dfd[12..16].copy_from_slice(&28u32.to_le_bytes());
    dfd[16] = 2; // SRGB
    dfd[17] = 1; // BT709
    dfd[20..24].copy_from_slice(&[1, 1, 1, 1]);
    dfd[24] = 4; // 4 bytes per pixel
    buffer.extend_from_slice(&dfd);

    // Alignment
    while buffer.len() % 4 != 0 {
        buffer.push(0);
    }

    // Level Index
    buffer.extend_from_slice(&level_data_start.to_le_bytes());
    buffer.extend_from_slice(&level_data_size.to_le_bytes());
    buffer.extend_from_slice(&level_data_size.to_le_bytes());

    // Alignment
    while buffer.len() % 4 != 0 {
        buffer.push(0);
    }

    // Data
    buffer.extend_from_slice(rgba_data);

    buffer
}
