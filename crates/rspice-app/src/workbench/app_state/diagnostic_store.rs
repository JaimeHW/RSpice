//! The conditions that stand against the design, in one place.
//!
//! Before this, every producer kept its findings in whatever shape suited it
//! and the Problems page assembled a view out of four of them plus the
//! session log. That is why the same finding could be current on one surface
//! and absent from another, why a refusal the reader had already dismissed
//! stood on the page forever, and why "where is this?" had a different
//! answer per producer.
//!
//! One store answers all three. A record is keyed by its canonical
//! `diagnostic_id`, says what it is about, carries the one route that repairs
//! it, and belongs to a batch whose currency is decided by a digest of the
//! inputs the batch was derived from — the same rule
//! [`super::design_checks`] already used for schematic check receipts,
//! applied to every producer. Nothing here decides how a finding is drawn;
//! that is the Problems page's business.
//!
//! Two producers are read rather than copied. The netlist editor and the code
//! editors already publish canonical collections behind an `Arc`, sized for a
//! million records, and copying those into this store every validation would
//! buy nothing and cost the editors their publication model. The listing in
//! [`listing`] reads them in place.

pub(in crate::workbench) mod listing;
mod producers;

use std::collections::{BTreeMap, HashMap};
use std::sync::Arc;

use sha2::{Digest, Sha256};

use rspice_app_types::diagnostics::{CanonicalDiagnosticMetadata, DiagnosticSeverity};

use crate::diagnostics::LogAnchor;
use crate::product::{ContentDigest, ProjectId};
use crate::state::CellViewRef;
use crate::workbench::state::PreflightRemediation;

use super::AppState;

const DIAGNOSTIC_INPUT_DIGEST_DOMAIN: &[u8] = b"rspice-diagnostic-inputs/v1\0";

/// How bad a finding is, on the one ladder every producer reports against.
///
/// Five rungs rather than the canonical four because the schematic checks
/// distinguish a critical failure from an error and the reader is entitled to
/// that word: `Critical` means the design cannot be simulated at all,
/// `Error` means a specific thing is wrong with it. Both are errors when the
/// counts are added up.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::workbench) enum DiagnosticLevel {
    Hint,
    Info,
    Warning,
    Error,
    Critical,
}

impl DiagnosticLevel {
    /// Every rung, worst first — the order a reader scans a severity filter
    /// in, and the order a worst-first tour visits findings in.
    pub(in crate::workbench) const ALL: [Self; 5] = [
        Self::Critical,
        Self::Error,
        Self::Warning,
        Self::Info,
        Self::Hint,
    ];

    /// This rung's position in [`Self::ALL`].
    ///
    /// One mapping, because anything that keeps a count or a flag per severity
    /// indexes by it, and two copies of the order would be two orders.
    pub(in crate::workbench) const fn rung(self) -> usize {
        match self {
            Self::Critical => 0,
            Self::Error => 1,
            Self::Warning => 2,
            Self::Info => 3,
            Self::Hint => 4,
        }
    }

    pub(in crate::workbench) const fn display_name(self) -> &'static str {
        match self {
            Self::Hint => "Hint",
            Self::Info => "Info",
            Self::Warning => "Warning",
            Self::Error => "Error",
            Self::Critical => "Critical",
        }
    }

    /// Whether this rung stops the design working, which is what separates an
    /// error from an advisory in every count the product states.
    pub(in crate::workbench) const fn is_error(self) -> bool {
        matches!(self, Self::Error | Self::Critical)
    }
}

impl From<DiagnosticSeverity> for DiagnosticLevel {
    fn from(severity: DiagnosticSeverity) -> Self {
        match severity {
            DiagnosticSeverity::Hint => Self::Hint,
            DiagnosticSeverity::Info => Self::Info,
            DiagnosticSeverity::Warning => Self::Warning,
            DiagnosticSeverity::Error => Self::Error,
        }
    }
}

impl From<crate::workbench::documents::netlist_document::DiagnosticSeverity> for DiagnosticLevel {
    fn from(severity: crate::workbench::documents::netlist_document::DiagnosticSeverity) -> Self {
        use crate::workbench::documents::netlist_document::DiagnosticSeverity as Netlist;
        match severity {
            Netlist::Hint => Self::Hint,
            Netlist::Info => Self::Info,
            Netlist::Warning => Self::Warning,
            Netlist::Error => Self::Error,
        }
    }
}

impl From<rspice_design::drc::DrcSeverity> for DiagnosticLevel {
    fn from(severity: rspice_design::drc::DrcSeverity) -> Self {
        use rspice_design::drc::DrcSeverity;
        match severity {
            DrcSeverity::Info => Self::Info,
            DrcSeverity::Warning => Self::Warning,
            DrcSeverity::Error => Self::Error,
            DrcSeverity::Critical => Self::Critical,
        }
    }
}

/// What a finding is about: the project it belongs to, the cell view it is
/// inside where it is inside one, and the object the producer named.
///
/// The object is the producer's own words — `Component M1`, `line 42`,
/// `Corner tt` — because the producer is the only thing that knows how the
/// object is addressed in its own domain. What this type adds is the context
/// that makes those words unambiguous across a project.
///
/// The two are kept apart rather than spelled into one line, because the grid
/// prints them in two columns: the object is what the finding is about, and
/// the cell view is where that object is. Spelling them together was what made
/// every row of one view repeat its path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) struct DiagnosticSubject {
    pub(in crate::workbench) project: ProjectId,
    pub(in crate::workbench) cell_view: Option<CellViewRef>,
    pub(in crate::workbench) object: String,
}

impl DiagnosticSubject {
    pub(in crate::workbench) fn new(project: ProjectId, object: impl Into<String>) -> Self {
        Self {
            project,
            cell_view: None,
            object: object.into(),
        }
    }

    pub(in crate::workbench) fn in_cell_view(mut self, cell_view: CellViewRef) -> Self {
        self.cell_view = Some(cell_view);
        self
    }
}

/// Where a finding is repaired.
///
/// Every variant addresses an object, not a workspace: that is what makes
/// following a finding land on the thing it is about rather than on whichever
/// document happened to be open. A finding with no route is still a finding —
/// the page renders it inert rather than offering a jump it cannot honour.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::workbench) enum DiagnosticRoute {
    /// A place in the drawing, or the objects a failed run named.
    Anchor(LogAnchor),
    /// A record the netlist editor owns, opened by its canonical identity.
    NetlistDiagnostic(uuid::Uuid),
    /// A source document a code editor owns: the bundle that holds it, its
    /// logical path inside that bundle, and the zero-based line.
    ///
    /// The bundle rather than the language, because the bundle is what names
    /// the editor. A Verilog-A bundle belongs to a cell view and is edited in
    /// the Design workspace showing that view; the automation bundle belongs
    /// to the project and is edited on the Netlist workspace's Automation
    /// page. Carrying the language alone would leave a Verilog-A finding
    /// resolvable only while its own view already happened to be open.
    CodeDocument {
        bundle: crate::state::ProjectSourceId,
        logical_path: Arc<str>,
        line: Option<usize>,
    },
    /// The destination a blocking dispatch finding named for itself.
    Repair(PreflightRemediation),
}

/// What a finding is about, independent of when it was found or how it is
/// worded: the producer, the rule, the document the object is in, and a
/// stable key of the object itself.
///
/// This is what a finding's canonical identity is minted from, so it is also
/// what decides whether a finding in a new pass is the same finding as one in
/// the last. An identity bound to the pass instead — its ordinal, the project
/// revision, the message — was re-minted by every re-check, and fixing one
/// finding handed its identity to the next finding of the same rule.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(in crate::workbench) struct FindingKey {
    pub(in crate::workbench) producer: Arc<str>,
    pub(in crate::workbench) code: Arc<str>,
    pub(in crate::workbench) document: Arc<str>,
    pub(in crate::workbench) object: Arc<str>,
}

impl FindingKey {
    pub(in crate::workbench) fn new(
        producer: impl Into<Arc<str>>,
        code: impl Into<Arc<str>>,
        document: impl Into<Arc<str>>,
        object: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            producer: producer.into(),
            code: code.into(),
            document: document.into(),
            object: object.into(),
        }
    }
}

/// The canonical metadata of every finding one pass reported, in the order it
/// reported them.
///
/// The one place a store record's identity is minted, so a surface that holds
/// the findings rather than the records — the canvas tour holds violations —
/// names them exactly as the records do by asking here. The occurrence index
/// counts only findings whose keys are identical, in the order they were
/// reported: it separates two findings about one object, and nothing else
/// about the pass can move an identity.
pub(in crate::workbench) fn mint_canonical(
    keys: &[FindingKey],
    revision: u64,
    validation_id: &str,
) -> Vec<CanonicalDiagnosticMetadata> {
    let mut occurrences: HashMap<&FindingKey, usize> = HashMap::with_capacity(keys.len());
    keys.iter()
        .map(|key| {
            let occurrence = occurrences.entry(key).or_insert(0);
            let canonical = CanonicalDiagnosticMetadata::structural(
                Arc::clone(&key.producer),
                Arc::clone(&key.code),
                Arc::clone(&key.document),
                &key.object,
                *occurrence,
                revision,
                validation_id,
            );
            *occurrence += 1;
            canonical
        })
        .collect()
}

/// One condition, in the shape every consumer reads it in.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct DiagnosticRecord {
    pub(in crate::workbench) canonical: CanonicalDiagnosticMetadata,
    /// What the finding is about, as its identity was minted from it. A
    /// republication that re-mints the current finding under a new
    /// occurrence finds it again by this.
    pub(in crate::workbench) key: FindingKey,
    pub(in crate::workbench) level: DiagnosticLevel,
    pub(in crate::workbench) subject: DiagnosticSubject,
    pub(in crate::workbench) message: String,
    /// What else the reader needs, shown on hover. Empty when the message is
    /// the whole finding.
    pub(in crate::workbench) details: String,
    /// What the rule itself requires, in the words the rule is written in.
    ///
    /// Not the message and not the fix: the message says what this instance
    /// is, the details say what to do about it, and this says what the rule
    /// demands of every design. Empty where the producer has no rule prose —
    /// a run that failed to converge is not an instance of a rule — and a
    /// surface that offers to explain a finding must offer nothing where this
    /// is empty rather than restate the message under a different heading.
    pub(in crate::workbench) rule: String,
    pub(in crate::workbench) route: Option<DiagnosticRoute>,
}

impl DiagnosticRecord {
    pub(in crate::workbench) fn new(
        canonical: CanonicalDiagnosticMetadata,
        level: DiagnosticLevel,
        subject: DiagnosticSubject,
        message: impl Into<String>,
    ) -> Self {
        // A record built from metadata alone is keyed by the document it is
        // in; a structural producer states the object with `keyed`.
        let key = FindingKey::new(
            Arc::clone(&canonical.source),
            Arc::clone(&canonical.code),
            Arc::clone(&canonical.document_id),
            "",
        );
        Self {
            canonical,
            key,
            level,
            subject,
            message: message.into(),
            details: String::new(),
            rule: String::new(),
            route: None,
        }
    }

    /// State the key the record's identity was minted from.
    pub(in crate::workbench) fn keyed(mut self, key: FindingKey) -> Self {
        self.key = key;
        self
    }

    pub(in crate::workbench) fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = details.into();
        self
    }

    /// State the rule this finding is an instance of, in the rule's own words.
    pub(in crate::workbench) fn with_rule(mut self, rule: impl Into<String>) -> Self {
        self.rule = rule.into();
        self
    }

    pub(in crate::workbench) fn with_route(mut self, route: Option<DiagnosticRoute>) -> Self {
        self.route = route;
        self
    }

    /// Whether a waiver covers this finding.
    ///
    /// A waived finding is a decision someone recorded, not a condition that
    /// went away, so it stays in the store and stays on the page — it just
    /// stops counting against the design. The decision itself rides on the
    /// canonical record's suppression, which is the field the rule deck
    /// writes, so there is exactly one place a waiver can be stated.
    pub(in crate::workbench) const fn waived(&self) -> bool {
        self.canonical.suppression.is_some()
    }
}

/// The owner of one currency rule. Not the producer identity a record states:
/// several producers can share a batch when they share the inputs their
/// findings were derived from, and `canonical.source` is what says which one
/// reported a given record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::workbench) enum DiagnosticOwner {
    /// The schematic and electrical rule checks, per cell view.
    DesignRules,
    /// One symbol's pin contract against the interface its cell declares,
    /// per symbol view.
    ///
    /// Not [`Self::DesignRules`]: those findings are about a schematic's
    /// electrical rules and are keyed to a cell view that has a schematic,
    /// while these are about the pins drawn on a symbol and stand or fall
    /// with that symbol's own artwork.
    SymbolPins,
    /// Everything a run has to be true before it can be dispatched: the
    /// plan, the technology, the model bindings, the hierarchy, and the
    /// netlist the deck is generated from. The design rules it checks are
    /// published under [`Self::DesignRules`], for the configured root.
    Dispatch,
    /// One failed run, attributed to the objects the engine named.
    Run,
}

impl DiagnosticOwner {
    const fn key_prefix(self) -> &'static str {
        match self {
            Self::DesignRules => "design-rules",
            Self::SymbolPins => "symbol-pins",
            Self::Dispatch => "dispatch",
            Self::Run => "run",
        }
    }
}

/// What a batch was derived from.
///
/// One rule, one place: a batch is current while the inputs it names still
/// digest to what they digested when it was published. Adding a producer
/// means adding the inputs it reads, and the compiler then insists the digest
/// is defined for them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum DiagnosticInputs {
    /// The complete live schematic working set behind one cell view's checks,
    /// exactly as the check receipt digests it.
    DesignCheck { subject: CellViewRef },
    /// One symbol view's published artwork and the interface contract its
    /// pins are compared against — exactly what the pin checks read.
    SymbolPins { subject: CellViewRef },
    /// The configured topology closure, the active simulation plan, and the
    /// preflight report the findings came from.
    Dispatch,
    /// One retained run, and the drawing its cross-probe names resolve
    /// against.
    Run { run_sequence: u64 },
}

impl DiagnosticInputs {
    pub(in crate::workbench) const fn owner(&self) -> DiagnosticOwner {
        match self {
            Self::DesignCheck { .. } => DiagnosticOwner::DesignRules,
            Self::SymbolPins { .. } => DiagnosticOwner::SymbolPins,
            Self::Dispatch => DiagnosticOwner::Dispatch,
            Self::Run { .. } => DiagnosticOwner::Run,
        }
    }

    /// What this batch was derived from, in words a reader can check against
    /// the design. For a stale batch this is exactly what moved under it.
    pub(in crate::workbench) fn description(&self) -> String {
        match self {
            Self::DesignCheck { subject } => format!(
                "the schematic working set behind {}",
                subject.display_path()
            ),
            Self::SymbolPins { subject } => format!(
                "the artwork of {} and the interface contract its pins are checked against",
                subject.display_path()
            ),
            Self::Dispatch => {
                "the configured design, the active simulation plan and the preflight report"
                    .to_owned()
            }
            Self::Run { run_sequence } => {
                format!("run {run_sequence} and the drawing its names resolve against")
            }
        }
    }

    fn key(&self) -> String {
        let owner = self.owner().key_prefix();
        match self {
            Self::DesignCheck { subject } | Self::SymbolPins { subject } => {
                format!("{owner}\u{1f}{}", subject.key())
            }
            Self::Dispatch => owner.to_owned(),
            Self::Run { run_sequence } => format!("{owner}\u{1f}{run_sequence}"),
        }
    }

    /// The digest of these inputs as they stand now.
    ///
    /// `Err` is not "stale": it is "cannot be evaluated", which the reader is
    /// told in the producer's own words rather than having findings silently
    /// dropped or silently kept.
    fn digest(&self, state: &AppState) -> Result<ContentDigest, String> {
        match self {
            Self::DesignCheck { subject } => {
                super::design_checks::live_input_digest(state, subject)
            }
            // The artwork and the contract, through the one accessor the
            // checks themselves read them through: a digest taken over
            // anything else would be a second notion of staleness.
            Self::SymbolPins { subject } => digest_of(&(
                state.workspace.content.project.id(),
                subject,
                state.symbol_pin_check_inputs(subject)?,
            )),
            Self::Dispatch => {
                let (root, revision, closure) = state.configured_topology_revision();
                digest_of(&(
                    state.workspace.content.project.id(),
                    state.workspace.content.project.revision().get(),
                    root,
                    revision,
                    closure,
                    state.active_plan_revision(),
                    // The report these findings were published from. Every
                    // edit to a simulation input — plan, models, technology,
                    // bindings, solver options — invalidates it at its own
                    // commit boundary, and so does dispatching the run it
                    // authorized. Once it is gone its findings describe a
                    // pass nobody can act on, so they read stale with it.
                    state.workbench.preflight.report.is_some(),
                ))
            }
            Self::Run { run_sequence } => {
                let retained = state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .any(|run| run.id == *run_sequence);
                // A failure names conductors and devices; an edited design is
                // no longer the one those names were read off. The design is
                // the configured one, as the dispatch checks digest it — not
                // whichever view the reader happens to have open, which
                // staled a failure on opening another view and un-staled it
                // on coming back.
                let (root, revision, closure) = state.configured_topology_revision();
                digest_of(&(
                    state.workspace.content.project.id(),
                    run_sequence,
                    retained,
                    root,
                    revision,
                    closure,
                ))
            }
        }
    }
}

fn digest_of(material: &impl serde::Serialize) -> Result<ContentDigest, String> {
    let encoded = serde_json::to_vec(material)
        .map_err(|error| format!("could not encode diagnostic inputs: {error}"))?;
    let mut digest = Sha256::new();
    digest.update(DIAGNOSTIC_INPUT_DIGEST_DOMAIN);
    digest.update(encoded);
    Ok(ContentDigest::from_bytes(digest.finalize().into()))
}

/// Everything one producer published in one pass, with the digest that decides
/// whether it still describes the design in front of the reader.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct DiagnosticBatch {
    pub(in crate::workbench) inputs: DiagnosticInputs,
    input_digest: ContentDigest,
    pub(in crate::workbench) records: Vec<DiagnosticRecord>,
}

/// Where a published batch stands against the live design.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum BatchStanding {
    Current,
    /// The inputs moved under it. Its findings usually still point at
    /// something real, so they are kept and said to be stale.
    Stale,
    /// The inputs cannot be read at all, in the producer's own words.
    Unavailable(String),
}

impl BatchStanding {
    pub(in crate::workbench) const fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }
}

/// The canonical diagnostic store.
///
/// Runtime-only, and deliberately so: a finding is evidence about a live
/// design, and evidence that outlives the session that produced it is a claim
/// nobody checked.
#[derive(Debug, Clone, Default)]
pub(crate) struct DiagnosticStore {
    batches: BTreeMap<String, DiagnosticBatch>,
    /// Which batch owns each record, keyed by canonical `diagnostic_id`.
    ///
    /// This is what makes the identity mean something: a producer that mints
    /// the same identity for two conditions is refused rather than quietly
    /// shadowing one with the other, which is the same rule the netlist and
    /// code collections already hold themselves to.
    owners: BTreeMap<uuid::Uuid, String>,
    /// A route a consumer asked to follow and the frame loop has not applied.
    ///
    /// The page that asks holds `&mut AppState`; applying a route runs
    /// commands, which is the application root's business. Recording the
    /// request keeps the page from reaching up for the dispatcher.
    pending_route: Option<DiagnosticRoute>,
    /// The one finding every surface treats as current, by canonical identity.
    ///
    /// The Problems grid's current row and the canvas finding tour are the
    /// same selection. They used to be two: the page had no selection at all
    /// and the tour kept an index into a list it sorted for itself, so
    /// "finding 3 of 7" named a different condition from the row a reader was
    /// looking at, and nothing could carry a selection from one surface to the
    /// other.
    ///
    /// An identity rather than an index, because every ordering here is a
    /// presentation choice: the grid sorts, filters and groups, the tour walks
    /// worst-severity-first, and an index means something different in each.
    /// A finding that leaves the store stops being current because nothing
    /// resolves it any more, which is what the index could not do when a
    /// batch was republished under it.
    current: Option<uuid::Uuid>,
    /// What the current finding is about, captured while a record of it was
    /// in reach.
    ///
    /// Identities are structural, so a re-check names an unchanged finding
    /// exactly as before and this is not consulted. It is what carries the
    /// selection across the republications that do re-mint: an occurrence
    /// index that moved because an identical finding before it was fixed,
    /// and a run outcome superseded by the next run's outcome for the same
    /// analysis.
    current_key: Option<FindingKey>,
    /// Moves on every publication and withdrawal, so a memo taken over the
    /// store's batches can tell it is describing a different store.
    generation: u64,
    /// What the current rendering pass decided about the list.
    pass_memo: PassMemoCell,
}

/// The list's standings and counts for one rendering pass.
///
/// Behind a lock because every surface that reads the list holds the session
/// immutably while it draws. A clone starts empty: a memo describes the store
/// it was taken from, not a copy of it.
#[derive(Default)]
struct PassMemoCell(std::sync::Mutex<Option<listing::PassMemo>>);

impl Clone for PassMemoCell {
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl std::fmt::Debug for PassMemoCell {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("PassMemoCell")
    }
}

impl DiagnosticStore {
    pub(in crate::workbench) fn batches(
        &self,
    ) -> impl Iterator<Item = &DiagnosticBatch> + Clone + '_ {
        self.batches.values()
    }

    const fn generation(&self) -> u64 {
        self.generation
    }

    /// The memo the given pass took over this generation of the store, if
    /// one was taken.
    fn pass_memo(&self, pass: u64, generation: u64) -> Option<listing::PassMemo> {
        let memo = self
            .pass_memo
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        memo.as_ref()
            .filter(|memo| memo.pass == pass && memo.generation == generation)
            .cloned()
    }

    fn remember_pass(&self, memo: listing::PassMemo) {
        *self
            .pass_memo
            .0
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner) = Some(memo);
    }

    pub(in crate::workbench) fn request_route(&mut self, route: DiagnosticRoute) {
        self.pending_route = Some(route);
    }

    pub(in crate::workbench) fn take_pending_route(&mut self) -> Option<DiagnosticRoute> {
        self.pending_route.take()
    }

    /// The finding the grid and the canvas tour are both standing on.
    ///
    /// Reachable from the whole crate because the canvas is: the finding tour
    /// lives with the drawing it moves, outside the shell, and the point of
    /// one selection is that both ends of it can read it.
    pub(crate) const fn current_finding(&self) -> Option<uuid::Uuid> {
        self.current
    }

    /// Stand on a finding, by the identity its producer minted for it.
    ///
    /// Accepting an identity the store does not hold is deliberate: the
    /// netlist and code editors own their own collections and the store never
    /// sees those records, so a selection in one of them would be refused by
    /// any check made here. What makes a selection meaningful is that
    /// something resolves it, which every reader of it does by looking the
    /// identity up in the list it is drawing.
    pub(crate) fn set_current_finding(&mut self, finding: Option<uuid::Uuid>) {
        if finding == self.current {
            return;
        }
        self.current = finding;
        self.current_key = finding.and_then(|id| self.record(id)).map(|record| record.key.clone());
    }

    /// The record a held identity names, where this store holds it.
    fn record(&self, id: uuid::Uuid) -> Option<&DiagnosticRecord> {
        let batch = self.batches.get(self.owners.get(&id)?)?;
        batch
            .records
            .iter()
            .find(|record| record.canonical.diagnostic_id == id)
    }

    /// Remember what the current finding is about while a record of it is
    /// still held, before a republication or a withdrawal takes it away.
    fn capture_current_key(&mut self) {
        if let Some(record) = self.current.and_then(|id| self.record(id)) {
            self.current_key = Some(record.key.clone());
        }
    }

    /// Stand on the same finding again after a republication re-minted it.
    ///
    /// Only when the held identity no longer resolves: a finding still in
    /// the store is still the current one, whatever else moved.
    fn reanchor_current(&mut self) {
        let (Some(current), Some(key)) = (self.current, self.current_key.as_ref()) else {
            return;
        };
        if self.owners.contains_key(&current) {
            return;
        }
        let found = self
            .batches
            .values()
            .flat_map(|batch| batch.records.iter())
            .find(|record| record.key == *key)
            .map(|record| record.canonical.diagnostic_id);
        if found.is_some() {
            self.current = found;
        }
    }

    pub(in crate::workbench) fn clear(&mut self) {
        self.batches.clear();
        self.owners.clear();
        self.pending_route = None;
        self.current = None;
        self.current_key = None;
        self.generation = self.generation.wrapping_add(1);
    }

    /// Replace a batch, and with it every identity the old one held.
    ///
    /// An empty batch is still evidence: it is what says the checks ran and
    /// found nothing, which the page states differently from "nobody has
    /// looked".
    fn insert(
        &mut self,
        digest: ContentDigest,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
    ) -> Result<(), String> {
        self.capture_current_key();
        let key = inputs.key();
        let replaced = self.batches.remove(&key);
        if let Some(replaced) = &replaced {
            for record in &replaced.records {
                self.owners.remove(&record.canonical.diagnostic_id);
            }
        }
        // One pass answers both questions: an identity already claimed is
        // either another batch's or one this batch minted twice.
        let mut claimed = Vec::with_capacity(records.len());
        for record in &records {
            let id = record.canonical.diagnostic_id;
            if self.owners.contains_key(&id) {
                // Put back what was taken out, so a refused publication
                // leaves the store exactly as it found it.
                for id in claimed {
                    self.owners.remove(&id);
                }
                if let Some(replaced) = replaced {
                    for record in &replaced.records {
                        self.owners
                            .insert(record.canonical.diagnostic_id, key.clone());
                    }
                    self.batches.insert(key, replaced);
                }
                return Err(format!(
                    "Diagnostic collection contains duplicate canonical ID {id}."
                ));
            }
            self.owners.insert(id, key.clone());
            claimed.push(id);
        }
        self.batches.insert(
            key,
            DiagnosticBatch {
                inputs,
                input_digest: digest,
                records,
            },
        );
        self.generation = self.generation.wrapping_add(1);
        self.reanchor_current();
        Ok(())
    }

    fn remove(&mut self, inputs: &DiagnosticInputs) {
        self.capture_current_key();
        if let Some(removed) = self.batches.remove(&inputs.key()) {
            self.generation = self.generation.wrapping_add(1);
            for record in &removed.records {
                self.owners.remove(&record.canonical.diagnostic_id);
            }
        }
    }
}

impl AppState {
    /// Publish everything one producer found in one pass.
    ///
    /// The digest is taken here, from the live state, which is what makes the
    /// currency rule one rule: a producer cannot publish evidence and declare
    /// it current against inputs it never read.
    pub(in crate::workbench) fn publish_diagnostics(
        &mut self,
        inputs: DiagnosticInputs,
        records: Vec<DiagnosticRecord>,
    ) -> Result<(), String> {
        let digest = inputs.digest(self)?;
        self.diagnostics.insert(digest, inputs, records)
    }

    /// Withdraw a batch, for a producer whose evidence no longer exists at
    /// all — a closed project, a retired run, a cell view that was deleted.
    pub(in crate::workbench) fn withdraw_diagnostics(&mut self, inputs: &DiagnosticInputs) {
        self.diagnostics.remove(inputs);
    }

    /// Withdraw the run outcomes a newly dispatched run supersedes.
    ///
    /// A run is a new measurement of the design as it now stands, so the
    /// failures an earlier run reported stop standing the moment the next one
    /// is dispatched — the way a simulator's run log starts over. Waiting for
    /// the new run to finish instead would leave the earlier failure current
    /// beside the new run's own failure of the same analysis, and both cannot
    /// stand: they are one condition with one identity. A run that is
    /// cancelled before it reports anything does not bring the earlier
    /// failure back; that failure is still in the Console and in the earlier
    /// run's own record.
    ///
    /// Members of one campaign do not supersede each other. They are
    /// dispatched one after another, but each measures its own configuration,
    /// and a campaign's outcome is all of them.
    pub(crate) fn supersede_run_diagnostics(&mut self, dispatched: u64) {
        let campaign = self.run_campaign(dispatched);
        let superseded: Vec<DiagnosticInputs> = self
            .diagnostics
            .batches()
            .filter(|batch| match batch.inputs {
                DiagnosticInputs::Run { run_sequence } => {
                    run_sequence != dispatched
                        && (campaign.is_none() || self.run_campaign(run_sequence) != campaign)
                }
                DiagnosticInputs::DesignCheck { .. }
                | DiagnosticInputs::SymbolPins { .. }
                | DiagnosticInputs::Dispatch => false,
            })
            .map(|batch| batch.inputs.clone())
            .collect();
        for inputs in &superseded {
            self.withdraw_diagnostics(inputs);
        }
        self.withdraw_retired_run_diagnostics();
    }

    /// Withdraw the outcomes of every run this session no longer retains.
    ///
    /// A pruned, deleted or cleared run has no outcome left to state: its
    /// dataset is gone and its failure cannot be opened. Removing a run is
    /// spread over the simulation state, so this is asked at the frame
    /// boundary as well as wherever the shell itself retires a run.
    pub(crate) fn withdraw_retired_run_diagnostics(&mut self) {
        let retired: Vec<DiagnosticInputs> = self
            .diagnostics
            .batches()
            .filter(|batch| match batch.inputs {
                DiagnosticInputs::Run { run_sequence } => self
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .all(|run| run.id != run_sequence),
                DiagnosticInputs::DesignCheck { .. }
                | DiagnosticInputs::SymbolPins { .. }
                | DiagnosticInputs::Dispatch => false,
            })
            .map(|batch| batch.inputs.clone())
            .collect();
        for inputs in &retired {
            self.withdraw_diagnostics(inputs);
        }
    }

    /// The campaign a retained run belongs to, if it belongs to one.
    fn run_campaign(&self, run_sequence: u64) -> Option<crate::product::SimulationCampaignId> {
        self.simulation
            .retained
            .run_by_sequence(run_sequence)
            .and_then(|run| run.campaign_membership())
            .map(|membership| membership.campaign_id())
    }

    /// Where one published batch stands against the live design.
    pub(in crate::workbench) fn diagnostic_batch_standing(
        &self,
        batch: &DiagnosticBatch,
    ) -> BatchStanding {
        match batch.inputs.digest(self) {
            Ok(digest) if digest == batch.input_digest => BatchStanding::Current,
            Ok(_) => BatchStanding::Stale,
            Err(reason) => BatchStanding::Unavailable(reason),
        }
    }
}

/// The stable rule code for a schematic check finding.
///
/// `FloatingNode` becomes `DRC-FLOATING-NODE`. Derived from the variant rather
/// than authored beside it so a new rule cannot ship without a code, and
/// shaped so the rule deck can address it as one word.
pub(in crate::workbench) fn design_rule_code(
    violation_type: rspice_design::drc::DrcViolationType,
) -> String {
    format!("DRC-{}", screaming_variant(violation_type))
}

/// The stable rule code for a symbol pin finding.
///
/// `SymbolUnplacedPin` becomes `SYMBOL-UNPLACED-PIN`. Derived from the variant
/// exactly as [`design_rule_code`] is, and without its `DRC-` family: a
/// symbol's pin contract is not one of a schematic's electrical rules, and a
/// reader filtering the page by code is entitled to that distinction.
pub(in crate::workbench) fn symbol_pin_rule_code(
    violation_type: rspice_design::drc::DrcViolationType,
) -> String {
    screaming_variant(violation_type)
}

/// `FloatingNode` becomes `FLOATING-NODE`.
///
/// One derivation for every family, so two codes for one rule cannot drift
/// apart on the word the rule is named by.
fn screaming_variant(violation_type: rspice_design::drc::DrcViolationType) -> String {
    let variant = format!("{violation_type:?}");
    let mut code = String::with_capacity(variant.len() + 8);
    for (index, character) in variant.char_indices() {
        if character.is_ascii_uppercase() && index > 0 {
            code.push('-');
        }
        code.push(character.to_ascii_uppercase());
    }
    code
}

#[cfg(test)]
mod tests {
    use super::*;

    fn completed_result() -> rspice_design::drc::DrcResult {
        let mut result = rspice_design::drc::DrcResult::new();
        result.completed = true;
        result
    }

    #[test]
    fn a_rule_code_is_derived_from_its_variant() {
        use rspice_design::drc::DrcViolationType;
        assert_eq!(
            design_rule_code(DrcViolationType::FloatingNode),
            "DRC-FLOATING-NODE"
        );
        assert_eq!(
            design_rule_code(DrcViolationType::ShortedOutputs),
            "DRC-SHORTED-OUTPUTS"
        );
        assert_eq!(
            symbol_pin_rule_code(DrcViolationType::SymbolUnplacedPin),
            "SYMBOL-UNPLACED-PIN",
            "a symbol's pin contract is not one of a schematic's rules",
        );
    }

    /// Currency is the digest and nothing else, for every producer.
    #[test]
    fn an_input_change_stales_a_published_batch() {
        let mut state = AppState::default();
        state
            .publish_active_design_check_result(completed_result())
            .expect("the fixture publishes a receipt");
        let inputs = DiagnosticInputs::DesignCheck {
            subject: state.workspace.content.active_schematic_reference(),
        };
        state
            .publish_diagnostics(inputs.clone(), Vec::new())
            .expect("an empty batch still records that the checks ran");

        let batch = state
            .diagnostics
            .batches()
            .find(|batch| batch.inputs == inputs)
            .expect("the batch was just published")
            .clone();
        assert!(state.diagnostic_batch_standing(&batch).is_current());

        state.workspace.content.connectivity.policy.width_mismatch =
            crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

        assert_eq!(
            state.diagnostic_batch_standing(&batch),
            BatchStanding::Stale,
        );
    }

    fn test_record(state: &AppState, message: &str) -> DiagnosticRecord {
        DiagnosticRecord::new(
            CanonicalDiagnosticMetadata::current(
                "rspice.test",
                "TEST-1",
                "doc",
                None,
                0,
                "validation",
                message,
            ),
            DiagnosticLevel::Error,
            DiagnosticSubject::new(state.workspace.content.project.id(), "Node 'n1'"),
            message,
        )
    }

    /// A republication takes back the identities the batch it replaces held,
    /// or the second publication of the same finding would collide with the
    /// first and be refused.
    #[test]
    fn republishing_a_batch_releases_the_identities_it_replaces() {
        let mut state = AppState::default();
        let record = test_record(&state, "first");
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record.clone()])
            .expect("the dispatch digest is available");
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, vec![record])
            .expect("republishing the same finding is not a collision");
        assert_eq!(state.diagnostics.batches().count(), 1);
    }

    fn run_batches(state: &AppState) -> Vec<u64> {
        state
            .diagnostics
            .batches()
            .filter_map(|batch| match batch.inputs {
                DiagnosticInputs::Run { run_sequence } => Some(run_sequence),
                DiagnosticInputs::DesignCheck { .. }
                | DiagnosticInputs::SymbolPins { .. }
                | DiagnosticInputs::Dispatch => None,
            })
            .collect()
    }

    /// The next run supersedes the last one's outcome. Nothing withdrew a
    /// failed run's batch before, so after a re-run that succeeded the status
    /// bar went on saying "Problems · 1 error", current, forever.
    #[test]
    fn a_dispatched_run_supersedes_the_failures_earlier_runs_reported() {
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![
            crate::state::SimulationRun::new(1),
            crate::state::SimulationRun::new(2),
        ]
        .into();
        state.publish_run_failure("Transient", "Analysis failed: tran", None, 1);
        assert_eq!(run_batches(&state), [1]);
        let failed = state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .map(|record| record.canonical.diagnostic_id)
            .next()
            .expect("the failure is published");
        state.diagnostics.set_current_finding(Some(failed));

        state.supersede_run_diagnostics(2);
        assert!(
            run_batches(&state).is_empty(),
            "the earlier run's failure stops standing when the next run is dispatched"
        );

        // The next run fails the same way: the same condition, standing
        // again, under the same identity — and still the current finding.
        state.publish_run_failure("Transient", "Analysis failed: tran again", None, 2);
        assert_eq!(run_batches(&state), [2]);
        assert_eq!(state.diagnostics.current_finding(), Some(failed));
        let republished = state
            .diagnostics
            .batches()
            .flat_map(|batch| batch.records.iter())
            .map(|record| record.canonical.diagnostic_id)
            .next();
        assert_eq!(republished, Some(failed));
    }

    /// A run with nothing retained has no outcome left to state.
    #[test]
    fn a_retired_run_takes_its_failure_with_it() {
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![crate::state::SimulationRun::new(4)].into();
        state.publish_run_failure("Transient", "Analysis failed: tran", None, 4);
        state.withdraw_retired_run_diagnostics();
        assert_eq!(run_batches(&state), [4], "a retained run's failure stands");

        state.simulation.clear_runs();
        state.withdraw_retired_run_diagnostics();
        assert!(run_batches(&state).is_empty());
    }

    /// A run failure names conductors in the configured design. Opening
    /// another cell view moves nothing it was read off.
    #[test]
    fn a_run_failure_stays_current_while_another_view_is_open() {
        let mut state = AppState::default();
        state.simulation.retained.runs = vec![crate::state::SimulationRun::new(5)].into();
        state.publish_run_failure("Transient", "Analysis failed: tran", None, 5);
        let batch = state
            .diagnostics
            .batches()
            .next()
            .expect("the failure is published")
            .clone();
        assert!(state.diagnostic_batch_standing(&batch).is_current());

        let root = state.workspace.content.active_view.clone();
        state
            .workspace
            .insert_schematic_editor(root.key(), state.schematic.clone());
        state.workspace.content.active_view =
            crate::state::CellViewRef::new(&root.library, "elsewhere", &root.view);
        state.schematic = crate::state::SchematicState::default();
        state.schematic.add_component(
            crate::state::ComponentType::Resistor,
            crate::state::Point::new(0, 0),
        );

        assert!(
            state.diagnostic_batch_standing(&batch).is_current(),
            "the open view is not the design the run executed"
        );
    }

    /// Two conditions that mint one identity would shadow each other, so the
    /// publication is refused instead and the store is left as it was.
    #[test]
    fn a_producer_that_mints_one_identity_twice_is_refused() {
        let mut state = AppState::default();
        let kept = test_record(&state, "kept");
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, vec![kept])
            .expect("the dispatch digest is available");

        let colliding = test_record(&state, "collides");
        let error = state
            .publish_diagnostics(
                DiagnosticInputs::Dispatch,
                vec![colliding.clone(), colliding],
            )
            .expect_err("two records with one identity must be refused");

        assert!(error.contains("duplicate canonical ID"), "{error}");
        let batch = state
            .diagnostics
            .batches()
            .next()
            .expect("the refused publication leaves the previous batch in place");
        assert_eq!(batch.records.len(), 1);
        assert_eq!(batch.records[0].message, "kept");
    }
}
