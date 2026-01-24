//! Schematic Editor View - Pure SVG Implementation
//!
//! Interactive schematic capture using SVG for all rendering.
//! Supports pan, zoom, drag-to-move, undo/redo, and context menus.
//!
//! This is the main orchestrating component that delegates rendering to:
//! - `svg::WireSvg`, `svg::CompSvg`, etc. for schematic element rendering
//! - `types` module for drag/selection state types
//! - `utils` module for geometry calculations

use dioxus::prelude::*;

use crate::app::WaveformVisible;
use crate::components::annotation_overlay::AnnotationOverlay;
use crate::components::context_menu::{
    canvas_context_menu, schematic_context_menu, ContextMenu, MenuAction,
};
use crate::components::file_handlers;
use crate::components::tab_bar::DocumentTabBar;
use crate::dialogs::ComponentEditModal;
use crate::dialogs::{SaveDialogResult, UnsavedChangesModal};
use crate::services::cross_probing::CrossProbeManager;
use crate::state::dc_annotation::{Annotation, AnnotationMode};
use crate::state::{
    CanvasFocusState, ComponentType, ConsoleMessage, DocumentManager, Point, Rotation,
    SchematicHistory, SchematicState, SimulationState, Tool, Viewport,
};
use crate::theme::Theme;
use crate::views::symbol_assets;

// Import extracted types from sibling modules
use super::annotation::DcAnnotationLayer;
use super::svg::{
    rotate_point_by_deg, symbol_path, CompSvg, NetLabelSvg, PreviewSvg, WirePreviewSvg, WireSvg,
};
use super::types::{BoxSelectionState, ContextMenuState, DragState, EditingState, LabelDragState};
use super::utils::point_to_segment_dist;

/// Main schematic editor component - Pure SVG
#[component]
pub fn Schematic() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut schematic: Signal<SchematicState> = use_context();
    let mut sim_state: Signal<SimulationState> = use_context();
    let mut waveform_visible: Signal<WaveformVisible> = use_context();
    let mut doc_manager: Signal<DocumentManager> = use_context();
    let cross_probe: Signal<CrossProbeManager> = use_context();

    // Viewport state - pan/zoom stored in SchematicState for per-document persistence
    // Local signals mirror SchematicState and sync back on changes
    let mut pan = use_signal(|| (0.0f64, 0.0f64));
    let mut zoom = use_signal(|| 1.0f64);
    let mut is_panning = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0f64, 0.0f64));
    let mut mouse_grid = use_signal(|| Point::new(0, 0));

    // Canvas size for zoom_to_fit (updated when canvas is mounted/resized)
    let mut canvas_size = use_signal(|| (800.0f64, 600.0f64));

    // Sync pan/zoom from SchematicState when it changes (e.g., tab switch, file open)
    // Use effect to properly react to schematic signal changes
    use_effect(move || {
        let sch = schematic.read();
        pan.set(sch.pan);
        zoom.set(sch.zoom);
    });

    // Drag-to-move state
    let mut drag = use_signal(DragState::default);

    // Highlighted junction point (persists after drag for selection highlighting)
    let mut highlighted_junction: Signal<Option<Point>> = use_signal(|| None);

    // Context menu state
    let mut context_menu = use_signal(ContextMenuState::default);

    // Box selection state (rubber-band selection)
    let mut box_selection = use_signal(BoxSelectionState::default);

    // Close confirmation dialog state
    let mut close_confirm: Signal<Option<(usize, String)>> = use_signal(|| None);

    // Probe tool: wire IDs of the hovered net (entire electrically connected net)
    let mut probe_hover_wires: Signal<std::collections::HashSet<u64>> =
        use_signal(std::collections::HashSet::new);

    // Wire corner hover state - shows visual feedback when cursor is near a draggable wire endpoint
    let mut hovered_corner: Signal<Option<Point>> = use_signal(|| None);

    // Global label drag state - shared with CompSvg via context for proper pointer capture
    // This allows a single global overlay at the Schematic level to capture events
    let mut label_drag: Signal<LabelDragState> = use_signal(LabelDragState::default);
    use_context_provider(|| label_drag);

    // Also provide component drag state so labels can disable pointer-events during component drag
    use_context_provider(|| drag);

    // Undo/Redo history (local for now, will integrate with SchematicHistory later)
    let mut history = use_signal(|| SchematicHistory::new(schematic.read().clone(), 100));

    // Helper closure that pushes to history AND marks document dirty
    // This encapsulates the dirty tracking logic in one place
    let mut push_edit = move |state: SchematicState, desc: &str| {
        history.write().push(state, desc);
        doc_manager.write().active_mut().mark_dirty();
    };

    // Component editing state
    let mut editing = use_signal(EditingState::default);

    // Canvas focus context - shared with other components for focus management
    let mut canvas_focus = use_signal(CanvasFocusState::new);
    use_context_provider(|| canvas_focus);

    // Handle needs_fit flag - perform zoom_to_fit with viewport dimensions
    // Read the signal first to create a reactive dependency, then check the flag
    {
        let needs_fit = schematic.read().needs_fit;
        if needs_fit {
            // Clear the flag first to prevent infinite loops
            schematic.write().needs_fit = false;

            // Get fresh canvas dimensions at this moment (not stale cached values)
            // Use the mounted element stored in canvas_focus to get current rect
            let element_opt = {
                let focus = canvas_focus.read();
                focus.get_element()
            };
            if let Some(element) = element_opt {
                spawn(async move {
                    if let Ok(rect) = element.get_client_rect().await {
                        let w = rect.width();
                        let h = rect.height();
                        if w > 0.0 && h > 0.0 {
                            schematic.write().zoom_to_fit(w, h);
                        }
                    }
                });
            }
        }
    }

    // Handle needs_history_reset flag - reset undo history after loading a file
    // This ensures undo doesn't revert to a previous/empty schematic
    {
        let needs_reset = schematic.read().needs_history_reset;
        if needs_reset {
            // Clear the flag first
            schematic.write().needs_history_reset = false;
            // Reset history with the loaded schematic as the new base state
            history.set(SchematicHistory::new(schematic.read().clone(), 100));
        }
    }

    // Global keyboard shortcut handler - works regardless of which panel has focus
    // Simulators use window-level shortcuts for consistent behavior
    use_effect(move || {
        // Set up window-level keydown listener via JavaScript
        // This ensures hotkeys work even when focus is on console or properties panel
        let _ = document::eval(
            r#"
            if (!window.__rspice_keydown_handler) {
                window.__rspice_keydown_handler = function(e) {
                    var canvas = document.getElementById('schematic-canvas-wrapper');
                    if (!canvas) return;
                    
                    // Only refocus if not in an input field
                    var tag = document.activeElement ? document.activeElement.tagName : '';
                    if (tag === 'INPUT' || tag === 'TEXTAREA') return;
                    
                    // If canvas doesn't have focus, focus it and re-dispatch the event
                    if (document.activeElement !== canvas) {
                        canvas.focus();
                        // Dispatch a new keyboard event to the canvas so first keypress works
                        var newEvent = new KeyboardEvent('keydown', {
                            key: e.key,
                            code: e.code,
                            keyCode: e.keyCode,
                            ctrlKey: e.ctrlKey,
                            shiftKey: e.shiftKey,
                            altKey: e.altKey,
                            metaKey: e.metaKey,
                            bubbles: true
                        });
                        canvas.dispatchEvent(newEvent);
                        e.stopPropagation();
                        e.preventDefault();
                    }
                };
                window.addEventListener('keydown', window.__rspice_keydown_handler, true);
            }
        "#,
        );
    });

    rsx! {
        div {
            class: "schematic-container",
            style: "display: flex; flex-direction: column; width: 100%; height: 100%; overflow: hidden;",

            // Main content area with library sidebar and canvas
            div {
                style: "display: flex; flex: 1 1 auto; overflow: hidden;",

                // Canvas container (tabs + canvas) - takes remaining width
                div {
                    style: "display: flex; flex-direction: column; flex: 1 1 auto; overflow: hidden;",

                    // Document tabs - only above canvas, not library
                    {
                        let mut doc_manager: Signal<DocumentManager> = use_context();
                        let mut sim_state_ctx: Signal<SimulationState> = use_context();

                        rsx! {
                            DocumentTabBar {
                                doc_manager: doc_manager,
                                on_tab_change: move |idx| {
                                    // Save current state to the currently active document before switching
                                    {
                                        let mut docs = doc_manager.write();
                                        docs.active_mut().schematic = schematic.read().clone();
                                        docs.active_mut().simulation = sim_state_ctx.read().clone();
                                        docs.set_active(idx);
                                    }
                                    // Load state from the newly active document
                                    let docs = doc_manager.read();
                                    schematic.set(docs.active().schematic.clone());
                                    sim_state_ctx.set(docs.active().simulation.clone());
                                },
                                on_tab_close: move |idx| {
                                    // Use captured signals from outer scope (doc_manager, schematic, sim_state_ctx)
                                    // IMPORTANT: Never call use_context() inside callbacks - hooks must only be at component top level

                                    let docs = doc_manager.read();
                                    let idx = idx as usize;
                                    let is_dirty = docs.documents[idx].is_dirty;
                                    let doc_name = docs.documents[idx].name.clone();
                                    drop(docs);

                                    if is_dirty {
                                        // Show custom confirmation dialog
                                        close_confirm.set(Some((idx, doc_name)));
                                    } else {
                                        // No unsaved changes - close immediately
                                        let was_active = doc_manager.read().active_index == idx;
                                        doc_manager.write().close_document(idx);
                                        if was_active {
                                            let docs = doc_manager.read();
                                            schematic.set(docs.active().schematic.clone());
                                            sim_state_ctx.set(docs.active().simulation.clone());
                                        }
                                    }
                                },
                                on_new_document: move |_| {
                                    // Save current state before creating new document
                                    {
                                        let mut docs = doc_manager.write();
                                        docs.active_mut().schematic = schematic.read().clone();
                                        docs.active_mut().simulation = sim_state_ctx.read().clone();
                                        docs.new_document();
                                    }
                                    // Load empty state for the new document
                                    let docs = doc_manager.read();
                                    schematic.set(docs.active().schematic.clone());
                                    sim_state_ctx.set(docs.active().simulation.clone());
                                },
                            }
                        }
                    }

                    // Canvas area - wrapper div is focusable for keyboard events
                    div {
                        id: "schematic-canvas-wrapper",
                        class: "schematic-canvas-wrapper",
                        style: "flex: 1 1 auto; position: relative; overflow: hidden; outline: none;",
                        tabindex: "0",
                    onmounted: move |evt| {
                        canvas_focus.write().set_element(evt.data());
                        // Auto-focus so keyboard shortcuts work immediately on startup
                        canvas_focus.read().focus();

                        // Initial canvas dimensions capture
                        let mounted = evt.data();
                        spawn(async move {
                            if let Ok(rect) = mounted.get_client_rect().await {
                                canvas_size.set((rect.width(), rect.height()));
                            }
                        });

                        // Set up ResizeObserver via JavaScript for continuous size tracking
                        // This is essential for proper viewport culling - without it, wires
                        // disappear during pan/zoom because viewport bounds use stale dimensions
                        let _ = document::eval(r#"
                            (function() {
                                var wrapper = document.getElementById('schematic-canvas-wrapper');
                                if (!wrapper) return;
                                
                                // Store last known size to avoid redundant updates
                                window.__rspice_canvas_size = { w: 0, h: 0 };
                                
                                // ResizeObserver for instant resize detection
                                if (!window.__rspice_resize_observer) {
                                    window.__rspice_resize_observer = new ResizeObserver(function(entries) {
                                        for (var entry of entries) {
                                            var rect = entry.contentRect;
                                            if (rect.width > 0 && rect.height > 0) {
                                                window.__rspice_canvas_size.w = rect.width;
                                                window.__rspice_canvas_size.h = rect.height;
                                            }
                                        }
                                    });
                                    window.__rspice_resize_observer.observe(wrapper);
                                }
                            })();
                        "#);

                        // Periodic polling to sync JS resize state with Rust signal
                        // This is a fallback mechanism since we can't directly call into Dioxus from JS
                        let mounted_for_poll = evt.data();
                        spawn(async move {
                            loop {
                                // Poll every 200ms for size changes
                                // Use platform-specific sleep
                                #[cfg(target_arch = "wasm32")]
                                {
                                    gloo_timers::future::TimeoutFuture::new(200).await;
                                }
                                #[cfg(not(target_arch = "wasm32"))]
                                {
                                    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
                                }
                                if let Ok(rect) = mounted_for_poll.get_client_rect().await {
                                    let w = rect.width();
                                    let h = rect.height();
                                    let (old_w, old_h) = *canvas_size.read();
                                    // Only update if size actually changed (avoids unnecessary re-renders)
                                    if (w - old_w).abs() > 1.0 || (h - old_h).abs() > 1.0 {
                                        canvas_size.set((w, h));
                                    }
                                }
                            }
                        });
                    },
                    // Re-focus when clicking on the canvas to restore keyboard shortcuts
                    onfocus: move |_| {
                        // Focus is being gained, no extra action needed
                    },
                    // Ensure clicks bring focus back to the canvas
                    onclick: move |_| {
                        let _ = document::eval(r#"document.getElementById('schematic-canvas-wrapper').focus()"#);
                    },
                    onkeydown: move |evt| {
                        // Handle Ctrl+key shortcuts
                        if evt.modifiers().ctrl() {
                            match evt.key() {
                                Key::Character(c) if c == "z" || c == "Z" => {
                                    if history.read().can_undo() {
                                        // Preserve current pan/zoom (not part of undo history)
                                        let current_pan = schematic.read().pan;
                                        let current_zoom = schematic.read().zoom;
                                        history.write().undo();
                                        let mut restored = history.read().current().clone();
                                        restored.pan = current_pan;
                                        restored.zoom = current_zoom;
                                        schematic.set(restored);
                                    }
                                    return;
                                }
                                Key::Character(c) if c == "y" || c == "Y" => {
                                    if history.read().can_redo() {
                                        // Preserve current pan/zoom (not part of undo history)
                                        let current_pan = schematic.read().pan;
                                        let current_zoom = schematic.read().zoom;
                                        history.write().redo();
                                        let mut restored = history.read().current().clone();
                                        restored.pan = current_pan;
                                        restored.zoom = current_zoom;
                                        schematic.set(restored);
                                    }
                                    return;
                                }
                                Key::Character(c) if c == "c" || c == "C" => {
                                    schematic.write().copy_selection();
                                    return;
                                }
                                Key::Character(c) if c == "v" || c == "V" => {
                                    let gp = *mouse_grid.read();
                                    schematic.write().paste_at(gp);
                                    push_edit(schematic.read().clone(), "Paste");
                                    return;
                                }
                                _ => {}
                            }
                        }

                        let mut s = schematic.write();
                        match evt.key() {
                            Key::Escape => { s.tool = Tool::Select; s.cancel_wire(); }
                            Key::Delete => {
                                s.delete_selection();
                                drop(s);
                                push_edit(schematic.read().clone(), "Delete selection");
                            }
                            Key::Character(c) if c == "r" || c == "R" => {
                                if matches!(s.tool, Tool::Place(_)) {
                                    s.preview_rotation = s.preview_rotation.rotate_cw();
                                } else {
                                    s.rotate_selection();
                                    drop(s);
                                    push_edit(schematic.read().clone(), "Rotate selection");
                                }
                            }
                            Key::Character(c) if c == "w" || c == "W" => {
                                s.tool = Tool::Wire;
                                s.selection.clear();
                                highlighted_junction.set(None);
                            }
                            Key::Character(c) if c == "s" || c == "S" => {
                                s.cancel_wire();
                                s.tool = Tool::Select;
                                s.selection.clear();
                                highlighted_junction.set(None);
                            }
                            Key::Character(c) if c == " " => {
                                if s.wire_drawing.active {
                                    s.toggle_wire_routing();
                                }
                            }
                            Key::Character(c) if c == "g" || c == "G" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::Ground);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "v" || c == "V" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::VoltageSource);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "i" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::CurrentSource);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "c" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::Capacitor);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "l" || c == "L" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::Inductor);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "d" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::Diode);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "q" || c == "Q" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::NpnBjt);
                                s.preview_rotation = Rotation::R0;
                            }
                            Key::Character(c) if c == "m" => {
                                s.cancel_wire();
                                s.selection.clear();
                                s.tool = Tool::Place(ComponentType::Nmos);
                                s.preview_rotation = Rotation::R0;
                            }
                            _ => {}
                        }
                    },

                // Pure SVG canvas
                {
                    let cursor = match schematic.read().tool {
                        Tool::Probe => "crosshair",
                        Tool::Wire => "crosshair",
                        Tool::Place(_) => "copy",
                        Tool::Label => "text",
                        Tool::Select => "default",
                    };
                    rsx! {
                        svg {
                            id: "schematic-canvas",
                            style: "position: absolute; inset: 0; width: 100%; height: 100%; background: {th.bg_primary()}; cursor: {cursor};",

                    onmousemove: move |evt| {
                        let c = evt.element_coordinates();

                        // Calculate grid position
                        let (px, py) = *pan.read();
                        let z = *zoom.read();
                        let wx = (c.x - px) / z;
                        let wy = (c.y - py) / z;
                        let gs = schematic.read().grid_size;
                        let gp = Point::from_pixels(wx, wy, gs);
                        mouse_grid.set(gp);

                        // Handle panning
                        if *is_panning.read() {
                            let (lx, ly) = *last_mouse.read();
                            let (opx, opy) = *pan.read();
                            pan.set((opx + c.x - lx, opy + c.y - ly));
                            // Sync to SchematicState for per-document persistence
                            schematic.write().pan = *pan.read();
                        }

                        // Handle component, wire, or junction dragging
                        let d = *drag.read();
                        if d.active && (d.component_id.is_some() || d.wire_id.is_some() || d.junction_point.is_some()) {
                            drag.set(DragState {
                                current_grid: gp,
                                ..d
                            });
                        }

                        // Handle label dragging (also handled here because overlay pointer-events
                        // updates asynchronously and may miss initial move events)
                        if label_drag.read().active {
                            let (start_x, start_y) = label_drag.read().start_offset;
                            let coords = evt.page_coordinates();
                            let z_current = *zoom.read();
                            let dx = (coords.x - start_x) / z_current;
                            let dy = (coords.y - start_y) / z_current;
                            label_drag.write().current_offset = (dx, dy);
                        }

                        // Handle box selection update with live preview
                        let bs = *box_selection.read();
                        if bs.active {
                            box_selection.set(BoxSelectionState {
                                end_grid: gp,
                                end_px: (c.x, c.y),
                                ..bs
                            });

                            // Update selection in real-time for visual feedback
                            let min_x = bs.start_grid.x.min(gp.x);
                            let max_x = bs.start_grid.x.max(gp.x);
                            let min_y = bs.start_grid.y.min(gp.y);
                            let max_y = bs.start_grid.y.max(gp.y);

                            // Collect IDs of items in box
                            let comp_ids: Vec<u64> = schematic.read()
                                .components.iter()
                                .filter(|comp| {
                                    comp.pos.x >= min_x && comp.pos.x <= max_x
                                    && comp.pos.y >= min_y && comp.pos.y <= max_y
                                })
                                .map(|comp| comp.id)
                                .collect();

                            let wire_ids: Vec<u64> = schematic.read()
                                .wires.iter()
                                .filter(|wire| {
                                    wire.points.iter().any(|p| {
                                        p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
                                    })
                                })
                                .map(|wire| wire.id)
                                .collect();

                            // Update selection for live preview
                            let mut s = schematic.write();
                            s.selection.components = comp_ids;
                            s.selection.wires = wire_ids;
                        }

                        // Update wire preview position for orthogonal routing
                        if schematic.read().wire_drawing.active {
                            schematic.write().update_wire_preview(gp);
                        }

                        // Select tool: detect wire endpoints/corners on hover for drag feedback
                        if matches!(schematic.read().tool, Tool::Select) && !d.active && !bs.active {
                            let s = schematic.read();
                            // Check if cursor is on any wire endpoint
                            let mut found_corner: Option<Point> = None;
                            'corner_search: for wire in &s.wires {
                                for pt in &wire.points {
                                    if *pt == gp {
                                        found_corner = Some(*pt);
                                        break 'corner_search;
                                    }
                                }
                            }
                            hovered_corner.set(found_corner);
                        } else if d.active || bs.active {
                            hovered_corner.set(None);
                        }

                        // Probe tool: detect wire under cursor and highlight entire connected net
                        if matches!(schematic.read().tool, Tool::Probe) {
                            let s = schematic.read();

                            // Check if cursor is near any wire segment (not just endpoints)
                            let mut hit_wire_id: Option<u64> = None;
                            'wire_loop: for wire in &s.wires {
                                for i in 0..wire.points.len().saturating_sub(1) {
                                    let p1 = wire.points[i];
                                    let p2 = wire.points[i + 1];
                                    // Point-to-segment distance check (in grid units)
                                    let dist = point_to_segment_dist(gp, p1, p2);
                                    if dist <= 1.5 {
                                        hit_wire_id = Some(wire.id);
                                        break 'wire_loop;
                                    }
                                }
                            }

                            if let Some(wire_id) = hit_wire_id {
                                // Find all wires connected to this net using flood-fill
                                let mut connected: std::collections::HashSet<u64> = std::collections::HashSet::new();
                                let mut to_visit = vec![wire_id];

                                while let Some(wid) = to_visit.pop() {
                                    if connected.contains(&wid) { continue; }
                                    connected.insert(wid);

                                    // Find this wire's endpoints
                                    if let Some(wire) = s.wires.iter().find(|w| w.id == wid) {
                                        for endpoint in [wire.points.first(), wire.points.last()].into_iter().flatten() {
                                            // Find other wires sharing this endpoint
                                            for other in &s.wires {
                                                if connected.contains(&other.id) { continue; }
                                                if other.points.contains(endpoint) {
                                                    to_visit.push(other.id);
                                                }
                                            }
                                        }
                                    }
                                }
                                probe_hover_wires.set(connected);
                            } else {
                                if !probe_hover_wires.read().is_empty() {
                                    probe_hover_wires.set(std::collections::HashSet::new());
                                }
                            }
                        } else {
                            // Clear highlight when not in probe mode
                            if !probe_hover_wires.read().is_empty() {
                                probe_hover_wires.set(std::collections::HashSet::new());
                            }
                        }

                        last_mouse.set((c.x, c.y));
                    },

                    onmousedown: move |evt| {
                        // Restore keyboard focus synchronously using JavaScript
                        // The async spawn approach wasn't reliable
                        let _ = document::eval(r#"document.getElementById('schematic-canvas-wrapper').focus()"#);

                        let c = evt.element_coordinates();
                        let (px, py) = *pan.read();
                        let z = *zoom.read();
                        let gs = schematic.read().grid_size;
                        let gp = Point::from_pixels((c.x - px) / z, (c.y - py) / z, gs);

                        // Middle mouse or shift+left for pan
                        if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Auxiliary)
                           || (evt.modifiers().shift() && evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary)) {
                            is_panning.set(true);
                            last_mouse.set((c.x, c.y));
                        }
                        // Left click on component, junction, endpoint, or wire to start drag (in Select mode)
                        else if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
                            // First, collect data we need with an immutable borrow
                            // Use wire_points_at to detect ALL points (including corners for dragging)
                            let (tool_is_select, points_at_pos, comp_at, wire_at) = {
                                let s = schematic.read();
                                (
                                    matches!(s.tool, Tool::Select),
                                    s.wire_points_at(gp),
                                    s.component_at(gp),
                                    s.wire_at(gp),
                                )
                            };

                            if tool_is_select {
                                if points_at_pos.len() >= 1 {
                                    // Junction (2+) or single endpoint (1) - start junction/endpoint drag
                                    drag.set(DragState {
                                        active: true,
                                        component_id: None,
                                        wire_id: None,
                                        junction_point: Some(gp),
                                        start_grid: gp,
                                        current_grid: gp,
                                        multi_selection: false,
                                    });
                                } else if let Some(comp_id) = comp_at {
                                    // Component - check if already in selection for multi-move
                                    highlighted_junction.set(None);
                                    let sel = schematic.read().selection.clone();
                                    let is_selected = sel.components.contains(&comp_id);
                                    let total_selected = sel.components.len() + sel.wires.len();
                                    // Use multi-selection if this component is already selected AND there are multiple items
                                    let use_multi = is_selected && total_selected > 1;

                                    if !is_selected {
                                        // Clear selection and select just this component
                                        let mut s = schematic.write();
                                        s.selection.clear();
                                        s.selection.components.push(comp_id);
                                    }

                                    drag.set(DragState {
                                        active: true,
                                        component_id: Some(comp_id),
                                        wire_id: None,
                                        junction_point: None,
                                        start_grid: gp,
                                        current_grid: gp,
                                        multi_selection: use_multi,
                                    });
                                } else if let Some(wire_id) = wire_at {
                                    // Wire segment - check if already in selection for multi-move
                                    highlighted_junction.set(None);
                                    let sel = schematic.read().selection.clone();
                                    let is_selected = sel.wires.contains(&wire_id);
                                    let total_selected = sel.components.len() + sel.wires.len();
                                    // Use multi-selection if this wire is already selected AND there are multiple items
                                    let use_multi = is_selected && total_selected > 1;

                                    if !is_selected {
                                        // Clear selection and select just this wire
                                        let mut s = schematic.write();
                                        s.selection.clear();
                                        s.selection.wires.push(wire_id);
                                    }

                                    drag.set(DragState {
                                        active: true,
                                        component_id: None,
                                        wire_id: Some(wire_id),
                                        junction_point: None,
                                        start_grid: gp,
                                        current_grid: gp,
                                        multi_selection: use_multi,
                                    });
                                } else {
                                    // Clicking on empty space - start box selection
                                    highlighted_junction.set(None);
                                    schematic.write().selection.clear();
                                    box_selection.set(BoxSelectionState {
                                        active: true,
                                        start_grid: gp,
                                        end_grid: gp,
                                        start_px: (c.x, c.y),
                                        end_px: (c.x, c.y),
                                        just_completed: false,
                                    });
                                }
                            }
                        }
                    },

                    onmouseup: move |_| {
                        is_panning.set(false);

                        // Complete box selection if active
                        let bs = *box_selection.read();
                        if bs.active {
                            // Calculate selection bounds (min/max in grid coordinates)
                            let min_x = bs.start_grid.x.min(bs.end_grid.x);
                            let max_x = bs.start_grid.x.max(bs.end_grid.x);
                            let min_y = bs.start_grid.y.min(bs.end_grid.y);
                            let max_y = bs.start_grid.y.max(bs.end_grid.y);

                            // Only do selection if box has meaningful size
                            if max_x > min_x || max_y > min_y {
                                // Collect IDs first to avoid borrow conflict
                                let comp_ids: Vec<u64> = schematic.read()
                                    .components.iter()
                                    .filter(|comp| {
                                        comp.pos.x >= min_x && comp.pos.x <= max_x
                                        && comp.pos.y >= min_y && comp.pos.y <= max_y
                                    })
                                    .map(|comp| comp.id)
                                    .collect();

                                let wire_ids: Vec<u64> = schematic.read()
                                    .wires.iter()
                                    .filter(|wire| {
                                        wire.points.iter().any(|p| {
                                            p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y
                                        })
                                    })
                                    .map(|wire| wire.id)
                                    .collect();

                                // Now update selection
                                let mut s = schematic.write();
                                s.selection.clear();
                                s.selection.components = comp_ids.clone();
                                s.selection.wires = wire_ids.clone();

                                // Set flag to prevent onclick from clearing selection
                                let has_selection = !comp_ids.is_empty() || !wire_ids.is_empty();
                                box_selection.set(BoxSelectionState {
                                    just_completed: has_selection,
                                    ..BoxSelectionState::default()
                                });
                            } else {
                                box_selection.set(BoxSelectionState::default());
                            }
                        }

                        // Commit drag if active
                        let d = *drag.read();
                        if d.active {
                            let delta_x = d.current_grid.x - d.start_grid.x;
                            let delta_y = d.current_grid.y - d.start_grid.y;

                            if let Some(junction_pos) = d.junction_point {
                                // Calculate final position
                                let new_pos = crate::state::Point::new(
                                    junction_pos.x + delta_x,
                                    junction_pos.y + delta_y,
                                );

                                if delta_x != 0 || delta_y != 0 {
                                    // Move junction - all wire endpoints at this position
                                    schematic.write().move_junction(junction_pos, new_pos);
                                    push_edit(schematic.read().clone(), "Move junction");
                                }

                                // Set highlighted junction to persist selection after drag
                                let final_pos = if delta_x != 0 || delta_y != 0 { new_pos } else { junction_pos };
                                highlighted_junction.set(Some(final_pos));

                                // Add all wires at the junction to the selection so Delete works
                                // This matches standard simulator behavior
                                let wire_points = schematic.read().wire_points_at(final_pos);
                                let mut s = schematic.write();
                                s.selection.clear();
                                for (wire_id, _) in wire_points {
                                    if !s.selection.wires.contains(&wire_id) {
                                        s.selection.wires.push(wire_id);
                                    }
                                }
                            } else if delta_x != 0 || delta_y != 0 {
                                let delta = crate::state::Point::new(delta_x, delta_y);

                                if d.multi_selection {
                                    // Move ALL selected components and wires using the unified method
                                    schematic.write().move_selection(delta);
                                    push_edit(schematic.read().clone(), "Move selection");
                                } else if let Some(comp_id) = d.component_id {
                                    // Move single component WITH attached wires (rubber-banding)
                                    schematic.write().move_component_with_wires(comp_id, delta);
                                    push_edit(schematic.read().clone(), "Move component");
                                } else if let Some(wire_id) = d.wire_id {
                                    // Move single wire (all points)
                                    schematic.write().move_wire(wire_id, delta);
                                    push_edit(schematic.read().clone(), "Move wire");
                                }
                            }
                            drag.set(DragState::default());
                        }

                        // Handle label drag finalization (also handled here for robustness)
                        if label_drag.read().active {
                            let ld = label_drag.read().clone();
                            let (dx, dy) = ld.current_offset;

                            // Only commit if actually moved
                            if dx.abs() > 2.0 || dy.abs() > 2.0 {
                                // Get required data for label position computation
                                let wires_snapshot: Vec<_> = schematic.read().wires.clone();
                                let components_snapshot: Vec<_> = schematic.read().components.clone();
                                let current_grid_size = schematic.read().grid_size;

                                if let Some(comp) = components_snapshot.iter().find(|c| c.id == ld.component_id) {
                                    let (_name_pos, _value_pos) = crate::views::label_placement::compute_label_positions(
                                        comp,
                                        &wires_snapshot,
                                        &components_snapshot,
                                        current_grid_size,
                                    );
                                    let base_pos = if ld.is_name_label { _name_pos } else { _value_pos };

                                    // Final position in screen space (rotated coordinates)
                                    let final_x = base_pos.x + dx;
                                    let final_y = base_pos.y + dy;

                                    // Apply INVERSE rotation to convert back to component-local coords
                                    // (because compute_label_positions will rotate Custom positions)
                                    let (stored_x, stored_y) = match comp.rotation {
                                        crate::state::Rotation::R0 => (final_x, final_y),
                                        crate::state::Rotation::R90 => (-final_y, final_x),  // inverse of (y, -x)
                                        crate::state::Rotation::R180 => (-final_x, -final_y),
                                        crate::state::Rotation::R270 => (final_y, -final_x), // inverse of (-y, x)
                                    };

                                    let new_pos = crate::state::LabelPosition::Custom {
                                        x_offset: stored_x,
                                        y_offset: stored_y,
                                    };

                                    let old_state = schematic.read().clone();
                                    {
                                        let mut sch = schematic.write();
                                        if let Some(c) = sch.components.iter_mut().find(|c| c.id == ld.component_id) {
                                            if ld.is_name_label {
                                                c.name_label_pos = new_pos;
                                            } else {
                                                c.value_label_pos = new_pos;
                                            }
                                        }
                                    }
                                    push_edit(old_state, "Move label");
                                }
                            }
                            label_drag.set(LabelDragState::default());
                        }
                    },



                    onwheel: move |evt| {
                        let c = evt.element_coordinates();
                        let dy = match evt.delta() {
                            dioxus::html::geometry::WheelDelta::Pixels(p) => p.y,
                            dioxus::html::geometry::WheelDelta::Lines(l) => l.y * 20.0,
                            dioxus::html::geometry::WheelDelta::Pages(p) => p.y * 100.0,
                        };
                        let old_z = *zoom.read();
                        let new_z = (old_z * if dy < 0.0 { 1.1 } else { 0.9 }).clamp(0.25, 4.0);
                        let (opx, opy) = *pan.read();
                        let r = new_z / old_z;
                        pan.set((c.x - (c.x - opx) * r, c.y - (c.y - opy) * r));
                        zoom.set(new_z);
                        // Sync to SchematicState for per-document persistence
                        {
                            let mut sch = schematic.write();
                            sch.pan = *pan.read();
                            sch.zoom = *zoom.read();
                        }
                    },

                    onclick: move |evt| {
                        // Close context menu if open
                        if context_menu.read().visible {
                            context_menu.set(ContextMenuState::default());
                            return;
                        }

                        // If box selection just completed, don't clear selection
                        // (onclick fires after mouseup, which already set the selection)
                        if box_selection.read().just_completed {
                            box_selection.set(BoxSelectionState::default());
                            return;
                        }

                        let c = evt.element_coordinates();
                        let (px, py) = *pan.read();
                        let z = *zoom.read();
                        let gs = schematic.read().grid_size;
                        let gp = Point::from_pixels((c.x - px) / z, (c.y - py) / z, gs);

                        let mut s = schematic.write();
                        match s.tool {
                            Tool::Select => {
                                // Selection and drag are handled in mousedown/mouseup
                                // No action needed on click
                            }
                            Tool::Place(k) => {
                                // Make the change first
                                s.add_component(k, gp);
                                // Then push to history AFTER the change
                                drop(s);
                                push_edit(schematic.read().clone(), &format!("Add {:?}", k));
                            }
                            Tool::Wire => {
                                // Wire drawing is transient - cancel with ESC
                                // Undo only affects completed wires
                                if s.wire_drawing.active {
                                    // Additional clicks add vertices (multi-segment wires)
                                    s.extend_wire(gp);
                                } else {
                                    // First click starts wire
                                    s.start_wire(gp);
                                }
                            }
                            Tool::Probe => {
                                // Probe behavior: probe WIRES/NODES, not components
                                // SPICE simulators measure node voltages, not component voltages
                                let clicked_component = s.component_at(gp);
                                let clicked_wire = s.wire_at(gp);

                                let probe_name = if clicked_wire.is_some() {
                                    // Wire probe - look up net name from cached mapping
                                    // This mapping is populated when netlist is generated
                                    s.net_mapping.get(&gp)
                                        .cloned()
                                        .or_else(|| {
                                            // Try nearby points (user may click slightly off)
                                            for neighbor in gp.neighbors() {
                                                if let Some(name) = s.net_mapping.get(&neighbor) {
                                                    return Some(name.clone());
                                                }
                                            }
                                            None
                                        })
                                        .map(|net| format!("V({})", net))
                                } else {
                                    None
                                };

                                // Release schematic lock before accessing sim_state
                                drop(s);

                                if let Some(name) = probe_name {
                                    // Check if this is the ground node before toggle
                                    let is_ground = {
                                        let state = sim_state.read();
                                        if let Some(ref ground) = state.ground_node {
                                            let net_name = name
                                                .trim_start_matches("V(")
                                                .trim_end_matches(')');
                                            ground.eq_ignore_ascii_case(net_name)
                                        } else {
                                            false
                                        }
                                    };

                                    if is_ground {
                                        // Ground node - add informative console message
                                        sim_state.write().console_messages.push(
                                            ConsoleMessage::info(format!(
                                                "{} is the ground reference (0V)",
                                                name
                                            ))
                                        );
                                        log::info!("{} is the ground reference", name);
                                    } else {
                                        // Try to toggle the waveform visibility
                                        let found = sim_state.write().toggle_waveform_visibility(&name);

                                        if found {
                                            // Show waveform panel if not already visible
                                            if !waveform_visible.read().0 {
                                                waveform_visible.set(WaveformVisible(true));
                                            }
                                        } else {
                                            // No matching waveform found - add console message
                                            sim_state.write().console_messages.push(
                                                ConsoleMessage::warning(format!(
                                                    "Probe: {} - no simulation data. Run simulation first.",
                                                    name
                                                ))
                                            );
                                            log::info!("Probe: {} - no simulation data available", name);
                                        }
                                    }
                                } else if clicked_wire.is_some() {
                                    // Wire was clicked, but no net mapping found (simulation not run yet)
                                    sim_state.write().console_messages.push(
                                        ConsoleMessage::warning("Run simulation first to probe node voltages".to_string())
                                    );
                                } else if clicked_component.is_some() {
                                    // User clicked on a component body - explain proper probe usage
                                    sim_state.write().console_messages.push(
                                        ConsoleMessage::info(
                                            "Tip: Probe wires to measure node voltages. Click on a wire, not the component body.".to_string()
                                        )
                                    );
                                } else {
                                    // No wire or component at this position
                                    sim_state.write().console_messages.push(
                                        ConsoleMessage::warning("Click on a wire to probe its voltage".to_string())
                                    );
                                }
                            }
                            Tool::Label => {
                                // Place a net label at clicked position
                                // For now use a default name, can be edited via double-click
                                let label_num = s.net_labels.len() + 1;
                                s.add_net_label(gp, format!("NET{}", label_num));
                                drop(s);
                                push_edit(schematic.read().clone(), "Add label");
                            }
                        }
                    },

                    // Context menu on right-click
                    oncontextmenu: move |evt| {
                        evt.prevent_default();
                        // Use client coordinates for menu position (fixed positioning)
                        let client = evt.client_coordinates();
                        // Use element coordinates for grid position calculation
                        let elem = evt.element_coordinates();
                        let (px, py) = *pan.read();
                        let z = *zoom.read();
                        let gs = schematic.read().grid_size;
                        let gp = Point::from_pixels((elem.x - px) / z, (elem.y - py) / z, gs);

                        // If drawing a wire, right-click finishes it
                        if schematic.read().wire_drawing.active {
                            if schematic.write().finish_wire().is_some() {
                                push_edit(schematic.read().clone(), "Add wire");
                            }
                            return;
                        }

                        let s = schematic.read();
                        let comp = s.component_at(gp);
                        let wire = s.wire_at(gp);

                        context_menu.set(ContextMenuState {
                            visible: true,
                            position: (client.x, client.y),
                            target_component: comp,
                            target_wire: wire,
                        });
                    },

                    ondoubleclick: move |evt| {
                        let c = evt.element_coordinates();
                        let px = pan.read().0;
                        let py = pan.read().1;
                        let z = *zoom.read();
                        let gs = schematic.read().grid_size;
                        let gp = Point::from_pixels((c.x - px) / z, (c.y - py) / z, gs);

                        // Check if we're finishing a wire
                        if schematic.read().tool == Tool::Wire && schematic.read().wire_drawing.active {
                            // Don't add the double-click point, just finish
                            if schematic.write().finish_wire().is_some() {
                                push_edit(schematic.read().clone(), "Add wire");
                            }
                            return;
                        }

                        let s = schematic.read();
                        // Check if double-clicked on a component to edit it
                        if let Some(comp_id) = s.component_at(gp) {
                            // Open edit modal
                            let client = evt.client_coordinates();
                            drop(s);
                            editing.write().component_id = Some(comp_id);
                            editing.write().position = (client.x, client.y);
                        }
                    },

                    // Definitions for patterns
                    defs {
                        // Minor grid pattern
                        pattern {
                            id: "minorGrid",
                            width: "20",
                            height: "20",
                            pattern_units: "userSpaceOnUse",
                            rect { width: "20", height: "20", fill: "none", stroke: "{th.border()}", stroke_width: "0.5", opacity: "0.2" }
                        }
                        // Major grid pattern
                        pattern {
                            id: "majorGrid",
                            width: "100",
                            height: "100",
                            pattern_units: "userSpaceOnUse",
                            rect { width: "100", height: "100", fill: "url(#minorGrid)" }
                            rect { width: "100", height: "100", fill: "none", stroke: "{th.border()}", stroke_width: "1", opacity: "0.3" }
                        }
                    }

                    // Transform group
                    g {
                        transform: {
                            let (px, py) = *pan.read();
                            let z = *zoom.read();
                            format!("translate({px},{py}) scale({z})")
                        },

                        // Grid background rectangle
                        rect {
                            x: "-5000",
                            y: "-5000",
                            width: "10000",
                            height: "10000",
                            fill: "url(#majorGrid)",
                        }

                        // Origin marker
                        circle { cx: "0", cy: "0", r: "5", fill: "{th.accent_primary()}" }
                        line { x1: "-30", y1: "0", x2: "30", y2: "0", stroke: "{th.accent_primary()}", stroke_width: "1" }
                        line { x1: "0", y1: "-30", x2: "0", y2: "30", stroke: "{th.accent_primary()}", stroke_width: "1" }

                        // Wires - render with preview positions during component or wire drag
                        // VIEWPORT CULLING: Only render wires that are visible in the current view
                        {
                            let d = *drag.read();
                            let comp_dragging_id = if d.active { d.component_id } else { None };
                            let wire_dragging_id = if d.active { d.wire_id } else { None };
                            let is_multi_drag = d.active && d.multi_selection;
                            let delta = if d.active {
                                crate::state::Point::new(
                                    d.current_grid.x - d.start_grid.x,
                                    d.current_grid.y - d.start_grid.y,
                                )
                            } else {
                                crate::state::Point::new(0, 0)
                            };
                            let gs = schematic.read().grid_size;
                            let junction_pos = if d.active { d.junction_point } else { None };
                            // Also check persisted junction highlight for after drag ends
                            let persisted_junction = *highlighted_junction.read();
                            let selection = schematic.read().selection.clone();
                            let probe_wires = probe_hover_wires.read().clone();

                            // Get cross-probe highlighted node names (strings like "N001", "out", etc.)
                            let cross_probe_nodes = cross_probe.read().highlighted_schematic().clone();
                            // Get net mapping from schematic (populated after simulation)
                            let net_map = schematic.read().net_mapping.clone();


                            // VIEWPORT CULLING: Re-enabled with proper dynamic canvas_size tracking
                            // Canvas dimensions are now updated via ResizeObserver polling
                            let (canvas_w, canvas_h) = *canvas_size.read();
                            let current_pan = *pan.read();
                            let current_zoom = *zoom.read();

                            // Compute viewport bounds in grid coordinates
                            let viewport = crate::state::viewport::Viewport::from_transform(
                                canvas_w, canvas_h, current_pan, current_zoom, gs
                            );

                            // Filter to only wires visible in the current viewport (performance optimization)
                            let visible_wires: Vec<_> = schematic.read().wires.iter()
                                .filter(|w| viewport.is_wire_visible(&w.points))
                                .cloned()
                                .collect();

                            rsx! {
                                for wire in visible_wires.iter() {
                                    // Check if wire is in the highlighted net (probe tool or cross-probe)
                                    {
                                        // Check probe tool highlight
                                        let is_probe_highlighted = probe_wires.contains(&wire.id);

                                        // Check cross-probe highlight - look up each wire point in net_mapping
                                        // A wire should highlight if ANY of its points belongs to a cross-probed net
                                        let is_cross_probe_highlighted = if cross_probe_nodes.is_empty() {
                                            false
                                        } else {
                                            wire.points.iter().any(|point| {
                                                if let Some(net_name) = net_map.get(point) {
                                                    // Check if this net name matches any cross-probe target
                                                    // Case-insensitive comparison for robustness
                                                    cross_probe_nodes.iter().any(|target| {
                                                        target.eq_ignore_ascii_case(net_name)
                                                    })
                                                } else {
                                                    false
                                                }
                                            })
                                        };

                                        let should_highlight = is_probe_highlighted || is_cross_probe_highlighted;

                                        // Calculate points based on drag type
                                        if Some(wire.id) == wire_dragging_id {
                                            // Wire drag - move all points
                                            rsx! {
                                                WireSvg {
                                                    points: wire.points.iter().map(|p| crate::state::Point::new(p.x + delta.x, p.y + delta.y)).collect(),
                                                    grid_size: gs,
                                                    selected: true,
                                                    probe_highlight: should_highlight,
                                                }
                                            }
                                        } else if is_multi_drag && selection.has_wire(wire.id) {
                                            // Multi-selection drag - move entire selected wire
                                            rsx! {
                                                WireSvg {
                                                    points: wire.points.iter().map(|p| crate::state::Point::new(p.x + delta.x, p.y + delta.y)).collect(),
                                                    grid_size: gs,
                                                    selected: true,
                                                    probe_highlight: should_highlight,
                                                }
                                            }
                                        } else if junction_pos.is_some() {
                                            // Junction/endpoint drag - move only points at junction position
                                            rsx! {
                                                WireSvg {
                                                    points: wire.points.iter().map(|p| {
                                                        if Some(*p) == junction_pos {
                                                            crate::state::Point::new(p.x + delta.x, p.y + delta.y)
                                                        } else {
                                                            *p
                                                        }
                                                    }).collect(),
                                                    grid_size: gs,
                                                    selected: (wire.points.first() == junction_pos.as_ref() || wire.points.last() == junction_pos.as_ref()) || selection.has_wire(wire.id),
                                                    probe_highlight: should_highlight,
                                                }
                                            }
                                        } else {
                                            // Normal rendering - check persisted junction highlight OR selection
                                            rsx! {
                                                WireSvg {
                                                    points: schematic.read().get_wire_preview_points(wire, comp_dragging_id, delta),
                                                    grid_size: gs,
                                                    selected: (persisted_junction.is_some() && (wire.points.first() == persisted_junction.as_ref() || wire.points.last() == persisted_junction.as_ref())) || selection.has_wire(wire.id),
                                                    probe_highlight: should_highlight,
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Wire being drawn - show committed segments
                        if schematic.read().wire_drawing.active {
                            WireSvg {
                                points: schematic.read().wire_drawing.points.clone(),
                                grid_size: schematic.read().grid_size,
                                selected: false,
                            }

                            // Wire preview - orthogonal path from last point to cursor
                            WirePreviewSvg {
                                schematic: schematic,
                            }
                        }

                        // Junction dots - render after all wires for proper z-ordering
                        // Dots are rendered separately from wires so selected dots appear on top
                        {
                            let d = *drag.read();
                            let junction_pos = if d.active { d.junction_point } else { None };
                            let persisted = *highlighted_junction.read();
                            let gs = schematic.read().grid_size;
                            let theme: Signal<Theme> = use_context();
                            let th = theme.read();
                            let selected_col = th.accent_primary();
                            let normal_col = th.accent_success();

                            // Calculate drag delta for junction points
                            let drag_delta = if d.active && junction_pos.is_some() {
                                crate::state::Point::new(
                                    d.current_grid.x - d.start_grid.x,
                                    d.current_grid.y - d.start_grid.y,
                                )
                            } else {
                                crate::state::Point::new(0, 0)
                            };

                            // Collect all component terminal positions to exclude from junction circles
                            // Standard simulators don't show junction circles at component terminals
                            let terminal_positions: std::collections::HashSet<Point> = schematic.read()
                                .components.iter()
                                .flat_map(|comp| comp.terminal_positions())
                                .map(|(_, pos)| pos)
                                .collect();

                            // For multi-selection, we need to know if wire is selected
                            let is_multi_drag = d.active && d.multi_selection;
                            let multi_delta = if is_multi_drag {
                                crate::state::Point::new(
                                    d.current_grid.x - d.start_grid.x,
                                    d.current_grid.y - d.start_grid.y,
                                )
                            } else {
                                crate::state::Point::new(0, 0)
                            };
                            let selection = schematic.read().selection.clone();
                            let probe_wires = probe_hover_wires.read().clone();

                            // Get cross-probe state for junction highlighting
                            let cross_probe_nodes = cross_probe.read().highlighted_schematic().clone();
                            let net_map = schematic.read().net_mapping.clone();

                            // Collect wire endpoint segment counts to identify true junctions
                            // A junction exists only where 3+ wire segments meet
                            // Each wire endpoint contributes 1 segment to that point
                            // (Middle points of a wire are NOT junctions even if multiple wires cross)
                            let mut point_segment_count: std::collections::HashMap<Point, usize> = std::collections::HashMap::new();
                            let mut point_selected_count: std::collections::HashMap<Point, usize> = std::collections::HashMap::new();
                            let mut point_probed: std::collections::HashMap<Point, bool> = std::collections::HashMap::new();

                            for wire in schematic.read().wires.iter() {
                                let wire_is_selected = selection.has_wire(wire.id);
                                let wire_is_probed = probe_wires.contains(&wire.id);

                                // Check if wire is cross-probed (any point matches a cross-probe net)
                                let wire_is_cross_probed = if cross_probe_nodes.is_empty() {
                                    false
                                } else {
                                    wire.points.iter().any(|pt| {
                                        if let Some(net_name) = net_map.get(pt) {
                                            cross_probe_nodes.iter().any(|target| target.eq_ignore_ascii_case(net_name))
                                        } else {
                                            false
                                        }
                                    })
                                };

                                // Only endpoints contribute to junction count (first and last point)
                                // Get actual display positions considering drag operations
                                for (idx, pt) in wire.points.iter().enumerate() {
                                    let is_endpoint = idx == 0 || idx == wire.points.len() - 1;
                                    if !is_endpoint {
                                        continue; // Skip middle points - they don't form junctions
                                    }

                                    // Apply drag delta based on drag type
                                    let display_pt = if junction_pos.is_some() && Some(*pt) == junction_pos {
                                        crate::state::Point::new(pt.x + drag_delta.x, pt.y + drag_delta.y)
                                    } else if is_multi_drag && wire_is_selected {
                                        crate::state::Point::new(pt.x + multi_delta.x, pt.y + multi_delta.y)
                                    } else {
                                        *pt
                                    };

                                    // Skip points at component terminals
                                    if terminal_positions.contains(&display_pt) {
                                        continue;
                                    }

                                    // Count this endpoint as contributing 1 segment
                                    *point_segment_count.entry(display_pt).or_insert(0) += 1;

                                    // Track selection state - count selected vs total for "all selected" logic
                                    let is_selected =
                                        (junction_pos.is_some() && (Some(*pt) == junction_pos || wire.points.first() == junction_pos.as_ref() || wire.points.last() == junction_pos.as_ref()))
                                        || (persisted.is_some() && (wire.points.first() == persisted.as_ref() || wire.points.last() == persisted.as_ref()))
                                        || wire_is_selected;

                                    if is_selected {
                                        *point_selected_count.entry(display_pt).or_insert(0) += 1;
                                    }
                                    // Mark as probed if either probe tool or cross-probe is active
                                    if wire_is_probed || wire_is_cross_probed {
                                        point_probed.insert(display_pt, true);
                                    }
                                }
                            }

                            rsx! {
                                // Junction dots at true junctions (3+ wire segments meeting)
                                // Highlight only when ALL connected wires are selected (box selection)
                                // This matches standard simulator behavior

                                // Render normal junction dots (not all-selected, not probed)
                                for (pt, total) in point_segment_count.iter() {
                                    if *total >= 3 && !*point_probed.get(pt).unwrap_or(&false) {
                                        {
                                            let selected = *point_selected_count.get(pt).unwrap_or(&0);
                                            let all_selected = selected >= *total;
                                            if !all_selected {
                                                let (x, y) = pt.to_pixels(gs);
                                                rsx! { circle { cx: "{x}", cy: "{y}", r: "4", fill: "{normal_col}" } }
                                            } else {
                                                rsx! {}
                                            }
                                        }
                                    }
                                }
                                // Render all-selected junction dots (highlighted)
                                for (pt, total) in point_segment_count.iter() {
                                    if *total >= 3 && !*point_probed.get(pt).unwrap_or(&false) {
                                        {
                                            let selected = *point_selected_count.get(pt).unwrap_or(&0);
                                            let all_selected = selected >= *total;
                                            if all_selected {
                                                let (x, y) = pt.to_pixels(gs);
                                                rsx! { circle { cx: "{x}", cy: "{y}", r: "4", fill: "{selected_col}" } }
                                            } else {
                                                rsx! {}
                                            }
                                        }
                                    }
                                }
                                // Render probe-highlighted junction dots on top (orange)
                                for (pt, total) in point_segment_count.iter() {
                                    if *total >= 3 && *point_probed.get(pt).unwrap_or(&false) {
                                        {
                                            let (x, y) = pt.to_pixels(gs);
                                            rsx! { circle { cx: "{x}", cy: "{y}", r: "5", fill: "#ffa500" } }
                                        }
                                    }
                                }

                                // Render EXPLICIT junctions from schematic.junctions
                                // These are shown as solid dots (same as implicit 3+ junctions)
                                {
                                    let sch = schematic.read();
                                    rsx! {
                                        for junction in sch.junctions.iter() {
                                            {
                                                let (x, y) = junction.pos.to_pixels(gs);
                                                rsx! {
                                                    circle { cx: "{x}", cy: "{y}", r: "4", fill: "{normal_col}" }
                                                }
                                            }
                                        }
                                    }
                                }

                                // Render hovered corner indicator (shows draggable point)
                                {
                                    if let Some(pt) = *hovered_corner.read() {
                                        let theme: Signal<Theme> = use_context();
                                        let accent = theme.read().accent_primary();
                                        let (x, y) = pt.to_pixels(gs);
                                        rsx! {
                                            // Outer ring - indicates hoverable/draggable
                                            circle {
                                                cx: "{x}",
                                                cy: "{y}",
                                                r: "7",
                                                fill: "none",
                                                stroke: "{accent}",
                                                stroke_width: "2",
                                                opacity: "0.8"
                                            }
                                            // Inner dot
                                            circle {
                                                cx: "{x}",
                                                cy: "{y}",
                                                r: "3",
                                                fill: "{accent}"
                                            }
                                        }
                                    } else {
                                        rsx! {}
                                    }
                                }
                            }
                        }

                        // Box selection rectangle (rubber-band selection visual)
                        {
                            let bs = *box_selection.read();
                            let gs = schematic.read().grid_size;
                            let theme: Signal<Theme> = use_context();
                            let selection_color = theme.read().accent_primary();

                            if bs.active {
                                let (x1, y1) = bs.start_grid.to_pixels(gs);
                                let (x2, y2) = bs.end_grid.to_pixels(gs);
                                let x = x1.min(x2);
                                let y = y1.min(y2);
                                let w = (x2 - x1).abs();
                                let h = (y2 - y1).abs();
                                rsx! {
                                    rect {
                                        x: "{x}",
                                        y: "{y}",
                                        width: "{w}",
                                        height: "{h}",
                                        fill: "rgba(0, 180, 255, 0.1)",
                                        stroke: "{selection_color}",
                                        stroke_width: "1",
                                        stroke_dasharray: "5,3",
                                    }
                                }
                            } else {
                                rsx! {}
                            }
                        }

                        // Components - render at current position, or with drag offset if being dragged
                        // VIEWPORT CULLING: Only render components visible in the current view
                        {
                            // Pre-compute drag state before the for loop (RSX for doesn't allow let statements)
                            let d = *drag.read();
                            let dragging_id = if d.active { d.component_id } else { None };
                            let is_multi_drag = d.active && d.multi_selection;
                            // Calculate delta for offset-based dragging (no snap to cursor)
                            let drag_delta = if d.active {
                                crate::state::Point::new(
                                    d.current_grid.x - d.start_grid.x,
                                    d.current_grid.y - d.start_grid.y,
                                )
                            } else {
                                crate::state::Point::new(0, 0)
                            };
                            let gs = schematic.read().grid_size;
                            let selection = schematic.read().selection.clone();

                            // VIEWPORT CULLING: Compute visible region for O(visible) rendering
                            let (canvas_w, canvas_h) = *canvas_size.read();
                            let (pan_x, pan_y) = *pan.read();
                            let current_zoom = *zoom.read();
                            let viewport = Viewport::from_transform(canvas_w, canvas_h, (pan_x, pan_y), current_zoom, gs);

                            // Pre-filter components for visibility (O(visible) rendering)
                            // Component symbols typically span ~60 grid units, so use half_size of 30
                            let visible_components: Vec<_> = schematic.read().components.iter()
                                .filter(|c| viewport.is_component_visible(c.pos, 40))
                                .cloned()
                                .collect();

                            rsx! {
                                for comp in visible_components.iter() {
                                    {
                                        // Compute smart label positions using LabelPlacer
                                        let wires: Vec<_> = schematic.read().wires.clone();
                                        let components: Vec<_> = schematic.read().components.clone();
                                        let (name_pos, value_pos) = crate::views::label_placement::compute_label_positions(
                                            comp,
                                            &wires,
                                            &components,
                                            gs,
                                        );

                                        rsx! {
                                            CompSvg {
                                                component_id: comp.id,
                                                kind: comp.kind,
                                                pos: if Some(comp.id) == dragging_id {
                                                    crate::state::Point::new(comp.pos.x + drag_delta.x, comp.pos.y + drag_delta.y)
                                                } else if is_multi_drag && selection.has_component(comp.id) {
                                                    crate::state::Point::new(comp.pos.x + drag_delta.x, comp.pos.y + drag_delta.y)
                                                } else {
                                                    comp.pos
                                                },
                                                rotation: comp.rotation.degrees(),
                                                name: comp.name.clone(),
                                                value: comp.value.clone(),
                                                grid_size: gs,
                                                selected: selection.has_component(comp.id),
                                                name_label_x: name_pos.x,
                                                name_label_y: name_pos.y,
                                                value_label_x: value_pos.x,
                                                value_label_y: value_pos.y,
                                                on_label_drag: move |(comp_id, is_name, new_x, new_y): (u64, bool, f64, f64)| {
                                                    // Update component label position in state
                                                    let old_state = schematic.read().clone();
                                                    {
                                                        let mut sch = schematic.write();
                                                        if let Some(c) = sch.components.iter_mut().find(|c| c.id == comp_id) {
                                                            let new_pos = crate::state::LabelPosition::Custom {
                                                                x_offset: new_x,
                                                                y_offset: new_y,
                                                            };
                                                            if is_name {
                                                                c.name_label_pos = new_pos;
                                                            } else {
                                                                c.value_label_pos = new_pos;
                                                            }
                                                        }
                                                    }
                                                    // Push to undo history
                                                    push_edit(old_state, "Move label");
                                                },
                                                zoom: *zoom.read(),
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // Net Labels
                        for label in schematic.read().net_labels.iter() {
                            NetLabelSvg {
                                pos: label.pos,
                                name: label.name.clone(),
                                grid_size: schematic.read().grid_size,
                            }
                        }

                        // ====================================================================
                        // DC Annotation Overlay (Operating point display)
                        // ====================================================================
                        // Renders node voltages and branch currents directly on the schematic
                        // when DC annotation mode is enabled.
                        // Note: sim_state is accessed from Schematic component props above.
                        // ====================================================================
                        DcAnnotationLayer {
                            grid_size: schematic.read().grid_size,
                            current_zoom: *zoom.read(),
                        }

                        // Placement preview
                        {
                            let tool = schematic.read().tool;
                            let rot = schematic.read().preview_rotation;
                            if let Tool::Place(k) = tool {
                                rsx! { PreviewSvg { kind: k, pos: *mouse_grid.read(), grid_size: schematic.read().grid_size, rotation: rot } }
                            } else {
                                rsx! {}
                            }
                        }
                    }
                    } // Close transform g element
                    // ========================================================================
                    // GLOBAL DRAG CAPTURE OVERLAY
                    // ========================================================================
                    // This permanent overlay is ALWAYS rendered but with pointer-events
                    // controlled by drag state. When ANY drag is active (component or label),
                    // pointer-events: all captures all mouse events. Otherwise, pointer-events: none
                    // allows events to pass through to underlying elements.
                    //
                    // This approach is used by high-end EDA tools because it:
                    // 1. Eliminates z-ordering issues (overlay is always on top)
                    // 2. Eliminates timing issues (no conditional rendering)
                    // 3. Provides single point of control for all drag operations
                    // ========================================================================
                    {
                        // Check if ANY drag is active - combine component drag and label drag states
                        let component_dragging = drag.read().active;
                        let label_dragging = label_drag.read().active;
                        let any_drag_active = component_dragging || label_dragging;

                        // Compute pointer-events based on drag state
                        let overlay_pointer_events = if any_drag_active { "all" } else { "none" };
                        let overlay_cursor = if label_dragging {
                            "grabbing"
                        } else if component_dragging {
                            "move"
                        } else {
                            "default"
                        };

                        // Capture values needed for handlers

                        rsx! {
                            rect {
                                // Large absolute values to cover entire viewport
                                // Must be outside transform group to use screen coordinates
                                x: "0",
                                y: "0",
                                width: "10000",
                                height: "10000",
                                fill: "transparent",
                                style: "pointer-events: {overlay_pointer_events}; cursor: {overlay_cursor};",

                                onmousemove: move |e| {
                                    // Handle label drag movement
                                    if label_drag.read().active {
                                        let (start_x, start_y) = label_drag.read().start_offset;
                                        let coords = e.page_coordinates();
                                        let z_current = *zoom.read();
                                        let dx = (coords.x - start_x) / z_current;
                                        let dy = (coords.y - start_y) / z_current;
                                        label_drag.write().current_offset = (dx, dy);
                                    }

                                    // Handle component drag movement
                                    if drag.read().active {
                                        // Get current mouse position and convert to grid coords
                                        let (pan_x, pan_y) = *pan.read();
                                        let z_current = *zoom.read();
                                        let grid = schematic.read().grid_size;
                                        let coords = e.element_coordinates();
                                        let wx = (coords.x - pan_x) / z_current;
                                        let wy = (coords.y - pan_y) / z_current;
                                        let gx = (wx / grid as f64).round() as i32;
                                        let gy = (wy / grid as f64).round() as i32;
                                        let new_grid = crate::state::Point::new(gx, gy);

                                        // Update drag current position
                                        drag.write().current_grid = new_grid;
                                    }
                                },

                                onmouseup: move |_| {
                                    // Handle label drag finalization
                                    if label_drag.read().active {
                                        let ld = label_drag.read().clone();
                                        let (dx, dy) = ld.current_offset;

                                        // Only commit if actually moved
                                        if dx.abs() > 2.0 || dy.abs() > 2.0 {
                                            // Get required data for label position computation
                                            let wires_snapshot: Vec<_> = schematic.read().wires.clone();
                                            let components_snapshot: Vec<_> = schematic.read().components.clone();
                                            let current_grid_size = schematic.read().grid_size;

                                            if let Some(comp) = components_snapshot.iter().find(|c| c.id == ld.component_id) {
                                                let (_name_pos, _value_pos) = crate::views::label_placement::compute_label_positions(
                                                    comp,
                                                    &wires_snapshot,
                                                    &components_snapshot,
                                                    current_grid_size,
                                                );
                                                let base_pos = if ld.is_name_label { _name_pos } else { _value_pos };

                                                // Final position in screen space (rotated coordinates)
                                                let final_x = base_pos.x + dx;
                                                let final_y = base_pos.y + dy;

                                                // Apply INVERSE rotation to convert back to component-local coords
                                                let (stored_x, stored_y) = match comp.rotation {
                                                    crate::state::Rotation::R0 => (final_x, final_y),
                                                    crate::state::Rotation::R90 => (-final_y, final_x),
                                                    crate::state::Rotation::R180 => (-final_x, -final_y),
                                                    crate::state::Rotation::R270 => (final_y, -final_x),
                                                };

                                                let new_pos = crate::state::LabelPosition::Custom {
                                                    x_offset: stored_x,
                                                    y_offset: stored_y,
                                                };

                                                let old_state = schematic.read().clone();
                                                {
                                                    let mut sch = schematic.write();
                                                    if let Some(c) = sch.components.iter_mut().find(|c| c.id == ld.component_id) {
                                                        if ld.is_name_label {
                                                            c.name_label_pos = new_pos;
                                                        } else {
                                                            c.value_label_pos = new_pos;
                                                        }
                                                    }
                                                }
                                                push_edit(old_state, "Move label");
                                            }
                                        }
                                        label_drag.set(LabelDragState::default());
                                    }

                                    // Handle component drag finalization
                                    if drag.read().active {
                                        let d = drag.read().clone();
                                        let delta_x = d.current_grid.x - d.start_grid.x;
                                        let delta_y = d.current_grid.y - d.start_grid.y;

                                        if delta_x != 0 || delta_y != 0 {
                                            let delta = crate::state::Point::new(delta_x, delta_y);

                                            if d.multi_selection {
                                                schematic.write().move_selection(delta);
                                                push_edit(schematic.read().clone(), "Move selection");
                                            } else if let Some(comp_id) = d.component_id {
                                                schematic.write().move_component_with_wires(comp_id, delta);
                                                push_edit(schematic.read().clone(), "Move component");
                                            } else if let Some(wire_id) = d.wire_id {
                                                schematic.write().move_wire(wire_id, delta);
                                                push_edit(schematic.read().clone(), "Move wire");
                                            } else if let Some(junction_pos) = d.junction_point {
                                                let new_pos = crate::state::Point::new(
                                                    junction_pos.x + delta_x,
                                                    junction_pos.y + delta_y,
                                                );
                                                schematic.write().move_junction(junction_pos, new_pos);
                                                push_edit(schematic.read().clone(), "Move junction");
                                            }
                                        }
                                        drag.set(DragState::default());
                                    }
                                },
                            }
                        }
                    }
                } // End svg rsx! block
                } // End cursor block

                // Status bar
                div {
                    style: "position: absolute; bottom: 0; left: 0; right: 0; display: flex; justify-content: space-between; padding: 4px 8px; background: {th.bg_tertiary()}dd; font-size: 11px; color: {th.text_muted()}; font-family: monospace;",
                    span { {match schematic.read().tool { Tool::Select => "Select | Del: delete | R: rotate | Ctrl+Z/Y: undo/redo | G/V/C/L/D: place", Tool::Wire => "Wire | Click: add points | Space: toggle route | DblClick/RightClick: finish", Tool::Place(_) => "Place | Click: place | R: rotate | Esc: cancel", Tool::Probe => "Probe | Click node/wire to add voltage trace", Tool::Label => "Label | Click to place net label | DblClick to edit" }} }
                    span { {format!("({}, {}) | {:.0}%", mouse_grid.read().x, mouse_grid.read().y, *zoom.read() * 100.0)} }
                }

                // Context menu overlay
                if context_menu.read().visible {
                    ContextMenu {
                        position: context_menu.read().position,
                        items: {
                            let cm = context_menu.read();
                            let has_selection = !schematic.read().selection.is_empty();
                            let can_undo = history.read().can_undo();
                            let can_redo = history.read().can_redo();

                            if cm.target_component.is_some() || has_selection {
                                schematic_context_menu(has_selection, false, can_undo, can_redo)
                            } else {
                                canvas_context_menu(false, can_undo, can_redo)
                            }
                        },
                        on_action: move |action| {
                            match action {
                                MenuAction::Undo => {
                                    if history.read().can_undo() {
                                        // Preserve current pan/zoom (not part of undo history)
                                        let current_pan = schematic.read().pan;
                                        let current_zoom = schematic.read().zoom;
                                        history.write().undo();
                                        let mut restored = history.read().current().clone();
                                        restored.pan = current_pan;
                                        restored.zoom = current_zoom;
                                        schematic.set(restored);
                                    }
                                }
                                MenuAction::Redo => {
                                    if history.read().can_redo() {
                                        // Preserve current pan/zoom (not part of undo history)
                                        let current_pan = schematic.read().pan;
                                        let current_zoom = schematic.read().zoom;
                                        history.write().redo();
                                        let mut restored = history.read().current().clone();
                                        restored.pan = current_pan;
                                        restored.zoom = current_zoom;
                                        schematic.set(restored);
                                    }
                                }
                                MenuAction::Delete => {
                                    schematic.write().delete_selection();
                                    push_edit(schematic.read().clone(), "Delete selection");
                                }
                                MenuAction::Rotate => {
                                    schematic.write().rotate_selection();
                                    push_edit(schematic.read().clone(), "Rotate selection");
                                }
                                _ => {} // Other actions not yet implemented
                            }
                        },
                        on_close: move |_| {
                            context_menu.set(ContextMenuState::default());
                        },
                    }
                }
            }
        }

            // Component Edit Modal
            if let Some(edit_comp_id) = editing.read().component_id {
                ComponentEditModal {
                    component_id: edit_comp_id,
                    position: editing.read().position,
                    schematic: schematic,
                    on_save: move |(name, value, params)| {
                        // Update component with new values
                        if let Some(comp) = schematic.write().components.iter_mut().find(|c| c.id == edit_comp_id) {
                            comp.name = name;
                            comp.value = value;
                            comp.params = params;
                        }
                        editing.write().component_id = None;
                    },
                    on_cancel: move |_| {
                        editing.write().component_id = None;
                    },
                }
            }

            // Close Confirmation Dialog - using reusable UnsavedChangesModal component
            if let Some((close_idx, doc_name)) = close_confirm.read().clone() {
                UnsavedChangesModal {
                    visible: true,
                    filename: Some(doc_name),
                    on_result: move |result: SaveDialogResult| {
                        match result {
                            SaveDialogResult::Save => {
                                // Save first, then close
                                spawn(async move {
                                    file_handlers::save_schematic(schematic, sim_state).await;
                                    // Now close the document
                                    let was_active = doc_manager.read().active_index == close_idx;
                                    doc_manager.write().close_document(close_idx);
                                    if was_active {
                                        let docs = doc_manager.read();
                                        schematic.set(docs.active().schematic.clone());
                                        sim_state.set(docs.active().simulation.clone());
                                    }
                                    close_confirm.set(None);
                                });
                            }
                            SaveDialogResult::DontSave => {
                                // Close without saving
                                let was_active = doc_manager.read().active_index == close_idx;
                                doc_manager.write().close_document(close_idx);
                                if was_active {
                                    let docs = doc_manager.read();
                                    schematic.set(docs.active().schematic.clone());
                                    sim_state.set(docs.active().simulation.clone());
                                }
                                close_confirm.set(None);
                            }
                            SaveDialogResult::Cancel => {
                                // Just dismiss the dialog
                                close_confirm.set(None);
                            }
                        }
                    },
                }
            }
        }
        // End canvas container (tabs + canvas)
        }
    }
}
