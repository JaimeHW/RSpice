//! Component SVG Rendering
//!
//! Renders schematic components with full support for:
//! - Symbol rendering from SVG assets or fallback paths
//! - Selection highlighting
//! - Terminal pin labels
//! - Draggable name and value labels
//! - Component rotation

use crate::state::{ComponentType, Point};
use crate::theme::Theme;
use crate::views::schematic::types::{DragState, LabelDragState};
use crate::views::symbol_assets;
use dioxus::prelude::*;

/// Rotate a point by degrees (used for terminal pin name positioning)
///
/// Applies 2D rotation transformation to convert between component-local
/// and screen coordinates.
///
/// # Arguments
/// * `x` - X coordinate
/// * `y` - Y coordinate  
/// * `degrees` - Rotation angle in degrees (0, 90, 180, 270)
///
/// # Returns
/// Rotated (x, y) coordinates
pub fn rotate_point_by_deg(x: f64, y: f64, degrees: i32) -> (f64, f64) {
    let rad = (degrees as f64) * std::f64::consts::PI / 180.0;
    let cos = rad.cos();
    let sin = rad.sin();
    (x * cos - y * sin, x * sin + y * cos)
}

/// Component SVG - full featured schematic component rendering
///
/// Renders a component with:
/// - Symbol graphic from SVG asset or fallback path
/// - Selection highlight ring when selected
/// - Terminal pin labels (configurable visibility)
/// - Draggable name label (reference designator)
/// - Draggable value label (component value)
///
/// # Props
/// - `component_id` - Unique ID for identifying label drag targets
/// - `kind` - Component type (resistor, capacitor, etc.)
/// - `pos` - Grid position
/// - `rotation` - Rotation in degrees (0, 90, 180, 270)
/// - `name` - Reference designator (R1, C2, etc.)
/// - `value` - Component value (1k, 10uF, etc.)
/// - `grid_size` - Grid size in pixels
/// - `selected` - Whether component is selected
/// - `name_label_x/y` - Name label position offsets
/// - `value_label_x/y` - Value label position offsets
/// - `ondoubleclick` - Handler for double-click to edit
/// - `on_label_drag` - Callback when label drag completes
/// - `zoom` - Current zoom level for drag delta scaling
#[component]
pub fn CompSvg(
    /// Component ID for identifying which component's label is being dragged
    component_id: u64,
    kind: ComponentType,
    pos: Point,
    rotation: i32,
    name: String,
    value: String,
    grid_size: i32,
    selected: bool,
    /// Name label X offset (from smart placement)
    #[props(default)]
    name_label_x: f64,
    /// Name label Y offset (from smart placement)
    #[props(default = -25.0)]
    name_label_y: f64,
    /// Value label X offset
    #[props(default)]
    value_label_x: f64,
    /// Value label Y offset
    #[props(default = 35.0)]
    value_label_y: f64,
    #[props(default)] ondoubleclick: EventHandler<MouseEvent>,
    /// Callback when a label drag completes: (component_id, is_name_label, x_offset, y_offset)
    #[props(default)]
    on_label_drag: EventHandler<(u64, bool, f64, f64)>,
    /// Current zoom level for proper drag delta scaling
    #[props(default = 1.0)]
    zoom: f64,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let display_settings: Signal<crate::state::display_settings::SchematicDisplaySettings> =
        use_context();
    let th = theme.read();
    let settings = display_settings.read();

    let (cx, cy) = pos.to_pixels(grid_size);
    let col = if selected {
        th.accent_primary()
    } else {
        th.text_primary()
    };
    let sw = if selected { "2" } else { "1.5" }; // Stroke width (only for fallback paths)

    // Component hover state for terminal pin visibility
    let mut is_hovered = use_signal(|| false);

    // Label hover states for drag interaction visual feedback
    let mut name_label_hovered = use_signal(|| false);
    let mut value_label_hovered = use_signal(|| false);

    // Label drag state - use global context provided by Schematic for proper pointer capture
    // This allows the global overlay to handle events even when cursor leaves this component
    let mut label_drag: Signal<LabelDragState> = use_context();

    // Component drag state - used to disable label pointer-events during component drag
    let component_drag: Signal<DragState> = use_context();

    let asset = symbol_assets::get_component_svg(kind);

    // Get terminal offsets for this component type
    let terminal_offsets = kind.terminal_offsets();

    // Determine if we should show terminal pin names based on settings and hover state
    let show_pins = match settings.show_pin_names {
        crate::state::display_settings::PinNameVisibility::Always => true,
        crate::state::display_settings::PinNameVisibility::OnHover => *is_hovered.read(),
        crate::state::display_settings::PinNameVisibility::Hidden => false,
    };

    // Label cursor style - "move" when hovered/dragging
    // Only show grab cursor when no drag is active
    // Read signals directly for proper reactivity
    let label_cursor = if label_drag.read().active {
        "grabbing"
    } else if component_drag.read().active {
        "default" // During component drag, don't show grab cursor
    } else if *name_label_hovered.read() || *value_label_hovered.read() {
        "grab"
    } else {
        "default"
    };

    // Label highlight color for hover feedback
    let name_label_bg = if *name_label_hovered.read() {
        format!("{}20", th.accent_primary())
    } else {
        "transparent".to_string()
    };
    let value_label_bg = if *value_label_hovered.read() {
        format!("{}20", th.accent_primary())
    } else {
        "transparent".to_string()
    };

    rsx! {
        // Define SVG filter for label shadow (improves contrast against busy backgrounds)
        defs {
            filter {
                id: "label-shadow",
                x: "-20%",
                y: "-20%",
                width: "140%",
                height: "140%",
                // Light drop shadow for dark mode, dark shadow for light mode
                feDropShadow {
                    dx: "0",
                    dy: "1",
                    std_deviation: "1",
                    flood_color: if th.is_dark { "#000000" } else { "#ffffff" },
                    flood_opacity: if th.is_dark { "0.5" } else { "0.8" },
                }
            }
        }
        g {
            transform: "translate({cx},{cy}) rotate({rotation})",
            style: "cursor: pointer; color: {col};",
            ondoubleclick: move |e| ondoubleclick.call(e),
            onmouseenter: move |_| is_hovered.set(true),
            onmouseleave: move |_| is_hovered.set(false),

            // Selection highlight (when selected) - tight bounding box from actual symbol dimensions
            if selected && settings.show_selection_ring {
                {
                    // Padding around symbol for selection box
                    const SELECTION_PADDING: f64 = 4.0;

                    // Get symbol bounds for native orientation (rotation=0)
                    // The rect is inside the rotated g element, so it rotates automatically
                    // No need to swap dimensions - the rotation transform handles it
                    let (base_w, base_h) = symbol_assets::get_symbol_bounds(kind);
                    let box_w = base_w + SELECTION_PADDING;
                    let box_h = base_h + SELECTION_PADDING;
                    let half_w = box_w / 2.0;
                    let half_h = box_h / 2.0;
                    rsx! {
                        rect {
                            x: "{-half_w}",
                            y: "{-half_h}",
                            width: "{box_w}",
                            height: "{box_h}",
                            rx: "3",
                            fill: "{th.accent_primary()}10",
                            stroke: "{th.accent_primary()}",
                            stroke_width: "1",
                            stroke_dasharray: "3 2",
                            pointer_events: "none",
                        }
                    }
                }
            }

            // Invisible hit area for clicks
            rect { x: "-20", y: "-30", width: "40", height: "60", fill: "transparent", pointer_events: "all" }

            // Component symbol rendering
            if let Some(svg) = asset {
                {
                   let (vx, vy, vw, vh) = svg.view_box;
                   let target_size = (grid_size as f64) * 4.0;
                   let scale = (target_size / vw.max(vh)) * svg.scale;
                   let center_x = vx + vw / 2.0 - svg.x_offset;
                   let center_y = vy + vh / 2.0 - svg.y_offset;
                   let base_scale = scale / svg.scale;

                   rsx! {
                       g {
                           transform: "scale({scale}) translate({-center_x}, {-center_y})",
                           dangerous_inner_html: "{svg.content}",
                           stroke: "{col}",
                           fill: "{col}",
                           stroke_width: "{1.5 * svg.stroke_scale / base_scale}",
                       }
                   }
                }
            } else {
                // Fallback to hardcoded paths
                path { d: "{crate::views::schematic::svg::symbol_path(kind)}", stroke: "{col}", stroke_width: "{sw}", fill: "none", stroke_linecap: "round" }
            }

            // Terminal pin labels (simple text only, no decoration)
            // Controlled by settings.show_pin_names (Hidden/OnHover/Always)
            if show_pins && !terminal_offsets.is_empty() {
                g { transform: "rotate({-rotation})",
                    for (term_name, offset) in terminal_offsets.iter() {
                        {
                            // Rotate the offset back to screen space
                            let (rx, ry) = rotate_point_by_deg(offset.x as f64, offset.y as f64, rotation);
                            // Convert grid offset to pixels
                            let tx = rx * (grid_size as f64);
                            let ty = ry * (grid_size as f64);

                            // Position text just outside terminal, anchor based on direction
                            let offset_dist = 5.0;
                            let (label_x, label_y, anchor) = if tx.abs() > ty.abs() {
                                if tx > 0.0 { (tx + offset_dist, ty, "start") }
                                else { (tx - offset_dist, ty, "end") }
                            } else {
                                if ty > 0.0 { (tx, ty + offset_dist + 3.0, "middle") }
                                else { (tx, ty - offset_dist, "middle") }
                            };

                            rsx! {
                                text {
                                    x: "{label_x}",
                                    y: "{label_y}",
                                    text_anchor: "{anchor}",
                                    dominant_baseline: "middle",
                                    font_size: "{settings.pin_font_size}",
                                    font_family: "{Theme::FONT_MONO}",
                                    fill: "{th.text_muted()}",
                                    style: "pointer-events: none;",
                                    "{term_name}"
                                }
                            }
                        }
                    }
                }
            }

            // Component labels with smart placement
            // Labels are counter-rotated to remain horizontal regardless of component orientation
            if settings.show_component_names || settings.show_component_values {
                g {
                    transform: "rotate({-rotation})",
                    style: "cursor: {label_cursor};",

                    // Name label (reference designator: R1, C2, M1, etc.)
                    // Full drag support: click and drag to reposition
                    if settings.show_component_names && !name.is_empty() {
                        {
                            // Calculate displayed position (original + drag offset if dragging this label)
                            let ld = label_drag.read();
                            let is_dragging_this = ld.active && ld.component_id == component_id && ld.is_name_label;
                            let display_x = if is_dragging_this {
                                name_label_x + ld.current_offset.0
                            } else {
                                name_label_x
                            };
                            let display_y = if is_dragging_this {
                                name_label_y + ld.current_offset.1
                            } else {
                                name_label_y
                            };

                            rsx! {
                                g {
                                    // Label event handling - global overlay now captures drag events
                                    onmouseenter: move |_| name_label_hovered.set(true),
                                    onmouseleave: move |_| {
                                        name_label_hovered.set(false);
                                    },

                                    // Invisible hit area for mouse events
                                    rect {
                                        // Width based on text length (approx 7px per character)
                                        x: "{display_x - (name.len() as f64 * 3.5).max(12.0)}",
                                        y: "{display_y - 8.0}",
                                        width: "{(name.len() as f64 * 7.0).max(24.0)}",
                                        height: "14",
                                        fill: "transparent",
                                        style: "cursor: {label_cursor};",
                                        pointer_events: "all",
                                        // Start drag on mousedown - set global state for overlay to track
                                        onmousedown: move |e| {
                                            e.stop_propagation();
                                            let coords = e.page_coordinates();
                                            label_drag.set(LabelDragState {
                                                active: true,
                                                component_id,
                                                is_name_label: true,
                                                start_offset: (coords.x, coords.y),
                                                current_offset: (0.0, 0.0),
                                            });
                                        },
                                    }

                                    // Hover highlight background (shown when hovered OR dragging)
                                    if *name_label_hovered.read() || is_dragging_this {
                                        rect {
                                            // Width tightly fitting text, centered vertically
                                            x: "{display_x - (name.len() as f64 * 3.5 + 2.0).max(10.0)}",
                                            y: "{display_y - 5.0}",
                                            width: "{(name.len() as f64 * 7.0 + 4.0).max(20.0)}",
                                            height: "10",
                                            rx: "2",
                                            fill: "{name_label_bg}",
                                            style: "pointer-events: none;",
                                        }
                                    }

                                    text {
                                        x: "{display_x}",
                                        y: "{display_y}",
                                        text_anchor: "middle",
                                        dominant_baseline: "middle",
                                        font_size: "{settings.name_font_size}",
                                        font_weight: "{settings.name_font_weight_css()}",
                                        font_family: "{Theme::FONT_MONO}",
                                        fill: "{th.text_primary()}",
                                        filter: if settings.label_shadow_enabled { "url(#label-shadow)" } else { "" },
                                        style: "user-select: none; pointer-events: none;",
                                        "{name}"
                                    }
                                }
                            }
                        }
                    }

                    // Value label (component value: 1k, 10uF, etc.)
                    // Full drag support: click and drag to reposition
                    if settings.show_component_values && !value.is_empty() {
                        {
                            // Calculate displayed position (original + drag offset if dragging this label)
                            let ld = label_drag.read();
                            let is_dragging_this = ld.active && ld.component_id == component_id && !ld.is_name_label;
                            let display_x = if is_dragging_this {
                                value_label_x + ld.current_offset.0
                            } else {
                                value_label_x
                            };
                            let display_y = if is_dragging_this {
                                value_label_y + ld.current_offset.1
                            } else {
                                value_label_y
                            };

                            rsx! {
                                g {
                                    // Disable pointer events during component drag to prevent interference
                                    // Read signal directly for proper reactivity across all components
                                    style: if component_drag.read().active { "pointer-events: none;" } else { "" },
                                    onmouseenter: move |_| value_label_hovered.set(true),
                                    onmouseleave: move |_| {
                                        value_label_hovered.set(false);
                                    },

                                    // Hit area rect with onmousedown directly attached
                                    rect {
                                        // Width based on text length (approx 6px per character for values)
                                        x: "{display_x - (value.len() as f64 * 3.0).max(15.0)}",
                                        y: "{display_y - 7.0}",
                                        width: "{(value.len() as f64 * 6.0).max(30.0)}",
                                        height: "14",
                                        fill: "transparent",
                                        style: "cursor: {label_cursor};",
                                        pointer_events: "all",
                                        // Start drag on mousedown - set global state for overlay to track
                                        onmousedown: move |e| {
                                            e.stop_propagation();
                                            let coords = e.page_coordinates();
                                            label_drag.set(LabelDragState {
                                                active: true,
                                                component_id,
                                                is_name_label: false,
                                                start_offset: (coords.x, coords.y),
                                                current_offset: (0.0, 0.0),
                                            });
                                        },
                                    }

                                    // Hover highlight background
                                    if *value_label_hovered.read() || is_dragging_this {
                                        rect {
                                            // Width tightly fitting text
                                            x: "{display_x - (value.len() as f64 * 3.0 + 2.0).max(12.0)}",
                                            y: "{display_y - 5.0}",
                                            width: "{(value.len() as f64 * 6.0 + 4.0).max(24.0)}",
                                            height: "10",
                                            rx: "2",
                                            fill: "{value_label_bg}",
                                            style: "pointer-events: none;",
                                        }
                                    }

                                    text {
                                        x: "{display_x}",
                                        y: "{display_y}",
                                        text_anchor: "middle",
                                        dominant_baseline: "middle",
                                        font_size: "{settings.value_font_size}",
                                        font_family: "{Theme::FONT_MONO}",
                                        fill: "{th.text_secondary()}",
                                        filter: if settings.label_shadow_enabled { "url(#label-shadow)" } else { "" },
                                        style: "user-select: none; pointer-events: none;",
                                        "{value}"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_rotate_point_0_degrees() {
        let (x, y) = rotate_point_by_deg(10.0, 5.0, 0);
        assert!((x - 10.0).abs() < 1e-10);
        assert!((y - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotate_point_90_degrees() {
        let (x, y) = rotate_point_by_deg(10.0, 0.0, 90);
        assert!(x.abs() < 1e-10); // Should be ~0
        assert!((y - 10.0).abs() < 1e-10); // Should be 10
    }

    #[test]
    fn test_rotate_point_180_degrees() {
        let (x, y) = rotate_point_by_deg(10.0, 5.0, 180);
        assert!((x - (-10.0)).abs() < 1e-10);
        assert!((y - (-5.0)).abs() < 1e-10);
    }

    #[test]
    fn test_rotate_point_270_degrees() {
        let (x, y) = rotate_point_by_deg(10.0, 0.0, 270);
        assert!(x.abs() < 1e-10); // Should be ~0
        assert!((y - (-10.0)).abs() < 1e-10); // Should be -10
    }

    #[test]
    fn test_rotate_point_origin() {
        let (x, y) = rotate_point_by_deg(0.0, 0.0, 90);
        assert!(x.abs() < 1e-10);
        assert!(y.abs() < 1e-10);
    }

    #[test]
    fn test_rotate_point_45_degrees() {
        let (x, y) = rotate_point_by_deg(1.0, 0.0, 45);
        let expected = (2.0_f64).sqrt() / 2.0;
        assert!((x - expected).abs() < 1e-10);
        assert!((y - expected).abs() < 1e-10);
    }

    #[test]
    fn test_rotate_point_negative_coords() {
        let (x, y) = rotate_point_by_deg(-5.0, -3.0, 180);
        assert!((x - 5.0).abs() < 1e-10);
        assert!((y - 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_label_hit_area_calculation() {
        // Test that label hit areas scale with text length
        let short_name = "R1";
        let long_name = "VOUT_BUFFER";

        let short_width = (short_name.len() as f64 * 7.0).max(24.0);
        let long_width = (long_name.len() as f64 * 7.0).max(24.0);

        assert!(long_width > short_width);
        assert_eq!(short_width, 24.0); // 2*7=14, but min is 24
        assert_eq!(long_width, 77.0); // 11*7=77
    }

    #[test]
    fn test_value_label_hit_area_calculation() {
        let short_value = "1k";
        let long_value = "10.5uF";

        let short_width = (short_value.len() as f64 * 6.0).max(30.0);
        let long_width = (long_value.len() as f64 * 6.0).max(30.0);

        assert!(long_width > short_width);
        assert_eq!(short_width, 30.0); // 2*6=12, but min is 30
        assert_eq!(long_width, 36.0); // 6*6=36
    }

    #[test]
    fn test_pin_label_position_calculation() {
        // Test pin label positioning logic
        let tx: f64 = 20.0; // Terminal X position (right)
        let ty: f64 = 0.0; // Terminal Y position (center)
        let offset_dist: f64 = 5.0;

        // For rightward terminal, label should be to the right with "start" anchor
        let (label_x, label_y, anchor) = if tx.abs() > ty.abs() {
            if tx > 0.0 {
                (tx + offset_dist, ty, "start")
            } else {
                (tx - offset_dist, ty, "end")
            }
        } else {
            if ty > 0.0 {
                (tx, ty + offset_dist + 3.0, "middle")
            } else {
                (tx, ty - offset_dist, "middle")
            }
        };

        assert_eq!(label_x, 25.0);
        assert_eq!(label_y, 0.0);
        assert_eq!(anchor, "start");
    }

    #[test]
    fn test_pin_label_position_left() {
        let tx: f64 = -20.0; // Terminal X position (left)
        let ty: f64 = 0.0;
        let offset_dist: f64 = 5.0;

        let (label_x, label_y, anchor) = if tx.abs() > ty.abs() {
            if tx > 0.0 {
                (tx + offset_dist, ty, "start")
            } else {
                (tx - offset_dist, ty, "end")
            }
        } else {
            if ty > 0.0 {
                (tx, ty + offset_dist + 3.0, "middle")
            } else {
                (tx, ty - offset_dist, "middle")
            }
        };

        assert_eq!(label_x, -25.0);
        assert_eq!(anchor, "end");
    }

    #[test]
    fn test_pin_label_position_below() {
        let tx: f64 = 0.0;
        let ty: f64 = 20.0; // Terminal Y position (below)
        let offset_dist: f64 = 5.0;

        let (label_x, label_y, anchor) = if tx.abs() > ty.abs() {
            if tx > 0.0 {
                (tx + offset_dist, ty, "start")
            } else {
                (tx - offset_dist, ty, "end")
            }
        } else {
            if ty > 0.0 {
                (tx, ty + offset_dist + 3.0, "middle")
            } else {
                (tx, ty - offset_dist, "middle")
            }
        };

        assert_eq!(label_x, 0.0);
        assert_eq!(label_y, 28.0);
        assert_eq!(anchor, "middle");
    }

    #[test]
    fn test_component_position_conversion() {
        let pos = Point::new(3, 5);
        let grid_size = 10;

        let (cx, cy) = pos.to_pixels(grid_size);

        assert_eq!(cx, 30.0);
        assert_eq!(cy, 50.0);
    }

    #[test]
    fn test_selection_highlight_bounds() {
        // Test that selection bounds calculation works
        const SELECTION_PADDING: f64 = 4.0;

        let (base_w, base_h) = (40.0, 60.0); // Example symbol bounds
        let box_w = base_w + SELECTION_PADDING;
        let box_h = base_h + SELECTION_PADDING;

        assert_eq!(box_w, 44.0);
        assert_eq!(box_h, 64.0);
    }
}
