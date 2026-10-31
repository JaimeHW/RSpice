//! The project's library catalog: the libraries, cells and views the project
//! owns, and the revision every change to them publishes.
//!
//! Every change to the catalog is one project transaction. Its revision is
//! reserved before any live state changes, so a refused change leaves the
//! project exactly as it was, and published once the change is made, so the
//! project never holds a catalog its revision does not describe. Whichever
//! feature changes the catalog, the change runs here.

use super::Model;
use crate::state::{PreparedProjectLibraryMutation, ProjectLibraryMutation};

impl Model {
    /// Reserve the next project revision before a library membership
    /// transaction publishes any live state.
    pub fn preflight_project_library_mutation(
        &self,
        mutation: ProjectLibraryMutation,
    ) -> Result<PreparedProjectLibraryMutation, String> {
        let operation = mutation.operation_label();
        if self.safe_mode.project_read_only() {
            return Err(format!(
                "{operation} is unavailable because the project is open read-only."
            ));
        }
        self.library_edit_locks
            .require_mutation_authority(
                self.workspace.content.project.id(),
                self.workspace.content.project.revision(),
                self.library_manager.revision(),
                &mutation,
            )
            .map_err(|error| format!("Could not reserve {operation}: {error}"))?;
        self.workspace
            .content
            .project
            .prepare_library_mutation(mutation, self.library_manager.revision())
            .map_err(|error| format!("Could not reserve {operation}: {error}"))
    }

    /// Publish the single project revision reserved immediately before a
    /// successful library membership transaction.
    pub fn publish_project_library_mutation(&mut self, prepared: PreparedProjectLibraryMutation) {
        let observed_library_revision = self.library_manager.revision();
        assert!(
            observed_library_revision >= prepared.minimum_library_revision(),
            "library mutation did not advance the library revision"
        );
        self.workspace
            .content
            .project
            .publish_library_mutation(prepared, observed_library_revision)
            .expect("the library mutation receipt was fully preflighted");
        self.workspace.content.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        self.clear_project_design_history();
    }
}
