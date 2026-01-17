//! Schematic Editor View - Pure SVG Implementation
//!
//! Interactive schematic capture using SVG for all rendering.
//! Supports pan, zoom, drag-to-move, undo/redo, and context menus.

use dioxus::prelude::*;

use crate::components::component_edit::ComponentEditModal;
use crate::components::component_library::ComponentLibrary;
use crate::components::context_menu::{schematic_context_menu, canvas_context_menu, ContextMenu, MenuAction};
use crate::state::{ComponentType, Point, SchematicState, Tool, SchematicHistory};
use crate::theme::Theme;

/// Drag operation state
#[derive(Clone, Copy, PartialEq, Default)]
struct DragState {
    active: bool,
    component_id: Option<u64>,
    start_grid: Point,
    current_grid: Point,
}

/// Context menu state
#[derive(Clone, Default)]
struct ContextMenuState {
    visible: bool,
    position: (f64, f64),
    target_component: Option<u64>,
    target_wire: Option<u64>,
}

/// Component editing state
#[derive(Clone, Default)]
struct EditingState {
    /// Component being edited
    component_id: Option<u64>,
    /// Screen position for popup
    position: (f64, f64),
}

/// Main schematic editor component - Pure SVG
#[component]
pub fn Schematic() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut schematic: Signal<SchematicState> = use_context();

    // Viewport state
    let mut pan = use_signal(|| (400.0f64, 300.0f64));
    let mut zoom = use_signal(|| 1.0f64);
    let mut is_panning = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0f64, 0.0f64));
    let mut mouse_grid = use_signal(|| Point::new(0, 0));

    // Drag-to-move state
    let mut drag = use_signal(DragState::default);

    // Context menu state
    let mut context_menu = use_signal(ContextMenuState::default);

    // Undo/Redo history (local for now, will integrate with SchematicHistory later)
    let mut history = use_signal(|| SchematicHistory::new(schematic.read().clone(), 100));

    // Component editing state
    let mut editing = use_signal(EditingState::default);

    rsx! {
        div {
            class: "schematic-container",
            style: "display: flex; flex-direction: column; width: 100%; height: 100%; overflow: hidden;",

            SchematicToolbar { schematic: schematic }

            // Main content area with library sidebar and canvas
            div {
                style: "display: flex; flex: 1 1 auto; overflow: hidden;",
                
                // Component Library sidebar
                ComponentLibrary { schematic: schematic }

                // Canvas area
                div {
                    class: "schematic-canvas-wrapper",
                    style: "flex: 1 1 auto; position: relative; overflow: hidden;",

                // Pure SVG canvas
                svg {
                    style: "position: absolute; inset: 0; width: 100%; height: 100%; background: {th.bg_primary()};",

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
                        }
                        
                        // Handle component dragging
                        let d = *drag.read();
                        if d.active && d.component_id.is_some() {
                            drag.set(DragState {
                                current_grid: gp,
                                ..d
                            });
                        }
                        
                        last_mouse.set((c.x, c.y));
                    },

                    onmousedown: move |evt| {
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
                        // Left click on component to start drag (in Select mode)
                        else if evt.trigger_button() == Some(dioxus::html::input_data::MouseButton::Primary) {
                            let s = schematic.read();
                            if matches!(s.tool, Tool::Select) {
                                if let Some(comp_id) = s.component_at(gp) {
                                    drag.set(DragState {
                                        active: true,
                                        component_id: Some(comp_id),
                                        start_grid: gp,
                                        current_grid: gp,
                                    });
                                }
                            }
                        }
                    },

                    onmouseup: move |_| {
                        is_panning.set(false);
                        
                        // Commit drag if active
                        let d = *drag.read();
                        if d.active {
                            if let Some(comp_id) = d.component_id {
                                let delta_x = d.current_grid.x - d.start_grid.x;
                                let delta_y = d.current_grid.y - d.start_grid.y;
                                
                                if delta_x != 0 || delta_y != 0 {
                                    // Apply the move FIRST
                                    {
                                        let mut s = schematic.write();
                                        if let Some(comp) = s.components.iter_mut().find(|c| c.id == comp_id) {
                                            comp.pos.x += delta_x;
                                            comp.pos.y += delta_y;
                                        }
                                    }
                                    // Then save state AFTER the change
                                    history.write().push(schematic.read().clone(), "Move component");
                                }
                            }
                            drag.set(DragState::default());
                        }
                    },
                    
                    onmouseleave: move |_| {
                        is_panning.set(false);
                        drag.set(DragState::default());
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
                    },

                    onclick: move |evt| {
                        // Close context menu if open
                        if context_menu.read().visible {
                            context_menu.set(ContextMenuState::default());
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
                                s.selection.clear();
                                if let Some(id) = s.component_at(gp) { s.selection.components.push(id); }
                                else if let Some(id) = s.wire_at(gp) { s.selection.wires.push(id); }
                            }
                            Tool::Place(k) => {
                                // Make the change first
                                s.add_component(k, gp);
                                // Then push to history AFTER the change
                                drop(s);
                                history.write().push(schematic.read().clone(), format!("Add {:?}", k));
                            }
                            Tool::Wire => {
                                if s.wire_drawing.active { s.extend_wire(gp); }
                                else { s.start_wire(gp); }
                            }
                            Tool::Probe => {
                                // Find what we're probing
                                if let Some(comp_id) = s.component_at(gp) {
                                    // Get the component name for probing
                                    if let Some(comp) = s.components.iter().find(|c| c.id == comp_id) {
                                        let probe_name = format!("V({})", comp.name);
                                        // Log for now - will integrate with SimulationState
                                        println!("Probe: {}", probe_name);
                                    }
                                } else if s.wire_at(gp).is_some() {
                                    // For wires, we'd need to identify the net name
                                    let probe_name = format!("V(net_{})", gp.x * 1000 + gp.y);
                                    println!("Probe: {}", probe_name);
                                }
                            }
                            Tool::Label => {
                                // Place a net label at clicked position
                                // For now use a default name, can be edited via double-click
                                let label_num = s.net_labels.len() + 1;
                                s.add_net_label(gp, format!("NET{}", label_num));
                                drop(s);
                                history.write().push(schematic.read().clone(), "Add label".to_string());
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
                        
                        let s = schematic.read();
                        // Check if double-clicked on a component
                        if let Some(comp_id) = s.component_at(gp) {
                            // Open edit modal
                            let client = evt.client_coordinates();
                            drop(s);
                            editing.write().component_id = Some(comp_id);
                            editing.write().position = (client.x, client.y);
                        } else if s.tool == Tool::Wire && s.wire_drawing.active {
                            drop(s);
                            schematic.write().finish_wire();
                        }
                    },

                    onkeydown: move |evt| {
                        // Handle Ctrl+key shortcuts
                        if evt.modifiers().ctrl() {
                            match evt.key() {
                                Key::Character(c) if c == "z" || c == "Z" => {
                                    if history.read().can_undo() {
                                        history.write().undo();
                                        schematic.set(history.read().current().clone());
                                    }
                                    return;
                                }
                                Key::Character(c) if c == "y" || c == "Y" => {
                                    if history.read().can_redo() {
                                        history.write().redo();
                                        schematic.set(history.read().current().clone());
                                    }
                                    return;
                                }
                                Key::Character(c) if c == "c" || c == "C" => {
                                    // Copy selection to clipboard
                                    schematic.write().copy_selection();
                                    return;
                                }
                                Key::Character(c) if c == "v" || c == "V" => {
                                    // Paste at current mouse position
                                    let gp = *mouse_grid.read();
                                    schematic.write().paste_at(gp);
                                    history.write().push(schematic.read().clone(), "Paste");
                                    return;
                                }
                                _ => {}
                            }
                        }
                        
                        let mut s = schematic.write();
                        match evt.key() {
                            Key::Escape => { s.tool = Tool::Select; s.wire_drawing.active = false; }
                            Key::Delete => {
                                // Make the change FIRST
                                s.delete_selection();
                                // Then push to history AFTER
                                drop(s);
                                history.write().push(schematic.read().clone(), "Delete selection");
                            }
                            Key::Character(c) if c == "r" || c == "R" => {
                                // Make the change FIRST
                                s.rotate_selection();
                                // Then push to history AFTER
                                drop(s);
                                history.write().push(schematic.read().clone(), "Rotate selection");
                            }
                            Key::Character(c) if c == "w" || c == "W" => s.tool = Tool::Wire,
                            _ => {}
                        }
                    },

                    tabindex: "0",

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

                        // Wires
                        for wire in schematic.read().wires.iter() {
                            WireSvg {
                                points: wire.points.clone(),
                                grid_size: schematic.read().grid_size,
                                selected: schematic.read().selection.has_wire(wire.id),
                            }
                        }

                        // Components
                        for comp in schematic.read().components.iter() {
                            CompSvg {
                                kind: comp.kind,
                                pos: comp.pos,
                                rotation: comp.rotation.degrees(),
                                name: comp.name.clone(),
                                value: comp.value.clone(),
                                grid_size: schematic.read().grid_size,
                                selected: schematic.read().selection.has_component(comp.id),
                            }
                        }
                        
                        // Drag preview ghost - shows where component will move to
                        if drag.read().active {
                            if let Some(comp_id) = drag.read().component_id {
                                if let Some(comp) = schematic.read().components.iter().find(|c| c.id == comp_id) {
                                    PreviewSvg {
                                        kind: comp.kind,
                                        pos: *mouse_grid.read(),
                                        grid_size: schematic.read().grid_size,
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

                        // Placement preview
                        if let Tool::Place(k) = schematic.read().tool {
                            PreviewSvg { kind: k, pos: *mouse_grid.read(), grid_size: schematic.read().grid_size }
                        }
                    }
                }

                // Status bar
                div {
                    style: "position: absolute; bottom: 0; left: 0; right: 0; display: flex; justify-content: space-between; padding: 4px 8px; background: {th.bg_tertiary()}dd; font-size: 11px; color: {th.text_muted()}; font-family: monospace;",
                    span { {match schematic.read().tool { Tool::Select => "Select | Del: delete | R: rotate | Ctrl+Z/Y: undo/redo", Tool::Wire => "Wire | Click: add points | DblClick: finish", Tool::Place(_) => "Place | Click: place | Esc: cancel", Tool::Probe => "Probe | Click node/wire to add voltage trace", Tool::Label => "Label | Click to place net label | DblClick to edit" }} }
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
                                        history.write().undo();
                                        schematic.set(history.read().current().clone());
                                    }
                                }
                                MenuAction::Redo => {
                                    if history.read().can_redo() {
                                        history.write().redo();
                                        schematic.set(history.read().current().clone());
                                    }
                                }
                                MenuAction::Delete => {
                                    schematic.write().delete_selection();
                                    history.write().push(schematic.read().clone(), "Delete selection");
                                }
                                MenuAction::Rotate => {
                                    schematic.write().rotate_selection();
                                    history.write().push(schematic.read().clone(), "Rotate selection");
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
        }
    }
}

#[component]
fn SchematicToolbar(schematic: Signal<SchematicState>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let tool = schematic.read().tool;

    rsx! {
        div {
            style: "display: flex; align-items: center; height: 32px; padding: 0 8px; background: {th.bg_tertiary()}; border-bottom: 1px solid {th.border()}; gap: 4px;",
            ToolBtn { label: "↖ Select", active: matches!(tool, Tool::Select), onclick: move |_| schematic.write().tool = Tool::Select }
            ToolBtn { label: "— Wire", active: matches!(tool, Tool::Wire), onclick: move |_| schematic.write().tool = Tool::Wire }
            ToolBtn { label: "⚡ Probe", active: matches!(tool, Tool::Probe), onclick: move |_| schematic.write().tool = Tool::Probe }
            ToolBtn { label: "🏷 Label", active: matches!(tool, Tool::Label), onclick: move |_| schematic.write().tool = Tool::Label }
            div { style: "width: 1px; height: 18px; background: {th.border()}; margin: 0 4px;" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer;", onclick: move |_| schematic.write().rotate_selection(), "⟳ Rotate" }
            button { style: "padding: 4px 8px; background: {th.surface()}; border: 1px solid {th.border()}; border-radius: 4px; color: {th.text_primary()}; font-size: 12px; cursor: pointer;", onclick: move |_| schematic.write().delete_selection(), "🗑 Delete" }
            div { style: "flex: 1;" }
            span { style: "font-size: 12px; color: {th.text_muted()};", {format!("{} components, {} wires", schematic.read().components.len(), schematic.read().wires.len())} }
        }
    }
}

#[component]
fn ToolBtn(label: &'static str, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (bg, col) = if active { (th.accent_primary(), "#fff") } else { (th.surface(), th.text_primary()) };
    rsx! { button { style: "padding: 4px 8px; background: {bg}; border: 1px solid {th.border()}; border-radius: 4px; color: {col}; font-size: 12px; cursor: pointer;", onclick: move |e| onclick.call(e), "{label}" } }
}

#[component]
fn WireSvg(points: Vec<Point>, grid_size: i32, selected: bool) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    if points.len() < 2 { return rsx! {}; }
    let col = if selected { th.accent_primary() } else { th.accent_success() };
    let sw = if selected { "3" } else { "2" };
    
    // Build path string properly
    let mut d = String::new();
    for (i, p) in points.iter().enumerate() {
        let (x, y) = p.to_pixels(grid_size);
        if i == 0 {
            d.push_str(&format!("M{} {}", x, y));
        } else {
            d.push_str(&format!(" L{} {}", x, y));
        }
    }
    
    rsx! {
        path { d: "{d}", stroke: "{col}", stroke_width: "{sw}", fill: "none", stroke_linecap: "round" }
        for p in points.iter() {
            {
                let (x, y) = p.to_pixels(grid_size);
                rsx! { circle { cx: "{x}", cy: "{y}", r: "4", fill: "{col}" } }
            }
        }
    }
}

#[component]
fn CompSvg(
    kind: ComponentType,
    pos: Point,
    rotation: i32,
    name: String,
    value: String,
    grid_size: i32,
    selected: bool,
    #[props(default)] ondoubleclick: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (cx, cy) = pos.to_pixels(grid_size);
    let col = if selected { th.accent_primary() } else { th.text_primary() };
    let sw = if selected { "2.5" } else { "2" };
    let path = symbol_path(kind);
    rsx! {
        g {
            transform: "translate({cx},{cy}) rotate({rotation})",
            style: "cursor: pointer;",
            ondoubleclick: move |e| ondoubleclick.call(e),
            if selected { circle { cx: "0", cy: "0", r: "25", fill: "{th.accent_primary()}20", stroke: "{th.accent_primary()}", stroke_width: "1", stroke_dasharray: "3,2" } }
            // Invisible hit area for clicks
            rect { x: "-20", y: "-30", width: "40", height: "60", fill: "transparent", pointer_events: "all" }
            path { d: "{path}", stroke: "{col}", stroke_width: "{sw}", fill: "none", stroke_linecap: "round" }
            g { transform: "rotate({-rotation})",
                text { x: "0", y: "-25", text_anchor: "middle", font_size: "10", fill: "{th.text_primary()}", font_weight: "600", "{name}" }
                text { x: "0", y: "35", text_anchor: "middle", font_size: "9", fill: "{th.text_secondary()}", "{value}" }
            }
        }
    }
}

#[component]
fn PreviewSvg(kind: ComponentType, pos: Point, grid_size: i32) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (cx, cy) = pos.to_pixels(grid_size);
    let path = symbol_path(kind);
    rsx! {
        g { transform: "translate({cx},{cy})", opacity: "0.6",
            circle { cx: "0", cy: "0", r: "20", fill: "{th.accent_primary()}30", stroke: "{th.accent_primary()}", stroke_dasharray: "4,2" }
            path { d: "{path}", stroke: "{th.accent_primary()}", stroke_width: "2", fill: "none" }
        }
    }
}

fn symbol_path(k: ComponentType) -> &'static str {
    match k {
        ComponentType::Resistor => "M-20 0 L-15 0 L-12-8 L-6 8 L0-8 L6 8 L12-8 L15 0 L20 0",
        ComponentType::Capacitor => "M-20 0 L-4 0 M-4-12 L-4 12 M4-12 L4 12 M4 0 L20 0",
        ComponentType::Inductor => "M-20 0 C-15 0-15-10-10-10 C-5-10-5 0 0 0 C5 0 5-10 10-10 C15-10 15 0 20 0",
        ComponentType::Diode => "M-20 0 L-8 0 M-8-10 L-8 10 L8 0 Z M8-10 L8 10 M8 0 L20 0",
        ComponentType::Ground => "M0-15 L0 0 M-12 0 L12 0 M-8 5 L8 5 M-4 10 L4 10",
        ComponentType::VoltageSource | ComponentType::VoltageSourceAc | ComponentType::VoltageSourcePulse | ComponentType::VoltageSourceSin => 
            "M0-20 L0-12 M0 12 L0 20 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M-4-4 L4-4 M0-8 L0 0",
        ComponentType::CurrentSource => "M0-20 L0-12 M0 12 L0 20 M0 0 m-12 0 a12 12 0 1 0 24 0 a12 12 0 1 0-24 0 M0-6 L0 6 M-3 3 L0 6 L3 3",
        ComponentType::NpnBjt => "M-15 0 L-5 0 M-5-12 L-5 12 M-5-6 L10-15 M-5 6 L10 15 M5 10 L10 15 L8 8",
        ComponentType::PnpBjt => "M-15 0 L-5 0 M-5-12 L-5 12 M-5-6 L10-15 M-5 6 L10 15 M-2 4 L-5 6 L0 10",
        ComponentType::Nmos => "M-15 0 L-8 0 M-8-10 L-8 10 M-4-8 L-4-4 M-4-2 L-4 2 M-4 4 L-4 8 M-4 0 L10 0 M-4-6 L10-6 L10-15 M-4 6 L10 6 L10 15",
        ComponentType::Pmos => "M-15 0 L-8 0 M-8-10 L-8 10 M-4-8 L-4-4 M-4-2 L-4 2 M-4 4 L-4 8 M-4 0 L10 0 M-4-6 L10-6 L10-15 M-4 6 L10 6 L10 15",
    }
}

/// Net label SVG component - flag symbol with name
#[component]
fn NetLabelSvg(pos: Point, name: String, grid_size: i32) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let (cx, cy) = pos.to_pixels(grid_size);
    
    // Calculate text width for background (approximate)
    let text_width = (name.len() * 7) as i32 + 10;
    
    rsx! {
        g { transform: "translate({cx},{cy})",
            // Connection point circle
            circle { cx: "0", cy: "0", r: "3", fill: "{th.accent_primary()}" }
            // Flag pole
            line { x1: "0", y1: "0", x2: "0", y2: "-15", stroke: "{th.accent_primary()}", stroke_width: "2" }
            // Flag background
            rect { x: "2", y: "-22", width: "{text_width}", height: "14", rx: "2", fill: "{th.accent_primary()}", opacity: "0.15" }
            rect { x: "2", y: "-22", width: "{text_width}", height: "14", rx: "2", stroke: "{th.accent_primary()}", stroke_width: "1", fill: "none" }
            // Net name text
            text { x: "6", y: "-12", font_size: "10", fill: "{th.accent_primary()}", font_weight: "600", font_family: "monospace", "{name}" }
        }
    }
}
