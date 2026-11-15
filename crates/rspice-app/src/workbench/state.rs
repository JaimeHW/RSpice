//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use rspice_app_simulate::workbench::state::simulate_session;
#[cfg(test)]
use rspice_app_simulate::workbench::state::workflow_drafts;
use rspice_app_workbench::workbench::state::console_views;
#[cfg(test)]
pub(crate) use rspice_app_workbench::workbench::state::navigator_tree::{
    NavigatorTreeNode, NavigatorTreeState,
};
pub(crate) use rspice_app_workbench::workbench::state::workbench_session;

// The workbench's own session and the views over its records, which the
// workbench reads by these paths.
pub use console_views::ConsoleChannel;
#[cfg(test)]
pub(crate) use console_views::InlineEditAuthority;
#[cfg(test)]
pub use console_views::InlineEditField;
#[cfg(test)]
pub(crate) use console_views::InlineEditSession;
#[cfg(test)]
pub use console_views::{
    ConsoleReveal, ConsoleSelection, ConsoleSeverity, ConsoleView, NotificationFilter,
};
#[cfg(test)]
pub use rspice_app_design::workbench::state::stimulus_editor::*;
pub use simulate_session::*;
#[cfg(test)]
use workbench_session::LAYOUT_SCHEMA_VERSION;
pub use workbench_session::default_console_height;
pub use workbench_session::{
    ConsolePage, Drawer, WorkbenchSession, WorkspaceDocumentId, WorkspaceDocumentRegistry,
    WorkspaceLayoutState,
};
#[cfg(test)]
pub use workbench_session::{StatusPrompt, WidthClass};
pub(in crate::workbench) use workbench_session::{
    default_inspector_width, default_navigator_width,
};
#[cfg(test)]
pub use workflow_drafts::*;
// Every tier reads the workspace, so it is the model's vocabulary.
pub use rspice_app_state::workbench::workspace::Workspace;

// What the session tests name the routes and presets by.
#[cfg(test)]
use super::{
    BrowserHistoryEffect, RouteTransitionSource, SurfaceId, SurfaceRoute, WorkspacePreset,
};

#[cfg(test)]
mod tests;
