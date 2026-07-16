//! Persistent and transient state for the RSpice workbench.
//!
//! This module deliberately contains no egui code.  It is the single owner
//! for navigation, dock visibility, responsive drawers, and the selection of
//! the task surface inside each canonical workspace.

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

use super::{
    BrowserHistoryEffect, RouteTransition, RouteTransitionSource, SurfaceId, SurfaceNavigation,
    SurfaceRoute,
};

const NAVIGATION_SCHEMA_VERSION: u8 = 1;

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
            Self::Simulate => "Simulation Studio",
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
            Self::ResultDataset(_) => Workspace::Results,
            Self::Verification => Workspace::Verify,
            Self::Models => Workspace::Models,
            Self::NetlistSource => Workspace::Netlist,
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
                "Schematic, custom layout, AMS, variation, reliability, PDK, model and sign-off workflows."
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
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
    #[serde(alias = "Catalog")]
    Models,
    #[serde(alias = "Libraries")]
    Symbols,
    #[serde(alias = "Pdk")]
    Corners,
    #[serde(alias = "Behavioral")]
    Include,
    Qualification,
}

impl ModelsPage {
    pub const ALL: [Self; 5] = [
        Self::Models,
        Self::Symbols,
        Self::Corners,
        Self::Include,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Models => "Models",
            Self::Symbols => "Symbols & CDF",
            Self::Corners => "Corners & sections",
            Self::Include => "Include graph",
            Self::Qualification => "Metadata audit",
        }
    }

    // Transitional source aliases keep code outside the new shell compiling
    // while persisted sessions migrate through the serde aliases above. They
    // are intentionally excluded from `ALL`, command routing, and rendering.
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub const Catalog: Self = Self::Models;
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub const Libraries: Self = Self::Symbols;
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub const Pdk: Self = Self::Corners;
    #[doc(hidden)]
    #[allow(non_upper_case_globals)]
    pub const Behavioral: Self = Self::Include;
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

/// Read-only capability document section from the mockup's section picker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapabilityMatrixSection {
    #[default]
    Platforms,
    PlannedDesigns,
    Analyses,
    Workspaces,
}

impl CapabilityMatrixSection {
    pub const ALL: [Self; 4] = [
        Self::Platforms,
        Self::PlannedDesigns,
        Self::Analyses,
        Self::Workspaces,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Platforms => "Platforms",
            Self::PlannedDesigns => "Planned designs",
            Self::Analyses => "Analyses",
            Self::Workspaces => "Workspaces",
        }
    }
}

/// Active section of the mockup-specified interoperability inspection
/// document. This is local presentation state, never project configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InteroperabilitySection {
    #[default]
    FormatMatrix,
    RoundTripContract,
    Qualification,
}

impl InteroperabilitySection {
    pub const ALL: [Self; 3] = [
        Self::FormatMatrix,
        Self::RoundTripContract,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::FormatMatrix => "Format matrix",
            Self::RoundTripContract => "Round-trip contract",
            Self::Qualification => "Qualification",
        }
    }
}

/// Domain projection selected in the interoperability format matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InteroperabilityDomain {
    #[default]
    All,
    NetlistAndSimulation,
    IcDesignAndLayout,
    PcbAndManufacturing,
    MechanicalExchange,
    ResultsAndReports,
}

impl InteroperabilityDomain {
    pub const ALL: [Self; 6] = [
        Self::All,
        Self::NetlistAndSimulation,
        Self::IcDesignAndLayout,
        Self::PcbAndManufacturing,
        Self::MechanicalExchange,
        Self::ResultsAndReports,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All domains",
            Self::NetlistAndSimulation => "Netlist and simulation",
            Self::IcDesignAndLayout => "IC design and layout",
            Self::PcbAndManufacturing => "PCB and manufacturing",
            Self::MechanicalExchange => "Mechanical exchange",
            Self::ResultsAndReports => "Results and reports",
        }
    }
}

/// Versioned support-level projection selected in the interoperability matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum InteroperabilitySupportLevel {
    #[default]
    All,
    Native,
    Qualified,
    ConnectorDependent,
    Planned,
}

impl InteroperabilitySupportLevel {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::Native,
        Self::Qualified,
        Self::ConnectorDependent,
        Self::Planned,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All support levels",
            Self::Native => "Native",
            Self::Qualified => "Qualified",
            Self::ConnectorDependent => "Connector-dependent",
            Self::Planned => "Planned",
        }
    }
}

/// Transient subordinate-document projection shown by the capability manager.
///
/// Canonical workflow routes own browser/back-stack semantics; this enum owns
/// only the retained presentation needed to render that route and is discarded
/// when the manager task closes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilityMatrixDrilldown {
    PlannedWorkflow(String),
    Interoperability,
    TouchEditGuide,
    PlatformLifecycle,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CapabilityMatrixState {
    /// Presentation-only jump target. The governed field contract requires
    /// Platforms on every open and forbids persistence of this selection.
    #[serde(skip)]
    pub section: CapabilityMatrixSection,
    /// Scroll position belongs only to the currently open task document.
    #[serde(skip)]
    pub scroll_offset: f32,
    /// Last responsive document class observed while the matrix body was
    /// rendered. A class change re-anchors the selected section because a
    /// numeric scroll offset cannot survive desktop/compact reflow reliably.
    #[serde(skip)]
    pub last_document_compact: Option<bool>,
    /// Nested inspection document. It is never session or project state.
    #[serde(skip)]
    pub drilldown: Option<CapabilityMatrixDrilldown>,
    /// Independent nested-document scroll position so returning to the matrix
    /// restores the exact review path.
    #[serde(skip)]
    pub drilldown_scroll_offset: f32,
    /// Local tab and filter state for the read-only interoperability document.
    #[serde(skip)]
    pub interoperability_section: InteroperabilitySection,
    #[serde(skip)]
    pub interoperability_domain: InteroperabilityDomain,
    #[serde(skip)]
    pub interoperability_support_level: InteroperabilitySupportLevel,
}

impl Default for CapabilityMatrixState {
    fn default() -> Self {
        Self {
            section: CapabilityMatrixSection::Platforms,
            scroll_offset: 0.0,
            last_document_compact: None,
            drilldown: None,
            drilldown_scroll_offset: 0.0,
            interoperability_section: InteroperabilitySection::default(),
            interoperability_domain: InteroperabilityDomain::default(),
            interoperability_support_level: InteroperabilitySupportLevel::default(),
        }
    }
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
    #[serde(default)]
    pub engineering_profile: EngineeringProfile,
    /// Version marker used to distinguish sessions that predate canonical
    /// routes from sessions whose current route is intentionally Design.
    #[serde(default)]
    navigation_schema_version: u8,
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
    #[serde(skip)]
    pub project_launcher_filter: ProjectLauncherFilter,
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
    #[serde(default)]
    pub verification_page: VerificationPage,
    #[serde(default)]
    pub models_page: ModelsPage,
    /// Analysis row whose configuration is shown in the simulation plan.
    /// Retained only to migrate pre-instance session selection.
    #[serde(default = "default_analysis_index")]
    pub active_analysis: usize,
    /// Stable analysis instance whose configuration is shown in the plan.
    #[serde(default)]
    pub active_analysis_instance: Option<crate::product::AnalysisInstanceId>,
    /// Last analysis lifecycle outcome announced by the transaction owner.
    #[serde(skip)]
    pub analysis_lifecycle_status: String,
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
    crate::common::simulation_analysis_tabs::TAB_TRANSIENT
}

impl Default for WorkbenchState {
    fn default() -> Self {
        Self {
            workspace: Workspace::Design,
            engineering_profile: EngineeringProfile::AnalogIc,
            navigation_schema_version: NAVIGATION_SCHEMA_VERSION,
            navigation: SurfaceNavigation::default(),
            route_diagnostic: None,
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
            project_launcher_filter: ProjectLauncherFilter::All,
            project_launcher_page: ProjectLauncherPage::Projects,
            project_launcher_recovery: super::recovery::RecoveryCatalog::default(),
            safe_mode: LocalSafeModeState::default(),
            project_close_destination: ProjectCloseDestination::Launcher,
            focus_project_launcher_search: false,
            navigator_width: default_navigator_width(),
            navigator_width_custom: false,
            inspector_width: default_inspector_width(),
            inspector_width_custom: false,
            console_height: default_console_height(),
            console_page: ConsolePage::Console,
            design_panel: DesignPanel::Navigator,
            documents: WorkspaceDocumentRegistry::default(),
            project_page: ProjectPage::Dashboard,
            verification_page: VerificationPage::Cockpit,
            models_page: ModelsPage::Models,
            active_analysis: default_analysis_index(),
            active_analysis_instance: None,
            analysis_lifecycle_status: "No lifecycle command has been committed this session."
                .to_owned(),
            selected_spec: None,
            selected_model: None,
            analysis_query: String::new(),
            navigator_query: String::new(),
            command_query: String::new(),
            project_name_draft: String::new(),
            project_name_error: None,
            drawer: None,
            focus_navigator_search: false,
            placement_query: String::new(),
            focus_placement_search: false,
            preflight: PreflightDialogState::default(),
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
            || matches!(
                self.current_route().surface_id(),
                SurfaceId::Preferences | SurfaceId::FeatureAvailability
            )
    }

    pub fn open_project_launcher(&mut self) {
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
        super::availability::require_available(route)?;
        let transition = self.navigation.navigate(route, source);
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        if let Some(workspace) = route.surface_id().workspace() {
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
        super::availability::require_available(route)?;
        let previous = self.navigation.current();
        self.navigation.replace(route, source);
        self.reconcile_capability_matrix_route(previous, route);
        if let Some(workspace) = route.surface_id().workspace() {
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
            if let Err(error) = super::availability::require_available(current) {
                self.route_diagnostic = Some(format!(
                    "The restored route was not opened because its executor is unavailable: {error}"
                ));
                self.navigation.replace(
                    SurfaceRoute::surface(SurfaceId::from_workspace(self.workspace)),
                    RouteTransitionSource::Restore,
                );
            } else if let Some(workspace) = current.surface_id().workspace() {
                self.workspace = workspace;
            }
        }
        let removed = self
            .navigation
            .retain_history(|route| super::availability::route_availability(route).can_open());
        if removed && self.route_diagnostic.is_none() {
            self.route_diagnostic = Some(
                "Unavailable routes were removed from restored task history; project and document state were preserved."
                    .to_owned(),
            );
        }
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

    #[must_use]
    pub fn forward_route_count(&self) -> usize {
        self.navigation.forward_entries().len()
    }

    pub fn navigate_back(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        let transition = self.navigation.go_back(source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().workspace() {
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
        let transition = self.navigation.go_back_steps(count, source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().workspace() {
            self.workspace = workspace;
        }
        self.close_drawer();
        Some(transition)
    }

    pub fn navigate_forward(&mut self, source: RouteTransitionSource) -> Option<RouteTransition> {
        self.navigate_forward_steps(1, source)
    }

    /// Atomically advance across up to `count` known routes, reconciling the
    /// foreground surface only once from the original route to the final
    /// destination.
    pub fn navigate_forward_steps(
        &mut self,
        count: usize,
        source: RouteTransitionSource,
    ) -> Option<RouteTransition> {
        let transition = self.navigation.go_forward_steps(count, source)?;
        self.reconcile_capability_matrix_route(transition.previous, transition.current);
        if let Some(workspace) = transition.current.surface_id().workspace() {
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
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::CapabilityWorkflowId;

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
    fn workspace_activation_updates_canonical_route_and_recent_history() {
        let mut state = WorkbenchState::default();
        state.activate(Workspace::Results);
        state.activate(Workspace::Design);

        assert_eq!(state.workspace, Workspace::Design);
        assert_eq!(
            state.navigation.recent_entries(),
            &[
                SurfaceRoute::surface(SurfaceId::Results),
                SurfaceRoute::surface(SurfaceId::Design),
            ]
        );
    }

    #[test]
    fn multi_step_return_reconciles_manager_and_workspace_once_at_final_route() {
        let mut state = WorkbenchState::default();
        state
            .navigate(
                SurfaceRoute::surface(SurfaceId::Results),
                RouteTransitionSource::User,
            )
            .expect("Results is available");
        state
            .navigate(
                SurfaceRoute::surface(SurfaceId::FeatureAvailability),
                RouteTransitionSource::User,
            )
            .expect("capability matrix is available");
        state
            .navigate(
                SurfaceRoute::capability_workflow(CapabilityWorkflowId::SourceLoadPullAnalysis),
                RouteTransitionSource::User,
            )
            .expect("planned workflow inspection is available");
        state.capability_matrix.section = CapabilityMatrixSection::PlannedDesigns;
        state.capability_matrix.scroll_offset = 240.0;
        state.clear_browser_history_effects();

        let transition = state
            .navigate_back_steps(2, RouteTransitionSource::User)
            .expect("matrix and source routes exist");

        assert_eq!(
            transition.previous,
            SurfaceRoute::capability_workflow(CapabilityWorkflowId::SourceLoadPullAnalysis)
        );
        assert_eq!(
            transition.current,
            SurfaceRoute::surface(SurfaceId::Results)
        );
        assert_eq!(state.workspace, Workspace::Results);
        assert_eq!(
            state.capability_matrix.section,
            CapabilityMatrixSection::Platforms
        );
        assert_eq!(state.capability_matrix.scroll_offset, 0.0);
        assert_eq!(
            state.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Traverse {
                delta: -2,
                destination: SurfaceRoute::surface(SurfaceId::Results),
            })
        );
    }

    #[test]
    fn touch_guide_history_roundtrips_through_its_platform_lifecycle_source() {
        let mut state = WorkbenchState::default();
        let matrix = SurfaceRoute::surface(SurfaceId::FeatureAvailability);
        let lifecycle = SurfaceRoute::capability_workflow(CapabilityWorkflowId::PlatformLifecycle);
        let touch_guide = SurfaceRoute::capability_workflow(CapabilityWorkflowId::TouchEditGuide);

        for route in [matrix, lifecycle, touch_guide] {
            state
                .navigate(route, RouteTransitionSource::User)
                .expect("the informational route has an explicit executor");
        }

        assert_eq!(state.current_route(), touch_guide);
        assert_eq!(state.previous_route(), Some(lifecycle));
        assert_eq!(
            state
                .navigate_back(RouteTransitionSource::User)
                .expect("touch guide retains its lifecycle source")
                .current,
            lifecycle
        );
        assert_eq!(
            state
                .navigate_back(RouteTransitionSource::User)
                .expect("lifecycle retains the capability matrix source")
                .current,
            matrix
        );
        assert_eq!(
            state
                .navigate_forward(RouteTransitionSource::User)
                .expect("forward restores lifecycle")
                .current,
            lifecycle
        );
        assert_eq!(
            state
                .navigate_forward(RouteTransitionSource::User)
                .expect("forward restores touch guidance")
                .current,
            touch_guide
        );
    }

    #[test]
    fn legacy_workspace_session_migrates_to_the_matching_primary_route() {
        let mut state: WorkbenchState =
            serde_json::from_str(r#"{"workspace":"Results"}"#).expect("legacy workbench restores");
        assert_eq!(state.current_route().surface_id(), SurfaceId::Design);

        state.reconcile_restored_navigation();
        assert_eq!(state.workspace, Workspace::Results);
        assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
        assert_eq!(
            state.take_browser_history_effect(),
            Some(BrowserHistoryEffect::Replace(SurfaceRoute::surface(
                SurfaceId::Results
            )))
        );
    }

    #[test]
    fn current_canonical_route_is_the_primary_workspace_source_of_truth() {
        let mut state = WorkbenchState::default();
        state.workspace = Workspace::Project;
        state.navigation.replace(
            SurfaceRoute::surface(SurfaceId::Results),
            RouteTransitionSource::BrowserPop,
        );

        state.reconcile_restored_navigation();
        assert_eq!(state.workspace, Workspace::Results);
        assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
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
        state.preflight.open = false;
        state
            .navigate(
                SurfaceRoute::surface(SurfaceId::Preferences),
                RouteTransitionSource::User,
            )
            .expect("Preferences has a registered executor");
        assert!(state.application_modal_open());
        state
            .navigate(
                SurfaceRoute::surface(SurfaceId::FeatureAvailability),
                RouteTransitionSource::User,
            )
            .expect("capability manager has a registered executor");
        assert!(state.application_modal_open());
    }

    #[test]
    fn capability_matrix_persists_profile_but_not_local_presentation() {
        let mut state = WorkbenchState::default();
        state.engineering_profile = EngineeringProfile::RfMicrowave;
        state.capability_matrix = CapabilityMatrixState {
            section: CapabilityMatrixSection::Workspaces,
            scroll_offset: 318.5,
            last_document_compact: Some(false),
            drilldown: Some(CapabilityMatrixDrilldown::PlannedWorkflow(
                "transient-noise".to_owned(),
            )),
            drilldown_scroll_offset: 91.0,
            interoperability_section: InteroperabilitySection::Qualification,
            interoperability_domain: InteroperabilityDomain::MechanicalExchange,
            interoperability_support_level: InteroperabilitySupportLevel::Planned,
        };

        let json = serde_json::to_string(&state).expect("workbench serializes");
        let restored: WorkbenchState =
            serde_json::from_str(&json).expect("workbench review context restores");
        assert_eq!(
            restored.engineering_profile,
            EngineeringProfile::RfMicrowave
        );
        assert_eq!(
            restored.capability_matrix.section,
            CapabilityMatrixSection::Platforms
        );
        assert_eq!(restored.capability_matrix.scroll_offset, 0.0);
        assert_eq!(restored.capability_matrix.last_document_compact, None);
        assert_eq!(restored.capability_matrix.drilldown, None);
        assert_eq!(restored.capability_matrix.drilldown_scroll_offset, 0.0);
        assert_eq!(
            restored.capability_matrix.interoperability_section,
            InteroperabilitySection::FormatMatrix
        );
        assert_eq!(
            restored.capability_matrix.interoperability_domain,
            InteroperabilityDomain::All
        );
        assert_eq!(
            restored.capability_matrix.interoperability_support_level,
            InteroperabilitySupportLevel::All
        );
    }

    #[test]
    fn capability_matrix_reopens_at_platforms_and_dismisses_drilldowns() {
        let mut state = WorkbenchState::default();
        state.capability_matrix.section = CapabilityMatrixSection::Analyses;
        state.capability_matrix.scroll_offset = 200.0;
        state.capability_matrix.last_document_compact = Some(false);
        state.capability_matrix.drilldown = Some(CapabilityMatrixDrilldown::PlannedWorkflow(
            "transient-noise".to_owned(),
        ));
        state.capability_matrix.interoperability_section = InteroperabilitySection::Qualification;
        state.capability_matrix.interoperability_domain =
            InteroperabilityDomain::MechanicalExchange;
        state.capability_matrix.interoperability_support_level =
            InteroperabilitySupportLevel::ConnectorDependent;

        state
            .navigate(
                SurfaceRoute::surface(SurfaceId::FeatureAvailability),
                RouteTransitionSource::User,
            )
            .expect("capability manager has an executor");
        assert_eq!(
            state.capability_matrix.section,
            CapabilityMatrixSection::Platforms
        );
        assert_eq!(state.capability_matrix.scroll_offset, 0.0);
        assert_eq!(state.capability_matrix.last_document_compact, None);
        assert_eq!(state.capability_matrix.drilldown, None);
        assert_eq!(
            state.capability_matrix.interoperability_section,
            InteroperabilitySection::FormatMatrix
        );
        assert_eq!(
            state.capability_matrix.interoperability_domain,
            InteroperabilityDomain::All
        );
        assert_eq!(
            state.capability_matrix.interoperability_support_level,
            InteroperabilitySupportLevel::All
        );

        state.capability_matrix.section = CapabilityMatrixSection::Workspaces;
        state.capability_matrix.scroll_offset = 42.0;
        state.capability_matrix.drilldown = Some(CapabilityMatrixDrilldown::PlannedWorkflow(
            "transient-noise".to_owned(),
        ));
        state.capability_matrix.interoperability_section =
            InteroperabilitySection::RoundTripContract;
        state.capability_matrix.interoperability_domain = InteroperabilityDomain::IcDesignAndLayout;
        state.capability_matrix.interoperability_support_level =
            InteroperabilitySupportLevel::Qualified;
        state.activate(Workspace::Design);
        assert_eq!(
            state.capability_matrix.section,
            CapabilityMatrixSection::Platforms
        );
        assert_eq!(state.capability_matrix.scroll_offset, 0.0);
        assert_eq!(state.capability_matrix.drilldown, None);
        assert_eq!(
            state.capability_matrix.interoperability_section,
            InteroperabilitySection::FormatMatrix
        );
        assert_eq!(
            state.capability_matrix.interoperability_domain,
            InteroperabilityDomain::All
        );
        assert_eq!(
            state.capability_matrix.interoperability_support_level,
            InteroperabilitySupportLevel::All
        );
    }

    #[test]
    fn fresh_browser_session_keeps_route_but_drops_restored_traversal() {
        let mut state = WorkbenchState::default();
        state.activate(Workspace::Results);
        assert!(state.previous_route().is_some());
        assert!(state.has_pending_browser_history_effects());

        state.reset_navigation_history_for_fresh_browser_session();

        assert_eq!(
            state.current_route(),
            SurfaceRoute::surface(SurfaceId::Results)
        );
        assert_eq!(state.previous_route(), None);
        assert_eq!(state.take_browser_history_effect(), None);
        assert!(state.navigate_back(RouteTransitionSource::User).is_none());
    }

    #[test]
    fn restored_route_reconciliation_is_idempotent_across_two_passes() {
        let mut state = WorkbenchState::default();
        state.navigation = serde_json::from_str(
            r#"{
                "current":"?view=design",
                "back":["?surface=rf-workbench","?surface=not-a-surface"],
                "forward":[],
                "recent":[]
            }"#,
        )
        .expect("navigation wire recovers malformed entries");

        state.reconcile_restored_navigation();
        let first = state
            .take_route_diagnostic()
            .expect("first pass reports malformed recovery");
        assert!(first.contains("Malformed routes"));
        assert!(state.navigation.back_entries().is_empty());
        assert!(!state.navigation.recovered_invalid_routes());

        state.reconcile_restored_navigation();
        assert_eq!(state.take_route_diagnostic(), None);
        assert!(!state.navigation.recovered_invalid_routes());
    }

    #[test]
    fn unavailable_history_removal_is_reported_once_without_malformed_flag() {
        let mut state = WorkbenchState::default();
        state.navigation = serde_json::from_str(
            r#"{
                "current":"?view=design",
                "back":["?surface=rf-workbench"],
                "forward":[],
                "recent":[]
            }"#,
        )
        .expect("canonical unavailable history restores");

        state.reconcile_restored_navigation();
        let first = state
            .take_route_diagnostic()
            .expect("first pass reports unavailable removal");
        assert!(first.contains("Unavailable routes"));
        assert!(!state.navigation.recovered_invalid_routes());

        state.reconcile_restored_navigation();
        assert_eq!(state.take_route_diagnostic(), None);
    }

    #[test]
    fn browser_effect_overflow_reaches_the_workbench_recovery_gate() {
        let mut state = WorkbenchState::default();
        for index in 0..65 {
            state.activate(if index % 2 == 0 {
                Workspace::Results
            } else {
                Workspace::Models
            });
        }

        assert!(state.has_pending_browser_history_effects());
        assert_eq!(state.take_browser_history_effect(), None);
        assert!(state.take_browser_history_effect_queue_overflowed());
        assert!(!state.take_browser_history_effect_queue_overflowed());
        assert!(!state.has_pending_browser_history_effects());
        assert_eq!(state.current_route().surface_id(), SurfaceId::Results);
    }
}
