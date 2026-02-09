#!/bin/bash
# KTX Software 安装脚本 (Arch Linux)

set -e

INSTALL_DIR="$HOME/.local"
SOURCE_DIR="/tmp/ktx-software-build"
BUILD_DIR="$SOURCE_DIR/build"

echo "=== KTX Software 安装脚本 ==="
echo ""

# 检查依赖
echo "📋 检查依赖..."
for cmd in git cmake make g++ ninja; do
    if ! command -v $cmd &> /dev/null; then
        echo "❌ 缺少依赖: $cmd"
        echo "请安装: sudo pacman -S base-devel cmake ninja"
        exit 1
    fi
done
echo "✅ 所有依赖已安装"
echo ""

# 清理旧构建
if [ -d "$SOURCE_DIR" ]; then
    echo "🧹 清理旧构建目录..."
    rm -rf "$SOURCE_DIR"
fi

# 克隆仓库
echo "📥 克隆 KTX Software 仓库..."
git clone --depth 1 https://github.com/KhronosGroup/KTX-Software.git "$SOURCE_DIR"
cd "$SOURCE_DIR"
echo "✅ 克隆完成"
echo ""

# 创建构建目录
echo "🔧 配置构建..."
mkdir -p "$BUILD_DIR"
cd "$BUILD_DIR"

# 配置 CMake
cmake .. \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX="$INSTALL_DIR" \
    -G Ninja \
    -DBUILD_TESTS=OFF \
    -DBUILD_EXAMPLES=OFF

echo ""
echo "🔨 编译中..."
ninja

echo ""
echo "📦 安装中..."
ninja install

echo ""
echo "=== 安装完成 ✅ ==="
echo ""
echo "工具已安装到: $INSTALL_DIR/bin"
echo ""
echo "可用工具:"
ls -1 "$INSTALL_DIR/bin"/toktx* 2>/dev/null || echo "未找到 toktx"
ls -1 "$INSTALL_DIR/bin"/ktx* 2>/dev/null || echo "未找到 ktx 工具"
echo ""
echo "=== 添加到 PATH ==="
echo "如果命令未找到，请运行:"
echo "  echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.bashrc"
echo "  source ~/.bashrc"
