//! Editor activation of generated model-bound symbol fixtures.

use super::SchematicState;
#[cfg(test)]
pub use rspice_design::model_bound_symbol::store_model_bound_symbol;
pub use rspice_design::model_bound_symbol::{
    SymbolDefinitionImport, load_model_bound_symbol, materialize_symbol_document,
    prepare_symbol_construction,
};
pub use rspice_model_library::symbol::*;

/// Open the generated fixture in fresh editor state without changing its design.
pub fn build_symbol_test_fixture(
    definition: &ModelBoundSymbolDefinition,
) -> Result<SchematicState, SymbolDefinitionError> {
    let document =
        rspice_design::model_bound_symbol::build_symbol_test_fixture_document(definition)?;
    let mut schematic = SchematicState::from_document(document);
    schematic.session.is_dirty = true;
    schematic.session.editor.needs_fit = true;
    Ok(schematic)
}
