//! The host the workbench's own containers are drawn for.
//!
//! [`Host`] keys its points and menus by types it leaves to the host. The
//! workbench's containers (the chrome, the docks, the stage, the palette and
//! the framework's dialogs) are drawn for the one host the shell runs, so they
//! name it by the workbench's own keys, and they read and change the session
//! through it rather than through the application that holds the session.

use super::host::{Host, ProblemsHost};
use crate::diagnostics::LogAnchor;
use crate::workbench::app_state::diagnostic_store::listing::ListingSource;
use crate::workbench::app_state::problems_view::ProblemsView;
use crate::workbench::app_state::waivers::WaiverTarget;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::{WorkbenchSession, Workspace};

/// A [`Host`] keyed by the workbench's own types, and the session it holds.
///
/// The menus a host hands over are the title bar's own types, which sit above
/// this trait, so the title bar names them where it draws them.
pub(crate) trait WorkbenchHost: Host<WorkspaceKey = Workspace, Layout = LayoutSpec> {
    /// The session a container is handed whole, for the pages that draw from
    /// it and change it in the same pass.
    type Session: ConsoleHost + 'static;

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

    /// The whole session, for a container that reads many of its parts.
    fn state(&self) -> &AppState;

    /// The whole session, to change.
    fn state_mut(&mut self) -> &mut AppState;
}

/// The session the console's pages draw from: the model and the workbench
/// session, the Problems page's own view, and the shell's answers for a log
/// line's anchor and a finding's waiver.
pub(crate) trait ConsoleHost: ProblemsHost + ListingSource {
    /// The engineering model the log and the findings belong to.
    fn model(&self) -> &Model;

    /// The engineering model, to change.
    fn model_mut(&mut self) -> &mut Model;

    /// The workbench session: the console's views, the prompt and the toasts.
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
