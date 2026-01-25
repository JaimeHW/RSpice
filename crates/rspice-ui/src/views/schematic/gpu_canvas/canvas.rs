//! GPU Canvas Component
//!
//! The main Dioxus component for GPU-accelerated schematic rendering.
//! Coordinates event handling, render passes, and GPU state.
//!
//! # Architecture
//!
//! This canvas uses an offscreen rendering approach:
//! 1. Render schematic to wgpu texture
//! 2. Read pixels back and display as image
//! 3. Handle all mouse/keyboard events in Dioxus
//!
//! For desktop-native rendering (bypassing webview), use the
//! `surface.rs` module which provides direct window access.

use dioxus::prelude::*;

use crate::gpu::camera::{Camera, CameraController};
use crate::gpu::integration::GpuSchematicBridge;
use crate::state::render_context::RenderContext;
use crate::state::{Point, Rotation, SchematicState, Tool};
use crate::theme::Theme;

use super::event_handler::{
    EventAction, EventHandlerConfig, Modifiers, MouseButton, MouseEvent as CanvasMouseEvent,
    SchematicEventHandler,
};
use super::render_pass::{OverlayState, RenderPass, RenderStats, ViewportState};

// =============================================================================
// Canvas State
// =============================================================================

/// GPU canvas state - not Clone (contains non-Clone types)
pub struct GpuCanvasState {
    /// Event handler for interaction
    pub event_handler: SchematicEventHandler,
    /// Render pass coordinator
    pub render_pass: RenderPass,
    /// GPU bridge for data sync
    pub bridge: GpuSchematicBridge,
    /// Camera for viewport
    pub camera: Camera,
    /// Camera controller for input
    pub controller: CameraController,
    /// Frame counter
    pub frame_count: u64,
    /// Last render stats
    pub stats: RenderStats,
    /// Whether initial render completed
    pub initialized: bool,
}

impl Default for GpuCanvasState {
    fn default() -> Self {
        Self {
            event_handler: SchematicEventHandler::default(),
            render_pass: RenderPass::new(),
            bridge: GpuSchematicBridge::new(),
            camera: Camera::new(800.0, 600.0, 10.0),
            controller: CameraController::new(),
            frame_count: 0,
            stats: RenderStats::default(),
            initialized: false,
        }
    }
}

// =============================================================================
// Canvas Props
// =============================================================================

/// GPU Canvas component props
#[derive(Props, Clone, PartialEq)]
pub struct GpuSchematicCanvasProps {
    /// Canvas width in pixels
    #[props(default = 800)]
    pub width: u32,

    /// Canvas height in pixels
    #[props(default = 600)]
    pub height: u32,
}

// =============================================================================
// Canvas Component
// =============================================================================

/// GPU-accelerated schematic canvas component
///
/// This component provides the main canvas for the schematic editor,
/// using GPU acceleration for rendering large designs efficiently.
#[component]
pub fn GpuSchematicCanvas(props: GpuSchematicCanvasProps) -> Element {
    // Get shared state from context
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut schematic: Signal<SchematicState> = use_context();

    // Canvas state (use_signal requires the type to be 'static)
    // For now, use simple signals for event handling state
    let mut drag_active = use_signal(|| false);
    let mut box_sel_active = use_signal(|| false);
    let mut box_start = use_signal(|| (0.0f64, 0.0f64));
    let mut box_end = use_signal(|| (0.0f64, 0.0f64));
    let mut pan_active = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0f64, 0.0f64));

    // Viewport signals (mirrored from SchematicState)
    let mut pan = use_signal(|| (0.0f64, 0.0f64));
    let mut zoom = use_signal(|| 1.0f64);

    // Mouse tracking
    let mut mouse_pos = use_signal(|| (0.0f64, 0.0f64));
    let mut mouse_grid = use_signal(|| Point::new(0, 0));

    // Sync viewport from SchematicState
    use_effect(move || {
        let sch = schematic.read();
        pan.set(sch.pan);
        zoom.set(sch.zoom);
    });

    // ==========================================================================
    // Event Handlers
    // ==========================================================================

    let handle_mouse_down = move |evt: dioxus::events::MouseEvent| {
        let button = map_button(&evt);
        let coords = evt.element_coordinates();
        mouse_pos.set((coords.x, coords.y));

        match button {
            MouseButton::Middle => {
                // Start panning
                pan_active.set(true);
                last_mouse.set((coords.x, coords.y));
            }
            MouseButton::Left => {
                // Check what we clicked on
                let (px, py) = *pan.read();
                let z = *zoom.read();
                let gs = schematic.read().grid_size;
                let wx = ((coords.x - px) / z) as f32;
                let wy = ((coords.y - py) / z) as f32;
                let gp = Point::new(
                    ((wx / gs as f32).round() as i32) * gs,
                    ((wy / gs as f32).round() as i32) * gs,
                );
                mouse_grid.set(gp);

                // Tool-specific handling
                let tool = schematic.read().tool;
                match tool {
                    Tool::Select => {
                        // Start box selection in empty area
                        box_sel_active.set(true);
                        box_start.set((coords.x, coords.y));
                        box_end.set((coords.x, coords.y));
                    }
                    Tool::Wire => {
                        let mut sch = schematic.write();
                        if sch.wire_drawing.active {
                            // Add segment at current position
                            sch.update_wire_preview(gp);
                        } else {
                            sch.start_wire(gp);
                        }
                    }
                    Tool::Place(kind) => {
                        let mut sch = schematic.write();
                        sch.add_component(kind, gp);
                    }
                    _ => {}
                }
            }
            MouseButton::Right => {
                // Could show context menu
            }
        }
    };

    let handle_mouse_up = move |evt: dioxus::events::MouseEvent| {
        let button = map_button(&evt);

        if button == MouseButton::Middle {
            pan_active.set(false);
        }

        if button == MouseButton::Left && *box_sel_active.read() {
            // Complete box selection
            box_sel_active.set(false);

            // Calculate selected items
            let (px, py) = *pan.read();
            let z = *zoom.read();
            let gs = schematic.read().grid_size;

            let (sx, sy) = *box_start.read();
            let (ex, ey) = *box_end.read();

            let x1 = ((sx.min(ex) - px) / z) as f32;
            let y1 = ((sy.min(ey) - py) / z) as f32;
            let x2 = ((sx.max(ex) - px) / z) as f32;
            let y2 = ((sy.max(ey) - py) / z) as f32;

            let mut sch = schematic.write();
            sch.selection.components = sch.components
                .iter()
                .filter(|c| {
                    let x = c.pos.x as f32;
                    let y = c.pos.y as f32;
                    x >= x1 && x <= x2 && y >= y1 && y <= y2
                })
                .map(|c| c.id)
                .collect();

            sch.selection.wires = sch.wires
                .iter()
                .filter(|w| {
                    w.points.iter().any(|p| {
                        let x = p.x as f32;
                        let y = p.y as f32;
                        x >= x1 && x <= x2 && y >= y1 && y <= y2
                    })
                })
                .map(|w| w.id)
                .collect();
        }
    };

    let handle_mouse_move = move |evt: dioxus::events::MouseEvent| {
        let coords = evt.element_coordinates();
        mouse_pos.set((coords.x, coords.y));

        // Update grid position
        let (px, py) = *pan.read();
        let z = *zoom.read();
        let gs = schematic.read().grid_size;
        let wx = ((coords.x - px) / z) as f32;
        let wy = ((coords.y - py) / z) as f32;
        let gp = Point::new(
            ((wx / gs as f32).round() as i32) * gs,
            ((wy / gs as f32).round() as i32) * gs,
        );
        mouse_grid.set(gp);

        // Handle panning
        if *pan_active.read() {
            let (lx, ly) = *last_mouse.read();
            let dx = coords.x - lx;
            let dy = coords.y - ly;
            let (opx, opy) = *pan.read();
            pan.set((opx + dx, opy + dy));
            schematic.write().pan = *pan.read();
            last_mouse.set((coords.x, coords.y));
        }

        // Handle box selection
        if *box_sel_active.read() {
            box_end.set((coords.x, coords.y));
        }

        // Update wire preview
        if schematic.read().wire_drawing.active {
            schematic.write().update_wire_preview(gp);
        }
    };

    let handle_wheel = move |evt: dioxus::events::WheelEvent| {
        let delta = match evt.delta() {
            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
            dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 20.0,
            dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
        };

        let (mx, my) = *mouse_pos.read();
        let z = *zoom.read();
        let (px, py) = *pan.read();

        // Zoom centered at mouse position
        let zoom_delta = -delta * 0.001;
        let new_zoom = (z * (1.0 + zoom_delta)).clamp(0.1, 10.0);
        let scale = new_zoom / z;

        let new_px = mx - (mx - px) * scale;
        let new_py = my - (my - py) * scale;

        zoom.set(new_zoom);
        pan.set((new_px, new_py));

        let mut sch = schematic.write();
        sch.zoom = new_zoom;
        sch.pan = (new_px, new_py);
    };

    // ==========================================================================
    // Cursor
    // ==========================================================================

    let cursor = match schematic.read().tool {
        Tool::Probe => "crosshair",
        Tool::Wire => "crosshair",
        Tool::Place(_) => "copy",
        Tool::Label => "text",
        Tool::Select => "default",
    };

    // ==========================================================================
    // Render
    // ==========================================================================

    let bg_color = th.bg_primary();
    let width = props.width;
    let height = props.height;

    // Calculate grid offset for seamless scrolling
    // SVG version uses 20px minor grid, 100px major grid (not schematic.grid_size)
    let (pan_x, pan_y) = *pan.read();
    let z = *zoom.read();
    let minor_grid_px = 20.0 * z;  // Match SVG: 20px minor grid
    let major_grid_px = 100.0 * z; // Match SVG: 100px major grid
    let offset_x = pan_x % minor_grid_px;
    let offset_y = pan_y % minor_grid_px;
    let major_offset_x = pan_x % major_grid_px;
    let major_offset_y = pan_y % major_grid_px;
    // For dots: CSS radial-gradient centers dot in tile, but SVG places at corner (cx=0, cy=0)
    // Shift by half a grid cell to align with SVG
    let dot_offset_x = offset_x - (minor_grid_px / 2.0);
    let dot_offset_y = offset_y - (minor_grid_px / 2.0);

    // Get display settings for grid style
    let display_settings: Signal<crate::state::display_settings::SchematicDisplaySettings> = use_context();
    let grid_style = display_settings.read().grid_style;

    // Theme border color for grid (matching SVG pattern)
    let border_color = th.border();

    rsx! {
        div {
            class: "gpu-canvas-container",
            id: "schematic-canvas-wrapper",
            style: "position: absolute; inset: 0; width: 100%; height: 100%; overflow: hidden; background: {bg_color}; cursor: {cursor};",
            tabindex: "0",

            onmousedown: handle_mouse_down,
            onmouseup: handle_mouse_up,
            onmousemove: handle_mouse_move,
            onwheel: handle_wheel,

            // Grid overlay - matches SVG version styling exactly
            {
                use crate::state::display_settings::GridStyle;
                match grid_style {
                    GridStyle::Lines => {
                        // SVG uses very subtle grid lines - reduce opacity further
                        rsx! {
                            // Minor grid (20px) - very subtle
                            div {
                                class: "grid-minor",
                                style: "position: absolute; inset: 0; pointer-events: none; background-size: {minor_grid_px}px {minor_grid_px}px; background-position: {offset_x}px {offset_y}px; background-image: linear-gradient(to right, rgba(128, 128, 128, 0.08) 1px, transparent 1px), linear-gradient(to bottom, rgba(128, 128, 128, 0.08) 1px, transparent 1px);",
                            }
                            // Major grid (100px) - slightly more visible
                            div {
                                class: "grid-major",
                                style: "position: absolute; inset: 0; pointer-events: none; background-size: {major_grid_px}px {major_grid_px}px; background-position: {major_offset_x}px {major_offset_y}px; background-image: linear-gradient(to right, rgba(128, 128, 128, 0.2) 1px, transparent 1px), linear-gradient(to bottom, rgba(128, 128, 128, 0.2) 1px, transparent 1px);",
                            }
                        }
                    },
                    GridStyle::Dots => {
                        // Dots position adjusted to align with SVG corner placement
                        rsx! {
                            div {
                                class: "grid-dots",
                                style: "position: absolute; inset: 0; pointer-events: none; background-size: {minor_grid_px}px {minor_grid_px}px; background-position: {dot_offset_x}px {dot_offset_y}px; background-image: radial-gradient(circle, rgba(128, 128, 128, 0.4) 1px, transparent 1px);",
                            }
                        }
                    },
                    GridStyle::Hidden => rsx! {},
                }
            }

            // Box selection overlay
            if *box_sel_active.read() {
                {
                    let (sx, sy) = *box_start.read();
                    let (ex, ey) = *box_end.read();
                    let x = sx.min(ex);
                    let y = sy.min(ey);
                    let w = (ex - sx).abs();
                    let h = (ey - sy).abs();
                    rsx! {
                        div {
                            class: "box-selection",
                            style: "position: absolute; left: {x}px; top: {y}px; width: {w}px; height: {h}px; border: 1px dashed #4dabf7; background: rgba(77, 171, 247, 0.1); pointer-events: none;",
                        }
                    }
                }
            }

            // Placeholder for actual GPU-rendered content
            div {
                class: "gpu-render-target",
                style: "position: absolute; inset: 0; pointer-events: none;",
                // GPU-rendered image will be displayed here
            }
        }
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

/// Map Dioxus mouse button to our MouseButton
fn map_button(evt: &dioxus::events::MouseEvent) -> MouseButton {
    // Dioxus MouseButton is in dioxus::html::input_data
    use dioxus::html::input_data::MouseButton as DioxusMouseButton;
    match evt.trigger_button() {
        Some(DioxusMouseButton::Primary) => MouseButton::Left,
        Some(DioxusMouseButton::Auxiliary) => MouseButton::Middle,
        Some(DioxusMouseButton::Secondary) => MouseButton::Right,
        _ => MouseButton::Left,
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // GpuCanvasState Tests
    // =========================================================================

    #[test]
    fn test_canvas_state_default() {
        let state = GpuCanvasState::default();
        assert_eq!(state.frame_count, 0);
        assert!(!state.initialized);
    }

    #[test]
    fn test_zoom_clamping() {
        // Verify zoom clamping logic
        let initial_zoom: f64 = 0.5;
        let delta: f64 = -0.9;
        let new_zoom = (initial_zoom * (1.0 + delta)).clamp(0.1_f64, 10.0_f64);
        assert_eq!(new_zoom, 0.1);
    }

    #[test]
    fn test_zoom_clamping_max() {
        let initial_zoom: f64 = 8.0;
        let delta: f64 = 0.5;
        let new_zoom = (initial_zoom * (1.0 + delta)).clamp(0.1_f64, 10.0_f64);
        assert_eq!(new_zoom, 10.0);
    }

    #[test]
    fn test_grid_snap_calculation() {
        let world_x = 15.3f32;
        let gs = 10;
        let snapped = ((world_x / gs as f32).round() as i32) * gs;
        assert_eq!(snapped, 20);
    }
}
