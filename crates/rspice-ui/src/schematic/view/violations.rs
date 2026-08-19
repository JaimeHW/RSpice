//! DRC/ERC violation markers on the canvas.
//!
//! After a check runs, every violation with a resolvable location gets a
//! screen-constant badge at the offending spot — a severity-colored
//! triangle with an exclamation tick. Hovering a badge shows the message
//! and the suggested fix. Markers hide as soon as the topology changes
//! (stale markers lie); the docbar pill flips to "ERC stale" instead.

use egui::{Painter, Pos2, Shape, Stroke, pos2, vec2};

use crate::diagnostics::ConsoleMessage;
use crate::services::drc::{DrcLocation, DrcSeverity, DrcViolation};
use crate::state::Point;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::active_palette;
use crate::workbench::app_state::AppState;

use super::sheet_visibility::object_is_on_active_sheet;
use super::viewport::Viewport;

/// Jump the view to the next (`step` = 1) or previous (−1) finding,
/// ordered worst severity first. Selects the offending object where one
/// exists and centers the canvas on the anchor; node/global findings have
/// no spot and stay in the console and pill.
pub(crate) fn cycle_violation(state: &mut AppState, step: isize) {
    let Some(result) = &state.dialogs.drc_results else {
        state.push_user_message(ConsoleMessage::info(
            "No check results — run design checks first",
        ));
        return;
    };
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        state.push_user_message(ConsoleMessage::warning(
            "Check results are stale — re-run design checks",
        ));
        return;
    }

    // Owned snapshot of the anchored findings so the borrows drop before
    // the selection/viewport mutations below.
    let mut anchored: Vec<(DrcSeverity, crate::diagnostics::LogAnchor, String)> = result
        .violations()
        .iter()
        .filter_map(|violation| {
            finding_anchor(state, violation)
                .map(|anchor| (violation.severity, anchor, violation.message.clone()))
        })
        .collect();
    if anchored.is_empty() {
        state.push_user_message(ConsoleMessage::info(
            "No findings with a canvas anchor — see the console summary",
        ));
        return;
    }
    // Worst first; stable within a severity.
    anchored.sort_by(|a, b| b.0.cmp(&a.0));

    let len = anchored.len() as isize;
    let index = match state.dialogs.drc_cycle {
        // First jump lands on the worst finding, not the second one.
        None if step >= 0 => 0,
        None => (len - 1) as usize,
        Some(current) => ((current as isize + step).rem_euclid(len)) as usize,
    };
    state.dialogs.drc_cycle = Some(index);

    let (severity, anchor, message) = anchored.swap_remove(index);
    state.jump_to_log_anchor(anchor);
    state.push_user_message(ConsoleMessage::info(format!(
        "Finding {}/{} — {} · {}",
        index + 1,
        len,
        severity.display_name(),
        message,
    )));
}

/// What a cycled finding selects on arrival.
/// Resolve a finding to a console jump target — shared by the check
/// runner's per-finding rows and anything else that wants click-to-source.
pub(crate) fn finding_anchor(
    state: &AppState,
    violation: &DrcViolation,
) -> Option<crate::diagnostics::LogAnchor> {
    location_anchor(state, &violation.location)
}

/// Where one finding location points, when it points anywhere.
///
/// Every surface that offers "go to this finding" resolves through here — the
/// canvas badges, the console's per-finding rows, and the validated-save
/// report, which carries a location rather than a whole violation. One
/// resolver keeps them from sending the author to three different places for
/// one location.
pub(crate) fn location_anchor(
    state: &AppState,
    location: &DrcLocation,
) -> Option<crate::diagnostics::LogAnchor> {
    if let DrcLocation::SymbolPin {
        reference,
        pin_name,
        point,
    } = location
    {
        return Some(crate::diagnostics::LogAnchor::Symbol {
            reference: reference.clone(),
            pin_name: pin_name.clone(),
            point: *point,
        });
    }

    let world = anchor(state, location)?;
    let (component, wire) = match location {
        DrcLocation::Component { id, .. } => (Some(*id), None),
        DrcLocation::Wire { id } => (None, Some(*id)),
        _ => (None, None),
    };
    Some(crate::diagnostics::LogAnchor::Schematic {
        x: world.x,
        y: world.y,
        component,
        wire,
    })
}

/// Marker half-extent in screen px (badges do not scale with zoom — they
/// are annotations, not geometry).
const HALF: f32 = 7.0;

/// Draw violation badges and the hover card for the closest hovered one.
pub(super) fn draw_violation_markers(painter: &Painter, viewport: &Viewport, state: &AppState) {
    let Some(result) = &state.dialogs.drc_results else {
        return;
    };
    // Stale results stay in the dialog/console but vanish from the canvas.
    if state.dialogs.drc_checked_version != state.schematic.topology_version() {
        return;
    }

    let palette = active_palette();
    let cursor = cursor_screen_pos(state, viewport);
    let mut hovered: Option<(Pos2, &DrcViolation)> = None;

    for violation in result.violations() {
        let Some(world) = anchor(state, &violation.location) else {
            continue;
        };
        let pos = viewport.schematic_to_screen(world);
        if !viewport.bounds.expand(HALF * 2.0).contains(pos) {
            continue;
        }

        let color = severity_color(violation.severity, &palette);
        draw_badge(painter, pos, color, palette.canvas_bg);

        if let Some(cursor) = cursor
            && cursor.distance(pos) <= HALF + 4.0
        {
            hovered = Some((pos, violation));
        }
    }

    if let Some((pos, violation)) = hovered {
        draw_hover_card(painter, viewport, pos, violation);
    }
}

fn severity_color(severity: DrcSeverity, palette: &crate::ui::palette::Palette) -> egui::Color32 {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => palette.err,
        DrcSeverity::Warning => palette.warn,
        DrcSeverity::Info => palette.text_dim,
    }
}

/// Resolve a violation to a schematic-space anchor, when it has one.
/// `Node` and `Global` violations have no single spot — they live in the
/// console summary and the ERC pill only.
fn anchor(state: &AppState, location: &DrcLocation) -> Option<Point> {
    match location {
        DrcLocation::Point { x, y } => Some(Point::new(*x as i32, *y as i32)),
        DrcLocation::Component { id, .. } => state
            .schematic
            .components
            .iter()
            .find(|component| component.id == *id && object_is_on_active_sheet(state, component.id))
            .map(|c| c.pos),
        DrcLocation::Wire { id } => state
            .schematic
            .wires
            .iter()
            .find(|wire| wire.id == *id && object_is_on_active_sheet(state, wire.id))
            .and_then(|w| {
                let first = w.points.first()?;
                let second = w.points.get(1).unwrap_or(first);
                Some(Point::new(
                    ((i64::from(first.x) + i64::from(second.x)) / 2) as i32,
                    ((i64::from(first.y) + i64::from(second.y)) / 2) as i32,
                ))
            }),
        DrcLocation::Bus { id } => state
            .schematic
            .buses
            .iter()
            .find(|bus| bus.id == *id && object_is_on_active_sheet(state, bus.id))
            .and_then(|bus| bus.points.first().copied()),
        DrcLocation::BusTap { id } => state
            .schematic
            .bus_taps
            .iter()
            .find(|tap| tap.id == *id && object_is_on_active_sheet(state, tap.id))
            .map(|tap| tap.connection_point),
        DrcLocation::NetLabel { name } => state
            .schematic
            .net_labels
            .iter()
            .find(|label| label.name == *name && object_is_on_active_sheet(state, label.id))
            .map(|l| l.pos),
        DrcLocation::Node { .. } | DrcLocation::Global | DrcLocation::SymbolPin { .. } => None,
    }
}

/// The cursor in screen space, derived from the grid-unit hover the canvas
/// reports each frame.
fn cursor_screen_pos(state: &AppState, viewport: &Viewport) -> Option<Pos2> {
    let (gx, gy) = state.ui.canvas_hover?;
    let grid = state.schematic.grid_size.max(1) as f32;
    Some(pos2(
        viewport.bounds.min.x + viewport.offset.x + (gx as f32) * grid * viewport.zoom,
        viewport.bounds.min.y + viewport.offset.y + (gy as f32) * grid * viewport.zoom,
    ))
}

/// A warning triangle with an exclamation tick, glyph in the canvas color
/// for contrast against the severity fill.
fn draw_badge(painter: &Painter, center: Pos2, fill: egui::Color32, ink: egui::Color32) {
    let points = vec![
        center + vec2(0.0, -HALF),
        center + vec2(HALF, HALF * 0.78),
        center + vec2(-HALF, HALF * 0.78),
    ];
    painter.add(Shape::convex_polygon(points, fill, Stroke::new(1.0, ink)));
    painter.line_segment(
        [center + vec2(0.0, -3.0), center + vec2(0.0, 1.0)],
        Stroke::new(1.5, ink),
    );
    painter.circle_filled(center + vec2(0.0, 3.4), 0.9, ink);
}

/// The hover card: severity + violation type, the message, and the
/// suggested fix, in an elevated box clamped to the canvas.
fn draw_hover_card(painter: &Painter, viewport: &Viewport, marker: Pos2, violation: &DrcViolation) {
    let palette = active_palette();

    let title = format!(
        "{} · {}",
        violation.severity.display_name(),
        violation.violation_type.description()
    );
    let fix = format!("Fix: {}", violation.violation_type.suggested_fix());

    let title_galley = painter.layout_no_wrap(
        title,
        theme::mono(11.0, FontWeight::Medium),
        severity_color(violation.severity, &palette),
    );
    let message_galley = painter.layout_no_wrap(
        truncate(&violation.message, 78),
        theme::sans(11.0, FontWeight::Regular),
        palette.text,
    );
    let fix_galley = painter.layout_no_wrap(
        truncate(&fix, 78),
        theme::sans(11.0, FontWeight::Regular),
        palette.text_faint,
    );

    let (pad_x, pad_y, line_gap) = (9.0, 7.0, 4.0);
    let width = title_galley
        .size()
        .x
        .max(message_galley.size().x)
        .max(fix_galley.size().x)
        + pad_x * 2.0;
    let height = title_galley.size().y
        + message_galley.size().y
        + fix_galley.size().y
        + line_gap * 2.0
        + pad_y * 2.0;

    let mut origin = marker + vec2(12.0, -height - 6.0);
    let bounds = viewport.bounds;
    if origin.x + width > bounds.right() - 4.0 {
        origin.x = marker.x - width - 12.0;
    }
    origin.y = origin.y.clamp(
        bounds.top() + 4.0,
        (bounds.bottom() - height - 4.0).max(bounds.top() + 4.0),
    );

    let rect = egui::Rect::from_min_size(origin, vec2(width, height));
    painter.rect(
        rect,
        3.0,
        palette.bg_elevated,
        Stroke::new(1.0, palette.border_strong),
        egui::StrokeKind::Inside,
    );

    let mut y = origin.y + pad_y;
    for galley in [title_galley, message_galley, fix_galley] {
        let galley_height = galley.size().y;
        painter.galley(pos2(origin.x + pad_x, y), galley, palette.text);
        y += galley_height + line_gap;
    }
}

/// Hard-truncate long strings for the single-line card rows.
fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        return text.to_owned();
    }
    let mut out: String = text.chars().take(max.saturating_sub(1)).collect();
    out.push('…');
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::drc::DrcViolationType;
    use crate::state::{
        Component, ComponentType, SheetDefinition, SheetPortPolicy, SheetTemplate, Wire,
    };

    fn state_with_hidden_object(hidden_id: u64) -> AppState {
        let mut state = AppState::default();
        let key = state.workspace.active_schematic_reference().key();
        let first = state
            .workspace
            .design_management
            .bootstrap_for_cell_view(&key, "Sheet 1", [hidden_id])
            .unwrap();
        let catalog = state
            .workspace
            .design_management
            .sheet_catalog_mut(&key)
            .unwrap();
        let second = catalog
            .create_sheet(
                SheetDefinition {
                    name: "Sheet 2".to_owned(),
                    template: SheetTemplate::AnalogSchematic,
                    port_policy: SheetPortPolicy::TypedOffSheetPorts,
                    explicit_page_number: Some(2),
                },
                Some(first),
            )
            .unwrap();
        catalog
            .assign_objects(catalog.revision(), second, [hidden_id])
            .unwrap();
        catalog.set_active(first).unwrap();
        state
    }

    #[test]
    fn hidden_identity_anchor_is_suppressed_but_unowned_point_remains_visible() {
        let mut state = state_with_hidden_object(20);
        state.schematic.components.push(Component::new(
            20,
            ComponentType::Resistor,
            Point::new(10, 10),
        ));
        state
            .schematic
            .wires
            .push(Wire::segment(20, Point::origin(), Point::new(20, 0)));

        let component = DrcViolation::new(
            1,
            DrcViolationType::UnconnectedPin,
            "hidden component",
            DrcLocation::Component {
                id: 20,
                name: "R1".to_owned(),
            },
        );
        let wire = DrcViolation::new(
            2,
            DrcViolationType::DanglingWire,
            "hidden wire",
            DrcLocation::Wire { id: 20 },
        );
        let point = DrcViolation::new(
            3,
            DrcViolationType::MissingGround,
            "unowned point",
            DrcLocation::Point { x: 4.0, y: 8.0 },
        );

        assert_eq!(anchor(&state, &component.location), None);
        assert_eq!(anchor(&state, &wire.location), None);
        assert_eq!(anchor(&state, &point.location), Some(Point::new(4, 8)));
    }
}
