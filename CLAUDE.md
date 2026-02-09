# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is an Iced Web application written in Rust that compiles to WebAssembly. It demonstrates mouse event handling and texture loading (PNG and KTX2 formats) in a browser environment.

**Key Technology:**
- **Iced 0.14**: Elm-inspired GUI framework with wgpu renderer
- **WebAssembly**: Browser runtime via wasm-bindgen
- **Bun**: JavaScript runtime and package manager for dev tooling
- **Rust 2024 Edition**: Uses latest Rust features

## Development Commands

### Building for Web (WASM)

```bash
# Full build (compile Rust + generate wasm bindings)
bun run build:all

# Individual steps
bun run build:wasm    # Runs scripts/build-wasm.sh
```

The build process:
1. Compiles Rust to wasm32-unknown-unknown target
2. Runs wasm-bindgen to generate JavaScript bindings
3. Outputs to `public/` directory (iced_web_app.js, iced_web_app_bg.wasm)

### Development Server

```bash
bun run dev    # Starts server.ts on port 8080
```

Serves static files from `public/` directory. Access at http://localhost:8080

### Desktop Testing (Native)

```bash
cargo run    # Runs native version (not WASM)
```

### KTX2 Tools

```bash
# Generate KTX2 from PNG
./scripts/generate_ktx2.sh input.png output.ktx2
# Or: cargo build --bin ktx2_generator --release && ./target/release/ktx2_generator input.png output.ktx2

# Test KTX2 functionality
cargo run --bin test_ktx2

# Install KTX Software (for compression support)
./scripts/install_ktx_software.sh
```

## Architecture

### Application Structure (src/main.rs)

The application follows Iced's Elm-inspired architecture:

```
User Input → Message → State Update → View Render
```

**Key Components:**
- `Counter` struct: Holds application state (value, mouse_position, texture_loader)
- `Message` enum: Defines all possible events/actions
- `update()`: Handles messages, returns Task<> for async operations
- `view()`: Renders UI using Iced widgets
- `subscription()`: Listens to system events (mouse, window)

### Platform-Specific Code

**WASM vs Native separation:**

```rust
#[cfg(target_arch = "wasm32")]
{
    // Browser-specific code (fetch API, web-sys)
}

#[cfg(not(target_arch = "wasm32"))]
{
    // Desktop-specific code (filesystem I/O)
}
```

**Texture Loading Pattern:**
- WASM: Uses JavaScript `fetch()` via wasm-bindgen → returns `Task<Message>`
- Native: Uses `std::fs::read()` → wraps in `Task::perform()`

### Texture Module (src/texture.rs)

`TextureLoader` provides unified texture loading for PNG and KTX2 formats:

- `load_from_png_bytes()`: Decodes PNG using image crate
- `load_from_ktx2_bytes()`: Parses KTX2 using ktx2 crate (pure Rust, WASM-compatible)
- `as_iced_handle()`: Converts loaded data to Iced image Handle

**Important Note:** The KTX2 implementation generates **uncompressed RGBA8** files (~18MB for 3412×1362 image). For production, use toktx tool to generate compressed KTX2 (~300-500 KB).

### Binary Tools (src/bin/)

- `ktx2_generator.rs`: Pure Rust KTX2 file generator from PNG
- `test_ktx2.rs`: Verification and testing tool
- `png_to_ktx2.rs`: Alternative PNG→KTX2 converter

## Build Configuration

### Cargo.toml Highlights

- **edition = "2024"**: Uses latest Rust edition
- **Multiple binaries**: Main app + utility tools
- **Conditional dependencies**: web-sys only for WASM target
- **Profile optimization**: Release builds use `opt-level = "z"` for size

### Package.json Scripts

- `dev`: Runs Bun-based dev server (server.ts)
- `build:wasm`: Executes shell script to build WASM
- `build:all`: Convenience wrapper for build:wasm

## WebAssembly Specifics

### Data Flow Between JS and Rust

```rust
// 1. JS fetch returns ArrayBuffer
// 2. Convert to Uint8Array
// 3. Copy to Rust Vec<u8>
// 4. Process in Rust
// 5. Return via Message
```

All async operations in WASM must return `Task<Message>`, which Iced executes asynchronously.

### Memory and Performance

- WASM module: ~4 MB (unoptimized)
- PNG textures: Browser-native decode, slower but small files
- KTX2 textures: Direct GPU upload, faster but larger files (uncompressed)

## Important Constraints

1. **No toktx in repository**: KTX2 compression tool must be installed separately via scripts/install_ktx_software.sh
2. **WASM-only features**: Some Iced features may not work in WASM (check Iced documentation)
3. **Browser compatibility**: Requires modern browser with WebGL2 support
4. **File paths**: WASM builds can only access files in `public/` directory served by dev server

## Git Workflow

This project uses a standard Git workflow. The repository is hosted at: https://github.com/linairx/iced-web-app

When adding new features:
1. Update `Message` enum in main.rs
2. Add handler in `update()` method
3. Update `view()` to display new UI elements
4. For WASM async: create helper function returning `Task<Message>`
5. Test both native (`cargo run`) and web (`bun run build:all && bun run dev`)
