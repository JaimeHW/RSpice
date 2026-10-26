//! The Models & PDKs workspace's session state: the catalog's facets, sort
//! and selections, what the last model operation concluded, and the
//! unsigned PDK draft kept for recovery.
//!
//! None of it is engineering data. Models, sources, symbols and PDK records
//! stay in the project's model library and the PDK configuration; this is
//! what the workspace shows them through.

use serde::{Deserialize, Serialize};

use crate::workbench::state::{ModelsCatalogScope, ModelsWorkbenchDialog};

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

/// Which column of the project model catalog decides its order.
///
/// Presentation only: it reorders rows the page has already derived and never
/// changes which rows those are. The default is the order the table has always
/// opened in — model name, case-folded, library breaking ties.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ModelsCatalogSortKey {
    #[default]
    Model,
    Family,
    Source,
    UsedBy,
    Vectors,
    Status,
}

/// A sortable table's ordering: which column, and which direction.
///
/// Durable, because it is a reading preference rather than an engineering
/// fact — a reader who ordered the catalog by status wants it that way again
/// when the project reopens. Both halves carry `serde(default)` so a session
/// written before this existed restores the default order instead of failing
/// the whole view state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct ModelsTableSort {
    #[serde(default)]
    pub key: ModelsCatalogSortKey,
    /// `false` is ascending, which is what a column first clicked gives.
    #[serde(default)]
    pub descending: bool,
}

impl ModelsTableSort {
    /// Apply a click on `key`: the same column toggles direction, a different
    /// one takes over ascending.
    pub fn toggle(&mut self, key: ModelsCatalogSortKey) {
        if self.key == key {
            self.descending = !self.descending;
        } else {
            self.key = key;
            self.descending = false;
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
    /// How the project catalog table is ordered.
    #[serde(default)]
    pub catalog_sort: ModelsTableSort,
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
            catalog_sort: ModelsTableSort::default(),
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

/// Recoverable working state for the application-owned unsigned PDK draft.
/// Runtime authority remains in `PdkConfig`; this record protects only edits
/// that have not yet completed the explicit durable Save draft transaction.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PdkTechnologyAuthoringRecovery {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) working_draft: Option<crate::state::pdk_config::PdkTechnologyDraft>,
    #[serde(default)]
    pub(crate) dirty: bool,
}

#[cfg(test)]
mod tests;
