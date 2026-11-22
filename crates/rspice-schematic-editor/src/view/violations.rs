//! Read-only check markers, and where a finding's objects are drawn, on
//! whichever sheet.

use super::{design_view::DesignView, viewport::Viewport};
use egui::{Painter, Pos2, Shape, Stroke, pos2, vec2};
use rspice_design::drc::{DrcLocation, DrcSeverity, DrcViolation};
use rspice_design::schematic::document::SchematicDocument;
use rspice_design_model::Point;
use rspice_design_model::design_management::{SheetCatalog, SheetId};
use rspice_ui_kit::palette::Palette;
use rspice_ui_kit::panels::elide_chars;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::Tokens;

/// Marker half-extent in screen px (badges do not scale with zoom — they
/// are annotations, not geometry).
const HALF: f32 = 7.0;

/// Draw violation badges and the hover card for the closest hovered one.
/// The app supplies only check results current for this design view.
///
/// Every finding at one place shares one badge, in the colour of the worst
/// of them, and its hover card lists them all, worst first, as Altium lists
/// the violations stacked on an object: badges drawn one per finding stacked
/// into one triangle whose colour was whichever rule ran last, and the card
/// reached only that one.
pub fn draw_violation_markers(
    painter: &Painter,
    viewport: &Viewport,
    view: &DesignView<'_>,
    violations: &[DrcViolation],
    canvas_hover: Option<(f64, f64)>,
) {
    let palette = Tokens::get(painter.ctx()).color;
    let cursor = cursor_screen_pos(view, viewport, canvas_hover);
    let mut hovered: Option<(f32, Pos2, &BadgeGroup<'_>)> = None;

    let groups = badge_groups(view, violations);
    for group in &groups {
        let pos = viewport.schematic_to_screen(group.at);
        if !viewport.bounds.expand(HALF * 2.0).contains(pos) {
            continue;
        }

        let color = severity_color(group.worst(), &palette);
        draw_badge(painter, pos, color, palette.canvas_bg);

        if let Some(cursor) = cursor {
            let distance = cursor.distance(pos);
            if distance <= HALF + 4.0 && hovered.is_none_or(|(closest, ..)| distance < closest) {
                hovered = Some((distance, pos, group));
            }
        }
    }

    if let Some((_, pos, group)) = hovered {
        draw_hover_card(painter, viewport, pos, &group.findings, &palette);
    }
}

/// The findings one badge stands for: every finding anchored at `at`, worst
/// first, and in the order the check reported them within a severity.
#[derive(Debug)]
pub struct BadgeGroup<'a> {
    pub at: Point,
    pub findings: Vec<&'a DrcViolation>,
}

impl BadgeGroup<'_> {
    /// The severity the badge is coloured by.
    #[must_use]
    pub fn worst(&self) -> DrcSeverity {
        self.findings
            .first()
            .map_or(DrcSeverity::Info, |finding| finding.severity)
    }
}

/// The badges `violations` draw in `view`: one per place a finding is
/// anchored, in the order the check first reported a finding there.
#[must_use]
pub fn badge_groups<'a>(
    view: &DesignView<'_>,
    violations: &'a [DrcViolation],
) -> Vec<BadgeGroup<'a>> {
    let mut groups: Vec<BadgeGroup<'a>> = Vec::new();
    for violation in violations {
        let Some(at) = anchor(view, &violation.location) else {
            continue;
        };
        match groups.iter_mut().find(|group| group.at == at) {
            Some(group) => group.findings.push(violation),
            None => groups.push(BadgeGroup {
                at,
                findings: vec![violation],
            }),
        }
    }
    for group in &mut groups {
        // Stable, so findings of one severity keep the check's order.
        group
            .findings
            .sort_by_key(|finding| std::cmp::Reverse(finding.severity));
    }
    groups
}

/// Ring radius of a related-object mark in screen px, clear of the badge the
/// object may carry for a finding of its own.
const RING: f32 = HALF + 4.0;

/// Mark the other objects the current finding involves, at `points` on the
/// sheet in view: a hollow ring in the finding's severity colour, secondary
/// to the filled badge the finding itself is marked with.
pub fn draw_related_marks(
    painter: &Painter,
    viewport: &Viewport,
    points: &[Point],
    severity: DrcSeverity,
) {
    let palette = Tokens::get(painter.ctx()).color;
    let stroke = Stroke::new(1.5, severity_color(severity, &palette));
    for world in points {
        let pos = viewport.schematic_to_screen(*world);
        if viewport.bounds.expand(RING * 2.0).contains(pos) {
            painter.circle_stroke(pos, RING, stroke);
        }
    }
}

fn severity_color(
    severity: DrcSeverity,
    palette: &rspice_ui_kit::palette::Palette,
) -> egui::Color32 {
    match severity {
        DrcSeverity::Critical | DrcSeverity::Error => palette.err,
        DrcSeverity::Warning => palette.warn,
        DrcSeverity::Info => palette.text_dim,
    }
}

/// Where a location is drawn: the point, and the sheet it is drawn on.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placement {
    pub point: Point,
    /// The sheet the object is drawn on. `None` where the drawing has no
    /// sheets or no sheet claims the object, which draws it on the sheet in
    /// view.
    pub sheet: Option<SheetId>,
}

impl Placement {
    /// Whether the sheet in view draws it.
    pub fn is_in_view(&self, view: &DesignView<'_>) -> bool {
        self.sheet.is_none_or(|sheet| {
            view.sheet_catalog
                .and_then(SheetCatalog::active_sheet_id)
                .is_none_or(|active| active == sheet)
        })
    }
}

/// Resolve a violation to a schematic-space anchor on the sheet in view, when
/// it has one there.
pub fn anchor(view: &DesignView<'_>, location: &DrcLocation) -> Option<Point> {
    placement(view, location)
        .filter(|placed| placed.is_in_view(view))
        .map(|placed| placed.point)
}

/// Resolve a location to where it is drawn, on whichever sheet that is.
///
/// What follows a finding to an object on another sheet brings that sheet
/// forward first; what marks the canvas keeps to the sheet in view, through
/// [`anchor`]. One resolver serves both, so the two cannot disagree about
/// where an object is.
///
/// `Node`, `Global` and findings inside an instantiated cellview have no single
/// spot in this drawing — they live in the console summary and the ERC pill
/// only.
pub fn placement(view: &DesignView<'_>, location: &DrcLocation) -> Option<Placement> {
    let on_sheet_of = |id: u64, point: Point| Placement {
        point,
        sheet: view
            .sheet_catalog
            .and_then(|sheets| sheets.sheet_for_object(id)),
    };
    match location {
        DrcLocation::Point { x, y } => {
            projected_point(view.sheet_catalog, Point::new(*x as i32, *y as i32))
        }
        DrcLocation::Component { id, .. } => view
            .document
            .components
            .iter()
            .find(|component| component.id == *id)
            .map(|component| on_sheet_of(component.id, component.pos)),
        DrcLocation::Wire { id } => view
            .document
            .wires
            .iter()
            .find(|wire| wire.id == *id)
            .and_then(|wire| {
                let first = wire.points.first()?;
                let second = wire.points.get(1).unwrap_or(first);
                let middle = Point::new(
                    ((i64::from(first.x) + i64::from(second.x)) / 2) as i32,
                    ((i64::from(first.y) + i64::from(second.y)) / 2) as i32,
                );
                Some(on_sheet_of(wire.id, middle))
            }),
        DrcLocation::Bus { id } => view
            .document
            .buses
            .iter()
            .find(|bus| bus.id == *id)
            .and_then(|bus| Some(on_sheet_of(bus.id, *bus.points.first()?))),
        DrcLocation::BusTap { id } => view
            .document
            .bus_taps
            .iter()
            .find(|tap| tap.id == *id)
            .map(|tap| on_sheet_of(tap.id, tap.connection_point)),
        // A label is named, not numbered, and a name may be drawn on several
        // sheets: the one on the sheet in view is the one meant there.
        DrcLocation::NetLabel { name } => {
            let named = || {
                view.document
                    .net_labels
                    .iter()
                    .filter(|label| label.name == *name)
            };
            let label = named()
                .find(|label| view.object_is_visible(label.id))
                .or_else(|| named().next())?;
            Some(on_sheet_of(label.id, label.pos))
        }
        // The checks locate a terminal in the projected document, which lays
        // each sheet a stride further along; the canvas draws the sheet where
        // it is authored, so the projection's offset comes back off.
        DrcLocation::Terminal {
            component_id,
            point,
            ..
        } => view
            .document
            .components
            .iter()
            .any(|component| component.id == *component_id)
            .then(|| {
                on_sheet_of(
                    *component_id,
                    live_terminal_point(view.sheet_catalog, *component_id, *point),
                )
            }),
        // A finding inside an instantiated cellview has no coordinate in this
        // one. Placing it on the instance symbol would point the author at a
        // drawing that does not contain the defect.
        DrcLocation::InsideInstance { .. }
        | DrcLocation::Node { .. }
        | DrcLocation::Global
        | DrcLocation::CellView { .. }
        | DrcLocation::SymbolPin { .. } => None,
    }
}

/// Where a terminal the checks located on the projected document is drawn.
///
/// A drawing with no sheets is its own projection, and so is the first sheet
/// of one; every later sheet is offset, by its component's sheet.
pub fn live_terminal_point(
    sheets: Option<&rspice_design_model::design_management::SheetCatalog>,
    component_id: u64,
    projected: Point,
) -> Point {
    let Some(sheets) = sheets else {
        return projected;
    };
    let offset = rspice_design::schematic::owned::projected_sheet_offset(sheets, component_id);
    Point::new(
        projected.x.saturating_sub(offset.x),
        projected.y.saturating_sub(offset.y),
    )
}

/// Where a point the checks located on the projected document is drawn, and
/// on which sheet.
///
/// A bare point names no object to ask for its sheet, so the sheet is the one
/// the projection laid the coordinate on. A drawing with no sheets is its own
/// projection.
fn projected_point(sheets: Option<&SheetCatalog>, projected: Point) -> Option<Placement> {
    let Some(sheets) = sheets.filter(|sheets| !sheets.sheets().is_empty()) else {
        return Some(Placement {
            point: projected,
            sheet: None,
        });
    };
    let (sheet, point) = rspice_design::schematic::owned::projected_point_sheet(sheets, projected)?;
    Some(Placement {
        point,
        sheet: Some(sheet),
    })
}

/// The cursor in screen space, derived from the grid-unit hover the canvas
/// reports each frame.
fn cursor_screen_pos(
    view: &DesignView<'_>,
    viewport: &Viewport,
    canvas_hover: Option<(f64, f64)>,
) -> Option<Pos2> {
    let (gx, gy) = canvas_hover?;
    let grid = view.document.grid_size.max(1) as f32;
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

/// The hover card: for each finding at the badge, its severity and type, the
/// message, and the suggested fix, in an elevated box clamped to the canvas.
fn draw_hover_card(
    painter: &Painter,
    viewport: &Viewport,
    marker: Pos2,
    findings: &[&DrcViolation],
    palette: &Palette,
) {
    let (pad_x, pad_y, line_gap, finding_gap) = (9.0, 7.0, 4.0, 8.0);
    let mut lines = Vec::with_capacity(findings.len() * 3);
    for violation in findings {
        let title = format!(
            "{} · {}",
            violation.severity.display_name(),
            violation.violation_type.description()
        );
        let fix = format!("Fix: {}", violation.violation_type.suggested_fix());
        lines.push((
            painter.layout_no_wrap(
                title,
                theme::mono(11.0, FontWeight::Medium),
                severity_color(violation.severity, palette),
            ),
            line_gap,
        ));
        lines.push((
            painter.layout_no_wrap(
                elide_chars(&violation.message, 78),
                theme::sans(11.0, FontWeight::Regular),
                palette.text,
            ),
            line_gap,
        ));
        lines.push((
            painter.layout_no_wrap(
                elide_chars(&fix, 78),
                theme::sans(11.0, FontWeight::Regular),
                palette.text_faint,
            ),
            finding_gap,
        ));
    }
    let width = lines
        .iter()
        .map(|(galley, _)| galley.size().x)
        .fold(0.0, f32::max)
        + pad_x * 2.0;
    let gaps: f32 =
        lines.iter().map(|(_, gap)| *gap).sum::<f32>() - lines.last().map_or(0.0, |(_, gap)| *gap);
    let height = lines.iter().map(|(galley, _)| galley.size().y).sum::<f32>() + gaps + pad_y * 2.0;

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
    for (galley, gap) in lines {
        let galley_height = galley.size().y;
        painter.galley(pos2(origin.x + pad_x, y), galley, palette.text);
        y += galley_height + gap;
    }
}

/// A digest of the drawing where a finding sits: the parts, wires and pins
/// its location names and whatever is joined to them there.
///
/// Taken while a check is current and taken again after an edit, it tells a
/// finding the edit did not touch, which keeps its marker as KiCad and
/// Virtuoso keep markers until the next check, from one whose objects the
/// edit moved, rewired, renamed or removed, which the edit may have fixed.
pub fn site_fingerprint(document: &SchematicDocument, location: &DrcLocation) -> u64 {
    use std::hash::{Hash, Hasher};

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    let part = |hasher: &mut std::collections::hash_map::DefaultHasher, id: u64| {
        let component = document
            .components
            .iter()
            .find(|component| component.id == id);
        component
            .map(|component| {
                (
                    &component.name,
                    &component.value,
                    component.terminal_positions(),
                )
            })
            .hash(hasher);
    };
    // What meets at `point`: the wires through it, the pins on it, its
    // junctions and labels.
    let joined_at = |hasher: &mut std::collections::hash_map::DefaultHasher, point: Point| {
        for wire in document
            .wires
            .iter()
            .filter(|wire| wire.contains_point(point))
        {
            wire.id.hash(hasher);
        }
        for component in &document.components {
            if component
                .terminal_positions()
                .iter()
                .any(|(_, terminal)| *terminal == point)
            {
                component.id.hash(hasher);
            }
        }
        for junction in document
            .junctions
            .iter()
            .filter(|junction| junction.pos == point)
        {
            junction.id.hash(hasher);
        }
        for label in document
            .net_labels
            .iter()
            .filter(|label| label.pos == point)
        {
            (label.id, &label.name).hash(hasher);
        }
    };
    match location {
        DrcLocation::Component { id, .. } => part(&mut hasher, *id),
        DrcLocation::Terminal {
            component_id,
            point,
            ..
        } => {
            part(&mut hasher, *component_id);
            joined_at(&mut hasher, *point);
        }
        DrcLocation::Wire { id } => {
            let wire = document.wires.iter().find(|wire| wire.id == *id);
            wire.map(|wire| &wire.points).hash(&mut hasher);
            for point in wire
                .iter()
                .flat_map(|wire| [wire.points.first(), wire.points.last()])
                .flatten()
            {
                joined_at(&mut hasher, *point);
            }
        }
        DrcLocation::Point { x, y } => joined_at(&mut hasher, Point::new(*x as i32, *y as i32)),
        // Nothing of this drawing marks the rest on the canvas.
        _ => {}
    }
    hasher.finish()
}

#[cfg(test)]
mod tests;
