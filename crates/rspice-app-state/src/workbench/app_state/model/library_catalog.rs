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
//!
//! Every change is also one step of the project's Undo: what it can change
//! is taken as the revision is reserved, and the step is recorded as the
//! revision is published, so it holds the work the caller carried out in
//! between as well.

mod cells;
mod copies;
mod libraries;
mod project_root;
mod properties;

pub use copies::HierarchyCopy;
pub(crate) use libraries::{conflicting_library_name, require_no_configuration_roots};

use super::Model;
use crate::state::workspace::{DeclaredModule, hdl_view, place_veriloga_view};
use crate::state::{
    CellViewRef, PreparedProjectLibraryMutation, ProjectLibraryMutation, ProjectSourceBundle,
    ProjectSourceFile, ProjectSourceId, ProjectSourceRole, ViewType,
};
use crate::workbench::app_state::design_history::LibraryOperationStart;

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
    pub mutation: PreparedLibraryOperation,
}

/// A library change made ready: the project revision it reserved, and what
/// it can change as it stood before it ran, which its step of Undo puts back.
#[must_use = "the reserved revision must be published"]
pub struct PreparedLibraryOperation {
    revision: PreparedProjectLibraryMutation,
    start: Option<Box<LibraryOperationStart>>,
}

impl std::fmt::Debug for PreparedLibraryOperation {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("PreparedLibraryOperation")
            .field("revision", &self.revision)
            .finish_non_exhaustive()
    }
}

impl Model {
    /// Publish `bundle` and a view of it for each module of `plan`: all of it, or,
    /// when the catalog refuses, none of it. `build_profile` is added to the bundle
    /// as its build settings.
    ///
    /// Each view is of the type the bundle's root file states (a `.v` root
    /// makes Verilog views, a `.vams` root Verilog-AMS ones, anything else
    /// Verilog-A), under the name its plan entry gives it.
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
        let view_type =
            ViewType::of_hdl_root(bundle.root().logical_path()).unwrap_or(ViewType::VerilogA);
        let mut libraries = self.library_manager.clone();
        let mut sources = self.workspace.content.project_sources.clone();
        for (module, reference, _) in plan {
            if *reference != first {
                bundle
                    .share_with(reference.clone())
                    .map_err(|error| error.to_string())?;
            }
            let view = hdl_view(&reference.view, view_type, Some(module))?;
            place_veriloga_view(&mut libraries, reference, view)?;
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
    /// transaction publishes any live state, and take what the transaction
    /// can change as it stands, for its step of Undo.
    pub fn preflight_project_library_mutation(
        &self,
        mutation: ProjectLibraryMutation,
    ) -> Result<PreparedLibraryOperation, String> {
        let operation = mutation.operation_label();
        if self.safe_mode.project_read_only() {
            return Err(format!(
                "{operation} is unavailable because the project is open read-only."
            ));
        }
        let start = self.start_library_operation(&mutation);
        let revision = self
            .workspace
            .content
            .project
            .prepare_library_mutation(mutation, self.library_manager.revision())
            .map_err(|error| format!("Could not reserve {operation}: {error}"))?;
        Ok(PreparedLibraryOperation { revision, start })
    }

    /// Publish the single project revision reserved immediately before a
    /// successful library membership transaction, and record the
    /// transaction as one step of Undo. The steps before it stay; a restore
    /// from a publication, which replaces the whole project, leaves none.
    pub fn publish_project_library_mutation(&mut self, prepared: PreparedLibraryOperation) {
        let PreparedLibraryOperation { revision, start } = prepared;
        let observed_library_revision = self.library_manager.revision();
        assert!(
            observed_library_revision >= revision.minimum_library_revision(),
            "library mutation did not advance the library revision"
        );
        let previous = self.workspace.content.project.revision();
        self.workspace
            .content
            .project
            .publish_library_mutation(revision, observed_library_revision)
            .expect("the library mutation receipt was fully preflighted");
        self.workspace.content.project_metadata_dirty = true;
        self.design_execution_epoch = self.design_execution_epoch.wrapping_add(1);
        self.netlist_artifacts.current_generation_input_digest = None;
        match start {
            Some(start) => {
                let published = self.workspace.content.project.revision();
                self.carry_project_history_over_revision(previous, published);
                self.record_library_operation(*start);
            }
            None => self.clear_project_design_history(),
        }
    }
}
