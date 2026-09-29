//! Capture authoritative working content without owning editor sessions.

use crate::lifecycle::ProjectLifecycleError;
use crate::results::ProjectSimulationResults;
use crate::{ProjectExecutionContext, ProjectFile, ProjectLibraries, ProjectWorkspace};
use rspice_app_types::product::ContentDigest;
use rspice_design::{
    library::ViewType,
    schematic::owned::{CancelledOperation, Schematic},
};
use rspice_results::result_presentation::ResultPresentation;

/// Saves capture committed edits; dirty comparison and execution provenance
/// include the current preview without changing the live working set.
#[derive(Clone, Copy)]
pub enum SnapshotContent {
    Committed,
    Current,
}

/// The caller keeps snapshot editor sessions paired with captured designs.
/// These hooks affect only the caller's cloned session, never its live editors.
pub trait SnapshotSessions {
    fn replace_active(&mut self, key: &str);
    fn reconcile_cancelled_operation(
        &mut self,
        key: &str,
        design: &mut Schematic,
        cancelled: Option<CancelledOperation>,
    );
    fn mark_all_clean(&mut self);
    fn strip_schematic_runtime(&mut self, key: &str);
}

/// Borrowed working authorities. The active schematic supersedes its buffered
/// copy only while a schematic or testbench view is active.
pub struct ProjectWorkingSet<'a> {
    pub workspace: &'a ProjectWorkspace,
    pub active_schematic: &'a Schematic,
    pub libraries: &'a ProjectLibraries,
}

impl ProjectWorkingSet<'_> {
    /// Freeze designs first, then lazily capture retained results, execution
    /// inputs and presentation in order. A failed input capture stops before
    /// presentation; only a fully validated file is returned.
    pub fn capture(
        self,
        content: SnapshotContent,
        sessions: &mut impl SnapshotSessions,
        results: impl FnOnce() -> ProjectSimulationResults,
        execution_context: impl FnOnce() -> Result<ProjectExecutionContext, ProjectLifecycleError>,
        presentation: impl FnOnce() -> ResultPresentation,
    ) -> Result<ProjectFile, ProjectLifecycleError> {
        let mut workspace = self.workspace.clone();
        if matches!(
            workspace.active_view_type(),
            ViewType::Schematic | ViewType::Testbench
        ) {
            let key = workspace.active_key();
            workspace
                .schematic_buffers
                .insert(key.clone(), self.active_schematic.clone());
            sessions.replace_active(&key);
        }
        if matches!(content, SnapshotContent::Committed) {
            for (key, design) in &mut workspace.schematic_buffers {
                let cancelled = design.cancel_operation();
                sessions.reconcile_cancelled_operation(key, design, cancelled);
            }
        }
        workspace.mark_all_clean();
        sessions.mark_all_clean();
        for (key, design) in &mut workspace.schematic_buffers {
            sessions.strip_schematic_runtime(key);
            design.strip_runtime_connections_for_save();
        }
        let mut libraries = self.libraries.clone();
        libraries.sanitize_views_for_persistence();
        let simulation_results = results();
        let execution_context = execution_context()?;
        let project = ProjectFile::new_with_execution_context(
            workspace,
            libraries,
            simulation_results,
            execution_context,
        )
        .with_result_presentation(presentation());
        project
            .validate()
            .map_err(|error| ProjectLifecycleError::InvalidState(error.to_string()))?;
        project
            .simulation_results
            .validate()
            .map_err(ProjectLifecycleError::InvalidState)?;
        Ok(project)
    }
}

impl ProjectFile {
    /// Identity of authoritative schematic generator inputs. Retained results,
    /// plot annotations, validation receipts and independently owned source
    /// decks are excluded; project sources, model bindings and plans remain.
    pub fn generated_netlist_input_digest(
        mut self,
    ) -> Result<ContentDigest, ProjectLifecycleError> {
        self.simulation_results = ProjectSimulationResults::default();
        if let Some(context) = self.execution_context.as_mut() {
            context.model_validation_receipt = None;
        }
        self.result_presentation = Default::default();
        self.workspace.netlist_source = None;
        self.workspace.netlist_source_path = None;
        self.workspace.netlist_document = None;
        self.workspace.netlist_descriptor = None;
        self.workspace.retained_netlist_decks.clear();
        crate::registry::content_digest(&self).map_err(ProjectLifecycleError::InvalidState)
    }
}

#[cfg(test)]
mod tests;
