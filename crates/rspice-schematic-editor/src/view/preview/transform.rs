//! Painting for validated move/stretch candidates and immutable array additions.

use super::super::{
    design_notes::draw_design_note,
    documentation_shapes::draw_documentation_shape,
    drawing::{
        ConductorInk, JunctionPaint, draw_bus, draw_bus_tap, draw_component, draw_junction,
        draw_wire,
    },
    net_labels::draw_net_label,
    symbol_context::SchematicSymbolContext,
    viewport::Viewport,
};
use super::PreviewView;
use crate::{session::visibility::SchematicParameterLabelVisibility, symbols::SymbolLibrary};
use egui::{Painter, Rect, Response, Stroke, Vec2};
use rspice_design::schematic::{
    array::SchematicArrayPreview,
    bus::{Bus, BusTap},
    component::Component,
    design_note::{DesignNote, DesignNoteRenderContext},
    document::SchematicDocument,
    documentation_shape::DocumentationShape,
    net_label::{Junction, NetLabel},
    wire::Wire,
};
use std::collections::HashMap;

pub struct TransformPreviewStyle<'a> {
    pub symbol_library: Option<&'a SymbolLibrary>,
    pub symbol_context: &'a SchematicSymbolContext,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub hover_wire_vertex: Option<(i32, i32)>,
}

/// What a move or stretch candidate changes against the document it was
/// derived from: every object whose content differs, or that the candidate
/// adds.
///
/// Taken once per candidate through id maps. Searching the source for each
/// candidate object, every frame, was quadratic in the drawing's size.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct TransformCandidateDiff {
    wires: Vec<Wire>,
    buses: Vec<Bus>,
    bus_taps: Vec<BusTap>,
    components: Vec<Component>,
    junctions: Vec<Junction>,
    net_labels: Vec<NetLabel>,
    design_notes: Vec<DesignNote>,
    documentation_shapes: Vec<DocumentationShape>,
}

/// The objects of `candidate` that `source` does not hold as they are. The
/// first source object under an id is the one compared, as a search would
/// find it.
fn changed<T: PartialEq + Clone>(source: &[T], candidate: &[T], id: impl Fn(&T) -> u64) -> Vec<T> {
    let mut by_id = HashMap::with_capacity(source.len());
    for object in source {
        by_id.entry(id(object)).or_insert(object);
    }
    candidate
        .iter()
        .filter(|object| by_id.get(&id(object)).is_none_or(|held| *held != *object))
        .cloned()
        .collect()
}

impl TransformCandidateDiff {
    pub fn between(source: &SchematicDocument, candidate: &SchematicDocument) -> Self {
        Self {
            wires: changed(&source.wires, &candidate.wires, |wire| wire.id),
            buses: changed(&source.buses, &candidate.buses, |bus| bus.id),
            bus_taps: changed(&source.bus_taps, &candidate.bus_taps, |tap| tap.id),
            components: changed(&source.components, &candidate.components, |c| c.id),
            junctions: changed(&source.junctions, &candidate.junctions, |j| j.id),
            net_labels: changed(&source.net_labels, &candidate.net_labels, |l| l.id),
            design_notes: changed(&source.design_notes, &candidate.design_notes, |n| n.id),
            documentation_shapes: changed(
                &source.documentation_shapes,
                &candidate.documentation_shapes,
                |shape| shape.id,
            ),
        }
    }
}

/// Paint what a move changes, over the unchanged drawing.
pub fn draw_move_diff(
    painter: &Painter,
    viewport: &Viewport,
    view: &PreviewView<'_>,
    diff: &TransformCandidateDiff,
    style: &TransformPreviewStyle<'_>,
    view_path: impl Fn() -> String,
) {
    draw_changed_conductors(painter, viewport, diff);

    for component in &diff.components {
        draw_component(
            painter,
            viewport,
            component,
            true,
            style.symbol_library,
            style.symbol_context,
            style.parameter_labels,
        );
    }
    for junction in &diff.junctions {
        draw_junction(
            painter,
            viewport,
            junction.pos,
            JunctionPaint {
                selected: view.editor.selection.has_junction(junction.pos),
                hovered: style.hover_wire_vertex == Some((junction.pos.x, junction.pos.y)),
                // A preview draws its conductors selected.
                incident: ConductorInk::Selected,
            },
        );
    }
    for label in &diff.net_labels {
        draw_net_label(painter, viewport, label, true, false, true);
    }
    if !diff.design_notes.is_empty() {
        let path = view_path();
        let context = DesignNoteRenderContext::for_document(view.design.document, &path);
        for note in &diff.design_notes {
            draw_design_note(painter, viewport, note, &context, true, false);
        }
    }
    draw_changed_documentation_shapes(painter, viewport, diff);
}

/// Paint what a stretch changes, over the unchanged drawing.
pub fn draw_stretch_diff(painter: &Painter, viewport: &Viewport, diff: &TransformCandidateDiff) {
    draw_changed_conductors(painter, viewport, diff);
    draw_changed_documentation_shapes(painter, viewport, diff);
}

pub fn draw_array_candidate(
    painter: &Painter,
    viewport: &Viewport,
    view: &PreviewView<'_>,
    preview: &SchematicArrayPreview,
    style: &TransformPreviewStyle<'_>,
    view_path: impl Fn() -> String,
) {
    for wire in preview.wires() {
        draw_wire(painter, viewport, wire, ConductorInk::Selected);
    }
    for bus in preview.buses() {
        draw_bus(painter, viewport, bus, true);
    }
    for tap in preview.bus_taps() {
        draw_bus_tap(painter, viewport, tap, true);
    }
    for component in preview.components() {
        draw_component(
            painter,
            viewport,
            component,
            true,
            style.symbol_library,
            style.symbol_context,
            style.parameter_labels,
        );
    }
    for junction in preview.junctions() {
        draw_junction(
            painter,
            viewport,
            junction.pos,
            JunctionPaint {
                selected: view.editor.selection.has_junction(junction.pos),
                hovered: style.hover_wire_vertex == Some((junction.pos.x, junction.pos.y)),
                incident: ConductorInk::Selected,
            },
        );
    }
    for label in preview.net_labels() {
        draw_net_label(painter, viewport, label, true, false, true);
    }
    for note in preview.design_notes() {
        draw_design_note(
            painter,
            viewport,
            note,
            &DesignNoteRenderContext::for_document(view.design.document, &view_path()),
            true,
            false,
        );
    }
    for shape in preview.documentation_shapes() {
        draw_documentation_shape(painter, viewport, shape, true, false);
    }
}

fn draw_changed_conductors(painter: &Painter, viewport: &Viewport, diff: &TransformCandidateDiff) {
    for wire in &diff.wires {
        draw_wire(painter, viewport, wire, ConductorInk::Selected);
    }
    for bus in &diff.buses {
        draw_bus(painter, viewport, bus, true);
    }
    for tap in &diff.bus_taps {
        draw_bus_tap(painter, viewport, tap, true);
    }
}

fn draw_changed_documentation_shapes(
    painter: &Painter,
    viewport: &Viewport,
    diff: &TransformCandidateDiff,
) {
    for shape in &diff.documentation_shapes {
        draw_documentation_shape(painter, viewport, shape, true, false);
    }
}

pub fn draw_transform_feedback(
    painter: &Painter,
    response: &Response,
    valid: bool,
    detail: String,
) {
    let palette = rspice_ui_kit::tokens::active_palette();
    let color = if valid { palette.ok } else { palette.err };
    let clip = painter.clip_rect();
    let tooltip_width = (clip.width() * 0.7).clamp(80.0, 300.0);
    let galley = painter.layout(detail, feedback_font(), color, tooltip_width);
    let requested = response
        .hover_pos()
        .or_else(|| response.interact_pointer_pos())
        .unwrap_or_else(|| clip.center())
        + Vec2::new(12.0, 12.0);
    let background_size = galley.size() + Vec2::splat(10.0);
    let inset = clip.shrink(4.0);
    let background_min = egui::pos2(
        (requested.x - 5.0).clamp(
            inset.left(),
            (inset.right() - background_size.x).max(inset.left()),
        ),
        (requested.y - 5.0).clamp(
            inset.top(),
            (inset.bottom() - background_size.y).max(inset.top()),
        ),
    );
    let background = Rect::from_min_size(background_min, background_size);
    let position = background.min + Vec2::splat(5.0);
    painter.rect_filled(background, 5.0, palette.bg_inset.gamma_multiply(0.96));
    painter.rect_stroke(
        background,
        5.0,
        Stroke::new(1.0, color.gamma_multiply(0.75)),
        egui::StrokeKind::Inside,
    );
    painter.galley(position, galley, color);
}

/// The face [`draw_transform_feedback`] letters its readout in.
fn feedback_font() -> egui::FontId {
    rspice_ui_kit::theme::mono(
        rspice_ui_kit::tokens::FS_0,
        rspice_ui_kit::theme::FontWeight::Medium,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::schematic::component_type::ComponentType;
    use rspice_design_model::Point;

    /// Every pixel a run of `text` paints in the readout face.
    fn ink(text: &str) -> Vec<egui::Color32> {
        let size = egui::vec2(260.0, 30.0);
        let canvas = rspice_ui_kit::raster::render(size, |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(text)
                            .font(feedback_font())
                            .color(egui::Color32::WHITE),
                    );
                });
        });
        canvas
            .pixels_in(egui::Rect::from_min_size(egui::Pos2::ZERO, size))
            .collect()
    }

    /// The move, stretch and array readouts are lettered in the mono face,
    /// which has no Greek. Every character outside ASCII in them has to come
    /// out as itself rather than as the replacement box: a missing glyph does
    /// leave ink, so each one is compared with the last private-use code
    /// point, which no face carries.
    #[test]
    fn the_transform_readouts_letter_every_glyph() {
        let replacement = ink("\u{10fffd}");
        for readout in [
            "\u{0394} 20, 0 \u{b7} Orthogonal",
            "\u{0394} -10, 30 \u{b7} Rubber band",
            "4 replicas \u{00b7} pitch \u{0394} 20, 0",
            "6 radial replicas \u{00b7} center 40, 40",
        ] {
            for glyph in readout.chars().filter(|glyph| !glyph.is_ascii()) {
                let painted = ink(&glyph.to_string());
                assert!(
                    painted != replacement && painted != ink(" "),
                    "{glyph:?} in {readout:?} letters as a replacement"
                );
            }
        }
    }

    /// The diff holds exactly what a search of the source for each candidate
    /// object would call changed: moved and added objects, nothing untouched.
    #[test]
    fn the_diff_holds_moved_and_added_objects_and_nothing_untouched() {
        let source = SchematicDocument {
            components: (1..=3)
                .map(|id| {
                    Component::new(id, ComponentType::Resistor, Point::new(40 * id as i32, 0))
                })
                .collect(),
            wires: (10..=12)
                .map(|id| {
                    Wire::new(
                        id,
                        vec![
                            Point::new(0, 10 * id as i32),
                            Point::new(40, 10 * id as i32),
                        ],
                    )
                })
                .collect(),
            junctions: vec![Junction::new(20, Point::new(0, 100))],
            ..Default::default()
        };
        let mut candidate = source.clone();
        candidate.components[1].pos = Point::new(500, 0);
        candidate.wires[2].points[1] = Point::new(80, 120);
        candidate
            .wires
            .push(Wire::new(13, vec![Point::new(0, 0), Point::new(0, 40)]));
        candidate.junctions[0].pos = Point::new(0, 140);

        let diff = TransformCandidateDiff::between(&source, &candidate);

        assert_eq!(diff.components, vec![candidate.components[1].clone()]);
        assert_eq!(
            diff.wires.iter().map(|wire| wire.id).collect::<Vec<_>>(),
            vec![12, 13]
        );
        assert_eq!(diff.junctions, vec![Junction::new(20, Point::new(0, 140))]);
        assert!(diff.net_labels.is_empty() && diff.buses.is_empty());
        assert_eq!(
            TransformCandidateDiff::between(&source, &source),
            TransformCandidateDiff::default()
        );
    }
}
