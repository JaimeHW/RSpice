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

use std::collections::BTreeMap;
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

/// One condition, in the shape every consumer reads it in.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct DiagnosticRecord {
    pub(in crate::workbench) canonical: CanonicalDiagnosticMetadata,
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
        Self {
            canonical,
            level,
            subject,
            message: message.into(),
            details: String::new(),
            rule: String::new(),
            route: None,
        }
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
    /// design checks it re-runs, the plan, the technology, the model
    /// bindings, and the netlist the deck is generated from.
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
    /// The configured topology closure and the active simulation plan.
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
            Self::Dispatch => "the configured design and the active simulation plan".to_owned(),
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
                ))
            }
            Self::Run { run_sequence } => {
                let retained = state
                    .simulation
                    .retained
                    .runs
                    .iter()
                    .any(|run| run.id == *run_sequence);
                digest_of(&(
                    state.workspace.content.project.id(),
                    run_sequence,
                    retained,
                    // A failure names conductors and devices; an edited
                    // drawing is no longer the one those names were read off.
                    state.schematic.topology_version(),
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
    /// It also needs no invalidation — a finding that leaves the store stops
    /// being current because nothing resolves it any more, which is what the
    /// index could not do when a batch was republished under it.
    current: Option<uuid::Uuid>,
}

impl DiagnosticStore {
    pub(in crate::workbench) fn batches(
        &self,
    ) -> impl Iterator<Item = &DiagnosticBatch> + Clone + '_ {
        self.batches.values()
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
        self.current = finding;
    }

    pub(in crate::workbench) fn clear(&mut self) {
        self.batches.clear();
        self.owners.clear();
        self.pending_route = None;
        self.current = None;
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
        Ok(())
    }

    fn remove(&mut self, inputs: &DiagnosticInputs) {
        if let Some(removed) = self.batches.remove(&inputs.key()) {
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
