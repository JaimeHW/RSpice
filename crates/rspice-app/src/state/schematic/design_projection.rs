//! Borrow live design authority for immutable execution projections.
use crate::state::SchematicState;
use rspice_design::projection::{ProjectionSource, SchematicSource};

impl ProjectionSource for SchematicState {
    fn projection_source(&self) -> SchematicSource<'_> {
        SchematicSource {
            schematic: &self.design,
            current_file: self.session.current_file.as_deref(),
            read_only: self.session.read_only,
            modified: self.session.is_dirty,
        }
    }
}
