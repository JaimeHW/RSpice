//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

mod session_views;

pub use session_views::*;

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[cfg(any(test, target_arch = "wasm32"))]
use super::BrowserHistoryEffect;
use super::{
    RouteTransition, RouteTransitionSource, SurfaceArchetype, SurfaceId, SurfaceNavigation,
    SurfaceRoute, WorkspacePreset,
};

const NAVIGATION_SCHEMA_VERSION: u8 = 1;
/// Device-local shell geometry schema.
///
/// Version 1 adopts the redesigned workbench's responsive dock proportions.
/// Older sessions may contain explicit widths captured from the retired shell;
/// those values must not distort the redesigned schematic workspace forever.
const LAYOUT_SCHEMA_VERSION: u8 = 1;

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

/// Transactional workflow currently owned by the Simulation Studio surface.
///
/// These are editor drafts only. They are deliberately excluded from session
/// persistence so an interrupted or cancelled workflow can never become
/// authoritative project configuration after restart.
#[derive(Debug, Clone)]
pub enum SimulationWorkflowDialog {
    ClonePlan(ClonePlanDraft),
    DesignVariable(DesignVariableDraft),
    SavedOutput(SavedOutputDraft),
}

#[derive(Debug, Clone)]
pub struct ClonePlanDraft {
    pub name: String,
    pub copy_analyses_options: bool,
    pub copy_variables_outputs_specs: bool,
    pub copy_pvt_model_bindings: bool,
    pub copy_regression_baseline: bool,
    pub validation_error: Option<String>,
}

impl ClonePlanDraft {
    pub fn for_source(source_name: &str) -> Self {
        Self {
            name: format!("{source_name} · variant"),
            copy_analyses_options: true,
            copy_variables_outputs_specs: true,
            copy_pvt_model_bindings: true,
            copy_regression_baseline: false,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DesignVariableDraft {
    pub name: String,
    pub expression: String,
    pub quantity: usize,
    pub scope: usize,
    pub description: String,
    pub allowed_range: String,
    pub sweep_eligibility: usize,
    pub override_policy: usize,
    pub validation_error: Option<String>,
}

impl Default for DesignVariableDraft {
    fn default() -> Self {
        Self {
            name: "RLOAD_TEST".to_owned(),
            expression: "10 kohm".to_owned(),
            quantity: 0,
            scope: 0,
            description: "Verification load used by characterization plans".to_owned(),
            allowed_range: "1 kohm … 1 Mohm".to_owned(),
            sweep_eligibility: 0,
            override_policy: 0,
            validation_error: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SavedOutputDraft {
    pub kind: usize,
    pub name: String,
    pub expression: String,
    pub compatible_analyses: usize,
    pub save_policy: usize,
    pub precision: usize,
    pub streaming: usize,
    pub validation_error: Option<String>,
}

impl Default for SavedOutputDraft {
    fn default() -> Self {
        Self {
            kind: 0,
            name: "V(afe_out)".to_owned(),
            expression: "V(afe_out)".to_owned(),
            compatible_analyses: 0,
            save_policy: 0,
            precision: 0,
            streaming: 0,
            validation_error: None,
        }
    }
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
            Self::Simulate => "Simulation Studio",
            Self::Results => "Result document",
            Self::Verify => "Verification evidence",
            Self::Models => "Model binding",
            Self::Netlist => "Automation pipeline",
        }
    }

    pub const fn inspector_title(self) -> &'static str {
        match self {
            Self::Project => "Project details",
            Self::Design => "Inspector",
            Self::Simulate => "Analysis inspector",
            Self::Results => "Data inspector",
            Self::Verify => "Evidence inspector",
            Self::Models => "Model details",
            Self::Netlist => "Source inspector",
        }
    }
}

/// Device-local dock composition retained independently for each workspace.
/// Engineering documents and project state never enter this presentation
/// snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct WorkspaceLayoutState {
    pub navigator_visible: bool,
    pub inspector_visible: bool,
    pub console_visible: bool,
    pub focus_mode: bool,
    pub navigator_width: f32,
    pub navigator_width_custom: bool,
    pub inspector_width: f32,
    pub inspector_width_custom: bool,
    pub console_height: f32,
}

impl Default for WorkspaceLayoutState {
    fn default() -> Self {
        Self {
            navigator_visible: true,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            navigator_width: default_navigator_width(),
            navigator_width_custom: false,
            inspector_width: default_inspector_width(),
            inspector_width_custom: false,
            console_height: default_console_height(),
        }
    }
}

impl WorkspaceLayoutState {
    #[must_use]
    pub fn for_preset(preset: WorkspacePreset) -> Self {
        let mut layout = Self::default();
        match preset {
            WorkspacePreset::Engineering => {}
            WorkspacePreset::Canvas => layout.focus_mode = true,
            WorkspacePreset::Diagnostics => {
                layout.console_visible = true;
                layout.console_height = 260.0;
            }
        }
        layout
    }
}

/// Stable identity of one document presentation in the seven workspaces.
///
/// These IDs point only at authoritative project objects. Closing a tab
/// removes its presentation from the local session; it never deletes the
/// cell view, configured analysis, or immutable result dataset behind it.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WorkspaceDocumentId {
    Project,
    CellView(crate::state::CellViewRef),
    SimulationPlan,
    AnalysisSetup(crate::product::AnalysisInstanceId),
    ResultDataset(crate::product::DatasetId),
    VisualizationDocument(crate::product::ResultDocumentId),
    Verification,
    Models,
    NetlistSource,
}

impl WorkspaceDocumentId {
    pub const fn workspace(&self) -> Workspace {
        match self {
            Self::Project => Workspace::Project,
            Self::CellView(_) => Workspace::Design,
            Self::SimulationPlan | Self::AnalysisSetup(_) => Workspace::Simulate,
            Self::ResultDataset(_) | Self::VisualizationDocument(_) => Workspace::Results,
            Self::Verification => Workspace::Verify,
            Self::Models => Workspace::Models,
            Self::NetlistSource => Workspace::Netlist,
        }
    }
}

/// Runtime draft for the dataset-driven Create Result Document transaction.
///
/// IDs, rather than translated labels or row positions, cross the modal
/// boundary. The project-owned document is created only after the workflow
/// module revalidates every selection against current retained datasets.
#[derive(Debug, Clone)]
pub struct CreateResultDocumentDialogState {
    pub open: bool,
    pub name: String,
    pub name_touched: bool,
    pub dataset_id: Option<crate::product::DatasetId>,
    pub family_id: String,
    pub viewer_id: String,
    pub layout_id: String,
    pub validation_error: Option<String>,
}

impl Default for CreateResultDocumentDialogState {
    fn default() -> Self {
        Self {
            open: false,
            name: String::new(),
            name_touched: false,
            dataset_id: None,
            family_id: "waveform-worksheet".to_owned(),
            viewer_id: "viewer-waveform".to_owned(),
            layout_id: "two-linked-panes".to_owned(),
            validation_error: None,
        }
    }
}

/// Device-local open-document session. Engineering content continues to live
/// in its owning project models; this registry retains only active/closed tab
/// presentation state for each workspace.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WorkspaceDocumentRegistry {
    #[serde(default)]
    active: HashMap<Workspace, WorkspaceDocumentId>,
    #[serde(default)]
    closed: HashSet<WorkspaceDocumentId>,
}

impl WorkspaceDocumentRegistry {
    pub fn active(&self, workspace: Workspace) -> Option<&WorkspaceDocumentId> {
        self.active.get(&workspace)
    }

    pub fn activate(&mut self, document: WorkspaceDocumentId) {
        let workspace = document.workspace();
        self.closed.remove(&document);
        self.active.insert(workspace, document);
    }

    pub fn close(&mut self, document: &WorkspaceDocumentId) {
        self.closed.insert(document.clone());
        if self.active.get(&document.workspace()) == Some(document) {
            self.active.remove(&document.workspace());
        }
    }

    pub fn is_closed(&self, document: &WorkspaceDocumentId) -> bool {
        self.closed.contains(document)
    }

    pub fn retain_available(
        &mut self,
        workspace: Workspace,
        available: impl IntoIterator<Item = WorkspaceDocumentId>,
    ) {
        let available = available.into_iter().collect::<HashSet<_>>();
        self.closed
            .retain(|document| document.workspace() != workspace || available.contains(document));
        if self
            .active
            .get(&workspace)
            .is_some_and(|document| !available.contains(document))
        {
            self.active.remove(&workspace);
        }
    }
}

/// Personal engineering-navigation profile from the governed mockup.
///
/// Profiles only reduce everyday discovery. They never change project data,
/// route identity, entitlement, runtime availability, or qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EngineeringProfile {
    #[default]
    AnalogIc,
    RfMicrowave,
    SiPi,
    Power,
    Emerging,
    All,
}

impl EngineeringProfile {
    pub const ALL: [Self; 6] = [
        Self::AnalogIc,
        Self::RfMicrowave,
        Self::SiPi,
        Self::Power,
        Self::Emerging,
        Self::All,
    ];

    #[cfg(test)]
    pub const fn id(self) -> &'static str {
        match self {
            Self::AnalogIc => "analog-ic",
            Self::RfMicrowave => "rf-microwave",
            Self::SiPi => "si-pi",
            Self::Power => "power",
            Self::Emerging => "emerging",
            Self::All => "all",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::AnalogIc => "Analog & mixed-signal IC",
            Self::RfMicrowave => "RF, microwave & wireless",
            Self::SiPi => "SI, PI, package & PCB",
            Self::Power => "Power electronics",
            Self::Emerging => "Photonics & quantum",
            Self::All => "All engineering domains",
        }
    }

    pub const fn detail(self) -> &'static str {
        match self {
            Self::AnalogIc => {
                "Schematic, custom layout, AMS, RF/periodic, variation, reliability, PDK, model and sign-off workflows."
            }
            Self::RfMicrowave => {
                "Periodic, network, load-pull, EM, RF display, measurement and application-design workflows."
            }
            Self::SiPi => {
                "Channel, SerDes, PDN, package, board, EM, compliance and manufacturing workflows."
            }
            Self::Power => {
                "Converters, devices, magnetics, controls, electrothermal, reliability and lab-correlation workflows."
            }
            Self::Emerging => {
                "Electronic-photonic, field, heterogeneous integration and quantum electronics research workflows."
            }
            Self::All => {
                "Expose every installed specialist workspace. Best for evaluators, administrators and cross-domain teams."
            }
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

    pub const fn uses_bottom_navigation(self) -> bool {
        matches!(self, Self::Phone | Self::Tablet)
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub enum ConsolePage {
    #[default]
    Console,
    Interactive,
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

/// Safe-mode controls requested for this process. Capability-gated options are
/// rejected before activation, so every applied bit represents an enforced
/// runtime policy rather than presentation-only state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalSafeModeOptions {
    pub disable_third_party_extensions: bool,
    pub disable_gpu_acceleration: bool,
    pub isolate_prior_documents: bool,
    pub reset_layout: bool,
    pub open_project_read_only: bool,
}

impl Default for LocalSafeModeOptions {
    fn default() -> Self {
        Self {
            disable_third_party_extensions: true,
            // eframe selects the graphics adapter before this launcher exists.
            // The UI capability-gates this option until a software adapter can
            // actually be selected for the current platform.
            disable_gpu_acceleration: false,
            isolate_prior_documents: true,
            reset_layout: false,
            open_project_read_only: false,
        }
    }
}

impl LocalSafeModeOptions {
    pub const fn has_effect(self) -> bool {
        self.disable_third_party_extensions
            || self.disable_gpu_acceleration
            || self.isolate_prior_documents
            || self.reset_layout
            || self.open_project_read_only
    }
}

/// Where the validated close-project transaction should leave the user.
///
/// This is runtime-only intent. Keeping it beside the launcher state lets a
/// browser canonical-save continuation complete the exact originally requested
/// transition without inferring intent from whichever surface is visible later.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum ProjectCloseDestination {
    #[default]
    Launcher,
    EmptyWorkbench,
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

    pub(crate) const fn project_read_only(&self) -> bool {
        self.active && self.applied.open_project_read_only
    }
}

impl ConsolePage {
    pub const ALL: [Self; 5] = [
        Self::Console,
        Self::Problems,
        Self::Measurements,
        Self::TaskLog,
        Self::Interactive,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Console => "Console",
            Self::Interactive => "Interactive",
            Self::Problems => "Problems",
            Self::Measurements => "Measurements",
            Self::TaskLog => "Task log",
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
    #[serde(alias = "Cockpit", alias = "Specifications", alias = "Checks")]
    Yield,
    Corners,
    Tuning,
    Optimization,
    Reliability,
    #[serde(alias = "History")]
    Regression,
    Drc,
}

impl VerificationPage {
    /// Persisted route catalog. Physical DRC remains decodable for backward
    /// compatibility but is absent from navigation until layout/rule-deck
    /// evidence has a production executor.
    #[cfg(test)]
    pub const ALL: [Self; 7] = [
        Self::Yield,
        Self::Corners,
        Self::Tuning,
        Self::Optimization,
        Self::Reliability,
        Self::Regression,
        Self::Drc,
    ];

    pub const NAVIGATION: [Self; 6] = [
        Self::Yield,
        Self::Corners,
        Self::Tuning,
        Self::Optimization,
        Self::Reliability,
        Self::Regression,
    ];

    /// Whether this route is backed by an executable, retained-evidence
    /// workflow. Unavailable routes remain decodable for old sessions and may
    /// remain visible where the mockup requires a capability boundary, but are
    /// never selectable.
    pub const fn is_operational(self) -> bool {
        !matches!(self, Self::Drc)
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Yield => "PVT & Monte Carlo verification",
            Self::Corners => "Process-corner verification",
            Self::Tuning => "Live design-space exploration",
            Self::Optimization => "Optimization candidate",
            Self::Reliability => "Reliability and safe-operating-area verification",
            Self::Regression => "Golden regression comparison",
            Self::Drc => "Design-rule checking",
        }
    }
}

/// New workbench session state.  Durable layout preferences are serialized;
/// one-frame requests and open drawers are intentionally transient.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkbenchState {
    #[serde(default)]
    pub workspace: Workspace,
    /// Device-local application-window presentations and their exclusive
    /// document ownership. Engineering documents remain owned by their
    /// project stores; this registry only controls where each is presented.
    #[serde(default)]
    pub window_session: crate::workbench::lifecycle::window_session::WindowSessionRegistry,
    #[serde(default)]
    pub engineering_profile: EngineeringProfile,
    /// Version marker used to distinguish sessions that predate canonical
    /// routes from sessions whose current route is intentionally Design.
    #[serde(default)]
    navigation_schema_version: u8,
    /// Version marker for persisted device-local dock geometry. This is kept
    /// independent from navigation because a visual-shell migration must not
    /// rewrite the user's current route or engineering state.
    #[serde(default)]
    layout_schema_version: u8,
    /// Canonical current route plus bounded back/forward/recent task history.
    #[serde(default)]
    navigation: SurfaceNavigation,
    /// Recovery or deep-link diagnostic awaiting presentation in the activity
    /// stream. It is runtime state, never engineering or preference data.
    #[serde(skip)]
    route_diagnostic: Option<String>,
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
    /// Device-local presentation toggle for the mockup's equal schematic /
    /// result stage. The immutable datasets and result-document state remain
    /// owned by `SimulationState` and `UiSessionState::results`; this flag
    /// only controls whether that existing document is projected beside an
    /// eligible primary workspace.
    #[serde(default)]
    pub split_with_results: bool,
    /// Session-only guard for a hierarchy view intentionally opened as a
    /// read-only reference. The underlying library and schematic document
    /// remain unchanged, so reopening the view normally restores editability.
    #[serde(skip)]
    pub hierarchy_reference_read_only: bool,
    /// Current viewport full-screen intent. Runtime-owned because the host
    /// window, browser, or mobile shell decides whether the request can be
    /// honored and must never restore a stale platform window state.
    #[serde(skip)]
    pub full_screen: bool,
    /// RSpice-owned presentation state. Unlike `full_screen`, this also
    /// covers the mockup's "Active canvas only" scope and remains true while
    /// application chrome is intentionally suppressed.
    #[serde(skip)]
    pub full_screen_presentation: bool,
    /// Whether the current full-screen transaction temporarily suppresses
    /// the Navigator and Inspector. Their durable visibility flags are never
    /// rewritten, so exiting restores the exact prior dock composition.
    #[serde(skip)]
    pub full_screen_hide_context_panels: bool,
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
    #[serde(skip)]
    pub project_launcher_filter: ProjectLauncherFilter,
    /// Selected startup page and scanned native checkpoint catalog are
    /// application-modal runtime state, never project or user preferences.
    #[serde(skip)]
    pub project_launcher_page: ProjectLauncherPage,
    #[serde(skip)]
    pub(crate) project_launcher_recovery: crate::workbench::lifecycle::recovery::RecoveryCatalog,
    /// Safe mode applies only to this process. The prior session remains the
    /// persisted source of truth for the next ordinary launch.
    #[serde(skip)]
    pub safe_mode: LocalSafeModeState,
    /// Destination owned by an in-flight close-project review. It survives an
    /// asynchronous canonical-save continuation but is never persisted.
    #[serde(skip)]
    pub(crate) project_close_destination: ProjectCloseDestination,
    /// Focus is requested only on the frame in which the launcher opens.
    #[serde(skip)]
    pub focus_project_launcher_search: bool,
    #[serde(default = "default_navigator_width")]
    pub navigator_width: f32,
    /// True after the user explicitly resizes the navigator. Until then the
    /// mockup's responsive 18vw clamp remains authoritative.
    #[serde(default)]
    pub navigator_width_custom: bool,
    #[serde(default = "default_inspector_width")]
    pub inspector_width: f32,
    /// True after the user explicitly resizes the inspector. Until then the
    /// mockup's responsive 22vw clamp remains authoritative.
    #[serde(default)]
    pub inspector_width_custom: bool,
    #[serde(default = "default_console_height")]
    pub console_height: f32,
    /// Independent device-local dock snapshots for every visited workspace.
    #[serde(default)]
    workspace_layouts: HashMap<Workspace, WorkspaceLayoutState>,
    #[serde(default)]
    pub console_page: ConsolePage,
    #[serde(default)]
    pub design_panel: DesignPanel,
    /// Active and locally closed document presentations in each workspace.
    /// Stable object IDs ensure a reordered run history or analysis plan does
    /// not silently retarget a restored tab.
    #[serde(default)]
    pub documents: WorkspaceDocumentRegistry,
    #[serde(default)]
    pub project_page: ProjectPage,
    /// Device-local filters and row selections for the Project workspace.
    /// They affect presentation only and never participate in project
    /// revisioning, execution snapshots, or recovery checkpoints.
    #[serde(default)]
    pub project_library_filter: String,
    #[serde(default)]
    pub project_dependency_filter: String,
    #[serde(default)]
    pub project_dependency_selection: Option<String>,
    #[serde(default)]
    pub project_recovery_filter: String,
    #[serde(default)]
    pub project_checkpoint_selection: Option<String>,
    /// Device-local projection retained by the Library/Cellview specialist
    /// route. Authoritative library, cellview, symbol, and form records remain
    /// in the project domains.
    #[serde(default)]
    pub library_cellview_page: LibraryCellviewPage,
    #[serde(default)]
    pub verification_page: VerificationPage,
    #[serde(default)]
    pub verification: VerificationSessionState,
    #[serde(default)]
    pub models_page: ModelsPage,
    /// Native confirmation owner for detaching every executable source from a
    /// shipped pack. This is never restored across an application session.
    #[serde(skip)]
    pub model_catalog_detach_pack: Option<String>,
    /// Session-local project catalog pins. They are navigation preferences,
    /// not project evidence or release state.
    #[serde(default)]
    pub model_catalog_pinned: HashSet<String>,
    /// Device-local Include Graph depth control. This changes only the graph
    /// projection; the complete authenticated closure remains authoritative.
    #[serde(default)]
    pub model_include_direct_only: bool,
    /// Stable source selection shown by the Include Graph inspector.
    #[serde(default)]
    pub model_include_selected_library: Option<String>,
    #[serde(default)]
    pub model_include_selected_source: Option<PathBuf>,
    /// Presentation-only definition table controls for the Include Graph.
    #[serde(default)]
    pub model_include_definition_query: String,
    #[serde(default)]
    pub model_include_exceptions_only: bool,
    /// Models & PDKs catalog filters and stable presentation selections.
    /// Model, source, symbol, and PDK records remain in their owning domains.
    #[serde(default)]
    pub models_view: ModelsWorkbenchViewState,
    /// Analysis row whose configuration is shown in the simulation plan.
    /// Retained only to migrate pre-instance session selection.
    #[serde(default = "default_analysis_index")]
    pub active_analysis: usize,
    /// Stable analysis instance whose configuration is shown in the plan.
    #[serde(default)]
    pub active_analysis_instance: Option<crate::product::AnalysisInstanceId>,
    /// Device-local Simulation Studio viewport. Keeping this outside egui's
    /// transient widget memory prevents a committed form edit from snapping
    /// the editor back to the top when the selected instance is rebuilt.
    #[serde(skip)]
    pub simulation_surface_scroll_y: f32,
    /// One-frame correction for structural edits above the stacked analysis
    /// editor. It is consumed after egui reports the current ScrollArea offset.
    #[serde(skip)]
    pub simulation_surface_pending_scroll_delta_y: f32,
    /// Screen-space top edge of the selected analysis form immediately before
    /// a stacked structural edit. The next frame measures the actual displaced
    /// position so scroll compensation follows egui's rendered geometry and
    /// status-message height changes cannot move the editable controls.
    #[serde(skip)]
    pub simulation_surface_editor_anchor_y: Option<f32>,
    /// Last analysis lifecycle outcome announced by the transaction owner.
    #[serde(skip)]
    pub analysis_lifecycle_status: String,
    /// Active Simulation Studio transaction. Drafts are runtime-only and are
    /// committed atomically by the dialog's primary action.
    #[serde(skip)]
    pub simulation_workflow: Option<SimulationWorkflowDialog>,
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
    /// The inspector's open inline-edit session. Runtime-only: a partially
    /// typed instance name must never be restored as authoritative design
    /// data, and the retained undo snapshot belongs to this process.
    #[serde(skip)]
    pub inline_edit: InlineEdit,
    #[serde(skip)]
    pub drawer: Option<Drawer>,
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
    /// Runtime-only Jobs manager review context. Job and run records live in
    /// `SimulationState`; this state owns presentation only.
    #[serde(skip)]
    pub jobs_manager: JobsManagerState,
    /// Personal specialist discovery metadata plus task-local browser state.
    /// The owned engineering documents remain in their canonical surfaces.
    #[serde(default)]
    pub specialist_tool_browser: SpecialistToolBrowserState,
    /// Persistent presentation document for Visualization Studio. It stores
    /// stable viewer composition and annotation identities only; immutable
    /// result samples remain owned by the result datasets.
    #[serde(default)]
    pub visualization_studio:
        crate::workbench::documents::visualization_studio::VisualizationStudioState,
    /// Runtime-only draft for the project-owned result-document transaction.
    #[serde(skip)]
    pub create_result_document: CreateResultDocumentDialogState,
    /// Session-local selection and editor drafts for the project-owned report
    /// documents.
    #[serde(default)]
    pub report_authoring: ReportAuthoringState,
    /// Runtime-only transaction and presentation state for the project-owned
    /// device-model editor. Committed source and qualification records remain
    /// in the project model-library domain.
    #[serde(skip)]
    pub model_editor: crate::workbench::documents::model_editor::ModelEditorState,
    /// Runtime-only selection and transactional drafts for measurement
    /// correlation. Committed suites and evidence remain in the project-owned
    /// model-library domain.
    #[serde(skip)]
    pub model_correlation:
        crate::workbench::documents::model_correlation::ModelCorrelationWorkspaceState,
    /// Session activity center. Its records live in `UiSessionState::toasts`;
    /// only this transient presentation state belongs to the workbench.
    #[serde(skip)]
    pub notification_center_open: bool,
    #[serde(skip)]
    pub notification_filter: NotificationFilter,
    /// Review filters plus transient presentation state for the canonical
    /// application-session-owned capability matrix. Individual fields define
    /// their own persistence boundary.
    #[serde(default)]
    pub capability_matrix: CapabilityMatrixState,
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
    crate::workbench::simulation_analysis_tabs::TAB_TRANSIENT
}

impl Default for WorkbenchState {
    fn default() -> Self {
        Self {
            workspace: Workspace::Design,
            window_session: crate::workbench::lifecycle::window_session::WindowSessionRegistry::default(),
            engineering_profile: EngineeringProfile::AnalogIc,
            navigation_schema_version: NAVIGATION_SCHEMA_VERSION,
            layout_schema_version: LAYOUT_SCHEMA_VERSION,
            navigation: SurfaceNavigation::default(),
            route_diagnostic: None,
            navigator_visible: true,
            inspector_visible: true,
            console_visible: false,
            console_maximized: false,
            focus_mode: false,
            split_with_results: false,
            hierarchy_reference_read_only: false,
            full_screen: false,
            full_screen_presentation: false,
            full_screen_hide_context_panels: false,
            coarse_pointer: false,
            project_launcher_open: false,
            project_launcher_query: String::new(),
            project_launcher_sort: ProjectLauncherSort::LastOpened,
            project_launcher_filter: ProjectLauncherFilter::All,
            project_launcher_page: ProjectLauncherPage::Projects,
            project_launcher_recovery: crate::workbench::lifecycle::recovery::RecoveryCatalog::default(),
            safe_mode: LocalSafeModeState::default(),
            project_close_destination: ProjectCloseDestination::Launcher,
            focus_project_launcher_search: false,
            navigator_width: default_navigator_width(),
            navigator_width_custom: false,
            inspector_width: default_inspector_width(),
            inspector_width_custom: false,
            console_height: default_console_height(),
            workspace_layouts: HashMap::new(),
            console_page: ConsolePage::Console,
            design_panel: DesignPanel::Navigator,
            documents: WorkspaceDocumentRegistry::default(),
            project_page: ProjectPage::Overview,
            project_library_filter: String::new(),
            project_dependency_filter: String::new(),
            project_dependency_selection: None,
            project_recovery_filter: String::new(),
            project_checkpoint_selection: None,
            library_cellview_page: LibraryCellviewPage::Libraries,
            verification_page: VerificationPage::Yield,
            verification: VerificationSessionState::default(),
            models_page: ModelsPage::Models,
            model_catalog_detach_pack: None,
            model_catalog_pinned: HashSet::new(),
            model_include_direct_only: false,
            model_include_selected_library: None,
            model_include_selected_source: None,
            model_include_definition_query: String::new(),
            model_include_exceptions_only: false,
            models_view: ModelsWorkbenchViewState::default(),
            active_analysis: default_analysis_index(),
            active_analysis_instance: None,
            simulation_surface_scroll_y: 0.0,
            simulation_surface_pending_scroll_delta_y: 0.0,
            simulation_surface_editor_anchor_y: None,
            analysis_lifecycle_status: "No lifecycle command has been committed this session."
                .to_owned(),
            simulation_workflow: None,
            selected_spec: None,
            selected_model: None,
            analysis_query: String::new(),
            navigator_query: String::new(),
            command_query: String::new(),
            inline_edit: InlineEdit::default(),
            drawer: None,
            focus_navigator_search: false,
            placement_query: String::new(),
            focus_placement_search: false,
            preflight: PreflightDialogState::default(),
            jobs_manager: JobsManagerState::default(),
            specialist_tool_browser: SpecialistToolBrowserState::default(),
            visualization_studio: crate::workbench::documents::visualization_studio::VisualizationStudioState::default(),
            create_result_document: CreateResultDocumentDialogState::default(),
            report_authoring: ReportAuthoringState::default(),
            model_editor: crate::workbench::documents::model_editor::ModelEditorState::default(),
            model_correlation: crate::workbench::documents::model_correlation::ModelCorrelationWorkspaceState::default(),
            notification_center_open: false,
            notification_filter: NotificationFilter::default(),
            capability_matrix: CapabilityMatrixState::default(),
        }
    }
}

impl WorkbenchState {
    /// Whether a workbench-owned application modal has exclusive keyboard and
    /// pointer intent. Global shortcuts must not mutate the document behind
    /// these surfaces.
    pub fn application_modal_open(&self) -> bool {
        self.project_launcher_open
            || self.preflight.open
            || self.notification_center_open
            || self.model_correlation.dialog_open()
            || self.create_result_document.open
            || self.model_catalog_detach_pack.is_some()
            || self.simulation_workflow.is_some()
            || self.verification.regression_baseline_picker_open
            || self.verification.tuning_review_open
            || matches!(
                self.current_route().surface_id(),
                SurfaceId::ProjectLauncher
                    | SurfaceId::Preferences
                    | SurfaceId::DesignManagement
                    | SurfaceId::AccountOrganization
                    | SurfaceId::JobsManager
                    | SurfaceId::SpecialistToolBrowser
                    | SurfaceId::NotificationCenter
                    | SurfaceId::FeatureAvailability
            )
    }

    pub fn open_project_launcher(&mut self) {
        let route = SurfaceRoute::surface(SurfaceId::ProjectLauncher);
        if self.current_route() != route
            && let Err(error) = self.navigate(route, RouteTransitionSource::User)
        {
            self.record_route_diagnostic(format!(
                "The Project Launcher could not be opened: {error}"
            ));
            return;
        }
        self.project_launcher_open = true;
        self.project_launcher_page = ProjectLauncherPage::Projects;
        self.project_launcher_recovery.request_refresh();
        self.focus_project_launcher_search = true;
    }

    pub(crate) fn begin_project_close(&mut self, destination: ProjectCloseDestination) {
        self.project_close_destination = destination;
    }

    pub(crate) fn cancel_project_close(&mut self) {
        self.project_close_destination = ProjectCloseDestination::Launcher;
    }

    pub(crate) fn take_project_close_destination(&mut self) -> ProjectCloseDestination {
        std::mem::take(&mut self.project_close_destination)
    }

    pub fn activate(&mut self, workspace: Workspace) {
        if let Err(error) = self.navigate(
            SurfaceRoute::surface(SurfaceId::from_workspace(workspace)),
            RouteTransitionSource::User,
        ) {
            self.record_route_diagnostic(format!(
                "The requested primary workspace could not be opened: {error}"
            ));
        }
    }

    /// Commit one canonical route transition and update the primary-workspace
    /// rendering projection when the destination is a primary surface.
    pub fn navigate(
        &mut self,
        route: SurfaceRoute,
        source: RouteTransitionSource,
    ) -> Result<RouteTransition, super::SurfaceRouteUnavailable> {
        crate::workbench::routing::availability::require_available(route)?;
        let previous = self.navigation.current();
        self.reconcile_workspace_layout(previous, route);
        let transition = self.navigation.navigate(route, source);
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = route.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        self.close_drawer();
        Ok(transition)
    }

    /// Replace the current route during startup canonicalization without
    /// creating a back entry.
    pub fn replace_route(
        &mut self,
        route: SurfaceRoute,
        source: RouteTransitionSource,
    ) -> Result<(), super::SurfaceRouteUnavailable> {
        crate::workbench::routing::availability::require_available(route)?;
        let previous = self.navigation.current();
        self.reconcile_workspace_layout(previous, route);
        self.navigation.replace(route, source);
        self.reconcile_capability_matrix_route(previous, route);
        self.reconcile_specialist_tool_browser_route(previous, route);
        if let Some(workspace) = route.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        self.close_drawer();
        Ok(())
    }

    /// Reconcile a restored session. Legacy sessions are migrated from the
    /// persisted workspace; current sessions treat the canonical route as the
    /// source of truth and keep the workspace only as a render projection.
    pub fn reconcile_restored_navigation(&mut self) {
        if self.layout_schema_version != LAYOUT_SCHEMA_VERSION {
            self.navigator_width = default_navigator_width();
            self.navigator_width_custom = false;
            self.inspector_width = default_inspector_width();
            self.inspector_width_custom = false;
            for layout in self.workspace_layouts.values_mut() {
                layout.navigator_width = default_navigator_width();
                layout.navigator_width_custom = false;
                layout.inspector_width = default_inspector_width();
                layout.inspector_width_custom = false;
            }
            self.layout_schema_version = LAYOUT_SCHEMA_VERSION;
        }
        if !self.verification_page.is_operational() {
            self.route_diagnostic = Some(format!(
                "The restored {} verification page was not opened because its executable evidence pipeline is unavailable.",
                self.verification_page.label()
            ));
            self.verification_page = VerificationPage::Yield;
        }
        if self.navigation.recovered_invalid_routes() {
            self.route_diagnostic = Some(
                "Malformed routes were removed from restored task history; project and document state were preserved."
                    .to_owned(),
            );
            self.navigation.acknowledge_recovery();
        }
        if self.navigation_schema_version != NAVIGATION_SCHEMA_VERSION {
            self.navigation.replace(
                SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace)),
                RouteTransitionSource::Restore,
            );
            self.navigation_schema_version = NAVIGATION_SCHEMA_VERSION;
        } else {
            let current = self.navigation.current();
            if let Err(error) = crate::workbench::routing::availability::require_available(current)
            {
                self.route_diagnostic = Some(format!(
                    "The restored route was not opened because its executor is unavailable: {error}"
                ));
                self.navigation.replace(
                    SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace)),
                    RouteTransitionSource::Restore,
                );
            } else if let Some(workspace) = current.surface_id().owner_workspace() {
                self.workspace = workspace;
            }
        }
        let removed = self.navigation.retain_history(|route| {
            crate::workbench::routing::availability::route_availability(route).can_open()
        });
        if removed && self.route_diagnostic.is_none() {
            self.route_diagnostic = Some(
                "Unavailable routes were removed from restored task history; project and document state were preserved."
                    .to_owned(),
            );
        }
        self.specialist_tool_browser.normalize();
        self.normalize_visualization_studio();
    }

    /// Repair presentation-only Visualization Studio state restored from a
    /// prior or malformed session. Stable pane order is preserved; duplicate
    /// or zero identities are reassigned instead of silently dropping a
    /// configured viewer or annotation.
    fn normalize_visualization_studio(&mut self) {
        let studio = &mut self.visualization_studio;
        studio.zoom = if studio.zoom.is_finite() {
            studio.zoom.clamp(0.25, 8.0)
        } else {
            1.0
        };
        studio.revision = studio.revision.max(1);
        studio.tile_memory_mib = studio.tile_memory_mib.clamp(64, 16_384);
        studio.significant_digits = studio.significant_digits.clamp(3, 17);

        if crate::results::viewer_catalog::viewer_document(&studio.selected_viewer_document)
            .is_none()
        {
            studio.selected_viewer_document = "viewer-waveform".to_owned();
        }

        let greatest_restored_identity = studio
            .panes
            .iter()
            .map(|pane| pane.id)
            .chain(studio.annotations.iter().map(|annotation| annotation.id))
            .chain(studio.markers.iter().map(|marker| marker.id))
            .max()
            .unwrap_or_default();
        let mut next_identity = studio
            .next_identity
            .max(greatest_restored_identity.saturating_add(1))
            .max(1);
        let mut used = HashSet::new();

        studio.panes.retain_mut(|pane| {
            let Some(canonical_document_id) = result_viewer_document_id(pane.viewer) else {
                // Dataset-native projections (currently Manifest) are owned
                // by the Results frame and can never become Visualization
                // Studio panes. Reject malformed/restored presentation state
                // instead of assigning it a misleading viewer document.
                return false;
            };
            if pane.viewer_document_id != canonical_document_id {
                pane.viewer_document_id = canonical_document_id.to_owned();
            }
            if pane.id != 0 && used.insert(pane.id) {
                return true;
            }
            let Some(replacement) = allocate_restored_identity(&used, &mut next_identity) else {
                return false;
            };
            pane.id = replacement;
            used.insert(replacement);
            true
        });

        studio.annotations.retain_mut(|annotation| {
            if !annotation.x.is_finite() {
                return false;
            }
            if annotation.id != 0 && used.insert(annotation.id) {
                return true;
            }
            let Some(replacement) = allocate_restored_identity(&used, &mut next_identity) else {
                return false;
            };
            annotation.id = replacement;
            used.insert(replacement);
            true
        });

        studio.markers.retain_mut(|marker| {
            if !marker.x.is_finite() || !marker.y.is_finite() {
                return false;
            }
            if marker.id != 0 && used.insert(marker.id) {
                return true;
            }
            let Some(replacement) = allocate_restored_identity(&used, &mut next_identity) else {
                return false;
            };
            marker.id = replacement;
            marker.label = format!("M{replacement}");
            used.insert(replacement);
            true
        });

        if studio
            .active_pane
            .is_none_or(|id| !studio.panes.iter().any(|pane| pane.id == id))
        {
            studio.active_pane = studio.panes.first().map(|pane| pane.id);
        }
        if let Some(active) = studio.active_pane
            && let Some(pane) = studio.panes.iter().find(|pane| pane.id == active)
        {
            studio.selected_viewer_document = pane.viewer_document_id.clone();
        }
        studio.next_identity = used
            .iter()
            .copied()
            .max()
            .unwrap_or_default()
            .saturating_add(1)
            .max(next_identity)
            .max(1);
    }

    pub fn record_route_diagnostic(&mut self, diagnostic: impl Into<String>) {
        self.route_diagnostic = Some(diagnostic.into());
    }

    pub fn take_route_diagnostic(&mut self) -> Option<String> {
        self.route_diagnostic.take()
    }

    #[must_use]
    pub const fn current_route(&self) -> SurfaceRoute {
        self.navigation.current()
    }

    pub(crate) fn project_application_window(
        &mut self,
        workspace: Workspace,
        route: SurfaceRoute,
        layout: WorkspaceLayoutState,
    ) {
        self.workspace = workspace;
        self.navigation.project_transient(route);
        self.apply_workspace_layout(layout);
    }

    /// Whether the foreground route is one of the three authoring/setup
    /// stages that the upgraded mockup permits beside a retained result
    /// document. Checking both route and workspace fails closed while a
    /// manager or specialist surface temporarily retains an owner workspace.
    #[must_use]
    pub fn supports_results_split(&self) -> bool {
        matches!(
            (self.workspace, self.current_route().surface_id()),
            (Workspace::Design, SurfaceId::Design)
                | (Workspace::Netlist, SurfaceId::Netlist)
                | (Workspace::Simulate, SurfaceId::Simulate)
        )
    }

    /// Effective split-stage visibility. A remembered presentation choice
    /// never creates a blank or misleading secondary pane after the project
    /// or its retained evidence goes away.
    #[must_use]
    pub fn results_split_visible(&self, project_open: bool, has_retained_result: bool) -> bool {
        self.split_with_results
            && project_open
            && has_retained_result
            && self.supports_results_split()
    }

    /// Immediate in-application return destination, if one exists. The
    /// mutable navigation stacks remain session-owned so surfaces cannot
    /// bypass route availability and reconciliation.
    #[must_use]
    pub fn previous_route(&self) -> Option<SurfaceRoute> {
        self.navigation.back_entries().last().copied()
    }

    /// Number of known in-application predecessors. Surfaces may use this to
    /// choose a fail-safe close strategy without gaining mutable stack access.
    #[must_use]
    pub fn back_route_count(&self) -> usize {
        self.navigation.back_entries().len()
    }

    /// Only the browser build reads this: `app::apply_browser_history_delta`
    /// refuses a forward jump that would leave the in-app task stack.
    #[must_use]
    #[cfg(target_arch = "wasm32")]
    pub fn forward_route_count(&self) -> usize {
        self.navigation.forward_entries().len()
    }

    pub fn navigate_back(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        let destination = self.navigation.back_entries().last().copied()?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_back(source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    /// Atomically return across up to `count` known routes, reconciling the
    /// foreground surface only once from the original route to the final
    /// destination.
    pub fn navigate_back_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let available = count.min(self.navigation.back_entries().len());
        let destination = *self.navigation.back_entries().get(
            self.navigation
                .back_entries()
                .len()
                .saturating_sub(available),
        )?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_back_steps(count, source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    #[cfg(test)]
    pub fn navigate_forward(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        self.navigate_forward_steps(1, source)
    }

    /// Atomically advance across up to `count` known routes, reconciling the
    /// foreground surface only once from the original route to the final
    /// destination.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn navigate_forward_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let available = count.min(self.navigation.forward_entries().len());
        let destination = *self.navigation.forward_entries().get(
            self.navigation
                .forward_entries()
                .len()
                .saturating_sub(available),
        )?;
        self.reconcile_workspace_layout(self.navigation.current(), destination);
        let transition = self.navigation.go_forward_steps(count, source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        self.reconcile_specialist_tool_browser_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().owner_workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    fn reconcile_capability_matrix_route(&mut self, previous: SurfaceRoute, current: SurfaceRoute) {
        let manager = SurfaceId::FeatureAvailability;
        if previous.surface_id() == current.surface_id()
            || (previous.surface_id() != manager && current.surface_id() != manager)
        {
            return;
        }

        // The section picker and scroll position are local presentation state
        // for a single open document. Closing or entering the manager creates
        // a fresh Platforms-first reading position and dismisses drilldowns.
        self.capability_matrix.section = CapabilityMatrixSection::Platforms;
        self.capability_matrix.scroll_offset = 0.0;
        self.capability_matrix.last_document_compact = None;
        self.capability_matrix.drilldown = None;
        self.capability_matrix.drilldown_scroll_offset = 0.0;
        self.capability_matrix.interoperability_section = InteroperabilitySection::default();
        self.capability_matrix.interoperability_domain = InteroperabilityDomain::default();
        self.capability_matrix.interoperability_support_level =
            InteroperabilitySupportLevel::default();
    }

    fn reconcile_specialist_tool_browser_route(
        &mut self,
        previous: SurfaceRoute,
        current: SurfaceRoute,
    ) {
        let browser = SurfaceId::SpecialistToolBrowser;
        if previous.surface_id() != browser && current.surface_id() == browser {
            self.specialist_tool_browser.normalize();
            self.specialist_tool_browser.focus_search = true;
        }
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn take_browser_history_effect(&mut self) -> Option<BrowserHistoryEffect> {
        self.navigation.take_browser_effect()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn has_pending_browser_history_effects(&self) -> bool {
        self.navigation.has_pending_browser_effects()
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn clear_browser_history_effects(&mut self) {
        self.navigation.clear_browser_effects();
    }

    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn take_browser_history_effect_queue_overflowed(&mut self) -> bool {
        self.navigation.take_browser_effect_queue_overflowed()
    }

    /// Drop restored traversal stacks when a new browser process takes
    /// ownership of the current URL. The active route and workspace remain
    /// unchanged; only host-history authority is reset.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub(crate) fn reset_navigation_history_for_fresh_browser_session(&mut self) {
        self.navigation.reset_history();
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

    /// Apply a reviewed workspace preset immediately and retain that exact
    /// composition for the active workspace.
    pub fn apply_workspace_preset(&mut self, preset: WorkspacePreset) {
        self.apply_workspace_layout(WorkspaceLayoutState::for_preset(preset));
        self.capture_workspace_layout(self.workspace);
    }

    /// Apply the console launch choice immediately. Errors and explicit
    /// Problems navigation may still open it later through their own owners.
    pub fn apply_console_launch_behavior(&mut self, open: bool) {
        self.focus_mode = false;
        self.console_visible = open;
        self.console_maximized = false;
        self.capture_workspace_layout(self.workspace);
    }

    #[must_use]
    pub fn workspace_layout(&self, workspace: Workspace) -> WorkspaceLayoutState {
        if workspace == self.workspace {
            return self.current_workspace_layout();
        }
        self.workspace_layouts
            .get(&workspace)
            .copied()
            .unwrap_or_default()
    }

    pub fn apply_preset_to_all_workspaces(&mut self, preset: WorkspacePreset) {
        let layout = WorkspaceLayoutState::for_preset(preset);
        for workspace in Workspace::ALL {
            self.workspace_layouts.insert(workspace, layout);
        }
        self.apply_workspace_layout(layout);
    }

    /// Apply the mockup's results-review composition: wide plot/inspector,
    /// hidden navigator, and collapsed console.
    pub fn apply_results_review_layout(&mut self) {
        let layout = WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            inspector_width: 332.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        };
        self.apply_workspace_layout(layout);
        self.capture_workspace_layout(self.workspace);
    }

    pub fn apply_results_review_to_all_workspaces(&mut self) {
        let layout = WorkspaceLayoutState {
            navigator_visible: false,
            inspector_visible: true,
            console_visible: false,
            focus_mode: false,
            inspector_width: 332.0,
            inspector_width_custom: true,
            ..WorkspaceLayoutState::default()
        };
        for workspace in Workspace::ALL {
            self.workspace_layouts.insert(workspace, layout);
        }
        self.apply_workspace_layout(layout);
    }

    pub(crate) fn current_workspace_layout(&self) -> WorkspaceLayoutState {
        WorkspaceLayoutState {
            navigator_visible: self.navigator_visible,
            inspector_visible: self.inspector_visible,
            console_visible: self.console_visible,
            focus_mode: self.focus_mode,
            navigator_width: self.navigator_width,
            navigator_width_custom: self.navigator_width_custom,
            inspector_width: self.inspector_width,
            inspector_width_custom: self.inspector_width_custom,
            console_height: self.console_height,
        }
    }

    fn capture_workspace_layout(&mut self, workspace: Workspace) {
        let layout = self.current_workspace_layout();
        self.workspace_layouts.insert(workspace, layout);
    }

    pub(crate) fn apply_workspace_layout(&mut self, layout: WorkspaceLayoutState) {
        self.navigator_visible = layout.navigator_visible;
        self.inspector_visible = layout.inspector_visible;
        self.console_visible = layout.console_visible;
        self.console_maximized = false;
        self.focus_mode = layout.focus_mode;
        self.navigator_width = layout.navigator_width;
        self.navigator_width_custom = layout.navigator_width_custom;
        self.inspector_width = layout.inspector_width;
        self.inspector_width_custom = layout.inspector_width_custom;
        self.console_height = layout.console_height;
        self.close_drawer();
    }

    fn reconcile_workspace_layout(&mut self, previous: SurfaceRoute, current: SurfaceRoute) {
        let previous_workspace = previous.surface_id().owner_workspace();
        let current_workspace = current.surface_id().owner_workspace();
        if previous_workspace == current_workspace {
            return;
        }
        if let Some(previous_workspace) = previous_workspace {
            self.capture_workspace_layout(previous_workspace);
        }
        if let Some(current_workspace) = current_workspace {
            let layout = self
                .workspace_layouts
                .get(&current_workspace)
                .copied()
                .unwrap_or_default();
            self.apply_workspace_layout(layout);
        }
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
        self.navigator_width_custom = false;
        self.inspector_width = default_inspector_width();
        self.inspector_width_custom = false;
        self.console_height = default_console_height();
        self.close_drawer();
        self.capture_workspace_layout(self.workspace);
    }
}

fn allocate_restored_identity(used: &HashSet<u64>, next_identity: &mut u64) -> Option<u64> {
    while *next_identity == 0 || used.contains(next_identity) {
        *next_identity = next_identity.checked_add(1)?;
    }
    let allocated = *next_identity;
    *next_identity = next_identity.saturating_add(1);
    Some(allocated)
}

const fn result_viewer_document_id(viewer: super::ResultViewer) -> Option<&'static str> {
    Some(match viewer {
        super::ResultViewer::Manifest => return None,
        super::ResultViewer::Waves | super::ResultViewer::DcSweep => "viewer-waveform",
        super::ResultViewer::Bode
        | super::ResultViewer::Nyquist
        | super::ResultViewer::NoiseContrib => "viewer-bode",
        super::ResultViewer::Fft | super::ResultViewer::HarmonicBalance => "viewer-spectrum",
        super::ResultViewer::PhaseNoise => "viewer-phase-noise",
        super::ResultViewer::Contribution => "viewer-contribution",
        super::ResultViewer::TransferFunction => "viewer-transfer-function",
        super::ResultViewer::Eye => "eye-viewer",
        super::ResultViewer::Hist => "viewer-histogram",
        super::ResultViewer::Op | super::ResultViewer::Specs | super::ResultViewer::Table => {
            "viewer-table"
        }
        super::ResultViewer::Smith => "viewer-smith",
        super::ResultViewer::PoleZero => "viewer-pz",
    })
}

#[cfg(test)]
mod tests;
