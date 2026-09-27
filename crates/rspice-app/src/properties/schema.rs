//! Property schemas for the active component-editing transaction.

use crate::state::property_types::PropertyCatalog;
use rspice_app_types::property::{PropertySheet, PropertyType};
use rspice_design::schematic::component_type::ComponentType;

/// Built-in schemas and the checked parameter form of the cell being edited.
#[derive(Debug, Clone)]
pub struct PropertyEditorSchema {
    catalog: PropertyCatalog,
    cell_instance_sheet: Option<PropertySheet>,
}

impl Default for PropertyEditorSchema {
    fn default() -> Self {
        Self::new()
    }
}

impl PropertyEditorSchema {
    pub fn new() -> Self {
        Self {
            catalog: PropertyCatalog::new(),
            cell_instance_sheet: None,
        }
    }

    pub fn get(&self, comp_type: ComponentType) -> Option<&PropertySheet> {
        if comp_type == ComponentType::CellInstance {
            return self.cell_instance_sheet.as_ref();
        }
        self.catalog.get(comp_type)
    }

    /// Install the authoritative parameter form for one active cell-instance
    /// property transaction. The sheet is validated before it can replace the
    /// prior transaction, preventing malformed library metadata from leaking
    /// into the generic property renderer.
    pub(crate) fn install_cell_instance_sheet(
        &mut self,
        sheet: PropertySheet,
    ) -> Result<(), String> {
        let Some(identity) = sheet.get("name") else {
            return Err("cell-instance property sheet has no reference designator".to_owned());
        };
        if identity.prop_type != PropertyType::String || identity.read_only {
            return Err("cell-instance reference designator must be an editable string".to_owned());
        }
        for definition in sheet.iter().filter(|definition| definition.name != "name") {
            definition
                .validate(&definition.default_value)
                .map_err(|error| format!("invalid `{}` default: {error}", definition.name))?;
        }
        self.cell_instance_sheet = Some(sheet);
        Ok(())
    }

    /// Clear the transaction-scoped cell-instance schema. A later legacy or
    /// unbound cell can therefore never reuse the previous master's fields.
    pub(crate) fn clear_cell_instance_sheet(&mut self) {
        self.cell_instance_sheet = None;
    }
}

#[cfg(test)]
mod tests {
    /// The one glyph this formatter introduces has to be one the bundled face
    /// can paint, or every spec limit under a millisecond shows a box where
    /// its prefix should be.
    ///
    /// Asked by rasterizing it, not by asking the font. The face is
    /// monospaced, so a missing glyph advances exactly as far as a present
    /// one and layout width cannot tell them apart; and `Fonts::has_glyph`
    /// answers for one face rather than for the family's fallbacks, so it says
    /// "no" to characters this application demonstrably paints. What the
    /// prefix actually has to do is leave ink, and that is what is measured.
    #[test]
    fn the_micro_prefix_is_a_glyph_the_bundled_face_carries() {
        let rows = |text: &str| crate::ui::raster::glyph_ink_rows(text, crate::ui::tokens::FS_0);
        let micro = rows("µ");
        assert!(
            micro.len() >= 3,
            "the micro prefix rasterized to {} row(s) of ink: {micro:?}",
            micro.len()
        );
        // A control that is definitely not the prefix, so a harness that
        // reported ink for everything fails here rather than passing.
        assert!(rows(" ").is_empty(), "a space rasterized ink");
    }
}
