//! Graphics rendering system
//!
//! Modern GPU-accelerated rendering using wgpu.
//! Replaces the legacy 8-bit palette-based rendering with modern techniques.

pub mod renderer;
pub mod sprite;
pub mod ui;

pub use renderer::Renderer;
pub use sprite::Sprite;
