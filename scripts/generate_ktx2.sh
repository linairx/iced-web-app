#!/bin/bash
# KTX2 生成脚本
# 从 PNG 文件生成 KTX2 纹理

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
BINARY="$PROJECT_DIR/target/release/ktx2_generator"

# 检查二进制文件是否存在
if [ ! -f "$BINARY" ]; then
    echo "📦 编译 KTX2 生成器..."
    cd "$PROJECT_DIR"
    cargo build --bin ktx2_generator --release
fi

# 检查参数
if [ $# -lt 2 ]; then
    echo "用法: $0 <输入 PNG> <输出 KTX2>"
    echo ""
    echo "示例:"
    echo "  $0 public/1.png public/1.ktx2"
    echo "  $0 texture.png texture.ktx2"
    echo ""
    echo "提示: 输出文件名建议使用 .ktx2 或 .ktx2 扩展名"
    exit 1
fi

INPUT="$1"
OUTPUT="$2"

# 检查输入文件
if [ ! -f "$INPUT" ]; then
    echo "❌ 错误: 输入文件不存在: $INPUT"
    exit 1
fi

# 运行生成器
echo "🚀 开始生成 KTX2 文件..."
"$BINARY" "$INPUT" "$OUTPUT"

echo ""
echo "✅ 完成! KTX2 文件已保存到: $OUTPUT"
echo ""
echo "💡 提示: 在 iced 应用中加载 KTX2 文件:"
echo "   texture_loader.load_from_ktx2_bytes(&ktx2_data)?;"
