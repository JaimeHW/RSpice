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
// Until every reader names them at their home in the model tier.
pub use crate::state::{SimulationPage, VerificationPage};

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

/// Authored text of the run-set execution budgets while they are being edited.
///
/// A budget is committed on release, not on keystroke: a partially typed
/// "4 Mi" would otherwise parse to four bytes and refuse the whole space for
/// as long as the field had focus.
#[derive(Debug, Clone, PartialEq)]
pub struct RunSetBudgetDrafts {
    pub maximum_tasks: String,
    pub maximum_storage: String,
    pub cost_per_point_ms: String,
    pub bytes_per_point: String,
}

/// Whether a Simulation Studio plan command changed the plan or was refused.
///
/// A receipt and a refusal are told apart here rather than by reading the
/// message, because the two are shown differently and only one of them has to
/// follow the reader off the Analyses page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnalysisLifecycleSeverity {
    /// Everything the command asked for was committed, and the message is the
    /// transaction's receipt.
    #[default]
    Receipt,
    /// Something the command asked for was refused, and the message names what
    /// and why. A command that commits one step and is refused the next is a
    /// refusal: the reader has to be told about the part that did not happen.
    Refusal,
}

/// The last outcome any Simulation Studio plan command announced.
///
/// Only the Analyses route draws this; the Console holds the record of each
/// outcome. An outcome is new only when it *changes*, not on every
/// restatement: three of the writers sit on the render path rather than in a
/// click handler, and recording each restatement would record the same
/// refusal forever.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnalysisLifecycleOutcome {
    severity: AnalysisLifecycleSeverity,
    message: String,
}

impl Default for AnalysisLifecycleOutcome {
    /// The field is runtime-only, so a restored session gets this rather than
    /// the outcome it was saved with. It has to read as a real status line.
    fn default() -> Self {
        Self {
            severity: AnalysisLifecycleSeverity::Receipt,
            message: "No lifecycle command has been committed this session.".to_owned(),
        }
    }
}

impl AnalysisLifecycleOutcome {
    pub fn message(&self) -> &str {
        &self.message
    }

    pub const fn is_refusal(&self) -> bool {
        matches!(self.severity, AnalysisLifecycleSeverity::Refusal)
    }

    /// Announce a committed change. Returns whether this is a new outcome, so
    /// that a reporter which also records it elsewhere records it once per
    /// event.
    pub fn record_receipt(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Receipt, message)
    }

    /// Announce a command that changed nothing, and why. Returns whether this
    /// is a new outcome, as [`Self::record_receipt`] does.
    pub fn record_refusal(&mut self, message: impl Into<String>) -> bool {
        self.record(AnalysisLifecycleSeverity::Refusal, message)
    }

    /// Whether the announced outcome differs from the one already held.
    ///
    /// Three announcing sites sit on the render path rather than in a click
    /// handler, so a standing refusal is restated every frame. Restating it is
    /// not an event, and a reporter reading the answer does not report it
    /// again.
    fn record(&mut self, severity: AnalysisLifecycleSeverity, message: impl Into<String>) -> bool {
        let message = message.into();
        if self.severity == severity && self.message == message {
            return false;
        }
        self.severity = severity;
        self.message = message;
        true
    }
}

/// Which specifications the specification registry lists.
///
/// The classes name evidence, not release policy. A specification carries no
/// severity, so a "blocking only" view would be a control with nothing behind
/// it; what the page does resolve for every row is whether the active dataset
/// answered the limit, and how completely. The registry owns that
/// classification — this only says which of its outcomes to show.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SpecificationEvidenceFilter {
    #[default]
    All,
    /// Limits the dataset answered with a failure, including a measurement
    /// that ran and did not succeed.
    Failing,
    /// Limits no measurement in the dataset answers at all.
    WithoutEvidence,
    /// Limits judged on the worst of several retained measurements, so the
    /// verdict speaks for the points the dataset kept and no others.
    Partial,
}

impl SpecificationEvidenceFilter {
    pub const ALL: [Self; 4] = [
        Self::All,
        Self::Failing,
        Self::WithoutEvidence,
        Self::Partial,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All requirements",
            Self::Failing => "Failing only",
            Self::WithoutEvidence => "Without results",
            Self::Partial => "Measured at several points",
        }
    }
}

pub(in crate::workbench) const fn default_analysis_index() -> usize {
    rspice_simulation_contract::analysis_kind::AnalysisKind::Transient.legacy_index()
}

#[cfg(test)]
mod tests;
