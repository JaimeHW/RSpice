//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use rspice_app_workbench::workbench::state::console_views;
pub(in crate::workbench) use rspice_app_workbench::workbench::state::navigator_tree;
pub(crate) mod plan_provenance;
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

#[cfg(test)]
use std::collections::HashSet;

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

/// Sort order offered by the project launcher. The recent-files store is
/// maintained newest-first and records the last-opened timestamp, so
/// `LastOpened` preserves its durable ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherSort {
    #[default]
    LastOpened,
    Name,
    Owner,
}

impl ProjectLauncherSort {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LastOpened => "Last opened",
            Self::Name => "Name",
            Self::Owner => "Owner",
        }
    }
}

/// Project groups backed by durable recent-project metadata.
///
/// Recovery has its own complete launcher page and governed templates are not
/// exposed until a template-package service exists, so neither is represented
/// as a decorative filter here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherFilter {
    #[default]
    All,
    Recent,
    Pinned,
    Shared,
}

impl ProjectLauncherFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Recent, Self::Pinned, Self::Shared];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Recent => "Recent",
            Self::Pinned => "Pinned",
            Self::Shared => "Shared",
        }
    }
}

/// Startup pages that have complete local executors in the desktop product.
/// Legal, identity, licensing, cloud, template, and extension-host pages are
/// intentionally absent until their governed services exist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherPage {
    #[default]
    Projects,
    Recovery,
    SafeMode,
}

impl ProjectLauncherPage {
    pub const ALL: [Self; 3] = [Self::Projects, Self::Recovery, Self::SafeMode];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Projects => "Projects",
            Self::Recovery => "Recovery",
            Self::SafeMode => "Safe mode",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ProjectPage {
    #[serde(alias = "Dashboard", alias = "Activity")]
    #[default]
    Overview,
    Library,
    Configuration,
    #[serde(alias = "Technology")]
    Dependencies,
    Recovery,
}

impl ProjectPage {
    pub const ALL: [Self; 5] = [
        Self::Overview,
        Self::Library,
        Self::Configuration,
        Self::Dependencies,
        Self::Recovery,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Overview => "Overview",
            Self::Library => "Library",
            Self::Configuration => "Configuration",
            Self::Dependencies => "Dependencies",
            Self::Recovery => "Recovery",
        }
    }
}

/// Persistent page selection inside the canonical Library/Cellview Manager.
///
/// The specialist surface is another projection over the project-owned
/// `LibraryManager`; this enum retains only which projection is visible and
/// never duplicates library, symbol, or form content.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum LibraryCellviewPage {
    #[default]
    Libraries,
    SymbolForm,
}

impl LibraryCellviewPage {
    pub const ALL: [Self; 2] = [Self::Libraries, Self::SymbolForm];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Libraries => "Libraries",
            Self::SymbolForm => "Symbol & form",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerificationPage {
    #[default]
    // A session saved while the placeholder DRC page existed restores to the
    // first page.
    #[serde(
        alias = "Cockpit",
        alias = "Specifications",
        alias = "Checks",
        alias = "Drc"
    )]
    Yield,
    Corners,
    Tuning,
    Optimization,
    DeviceSafety,
    #[serde(alias = "History")]
    Regression,
}

impl VerificationPage {
    pub const NAVIGATION: [Self; 6] = [
        Self::Yield,
        Self::Corners,
        Self::Tuning,
        Self::Optimization,
        Self::DeviceSafety,
        Self::Regression,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Yield => "PVT & Monte Carlo verification",
            Self::Corners => "Process-corner verification",
            Self::Tuning => "Live design-space exploration",
            Self::Optimization => "Optimization candidate",
            Self::DeviceSafety => "Safe-operating-area verification",
            Self::Regression => "Golden regression comparison",
        }
    }
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

/// Simulation Studio setup route.
///
/// The analyses page owns the ordered plan and its per-analysis forms; the
/// remaining pages own one plan-scoped concern each. Every route is backed by
/// state the run actually consumes, so a page can never present a control that
/// does not reach the engine.
#[derive(
    Debug, Clone, Copy, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum SimulationPage {
    #[default]
    Analyses,
    Excitations,
    Variables,
    Outputs,
    Specifications,
    RunSet,
    Models,
    Solver,
    Save,
}

impl SimulationPage {
    /// Navigation order, matching the order a plan is authored in: the
    /// analyses first, then what they are parameterized by, what they produce,
    /// what judges them, what they are swept over, what they are bound
    /// against, how they are solved, and what is kept.
    pub const NAVIGATION: [Self; 9] = [
        Self::Analyses,
        Self::Excitations,
        Self::Variables,
        Self::Outputs,
        Self::Specifications,
        Self::RunSet,
        Self::Models,
        Self::Solver,
        Self::Save,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Analyses => "Analyses",
            Self::Excitations => "Excitations",
            Self::Variables => "Design variables",
            Self::Outputs => "Outputs & expressions",
            Self::Specifications => "Requirements & specs",
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::Solver => "Solver & convergence",
            Self::Save => "Save & streaming policy",
        }
    }

    /// Page heading. Longer than the navigation label where the page owns more
    /// than its label implies.
    pub const fn title(self) -> &'static str {
        match self {
            Self::Analyses => "Analyses",
            Self::Excitations => "Excitations & the analyses that read them",
            Self::Variables => "Design variables",
            Self::Outputs => "Outputs & expressions",
            Self::Specifications => "Requirements & specifications",
            Self::RunSet => "PVT, sweeps & variation",
            Self::Models => "Models & sections",
            Self::Solver => "Solver, convergence & per-analysis overrides",
            Self::Save => "Save, streaming & retention policy",
        }
    }
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
