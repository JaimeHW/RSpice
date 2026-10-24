//! The host the workbench's own containers are drawn for.
//!
//! [`Host`] keys its points and menus by types it leaves to the host. The
//! workbench's containers (the chrome, the docks, the stage, the palette and
//! the framework's dialogs) are drawn for the one host the shell runs, so they
//! name it by the workbench's own keys, and they read and change the session
//! through it rather than through the application that holds the session.

use super::host::Host;
use crate::workbench::app_state::{AppState, Model};
use crate::workbench::layout::LayoutSpec;
use crate::workbench::state::{WorkbenchSession, Workspace};

/// A [`Host`] keyed by the workbench's own types, and the session it holds.
///
/// The menus a host hands over are the title bar's own types, which sit above
/// this trait, so the title bar names them where it draws them.
pub(crate) trait WorkbenchHost: Host<WorkspaceKey = Workspace, Layout = LayoutSpec> {
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
