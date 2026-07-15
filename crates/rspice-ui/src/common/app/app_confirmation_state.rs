//! Confirmation workflow state.
//!
//! Typed confirmation requests and responses shared by application workflows.

/// Actions that can trigger a save confirmation dialog
///
/// Commercial EDA tools like Cadence Virtuoso always prompt the user before
/// discarding unsaved work. This enum captures the pending action so it can
/// be executed after the user responds to the confirmation dialog.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationAction {
    /// Create new project (discard current workspace)
    ProjectNew,
    /// Open another project (discard current workspace)
    ProjectOpen,
    /// Close the current project while leaving RSpice running.
    CloseProject,
    /// Create new schematic (discard current)
    FileNew,
    /// Open another schematic (discard current)
    FileOpen,
    /// Open a file from the recent-files list (path stored alongside the
    /// pending action in [`ConfirmationDialogState::pending_path`])
    OpenRecent,
    /// Load a bundled example circuit (name stored alongside the pending
    /// action in [`ConfirmationDialogState::pending_example`]).
    OpenExample,
    /// Import a SPICE deck into the Netlist workspace.
    ImportNetlist,
    /// Close the application
    Exit,
}

impl ConfirmationAction {
    /// Get the dialog title for this action
    pub fn dialog_title(&self) -> &'static str {
        match self {
            ConfirmationAction::ProjectNew => "Create New Project",
            ConfirmationAction::ProjectOpen => "Open Project",
            ConfirmationAction::CloseProject => "Close Project",
            ConfirmationAction::FileNew => "Create New Schematic",
            ConfirmationAction::FileOpen => "Open Schematic",
            ConfirmationAction::OpenRecent => "Open Recent File",
            ConfirmationAction::OpenExample => "Open Example",
            ConfirmationAction::ImportNetlist => "Import SPICE Deck",
            ConfirmationAction::Exit => "Exit RSpice",
        }
    }

    /// Get the prompt message for this action
    pub fn prompt_message(&self) -> &'static str {
        match self {
            ConfirmationAction::ProjectNew
            | ConfirmationAction::ProjectOpen
            | ConfirmationAction::CloseProject => {
                "The current project has unsaved changes.\nDo you want to save before continuing?"
            }
            ConfirmationAction::FileNew
            | ConfirmationAction::FileOpen
            | ConfirmationAction::OpenRecent
            | ConfirmationAction::OpenExample
            | ConfirmationAction::ImportNetlist
            | ConfirmationAction::Exit => {
                "The current design has unsaved changes.\nDo you want to save before continuing?"
            }
        }
    }
}

/// State for the save confirmation dialog
///
/// When visible is true, the confirmation dialog is shown. The pending_action
/// field stores what should happen after the user responds.
#[derive(Debug, Clone, Default)]
pub struct ConfirmationDialogState {
    /// Whether the dialog is currently visible
    pub visible: bool,
    /// The action pending user confirmation
    pub pending_action: Option<ConfirmationAction>,
    /// Target path for path-carrying actions ([`ConfirmationAction::OpenRecent`]).
    pub pending_path: Option<std::path::PathBuf>,
    /// Exact recent-file kind captured with [`ConfirmationAction::OpenRecent`].
    /// Project and schematic entries have different save scopes and must not
    /// be inferred later from a mutable recent-files list or extension.
    pub(crate) pending_recent_kind: Option<super::RecentKind>,
    /// Example name for [`ConfirmationAction::OpenExample`].
    pub pending_example: Option<String>,
    /// Destructive action suspended until the exact browser save transaction
    /// reaches a verified canonical completion.
    pub(crate) awaiting_canonical_save: Option<PendingConfirmationContinuation>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Debug, Clone)]
pub(crate) struct PendingConfirmationContinuation {
    pub(crate) transaction: crate::common::project_lifecycle::TransactionId,
    pub(crate) action: ConfirmationAction,
    pub(crate) path: Option<std::path::PathBuf>,
    pub(crate) recent_kind: Option<super::RecentKind>,
    pub(crate) example: Option<String>,
}

impl ConfirmationDialogState {
    /// Open the confirmation dialog for a specific action
    pub fn show(&mut self, action: ConfirmationAction) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = None;
        self.pending_recent_kind = None;
        self.pending_example = None;
    }

    /// Open the confirmation dialog for an action that targets a known path.
    pub fn show_with_path(&mut self, action: ConfirmationAction, path: std::path::PathBuf) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = Some(path);
        self.pending_recent_kind = None;
        self.pending_example = None;
    }

    /// Open the confirmation dialog for an exact recent-file entry.
    pub(crate) fn show_recent(&mut self, path: std::path::PathBuf, kind: super::RecentKind) {
        self.visible = true;
        self.pending_action = Some(ConfirmationAction::OpenRecent);
        self.pending_path = Some(path);
        self.pending_recent_kind = Some(kind);
        self.pending_example = None;
    }

    /// Open the confirmation dialog for an action that targets a bundled example.
    pub fn show_with_example(&mut self, action: ConfirmationAction, name: String) {
        self.visible = true;
        self.pending_action = Some(action);
        self.pending_path = None;
        self.pending_recent_kind = None;
        self.pending_example = Some(name);
    }

    /// Close the dialog and clear pending action
    pub fn close(&mut self) {
        self.visible = false;
        self.pending_action = None;
        self.pending_path = None;
        self.pending_recent_kind = None;
        self.pending_example = None;
    }

    pub(crate) fn await_canonical_save(
        &mut self,
        transaction: crate::common::project_lifecycle::TransactionId,
        action: ConfirmationAction,
        path: Option<std::path::PathBuf>,
        recent_kind: Option<super::RecentKind>,
        example: Option<String>,
    ) {
        self.awaiting_canonical_save = Some(PendingConfirmationContinuation {
            transaction,
            action,
            path,
            recent_kind,
            example,
        });
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn take_canonical_save(
        &mut self,
        transaction: crate::common::project_lifecycle::TransactionId,
    ) -> Option<PendingConfirmationContinuation> {
        let matching = self
            .awaiting_canonical_save
            .as_ref()
            .is_some_and(|pending| pending.transaction == transaction);
        if !matching {
            return None;
        }
        self.awaiting_canonical_save.take()
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn restore_continuation_for_review(
        &mut self,
        pending: PendingConfirmationContinuation,
    ) {
        self.visible = true;
        self.pending_action = Some(pending.action);
        self.pending_path = pending.path;
        self.pending_recent_kind = pending.recent_kind;
        self.pending_example = pending.example;
    }

    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    pub(crate) fn cancel_awaiting_canonical_save(&mut self) -> bool {
        self.awaiting_canonical_save.take().is_some()
    }

    /// Check if dialog is open for a specific action
    pub fn is_showing(&self, action: ConfirmationAction) -> bool {
        self.visible && self.pending_action == Some(action)
    }
}

/// User response to a save confirmation dialog
///
/// Standard Yes/No/Cancel pattern matching commercial EDA tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfirmationResponse {
    /// Save changes and proceed with action
    Yes,
    /// Discard changes and proceed with action
    No,
    /// Cancel the action, keep changes
    Cancel,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct ProjectReviewDialogState {
    pub(crate) request: Option<ProjectReviewRequest>,
}

#[derive(Debug, Clone)]
pub(crate) enum ProjectReviewRequest {
    RevertActive(crate::common::project_lifecycle::RevertReviewToken),
    CloseProject,
}

impl ProjectReviewDialogState {
    pub(crate) fn show_revert(
        &mut self,
        token: crate::common::project_lifecycle::RevertReviewToken,
    ) {
        self.request = Some(ProjectReviewRequest::RevertActive(token));
    }

    pub(crate) fn show_close_project(&mut self) {
        self.request = Some(ProjectReviewRequest::CloseProject);
    }

    pub(crate) fn close(&mut self) {
        self.request = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::app::RecentKind;

    #[test]
    fn async_canonical_save_continuation_is_transaction_bound_for_all_destructive_actions() {
        for action in [
            ConfirmationAction::CloseProject,
            ConfirmationAction::ProjectOpen,
            ConfirmationAction::ProjectNew,
            ConfirmationAction::Exit,
        ] {
            let expected = crate::common::project_lifecycle::TransactionId::new();
            let stale = crate::common::project_lifecycle::TransactionId::new();
            let mut state = ConfirmationDialogState::default();
            state.await_canonical_save(expected, action, None, None, None);

            assert!(state.take_canonical_save(stale).is_none());
            assert!(state.awaiting_canonical_save.is_some());

            let continuation = state
                .take_canonical_save(expected)
                .expect("matching verified save resumes action");
            assert_eq!(continuation.action, action);
            assert!(state.awaiting_canonical_save.is_none());
        }
    }

    #[test]
    fn cancel_external_change_and_failure_drop_continuation_without_proceeding() {
        for action in [
            ConfirmationAction::CloseProject,
            ConfirmationAction::ProjectOpen,
            ConfirmationAction::ProjectNew,
            ConfirmationAction::Exit,
        ] {
            let transaction = crate::common::project_lifecycle::TransactionId::new();
            let mut state = ConfirmationDialogState::default();
            state.await_canonical_save(transaction, action, None, None, None);
            assert!(state.take_canonical_save(transaction).is_some());
            assert!(state.awaiting_canonical_save.is_none());
        }
    }

    #[test]
    fn explicit_browser_operation_cancel_revokes_exact_pending_continuation() {
        let transaction = crate::common::project_lifecycle::TransactionId::new();
        let mut state = ConfirmationDialogState::default();
        state.await_canonical_save(
            transaction,
            ConfirmationAction::CloseProject,
            None,
            None,
            None,
        );

        assert!(state.cancel_awaiting_canonical_save());
        assert!(state.take_canonical_save(transaction).is_none());
        assert!(!state.cancel_awaiting_canonical_save());
    }

    #[test]
    fn recent_kind_survives_confirmation_and_async_save_continuation() {
        let transaction = crate::common::project_lifecycle::TransactionId::new();
        let path = std::path::PathBuf::from("typed-recent.rspiceproj");
        let mut state = ConfirmationDialogState::default();
        state.show_recent(path.clone(), RecentKind::Project);
        assert_eq!(state.pending_recent_kind, Some(RecentKind::Project));

        state.close();
        state.await_canonical_save(
            transaction,
            ConfirmationAction::OpenRecent,
            Some(path.clone()),
            Some(RecentKind::Project),
            None,
        );
        let pending = state
            .take_canonical_save(transaction)
            .expect("exact transaction resumes typed recent entry");
        assert_eq!(pending.path.as_deref(), Some(path.as_path()));
        assert_eq!(pending.recent_kind, Some(RecentKind::Project));

        state.restore_continuation_for_review(pending);
        assert_eq!(state.pending_recent_kind, Some(RecentKind::Project));
    }
}
