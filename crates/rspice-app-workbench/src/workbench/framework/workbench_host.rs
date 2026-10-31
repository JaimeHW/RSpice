//! The host the workbench's own containers are drawn for.
//!
//! [`Host`] keys its points and menus by types it leaves to the host. The
//! workbench's containers (the chrome, the docks, the stage, the palette and
//! the framework's dialogs) are drawn for the one host the shell runs, so they
//! name it by the workbench's own keys, and they read and change the session
//! through it rather than through the application that holds the session.

use super::host::{DocumentHost, Host, ProblemsHost};
use crate::diagnostics::LogAnchor;
use crate::state::CellViewRef;
use crate::workbench::app_state::Model;
use crate::workbench::app_state::diagnostic_store::listing::ListingSource;
use crate::workbench::app_state::waivers::WaiverTarget;
use crate::workbench::layout::LayoutSpec;
use crate::workbench::lifecycle::project_lifecycle::{LifecycleSource, PlatformState};
use crate::workbench::state::problems_view::ProblemsView;
use crate::workbench::state::workbench_session::WorkspaceDocument;
use crate::workbench::state::{WorkbenchSession, Workspace, WorkspaceDocumentId};

/// A [`Host`] keyed by the workbench's own types, and the session it holds.
///
/// The menus a host hands over are the title bar's own types, which sit above
/// this trait, so the title bar names them where it draws them.
pub trait WorkbenchHost: Host<WorkspaceKey = Workspace, Layout = LayoutSpec> {
    /// The session a container is handed whole, for the pages that draw from
    /// it and change it in the same pass: the console's host, the documents'
    /// host and the project lifecycle's source.
    type Session: ConsoleHost
        + DocumentHost<
            WorkspaceKey = Workspace,
            DocumentId = WorkspaceDocumentId,
            Document = WorkspaceDocument,
        > + LifecycleSource
        + 'static;

    /// The session, for a page drawn from it.
    fn session(&self) -> &Self::Session;

    /// The session, for a page that changes it as it draws.
    fn session_mut(&mut self) -> &mut Self::Session;

    /// The workbench session: what is showing, the docks, drawers and
    /// dialogs around it, and the outbox.
    fn workbench(&self) -> &WorkbenchSession;

    /// The workbench session, to change.
    fn workbench_mut(&mut self) -> &mut WorkbenchSession;

    /// The engineering model: the project, its libraries, the schematic and
    /// the simulation.
    fn model(&self) -> &Model;

    /// The engineering model, to change.
    fn model_mut(&mut self) -> &mut Model;

    /// The platform's record: the project lifecycle and the autosave
    /// preference.
    fn platform(&self) -> &PlatformState;

    /// Whether a run's preflight check is under way, which the run control
    /// offers to stop.
    fn run_preflight_checking(&self) -> bool;

    /// Whether anything modal the shell lists is open, which owns the
    /// keyboard and the pointer for the frame.
    fn application_modal_open(&self) -> bool;

    /// Whether the device-model editor holds a draft with unsaved changes.
    fn model_draft_unsaved(&self) -> bool;

    /// The stimulus definitions whose drafts are not applied, by name.
    fn unapplied_stimulus_drafts(&self) -> Vec<String>;

    /// The configuration whose edits on the Configuration page are not
    /// applied, by name.
    fn unapplied_configuration_edits(&self) -> Option<String>;

    /// Open `reference` as the design's active view.
    fn open_cell_view(&mut self, reference: CellViewRef);

    /// Show `model` from `library` in the Models workspace's catalog.
    fn show_catalog_model(&mut self, library: &str, model: String);

    /// Show the waveform `waveform` of analysis `analysis` of retained run
    /// `run` in the Results workspace.
    fn show_signal(&mut self, run: usize, analysis: usize, waveform: usize);
}

/// The session the console's pages draw from: the model and the workbench
/// session, the Problems page's own view, and the shell's answers for a log
/// line's anchor and a finding's waiver.
pub trait ConsoleHost: ProblemsHost + ListingSource {
    /// The engineering model the log and the findings belong to.
    fn model(&self) -> &Model;

    /// The engineering model, to change.
    fn model_mut(&mut self) -> &mut Model;

    /// The workbench session: the console's views, the open documents, the
    /// prompt and the toasts.
    fn workbench(&self) -> &WorkbenchSession;

    /// The workbench session, to change.
    fn workbench_mut(&mut self) -> &mut WorkbenchSession;

    /// The Problems page's filters, columns and waiver draft.
    fn problems_view(&self) -> &ProblemsView;

    /// The Problems page's view, to change.
    fn problems_view_mut(&mut self) -> &mut ProblemsView;

    /// Leave the console for whatever `anchor` names.
    fn jump_to_log_anchor(&mut self, anchor: LogAnchor);

    /// Why a line carrying `anchor` cannot be followed, or `None` if it can.
    fn log_anchor_refusal(&self, anchor: &LogAnchor) -> Option<String>;

    /// Waive the finding at `target` for `reason`, as an edit of the document
    /// that stores it.
    fn waive_finding(&mut self, target: &WaiverTarget, reason: &str) -> Result<(), String>;

    /// Remove the waiver stored at `target`.
    fn remove_finding_waiver(&mut self, target: &WaiverTarget) -> Result<(), String>;
}
