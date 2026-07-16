use egui::{Painter, Rect, Stroke};

use crate::common::app::AppState;
use crate::state::{Component, OperatingPointAnnotationPolicy, Point};

use super::super::symbols::SymbolLibrary;
use super::SchematicSymbolContext;
use super::drawing::{draw_component, draw_junction, draw_wire};
use super::grid::draw_grid;
use super::viewport::Viewport;

/// Culling margin in world units: symbols extend up to ~40 units from their
/// anchor and labels overhang further; generous slack keeps pop-in impossible
/// while still rejecting everything genuinely off-screen.
const CULL_MARGIN: f32 = 160.0;
const EMPTY_HINT_MOBILE_BREAKPOINT: f32 = 460.0;
const EMPTY_HINT_DESKTOP_LINES: [&str; 3] = [
    "Empty schematic",
    "Pick a part from the left panel to place a device or source",
    "File > Open example loads a ready-to-run circuit",
];
const EMPTY_HINT_MOBILE_LINES: [&str; 4] = [
    "Empty schematic",
    "Use Library to place devices and sources",
    "The toolbar provides wiring, labels, and probes",
    "File > Open example loads a circuit",
];

pub(super) fn component_cull_bounds(
    component: &Component,
    symbol_context: &SchematicSymbolContext,
) -> (Point, Point) {
    symbol_context.component_bounds(component)
}

pub(super) fn draw_scene(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
    symbol_library: Option<&SymbolLibrary>,
    symbol_context: &SchematicSymbolContext,
) {
    painter.rect_filled(
        available,
        0.0,
        crate::ui::tokens::active_palette().canvas_bg,
    );
    draw_grid(painter, available, state);

    // First-run guidance: an empty sheet says what to do next instead of
    // presenting a silent dot field.
    if state.schematic.components.is_empty() && state.schematic.wires.is_empty() {
        draw_empty_hint(painter, available);
    }

    let preview_bounds = if state.schematic.selection_rect.is_active() {
        let (min_x, min_y, max_x, max_y) = state.schematic.selection_rect.bounds();
        Some((min_x, min_y, max_x, max_y))
    } else {
        None
    };

    // Viewport culling: only elements whose bounds intersect the visible
    // world rect are transformed and tessellated.
    let (wx0, wy0, wx1, wy1) = viewport.visible_world_rect(CULL_MARGIN);
    let cache = state.schematic.canvas_cache();

    for (index, wire) in state.schematic.wires.iter().enumerate() {
        if let Some((min, max)) = cache.and_then(|c| c.wire_bounds.get(index))
            && ((max.x as f32) < wx0
                || (min.x as f32) > wx1
                || (max.y as f32) < wy0
                || (min.y as f32) > wy1)
        {
            continue;
        }
        let mut is_selected = state.schematic.selection.wires.contains(&wire.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = wire
                .points
                .iter()
                .any(|p| p.x >= min_x && p.x <= max_x && p.y >= min_y && p.y <= max_y);
        }

        let is_highlighted = state.schematic.net_highlight.is_wire_highlighted(wire.id);
        draw_wire(painter, viewport, wire, is_selected, is_highlighted);
    }

    for component in &state.schematic.components {
        let (min, max) = component_cull_bounds(component, symbol_context);
        if (max.x as f32) < wx0
            || (min.x as f32) > wx1
            || (max.y as f32) < wy0
            || (min.y as f32) > wy1
        {
            continue;
        }
        let mut is_selected = state.schematic.selection.components.contains(&component.id);

        if !is_selected && let Some((min_x, min_y, max_x, max_y)) = preview_bounds {
            is_selected = max.x >= min_x && min.x <= max_x && max.y >= min_y && min.y <= max_y;
        }

        draw_component(
            painter,
            viewport,
            component,
            is_selected,
            symbol_library,
            symbol_context,
        );
    }

    for junction in &state.schematic.junctions {
        let (jx, jy) = (junction.pos.x as f32, junction.pos.y as f32);
        if jx < wx0 || jx > wx1 || jy < wy0 || jy > wy1 {
            continue;
        }
        draw_junction(painter, viewport, junction.pos, state);
    }

    draw_operating_point_annotations(painter, available, viewport, state);

    if let Some((hx, hy)) = state.dialogs.interaction.hover_wire_vertex {
        let hover_pos = Point::new(hx, hy);
        let is_junction = match cache {
            Some(cache) => cache.junctions.contains(&hover_pos),
            None => state.schematic.junctions.iter().any(|j| j.pos == hover_pos),
        };
        if !is_junction {
            let pos = viewport.schematic_to_screen(hover_pos);
            let radius = 3.0 * viewport.zoom;
            painter.circle_stroke(
                pos,
                radius,
                Stroke::new(
                    1.0 * viewport.zoom,
                    crate::ui::tokens::active_palette().accent,
                ),
            );
        }
    }

    // Check results last — violation badges annotate everything below.
    super::violations::draw_violation_markers(painter, viewport, state);
}

#[derive(Debug, Clone, PartialEq)]
struct OperatingPointCanvasAnnotation {
    position: Point,
    label: String,
    selected_current: bool,
}

fn wrapped_signal_name(name: &str, prefix: char) -> Option<&str> {
    let name = name.trim();
    let (head, tail) = name.split_once('(')?;
    if !head.eq_ignore_ascii_case(&prefix.to_string()) || !tail.ends_with(')') {
        return None;
    }
    let inner = tail[..tail.len() - 1].trim();
    (!inner.is_empty()).then_some(inner)
}

/// Produce annotations only from the newest completed run. The retained
/// cross-probe point map is replaced for each dispatch, so pairing it with an
/// older selected dataset would falsely attach values to a different design.
fn operating_point_annotations(state: &AppState) -> Vec<OperatingPointCanvasAnnotation> {
    let policy = state.schematic.document_policy.operating_point_annotations;
    if policy == OperatingPointAnnotationPolicy::Hidden
        || !state.simulation.cross_probe.is_populated()
        || state.simulation.cross_probe.source_topology_version
            != Some(state.schematic.topology_version())
    {
        return Vec::new();
    }
    let Some(dc_op) = state.simulation.runs.first().and_then(|run| {
        run.analyses
            .iter()
            .find_map(|analysis| analysis.dc_op.as_ref())
    }) else {
        return Vec::new();
    };

    let mut annotations = Vec::new();
    for voltage in &dc_op.node_voltages {
        if !voltage.value.is_finite() {
            continue;
        }
        let Some(net_name) = wrapped_signal_name(&voltage.name, 'V') else {
            continue;
        };
        let points = state
            .simulation
            .cross_probe
            .net_to_points
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(net_name))
            .map(|(_, points)| points);
        let Some(position) = points.and_then(|points| {
            points
                .iter()
                .copied()
                .min_by_key(|point| (point.y, point.x))
        }) else {
            continue;
        };
        annotations.push(OperatingPointCanvasAnnotation {
            position,
            label: format!(
                "{} = {} {}",
                voltage.name,
                crate::state::format_engineering(voltage.value),
                voltage.unit
            ),
            selected_current: false,
        });
    }

    if policy == OperatingPointAnnotationPolicy::VoltagesAndSelectedCurrents {
        for component in state
            .schematic
            .components
            .iter()
            .filter(|component| state.schematic.selection.has_component(component.id))
        {
            let Some(current) = dc_op.branch_currents.iter().find(|current| {
                current.value.is_finite()
                    && wrapped_signal_name(&current.name, 'I')
                        .is_some_and(|name| name.eq_ignore_ascii_case(&component.name))
            }) else {
                continue;
            };
            annotations.push(OperatingPointCanvasAnnotation {
                position: component.pos,
                label: format!(
                    "{} = {} {}",
                    current.name,
                    crate::state::format_engineering(current.value),
                    current.unit
                ),
                selected_current: true,
            });
        }
    }
    annotations
}

fn draw_operating_point_annotations(
    painter: &Painter,
    available: Rect,
    viewport: &Viewport,
    state: &AppState,
) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    for annotation in operating_point_annotations(state) {
        let anchor = viewport.schematic_to_screen(annotation.position);
        if !available.expand(12.0).contains(anchor) {
            continue;
        }
        let galley = painter.layout_no_wrap(
            annotation.label,
            theme::mono(crate::ui::tokens::FS_0, FontWeight::Medium),
            if annotation.selected_current {
                palette.info
            } else {
                palette.net_label
            },
        );
        let offset = if annotation.selected_current {
            egui::vec2(8.0, 12.0)
        } else {
            egui::vec2(7.0, -galley.size().y - 7.0)
        };
        let mut text_pos = anchor + offset;
        text_pos.x = text_pos.x.clamp(
            available.left() + 3.0,
            available.right() - galley.size().x - 3.0,
        );
        text_pos.y = text_pos.y.clamp(
            available.top() + 3.0,
            available.bottom() - galley.size().y - 3.0,
        );
        let background = Rect::from_min_size(text_pos, galley.size()).expand2(egui::vec2(4.0, 2.0));
        painter.rect_filled(background, 2.0, palette.bg_elevated);
        painter.rect_stroke(
            background,
            2.0,
            Stroke::new(1.0, palette.border),
            egui::StrokeKind::Inside,
        );
        painter.galley(text_pos, galley, palette.text);
    }
}

/// Centered get-started hint for an empty sheet.
fn draw_empty_hint(painter: &Painter, available: Rect) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    let center = available.center();
    if available.width() < EMPTY_HINT_MOBILE_BREAKPOINT {
        draw_empty_hint_mobile(painter, center);
        return;
    }
    painter.text(
        center - egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        "Empty schematic",
        theme::sans(15.0, FontWeight::Medium),
        palette.text_dim,
    );
    painter.text(
        center + egui::vec2(0.0, 2.0),
        egui::Align2::CENTER_CENTER,
        "Pick a part from the left panel to place a device or source",
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
    painter.text(
        center + egui::vec2(0.0, 22.0),
        egui::Align2::CENTER_CENTER,
        "File ▸ Open example loads a ready-to-run circuit",
        theme::sans(12.0, FontWeight::Regular),
        palette.text_faint,
    );
}

fn draw_empty_hint_mobile(painter: &Painter, center: egui::Pos2) {
    use crate::ui::theme::{self, FontWeight};

    let palette = crate::ui::tokens::active_palette();
    let lines = empty_hint_lines_for_width(EMPTY_HINT_MOBILE_BREAKPOINT - 1.0);
    let line_height = 20.0;
    let first_y = center.y - (lines.len().saturating_sub(1) as f32 * line_height) * 0.5;
    for (index, line) in lines.iter().enumerate() {
        let title = index == 0;
        painter.text(
            egui::pos2(center.x, first_y + index as f32 * line_height),
            egui::Align2::CENTER_CENTER,
            *line,
            theme::sans(
                if title { 15.0 } else { 12.0 },
                if title {
                    FontWeight::Medium
                } else {
                    FontWeight::Regular
                },
            ),
            if title {
                palette.text_dim
            } else {
                palette.text_faint
            },
        );
    }
}

fn empty_hint_lines_for_width(width: f32) -> &'static [&'static str] {
    if width < EMPTY_HINT_MOBILE_BREAKPOINT {
        &EMPTY_HINT_MOBILE_LINES
    } else {
        &EMPTY_HINT_DESKTOP_LINES
    }
}

#[cfg(test)]
fn empty_hint_estimated_width(line: &str) -> f32 {
    line.chars().count() as f32 * 7.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::AppState;
    use crate::state::{
        AnalysisResult, AnalysisType, ComponentType, DcOpResult, OperatingPointValue,
        PortDirection, PortSpec, ResolvedCellSymbol, SimulationRun, SymbolDocument, SymbolPin,
        SymbolShape,
    };
    use std::collections::HashMap;

    fn port(name: &str, direction: PortDirection) -> PortSpec {
        PortSpec {
            name: name.to_owned(),
            direction,
        }
    }

    #[test]
    fn component_cull_bounds_use_resolved_symbol_bounds() {
        let component = Component::new(1, ComponentType::CellInstance, Point::new(100, 50));
        let symbol = ResolvedCellSymbol::from_authored_document(
            SymbolDocument {
                body: vec![SymbolShape::Polyline {
                    points: vec![Point::new(80, -10), Point::new(120, 10)],
                    closed: false,
                }],
                pins: vec![SymbolPin::new(
                    "OUT",
                    PortDirection::Out,
                    Some(Point::new(120, 0)),
                )],
                ..SymbolDocument::default()
            },
            &[port("OUT", PortDirection::Out)],
        );
        let mut resolved_by_component_id = HashMap::new();
        resolved_by_component_id.insert(component.id, symbol);
        let context = SchematicSymbolContext {
            resolved_by_component_id,
            pending_library_symbol: None,
        };

        assert_eq!(
            component_cull_bounds(&component, &context),
            (Point::new(80, 10), Point::new(220, 90))
        );
    }

    #[test]
    fn phone_width_empty_hint_lines_fit_canvas() {
        let lines = empty_hint_lines_for_width(390.0);
        let safe_width = 390.0 - 32.0;

        for line in lines {
            assert!(
                empty_hint_estimated_width(line) <= safe_width,
                "{line:?} should fit within {safe_width}px"
            );
        }
    }

    fn state_with_operating_point() -> AppState {
        let mut state = AppState::default();
        let mut component = Component::new(1, ComponentType::VoltageSource, Point::new(40, 30));
        component.name = "VBIAS".to_owned();
        state.schematic.components.push(component);
        state.schematic.selection.select_component(1);

        let point = Point::new(20, 10);
        state.simulation.cross_probe.update(
            HashMap::from([(point, "OUT".to_owned())]),
            HashMap::from([("OUT".to_owned(), vec![Point::new(30, 10), point])]),
            HashMap::new(),
            state.schematic.topology_version(),
        );
        let mut run = SimulationRun::new(1);
        run.add_analysis(
            AnalysisResult::new(1, AnalysisType::DcOp, "OP").with_dc_op(DcOpResult {
                node_voltages: vec![OperatingPointValue {
                    name: "V(out)".to_owned(),
                    value: 1.25,
                    unit: "V".to_owned(),
                }],
                branch_currents: vec![OperatingPointValue {
                    name: "I(vbias)".to_owned(),
                    value: 2.0e-3,
                    unit: "A".to_owned(),
                }],
                power_dissipation: Vec::new(),
            }),
        );
        state.simulation.runs.insert(0, run);
        state
    }

    #[test]
    fn operating_point_policy_maps_latest_values_to_exact_schematic_points() {
        let state = state_with_operating_point();
        let annotations = operating_point_annotations(&state);

        assert_eq!(annotations.len(), 2);
        assert_eq!(annotations[0].position, Point::new(20, 10));
        assert!(annotations[0].label.starts_with("V(out) = 1.25"));
        assert_eq!(annotations[1].position, Point::new(40, 30));
        assert!(annotations[1].selected_current);
    }

    #[test]
    fn hidden_or_voltage_only_policy_enforces_annotation_detail() {
        let mut state = state_with_operating_point();
        state.schematic.document_policy.operating_point_annotations =
            OperatingPointAnnotationPolicy::VoltagesOnly;
        assert_eq!(operating_point_annotations(&state).len(), 1);

        state.schematic.document_policy.operating_point_annotations =
            OperatingPointAnnotationPolicy::Hidden;
        assert!(operating_point_annotations(&state).is_empty());
    }

    #[test]
    fn topology_edit_invalidates_retained_operating_point_positions() {
        let mut state = state_with_operating_point();
        assert!(!operating_point_annotations(&state).is_empty());

        state.schematic.bump_topology_version();

        assert!(operating_point_annotations(&state).is_empty());
    }
}
