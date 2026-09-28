//! Annotation translations used after the layout command validates every target.

use super::Schematic;
use rspice_design_model::Point;

impl Schematic {
    pub fn translate_layout_design_note(&mut self, id: u64, delta: Point) {
        let note = self
            .document
            .design_notes
            .iter_mut()
            .find(|note| note.id == id)
            .expect("validated design-note target remains live");
        note.translate(delta);
    }

    pub fn translate_layout_documentation_shape(&mut self, id: u64, delta: Point) {
        let shape = self
            .document
            .documentation_shapes
            .iter_mut()
            .find(|shape| shape.id == id)
            .expect("validated documentation-shape target remains live");
        shape.translate(delta);
    }

    pub fn translate_layout_probe(&mut self, id: u64, delta: Point) {
        let probe = self
            .document
            .probes
            .iter_mut()
            .find(|probe| probe.id == id)
            .expect("validated probe target remains live");
        probe.position = Point::new(
            probe
                .position
                .x
                .checked_add(delta.x)
                .expect("layout preflighted probe x"),
            probe
                .position
                .y
                .checked_add(delta.y)
                .expect("layout preflighted probe y"),
        );
    }
}
