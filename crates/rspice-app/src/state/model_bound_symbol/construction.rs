//! Building a model-bound symbol.
//!
//! Assembles a symbol definition from a model and a body, including the
//! test-fixture contract that lets a definition be exercised without a
//! project around it.

use super::*;

#[derive(Debug, Clone)]
pub struct SymbolConstructionPlan {
    pub(super) library: String,
    pub(super) cell: String,
    pub(super) after: Cell,
    pub(super) expected_cell_json: Option<String>,
}

impl SymbolConstructionPlan {
    pub fn commit(self, library: &mut Library) -> Result<(), SymbolDefinitionError> {
        if library.read_only {
            return Err(SymbolDefinitionError::ReadOnlyLibrary(library.name.clone()));
        }
        if library.name != self.library {
            return Err(SymbolDefinitionError::LibraryIdentityMismatch {
                expected: self.library,
                actual: library.name.clone(),
            });
        }
        let observed = library
            .get_cell(&self.cell)
            .map(serialize_cell)
            .transpose()?;
        if observed != self.expected_cell_json {
            return Err(SymbolDefinitionError::StaleTarget(self.cell));
        }
        library.cells.insert(self.cell, self.after);
        Ok(())
    }
}
