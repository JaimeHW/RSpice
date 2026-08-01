//! Building a model-bound symbol.
//!
//! Assembles a symbol definition from a model and a body, including the
//! test-fixture contract that lets a definition be exercised without a
//! project around it.

use super::*;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolTestFixtureAccess {
    pub port_name: String,
    pub order: usize,
    pub electrical_type: SymbolElectricalType,
    pub direction: PortDirection,
    pub ground: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolTestFixtureContract {
    pub schema_version: u32,
    pub library: String,
    pub cell: String,
    pub implementation_view: String,
    pub dut_instance_name: String,
    pub accesses: Vec<SymbolTestFixtureAccess>,
}

#[derive(Debug, Clone)]
pub struct SymbolConstructionPlan {
    pub(super) library: String,
    pub(super) cell: String,
    pub(super) before: Option<Cell>,
    pub(super) after: Cell,
    pub(super) expected_cell_json: Option<String>,
    pub(super) after_cell_json: String,
}

impl SymbolConstructionPlan {
    pub fn commit(
        self,
        library: &mut Library,
    ) -> Result<SymbolConstructionReceipt, SymbolDefinitionError> {
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
        library.cells.insert(self.cell.clone(), self.after);
        Ok(SymbolConstructionReceipt {
            library: library.name.clone(),
            cell: self.cell,
            before: self.before,
            after_cell_json: self.after_cell_json,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SymbolConstructionReceipt {
    pub library: String,
    pub cell: String,
    pub before: Option<Cell>,
    after_cell_json: String,
}

impl SymbolConstructionReceipt {
    /// Guarded inverse suitable for app-level undo/history. It refuses to
    /// overwrite a cell that has changed since this receipt was committed.
    #[cfg(test)]
    pub fn undo(self, library: &mut Library) -> Result<(), SymbolDefinitionError> {
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
        if observed.as_deref() != Some(self.after_cell_json.as_str()) {
            return Err(SymbolDefinitionError::StaleTarget(self.cell));
        }
        if let Some(before) = self.before {
            library.cells.insert(self.cell, before);
        } else {
            library.cells.remove(&self.cell);
        }
        Ok(())
    }
}
