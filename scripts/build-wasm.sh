#!/bin/bash
set -e

echo "🔨 Building Rust WebAssembly target..."
cargo build --release --target wasm32-unknown-unknown

echo "📦 Generating JavaScript bindings..."
wasm-bindgen --target web --out-dir public \
  target/wasm32-unknown-unknown/release/iced_web_app.wasm

echo "✅ Build complete!"
echo "   📄 Output: public/iced_web_app.js"
echo "   🔧 WASM: public/iced_web_app_bg.wasm"
