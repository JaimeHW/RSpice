//! The shell's transactions over the whole project, carried out where a
//! feature's own work asks for one.
//!
//! A feature holds only its part of the application, so it cannot close or
//! replace the open project itself. It hands the shell what it needs between
//! the steps of its work, and the shell runs the transaction there, in the
//! same frame and the same order as if the feature had run it, and tells the
//! feature how it went.

#[cfg(test)]
use crate::diagnostics::{LogSeverity, LogSource};
use crate::io::ProjectSnapshot;
use crate::product::TransactionId;
use crate::state::{SchematicState, ViewType};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
#[cfg(doc)]
use crate::workbench::app_state::Model;
#[cfg(test)]
use crate::workbench::app_state::model::LibraryRemoval;
use crate::workbench::app_state::model::WorkspacePruned;
use crate::workbench::app_state::services_access;
use crate::workbench::documents::netlist_document::adopt_live_owned_source;
use crate::workbench::framework::requests::{self, AdoptLiveOwnedSource};
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::lifecycle::project_lifecycle::DestinationAuthority;
use crate::workbench::lifecycle::project_lifecycle::SaveScope;
use crate::workbench::lifecycle::project_lifecycle::{
    LifecycleParts, PersistenceBinding, ProjectCloseDestination, RevertReviewToken,
    has_unsaved_changes,
};
use crate::workbench::live_session::{CloudHandles, DiscardMirror, LiveHost, MirrorApply};
use crate::workbench::state::Workspace;
#[cfg(target_arch = "wasm32")]
use crate::workbench::workflows::file_workflow::BrowserSchematic;
use crate::workbench::workflows::file_workflow::{self, SchematicLoadOrigin};
#[cfg(target_arch = "wasm32")]
use crate::workbench::workflows::project_workflow::BrowserProjectOpen;
use crate::workbench::workflows::project_workflow::{
    self, ClosedProject, LiveProjectApply, LoadedProject, NewProject, NewProjectParams,
    OpenedProject, ProjectLoadOrigin, SaveParts, SaveRequestOutcome, apply_live_project_snapshot,
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

/// Replacing the drawing in front with a schematic file's, on the
/// no-project screen: the project's phases over its own parts, with the
/// shell's steps between them, in the order the replacement has always run.
impl AppState {
    /// Apply a schematic read from `origin`: inside a project it is added
    /// as a new cell view, and otherwise its drawing replaces the one in
    /// front. Returns whether it was applied.
    pub(crate) fn apply_loaded_schematic(
        &mut self,
        schematic: SchematicState,
        origin: SchematicLoadOrigin<'_>,
    ) -> bool {
        // Inside a project a schematic file is added to it, never poured over
        // the cell view the user has open.
        if self.platform.project_lifecycle.is_open() {
            return file_workflow::import_loaded_schematic(self, schematic, origin);
        }
        let schematic =
            file_workflow::prepare_loaded_schematic(&mut self.platform, schematic, origin);
        replace_active_schematic_document(self, schematic);
        self.clear_design_execution_context();
        file_workflow::settle_loaded_schematic(&mut self.model, &mut self.platform, origin);
        true
    }

    /// Open the schematic the browser read, its file's name kept as the next
    /// save's suggestion. Returns whether it was applied.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open_browser_schematic(&mut self, opened: BrowserSchematic) -> bool {
        self.apply_loaded_schematic(
            opened.schematic,
            SchematicLoadOrigin::BrowserImport(&opened.name),
        )
    }

    /// Reset the current schematic to a new empty document.
    #[cfg(test)]
    pub(crate) fn create_new_schematic(&mut self) {
        let schematic = self.new_schematic_document();
        replace_active_schematic_document(self, schematic);
        self.clear_design_execution_context();
        self.record(
            LogSeverity::Info,
            LogSource::Studio,
            "Created a new schematic.",
        );
    }
}

/// Make `schematic` the drawing in front, under the session's snap settings,
/// with the workspace's copy of it kept in step.
fn replace_active_schematic_document(state: &mut AppState, mut schematic: SchematicState) {
    // Loaded document data owns the grid pitch; the active device session owns
    // snap targets and radius. Install both authorities before publishing the
    // new active buffer, then mirror the derived pitch back into the session.
    schematic.session.editor.snap_engine = state.design.schematic_snap.clone();
    schematic.reconcile_grid_pitch_runtime();
    state.model.schematic = schematic;
    state.design.schematic_snap = state.model.schematic.session.editor.snap_engine.clone();
    state.bump_active_schematic_epoch();
    match state.model.workspace.content.active_view_type() {
        ViewType::Schematic | ViewType::Testbench => {
            state.sync_active_schematic_to_workspace();
        }
        _ => {
            let reference = state.model.workspace.content.active_schematic_reference();
            let dirty = state.model.schematic.session.is_dirty;
            state
                .model
                .workspace
                .insert_schematic_editor(reference.key(), state.model.schematic.clone());
            if let Some(open_view) = state
                .model
                .workspace
                .content
                .open_views
                .iter_mut()
                .find(|open_view| open_view.reference == reference)
            {
                open_view.dirty = dirty;
            }
        }
    }
}

/// Closing the open project without saving it, and opening the project the
/// restored session holds at startup: the project's phases over its own
/// parts, with the shell's steps between them, in the order each has always
/// run.
impl AppState {
    /// Close the open project without saving it, leaving the reader at
    /// `destination`. Returns whether it closed.
    pub(crate) fn close_project_discard(&mut self, destination: ProjectCloseDestination) -> bool {
        close_project_discard(self, destination)
    }

    /// Bind the project the restored session holds, then apply Projects and
    /// files ▸ On startup. Called once the session and preferences are
    /// restored, because the choice is read from them. A session saved after
    /// Close Project starts with no project whichever is chosen.
    pub(crate) fn open_session_project(&mut self) {
        services_access::initialize_from_session(self);
        #[cfg(not(target_arch = "wasm32"))]
        self.apply_startup_project_choice();
    }

    /// "Start with no project" closes the reopened project when that loses
    /// nothing, as Close Project would without asking. Unsaved work keeps it
    /// open. Desktop only: the browser binds its project file after startup,
    /// once its storage answers, so there is nothing settled to close here.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn apply_startup_project_choice(&mut self) {
        if project_workflow::startup_keeps_project(&self.model, &self.platform) {
            return;
        }
        if project_workflow::closing_loses_nothing(self) {
            close_project_discard(self, ProjectCloseDestination::EmptyWorkbench);
        } else {
            project_workflow::report_startup_project_kept(&mut self.model, &self.platform);
        }
    }
}

/// The close every route shares. The empty project left open is built off
/// to the side; the previous project's execution context is retired under
/// the close's announcement before the empty project is installed with the
/// drawing the editor opens on; the editor takes that drawing, the empty
/// project takes the session's first plan and model catalog, the features
/// let go of what they kept of the closed project, and the reader lands on
/// the no-project screen, or in the live session's mirror.
fn close_project_discard(state: &mut AppState, destination: ProjectCloseDestination) -> bool {
    if project_workflow::refuse_project_close(&mut state.model, &mut state.workbench) {
        return false;
    }
    let ClosedProject {
        library_manager,
        workspace,
        announcement,
    } = project_workflow::build_closed_project(&state.model);
    let schematic = state.new_schematic_document();
    state.replace_project_execution_context(announcement);
    project_workflow::install_project_drawing(
        &mut state.model,
        library_manager,
        workspace,
        schematic,
    );
    state.design.schematic_snap = state.model.schematic.session.editor.snap_engine.clone();
    state.bump_active_schematic_epoch();
    let model_restore_errors =
        project_workflow::settle_closed_project(&mut state.model, &mut state.platform);
    services_access::mark_project_closed(state);
    project_workflow::emit_session_model_restore_errors(
        &mut state.model,
        "project close",
        model_restore_errors,
    );
    state.project.project_launcher_open = false;
    state.workbench.activate(Workspace::Project);
    if destination == ProjectCloseDestination::LiveMirror {
        state.workbench.outbox.push(requests::LiveMirrorEntry);
    }
    true
}

/// Opening a project read from a file, a browser import or binding, or a
/// live session's host, in place of the open one: the project's phases over
/// its own parts, with the shell's steps between them, in the order the open
/// has always run.
impl AppState {
    /// Replace the open project with `project`, read from `origin` under the
    /// replacement `transaction` reserved for it, and accept it under
    /// `binding`. Returns whether it opened.
    pub(crate) fn apply_loaded_project_authorized(
        &mut self,
        project: ProjectSnapshot,
        origin: ProjectLoadOrigin<'_>,
        binding: Option<PersistenceBinding>,
        transaction: TransactionId,
    ) -> bool {
        let lifecycle = LifecycleParts {
            model: &mut self.model,
            platform: &mut self.platform,
            workspace: self.workbench.workspace,
            presentation: &self.results.results,
        };
        let Some(LoadedProject {
            announcement,
            install,
            restore,
        }) = project_workflow::prepare_loaded_project(
            lifecycle,
            &mut self.workbench,
            project,
            origin,
            transaction,
        )
        else {
            return false;
        };
        self.replace_project_execution_context(announcement);
        project_workflow::install_loaded_project(&mut self.model, install);
        self.restore_active_schematic_from_workspace();
        let OpenedProject {
            result_presentation,
            accepted_baseline,
            report,
        } = project_workflow::restore_loaded_simulation(&mut self.model, restore);
        // Markers are restored after the datasets they annotate, and only those
        // that still find their analysis: a marker pointing at a dataset this
        // project no longer retains would draw on nothing.
        self.restore_result_presentation(result_presentation);
        project_workflow::report_opened_project(
            &mut self.model,
            &mut self.platform,
            origin,
            report,
        );
        services_access::accept_loaded_project(self, accepted_baseline, binding);
        // What the stored rule deck holds that this build does not apply, on the
        // page before anything is checked under it.
        self.publish_rule_deck_issues();
        true
    }

    /// Open the project the browser read: a copy imported from a picked
    /// file, or the project a browser binding opened, whose binding is then
    /// settled. Returns whether it opened.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn open_browser_project(&mut self, open: BrowserProjectOpen) -> bool {
        match open {
            BrowserProjectOpen::Import {
                project,
                name,
                transaction,
            } => self.apply_loaded_project_authorized(
                project,
                ProjectLoadOrigin::BrowserImport(&name),
                None,
                transaction,
            ),
            BrowserProjectOpen::Canonical(open) => {
                let opened = self.apply_loaded_project_authorized(
                    open.project,
                    ProjectLoadOrigin::BrowserCanonical(&open.display_name),
                    Some(open.binding.clone()),
                    open.transaction,
                );
                project_workflow::settle_browser_canonical_open(
                    &self.model,
                    &mut self.platform,
                    opened,
                    open.handle_id,
                    open.display_name,
                    open.binding,
                )
            }
        }
    }
}

/// Reverting the active document to what its project last saved, answered
/// from the revert review: the shell's revert over the whole project, then
/// the project's report of it.
impl AppState {
    /// Revert the document `token` reviewed. Returns whether it was reverted.
    pub(crate) fn confirm_revert_active_document(&mut self, token: &RevertReviewToken) -> bool {
        let reverted = services_access::confirm_revert_active_document(self, token);
        project_workflow::report_revert(&mut self.model, &mut self.workbench, token, reverted)
    }
}

/// Saving the open project: the inspector's pending inline edit committed
/// before the save reads the drawing, and again after the desktop's save
/// dialog, then the project's save over its parts.
impl AppState {
    /// Save `scope` of the open project where it saves: its own file, a file
    /// the reader picks, or the browser's. Returns how the save went.
    pub(crate) fn save_project_scope(&mut self, scope: SaveScope) -> SaveRequestOutcome {
        if let Err(error) = self.commit_inline_component_edit() {
            self.prompt(format!("Inspector edit was not applied: {error}"));
            return SaveRequestOutcome::Failed(error);
        }
        save_scope_for_platform(self, scope)
    }

    /// Save `scope` of the open project to `path` under `authority`.
    #[cfg(all(not(target_arch = "wasm32"), test))]
    pub(crate) fn save_project_scope_to(
        &mut self,
        scope: SaveScope,
        path: &std::path::Path,
        authority: DestinationAuthority,
    ) -> Result<(), String> {
        save_native_scope(self, scope, path, authority)
    }

    /// Save a copy of the open project through the browser. Returns whether
    /// the save started.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn save_project_copy_in_browser(&mut self) -> bool {
        project_workflow::save_project_copy_in_browser(save_parts(self))
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn save_scope_for_platform(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    let (path, authority) = match project_workflow::native_save_destination(
        &mut state.model,
        &mut state.workbench,
        &state.platform,
    ) {
        Ok(destination) => destination,
        Err(outcome) => return outcome,
    };
    save_native_scope(state, scope, &path, authority)
        .map_or_else(SaveRequestOutcome::Failed, |()| {
            SaveRequestOutcome::CanonicalComplete
        })
}

#[cfg(target_arch = "wasm32")]
fn save_scope_for_platform(state: &mut AppState, scope: SaveScope) -> SaveRequestOutcome {
    project_workflow::save_scope_in_browser(save_parts(state), scope)
}

/// The desktop save once its file is settled: the inspector's pending inline
/// edit committed again, since the save dialog may have let the reader make
/// one, then the project's save over its parts.
#[cfg(not(target_arch = "wasm32"))]
fn save_native_scope(
    state: &mut AppState,
    scope: SaveScope,
    path: &std::path::Path,
    authority: DestinationAuthority,
) -> Result<(), String> {
    state.commit_inline_component_edit()?;
    project_workflow::save_native_scope(save_parts(state), scope, path, authority)
}

/// What the project's save reads and writes, with the features' facts its
/// design checks read, taken as the save starts.
fn save_parts(state: &mut AppState) -> SaveParts<'_> {
    let facts = state.slot_facts();
    let workspace = state.workbench.workspace;
    SaveParts {
        lifecycle: LifecycleParts {
            model: &mut state.model,
            platform: &mut state.platform,
            workspace,
            presentation: &state.results.results,
        },
        workbench: &mut state.workbench,
        facts,
    }
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
    #[cfg(test)]
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
pub(in crate::workbench) fn apply_workspace_pruned(state: &mut AppState, pruned: WorkspacePruned) {
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

    #[test]
    fn replacing_the_active_document_reconciles_document_pitch_and_session_snap_preferences() {
        let mut state = AppState::default();
        state.design.schematic_snap.snap_radius = 8;
        state.design.schematic_snap.snap_to_grid = false;
        state.design.schematic_snap.snap_to_wire_segments = false;
        state.design.schematic_snap.grid_size = 999;
        let mut loaded = SchematicState::default();
        loaded.document_mut_for_test().document_policy.grid_pitch =
            crate::state::SchematicGridPitch::Metric;
        loaded.document_mut_for_test().grid_size = 123;
        loaded.session.editor.snap_engine = crate::state::SnapEngine::default();

        replace_active_schematic_document(&mut state, loaded);

        let expected = crate::state::SchematicGridPitch::Metric.canvas_grid_size();
        assert_eq!(
            state.model.schematic.document().document_policy.grid_pitch,
            crate::state::SchematicGridPitch::Metric
        );
        assert_eq!(state.model.schematic.document().grid_size, expected);
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.grid_size,
            expected
        );
        assert_eq!(
            state.model.schematic.session.editor.snap_engine.snap_radius,
            8
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_grid
        );
        assert!(
            !state
                .model
                .schematic
                .session
                .editor
                .snap_engine
                .snap_to_wire_segments
        );
        assert_eq!(
            state.design.schematic_snap,
            state.model.schematic.session.editor.snap_engine
        );
    }

    /// Assert that the body of this module's function `name` runs `steps` in
    /// order, each first found after the one before it. A free fn of that
    /// name is the transaction itself, read before a method that enters it.
    fn assert_runs_in_order(name: &str, steps: &[&str]) {
        const SOURCE: &str = include_str!("project_transactions.rs");
        let definition = format!("fn {name}(");
        let start = SOURCE
            .find(&format!("\n{definition}"))
            .map(|newline| newline + 1)
            .or_else(|| SOURCE.find(&definition))
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

    /// A schematic file replaces the drawing in front in the order it always
    /// has: inside a project it is imported instead; otherwise where it saves
    /// next is settled, the editor takes its drawing, the execution context
    /// is retired, and then the file is remembered and the open recorded.
    #[test]
    fn a_loaded_schematic_replaces_the_drawing_in_the_order_it_always_has() {
        assert_runs_in_order(
            "apply_loaded_schematic",
            &[
                "self.platform.project_lifecycle.is_open()",
                "file_workflow::import_loaded_schematic(self, schematic, origin)",
                "file_workflow::prepare_loaded_schematic(",
                "replace_active_schematic_document(self, schematic)",
                "self.clear_design_execution_context()",
                "file_workflow::settle_loaded_schematic(",
            ],
        );
    }

    /// The empty project a close leaves open holds, as its active view, the
    /// drawing the editor opens on, with the same unsaved mark: the drawing
    /// is saved into its workspace, a value no other step reads, wherever
    /// the close does it among its steps.
    #[test]
    fn a_closed_project_holds_the_drawing_the_editor_opens_on() {
        let mut state = AppState::default();
        assert!(state.close_project_discard(ProjectCloseDestination::EmptyWorkbench));

        let workspace = &state.model.workspace;
        let saved = workspace
            .clone_schematic_editor(&workspace.content.active_key())
            .expect("the empty project opens on its schematic");
        assert_eq!(
            format!("{:?}", saved.document()),
            format!("{:?}", state.model.schematic.document())
        );
        let active = workspace
            .content
            .open_views
            .iter()
            .find(|open| open.reference == workspace.content.active_view)
            .expect("the active view is open");
        assert_eq!(active.dirty, state.model.schematic.session.is_dirty);
    }

    /// The open project closes in the order it always has: the refusal
    /// before anything changes, the empty project built off to the side and
    /// its drawing made, the execution context retired before the empty
    /// project is installed, the editor's snap and epoch, the plan and model
    /// catalog, the features let go, the restore errors reported, and then
    /// the reader lands where the close was asked to leave them.
    #[test]
    fn a_project_closes_in_the_order_it_always_has() {
        assert_runs_in_order(
            "close_project_discard",
            &[
                "project_workflow::refuse_project_close(",
                "project_workflow::build_closed_project(",
                "state.new_schematic_document()",
                "state.replace_project_execution_context(announcement)",
                "project_workflow::install_project_drawing(",
                "state.design.schematic_snap =",
                "state.bump_active_schematic_epoch()",
                "project_workflow::settle_closed_project(",
                "services_access::mark_project_closed(state)",
                "project_workflow::emit_session_model_restore_errors(",
                "state.project.project_launcher_open = false",
                "state.workbench.activate(Workspace::Project)",
                "state.workbench.outbox.push(requests::LiveMirrorEntry)",
            ],
        );
    }

    /// The session's project opens at startup in the order it always has:
    /// bound from the restored session, then Projects and files ▸ On
    /// startup applied to it, which closes it only when that loses nothing.
    #[test]
    fn the_session_project_opens_in_the_order_it_always_has() {
        assert_runs_in_order(
            "open_session_project",
            &[
                "services_access::initialize_from_session(self)",
                "self.apply_startup_project_choice()",
            ],
        );
        assert_runs_in_order(
            "apply_startup_project_choice",
            &[
                "project_workflow::startup_keeps_project(",
                "project_workflow::closing_loses_nothing(self)",
                "close_project_discard(self, ProjectCloseDestination::EmptyWorkbench)",
                "project_workflow::report_startup_project_kept(",
            ],
        );
    }

    /// A project opens in place of the open one in the order it always has:
    /// the refusals and the execution context it was saved with before
    /// anything changes, the replaced project's execution context retired,
    /// the project installed and its drawing reloaded, its simulation and
    /// then its markers restored, the open reported, the project accepted,
    /// and then the rule deck's issues published.
    #[test]
    fn a_loaded_project_opens_in_the_order_it_always_has() {
        assert_runs_in_order(
            "apply_loaded_project_authorized",
            &[
                "project_workflow::prepare_loaded_project(",
                "self.replace_project_execution_context(announcement)",
                "project_workflow::install_loaded_project(&mut self.model, install)",
                "self.restore_active_schematic_from_workspace()",
                "project_workflow::restore_loaded_simulation(&mut self.model, restore)",
                "self.restore_result_presentation(result_presentation)",
                "project_workflow::report_opened_project(",
                "services_access::accept_loaded_project(self, accepted_baseline, binding)",
                "self.publish_rule_deck_issues()",
            ],
        );
    }

    /// A save commits the inspector's pending inline edit before anything
    /// else, and on the desktop commits it again once the save dialog has
    /// settled the file, right before the project's save.
    #[test]
    fn a_save_commits_the_inline_edit_before_the_project_saves() {
        assert_runs_in_order(
            "save_project_scope",
            &[
                "self.commit_inline_component_edit()",
                "save_scope_for_platform(self, scope)",
            ],
        );
        assert_runs_in_order(
            "save_scope_for_platform",
            &[
                "project_workflow::native_save_destination(",
                "save_native_scope(state, scope, &path, authority)",
            ],
        );
        assert_runs_in_order(
            "save_native_scope",
            &[
                "state.commit_inline_component_edit()?",
                "project_workflow::save_native_scope(save_parts(state), scope, path, authority)",
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
