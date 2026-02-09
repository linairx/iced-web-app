//! KTX2 文件生成器
//!
//! 从 PNG 文件生成未压缩的 KTX2 文件（RGBA8 格式）
//! 用于测试和开发

use std::env;
use std::fs;
use std::io::Write;

/// KTX2 文件头
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Ktx2Header {
    identifier: [u8; 12],      // «KTX 2»\0\r\n\x1A\n
    vk_format: u32,            // VkFormat
    type_size: u32,            // 类型大小（像素大小，字节数）
    pixel_width: u32,          // 像素宽度
    pixel_height: u32,         // 像素高度
    pixel_depth: u32,          // 像素深度
    array_element_count: u32,  // 数组元素数量
    face_count: u32,           // 面（立方体贴图）数量
    level_count: u32,          // mipmap 层级数量
    supercompression_scheme: u32, // 超级压缩方案
    data_format_descriptor: [u32; 8], // 数据格式描述符偏移/长度
    key_value_data: [u32; 2],  // 键值数据偏移/长度
    supercompression_global_data: [u32; 2], // 超级压缩全局数据偏移/长度
}

/// KTX2 数据格式描述符（针对 RGBA8）
#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct DataFormatDescriptor {
    vendor_id: [u8; 4],        // KHRONOS
    descriptor_type: u32,      // BASIC_FORMAT
    version_number: u32,       // 版本
    descriptor_block_size: u32,
    transfer_function: u8,
    color_primaries: u8,
    flags: u16,
    texel_block_dimensions: [u8; 4], // +x, +y, +z, +w
    bytes_plane: [u8; 8],      // 每个平面的字节数
    // 扩展字段...
}

/// 纹理层级
#[derive(Debug, Clone)]
struct TextureLevel {
    level_index: u32,
    byte_offset: u64,
    byte_length: u64,
    uncompressed_byte_length: u64,
}

/// KTX2 生成器
struct Ktx2Generator {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Ktx2Generator {
    fn new(width: u32, height: u32, rgba_data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data: rgba_data,
        }
    }

    /// 生成 KTX2 文件
    fn generate(&self) -> Vec<u8> {
        // 1. 创建数据格式描述符
        let dfd = self.create_data_format_descriptor();
        let dfd_offset = std::mem::size_of::<Ktx2Header>() as u32;

        // 2. 计算偏移量
        let level_index_offset = ((dfd_offset + 28 + 3) / 4) * 4; // 对齐到 4 字节
        let level_data_size = self.data.len() as u64;
        let level_data_start = level_index_offset as u64 + 24; // +24 是三个 u64 字段

        // 3. 创建头部（包含正确的偏移量）
        let mut header = self.create_header();
        header.data_format_descriptor[0] = dfd_offset;
        header.data_format_descriptor[1] = 28; // DFD 大小

        // 4. 构建整个文件
        let mut buffer = Vec::new();

        // 写入 KTX2 头部
        buffer.extend_from_slice(unsafe {
            std::slice::from_raw_parts(
                &header as *const Ktx2Header as *const u8,
                std::mem::size_of::<Ktx2Header>(),
            )
        });

        // 写入数据格式描述符
        buffer.extend_from_slice(&dfd);

        // 对齐到 4 字节边界
        while buffer.len() % 4 != 0 {
            buffer.push(0);
        }

        // 写入层级索引 (3个 u64: byteOffset, byteLength, uncompressedByteLength)
        buffer.extend_from_slice(&level_data_start.to_le_bytes());
        buffer.extend_from_slice(&level_data_size.to_le_bytes());
        buffer.extend_from_slice(&level_data_size.to_le_bytes());

        // 对齐到 4 字节边界
        while buffer.len() % 4 != 0 {
            buffer.push(0);
        }

        // 写入纹理数据
        buffer.extend_from_slice(&self.data);

        buffer
    }

    fn create_header(&self) -> Ktx2Header {
        Ktx2Header {
            // KTX2 identifier: «KTX 2»\r\n\x1A\n
            // 注意：第 8 个字节是 0xBB 而不是 '\r'
            identifier: [0xAB, b'K', b'T', b'X', b' ', b'2', b'0', 0xBB, b'\r', b'\n', 0x1A, b'\n'],
            vk_format: 0, // VK_FORMAT_UNDEFINED
            type_size: 1,
            pixel_width: self.width,
            pixel_height: self.height,
            pixel_depth: 0,
            array_element_count: 1,
            face_count: 1,
            level_count: 1,
            supercompression_scheme: 0, // 无压缩
            data_format_descriptor: [0, 0, 0, 0, 0, 0, 0, 0],
            key_value_data: [0, 0],
            supercompression_global_data: [0, 0],
        }
    }

    fn create_data_format_descriptor(&self) -> Vec<u8> {
        let mut dfd = vec
![0u8; 28];

        // Vendor ID: KHRONOS
        dfd[0..4].copy_from_slice(&[0x00, 0x00, 0x00, 0x00]);

        // Descriptor type: BASIC_FORMAT
        dfd[4..8].copy_from_slice(&0u32.to_le_bytes());

        // Version number
        dfd[8..12].copy_from_slice(&2u32.to_le_bytes());

        // Descriptor block size
        dfd[12..16].copy_from_slice(&28u32.to_le_bytes());

        // Transfer function: SRGB
        dfd[16] = 2;

        // Color primaries: BT709
        dfd[17] = 1;

        // Flags
        dfd[18..20].copy_from_slice(&0u16.to_le_bytes());

        // Texel block dimensions: 1x1x1x1
        dfd[20] = 1;
        dfd[21] = 1;
        dfd[22] = 1;
        dfd[23] = 1;

        // Bytes plane: 4 bytes per texel (RGBA)
        dfd[24] = 4;
        dfd[25] = 0;
        dfd[26] = 0;
        dfd[27] = 0;

        dfd
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("用法: {} <输入 PNG> <输出 KTX2>", args[0]);
        eprintln!("示例: {} input.png output.ktx2", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let output_path = &args[2];

    println!("📖 读取 PNG 文件: {}", input_path);

    // 使用 image crate 加载 PNG
    let img = image::open(input_path).expect("无法加载 PNG 文件");
    let rgba = img.to_rgba8();

    println!("✅ 图像尺寸: {}x{}", img.width(), img.height());
    println!("✅ 像素数据: {} 字节", rgba.len());

    // 创建 KTX2 生成器
    let generator = Ktx2Generator::new(img.width(), img.height(), rgba.to_vec());

    println!("🔧 生成 KTX2 文件...");

    // 生成 KTX2 数据
    let ktx2_data = generator.generate();

    println!("✅ KTX2 数据大小: {} 字节", ktx2_data.len());

    // 写入文件
    let mut file = fs::File::create(output_path).expect("无法创建输出文件");
    file.write_all(&ktx2_data).expect("无法写入 KTX2 数据");

    println!("💾 KTX2 文件已保存: {}", output_path);

    // 验证
    println!("\n🔍 验证 KTX2 文件...");
    match ktx2::Reader::new(&ktx2_data) {
        Ok(reader) => {
            let header = reader.header();
            println!("✅ 验证通过!");
            println!("   - 格式: {:?}", header.format);
            println!("   - 尺寸: {}x{}", header.pixel_width, header.pixel_height);
            println!("   - 层级: {}", header.level_count);
        }
        Err(e) => {
            eprintln!("❌ 验证失败: {:?}", e);

            // 调试：打印前 80 字节
            println!("\n调试信息 (前 80 字节):");
            for (i, chunk) in ktx2_data[0..80.min(ktx2_data.len())].chunks(16).enumerate() {
                print!("{:04x}: ", i * 16);
                for byte in chunk {
                    print!("{:02x} ", byte);
                }
                println!();
            }
        }
    }
}
