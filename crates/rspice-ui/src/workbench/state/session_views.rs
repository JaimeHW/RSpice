//! Session-scoped state for verification, preflight, and the capability views.
//!
//! Everything here is retained only for the active session and never treated
//! as evidence: a regression receipt records what a review saw, but the source
//! runs and datasets remain the authoritative owners, and a preflight report
//! is a snapshot of one check rather than a standing verdict. That is why none
//! of it is durable — reopening the project must re-derive it.

use super::*;

/// Immutable regression comparison receipt retained only for the active
/// verification review session. The source runs and datasets remain the
/// authoritative evidence owners.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RegressionComparisonReceipt {
    pub plan_id: crate::product::SimulationPlanId,
    pub plan_revision: crate::product::ObjectRevision,
    pub tolerance_digest: crate::product::ContentDigest,
    pub baseline_run: crate::product::RunId,
    pub candidate_run: crate::product::RunId,
    pub baseline_dataset: crate::product::DatasetId,
    pub candidate_dataset: crate::product::DatasetId,
    pub baseline_content_digest: crate::product::ContentDigest,
    pub candidate_content_digest: crate::product::ContentDigest,
    pub baseline_authority_digest: crate::product::ContentDigest,
    pub candidate_authority_digest: crate::product::ContentDigest,
    pub aligned_checks: usize,
    pub aligned_waveforms: usize,
    pub changed_checks: usize,
    #[serde(default)]
    pub passed_checks: usize,
    #[serde(default)]
    pub failed_checks: usize,
    #[serde(default)]
    pub passed_waveforms: usize,
    #[serde(default)]
    pub failed_waveforms: usize,
    #[serde(default)]
    pub unconfigured_targets: usize,
    #[serde(default)]
    pub unevaluated_targets: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct RegressionToleranceDraft {
    pub target: crate::state::RegressionTargetSelector,
    pub method: crate::state::RegressionComparisonMethod,
    pub absolute_tolerance: String,
    pub relative_tolerance_percent: String,
    pub time_skew_allowance: String,
    pub comparison_window: String,
    pub dirty: bool,
    pub validation_error: Option<String>,
}

/// Runtime-only candidate value owned by the non-destructive parameter tuner.
/// The persisted `DesignVariable` remains authoritative until the user commits
/// the complete candidate set as one simulation-plan revision.
#[derive(Debug, Clone, PartialEq)]
pub struct TuningVariableDraft {
    pub variable_id: crate::product::DesignVariableId,
    pub baseline_expression: String,
    pub candidate_expression: String,
    pub validation_error: Option<String>,
    /// A proposed variable does not exist in the authoritative plan yet.
    /// Its candidate expression is still initialized from the selected
    /// instance literal, so this bit participates in dirty detection.
    pub proposed: bool,
}

impl TuningVariableDraft {
    pub fn is_dirty(&self) -> bool {
        self.proposed || self.candidate_expression.trim() != self.baseline_expression.trim()
    }
}

/// Runtime-only bridge between a selected schematic instance and the tuning
/// sandbox.  The proposed variable and binding are deliberately kept outside
/// both authoritative stores until the review dialog commits them together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TuningInstanceBindingDraft {
    pub component_id: u64,
    pub component_name: String,
    pub source_view: crate::state::CellViewRef,
    pub source_topology_version: u64,
    pub source_value: String,
    pub binding_expression: String,
    pub variable: crate::state::DesignVariable,
    pub creates_variable: bool,
}

impl TuningInstanceBindingDraft {
    #[must_use]
    pub fn requires_schematic_edit(&self) -> bool {
        self.source_value.trim() != self.binding_expression
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerificationSessionState {
    #[serde(default)]
    pub regression_baseline_run: Option<crate::product::RunId>,
    /// When enabled, process-corner cells are expressed as exact deltas from
    /// the revision-matched TT/nominal/room point retained by the active
    /// corner plan. The flag is review state only; result datasets remain
    /// immutable.
    #[serde(default)]
    pub corner_compare_nominal: bool,
    #[serde(skip)]
    pub regression_comparison: Option<RegressionComparisonReceipt>,
    #[serde(skip)]
    pub regression_baseline_picker_open: bool,
    #[serde(skip)]
    pub regression_baseline_picker_selection: Option<crate::product::RunId>,
    #[serde(skip)]
    pub regression_selected_target: Option<crate::state::RegressionTargetSelector>,
    #[serde(skip)]
    pub regression_tolerance_drafts: Vec<RegressionToleranceDraft>,
    /// Stable plan/revision that owns `tuning_variables`. All three fields are
    /// transient so an interrupted sandbox can never restore as project data.
    #[serde(skip)]
    pub tuning_plan_id: Option<crate::product::SimulationPlanId>,
    #[serde(skip)]
    pub tuning_plan_revision: Option<crate::product::ObjectRevision>,
    #[serde(skip)]
    pub tuning_variables: Vec<TuningVariableDraft>,
    /// Optional instance Value-row proposal that opened this sandbox.
    #[serde(skip)]
    pub tuning_instance_binding: Option<TuningInstanceBindingDraft>,
    /// Selected variable card and one-shot editor focus request. Both are
    /// transient presentation state and never confer commit authority.
    #[serde(skip)]
    pub tuning_selected_variable: Option<crate::product::DesignVariableId>,
    #[serde(skip)]
    pub tuning_focus_variable: Option<crate::product::DesignVariableId>,
    /// Immutable retained run selected when the sandbox was opened. This is
    /// presentation state only; the run and dataset remain owned by
    /// `SimulationState` and are never copied into the tuner.
    #[serde(skip)]
    pub tuning_baseline_run: Option<crate::product::RunId>,
    /// Review gate opened by the mockup-specified tuning commit action. The
    /// dialog is transient and never restores across an application session.
    #[serde(skip)]
    pub tuning_review_open: bool,
    #[serde(skip, default = "default_verification_action_receipt")]
    pub action_receipt: String,
}

pub(super) fn default_verification_action_receipt() -> String {
    "No verification action has been committed this session.".to_owned()
}

impl Default for VerificationSessionState {
    fn default() -> Self {
        Self {
            regression_baseline_run: None,
            corner_compare_nominal: false,
            regression_comparison: None,
            regression_baseline_picker_open: false,
            regression_baseline_picker_selection: None,
            regression_selected_target: None,
            regression_tolerance_drafts: Vec::new(),
            tuning_plan_id: None,
            tuning_plan_revision: None,
            tuning_variables: Vec::new(),
            tuning_instance_binding: None,
            tuning_selected_variable: None,
            tuning_focus_variable: None,
            tuning_baseline_run: None,
            tuning_review_open: false,
            action_receipt: default_verification_action_receipt(),
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
    Bins,
    #[serde(alias = "Behavioral")]
    Include,
    Qualification,
}

impl ModelsPage {
    pub const ALL: [Self; 6] = [
        Self::Models,
        Self::Symbols,
        Self::Corners,
        Self::Bins,
        Self::Include,
        Self::Qualification,
    ];

    pub const fn label(self) -> &'static str {
        match self {
            Self::Models => "Models",
            Self::Symbols => "Symbols & CDF",
            Self::Corners => "Corners & sections",
            Self::Bins => "Bins & geometry",
            Self::Include => "Include graph",
            Self::Qualification => "Qualification",
        }
    }
}

/// Corpus projection selected in the Models & PDKs catalog. This is
/// presentation state only: attaching a source or adding a project model is a
/// separate guarded engineering transaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelsCatalogScope {
    #[default]
    Project,
    InstalledPacks,
    RSpiceLibrary,
}

impl ModelsCatalogScope {
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Project => "Project",
            Self::InstalledPacks => "Installed packs",
            Self::RSpiceLibrary => "RSpice library",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProjectModelFacet {
    #[default]
    All,
    Bound,
    Pinned,
    Review,
    /// Models compiled into the engine. Named `Protected` until 2026-08, which
    /// read as a sealed-IP guarantee the product does not make; the alias is
    /// what lets a session saved under the old spelling still open.
    #[serde(alias = "protected")]
    BuiltIn,
}

impl ProjectModelFacet {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::Bound,
        Self::Pinned,
        Self::Review,
        Self::BuiltIn,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Bound => "Bound",
            Self::Pinned => "Pinned",
            Self::Review => "Review",
            Self::BuiltIn => "Built-in",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelPackFacet {
    #[default]
    All,
    NeedsAttention,
    Attached,
    Foundry,
    Vendor,
    Community,
    Redistributable,
}

impl ModelPackFacet {
    pub const ALL: [Self; 7] = [
        Self::All,
        Self::NeedsAttention,
        Self::Attached,
        Self::Foundry,
        Self::Vendor,
        Self::Community,
        Self::Redistributable,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::NeedsAttention => "Needs attention",
            Self::Attached => "Attached",
            Self::Foundry => "Foundry",
            Self::Vendor => "Vendor",
            Self::Community => "Community",
            Self::Redistributable => "Redistributable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum RSpicePartFacet {
    #[default]
    All,
    Mosfet,
    Bipolar,
    Diode,
    JfetAndHemt,
    Passive,
    IcAndMacro,
}

impl RSpicePartFacet {
    pub const ALL: [Self; 7] = [
        Self::All,
        Self::Mosfet,
        Self::Bipolar,
        Self::Diode,
        Self::JfetAndHemt,
        Self::Passive,
        Self::IcAndMacro,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All classes",
            Self::Mosfet => "MOSFET",
            Self::Bipolar => "Bipolar",
            Self::Diode => "Diode",
            Self::JfetAndHemt => "JFET & HEMT",
            Self::Passive => "Passive",
            Self::IcAndMacro => "IC & macro",
        }
    }

    #[must_use]
    pub const fn device_filters(self) -> &'static [&'static str] {
        match self {
            Self::All => &[],
            Self::Mosfet => &[
                "mosfet-n",
                "mosfet-p",
                "mosfet-vdmos",
                "fdsoin",
                "fdsoip",
                "nsoi",
                "psoi",
                "psp103_va",
                "psp103va",
                "pspnqs103va",
                "mosvar",
            ],
            Self::Bipolar => &["bjt-npn", "bjt-pnp"],
            Self::Diode => &["diode", "sidiode"],
            Self::JfetAndHemt => &["jfet-n", "jfet-p", "gasfet", "mesfet-n"],
            Self::Passive => &[
                "resistor",
                "capacitor",
                "inductor",
                "magnetic-core",
                "lcouple",
                "transmission-line",
            ],
            Self::IcAndMacro => &["subckt"],
        }
    }
}

/// User-visible lifecycle state for the Models & PDKs manager.
///
/// Engineering data remains authoritative in the model-library manager. This
/// runtime-only state describes the last attempted workflow so diagnostics do
/// not collapse distinct recovery paths into a generic error toast.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ModelsOperationalState {
    #[default]
    Ready,
    InvalidInput,
    ExecutionError,
    ReadOnly,
    Offline,
    Conflict,
    Stale,
    Permission,
    Entitlement,
    Cancelled,
    Rollback,
    Partial,
    Corrupted,
    Recovered,
}

impl ModelsOperationalState {
    /// The registry the coverage tests walk. `label` and `consequence` are
    /// live — the packs page paints both — but nothing in a shipped frame
    /// iterates every variant at once, so the array itself stays with the
    /// tests that do.
    #[cfg(test)]
    pub const ALL: [Self; 14] = [
        Self::Ready,
        Self::InvalidInput,
        Self::ExecutionError,
        Self::ReadOnly,
        Self::Offline,
        Self::Conflict,
        Self::Stale,
        Self::Permission,
        Self::Entitlement,
        Self::Cancelled,
        Self::Rollback,
        Self::Partial,
        Self::Corrupted,
        Self::Recovered,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::InvalidInput => "Invalid input",
            Self::ExecutionError => "Execution error",
            Self::ReadOnly => "Read-only",
            Self::Offline => "Offline",
            Self::Conflict => "Conflict",
            Self::Stale => "Stale",
            Self::Permission => "Permission",
            Self::Entitlement => "Entitlement",
            Self::Cancelled => "Cancelled",
            Self::Rollback => "Rollback",
            Self::Partial => "Partial",
            Self::Corrupted => "Corrupted",
            Self::Recovered => "Recovered",
        }
    }

    /// What the failure left behind, and what has to happen next.
    ///
    /// This is the half that makes the vocabulary worth having. "Offline" and
    /// "Conflict" are the same colour and the same length; they differ in
    /// whether anything changed on this machine and in what the reader does
    /// about it, and a workspace that prints only the word has told them
    /// nothing they could not have guessed from the error text.
    #[must_use]
    pub const fn consequence(self) -> &'static str {
        match self {
            Self::Ready => "Nothing needs a decision.",
            Self::InvalidInput => {
                "Nothing was published. Correct the value the operation named and run it again."
            }
            Self::ExecutionError => {
                "Nothing was published. The step that failed names what it was reading."
            }
            Self::ReadOnly => {
                "Nothing was written. Open the project for editing before running this again."
            }
            Self::Offline => {
                "Nothing changed on this machine. Installed packs, project pins and recorded \
                 results are unaffected."
            }
            Self::Conflict => {
                "Nothing was published. Resolve the contested identity before running this again."
            }
            Self::Stale => {
                "The candidate was discarded without mutation because the project or catalog \
                 moved under it. Run it again against the current project."
            }
            Self::Permission => {
                "Nothing was written. The operating system refused RSpice this location."
            }
            Self::Entitlement => {
                "Nothing was installed. This release's licence does not grant what the operation \
                 needed."
            }
            Self::Cancelled => {
                "Nothing was published. The operation stopped where it was asked to."
            }
            Self::Rollback => {
                "Every step that ran was undone; this machine holds exactly what it held before."
            }
            Self::Partial => {
                "Part of the operation landed. What did not is listed in the console, and the \
                 rest is unchanged."
            }
            Self::Corrupted => {
                "The bytes read do not describe what they claim to, so nothing from them entered \
                 the catalog."
            }
            Self::Recovered => "The operation repaired what it found and completed.",
        }
    }

    #[must_use]
    pub fn from_failure(message: &str) -> Self {
        let normalized = message.to_ascii_lowercase();
        if normalized.contains("invalid")
            || normalized.contains("must ")
            || normalized.contains("required")
        {
            Self::InvalidInput
        } else if normalized.contains("read-only") || normalized.contains("not open") {
            Self::ReadOnly
        } else if normalized.contains("offline")
            || normalized.contains("unavailable")
            // The Model Hub says an unreachable service in exactly these
            // words. Without this the plainest possible statement of "the
            // network is down" classified as a generic execution error, which
            // is the one recovery path it definitely is not.
            || normalized.contains("could not be reached")
        {
            Self::Offline
        } else if normalized.contains("conflict")
            || normalized.contains("collision")
            || normalized.contains("duplicate")
        {
            Self::Conflict
        } else if normalized.contains("stale")
            || normalized.contains("drift")
            || normalized.contains("changed on disk")
        {
            Self::Stale
        } else if normalized.contains("permission")
            || normalized.contains("access denied")
            // Windows spells the same refusal "Access is denied. (os error 5)",
            // which classified as a generic execution error and told the reader
            // to read a diagnostic instead of to fix a directory ACL.
            || normalized.contains("access is denied")
        {
            Self::Permission
        } else if normalized.contains("license")
            || normalized.contains("entitlement")
            || normalized.contains("restricted")
        {
            Self::Entitlement
        } else if normalized.contains("cancel") {
            Self::Cancelled
        } else if normalized.contains("roll back")
            || normalized.contains("rolled back")
            || normalized.contains("rollback")
            || normalized.contains("left unchanged")
        {
            Self::Rollback
        } else if normalized.contains("partial") || normalized.contains("path errors") {
            Self::Partial
        } else if normalized.contains("corrupt") || normalized.contains("malformed") {
            Self::Corrupted
        } else if normalized.contains("recover") {
            Self::Recovered
        } else {
            Self::ExecutionError
        }
    }
}

/// What the last model-source or Model Hub operation was, beside its receipt.
///
/// A receipt says what happened; without this it does not say what was being
/// attempted, and "the pack format refused" is a different sentence depending
/// on whether the workspace was refreshing a catalog or installing a release.
/// It is captured where the operation starts, which is the only place that
/// knows, and it outlives the operation so a failure can still name itself.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelsAttemptedOperation {
    /// The operation in the workspace's own words, e.g. `model-catalog
    /// refresh` or `model-pack install of 'rspice-diodes 1.2.0'`.
    pub label: String,
    /// Whether the workspace can re-issue it from this record alone. Only the
    /// catalog refresh takes no argument a user would have to choose again; an
    /// install is retried from the release row that named the version, so
    /// offering a bare "retry" for one would be a button that guesses.
    pub reissuable: bool,
}

/// What re-proving one installed release concluded, this session.
///
/// Deliberately not durable. A re-proof is a statement about bytes at one
/// instant; restoring "verified" from a saved session would be a claim about
/// a machine that has been running for a week since.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PackReProof {
    /// The archive verified end to end under the release key and still
    /// describes the release its directory claims.
    Verified,
    /// It did not, in the words the pack format or the store used.
    Failed(String),
}

/// Models-manager selections and filters that do not own engineering data.
/// Stable strings are used only for presentation; every mutation re-resolves
/// the live library/pack/source identity and fails closed if it changed.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModelsWorkbenchViewState {
    #[serde(default)]
    pub catalog_scope: ModelsCatalogScope,
    #[serde(default)]
    pub project_facet: ProjectModelFacet,
    #[serde(default)]
    pub pack_facet: ModelPackFacet,
    #[serde(default)]
    pub part_facet: RSpicePartFacet,
    #[serde(default)]
    pub catalog_query: String,
    #[serde(default)]
    pub selected_pack: Option<String>,
    #[serde(default)]
    pub selected_part: Option<String>,
    /// Zero-based offset into the exact current part-catalog query.
    #[serde(default)]
    pub part_catalog_offset: usize,
    #[serde(default)]
    pub selected_symbol: Option<String>,
    #[serde(default)]
    pub selected_corner: Option<String>,
    #[serde(default)]
    pub selected_bin_family: Option<String>,
    #[serde(default)]
    pub include_selected_source: Option<String>,
    #[serde(default)]
    pub include_direct_only: bool,
    #[serde(default)]
    pub include_definition_query: String,
    /// Last real workflow result. It is deliberately not durable: restored
    /// sessions must derive health from retained sources, not trust a toast.
    #[serde(skip)]
    pub action_receipt: Option<Result<String, String>>,
    #[serde(skip)]
    pub operational_state: ModelsOperationalState,
    /// What that receipt is about. Set where an operation starts.
    #[serde(skip)]
    pub attempted_operation: Option<ModelsAttemptedOperation>,
    /// Re-proof outcomes keyed `<pack id>@<version>`. An absent entry means
    /// nothing has re-proved that release since this session started, which is
    /// a state the pack table names rather than one it hides.
    #[serde(skip)]
    pub pack_verification: std::collections::BTreeMap<String, PackReProof>,
    /// One authenticated source import may parse at a time. Parsing is owned
    /// by a native background thread or a dedicated browser worker; these
    /// fields are presentation only and never restore as engineering state.
    #[serde(skip)]
    pub model_import_in_progress: bool,
    #[serde(skip)]
    pub model_import_label: Option<String>,
    /// Completed fraction of the operation in progress, when its length is
    /// known in advance. Only a pack download has one, and it comes from the
    /// signed catalog rather than from the service that serves the bytes.
    #[serde(skip)]
    pub model_import_progress: Option<f32>,
    #[serde(skip)]
    pub dialog: Option<ModelsWorkbenchDialog>,
    /// Which distributed releases the pack table lists.
    #[serde(default)]
    pub hub_facet: ModelHubFacet,
    /// Whether this session already asked to refresh a stale catalog.
    ///
    /// Opening the workspace refreshes a catalog older than a week, once. The
    /// latch is what makes it once rather than every frame the workspace is
    /// visible, which is the shape a per-frame condition would otherwise have.
    #[serde(skip)]
    pub catalog_refresh_requested: bool,
}

impl Default for ModelsWorkbenchViewState {
    fn default() -> Self {
        Self {
            catalog_scope: ModelsCatalogScope::default(),
            project_facet: ProjectModelFacet::default(),
            pack_facet: ModelPackFacet::default(),
            part_facet: RSpicePartFacet::default(),
            catalog_query: String::new(),
            selected_pack: None,
            selected_part: None,
            part_catalog_offset: 0,
            selected_symbol: None,
            selected_corner: None,
            selected_bin_family: None,
            include_selected_source: None,
            include_direct_only: false,
            include_definition_query: String::new(),
            action_receipt: None,
            operational_state: ModelsOperationalState::Ready,
            attempted_operation: None,
            pack_verification: std::collections::BTreeMap::new(),
            model_import_in_progress: false,
            model_import_label: None,
            model_import_progress: None,
            dialog: None,
            hub_facet: ModelHubFacet::default(),
            catalog_refresh_requested: false,
        }
    }
}

/// Which distributed packs the Model Hub table lists.
///
/// The table always spans installed *and* available releases, because "what
/// this machine has" and "what the catalog offers" are the same question asked
/// once. The facet narrows that one list; it never switches between two.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelHubFacet {
    #[default]
    All,
    Installed,
    Available,
    Updatable,
    Incompatible,
}

impl ModelHubFacet {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::Installed,
        Self::Available,
        Self::Updatable,
        Self::Incompatible,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Installed => "Installed",
            Self::Available => "Available",
            Self::Updatable => "Update",
            Self::Incompatible => "Incompatible",
        }
    }
}

/// Everything a distributed-release confirmation states before it acts.
///
/// It is a captured projection rather than a live query: the dialog must show
/// the same release it will install, and re-reading the catalog while the
/// dialog is open could change the answer between the sentence and the click.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackReleaseConfirmation {
    pub name: String,
    pub version: String,
    pub spdx: String,
    pub archive_length: u64,
    pub parts: usize,
    pub capabilities: Vec<String>,
    /// Capabilities this engine build does not offer. Non-empty disables the
    /// action and is the reason shown for it.
    pub missing: Vec<String>,
    /// The one part this confirmation came to add, when it came from the
    /// component shelf rather than from the pack table.
    pub part: Option<String>,
    /// The installed version this replaces, when the action is an update.
    pub replaces: Option<String>,
}

impl PackReleaseConfirmation {
    /// Describes one published release from whatever the session already
    /// proved about it.
    ///
    /// The signed catalog is preferred because it is the only source that
    /// knows the download size; an installed release whose catalog entry has
    /// since been withdrawn is still described, from its own signed manifest,
    /// so a part already on this machine never becomes undescribable.
    #[must_use]
    pub fn for_release(
        service: &crate::services::model_hub::ModelHubService,
        pack_id: &str,
        version: &str,
        part: Option<String>,
    ) -> Option<Self> {
        let hub = service.hub()?;
        let listed = hub.snapshot().and_then(|snapshot| {
            snapshot
                .packs
                .iter()
                .find(|pack| pack.id == pack_id)
                .and_then(|pack| {
                    pack.releases
                        .iter()
                        .find(|release| release.version == version)
                        .map(|release| (pack, release))
                })
        });
        let missing =
            |capabilities: &[String]| crate::state::model_hub::missing_capabilities(capabilities);
        if let Some((pack, release)) = listed {
            return Some(Self {
                name: pack.name.clone(),
                version: release.version.clone(),
                spdx: release.spdx.clone(),
                archive_length: release.archive_length,
                parts: release.parts.len(),
                capabilities: release.capabilities.clone(),
                missing: missing(&release.capabilities),
                part,
                replaces: None,
            });
        }
        let installed = hub
            .installed()
            .iter()
            .find(|pack| pack.pack_id() == pack_id && pack.version() == version)?;
        Some(Self {
            name: installed.manifest.pack.name.clone(),
            version: installed.version().to_owned(),
            spdx: installed.manifest.license.spdx.clone(),
            archive_length: 0,
            parts: installed.manifest.parts.len(),
            capabilities: installed.manifest.requires.capabilities.clone(),
            missing: missing(&installed.manifest.requires.capabilities),
            part,
            replaces: None,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModelsWorkbenchDialog {
    #[cfg(target_arch = "wasm32")]
    SelectBrowserImportRoot {
        candidates: Vec<String>,
        selected: usize,
    },
    SourcePreview {
        title: String,
        subtitle: String,
        source: String,
        /// Whether this source can be authored, which offers a route into
        /// Model Editor. The preview itself is never a writable buffer — it
        /// holds a copy of the retained bytes with nowhere to write back to.
        editable: bool,
    },
    CompareModels {
        left_library: String,
        left_model: String,
        /// The chosen counterpart as `(library, model)`. `None` means the
        /// dialog is still asking: a comparison against an arbitrary
        /// definition is not a comparison, so there is no default beyond the
        /// same card in another library.
        right: Option<(String, String)>,
    },
    ConfirmPack {
        pack_id: String,
        attach: bool,
        /// The distributed release this confirmation is about, when it is one.
        ///
        /// A shipped-corpus attach and a signed-release install are the same
        /// user decision — "commit this pack to this project" — asked about
        /// two different objects. One dialog asks it, and this is what makes
        /// the release case able to state its version, licence, size, and
        /// capability verdict before the user says yes.
        release: Option<Box<PackReleaseConfirmation>>,
    },
    ConfirmPart {
        pack_id: String,
        part_name: String,
    },
    AuthorTechnologySymbolVariant {
        package_id: String,
        source_cell: String,
        target_library: String,
        target_cell: String,
    },
    DefinitionConflict {
        definition: String,
        scope: crate::state::model_library::ModelConsumerScope,
        providers: Vec<String>,
        selected_provider: String,
        reason: String,
    },
    BindingTrace {
        model: String,
        consumers: Vec<String>,
    },
    AddCorner {
        library: String,
        name: String,
        temperature_c: String,
        supply_factor: String,
    },
    EditCorner {
        library: String,
        original_name: String,
        duplicate: bool,
        name: String,
        description: String,
        nmos_corner: String,
        pmos_corner: String,
        temperature_c: String,
        supply_factor: String,
        minimum_temperature_c: String,
        maximum_temperature_c: String,
        required_domains: Vec<crate::state::model_library::CornerSectionDomain>,
        make_default: bool,
    },
    ConfirmDeleteCorner {
        library: String,
        corner: String,
    },
    BindCornerSection {
        library: String,
        corner: String,
        domain: crate::state::model_library::CornerSectionDomain,
        section: String,
    },
}

/// Destination that can resolve a blocking simulation-preflight finding.
/// The dialog stores semantic destinations instead of callbacks so a report
/// remains deterministic for the exact project revision that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightRemediation {
    DesignChecks,
    SimulationPlan,
    ProjectTechnology,
    /// One page of the Models workspace. A model-binding finding is repaired
    /// where the bindings live — Corners & sections — and not by re-running
    /// the source checks that reported it.
    Models(ModelsPage),
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
    /// Exact configured library/cell/view root whose topology was inspected.
    pub topology_root: String,
    pub topology_revision: u64,
    /// Canonical configured hierarchy closure, keyed by LCV identity. This
    /// catches edits to referenced child schematics without coupling evidence
    /// to whichever unrelated document happens to be active in the editor.
    pub topology_closure: Vec<(String, u64)>,
    /// Stable identity and exact revision of the active simulation plan that
    /// was inspected. `None` is retained only for a blocked report produced
    /// while a legacy or otherwise unavailable plan is being diagnosed.
    pub simulation_plan_id: Option<crate::product::SimulationPlanId>,
    pub simulation_plan_revision: Option<crate::product::ObjectRevision>,
    pub blockers: Vec<PreflightIssue>,
    pub advisories: Vec<String>,
    /// Present only when the controller retained a real authorized immutable
    /// execution snapshot. Blocked reports never fabricate contract fields.
    pub prepared: Option<PreparedPreflightContract>,
}

impl PreflightReport {
    /// Whether the immutable report contains a complete authorization. Call
    /// `is_runnable_for` at every live UI or dispatch boundary so currentness
    /// is evaluated as well.
    pub fn is_runnable(&self) -> bool {
        self.blockers.is_empty() && self.prepared.is_some()
    }

    pub fn is_current_for(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> bool {
        let plan_is_current = match (
            self.simulation_plan_id,
            self.simulation_plan_revision,
            simulation_plan,
        ) {
            (Some(report_id), Some(report_revision), Some((live_id, live_revision))) => {
                report_id == live_id && report_revision == live_revision
            }
            (None, None, None) => true,
            _ => false,
        };
        self.project_revision == project_revision
            && self.topology_root.eq_ignore_ascii_case(topology_root)
            && self.topology_revision == topology_revision
            && self.topology_closure == topology_closure
            && plan_is_current
    }

    pub fn is_runnable_for(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> bool {
        self.is_runnable()
            && self.is_current_for(
                project_revision,
                topology_root,
                topology_revision,
                topology_closure,
                simulation_plan,
            )
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
    pub saved_output_contract_count: usize,
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
    /// A caller asked for the governed preflight-and-queue pass. The frame
    /// loop consumes it immediately before the dialog is rendered, so the
    /// request records intent without its origin naming the workflow.
    pub(in crate::workbench) run_and_queue_requested: bool,
}

impl PreflightDialogState {
    /// Drop every presentation artifact backed by a no-longer-current
    /// execution contract. The controller's one-shot permit is invalidated by
    /// the owning `RSpiceApp` at the same mutation boundary.
    ///
    /// A pending run request deliberately survives: the workflow it asks for
    /// re-runs preflight and digest-compares the live contract before it
    /// queues anything, so dropping the request here would lose a deliberate
    /// operator action to a design edit that the workflow already refuses.
    pub fn invalidate(&mut self) {
        self.open = false;
        self.report = None;
        self.pending_toast = None;
    }

    /// Ask the frame loop to run the governed preflight-and-queue pass.
    pub fn request_run_and_queue(&mut self) {
        self.run_and_queue_requested = true;
    }

    /// Consume a pending request. Returns whether one was outstanding.
    pub fn take_run_and_queue_request(&mut self) -> bool {
        std::mem::take(&mut self.run_and_queue_requested)
    }
}

/// Local presentation state for the canonical Jobs manager. Selection uses
/// stable run identity so history insertion, pruning, and reload cannot
/// silently retarget the inspector or exported manifest.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum JobsPlanScope {
    ActivePlan,
    #[default]
    AllPlans,
    ManualDeck,
}

impl JobsPlanScope {
    pub const ALL: [Self; 3] = [Self::ActivePlan, Self::AllPlans, Self::ManualDeck];

    pub const fn label(self) -> &'static str {
        match self {
            Self::ActivePlan => "Active plan",
            Self::AllPlans => "All plans",
            Self::ManualDeck => "Manual decks",
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JobsManagerState {
    pub selected_run_id: Option<crate::product::RunId>,
    pub scroll_offset: f32,
    pub plan_scope: JobsPlanScope,
}

/// Report-composer selection and transactional editor presentation. The
/// canonical report documents live in `ProjectWorkspace`; only stable
/// selection identity is restored with the application session. Dialog drafts
/// never persist until their domain transaction commits.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportAuthoringState {
    #[serde(default)]
    pub selected_document: Option<crate::product::ResultDocumentId>,
    #[serde(default)]
    pub selected_page: Option<crate::results::report_document::ReportPageId>,
    #[serde(skip)]
    pub create_document_open: bool,
    #[serde(skip)]
    pub create_document_title: String,
    #[serde(skip)]
    pub create_document_template: usize,
    #[serde(skip)]
    pub preview_block_page: usize,
    #[serde(skip)]
    pub add_page_open: bool,
    #[serde(skip)]
    pub add_page_title: String,
    #[serde(skip)]
    pub page_properties_open: bool,
    #[serde(skip)]
    pub page_properties_page: Option<crate::results::report_document::ReportPageId>,
    #[serde(skip)]
    pub page_title_draft: String,
    #[serde(skip)]
    pub inline_page_settings_page: Option<crate::results::report_document::ReportPageId>,
    #[serde(skip)]
    pub inline_page_title_draft: String,
    #[serde(skip)]
    pub selected_report_block: Option<crate::results::report_document::ReportBlockId>,
    #[serde(skip)]
    pub add_report_element_open: bool,
    #[serde(skip)]
    pub add_report_element_kind: usize,
    #[serde(skip)]
    pub add_report_element_title: String,
    #[serde(skip)]
    pub add_report_element_primary: String,
    #[serde(skip)]
    pub add_report_element_secondary: String,
    #[serde(skip)]
    pub add_report_element_tertiary: String,
    #[serde(skip)]
    pub add_report_element_style: usize,
    #[serde(skip)]
    pub add_report_element_status: usize,
    #[serde(skip)]
    pub add_report_element_source_run: usize,
    #[serde(skip)]
    pub remove_report_block_open: bool,
    #[serde(skip)]
    pub insert_result_document_open: bool,
    #[serde(skip)]
    pub insert_result_document_index: usize,
    #[serde(skip)]
    pub insert_result_caption: String,
    #[serde(skip)]
    pub insert_result_alternative_text: String,
    #[serde(skip)]
    pub insert_result_sizing: usize,
    #[serde(skip)]
    pub insert_result_frozen: bool,
    #[serde(skip)]
    pub report_template_draft: usize,
    #[serde(skip)]
    pub page_update_policy_draft: usize,
    #[serde(skip)]
    pub transaction_error: Option<String>,
}

/// Canonical discovery projection used by the mockup's specialist-tool
/// browser. Pins and favorites are personal application preferences, while
/// recent tools are bounded device-local history. None of these collections
/// creates, copies, or changes the owner of an engineering document.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpecialistToolBrowserState {
    /// Search and filter are task-local review state. They deliberately reset
    /// when a new application session starts instead of becoming project data.
    #[serde(skip)]
    pub query: String,
    #[serde(skip)]
    pub filter: SpecialistToolFilter,
    #[serde(skip)]
    pub focus_search: bool,
    /// Personal discovery preferences retained by the application session.
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub favorites: Vec<SurfaceId>,
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub pinned: Vec<SurfaceId>,
    /// Newest-first, bounded device-local navigation history.
    #[serde(default, deserialize_with = "deserialize_specialist_surfaces")]
    pub recents: Vec<SurfaceId>,
}

impl Default for SpecialistToolBrowserState {
    fn default() -> Self {
        Self {
            query: String::new(),
            filter: SpecialistToolFilter::All,
            focus_search: false,
            favorites: Vec::new(),
            pinned: Vec::new(),
            recents: Vec::new(),
        }
    }
}

impl SpecialistToolBrowserState {
    pub(super) const RECENT_LIMIT: usize = 12;

    #[must_use]
    pub fn is_favorite(&self, surface: SurfaceId) -> bool {
        self.favorites.contains(&surface)
    }

    #[must_use]
    pub fn is_pinned(&self, surface: SurfaceId) -> bool {
        self.pinned.contains(&surface)
    }

    #[must_use]
    pub fn is_recent(&self, surface: SurfaceId) -> bool {
        self.recents.contains(&surface)
    }

    pub fn toggle_favorite(&mut self, surface: SurfaceId) {
        toggle_surface_membership(&mut self.favorites, surface);
    }

    pub fn toggle_pin(&mut self, surface: SurfaceId) {
        toggle_surface_membership(&mut self.pinned, surface);
    }

    pub fn record_recent(&mut self, surface: SurfaceId) {
        if surface.archetype() != SurfaceArchetype::SpecialistWorkspace {
            return;
        }
        self.recents.retain(|candidate| *candidate != surface);
        self.recents.insert(0, surface);
        self.recents.truncate(Self::RECENT_LIMIT);
    }

    /// Remove non-specialist and duplicate identities from restored personal
    /// metadata. This is intentionally lossless for every still-canonical
    /// specialist identity and never guesses replacements for removed IDs.
    pub fn normalize(&mut self) {
        normalize_specialist_list(&mut self.favorites, None);
        normalize_specialist_list(&mut self.pinned, None);
        normalize_specialist_list(&mut self.recents, Some(Self::RECENT_LIMIT));
    }
}

pub(super) fn toggle_surface_membership(surfaces: &mut Vec<SurfaceId>, surface: SurfaceId) {
    if let Some(index) = surfaces.iter().position(|candidate| *candidate == surface) {
        surfaces.remove(index);
    } else if surface.archetype() == SurfaceArchetype::SpecialistWorkspace {
        surfaces.push(surface);
    }
}

pub(super) fn normalize_specialist_list(surfaces: &mut Vec<SurfaceId>, limit: Option<usize>) {
    let mut normalized = Vec::with_capacity(surfaces.len());
    for surface in surfaces.drain(..) {
        if surface.archetype() == SurfaceArchetype::SpecialistWorkspace
            && !normalized.contains(&surface)
        {
            normalized.push(surface);
        }
    }
    if let Some(limit) = limit {
        normalized.truncate(limit);
    }
    *surfaces = normalized;
}

pub(super) fn deserialize_specialist_surfaces<'de, D>(
    deserializer: D,
) -> Result<Vec<SurfaceId>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    let Some(stable_ids) = value.as_array() else {
        return Ok(Vec::new());
    };
    Ok(stable_ids
        .iter()
        .filter_map(serde_json::Value::as_str)
        .filter_map(|stable_id| stable_id.parse().ok())
        .collect())
}

/// Mockup-authored specialist discovery projections.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SpecialistToolFilter {
    #[default]
    All,
    ActiveProfile,
    Pinned,
    Favorites,
    Recent,
}

impl SpecialistToolFilter {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::ActiveProfile,
        Self::Pinned,
        Self::Favorites,
        Self::Recent,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::ActiveProfile => "Active profile",
            Self::Pinned => "Pinned",
            Self::Favorites => "Favorites",
            Self::Recent => "Recent",
        }
    }
}

/// Domain projection selected in the mockup-specified notification center.
/// The underlying activity stream is never discarded when this changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NotificationFilter {
    #[default]
    All,
    Jobs,
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
    pub const ALL: [Self; 3] = [Self::All, Self::Jobs, Self::System];

    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Jobs => "Jobs",
            Self::System => "System",
        }
    }
}

/// Which instance field an inspector inline edit is editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InlineEditField {
    /// The reference designator. Validated against the SPICE designator
    /// rules and case-insensitive uniqueness before it is ever applied.
    Instance,
    /// The instance value.
    Value,
    /// The complete free-form instance-parameter override string used by
    /// component families that do not publish a typed parameter contract.
    Parameters,
    /// One `key=value` instance parameter.
    Parameter(String),
}

/// One live inline-edit session in the inspector.
///
/// Edits apply to the design on every keystroke so the canvas, netlist, and
/// connectivity track what the field says. The undo history, however,
/// records **one** entry per session: the snapshot is captured when the
/// field takes focus and committed when it loses it, so typing a value is a
/// single undo step rather than one per character.
#[derive(Debug, Clone, Default)]
pub struct InlineEdit {
    /// Instance being edited, and which of its fields.
    target: Option<(u64, InlineEditField)>,
    /// Design state as it stood when the field took focus.
    before: Option<crate::state::SchematicSnapshot>,
    /// Text as typed, which may not yet be a legal value.
    buffer: String,
    /// Why the typed text has not been applied, when it has not.
    error: Option<String>,
}

impl InlineEdit {
    /// The buffer for `target`, or `None` when a different field (or no
    /// field) owns the session.
    pub fn buffer_for(&self, component: u64, field: &InlineEditField) -> Option<&str> {
        (self.target.as_ref() == Some(&(component, field.clone()))).then_some(self.buffer.as_str())
    }

    /// Which of `component`'s fields holds the open session, if any.
    ///
    /// The inspector asks so a group of editable rows can reserve its
    /// validation strip for exactly as long as one of its own fields is being
    /// typed into, rather than permanently.
    pub fn editing_field(&self, component: u64) -> Option<&InlineEditField> {
        self.target
            .as_ref()
            .filter(|(id, _)| *id == component)
            .map(|(_, field)| field)
    }

    /// Why the open session's text was rejected, if it was.
    pub fn error_for(&self, component: u64, field: &InlineEditField) -> Option<&str> {
        (self.target.as_ref() == Some(&(component, field.clone())))
            .then_some(self.error.as_deref())
            .flatten()
    }

    /// Open a session on `field`, seeding the buffer with the current text
    /// and capturing the snapshot this session will fold into one undo
    /// entry. Re-opening the same field keeps the session intact.
    pub fn begin(
        &mut self,
        component: u64,
        field: InlineEditField,
        current: &str,
        before: crate::state::SchematicSnapshot,
    ) {
        if self.target.as_ref() == Some(&(component, field.clone())) {
            return;
        }
        self.target = Some((component, field));
        self.before = Some(before);
        self.buffer = current.to_owned();
        self.error = None;
    }

    /// Replace the typed text.
    pub fn set_buffer(&mut self, text: String) {
        self.buffer = text;
    }

    /// Record why the typed text was not applied, or clear the rejection.
    pub fn set_error(&mut self, error: Option<String>) {
        self.error = error;
    }

    /// End the session, returning the snapshot to fold into one undo entry.
    pub fn end(&mut self) -> Option<crate::state::SchematicSnapshot> {
        self.target = None;
        self.buffer.clear();
        self.error = None;
        self.before.take()
    }

    /// Abandon any session that does not belong to `component` — selection
    /// moved on, so its buffer and snapshot are no longer meaningful.
    pub fn release_unless(
        &mut self,
        component: Option<u64>,
    ) -> Option<crate::state::SchematicSnapshot> {
        match (&self.target, component) {
            (Some((owner, _)), Some(id)) if *owner == id => None,
            (Some(_), _) => self.end(),
            (None, _) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ModelsOperationalState;

    #[test]
    fn models_operational_state_registry_matches_the_mockup_contract() {
        assert_eq!(
            ModelsOperationalState::ALL.map(ModelsOperationalState::label),
            [
                "Ready",
                "Invalid input",
                "Execution error",
                "Read-only",
                "Offline",
                "Conflict",
                "Stale",
                "Permission",
                "Entitlement",
                "Cancelled",
                "Rollback",
                "Partial",
                "Corrupted",
                "Recovered",
            ]
        );
    }

    #[test]
    fn models_failures_map_to_actionable_operational_states() {
        use ModelsOperationalState as State;

        for (message, expected) in [
            ("Section name is required", State::InvalidInput),
            ("Project is read-only", State::ReadOnly),
            ("Catalog is offline", State::Offline),
            ("Duplicate model conflict", State::Conflict),
            ("Source drift detected", State::Stale),
            ("Access denied by permission policy", State::Permission),
            ("Pack license is restricted", State::Entitlement),
            ("Operation cancelled", State::Cancelled),
            ("Transaction rolled back", State::Rollback),
            ("Import completed with partial results", State::Partial),
            ("Metadata is corrupted", State::Corrupted),
            ("Parser failed", State::ExecutionError),
        ] {
            assert_eq!(State::from_failure(message), expected, "{message}");
        }
    }
}
