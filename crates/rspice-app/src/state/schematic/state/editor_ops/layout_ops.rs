//! Annotation translations used after the layout command validates every target.

use crate::state::{Point, SchematicState};

impl SchematicState {
    pub(crate) fn translate_layout_design_note(&mut self, id: u64, delta: Point) {
        self.design.translate_layout_design_note(id, delta);
    }

    pub(crate) fn translate_layout_documentation_shape(&mut self, id: u64, delta: Point) {
        self.design.translate_layout_documentation_shape(id, delta);
    }

    pub(crate) fn translate_layout_probe(&mut self, id: u64, delta: Point) {
        self.design.translate_layout_probe(id, delta);
    }
}
