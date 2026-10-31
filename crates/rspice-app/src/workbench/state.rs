//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use rspice_app_workbench::workbench::state::console_views;
pub(in crate::workbench) use rspice_app_workbench::workbench::state::navigator_tree;
mod session_views;
mod stimulus_editor;
pub(crate) use rspice_app_workbench::workbench::state::workbench_session;
mod simulate_session;
mod workflow_drafts;

// The workbench's own session and the views over its records, which the
// workbench reads by these paths.
pub use console_views::{ConsoleChannel, InlineEditField, SpecialistToolBrowserState};
#[cfg(test)]
pub use console_views::{
    ConsoleReveal, ConsoleSelection, ConsoleSeverity, ConsoleView, NotificationFilter,
    SpecialistToolFilter,
};
pub(crate) use console_views::{InlineEditAuthority, InlineEditSession};
pub use navigator_tree::*;
pub use session_views::*;
pub use simulate_session::*;
pub use stimulus_editor::*;
#[cfg(test)]
use workbench_session::LAYOUT_SCHEMA_VERSION;
pub(crate) use workbench_session::{ActiveSourceEditor, default_console_height};
pub use workbench_session::{
    ConsolePage, Drawer, WorkbenchSession, WorkspaceDocumentId, WorkspaceDocumentRegistry,
    WorkspaceLayoutState,
};
#[cfg(test)]
pub use workbench_session::{StatusPrompt, WidthClass};
pub(in crate::workbench) use workbench_session::{
    default_inspector_width, default_navigator_width,
};
pub use workflow_drafts::*;
// Every tier reads the workspace, so it is the model's vocabulary.
pub use rspice_app_state::workbench::workspace::Workspace;

use serde::{Deserialize, Serialize};

// What the session tests name the routes and presets by.
#[cfg(test)]
use super::{
    BrowserHistoryEffect, RouteTransitionSource, SurfaceArchetype, SurfaceId, SurfaceRoute,
    WorkspacePreset,
};

/// The two Design navigator tabs specified by the workbench mockup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DesignPanel {
    #[default]
    Navigator,
    ComponentShelf,
}

#[cfg(test)]
mod tests;
