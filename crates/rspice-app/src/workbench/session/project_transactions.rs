//! The shell's transactions over the whole project, carried out where a
//! feature's own work asks for one.
//!
//! A feature holds only its part of the application, so it cannot close or
//! replace the open project itself. It hands the shell what it needs between
//! the steps of its work, and the shell runs the transaction there, in the
//! same frame and the same order as if the feature had run it, and tells the
//! feature how it went.

use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
#[cfg(doc)]
use crate::workbench::app_state::Model;
use crate::workbench::app_state::model::{LibraryRemoval, WorkspacePruned};
use crate::workbench::app_state::services_access;
use crate::workbench::documents::netlist_document::adopt_live_owned_source;
use crate::workbench::framework::requests::{self, AdoptLiveOwnedSource};
use crate::workbench::lifecycle::project_lifecycle::{
    ProjectCloseDestination, has_unsaved_changes,
};
use crate::workbench::live_session::{CloudHandles, DiscardMirror, LiveHost, MirrorApply};
use crate::workbench::workflows::project_workflow::{
    self, LiveProjectApply, NewProject, NewProjectParams, apply_live_project_snapshot,
    close_project_discard,
};

/// Replacing the open project with a new one: the project's phases over its
/// own parts, with the shell's steps between them, in the order the
/// replacement has always run.
impl AppState {
    /// Replace the session with a new project under `params`.
    ///
    /// Returns the exact refusal so a dialog can hold itself open beside the
    /// prompt rather than closing over a project that was never created.
    pub(crate) fn create_new_project_with(
        &mut self,
        params: &NewProjectParams,
    ) -> Result<(), String> {
        let announcement = project_workflow::new_project_announcement(params);
        let model_restore_errors = replace_with_new_project(self, params, announcement)?;
        project_workflow::emit_session_model_restore_errors(
            &mut self.model,
            "new project",
            model_restore_errors,
        );
        Ok(())
    }

    /// Replace the session with a new project for a netlist being opened as
    /// one, named and announced as [`project_workflow::netlist_project`] says.
    pub(crate) fn create_netlist_project(&mut self, display_name: &str) {
        let (params, announcement) = project_workflow::netlist_project(display_name);
        if let Ok(model_restore_errors) = replace_with_new_project(self, &params, announcement) {
            project_workflow::emit_session_model_restore_errors(
                &mut self.model,
                "netlist project",
                model_restore_errors,
            );
        }
    }
}

/// The replacement every creator shares. The new project is built off
/// to the side and installed with the drawing the editor opens on; the
/// editor takes that drawing, the previous project's execution context
/// is retired under `announcement`, the new project takes the session's
/// first plan, and the features let go of what they kept of the
/// previous one. Returns the model catalog's restore errors for the
/// caller to report after the announcement.
fn replace_with_new_project(
    state: &mut AppState,
    params: &NewProjectParams,
    announcement: String,
) -> Result<Vec<String>, String> {
    project_workflow::refuse_new_project(&mut state.model, &mut state.workbench, params)?;
    let NewProject {
        library_manager,
        workspace,
        model_library_manager,
        model_restore_errors,
    } = project_workflow::build_new_project(&mut state.model, params);
    let schematic = state.new_schematic_document();
    project_workflow::install_project_drawing(
        &mut state.model,
        library_manager,
        workspace,
        schematic,
    );
    state.design.schematic_snap = state.model.schematic.session.editor.snap_engine.clone();
    state.bump_active_schematic_epoch();
    state.replace_project_execution_context(announcement);
    project_workflow::settle_new_project(
        &mut state.model,
        &mut state.platform,
        model_library_manager,
    );
    services_access::reset_for_new_project(state);
    Ok(model_restore_errors)
}

impl RSpiceApp {
    /// Run the live session for this frame, with the project transactions
    /// it asks for.
    pub(crate) fn pump_live_session(&mut self) {
        pump_live_session(&mut self.state, &mut self.cloud);
    }

    /// Close the open project to make room for the live session host's as
    /// this install's mirror: through the close review when it holds unsaved
    /// work, at once otherwise. The close lets the mirror in once it lands.
    pub(crate) fn close_for_live_mirror(&mut self) {
        let state = &mut self.state;
        let destination = ProjectCloseDestination::LiveMirror;
        if has_unsaved_changes(state) {
            state
                .workbench
                .project_review_dialog
                .show_close_project(destination);
        } else {
            close_project_discard(state, destination);
        }
    }
}

/// The library catalog's changes, run by the model, with what each leaves
/// outside the model carried out in place, before anything else reads the
/// project, in the order the change once did it itself.
impl AppState {
    /// [`Model::rename_library`], then what it left.
    pub(crate) fn rename_library(
        &mut self,
        library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let (remapped, pruned) = self.model.rename_library(library, new_name)?;
        apply_workspace_pruned(self, pruned);
        Ok(remapped)
    }

    /// [`Model::delete_library`], then what it left, and only then its
    /// revision: the revision records the catalog the editor's reload leaves.
    pub(crate) fn delete_library(&mut self, library: &str) -> Result<usize, String> {
        let LibraryRemoval {
            cells,
            pruned,
            mutation,
        } = self.model.delete_library(library)?;
        apply_workspace_pruned(self, pruned);
        self.model.publish_project_library_mutation(mutation);
        Ok(cells)
    }

    /// [`Model::copy_cell`], stating what its drawing sync had to say.
    pub(crate) fn copy_cell(
        &mut self,
        src_library: &str,
        cell: &str,
        dst_library: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let (outcome, prompt) = self
            .model
            .copy_cell(src_library, cell, dst_library, new_name);
        if let Some(prompt) = prompt {
            self.prompt(prompt);
        }
        outcome
    }

    /// [`Model::rename_cell`], then what it left.
    pub(crate) fn rename_cell(
        &mut self,
        library: &str,
        cell: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let (remapped, pruned) = self.model.rename_cell(library, cell, new_name)?;
        apply_workspace_pruned(self, pruned);
        Ok(remapped)
    }

    /// [`Model::rename_view`], then what it left.
    pub(crate) fn rename_view(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
        new_name: &str,
    ) -> Result<usize, String> {
        let (remapped, pruned) = self.model.rename_view(library, cell, view, new_name)?;
        apply_workspace_pruned(self, pruned);
        Ok(remapped)
    }

    /// [`Model::prune_workspace_after_cell_deleted`], then what it left.
    pub(crate) fn prune_workspace_after_cell_deleted(&mut self, library: &str, cell: &str) {
        let pruned = self.model.prune_workspace_after_cell_deleted(library, cell);
        apply_workspace_pruned(self, pruned);
    }

    /// [`Model::prune_workspace_after_view_deleted`], then what it left.
    pub(crate) fn prune_workspace_after_view_deleted(
        &mut self,
        library: &str,
        cell: &str,
        view: &str,
    ) {
        let pruned = self
            .model
            .prune_workspace_after_view_deleted(library, cell, view);
        apply_workspace_pruned(self, pruned);
    }
}

/// Carry out what a catalog change left outside the model, in the order
/// of [`WorkspacePruned`]'s fields: the drawing sync's prompt, the
/// editor's reload of the schematic now in front, the retirement of
/// everything run against the previous design, and the Verilog-A compile
/// evidence that names a source whose owner moved.
fn apply_workspace_pruned(state: &mut AppState, pruned: WorkspacePruned) {
    let WorkspacePruned {
        prompt,
        restore_active_schematic,
        clear_execution_context,
        void_veriloga_evidence,
    } = pruned;
    if let Some(prompt) = prompt {
        state.prompt(prompt);
    }
    if restore_active_schematic {
        state.restore_active_schematic_from_workspace();
    }
    if clear_execution_context {
        state.clear_design_execution_context();
    }
    let veriloga = &mut state.code.code_workspace.veriloga;
    let names_a_moved_source = veriloga.receipt.as_ref().is_some_and(|receipt| {
        void_veriloga_evidence.contains(&receipt.compilation.token().bundle_id)
    }) || veriloga
        .pending
        .as_ref()
        .is_some_and(|pending| void_veriloga_evidence.contains(&pending.token.bundle_id));
    if names_a_moved_source {
        *veriloga = Default::default();
    }
}

/// Drive the live session for one frame: settle who this install is in the
/// session, then its inbound half, then the host's snapshot replacing a
/// guest's mirror, then its outbound half. Between the phases the shell
/// closes a mirror whose session ended without a copy kept, at once or once
/// the local run that held it has stopped, has the netlist editor adopt the
/// project source a participant's update replaced, before a snapshot can
/// replace that project, and applies the host's project snapshot.
pub(crate) fn pump_live_session(state: &mut AppState, cloud: &mut CloudHandles) {
    let CloudHandles {
        account,
        live_session,
    } = cloud;
    if let Some(DiscardMirror) = live_session.reconcile_role(&mut live_host(state), account) {
        let closed = close_project_discard(state, ProjectCloseDestination::EmptyWorkbench);
        live_session.mirror_retired(closed);
    }
    if let Some(DiscardMirror) = live_session.mirror_discard_due(&live_host(state)) {
        let closed = close_project_discard(state, ProjectCloseDestination::EmptyWorkbench);
        live_session.mirror_discard_settled(closed);
    }
    if !live_session.pump_inbound(&mut live_host(state), account) {
        return;
    }
    requests::drain(
        state,
        |state| &mut state.workbench.outbox,
        |state, AdoptLiveOwnedSource(document)| {
            adopt_live_owned_source(&mut state.code.netlist, document);
        },
    );
    if let Some(snapshot) = live_session.take_mirror_snapshot() {
        let outcome =
            match apply_live_project_snapshot(state, snapshot.bytes(), snapshot.host_label()) {
                LiveProjectApply::Applied => MirrorApply::Applied,
                LiveProjectApply::RetryLater => MirrorApply::RetryLater,
                LiveProjectApply::Rejected => MirrorApply::Rejected,
            };
        live_session.mirror_snapshot_applied(&mut live_host(state), snapshot, outcome);
    }
    live_session.pump_outbound(&mut live_host(state));
}

/// What the live session's engine reads and writes of `state` in one phase
/// of the pump. A project snapshot reads the same four parts from it as from
/// `state` itself.
fn live_host(state: &mut AppState) -> LiveHost<'_> {
    LiveHost {
        model: &mut state.model,
        workbench: &mut state.workbench,
        platform: &state.platform,
        presentation: &state.results.results,
        design: &*state.design,
        code: &*state.code,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::cloud_account::CloudAccountService;
    use crate::workbench::framework::requests::LiveMirrorEntry;
    use crate::workbench::live_session::LiveSessionEngine;

    /// A close-to-mirror transaction that outlives its session leaves the
    /// reader where every project close does: on the no-project landing.
    #[test]
    fn a_mirror_entry_that_outlives_its_session_stays_on_the_no_project_landing() {
        let mut state = AppState::default();
        assert!(close_project_discard(
            &mut state,
            ProjectCloseDestination::LiveMirror,
        ));

        pump_live_session(
            &mut state,
            &mut CloudHandles {
                account: CloudAccountService::unconfigured(),
                live_session: LiveSessionEngine::default(),
            },
        );

        assert!(!state.project.project_launcher_open);
        assert_eq!(
            state.workbench.workspace,
            crate::workbench::state::Workspace::Project
        );
        assert!(
            state.workbench.outbox.peek::<LiveMirrorEntry>().is_none(),
            "the pump consumes the stale entry"
        );
    }

    /// Assert that the body of this module's function `name` runs `steps` in
    /// order, each first found after the one before it.
    fn assert_runs_in_order(name: &str, steps: &[&str]) {
        const SOURCE: &str = include_str!("project_transactions.rs");
        let definition = format!("fn {name}(");
        let start = SOURCE
            .find(&definition)
            .unwrap_or_else(|| panic!("`{definition}` is defined here"))
            + definition.len();
        // The body ends where the next item begins.
        let end = [
            "\n    fn ",
            "\n    pub(crate) fn ",
            "\nfn ",
            "\npub(crate) fn ",
            "\nimpl ",
        ]
        .iter()
        .filter_map(|next| SOURCE[start..].find(next))
        .min()
        .map_or(SOURCE.len(), |offset| start + offset);
        let body = &SOURCE[start..end];
        let mut at = 0;
        for step in steps {
            let found = body[at..]
                .find(step)
                .unwrap_or_else(|| panic!("`{name}` runs `{step}` after the steps before it"));
            at += found + step.len();
        }
    }

    /// A new project replaces the open one in the order it always has: the
    /// refusals before anything changes, the project built off to the side,
    /// the editor's drawing installed with it, the execution context retired
    /// once the new project is in place, then its plan, then the features
    /// let go of the previous one.
    #[test]
    fn a_new_project_replaces_the_open_one_in_the_order_it_always_has() {
        assert_runs_in_order(
            "replace_with_new_project",
            &[
                "project_workflow::refuse_new_project(",
                "project_workflow::build_new_project(",
                "state.new_schematic_document()",
                "project_workflow::install_project_drawing(",
                "state.design.schematic_snap =",
                "state.bump_active_schematic_epoch()",
                "state.replace_project_execution_context(announcement)",
                "project_workflow::settle_new_project(",
                "services_access::reset_for_new_project(state)",
            ],
        );
    }

    /// What a catalog change leaves outside the model runs in the order the
    /// change once ran it itself: the prompt is stated, the editor reloads,
    /// the execution context is retired and the stale compile evidence goes.
    #[test]
    fn a_pruned_workspace_is_carried_out_in_the_order_the_change_did_it() {
        assert_runs_in_order(
            "apply_workspace_pruned",
            &[
                "state.prompt(prompt)",
                "state.restore_active_schematic_from_workspace()",
                "state.clear_design_execution_context()",
                "*veriloga = Default::default()",
            ],
        );
    }

    /// A deleted library's revision is published after the editor's reload
    /// and the execution context's retirement, as the deletion published it.
    #[test]
    fn a_library_deletion_publishes_its_revision_after_what_it_left() {
        assert_runs_in_order(
            "delete_library",
            &[
                "self.model.delete_library(library)?",
                "apply_workspace_pruned(self, pruned)",
                "self.model.publish_project_library_mutation(mutation)",
            ],
        );
    }
}
