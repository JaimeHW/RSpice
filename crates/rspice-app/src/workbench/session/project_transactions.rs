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
use crate::workbench::documents::netlist_document::adopt_live_owned_source;
use crate::workbench::framework::requests::{self, AdoptLiveOwnedSource};
use crate::workbench::lifecycle::project_lifecycle::{
    ProjectCloseDestination, has_unsaved_changes,
};
use crate::workbench::live_session::{CloudHandles, DiscardMirror, LiveHost, MirrorApply};
use crate::workbench::workflows::project_workflow::{
    LiveProjectApply, apply_live_project_snapshot, close_project_discard,
};

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
        self.apply_workspace_pruned(pruned);
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
        self.apply_workspace_pruned(pruned);
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
        self.apply_workspace_pruned(pruned);
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
        self.apply_workspace_pruned(pruned);
        Ok(remapped)
    }

    /// [`Model::prune_workspace_after_cell_deleted`], then what it left.
    pub(crate) fn prune_workspace_after_cell_deleted(&mut self, library: &str, cell: &str) {
        let pruned = self.model.prune_workspace_after_cell_deleted(library, cell);
        self.apply_workspace_pruned(pruned);
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
        self.apply_workspace_pruned(pruned);
    }

    /// Carry out what a catalog change left outside the model, in the order
    /// of [`WorkspacePruned`]'s fields: the drawing sync's prompt, the
    /// editor's reload of the schematic now in front, the retirement of
    /// everything run against the previous design, and the Verilog-A compile
    /// evidence that names a source whose owner moved.
    fn apply_workspace_pruned(&mut self, pruned: WorkspacePruned) {
        let WorkspacePruned {
            prompt,
            restore_active_schematic,
            clear_execution_context,
            void_veriloga_evidence,
        } = pruned;
        if let Some(prompt) = prompt {
            self.prompt(prompt);
        }
        if restore_active_schematic {
            self.restore_active_schematic_from_workspace();
        }
        if clear_execution_context {
            self.clear_design_execution_context();
        }
        let veriloga = &mut self.code.code_workspace.veriloga;
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

    /// Assert that `steps` appear in this file in order after the definition
    /// of `name`, each first found after the one before it.
    fn assert_runs_in_order(name: &str, steps: &[&str]) {
        const SOURCE: &str = include_str!("project_transactions.rs");
        let definition = format!("fn {name}(");
        let mut at = SOURCE
            .find(&definition)
            .unwrap_or_else(|| panic!("`{definition}` is defined here"));
        for step in steps {
            let found = SOURCE[at..]
                .find(step)
                .unwrap_or_else(|| panic!("`{name}` runs `{step}` after the steps before it"));
            at += found + step.len();
        }
    }

    /// What a catalog change leaves outside the model runs in the order the
    /// change once ran it itself: the prompt is stated, the editor reloads,
    /// the execution context is retired and the stale compile evidence goes.
    #[test]
    fn a_pruned_workspace_is_carried_out_in_the_order_the_change_did_it() {
        assert_runs_in_order(
            "apply_workspace_pruned",
            &[
                "self.prompt(prompt)",
                "self.restore_active_schematic_from_workspace()",
                "self.clear_design_execution_context()",
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
                "self.apply_workspace_pruned(pruned)",
                "self.model.publish_project_library_mutation(mutation)",
            ],
        );
    }
}
