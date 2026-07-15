//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use serde::{Deserialize, Serialize};

/// The seven canonical workspaces from the product contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum Workspace {
    Project,
    #[default]
    Design,
    Simulate,
    Results,
    Verify,
    Models,
    Netlist,
}

impl Workspace {
    pub const ALL: [Self; 7] = [
        Self::Project,
        Self::Design,
        Self::Simulate,
        Self::Results,
        Self::Verify,
        Self::Models,
        Self::Netlist,
    ];

    pub const PHONE_PRIMARY: [Self; 4] =
        [Self::Design, Self::Simulate, Self::Results, Self::Verify];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulate",
            Self::Results => "Results",
            Self::Verify => "Verify",
            Self::Models => "Models",
            Self::Netlist => "Netlist",
        }
    }

    pub const fn owner_label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::Design => "Design",
            Self::Simulate => "Simulation plan",
            Self::Results => "Result document",
            Self::Verify => "Verification evidence",
            Self::Models => "Model binding",
            Self::Netlist => "Automation pipeline",
        }
    }

    pub const fn shortcut(self) -> &'static str {
        match self {
            Self::Project => "Alt+1",
            Self::Design => "Alt+2",
            Self::Simulate => "Alt+3",
            Self::Results => "Alt+4",
            Self::Verify => "Alt+5",
            Self::Models => "Alt+6",
            Self::Netlist => "Alt+7",
        }
    }

    pub const fn navigator_title(self) -> &'static str {
        match self {
            Self::Project => "Project navigator",
            Self::Design => "Design navigator",
            Self::Simulate => "Simulation plan",
            Self::Results => "Results navigator",
            Self::Verify => "Verification navigator",
            Self::Models => "Models and libraries",
            Self::Netlist => "Code and automation",
        }
    }

    pub const fn inspector_title(self) -> &'static str {
        match self {
            Self::Project => "Project details",
            Self::Design => "Inspector",
            Self::Simulate => "Analysis inspector",
            Self::Results => "Data inspector",
            Self::Verify => "Evidence inspector",
            Self::Models => "Model inspector",
            Self::Netlist => "Source inspector",
        }
    }
}

/// Width class used by layout composition.  Breakpoints are based on task
/// density rather than an assumed device or operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WidthClass {
    Phone,
    Tablet,
    Desktop,
    Wide,
}

impl WidthClass {
    pub const fn for_width(width: f32) -> Self {
        if width <= 560.0 {
            Self::Phone
        } else if width <= 820.0 {
            Self::Tablet
        } else if width <= 1260.0 {
            Self::Desktop
        } else {
            Self::Wide
        }
    }

    pub const fn is_phone(self) -> bool {
        matches!(self, Self::Phone)
    }

    pub const fn uses_bottom_navigation(self) -> bool {
        matches!(self, Self::Phone | Self::Tablet)
    }

    pub const fn navigator_uses_drawer(self) -> bool {
        self.uses_bottom_navigation()
    }

    pub const fn inspector_uses_drawer(self) -> bool {
        !matches!(self, Self::Wide)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Drawer {
    Navigator,
    Inspector,
    Workspaces,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ConsolePage {
    #[default]
    Console,
    Problems,
    Measurements,
    TaskLog,
}

/// The two Design navigator tabs specified by the workbench mockup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum DesignPanel {
    #[default]
    Navigator,
    ComponentShelf,
}

/// Sort order offered by the project launcher. The recent-files store is
/// already maintained newest-first, so `LastOpened` preserves that durable
/// ordering without fabricating timestamps that the application does not
/// persist.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ProjectLauncherSort {
    #[default]
    LastOpened,
    Name,
}

impl ProjectLauncherSort {
    pub const fn label(self) -> &'static str {
        match self {
            Self::LastOpened => "Last opened",
            Self::Name => "Name",
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

/// Safe-mode controls that can be enforced locally without an extension,
/// account, renderer-restart, or platform service.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSafeModeOptions {
    pub isolate_prior_documents: bool,
    pub reset_layout: bool,
}

impl Default for LocalSafeModeOptions {
    fn default() -> Self {
        Self {
            isolate_prior_documents: true,
            reset_layout: false,
        }
    }
}

impl LocalSafeModeOptions {
    pub const fn has_effect(self) -> bool {
        self.isolate_prior_documents || self.reset_layout
    }
}

/// Current-launch safe-mode state. The serialized pre-safe-mode session is
/// retained only so application persistence can keep it byte-for-byte
/// equivalent for the next normal launch.
#[derive(Debug, Clone, Default)]
pub struct LocalSafeModeState {
    pub active: bool,
    pub draft: LocalSafeModeOptions,
    pub applied: LocalSafeModeOptions,
    preserved_session: Option<String>,
}

impl LocalSafeModeState {
    pub(crate) fn activate(&mut self, options: LocalSafeModeOptions, session: String) {
        self.active = true;
        self.draft = options;
        self.applied = options;
        self.preserved_session = Some(session);
    }

    pub(crate) fn preserved_session(&self) -> Option<&str> {
        self.preserved_session.as_deref()
    }
}

impl ConsolePage {
    pub const ALL: [Self; 4] = [
        Self::Console,
        Self::Problems,
        Self::Measurements,
        Self::TaskLog,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Problems => "Problems",
            Self::Measurements => "Measurements",
            Self::TaskLog => "Task log",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ProjectPage {
    #[serde(alias = "Overview", alias = "Activity")]
    #[default]
    Dashboard,
    Configuration,
    Technology,
    Dependencies,
    Recovery,
}

impl ProjectPage {
    pub const ALL: [Self; 5] = [
        Self::Dashboard,
        Self::Configuration,
        Self::Technology,
        Self::Dependencies,
        Self::Recovery,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Project overview",
            Self::Configuration => "Testbench configuration",
            Self::Technology => "Technology & PDK",
            Self::Dependencies => "Dependency manifest",
            Self::Recovery => "Recovery center",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum VerificationPage {
    #[default]
    Cockpit,
    Specifications,
    Checks,
    Reliability,
    History,
}

impl VerificationPage {
    pub const ALL: [Self; 5] = [
        Self::Cockpit,
        Self::Specifications,
        Self::Checks,
        Self::Reliability,
        Self::History,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Cockpit => "Verification cockpit",
            Self::Specifications => "Specification matrix",
            Self::Checks => "Design checks",
            Self::Reliability => "Reliability and SOA",
            Self::History => "Run history",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum ModelsPage {
    #[default]
    Catalog,
    Libraries,
    Pdk,
    Behavioral,
    Qualification,
}

impl ModelsPage {
    pub const ALL: [Self; 5] = [
        Self::Catalog,
        Self::Libraries,
        Self::Pdk,
        Self::Behavioral,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Catalog => "Model catalog",
            Self::Libraries => "Libraries and cellviews",
            Self::Pdk => "PDK configuration",
            Self::Behavioral => "Verilog-A / AMS",
            Self::Qualification => "Qualification",
        }
    }
}

/// Destination that can resolve a blocking simulation-preflight finding.
/// The dialog stores semantic destinations instead of callbacks so a report
/// remains deterministic for the exact project revision that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightRemediation {
    DesignChecks,
    SimulationPlan,
}

/// One ordered, actionable finding in a simulation-preflight report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightIssue {
    pub check: String,
    pub observed: String,
    pub required: String,
    pub remediation: PreflightRemediation,
}

/// Immutable snapshot rendered by the mockup-specified preflight workflow.
/// It is intentionally runtime-only: a saved project persists the simulation
/// plan, while validation evidence must be regenerated for the live revision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightReport {
    pub project_revision: u64,
    pub topology_revision: u64,
    pub blockers: Vec<PreflightIssue>,
    pub advisories: Vec<String>,
    /// Present only when the controller retained a real authorized immutable
    /// execution snapshot. Blocked reports never fabricate contract fields.
    pub prepared: Option<PreparedPreflightContract>,
}

impl PreflightReport {
    pub fn is_runnable(&self) -> bool {
        self.blockers.is_empty() && self.prepared.is_some()
    }
}

/// Display-safe copy of the authoritative prepared snapshot metadata.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreparedPreflightContract {
    pub snapshot_digest: crate::product::ContentDigest,
    pub source_digest: crate::product::ContentDigest,
    pub receipt_digest: crate::product::ContentDigest,
    pub receipt_label: &'static str,
    pub analysis_ids: Vec<crate::product::ContentDigest>,
    pub task_count: usize,
    pub pvt_point_count: usize,
    pub target: &'static str,
    pub save_policy: &'static str,
    pub model_identity_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightToast {
    pub message: String,
    pub warning: bool,
}

/// Transient state for the simulation-preflight dialog and its one-frame
/// notification. Reports never survive a project or application reload.
#[derive(Debug, Clone, Default)]
pub struct PreflightDialogState {
    pub open: bool,
    pub report: Option<PreflightReport>,
    pub pending_toast: Option<PreflightToast>,
}

/// Domain projection selected in the mockup-specified notification center.
/// The underlying activity stream is never discarded when this changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationFilter {
    #[default]
    All,
    Jobs,
    Approvals,
    System,
}

impl NotificationFilter {
    pub const ALL: [Self; 4] = [Self::All, Self::Jobs, Self::Approvals, Self::System];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Jobs => "Jobs",
            Self::Approvals => "Approvals",
            Self::System => "System",
        }
    }
}

/// New workbench session state.  Durable layout preferences are serialized;
/// one-frame requests and open drawers are intentionally transient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchState {
    #[serde(default)]
    pub workspace: Workspace,
    #[serde(default = "default_true")]
    pub navigator_visible: bool,
    #[serde(default = "default_true")]
    pub inspector_visible: bool,
    #[serde(default)]
    pub console_visible: bool,
    #[serde(default)]
    pub console_maximized: bool,
    #[serde(default)]
    pub focus_mode: bool,
    /// Current viewport full-screen intent. Runtime-owned because the host
    /// window, browser, or mobile shell decides whether the request can be
    /// honored and must never restore a stale platform window state.
    #[serde(skip)]
    pub full_screen: bool,
    /// Capability-derived touch composition. Once a native touch event is
    /// observed it remains enabled for this process; browsers also refresh it
    /// from the exact `(pointer: coarse)` media query every frame.
    #[serde(skip)]
    pub coarse_pointer: bool,
    /// The Project Launcher is an application-level modal, not a workspace.
    #[serde(skip)]
    pub project_launcher_open: bool,
    /// Search text is transient and never becomes project state.
    #[serde(skip)]
    pub project_launcher_query: String,
    #[serde(skip)]
    pub project_launcher_sort: ProjectLauncherSort,
    /// Selected startup page and scanned native checkpoint catalog are
    /// application-modal runtime state, never project or user preferences.
    #[serde(skip)]
    pub project_launcher_page: ProjectLauncherPage,
    #[serde(skip)]
    pub(crate) project_launcher_recovery: super::recovery::RecoveryCatalog,
    /// Safe mode applies only to this process. The prior session remains the
    /// persisted source of truth for the next ordinary launch.
    #[serde(skip)]
    pub safe_mode: LocalSafeModeState,
    /// Focus is requested only on the frame in which the launcher opens.
    #[serde(skip)]
    pub focus_project_launcher_search: bool,
    #[serde(default = "default_navigator_width")]
    pub navigator_width: f32,
    #[serde(default = "default_inspector_width")]
    pub inspector_width: f32,
    #[serde(default = "default_console_height")]
    pub console_height: f32,
    #[serde(default)]
    pub console_page: ConsolePage,
    #[serde(default)]
    pub design_panel: DesignPanel,
    #[serde(default)]
    pub project_page: ProjectPage,
    #[serde(default)]
    pub verification_page: VerificationPage,
    #[serde(default)]
    pub models_page: ModelsPage,
    /// Analysis row whose configuration is shown in the simulation plan.
    #[serde(default = "default_analysis_index")]
    pub active_analysis: usize,
    /// Selected specification row in the verification matrix.
    #[serde(default)]
    pub selected_spec: Option<usize>,
    /// Selected model name within the currently selected model library.
    #[serde(default)]
    pub selected_model: Option<String>,
    /// Filter local to the simulation analysis catalog.
    #[serde(default)]
    pub analysis_query: String,
    #[serde(default)]
    pub navigator_query: String,
    #[serde(default)]
    pub command_query: String,
    /// Transactional project-name editor buffer. Runtime-only so a partially
    /// entered value can never be restored as authoritative project identity.
    #[serde(skip)]
    pub project_name_draft: String,
    /// Last project metadata validation error.
    #[serde(skip)]
    pub project_name_error: Option<String>,
    #[serde(skip)]
    pub drawer: Option<Drawer>,
    #[serde(skip)]
    pub workspace_history: Vec<Workspace>,
    /// One-frame request to focus the workspace navigator filter.
    #[serde(skip)]
    pub focus_navigator_search: bool,
    /// Filter text in the component chooser.
    #[serde(skip)]
    pub placement_query: String,
    /// One-frame request to focus the component chooser filter.
    #[serde(skip)]
    pub focus_placement_search: bool,
    /// Mockup-specified simulation-preflight workflow. Validation reports are
    /// revision-bound runtime evidence and therefore never serialized.
    #[serde(skip)]
    pub preflight: PreflightDialogState,
    /// Session activity center. Its records live in `UiSessionState::toasts`;
    /// only this transient presentation state belongs to the workbench.
    #[serde(skip)]
    pub notification_center_open: bool,
    #[serde(skip)]
    pub notification_filter: NotificationFilter,
}

const fn default_true() -> bool {
    true
}

const fn default_navigator_width() -> f32 {
    256.0
}

const fn default_inspector_width() -> f32 {
    312.0
}

const fn default_console_height() -> f32 {
    145.0
}

const fn default_analysis_index() -> usize {
    crate::common::simulation_analysis_tabs::TAB_TRANSIENT
}

impl Default for WorkbenchState {
    fn default() -> Self {
        Self {
            workspace: Workspace::Design,
            navigator_visible: true,
            inspector_visible: true,
            console_visible: false,
            console_maximized: false,
            focus_mode: false,
            full_screen: false,
            coarse_pointer: false,
            project_launcher_open: false,
            project_launcher_query: String::new(),
            project_launcher_sort: ProjectLauncherSort::LastOpened,
            project_launcher_page: ProjectLauncherPage::Projects,
            project_launcher_recovery: super::recovery::RecoveryCatalog::default(),
            safe_mode: LocalSafeModeState::default(),
            focus_project_launcher_search: false,
            navigator_width: default_navigator_width(),
            inspector_width: default_inspector_width(),
            console_height: default_console_height(),
            console_page: ConsolePage::Console,
            design_panel: DesignPanel::Navigator,
            project_page: ProjectPage::Dashboard,
            verification_page: VerificationPage::Cockpit,
            models_page: ModelsPage::Catalog,
            active_analysis: default_analysis_index(),
            selected_spec: None,
            selected_model: None,
            analysis_query: String::new(),
            navigator_query: String::new(),
            command_query: String::new(),
            project_name_draft: String::new(),
            project_name_error: None,
            drawer: None,
            workspace_history: vec![Workspace::Design],
            focus_navigator_search: false,
            placement_query: String::new(),
            focus_placement_search: false,
            preflight: PreflightDialogState::default(),
            notification_center_open: false,
            notification_filter: NotificationFilter::default(),
        }
    }
}

impl WorkbenchState {
    /// Whether a workbench-owned application modal has exclusive keyboard and
    /// pointer intent. Global shortcuts must not mutate the document behind
    /// these surfaces.
    pub fn application_modal_open(&self) -> bool {
        self.project_launcher_open || self.preflight.open || self.notification_center_open
    }

    pub fn open_project_launcher(&mut self) {
        self.project_launcher_open = true;
        self.project_launcher_page = ProjectLauncherPage::Projects;
        self.project_launcher_recovery.request_refresh();
        self.focus_project_launcher_search = true;
    }

    pub fn activate(&mut self, workspace: Workspace) {
        if self.workspace == workspace {
            self.close_drawer();
            return;
        }
        self.workspace = workspace;
        self.close_drawer();
        self.workspace_history.retain(|entry| *entry != workspace);
        self.workspace_history.push(workspace);
        self.workspace_history.truncate(16);
    }

    pub fn cycle_workspace(&mut self, backwards: bool) {
        let current = Workspace::ALL
            .iter()
            .position(|workspace| *workspace == self.workspace)
            .unwrap_or(0);
        let next = if backwards {
            (current + Workspace::ALL.len() - 1) % Workspace::ALL.len()
        } else {
            (current + 1) % Workspace::ALL.len()
        };
        self.activate(Workspace::ALL[next]);
    }

    pub fn toggle_drawer(&mut self, drawer: Drawer) {
        if self.drawer == Some(drawer) {
            self.close_drawer();
        } else {
            self.drawer = Some(drawer);
        }
    }

    pub fn close_drawer(&mut self) {
        self.drawer = None;
    }

    /// Dismiss the navigator's current presentation. Closing a compact
    /// drawer must not overwrite the saved desktop dock preference.
    pub fn dismiss_navigator(&mut self) {
        if self.drawer == Some(Drawer::Navigator) {
            self.close_drawer();
        } else {
            self.navigator_visible = false;
        }
    }

    /// Dismiss the inspector's current presentation. Closing a compact
    /// drawer must not overwrite the saved desktop dock preference.
    pub fn dismiss_inspector(&mut self) {
        if self.drawer == Some(Drawer::Inspector) {
            self.close_drawer();
        } else {
            self.inspector_visible = false;
        }
    }

    /// Clear a transient drawer as soon as the current responsive composition
    /// cannot present it. Navigator and inspector intentionally have distinct
    /// 821-1260 px behavior in the mockup.
    pub fn reconcile_drawer_mode(
        &mut self,
        navigator_uses_drawer: bool,
        inspector_uses_drawer: bool,
        workspaces_uses_drawer: bool,
    ) {
        let supported = match self.drawer {
            Some(Drawer::Navigator) => navigator_uses_drawer,
            Some(Drawer::Inspector) => inspector_uses_drawer,
            Some(Drawer::Workspaces) => workspaces_uses_drawer,
            None => true,
        };
        if !supported {
            self.close_drawer();
        }
    }

    pub fn reset_layout(&mut self) {
        self.navigator_visible = true;
        self.inspector_visible = true;
        self.console_visible = false;
        self.console_maximized = false;
        self.focus_mode = false;
        self.navigator_width = default_navigator_width();
        self.inspector_width = default_inspector_width();
        self.console_height = default_console_height();
        self.close_drawer();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_workspace_order_and_shortcuts_are_stable() {
        assert_eq!(Workspace::ALL.len(), 7);
        for (index, workspace) in Workspace::ALL.into_iter().enumerate() {
            assert_eq!(workspace.shortcut(), format!("Alt+{}", index + 1));
        }
    }

    #[test]
    fn responsive_width_classes_match_every_mockup_boundary() {
        assert_eq!(WidthClass::for_width(560.0), WidthClass::Phone);
        assert_eq!(WidthClass::for_width(561.0), WidthClass::Tablet);
        assert_eq!(WidthClass::for_width(820.0), WidthClass::Tablet);
        assert_eq!(WidthClass::for_width(821.0), WidthClass::Desktop);
        assert_eq!(WidthClass::for_width(1260.0), WidthClass::Desktop);
        assert_eq!(WidthClass::for_width(1261.0), WidthClass::Wide);
    }

    #[test]
    fn workspace_history_is_deduplicated_and_bounded() {
        let mut state = WorkbenchState::default();
        state.activate(Workspace::Results);
        state.activate(Workspace::Design);

        assert_eq!(state.workspace, Workspace::Design);
        assert_eq!(
            state.workspace_history,
            vec![Workspace::Results, Workspace::Design]
        );
    }

    #[test]
    fn toggling_the_active_drawer_closes_it_deterministically() {
        let mut state = WorkbenchState::default();

        state.toggle_drawer(Drawer::Navigator);
        assert_eq!(state.drawer, Some(Drawer::Navigator));
        state.toggle_drawer(Drawer::Navigator);
        assert_eq!(state.drawer, None);

        state.toggle_drawer(Drawer::Navigator);
        state.dismiss_navigator();
        assert!(state.navigator_visible);
        assert_eq!(state.drawer, None);
        state.dismiss_navigator();
        assert!(!state.navigator_visible);

        state.navigator_visible = true;
        state.toggle_drawer(Drawer::Navigator);
        state.toggle_drawer(Drawer::Inspector);
        assert_eq!(state.drawer, Some(Drawer::Inspector));
        state.dismiss_inspector();
        assert!(state.inspector_visible);
        assert_eq!(state.drawer, None);

        state.dismiss_inspector();
        assert!(!state.inspector_visible);
    }

    #[test]
    fn mixed_dock_composition_keeps_only_the_supported_drawer() {
        let mut state = WorkbenchState::default();
        state.toggle_drawer(Drawer::Navigator);
        state.reconcile_drawer_mode(false, true, false);
        assert_eq!(state.drawer, None);

        state.toggle_drawer(Drawer::Inspector);
        state.reconcile_drawer_mode(false, true, false);
        assert_eq!(state.drawer, Some(Drawer::Inspector));

        state.toggle_drawer(Drawer::Workspaces);
        state.reconcile_drawer_mode(false, true, false);
        assert_eq!(state.drawer, None);
    }

    #[test]
    fn application_modals_own_global_input_exclusively() {
        let mut state = WorkbenchState::default();
        assert!(!state.application_modal_open());

        state.project_launcher_open = true;
        assert!(state.application_modal_open());
        state.project_launcher_open = false;
        state.preflight.open = true;
        assert!(state.application_modal_open());
    }
}
