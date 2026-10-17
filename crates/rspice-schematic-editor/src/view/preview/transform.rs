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
    array::SchematicArrayPreview, design_note::DesignNoteRenderContext, document::SchematicDocument,
};

pub struct TransformPreviewStyle<'a> {
    pub symbol_library: Option<&'a SymbolLibrary>,
    pub symbol_context: &'a SchematicSymbolContext,
    pub parameter_labels: SchematicParameterLabelVisibility,
    pub hover_wire_vertex: Option<(i32, i32)>,
}

pub fn draw_move_candidate(
    painter: &Painter,
    viewport: &Viewport,
    view: &PreviewView<'_>,
    candidate: &SchematicDocument,
    style: &TransformPreviewStyle<'_>,
    view_path: impl Fn() -> String,
) {
    draw_changed_conductors(painter, viewport, view.design.document, candidate);

    for component in candidate.components.iter().filter(|candidate_component| {
        view.design
            .document
            .components
            .iter()
            .find(|component| component.id == candidate_component.id)
            != Some(*candidate_component)
    }) {
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
    for junction in candidate.junctions.iter().filter(|candidate_junction| {
        !view
            .design
            .document
            .junctions
            .iter()
            .any(|junction| junction == *candidate_junction)
    }) {
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
    for label in candidate.net_labels.iter().filter(|candidate_label| {
        view.design
            .document
            .net_labels
            .iter()
            .find(|label| label.id == candidate_label.id)
            != Some(*candidate_label)
    }) {
        draw_net_label(painter, viewport, label, true, false, true);
    }
    for note in candidate.design_notes.iter().filter(|candidate_note| {
        view.design
            .document
            .design_notes
            .iter()
            .find(|note| note.id == candidate_note.id)
            != Some(*candidate_note)
    }) {
        draw_design_note(
            painter,
            viewport,
            note,
            &DesignNoteRenderContext::for_document(view.design.document, &view_path()),
            true,
            false,
        );
    }
    draw_changed_documentation_shapes(painter, viewport, view.design.document, candidate);
}

pub fn draw_stretch_candidate(
    painter: &Painter,
    viewport: &Viewport,
    view: &PreviewView<'_>,
    candidate: &SchematicDocument,
) {
    draw_changed_conductors(painter, viewport, view.design.document, candidate);
    draw_changed_documentation_shapes(painter, viewport, view.design.document, candidate);
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

fn draw_changed_conductors(
    painter: &Painter,
    viewport: &Viewport,
    source: &SchematicDocument,
    candidate: &SchematicDocument,
) {
    for wire in candidate.wires.iter().filter(|candidate_wire| {
        source
            .wires
            .iter()
            .find(|wire| wire.id == candidate_wire.id)
            != Some(*candidate_wire)
    }) {
        draw_wire(painter, viewport, wire, ConductorInk::Selected);
    }
    for bus in candidate.buses.iter().filter(|candidate_bus| {
        source.buses.iter().find(|bus| bus.id == candidate_bus.id) != Some(*candidate_bus)
    }) {
        draw_bus(painter, viewport, bus, true);
    }
    for tap in candidate.bus_taps.iter().filter(|candidate_tap| {
        source
            .bus_taps
            .iter()
            .find(|tap| tap.id == candidate_tap.id)
            != Some(*candidate_tap)
    }) {
        draw_bus_tap(painter, viewport, tap, true);
    }
}

fn draw_changed_documentation_shapes(
    painter: &Painter,
    viewport: &Viewport,
    source: &SchematicDocument,
    candidate: &SchematicDocument,
) {
    for shape in candidate
        .documentation_shapes
        .iter()
        .filter(|candidate_shape| {
            source
                .documentation_shapes
                .iter()
                .find(|shape| shape.id == candidate_shape.id)
                != Some(*candidate_shape)
        })
    {
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
    use super::feedback_font;

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
}
