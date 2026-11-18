//! Specific changes to the persisted identities carried by placed instances.

use crate::state::{LibraryCellInstance, SchematicState};

impl SchematicState {
    pub fn rename_library_bindings(&mut self, library: &str, new_name: &str) -> usize {
        self.design.rename_library_bindings(library, new_name)
    }

    pub fn rename_cell_bindings(&mut self, library: &str, cell: &str, new_name: &str) -> usize {
        self.design.rename_cell_bindings(library, cell, new_name)
    }

    /// Bind every placement of the cell `from` names to the cell `to` names,
    /// each keeping its view.
    pub fn rebind_cell_bindings(&mut self, from: (&str, &str), to: (&str, &str)) -> usize {
        self.design.rebind_cell_bindings(from, to)
    }

    pub fn rename_view_bindings(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> usize {
        self.design
            .rename_view_bindings(library, cell, view, new_name)
    }

    pub(crate) fn restore_placement_binding(
        &mut self,
        object: u64,
        expected: &LibraryCellInstance,
    ) {
        self.design.restore_placement_binding(object, expected)
    }
}

impl SchematicState {
    pub fn remove_master_placements(&mut self, library: &str, cell: &str, view: Option<&str>) {
        let repaired = self.design.remove_master_placements(library, cell, view);
        self.remove_stale_runtime_references(&repaired);
        self.session.is_dirty = true;
    }
}

#[cfg(test)]
mod tests {
    use crate::state::{LibraryCellInstance, Point, SchematicState};

    /// A rename rewrites only the drawings that place what it renames; any
    /// other keeps its stamp, so nothing derived from its content is read
    /// again.
    #[test]
    fn a_rename_leaves_a_drawing_that_places_none_of_it_on_its_stamp() {
        let mut drawing = SchematicState::default();
        drawing.add_library_cell_component(
            Point::new(10, 10),
            LibraryCellInstance::new("parts", "amp", "schematic"),
        );
        let stamp = drawing.content_stamp();

        assert_eq!(drawing.rename_cell_bindings("parts", "leaf", "twig"), 0);
        assert_eq!(drawing.rename_library_bindings("scratch", "notes"), 0);
        assert_eq!(
            drawing.rebind_cell_bindings(("parts", "leaf"), ("blocks", "leaf")),
            0
        );
        assert_eq!(
            drawing.rename_view_bindings("parts", "amp", "symbol", "icon"),
            0
        );
        assert_eq!(
            drawing.content_stamp(),
            stamp,
            "nothing renamed, nothing written"
        );

        assert_eq!(drawing.rename_cell_bindings("parts", "amp", "buffer"), 1);
        assert_ne!(
            drawing.content_stamp(),
            stamp,
            "the renamed placement is a new content"
        );
        assert_eq!(
            drawing.document().components[0]
                .library_cell
                .as_ref()
                .map(|binding| binding.cell.as_str()),
            Some("buffer")
        );
    }
}
