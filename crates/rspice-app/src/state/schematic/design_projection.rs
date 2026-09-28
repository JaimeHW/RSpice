//! Borrow live design authority for immutable execution projections.
use crate::state::SchematicState;
use rspice_design::projection::{ProjectionSource, SchematicSource};

impl ProjectionSource for SchematicState {
    fn projection_source(&self) -> SchematicSource<'_> {
        SchematicSource {
            schematic: &self.design,
            current_file: self.current_file.as_deref(),
            read_only: self.read_only,
            modified: self.is_dirty,
        }
    }
}
