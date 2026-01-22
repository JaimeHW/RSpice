//! DC Annotation Overlay Component
//!
//! Renders simulation results (node voltages, branch currents) as an overlay
//! on top of the schematic canvas. Follows the pattern of
//! displaying operating point values directly on the schematic.
//!
//! The overlay consists of small badges positioned near circuit nodes and
//! components, showing formatted voltage/current values.

use dioxus::prelude::*;

use crate::state::dc_annotation::{Annotation, AnnotationKind};
use crate::state::leader_line::{compute_leader_line, LabelBounds};
use crate::state::{Point, Wire};
use crate::theme::Theme;

/// Props for the annotation overlay
#[derive(Props, Clone, PartialEq)]
pub struct AnnotationOverlayProps {
    /// Annotations to display
    pub annotations: Vec<Annotation>,
    /// Current zoom level (for scaling text)
    pub zoom: f64,
    /// Grid size in pixels
    pub grid_size: i32,
    /// Whether annotations are stale (schematic changed)
    #[props(default = false)]
    pub is_stale: bool,
    /// Live wires for tracking wire movements (looked up by wire_id)
    #[props(default = Vec::new())]
    pub wires: Vec<Wire>,
    /// Debug: exclusion zones to visualize (as (left, right, top, bottom) tuples in grid units)
    #[props(default = Vec::new())]
    pub debug_zones: Vec<(f64, f64, f64, f64)>,
}

/// Annotation overlay - renders as SVG elements within the schematic
///
/// This component should be rendered inside the schematic SVG, after all
/// wires and components, so annotations appear on top.
#[component]
pub fn AnnotationOverlay(props: AnnotationOverlayProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let _th = theme.read();

    // Hide annotations entirely when stale
    // (schematic topology changed - simulation results are invalid)
    if props.is_stale || props.annotations.is_empty() {
        return rsx! {};
    }

    // Calculate appropriate font size based on zoom
    // At normal zoom (1.0), use 11px for readability
    let base_font_size = 11.0;
    let font_size = (base_font_size / props.zoom).clamp(9.0, 16.0);
    let padding_h = (3.0 / props.zoom).clamp(2.0, 5.0); // Horizontal padding
    let padding_v = (2.0 / props.zoom).clamp(2.0, 4.0); // Vertical padding
    let border_radius = (4.0 / props.zoom).clamp(2.0, 5.0);

    // No additional fixed offset needed - wire point selection already finds
    // optimal positions far from components. The annotation.offset field provides
    // per-annotation fine-tuning if needed.

    // Stale annotations have reduced opacity
    let opacity = if props.is_stale { "0.5" } else { "1.0" };

    let gs = props.grid_size as f64;

    rsx! {
        g {
            class: "annotation-overlay",
            opacity: "{opacity}",

            // DEBUG: Render exclusion zones as semi-transparent rectangles
            for (idx, (left, right, top, bottom)) in props.debug_zones.iter().enumerate() {
                {
                    let x = left * gs;
                    let y = top * gs;
                    let width = (right - left) * gs;
                    let height = (bottom - top) * gs;
                    // Alternate colors for different zone types
                    let fill_color = if idx % 3 == 0 { "rgba(255, 0, 0, 0.2)" }
                                     else if idx % 3 == 1 { "rgba(0, 255, 0, 0.2)" }
                                     else { "rgba(0, 0, 255, 0.2)" };
                    rsx! {
                        rect {
                            x: "{x}",
                            y: "{y}",
                            width: "{width}",
                            height: "{height}",
                            fill: "{fill_color}",
                            stroke: "rgba(255, 0, 0, 0.5)",
                            stroke_width: "1",
                            pointer_events: "none",
                        }
                    }
                }
            }

            for annotation in &props.annotations {
                {
                    // Look up LIVE wire position
                    // Find which wire contains the annotation's original position.
                    // If the wire is deleted, don't render this annotation.
                    let current_position: Option<Point> = if !props.wires.is_empty() {
                        // Find wire that contains (or contained) the annotation position
                        let original_pos = annotation.position;

                        // Try to find a wire with a point matching the original position
                        let wire_and_idx = props.wires.iter()
                            .find_map(|wire| {
                                wire.points.iter().position(|p| *p == original_pos)
                                    .map(|idx| (wire, idx))
                            });

                        if let Some((wire, idx)) = wire_and_idx {
                            // Found the wire - get its CURRENT position at this index
                            Some(wire.points.get(idx).copied().unwrap_or(original_pos))
                        } else {
                            // Wire not found by exact position - try wire_id fallback
                            if let (Some(wire_id), Some(point_idx)) = (annotation.wire_id, annotation.point_index) {
                                props.wires.iter()
                                    .find(|w| w.id == wire_id)
                                    .and_then(|w| w.points.get(point_idx).copied())
                            } else {
                                // Wire not found by exact position or wire_id
                                // Fall back to stored position (schematic may not have changed significantly)
                                Some(original_pos)
                            }
                        }
                    } else {
                        // No wires passed - use stored position (legacy mode)
                        Some(annotation.position)
                    };

                    // Only render if wire exists (hidden when wire deleted)
                    if let Some(current_position) = current_position {
                        // Calculate pixel position from current (possibly live) grid position
                        let node_x = current_position.x as f64 * gs;
                        let node_y = current_position.y as f64 * gs;

                        // Use annotation's offset (set by smart placement algorithm)
                        // Offset is already in pixels, just add it directly
                        let badge_x = node_x + annotation.offset.0;
                        let badge_y = node_y + annotation.offset.1;

                        // Approximate text width - use tighter metrics for compact labels
                        let char_width = font_size * 0.55;  // Tighter fit for compact labels
                        let text_width = annotation.label.len() as f64 * char_width + padding_h * 2.0;
                        let text_height = font_size + padding_v * 2.0;

                        let bg_color = annotation.kind.background();
                        let text_color = annotation.kind.color();
                        let border_color = text_color;

                        // Compute leader line with standard geometry:
                        // 1. Start from closest point on label to node
                        // 2. End at first wire intersection (not node center)
                        let label_bounds = LabelBounds::new(badge_x, badge_y, text_width, text_height);
                        let (line_x1, line_y1, line_x2, line_y2) = compute_leader_line(
                            &label_bounds,
                            (node_x, node_y),
                            &props.wires,
                            props.grid_size,
                        );

                        rsx! {
                            g {
                                class: "annotation-badge",
                                key: "{annotation.source}-{annotation.kind:?}",

                                // Leader line from closest point on label to wire intersection
                                line {
                                    x1: "{line_x1}",
                                    y1: "{line_y1}",
                                    x2: "{line_x2}",
                                    y2: "{line_y2}",
                                    stroke: "{text_color}",
                                    stroke_width: "{1.0 / props.zoom}",
                                    stroke_opacity: "0.5",
                                }

                                // Shadow for depth (subtle)
                                rect {
                                    x: "{badge_x + 1.0}",
                                    y: "{badge_y - text_height + 1.0}",
                                    width: "{text_width}",
                                    height: "{text_height}",
                                    rx: "{border_radius}",
                                    ry: "{border_radius}",
                                    fill: "rgba(0,0,0,0.15)",
                                }

                                // Background rectangle with border
                                rect {
                                    x: "{badge_x}",
                                    y: "{badge_y - text_height}",
                                    width: "{text_width}",
                                    height: "{text_height}",
                                    rx: "{border_radius}",
                                    ry: "{border_radius}",
                                    fill: "{bg_color}",
                                    stroke: "{border_color}",
                                    stroke_width: "{1.0 / props.zoom}",
                                }

                                // Value text
                                text {
                                    x: "{badge_x + padding_h}",
                                    y: "{badge_y - padding_v - 1.0}",
                                    font_family: "{Theme::FONT_MONO}",
                                    font_size: "{font_size}px",
                                    fill: "{text_color}",
                                    font_weight: "600",
                                    style: "user-select: none; pointer-events: none;",
                                    "{annotation.label}"
                                }
                            }
                        }
                    } else {
                        rsx! {}
                    }
                }
            }
        }
    }
}

/// Toggle button for annotation mode in toolbar
#[component]
pub fn AnnotationModeToggle(
    mode: crate::state::AnnotationMode,
    on_toggle: EventHandler<()>,
    #[props(default = false)] has_annotations: bool,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let is_active = mode != crate::state::AnnotationMode::Hidden;

    let bg = if is_active {
        th.accent_primary()
    } else if *hovered.read() {
        th.surface_hover()
    } else {
        th.surface()
    };

    let text_color = if is_active {
        "#ffffff"
    } else if has_annotations {
        th.text_primary()
    } else {
        th.text_muted()
    };

    let tooltip = match mode {
        crate::state::AnnotationMode::Hidden => "Show DC Operating Point (click to enable)",
        crate::state::AnnotationMode::Voltages => "Showing Voltages (click for Currents)",
        crate::state::AnnotationMode::Currents => "Showing Currents (click for All)",
        crate::state::AnnotationMode::All => "Showing All (click to hide)",
    };

    let btn_opacity = if has_annotations { "1.0" } else { "0.6" };

    let btn_style = format!(
        "height: 28px; padding: 0 8px; display: flex; align-items: center; justify-content: center; \
         gap: 4px; background: {bg}; border: 1px solid {border}; border-radius: {radius}; \
         color: {text_color}; font-size: 11px; font-weight: 500; cursor: pointer; \
         transition: all {trans}; opacity: {btn_opacity};",
        border = th.border(),
        radius = Theme::RADIUS_SM,
        trans = Theme::TRANSITION_FAST,
    );

    rsx! {
        button {
            title: "{tooltip}",
            style: "{btn_style}",
            disabled: !has_annotations,
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |_| on_toggle.call(()),

            // Icon
            span {
                style: "font-size: 14px;",
                "📊"
            }

            // Mode label
            span {
                "{mode.label()}"
            }
        }
    }
}

/// Compact annotation indicator (for status bar)
#[component]
pub fn AnnotationStatusIndicator(
    voltage_count: usize,
    current_count: usize,
    is_stale: bool,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let total = voltage_count + current_count;
    if total == 0 {
        return rsx! {};
    }

    let opacity = if is_stale { "0.5" } else { "1.0" };
    let text_secondary = th.text_secondary();
    let text_muted = th.text_muted();
    let voltage_color = AnnotationKind::Voltage.color();
    let current_color = AnnotationKind::Current.color();

    let container_style = format!(
        "display: inline-flex; align-items: center; gap: 8px; font-size: 11px; \
         color: {text_secondary}; opacity: {opacity};"
    );

    rsx! {
        span {
            style: "{container_style}",

            if voltage_count > 0 {
                span {
                    style: "color: {voltage_color};",
                    "V: {voltage_count}"
                }
            }

            if current_count > 0 {
                span {
                    style: "color: {current_color};",
                    "I: {current_count}"
                }
            }

            if is_stale {
                span {
                    style: "color: {text_muted}; font-style: italic;",
                    "(stale)"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;

    #[test]
    fn test_annotation_props() {
        let annotations = vec![
            Annotation::voltage(Point::new(10, 20), 3.3, "N001".to_string()),
            Annotation::current(Point::new(30, 40), 0.001, "R1".to_string()),
        ];

        assert_eq!(annotations.len(), 2);
        assert_eq!(annotations[0].kind, AnnotationKind::Voltage);
        assert_eq!(annotations[1].kind, AnnotationKind::Current);
    }
}
