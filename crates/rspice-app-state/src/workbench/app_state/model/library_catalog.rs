//! The project's library catalog: the libraries, cells and views the project
//! owns, and the revision every change to them publishes.
//!
//! Every change to the catalog is one project transaction. Its revision is
//! reserved before any live state changes, so a refused change leaves the
//! project exactly as it was, and published once the change is made, so the
//! project never holds a catalog its revision does not describe. Whichever
//! feature changes the catalog, the change runs here.
//!
//! A change that removes or renames what the open design names also leaves
//! work outside the model, which the change hands back as a
//! [`WorkspacePruned`] for the caller to carry out before anything else
//! reads the project.

mod cells;
mod libraries;

use super::Model;
use crate::state::workspace::{DeclaredModule, place_veriloga_view, veriloga_view};
use crate::state::{
    CellViewRef, PreparedProjectLibraryMutation, ProjectLibraryMutation, ProjectSourceBundle,
    ProjectSourceFile, ProjectSourceId, ProjectSourceRole,
};

/// What a change to the catalog leaves for the application to do outside
/// the model, in the order of its fields: the sync's prompt is stated, the
/// editor is brought to the schematic now in front, everything run against
/// the previous design is retired, and Verilog-A compile evidence naming a
/// moved source is dropped.
#[must_use = "a pruned workspace leaves work outside the model"]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct WorkspacePruned {
    /// What the drawing sync had to say on the status bar.
    pub prompt: Option<String>,
    /// The workspace's focus moved, so the editor reloads the active
    /// schematic from it.
    pub restore_active_schematic: bool,
    /// The design that ran is gone, so its execution context goes with it.
    pub clear_execution_context: bool,
    /// The project sources whose owner moved. Compile evidence naming one of
    /// them no longer describes its source.
    pub void_veriloga_evidence: Vec<ProjectSourceId>,
}

/// A library taken out of the catalog, its revision not yet published.
///
/// The caller carries out `pruned` first and then publishes `mutation`, as
/// the deletion did in one transaction: the revision records the catalog
/// the editor's reload leaves.
#[must_use = "the deletion's revision must be published"]
pub struct LibraryRemoval {
    /// How many cells went with the library.
    pub cells: usize,
    /// What the deletion leaves outside the model.
    pub pruned: WorkspacePruned,
    /// The revision reserved for the deletion.
    pub mutation: PreparedProjectLibraryMutation,
}

impl Model {
    /// Publish `bundle` and a view of it for each module of `plan`: all of it, or,
    /// when the catalog refuses, none of it. `build_profile` is added to the bundle
    /// as its build settings.
    pub fn publish_veriloga_cells(
        &mut self,
        plan: &[(DeclaredModule, CellViewRef, bool)],
        first_is_new: bool,
        mut bundle: ProjectSourceBundle,
        build_profile: Option<ProjectSourceFile>,
    ) -> Result<(), String> {
        let first = plan
            .first()
            .map(|(_, reference, _)| reference.clone())
            .ok_or_else(|| "The source declares no module.".to_owned())?;
        let mut libraries = self.library_manager.clone();
        let mut sources = self.workspace.content.project_sources.clone();
        for (module, reference, _) in plan {
            if *reference != first {
                bundle
                    .share_with(reference.clone())
                    .map_err(|error| error.to_string())?;
            }
            place_veriloga_view(&mut libraries, reference, veriloga_view(Some(module))?)?;
        }
        let (bundle_id, root) = (bundle.id(), bundle.root().logical_path().to_owned());
        sources
            .insert_bundle(bundle)
            .map_err(|error| error.to_string())?;
        if let Some(profile) = build_profile {
            sources
                .add_bundle_file_with_role(
                    bundle_id,
                    &root,
                    profile,
                    ProjectSourceRole::VerilogABuildProfile,
                )
                .map_err(|error| error.to_string())?;
        }
        let mutation = if first_is_new {
            ProjectLibraryMutation::CreateCell {
                library: first.library.clone(),
                cell: first.cell.clone(),
            }
        } else {
            ProjectLibraryMutation::CreateView {
                library: first.library.clone(),
                cell: first.cell.clone(),
                view: first.view.clone(),
            }
        };
        let prepared = self.preflight_project_library_mutation(mutation)?;
        self.workspace.content.project_sources = sources;
        self.workspace.content.project_sources_dirty = true;
        self.library_manager = libraries;
        self.publish_project_library_mutation(prepared);
        Ok(())
    }

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
