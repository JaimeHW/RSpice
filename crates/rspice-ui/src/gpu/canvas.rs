//! GPU Canvas Component
//!
//! Dioxus component that renders a GPU-accelerated schematic canvas.
//!
//! # Integration Strategy
//!
//! Since Dioxus desktop uses a webview, we use an offscreen rendering approach:
//! 1. Render schematic to a texture using wgpu
//! 2. Read pixels back and encode as PNG
//! 3. Display as data URL image in the component
//! 4. Handle mouse events in Dioxus, forward to camera controller
//!
//! This provides cross-platform compatibility (desktop and web) while
//! maintaining GPU acceleration for the actual rendering.
//!
//! For maximum performance on desktop-only builds, a native window approach
//! can be used instead (requires raw window handle access).

use dioxus::prelude::*;
use std::sync::Arc;

use crate::gpu::camera::{Camera, CameraController};
use crate::gpu::renderer::{ComponentData, JunctionData, SchematicRenderer, WireData};
use crate::state::{Point, SchematicState};

/// State for the GPU canvas
#[derive(Clone)]
pub struct GpuCanvasState {
    /// Current camera position and zoom
    pub camera: Camera,

    /// Camera controller for input handling
    pub controller: CameraController,

    /// Rendered image as data URL (base64 PNG)
    pub image_data: String,

    /// Whether render is needed
    pub needs_render: bool,

    /// Last schematic topology version rendered
    pub last_topology_version: u64,
}

impl Default for GpuCanvasState {
    fn default() -> Self {
        Self {
            camera: Camera::new(800.0, 600.0, 10.0),
            controller: CameraController::new(),
            image_data: String::new(),
            needs_render: true,
            last_topology_version: 0,
        }
    }
}

/// Convert schematic state to GPU-compatible data
pub fn schematic_to_gpu_data(
    schematic: &SchematicState,
) -> (Vec<WireData>, Vec<ComponentData>, Vec<JunctionData>) {
    // Convert wires
    let wires: Vec<WireData> = schematic
        .wires
        .iter()
        .map(|w| WireData {
            points: w.points.iter().map(|p| [p.x as f32, p.y as f32]).collect(),
            selected: schematic.selection.has_wire(w.id),
        })
        .collect();

    // Convert components
    let components: Vec<ComponentData> = schematic
        .components
        .iter()
        .map(|c| ComponentData {
            x: c.pos.x as f32,
            y: c.pos.y as f32,
            rotation: c.rotation.degrees() as f32 * std::f32::consts::PI / 180.0,
            symbol_type: 0, // TODO: map component type to symbol ID
            selected: schematic.selection.has_component(c.id),
        })
        .collect();

    // Convert junctions (simplified - use cached junction points)
    let junctions: Vec<JunctionData> = Vec::new(); // TODO: integrate with RenderContext junction cache

    (wires, components, junctions)
}

/// GPU Canvas component props
#[derive(Props, Clone, PartialEq)]
pub struct GpuCanvasProps {
    /// Width in pixels
    #[props(default = 800)]
    pub width: u32,

    /// Height in pixels
    #[props(default = 600)]
    pub height: u32,

    /// Enable GPU rendering (false = fallback to SVG)
    #[props(default = true)]
    pub enabled: bool,
}

/// GPU-accelerated schematic canvas component
///
/// This component renders the schematic using wgpu for high performance
/// with large designs. Falls back to SVG if GPU is unavailable.
#[component]
pub fn GpuCanvas(props: GpuCanvasProps) -> Element {
    // Canvas state with camera
    let mut canvas_state = use_signal(GpuCanvasState::default);

    // Get schematic from context
    let schematic: Signal<SchematicState> = use_context();

    // Track mouse position for interactions
    let mut mouse_pos = use_signal(|| (0.0f64, 0.0f64));
    let mut is_dragging = use_signal(|| false);

    // Update camera viewport when props change
    use_effect(move || {
        let mut state = canvas_state.write();
        state
            .camera
            .set_viewport(props.width as f32, props.height as f32);
        state.needs_render = true;
    });

    // Check if schematic changed
    use_effect(move || {
        let sch = schematic.read();
        let mut state = canvas_state.write();
        if sch.topology_version() != state.last_topology_version {
            state.needs_render = true;
            state.last_topology_version = sch.topology_version();
        }
    });

    // Initialize GPU renderer and trigger rendering when needed
    #[cfg(not(target_arch = "wasm32"))]
    {
        use_coroutine(
            move |mut _rx: dioxus::prelude::UnboundedReceiver<()>| async move {
                // Initialize renderer once
                let renderer_result = SchematicRenderer::new().await;
                let mut renderer = match renderer_result {
                    Ok(r) => r,
                    Err(e) => {
                        log::error!("Failed to initialize GPU renderer: {:?}", e);
                        return;
                    }
                };

                log::info!("GPU renderer initialized successfully");

                // Initial render
                {
                    let sch = schematic.read();
                    let state = canvas_state.read();
                    if let Ok(data_url) = render_to_data_url(
                        &mut renderer,
                        &sch,
                        &state.camera,
                        props.width,
                        props.height,
                    )
                    .await
                    {
                        drop(state);
                        canvas_state.write().image_data = data_url;
                        canvas_state.write().needs_render = false;
                    }
                }

                // Render loop - check periodically for changes
                // Use 50ms (~20fps) to reduce PNG encoding overhead during pan/zoom
                loop {
                    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

                    let needs_render = canvas_state.read().needs_render;
                    if needs_render {
                        let sch = schematic.read();
                        let state = canvas_state.read();
                        if let Ok(data_url) = render_to_data_url(
                            &mut renderer,
                            &sch,
                            &state.camera,
                            props.width,
                            props.height,
                        )
                        .await
                        {
                            drop(state);
                            canvas_state.write().image_data = data_url;
                            canvas_state.write().needs_render = false;
                        }
                    }
                }
            },
        );
    }

    // Mouse event handlers
    let onmousedown = move |evt: MouseEvent| {
        let coords = evt.data().coordinates().client();
        let (x, y) = (coords.x, coords.y);
        mouse_pos.set((x, y));
        is_dragging.set(true);

        let mut state = canvas_state.write();
        state.controller.start_pan(x as f32, y as f32);
    };

    let onmousemove = move |evt: MouseEvent| {
        let coords = evt.data().coordinates().client();
        let (x, y) = (coords.x, coords.y);
        mouse_pos.set((x, y));

        if *is_dragging.read() {
            let mut state = canvas_state.write();
            // Extract camera, modify it, put it back to avoid double mutable borrow
            let mut camera = state.camera.clone();
            state.controller.update_pan(&mut camera, x as f32, y as f32);
            state.camera = camera;
            state.needs_render = true;
        }
    };

    let onmouseup = move |_evt: MouseEvent| {
        is_dragging.set(false);
        canvas_state.write().controller.end_pan();
    };

    let onwheel = move |evt: WheelEvent| {
        let (mx, my) = *mouse_pos.read();
        let delta = evt.delta();
        let delta_y = match delta {
            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
            dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 20.0,
            dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
        };
        let normalized_delta = -delta_y as f32 / 100.0; // Normalize wheel delta

        let mut state = canvas_state.write();
        // Extract camera, modify it, put it back to avoid double mutable borrow
        let mut camera = state.camera.clone();
        state
            .controller
            .zoom_at(&mut camera, mx as f32, my as f32, normalized_delta);
        state.camera = camera;
        state.needs_render = true;
    };

    // Render placeholder div (actual rendering happens in use_effect or async task)
    rsx! {
        div {
            class: "gpu-canvas",
            style: "width: {props.width}px; height: {props.height}px; background: #1a1a1a; cursor: grab; overflow: hidden;",
            onmousedown: onmousedown,
            onmousemove: onmousemove,
            onmouseup: onmouseup,
            onwheel: onwheel,

            // Display rendered image or loading state
            if canvas_state.read().image_data.is_empty() {
                div {
                    class: "gpu-loading",
                    style: "display: flex; align-items: center; justify-content: center; height: 100%; color: #888;",
                    "Initializing GPU renderer..."
                }
            } else {
                img {
                    src: "{canvas_state.read().image_data}",
                    style: "width: 100%; height: 100%; image-rendering: pixelated;",
                    draggable: "false",
                }
            }
        }
    }
}

/// Render schematic to PNG data URL (called from async task or render loop)
#[cfg(not(target_arch = "wasm32"))]
pub async fn render_to_data_url(
    renderer: &mut SchematicRenderer,
    schematic: &SchematicState,
    camera: &Camera,
    width: u32,
    height: u32,
) -> Result<String, String> {
    // Convert schematic data
    let (wires, components, junctions) = schematic_to_gpu_data(schematic);

    // Update renderer
    renderer.update_camera(camera);
    renderer.update_grid(camera);
    renderer.update_wires(&wires);
    renderer.update_components(&components);
    renderer.update_junctions(&junctions);

    // Render to texture and encode as PNG
    let pixels = renderer
        .render_to_image(width, height)
        .map_err(|e| format!("Render failed: {:?}", e))?;

    if pixels.is_empty() {
        return Ok(String::new());
    }

    // Encode as PNG
    let mut png_data = Vec::new();
    {
        let mut encoder = png::Encoder::new(&mut png_data, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|e| e.to_string())?;
        writer
            .write_image_data(&pixels)
            .map_err(|e| e.to_string())?;
    }

    // Encode as base64 data URL
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &png_data);
    Ok(format!("data:image/png;base64,{}", b64))
}

#[cfg(target_arch = "wasm32")]
pub async fn render_to_data_url(
    _renderer: &mut SchematicRenderer,
    _schematic: &SchematicState,
    _camera: &Camera,
    _width: u32,
    _height: u32,
) -> Result<String, String> {
    // WebGPU implementation would go here
    Err("WebGPU rendering not yet implemented".to_string())
}
