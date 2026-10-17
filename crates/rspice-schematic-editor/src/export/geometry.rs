//! SVG path geometry.
//!
//! Emits the path and shape primitives the symbol writers build on, and the
//! electrical marks and text the canvas lays out. Every position and size is
//! the canvas's own, in world units, times `units_per_world`.

use std::fmt::Write;

use egui::{Align, Align2};
use rspice_design::schematic::{
    bus::{Bus, BusTap},
    design_note::{DesignNote, DesignNoteKind, DesignNoteRenderContext},
    document::SchematicDocument,
    documentation_shape::{DocumentationShape, DocumentationShapeGeometry, arc_parameters},
    net_label::{Junction, NetLabel},
    probe::SchematicProbe,
};
use rspice_design_model::Point;

use super::SvgExportConfig;
use crate::view::{
    design_notes, drawing, lod, net_labels,
    resolved_symbol_render::{svg_text_baseline, write_text_svg},
};

/// The page rectangle grown to hold everything drawn, in SVG units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Extent {
    pub(super) min_x: f64,
    pub(super) min_y: f64,
    pub(super) max_x: f64,
    pub(super) max_y: f64,
}

impl Default for Extent {
    fn default() -> Self {
        Self {
            min_x: f64::MAX,
            min_y: f64::MAX,
            max_x: f64::MIN,
            max_y: f64::MIN,
        }
    }
}

impl Extent {
    pub(super) fn include(&mut self, x: f64, y: f64) {
        self.min_x = self.min_x.min(x);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.max_y = self.max_y.max(y);
    }

    pub(super) fn include_box(&mut self, (min_x, min_y, max_x, max_y): (f64, f64, f64, f64)) {
        self.include(min_x, min_y);
        self.include(max_x, max_y);
    }

    pub(super) const fn is_empty(&self) -> bool {
        self.min_x > self.max_x
    }

    /// The box a run of text takes when hung off `(x, y)` by `align`: its
    /// estimated advance across, and Plex's 1.3 em line box down.
    pub(super) fn include_text(
        &mut self,
        (x, y): (f64, f64),
        align: Align2,
        text: &str,
        size: f64,
    ) {
        let width = estimated_text_width(text, size);
        let left = match align.x() {
            Align::Min => x,
            Align::Center => x - width * 0.5,
            Align::Max => x - width,
        };
        let top = match align.y() {
            Align::Min => y,
            Align::Center => y - LINE_BOX_EM * size * 0.5,
            Align::Max => y - LINE_BOX_EM * size,
        };
        self.include(left, top);
        self.include(left + width, top + LINE_BOX_EM * size);
    }
}

/// Plex's line box, in em.
const LINE_BOX_EM: f64 = 1.3;

/// A world point in SVG units.
pub(super) fn page(point: Point, config: &SvgExportConfig) -> (f64, f64) {
    (
        f64::from(point.x) * config.units_per_world,
        f64::from(point.y) * config.units_per_world,
    )
}

/// A world offset from a world point, in SVG units.
fn page_offset(point: Point, offset: egui::Vec2, config: &SvgExportConfig) -> (f64, f64) {
    let (x, y) = page(point, config);
    (
        x + f64::from(offset.x) * config.units_per_world,
        y + f64::from(offset.y) * config.units_per_world,
    )
}

/// A world size in SVG units, to a millionth: the canvas's sizes are `f32`,
/// and 1.1 wu should read 11 on the page, not 11.000000238418579.
pub(super) fn world(size: f32, config: &SvgExportConfig) -> f64 {
    (f64::from(size) * config.units_per_world * 1e6).round() / 1e6
}

/// A canvas world point (`f32`) in SVG units.
fn world_point(point: egui::Pos2, config: &SvgExportConfig) -> (f64, f64) {
    (world(point.x, config), world(point.y, config))
}

pub(super) fn include_documentation_shape_bounds(
    document: &SchematicDocument,
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    for shape in &document.documentation_shapes {
        let (min, max) = shape.bounds();
        let (min_x, min_y) = page(min, config);
        let (max_x, max_y) = page(max, config);
        extent.include_box((min_x, min_y, max_x, max_y));
    }
}

pub(super) fn write_documentation_shape(
    svg: &mut String,
    shape: &DocumentationShape,
    config: &SvgExportConfig,
) {
    let kind = shape.kind().label().to_ascii_lowercase();
    writeln!(
        svg,
        "<g class=\"documentation-shape documentation-shape-{kind}\" data-object-id=\"{}\" data-layer=\"drawing-documentation\" data-kind=\"{kind}\">",
        shape.id
    )
    .unwrap();
    let scale = config.units_per_world;
    match &shape.geometry {
        DocumentationShapeGeometry::Rectangle { first, opposite } => {
            let x = f64::from(first.x.min(opposite.x)) * scale;
            let y = f64::from(first.y.min(opposite.y)) * scale;
            let width = f64::from(first.x.abs_diff(opposite.x)) * scale;
            let height = f64::from(first.y.abs_diff(opposite.y)) * scale;
            writeln!(
                svg,
                "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" rx=\"{}\"/>",
                6.0 * scale
            )
            .unwrap();
        }
        DocumentationShapeGeometry::Line { start, end } => {
            writeln!(
                svg,
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                f64::from(start.x) * scale,
                f64::from(start.y) * scale,
                f64::from(end.x) * scale,
                f64::from(end.y) * scale
            )
            .unwrap();
        }
        DocumentationShapeGeometry::Polygon { points } => {
            let points = points
                .iter()
                .map(|point| {
                    format!(
                        "{},{}",
                        f64::from(point.x) * scale,
                        f64::from(point.y) * scale
                    )
                })
                .collect::<Vec<_>>()
                .join(" ");
            writeln!(svg, "<polygon points=\"{points}\"/>").unwrap();
        }
        DocumentationShapeGeometry::Arc {
            start,
            through,
            end,
        } => {
            if let Some((_, _, radius, _, sweep)) = arc_parameters(*start, *through, *end) {
                let large_arc = u8::from(sweep.abs() > std::f64::consts::PI);
                let sweep_flag = u8::from(sweep >= 0.0);
                writeln!(
                    svg,
                    "<path d=\"M {} {} A {} {} 0 {large_arc} {sweep_flag} {} {}\"/>",
                    f64::from(start.x) * scale,
                    f64::from(start.y) * scale,
                    radius * scale,
                    radius * scale,
                    f64::from(end.x) * scale,
                    f64::from(end.y) * scale
                )
                .unwrap();
            }
        }
        DocumentationShapeGeometry::Callout {
            tip,
            elbow,
            box_corner,
        } => {
            writeln!(
                svg,
                "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\"/>",
                f64::from(tip.x) * scale,
                f64::from(tip.y) * scale,
                f64::from(elbow.x) * scale,
                f64::from(elbow.y) * scale
            )
            .unwrap();
            let x = f64::from(elbow.x.min(box_corner.x)) * scale;
            let y = f64::from(elbow.y.min(box_corner.y)) * scale;
            let width = f64::from(elbow.x.abs_diff(box_corner.x)) * scale;
            let height = f64::from(elbow.y.abs_diff(box_corner.y)) * scale;
            writeln!(
                svg,
                "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" rx=\"{}\"/>",
                6.0 * scale
            )
            .unwrap();
        }
    }
    svg.push_str("</g>\n");
}

fn design_note_export_text(
    document: &SchematicDocument,
    note: &DesignNote,
    view_path: &str,
) -> String {
    note.rendered_text(&DesignNoteRenderContext::for_document(document, view_path))
}

/// The top-left corner of a note's first line, as the canvas places it.
fn design_note_origin(note: &DesignNote, config: &SvgExportConfig) -> (f64, f64) {
    page_offset(
        note.pos,
        egui::vec2(design_notes::OFFSET_X_WORLD, design_notes::OFFSET_Y_WORLD),
        config,
    )
}

/// `context` is the whole document a note's properties read; `notes` are
/// the ones drawn.
pub(super) fn include_design_note_bounds(
    context: &SchematicDocument,
    notes: &[DesignNote],
    config: &SvgExportConfig,
    view_path: &str,
    extent: &mut Extent,
) {
    let size = world(design_notes::FONT_WORLD_SIZE, config);
    let advance = world(design_notes::LINE_ADVANCE_WORLD, config);
    for note in notes {
        let (x, top) = design_note_origin(note, config);
        for (index, line) in design_note_export_text(context, note, view_path)
            .split('\n')
            .enumerate()
        {
            extent.include_text(
                (x, top + index as f64 * advance),
                Align2::LEFT_TOP,
                line,
                size,
            );
        }
        let (anchor_x, anchor_y) = page(note.pos, config);
        let marker = world(design_notes::MARKER_RADIUS_WORLD, config);
        extent.include_box((
            anchor_x - marker,
            anchor_y - marker,
            anchor_x + marker,
            anchor_y + marker,
        ));
    }
}

pub(super) fn write_design_note(
    svg: &mut String,
    document: &SchematicDocument,
    note: &DesignNote,
    config: &SvgExportConfig,
    view_path: &str,
) {
    let size = world(design_notes::FONT_WORLD_SIZE, config);
    let advance = world(design_notes::LINE_ADVANCE_WORLD, config);
    let (x, top) = design_note_origin(note, config);
    let y = svg_text_baseline(top, Align::Min, size);
    let kind = match note.kind {
        DesignNoteKind::PlainText => "plain-text",
        DesignNoteKind::PropertyDisplay => "property-display",
        DesignNoteKind::RequirementLink => "requirement-link",
        DesignNoteKind::ReviewNote => "review-note",
    };
    let review_attributes = note.review.as_ref().map_or_else(String::new, |review| {
        format!(
            " data-review-id=\"{}\" data-review-state=\"{}\"",
            super::escape_xml(&review.record_id),
            review.state.keyword()
        )
    });
    writeln!(
        svg,
        "<g class=\"design-note design-note-{kind}\" data-object-id=\"{}\" data-layer=\"drawing-annotation\"{review_attributes}>",
        note.id
    )
    .unwrap();
    let (anchor_x, anchor_y) = page(note.pos, config);
    writeln!(
        svg,
        "<circle class=\"design-note-anchor\" cx=\"{anchor_x}\" cy=\"{anchor_y}\" r=\"{}\"/>",
        world(design_notes::MARKER_RADIUS_WORLD, config)
    )
    .unwrap();
    let text = design_note_export_text(document, note, view_path);
    writeln!(svg, "<text class=\"note\" x=\"{x}\" y=\"{y}\">").unwrap();
    for (index, line) in text.split('\n').enumerate() {
        let dy = if index == 0 { 0.0 } else { advance };
        writeln!(
            svg,
            "<tspan x=\"{x}\" dy=\"{dy}\">{}</tspan>",
            super::escape_xml(line)
        )
        .unwrap();
    }
    svg.push_str("</text>\n</g>\n");
}

/// Where a bus declaration stands: its bottom-left corner.
fn bus_declaration_corner(bus: &Bus, config: &SvgExportConfig) -> Option<(f64, f64)> {
    let anchor = *bus.points.last()?;
    bus.declaration.as_ref()?;
    Some(page_offset(
        anchor,
        drawing::BUS_DECLARATION_OFFSET_WU,
        config,
    ))
}

fn tap_slice_corner(tap: &BusTap, config: &SvgExportConfig) -> (f64, f64) {
    page_offset(tap.connection_point, drawing::TAP_SLICE_OFFSET_WU, config)
}

pub(super) fn include_bus_bounds(
    buses: &[Bus],
    taps: &[BusTap],
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    let size = world(drawing::SMALL_TEXT_WORLD, config);
    let reach = world(
        lod::BUS_STRAND_OFFSET_WU + lod::nominal_wu(lod::StrokeRole::BusStrand) * 0.5,
        config,
    );
    for bus in buses {
        for point in &bus.points {
            let (x, y) = page(*point, config);
            extent.include_box((x - reach, y - reach, x + reach, y + reach));
        }
        if let (Some(declaration), Some(corner)) =
            (&bus.declaration, bus_declaration_corner(bus, config))
        {
            extent.include_text(corner, Align2::LEFT_BOTTOM, &declaration.to_string(), size);
        }
    }

    for tap in taps {
        for point in crate::bus_geometry::bus_tap_route_points(tap) {
            let (x, y) = page(point, config);
            extent.include(x, y);
        }
        extent.include_text(
            tap_slice_corner(tap, config),
            Align2::LEFT_BOTTOM,
            &tap.slice.to_string(),
            size,
        );
    }
}

pub(super) fn junction_radius(config: &SvgExportConfig) -> f64 {
    world(lod::JUNCTION_RADIUS_WU, config)
}

pub(super) fn include_junction_bounds(
    junctions: &[Junction],
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    let radius = junction_radius(config);
    for junction in junctions {
        let (x, y) = page(junction.pos, config);
        extent.include_box((x - radius, y - radius, x + radius, y + radius));
    }
}

/// One conductor chain as one path, so its corners are joined rather than
/// two butt ends with a notch between them.
pub(super) fn write_conductor(svg: &mut String, points: &[Point], config: &SvgExportConfig) {
    let points: Vec<(f64, f64)> = points.iter().map(|point| page(*point, config)).collect();
    write_svg_path(svg, "wire", &points);
}

pub(super) fn write_bus(svg: &mut String, bus: &Bus, config: &SvgExportConfig) {
    if bus.points.len() < 2 {
        return;
    }

    let centerline: Vec<(f64, f64)> = bus
        .points
        .iter()
        .map(|point| page(*point, config))
        .collect();
    let offset = world(lod::BUS_STRAND_OFFSET_WU, config);
    for offset in [-offset, 0.0, offset] {
        write_svg_path(svg, "bus", &offset_svg_polyline(&centerline, offset));
    }
    if let (Some(declaration), Some(corner)) =
        (&bus.declaration, bus_declaration_corner(bus, config))
    {
        write_text_svg(
            svg,
            "slice bus-declaration",
            corner,
            &declaration.to_string(),
            Align2::LEFT_BOTTOM,
            world(drawing::SMALL_TEXT_WORLD, config),
        );
    }
}

pub(super) fn write_bus_tap(svg: &mut String, tap: &BusTap, config: &SvgExportConfig) {
    let route: Vec<(f64, f64)> = crate::bus_geometry::bus_tap_route_points(tap)
        .into_iter()
        .map(|point| page(point, config))
        .collect();
    write_svg_path(svg, "tap", &route);
    write_text_svg(
        svg,
        "slice bus-tap-label",
        tap_slice_corner(tap, config),
        &tap.slice.to_string(),
        Align2::LEFT_BOTTOM,
        world(drawing::SMALL_TEXT_WORLD, config),
    );
}

fn write_svg_path(svg: &mut String, class: &str, points: &[(f64, f64)]) {
    if points.len() < 2 {
        return;
    }
    write!(svg, "<path class=\"{class}\" d=\"").unwrap();
    for (index, &(x, y)) in points.iter().enumerate() {
        if index == 0 {
            write!(svg, "M {x} {y}").unwrap();
        } else {
            write!(svg, " L {x} {y}").unwrap();
        }
    }
    svg.push_str("\"/>\n");
}

fn offset_svg_polyline(points: &[(f64, f64)], offset: f64) -> Vec<(f64, f64)> {
    if points.len() < 2 || offset == 0.0 {
        return points.to_vec();
    }
    let normal = |start: (f64, f64), end: (f64, f64)| {
        let (dx, dy) = (end.0 - start.0, end.1 - start.1);
        let length = (dx * dx + dy * dy).sqrt().max(f64::EPSILON);
        (-dy / length, dx / length)
    };
    points
        .iter()
        .enumerate()
        .map(|(index, &point)| {
            let vector = if index == 0 {
                normal(points[0], points[1])
            } else if index + 1 == points.len() {
                normal(points[index - 1], points[index])
            } else {
                let previous = normal(points[index - 1], points[index]);
                let next = normal(points[index], points[index + 1]);
                let sum = (previous.0 + next.0, previous.1 + next.1);
                let length = (sum.0 * sum.0 + sum.1 * sum.1).sqrt();
                if length <= f64::EPSILON {
                    next
                } else {
                    let miter = (sum.0 / length, sum.1 / length);
                    let denominator = (miter.0 * next.0 + miter.1 * next.1).abs().max(0.25);
                    (miter.0 / denominator, miter.1 / denominator)
                }
            };
            (point.0 + vector.0 * offset, point.1 + vector.1 * offset)
        })
        .collect()
}

/// A run's advance, estimated at 0.62 em a glyph: wide enough for Plex Mono
/// and Plex Sans alike, so a page never clips a label.
pub(super) fn estimated_text_width(text: &str, size: f64) -> f64 {
    text.chars().count() as f64 * size * 0.62
}

pub(super) fn write_junction(svg: &mut String, junction: &Junction, config: &SvgExportConfig) {
    let (cx, cy) = page(junction.pos, config);
    writeln!(
        svg,
        "<circle class=\"junction\" cx=\"{cx}\" cy=\"{cy}\" r=\"{}\"/>",
        junction_radius(config)
    )
    .unwrap();
}

/// A net label's diamond, name and connector tab, placed by the canvas's
/// own net-label layout.
pub(super) fn write_net_label(svg: &mut String, label: &NetLabel, config: &SvgExportConfig) {
    let layout = net_labels::world_layout(label);
    let at = |point: egui::Pos2| world_point(point, config);
    writeln!(svg, "<g class=\"net\" data-object-id=\"{}\">", label.id).unwrap();
    let (x, y) = page(label.pos, config);
    let half = world(layout.diamond_half, config);
    writeln!(
        svg,
        "<polygon class=\"net-mark\" points=\"{x},{} {},{y} {x},{} {},{y}\"/>",
        y - half,
        x + half,
        y + half,
        x - half
    )
    .unwrap();
    write_text_svg(
        svg,
        "net-name",
        at(layout.corner),
        layout.name,
        Align2::LEFT_BOTTOM,
        world(layout.font_size, config),
    );
    if let Some((tab, direction)) = layout.tab {
        let (left, top) = at(tab.min);
        writeln!(
            svg,
            "<g class=\"net-tab\" data-direction=\"{direction:?}\" style=\"stroke-width:{}\">",
            world(
                tab.height() * net_labels::CONNECTOR_TAB_STROKE_PER_HEIGHT,
                config
            )
        )
        .unwrap();
        writeln!(
            svg,
            "<rect x=\"{left}\" y=\"{top}\" width=\"{}\" height=\"{}\" rx=\"{}\"/>",
            world(tab.width(), config),
            world(tab.height(), config),
            config.units_per_world
        )
        .unwrap();
        let mut data = String::new();
        for [start, end] in net_labels::connector_tab_marks(tab, direction) {
            let (x1, y1) = at(start);
            let (x2, y2) = at(end);
            write!(data, "M {x1} {y1} L {x2} {y2} ").unwrap();
        }
        writeln!(svg, "<path d=\"{}\"/>", data.trim_end()).unwrap();
        svg.push_str("</g>\n");
    }
    svg.push_str("</g>\n");
}

pub(super) fn include_net_label_bounds(
    labels: &[NetLabel],
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    for label in labels {
        let layout = net_labels::world_layout(label);
        let (x, y) = page(label.pos, config);
        let half = world(layout.diamond_half, config);
        extent.include_box((x - half, y - half, x + half, y + half));
        extent.include_text(
            world_point(layout.corner, config),
            Align2::LEFT_BOTTOM,
            layout.name,
            world(layout.font_size, config),
        );
        if let Some((tab, _)) = layout.tab {
            extent.include_box((
                world(tab.min.x, config),
                world(tab.min.y, config),
                world(tab.max.x, config),
                world(tab.max.y, config),
            ));
        }
    }
}

/// A probe's ring, crosshair and reference, placed as the canvas places
/// them. A printed sheet carries no run, so the reference has no status.
pub(super) fn write_probe(svg: &mut String, probe: &SchematicProbe, config: &SvgExportConfig) {
    let (x, y) = page(probe.position, config);
    let radius = world(drawing::PROBE_RADIUS, config);
    let span = world(drawing::PROBE_CROSSHAIR_HALF_SPAN, config);
    writeln!(svg, "<g class=\"probe\" data-object-id=\"{}\">", probe.id).unwrap();
    writeln!(svg, "<circle cx=\"{x}\" cy=\"{y}\" r=\"{radius}\"/>").unwrap();
    writeln!(
        svg,
        "<path d=\"M {} {y} L {} {y} M {x} {} L {x} {}\"/>",
        x - span,
        x + span,
        y - span,
        y + span
    )
    .unwrap();
    write_text_svg(
        svg,
        "probe-label",
        page_offset(probe.position, drawing::PROBE_LABEL_OFFSET_WU, config),
        &probe.reference,
        Align2::LEFT_BOTTOM,
        world(drawing::SMALL_TEXT_WORLD, config),
    );
    svg.push_str("</g>\n");
}

pub(super) fn include_probe_bounds(
    probes: &[SchematicProbe],
    config: &SvgExportConfig,
    extent: &mut Extent,
) {
    for probe in probes {
        let (x, y) = page(probe.position, config);
        let span = world(drawing::PROBE_CROSSHAIR_HALF_SPAN, config);
        extent.include_box((x - span, y - span, x + span, y + span));
        extent.include_text(
            page_offset(probe.position, drawing::PROBE_LABEL_OFFSET_WU, config),
            Align2::LEFT_BOTTOM,
            &probe.reference,
            world(drawing::SMALL_TEXT_WORLD, config),
        );
    }
}
