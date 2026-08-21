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
    /// Which point of the active run the executed-deck viewer opens.
    ///
    /// Review state, and never durable: it indexes the points of one session's
    /// retained decks, and a restored session holds none of them.
    #[serde(skip)]
    pub executed_deck_point: usize,
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
            executed_deck_point: 0,
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
            // Both MESFET polarities. `mesfet-p` was missing, so every
            // p-channel MESFET the catalog classified — the foundation's own
            // among them — was reachable from no class chip at all.
            Self::JfetAndHemt => &["jfet-n", "jfet-p", "gasfet", "mesfet-n", "mesfet-p"],
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
    Corrupted,
    /// The publisher withdrew what the operation named.
    ///
    /// Its own rung because no other one is true of it. The bytes are
    /// authentic, so it is not `Corrupted`; the licence grants what it always
    /// did, so it is not `Entitlement`; and nothing about the project or the
    /// catalog moved under the operation, so it is not `Stale`. What happened
    /// is that the thing being asked for was recalled, and the only recovery
    /// is to use something else — which is a consequence no other rung states.
    Recalled,
}

impl ModelsOperationalState {
    /// The registry the coverage tests walk. `label` and `consequence` are
    /// live — the packs page paints both — but nothing in a shipped frame
    /// iterates every variant at once, so the array itself stays with the
    /// tests that do.
    #[cfg(test)]
    pub const ALL: [Self; 12] = [
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
        Self::Corrupted,
        Self::Recalled,
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
            Self::Corrupted => "Corrupted",
            Self::Recalled => "Recalled",
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
            // "against the current project" was the old second sentence, and
            // it misdirected the two catalog cases outright: a rolled-back or
            // expired catalog is nothing a reader does anything to the project
            // about. What is true of every Stale refusal is that the thing it
            // was decided against has moved on.
            Self::Stale => {
                "The candidate was discarded without mutation because the project or catalog \
                 moved under it. Run it again against what is current now."
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
            Self::Corrupted => {
                "The bytes read do not describe what they claim to, so nothing from them entered \
                 the catalog."
            }
            Self::Recalled => {
                "Nothing changed on this machine. The publisher withdrew this release; anything \
                 already installed or retained from it keeps working, and nothing new can be \
                 taken from it."
            }
        }
    }

    #[must_use]
    pub fn from_failure(message: &str) -> Self {
        let normalized = message.to_ascii_lowercase();
        // Asked first, and deliberately. A recall carries the publisher's own
        // prose, and that prose is arbitrary: a reason mentioning a licence or
        // an invalid card would otherwise be classified by the *reason* rather
        // than by the refusal, and the reader would be told to fix something.
        if normalized.contains("recalled by its publisher") {
            return Self::Recalled;
        }
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
        } else if normalized.contains("corrupt") || normalized.contains("malformed") {
            Self::Corrupted
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
    /// The pack this operation puts on the machine, when it is one that does.
    ///
    /// One model-catalog operation runs at a time, so this plus
    /// `model_import_in_progress` is exactly "which pack is landing right
    /// now" — which is what lets the ledger light the row a reader is waiting
    /// on rather than every row or none. It is recorded here rather than in a
    /// field of its own because this record is already written at every start
    /// and read after every finish, and a second field would be a second
    /// thing to clear.
    ///
    /// `None` for a catalog refresh, a removal, a re-proof and every
    /// model-source import: none of them puts a release on this machine.
    pub landing_pack: Option<String>,
}

/// One retained source whose bytes no longer hash to what the project accepted.
///
/// The pinned digest is the whole of a project's claim to reproducibility: a
/// run authenticates every executable source against it and refuses when they
/// disagree. Discovering that disagreement in a preflight refusal, minutes
/// before a run, is the expensive place to discover it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelSourceDriftFinding {
    pub path: std::path::PathBuf,
    /// The digest the accepted closure recorded, shortened for display.
    pub pinned: String,
    /// What the bytes hash to now, shortened the same way. `None` means the
    /// source could not be read at all, which is its own kind of drift.
    pub on_disk: Option<String>,
    /// What changed, when both the accepted and the present bytes are on hand
    /// to compare. A retained source keeps only one copy, so its accepted
    /// bytes are gone and nothing can be said about the difference.
    pub change: Option<String>,
}

/// What a drift report describes: which project, and which catalogue.
///
/// Both halves, because neither alone identifies the thing scanned. The
/// revision moves when a project publishes one, which is what re-arms the scan
/// after an import or a re-pin. The catalogue key is *content*, which is what
/// makes a report expire when the catalogue is replaced wholesale — opening a
/// project, accepting a recovery comparison, restoring a design-history
/// candidate. None of those routes touches this view state, and a project file
/// arrives carrying whatever revision it was saved with, so two projects at
/// equal revisions is entirely ordinary. A catalogue cannot present a key it
/// did not earn, however it arrived.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelSourceDriftScope {
    pub project_revision: u64,
    pub catalog_key: u64,
}

/// Pinned-versus-present source drift, as of the last scan.
///
/// Deciding this rehashes every retained byte, so it is decided on events —
/// opening the workspace, finishing an import, asking for it — and never on a
/// frame. That is also why the report records what it describes: a clean result
/// from before an import, or from another project, is not a clean result.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ModelSourceDrift {
    pub scanned: Option<ModelSourceDriftScope>,
    /// When the scan ran, in UTC, when the platform offered a usable clock.
    pub scanned_at: Option<String>,
    /// Findings by library name. A library with none is absent from the map,
    /// so an empty map is the healthy state and says nothing.
    pub libraries: std::collections::BTreeMap<String, Vec<ModelSourceDriftFinding>>,
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
    /// Which shipped-corpus pack the corpus table below the ledger has
    /// selected.
    ///
    /// Its own field, because it is its own table. Both selections lived in
    /// `selected_pack`, and the corpus detail writes its fallback back to
    /// state on every frame — so selecting a pack in the ledger was overwritten
    /// by the corpus table before the next paint, and the ledger's inspector
    /// silently reverted to its first row.
    #[serde(default)]
    pub selected_corpus_pack: Option<String>,
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
    /// What the catalog states the offered release changes about the held one,
    /// for the pack the ledger has selected.
    ///
    /// Never durable, and never recomputed for a question it has already
    /// answered: the key inside it names the *digest* of the snapshot both
    /// release records were read from, so a catalog replaced wholesale cannot
    /// be handed an answer it did not earn, and a repainting inspector reads
    /// the value rather than walking two part lists sixty times a second.
    #[serde(skip)]
    pub release_diff: Option<crate::state::model_hub::ReleaseDiff>,
    /// Source drift as of the last event-driven scan. Never durable: a saved
    /// verdict about bytes is a verdict about a moment that has passed.
    #[serde(skip)]
    pub source_drift: ModelSourceDrift,
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
    /// What the last recall report was about: which catalog, and which pins.
    ///
    /// A project pinned to a recalled release is told once per (catalog, pins)
    /// pair rather than on every frame, and told *again* when either half
    /// moves — which is what makes opening a second project, or refreshing
    /// into a catalog that recalls something, a fresh piece of news. Both
    /// halves are content, so neither a project nor a catalog can present a
    /// key it did not earn, however it arrived.
    ///
    /// Never durable. A saved "already told them" would silence the notice for
    /// a reader who has not been told anything this session.
    #[serde(skip)]
    pub recall_notice: Option<RecallNoticeScope>,
}

/// What a recall report describes: which catalog, and which project pins.
///
/// Both halves, because neither alone identifies the thing reported on. The
/// catalog half is the snapshot digest, so a refresh that changes the recall
/// list re-arms the notice; the pin half moves when the project's commitments
/// do, including when a different project is opened over this one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecallNoticeScope {
    pub catalog_digest: String,
    pub pack_pin_key: u64,
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
            selected_corpus_pack: None,
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
            release_diff: None,
            source_drift: ModelSourceDrift::default(),
            model_import_in_progress: false,
            model_import_label: None,
            model_import_progress: None,
            dialog: None,
            hub_facet: ModelHubFacet::default(),
            catalog_refresh_requested: false,
            recall_notice: None,
        }
    }
}

/// Which packs the Model Hub ledger lists.
///
/// The ledger always spans installed *and* available packs, because "what this
/// machine has" and "what the catalog offers" are the same question asked once.
/// The facet narrows that one list; it never switches between two.
///
/// # Why these five, and not the old five
///
/// The facets used to name release *states* — `Updatable` and `Incompatible`
/// were two of them — which made a reader pick the rail entry matching whatever
/// exception they had already guessed at. The ledger states one exception per
/// pack in its own column now, so the question a facet answers is the reader's:
/// what needs me, what is here, what has this design committed to, and what is
/// on offer. A saved session that named a state facet is restored onto
/// `NeedsAttention`, which is where both of those exceptions are now reported.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelHubFacet {
    #[default]
    All,
    /// An update on offer and a release this build cannot run are both
    /// attention rungs, so both old spellings restore here.
    #[serde(alias = "updatable", alias = "incompatible")]
    NeedsAttention,
    Installed,
    Pinned,
    Available,
}

impl ModelHubFacet {
    pub const ALL: [Self; 5] = [
        Self::All,
        Self::NeedsAttention,
        Self::Installed,
        Self::Pinned,
        Self::Available,
    ];

    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::All => "All",
            Self::NeedsAttention => "Needs attention",
            Self::Installed => "Installed",
            Self::Pinned => "Pinned",
            Self::Available => "Available",
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
    /// The Model Hub's reference material, in one place: what the held catalog
    /// is, the contract it is accepted under, and the last thing this session
    /// tried to do with it.
    ///
    /// It carries no fields because it states nothing the workspace does not
    /// already hold. Capturing a copy would let the card and the page it was
    /// opened from disagree after a refresh lands underneath it.
    HeldCatalog,
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
    /// The findings one library's last source-drift scan produced, and the
    /// one repair. It names the library rather than carrying the findings so
    /// the dialog and the pages behind it read the same retained report.
    ResolveDrift {
        library: String,
    },
}

/// Destination that can resolve a blocking simulation-preflight finding.
/// The dialog stores semantic destinations instead of callbacks so a report
/// remains deterministic for the exact project revision that produced it.
///
/// A destination carries the identity of the object it lands on rather than
/// leaving the reader to find it again. The identity is threaded from the
/// same values the finding's prose was built from — never parsed back out of
/// that prose, which would make the sentence a load-bearing format.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreflightRemediation {
    DesignChecks,
    SimulationPlan,
    ProjectTechnology,
    /// One page of the Models workspace, and the object it opens on. A
    /// model-binding finding is repaired where the bindings live — Corners &
    /// sections — and not by re-running the source checks that reported it.
    ///
    /// `library` is `None` for a finding that names no single library; the
    /// page then opens on whatever it already had selected, which is the
    /// honest answer when the finding is about the closure rather than about
    /// one of its members.
    Models {
        page: ModelsPage,
        library: Option<String>,
        /// The corner inside `library` the finding is about, when it is about
        /// one. Never `Some` without a `library` — a corner name is only
        /// unique within its library.
        corner: Option<String>,
    },
    /// The netlist source the run's executable deck is generated from.
    ///
    /// A netlist-stage preparation failure is a defect in the deck, so it is
    /// repaired in the Netlist workspace. It used to route to the design
    /// checks, which re-ran the checks that had already passed and then
    /// opened the Verify workspace — never the offending source.
    NetlistSource {
        /// The 1-based deck line the failure named, where it named one.
        ///
        /// Threaded from the parser's own report, never read back out of the
        /// finding's prose. `None` means the deck opens without the cursor
        /// being moved — the finding knew the document but not the line, and
        /// landing on an arbitrary one would be worse than landing on none.
        line: Option<usize>,
    },
}

impl PreflightRemediation {
    /// The Models destination for a finding that names no single library.
    pub const fn models_page(page: ModelsPage) -> Self {
        Self::Models {
            page,
            library: None,
            corner: None,
        }
    }

    /// The Corners & sections destination for a finding about one library,
    /// and optionally one corner inside it.
    pub fn model_corner(library: impl Into<String>, corner: Option<String>) -> Self {
        Self::Models {
            page: ModelsPage::Corners,
            library: Some(library.into()),
            corner,
        }
    }

    /// Whether this finding stands between the design and an executable
    /// netlist.
    ///
    /// The preflight strip's Netlist cell asks this question, and asking it
    /// here is what keeps a new destination for a netlist-stage failure from
    /// silently turning that cell green.
    pub const fn blocks_executable_netlist(&self) -> bool {
        matches!(self, Self::DesignChecks | Self::NetlistSource { .. })
    }
}

/// One non-blocking finding in a simulation-preflight report.
///
/// An advisory is something the run will proceed *through*, so its whole value
/// is that a reader can weigh it — and weighing it usually means going and
/// looking at the thing it names. It carried only prose for a release, which
/// left every advisory ending in an instruction to navigate somewhere the
/// report could have offered to open.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightAdvisory {
    pub message: String,
    /// Where this is repaired, when repairing it has a home. `None` for an
    /// advisory that states a fact rather than naming a thing to change.
    pub remediation: Option<PreflightRemediation>,
}

impl From<String> for PreflightAdvisory {
    fn from(message: String) -> Self {
        Self {
            message,
            remediation: None,
        }
    }
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
    pub advisories: Vec<PreflightAdvisory>,
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

    /// Where the retained report stands against the live design.
    ///
    /// The verdict is derived from [`PreflightReport::is_current_for`] and
    /// nothing else, so every surface that states preflight currency states
    /// the same thing. It lives beside the report rather than in one surface
    /// because the report is not the Analyses page's private fact: a reader
    /// editing outputs, models, or solver options is changing exactly the
    /// inputs that expire it.
    pub fn currency(
        &self,
        project_revision: u64,
        topology_root: &str,
        topology_revision: u64,
        topology_closure: &[(String, u64)],
        simulation_plan: Option<(
            crate::product::SimulationPlanId,
            crate::product::ObjectRevision,
        )>,
    ) -> PreflightCurrency {
        let Some(report) = self.report.as_ref() else {
            return PreflightCurrency::Absent;
        };
        if !report.is_current_for(
            project_revision,
            topology_root,
            topology_revision,
            topology_closure,
            simulation_plan,
        ) {
            return PreflightCurrency::Expired;
        }
        match report.prepared.as_ref() {
            Some(prepared) if report.blockers.is_empty() => PreflightCurrency::Authorized {
                tasks: prepared.task_count,
            },
            _ => PreflightCurrency::Blocked {
                blockers: report.blockers.len(),
            },
        }
    }
}

/// What the Console page is narrowed to, and the producer that narrowing is
/// about.
///
/// The results navigator's "Reveal producer log" used to print one info line
/// and open the console unfiltered, which stated the producer and then handed
/// back every entry in the session. The filter is what makes that row do what
/// it says.
///
/// Two rules match, and they are deliberately different in kind. The first is
/// exact: an entry tagged with this producer's stable path. Nothing emits that
/// tag yet — the simulation controller writes `None` for every entry's context
/// — so the second rule carries the row today: a simulation entry that names
/// the producer's own quantity, which is how the measurement and
/// operating-point echoes name what they are reporting. The chip says which
/// producer is being matched, and the empty state says plainly when nothing
/// does.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConsoleProducerFilter {
    /// Stable `dataset/…/analysis/…/…` path of the revealed producer.
    pub producer: String,
    /// The producer's own quantity or artifact name, as an emitted entry
    /// would write it. Empty leaves only the exact tag rule.
    pub quantity: String,
    /// Ask the console body to scroll to the newest matching entry once.
    /// Consumed by the renderer, the way a preflight run request is.
    pub scroll_to_newest: bool,
}

impl ConsoleProducerFilter {
    pub fn new(producer: impl Into<String>, quantity: impl Into<String>) -> Self {
        Self {
            producer: producer.into(),
            quantity: quantity.into(),
            scroll_to_newest: true,
        }
    }

    /// What the chip calls this producer: its quantity name when it has one,
    /// and the stable path otherwise. Never a truncation of the path — a
    /// reader cannot check a partial identity.
    pub fn label(&self) -> &str {
        if self.quantity.is_empty() {
            &self.producer
        } else {
            &self.quantity
        }
    }

    pub fn matches(&self, entry: &crate::diagnostics::LogEntry) -> bool {
        if entry.context.as_deref() == Some(self.producer.as_str()) {
            return true;
        }
        !self.quantity.is_empty()
            && entry.source == crate::diagnostics::LogSource::Simulation
            && entry
                .message
                .to_ascii_lowercase()
                .contains(&self.quantity.to_ascii_lowercase())
    }
}

/// Where a session's retained preflight report stands against the live design.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PreflightCurrency {
    /// No preflight has been run in this session.
    Absent,
    /// A report exists, but the design or the active plan moved underneath it.
    Expired,
    /// A current report that still lists blocking findings.
    Blocked { blockers: usize },
    /// A current report that authorizes dispatch.
    Authorized { tasks: usize },
}

impl PreflightCurrency {
    /// Whether a chip should be painted at all. Nothing is stated about a
    /// preflight that was never run — an absent report is the ordinary state
    /// of a session that has not reached dispatch yet, and a chip announcing
    /// it on every route would be noise rather than evidence.
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::Absent)
    }

    /// Whether the retained report leaves something for the reader to do
    /// before a run can be authorized — an expired report has to be rerun, a
    /// blocked one has findings to clear and then rerun.
    pub const fn wants_rerun(self) -> bool {
        matches!(self, Self::Expired | Self::Blocked { .. })
    }

    /// The chip's leading word, in the vocabulary the preflight dialog uses.
    pub const fn status(self) -> &'static str {
        match self {
            Self::Absent => "not run",
            Self::Expired => "expired",
            Self::Blocked { .. } => "blocked",
            Self::Authorized { .. } => "current",
        }
    }

    /// What the status is counted in, when it is counted in anything.
    pub fn detail(self) -> Option<String> {
        match self {
            Self::Absent => None,
            Self::Expired => Some("design or plan changed".to_owned()),
            Self::Blocked { blockers } => Some(format!(
                "{blockers} blocking finding{}",
                if blockers == 1 { "" } else { "s" }
            )),
            Self::Authorized { tasks } => Some(format!(
                "{tasks} task{} ready",
                if tasks == 1 { "" } else { "s" }
            )),
        }
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
    use super::{ModelHubFacet, ModelsOperationalState, RSpicePartFacet};

    /// Every class the catalog assigns is reachable from a class chip, or is
    /// named here as one nothing offers.
    ///
    /// A class no facet filters on hides its parts behind "All classes", which
    /// is how the two foundation power MOSFETs and every p-channel MESFET
    /// became unreachable — each by a different one-line omission, and neither
    /// visible from the file that caused it. The five below have no chip
    /// because no chip has been designed for them; they are a decision, not an
    /// oversight, and designing one is what removes a name from this list.
    #[test]
    fn every_class_the_catalog_assigns_is_reachable_from_a_chip() {
        const NO_CHIP_OFFERS: &[&str] = &[
            "switch",
            "switch-current",
            "coupling",
            "distributed-rc",
            "piezo",
        ];

        for (token, device) in crate::state::model_library::DEVICE_CLASS {
            if NO_CHIP_OFFERS.contains(device) {
                continue;
            }
            assert!(
                RSpicePartFacet::ALL
                    .iter()
                    .any(|facet| facet.device_filters().contains(device)),
                "'{token}' classifies to '{device}', which no class chip filters on"
            );
        }
    }

    /// A session saved under any spelling this facet has ever had still opens.
    ///
    /// The facet is durable view state, so a rename is a compatibility event:
    /// without the aliases, a session naming `updatable` deserializes as an
    /// error and takes the whole `ModelsWorkbenchViewState` down with it — the
    /// reader loses every selection in the workspace to a facet rename.
    #[test]
    fn every_spelling_this_facet_has_shipped_still_deserializes() {
        for (stored, expected) in [
            ("\"all\"", ModelHubFacet::All),
            ("\"needs-attention\"", ModelHubFacet::NeedsAttention),
            ("\"installed\"", ModelHubFacet::Installed),
            ("\"pinned\"", ModelHubFacet::Pinned),
            ("\"available\"", ModelHubFacet::Available),
            // Retired spellings. Both named an exception the ledger now
            // reports in its attention column, so both restore onto the facet
            // that collects exceptions rather than onto "all", which would
            // silently widen what the reader had narrowed.
            ("\"updatable\"", ModelHubFacet::NeedsAttention),
            ("\"incompatible\"", ModelHubFacet::NeedsAttention),
        ] {
            assert_eq!(
                serde_json::from_str::<ModelHubFacet>(stored).expect(stored),
                expected,
                "{stored} no longer opens"
            );
        }
        assert_eq!(
            serde_json::to_string(&ModelHubFacet::NeedsAttention).expect("serializes"),
            "\"needs-attention\"",
            "the spelling written today is the kebab-case one"
        );
    }

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
                "Corrupted",
                "Recalled",
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
            ("Metadata is corrupted", State::Corrupted),
            ("malformed archive: truncated", State::Corrupted),
            ("Parser failed", State::ExecutionError),
            // The three trust refusals the Model Hub raises, in the exact
            // sentences it writes. A classifier that reads message text is
            // only as good as the texts it was checked against.
            (
                "the model hub offered catalog serial 40, which is stale beside serial 41 this \
                 machine has already accepted; the held catalog was kept",
                State::Stale,
            ),
            (
                "the held catalog is stale: it expired at 2026-07-01T00:00:00Z, so the hub \
                 offers nothing until it is refreshed",
                State::Stale,
            ),
            (
                "rspice-opamps 2.0.0 was recalled by its publisher: an invalid bias network was \
                 published under a restricted licence",
                State::Recalled,
            ),
            // The three states this replaces — rollback, partial, recovered —
            // had classifiers and no producer: nothing that writes a models
            // receipt could say "rolled back", "partial" or "recovered", so
            // they were vocabulary the workspace could never reach. They now
            // land where every other unclassified failure lands.
            ("Transaction rolled back", State::ExecutionError),
            (
                "Import completed with partial results",
                State::ExecutionError,
            ),
        ] {
            assert_eq!(State::from_failure(message), expected, "{message}");
        }
    }
}
