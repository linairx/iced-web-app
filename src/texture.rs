//! 纹理处理模块
//!
//! 提供纹理加载、转换和管理的功能
//! 支持 PNG、JPEG 和 KTX2 格式

use iced::widget::image::Handle;
use image::{GenericImageView, ImageFormat};

/// 纹理加载器
pub struct TextureLoader {
    /// 图像数据
    image_data: Option<Vec<u8>>,
    /// 图像尺寸
    dimensions: Option<(u32, u32)>,
}

impl TextureLoader {
    /// 创建新的纹理加载器
    pub fn new() -> Self {
        Self {
            image_data: None,
            dimensions: None,
        }
    }

    /// 从字节数据加载 PNG 图像
    pub fn load_from_png_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        let reader = ImageFormat::Png;
        let img = image::load_from_memory_with_format(bytes, reader)
            .map_err(|e| format!("Failed to load PNG: {}", e))?;

        self.dimensions = Some(img.dimensions());
        self.image_data = Some(img.to_rgba8().to_vec());

        Ok(())
    }

    /// 从字节数据加载 KTX2 纹理
    ///
    /// 注意：此方法使用纯 Rust 的 ktx2 crate，无原生依赖
    /// 适用于 WASM 环境
    pub fn load_from_ktx2_bytes(&mut self, bytes: &[u8]) -> Result<(), String> {
        // 使用纯 Rust 的 ktx2 库解析
        let reader = ktx2::Reader::new(bytes)
            .map_err(|e| format!("Failed to create KTX2 reader: {:?}", e))?;

        // 获取纹理信息
        let header = reader.header();
        let width = header.pixel_width;
        let height = if header.pixel_height > 0 {
            header.pixel_height
        } else {
            1
        };

        // ktx2 crate 的 levels() 可能返回空数据，需要手动解析
        // 从 header 读取 DFD 偏移和长度，然后找到 Level Index

        let dfd_offset = header.data_format_descriptor[0] as usize;
        let dfd_length = header.data_format_descriptor[1] as usize;
        let level_index_offset = dfd_offset + dfd_length;

        // Level Index 包含 3 个 u64 值
        if level_index_offset + 24 > bytes.len() {
            return Err("KTX2 file too short for Level Index".to_string());
        }

        let index_data = &bytes[level_index_offset..level_index_offset + 24];
        let byte_offset = u64::from_le_bytes(index_data[0..8].try_into().unwrap()) as usize;
        let uncompressed_length = u64::from_le_bytes(index_data[16..24].try_into().unwrap()) as usize;

        // 验证偏移
        if byte_offset >= bytes.len() {
            return Err(format!("Invalid KTX2 byteOffset: {}", byte_offset));
        }

        // 读取纹理数据
        let data_start = byte_offset;
        let data_end = byte_offset + uncompressed_length;

        if data_end > bytes.len() {
            return Err("KTX2 data extends beyond file".to_string());
        }

        let texture_data = bytes[data_start..data_end].to_vec();

        // 验证数据大小
        let expected_size = width as usize * height as usize * 4; // RGBA8
        if texture_data.len() != expected_size {
            return Err(format!(
                "KTX2 data size mismatch: expected {} bytes, got {} bytes",
                expected_size,
                texture_data.len()
            ));
        }

        self.dimensions = Some((width, height));
        self.image_data = Some(texture_data);

        Ok(())
    }

    /// 获取 iced 图像句柄
    pub fn as_iced_handle(&self) -> Option<Handle> {
        let data = self.image_data.as_ref()?;
        let dims = self.dimensions?;

        Some(Handle::from_rgba(dims.0, dims.1, data.clone()))
    }

    /// 获取图像尺寸
    pub fn dimensions(&self) -> Option<(u32, u32)> {
        self.dimensions
    }

    /// 获取图像数据
    pub fn data(&self) -> Option<&[u8]> {
        self.image_data.as_deref()
    }
}

impl Default for TextureLoader {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_texture_loader_creation() {
        let loader = TextureLoader::new();
        assert!(loader.image_data.is_none());
        assert!(loader.dimensions.is_none());
    }
}
