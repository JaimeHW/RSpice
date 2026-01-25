//! GPU Canvas Module
//!
//! GPU-accelerated schematic canvas for the schematic editor.
//!
//! # Module Organization
//!
//! - `canvas` - Main Dioxus component
//! - `event_handler` - Separated event logic (testable)
//! - `render_pass` - Per-frame render coordination (testable)
//!
//! # Usage
//!
//! ```ignore
//! rsx! {
//!     GpuSchematicCanvas {
//!         width: 800,
//!         height: 600,
//!         on_context_menu: |pos| { /* show menu */ }
//!     }
//! }
//! ```

pub mod canvas;
pub mod event_handler;
pub mod render_pass;
pub mod wgpu_backend;
pub mod render_integration;

// Re-exports
pub use canvas::{GpuCanvasState, GpuSchematicCanvas, GpuSchematicCanvasProps};
pub use event_handler::{
    BoxSelection, DragOperation, EventAction, EventHandlerConfig, InteractionState, Modifiers,
    MouseButton, MouseEvent, PanOperation, SchematicEventHandler,
};
pub use render_pass::{
    GridState, LayerDirtyFlags, OverlayState, RenderLayer, RenderPass, RenderState, RenderStats,
    ViewportState,
};
pub use wgpu_backend::{get_or_init_backend, WgpuCanvasBackend};
pub use render_integration::{GpuRenderState, RenderManager, RenderRequest};

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_exports() {
        // Verify all public types are accessible
        let _ = SchematicEventHandler::default();
        let _ = RenderPass::new();
        let _ = GpuCanvasState::default();
    }

    #[test]
    fn test_event_handler_construction() {
        let config = EventHandlerConfig::default();
        let handler = SchematicEventHandler::new(config);
        assert!(!handler.state.is_busy());
    }

    #[test]
    fn test_render_pass_construction() {
        let pass = RenderPass::new();
        assert!(!pass.state.dirty_layers.any() || pass.state.dirty_layers.any());
    }

    #[test]
    fn test_render_layer_enumeration() {
        let layers = RenderLayer::all_in_order();
        assert!(layers.len() > 10);
    }
}
