//! GPU Schematic Renderer
//!
//! Commercial-grade GPU-accelerated schematic rendering using wgpu.
//! This module provides hardware-accelerated rendering for large schematics,
//! following patterns used by professional EDA tools like Cadence and Altium.
//!
//! # Architecture
//!
//! The renderer is designed for maximum performance with large schematics:
//! - **Instance Rendering**: 10,000+ components rendered in ~4 draw calls
//! - **GPU-side Transform**: Camera/zoom handled entirely in vertex shader
//! - **Batched Updates**: Only changed geometry uploaded to GPU
//! - **Layer-based Rendering**: Background → Wires → Components → Labels → Selection
//!
//! # Integration
//!
//! This module integrates with the existing Dioxus UI:
//! - Dioxus handles menus, toolbars, dialogs, property panels
//! - GPU canvas handles schematic visualization
//! - State is shared via Dioxus signals
//!
//! # Usage
//!
//! ```ignore
//! // Create renderer for a window
//! let mut canvas = ManagedGpuCanvas::new(&window, 800, 600).await?;
//!
//! // Sync schematic state to GPU cache
//! gpu_cache.synchronize(&schematic, &render_ctx);
//!
//! // Render frame
//! canvas.render()?;
//! ```

pub mod buffers;
pub mod camera;
pub mod canvas;
pub mod context;
pub mod geometry;
pub mod gpu_cache;
pub mod hit_test;
pub mod integration;
pub mod pipeline;
pub mod renderer;
pub mod shaders;
pub mod surface;
pub mod text;
pub mod vertex;
pub mod webgpu_runtime;

// Re-exports for convenient access
pub use camera::{Camera, CameraController};
pub use context::GpuContext;
pub use gpu_cache::{CacheStats, DirtyFlags, GpuRenderCache};
pub use hit_test::{BoundingBox, HitResult, HitTestConfig, HitTester};
pub use integration::GpuSchematicBridge;
pub use renderer::SchematicRenderer;
pub use surface::{GpuSurface, ManagedGpuCanvas};
pub use text::{GlyphAtlas, LabelData, TextAlign, TextInstance, TextLayout};
pub use webgpu_runtime::WebGpuRuntime;
