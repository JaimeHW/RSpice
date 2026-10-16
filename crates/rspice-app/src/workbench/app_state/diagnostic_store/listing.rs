//! Every standing condition, as one addressable list.
//!
//! The Problems page, the tab badge and the status bar all state the same
//! thing, so they read the same list. Three surfaces each deriving their own
//! count from four producers is how the badge came to claim findings the body
//! under it did not list.
//!
//! The list is built from segment descriptors rather than from a vector of
//! rows: the netlist and code editors publish collections sized for a million
//! records, and a page that materialized them would allocate a row per record
//! sixty times a second. The counts are tallied from what each record says
//! about itself — its standing, its waiver, its severity — by the rule the
//! rows are drawn by, because a producer's own summary counts waived and
//! stale records as if they stood. A row is built only when it is about to be
//! drawn.

use std::sync::Arc;

use rspice_app_types::diagnostics::{
    CanonicalQuickFix, DiagnosticCurrentness, DiagnosticRelatedLocation, DiagnosticSuppression,
};

use super::{BatchStanding, DiagnosticBatch, DiagnosticLevel, DiagnosticRoute};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::CodeDiagnosticCollection;
use crate::workbench::documents::netlist_document::NetlistDiagnosticCollection;

/// Where one finding stands against the design in front of the reader.
///
/// The faint rows on the page used to say only "not current", which left the
/// reader to guess whether the finding had been superseded, whether the
/// drawing had moved under it, or whether the checks had failed to run at
/// all. Each is a different thing to do next, so each says which.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum RowStanding {
    Current,
    /// The inputs this finding was derived from have moved. The words say
    /// which, in the terms the producer measured them in.
    Stale(String),
    /// The inputs cannot be read at all, in the producer's own words.
    Unavailable(String),
}

impl RowStanding {
    pub(in crate::workbench) const fn is_current(&self) -> bool {
        matches!(self, Self::Current)
    }
}

/// One row, assembled when it is drawn.
pub(in crate::workbench) struct DiagnosticRow<'a> {
    /// The canonical identity its producer minted for it: what a selection,
    /// a quick fix and the canvas tour all name it by.
    pub(in crate::workbench) id: uuid::Uuid,
    /// The rule this finding is an instance of, which is how a reader names
    /// it when they go looking for why it fired.
    pub(in crate::workbench) code: &'a str,
    /// Who reported it, by the producer identity the record carries. Stated
    /// in full: a truncated identity is one a reader cannot check.
    pub(in crate::workbench) producer: &'a str,
    pub(in crate::workbench) level: DiagnosticLevel,
    /// Whether this finding describes the design as it stands, and what moved
    /// where it does not.
    pub(in crate::workbench) standing: RowStanding,
    /// What the finding is about: the object a rule named, or the document a
    /// parser or compiler was reading.
    pub(in crate::workbench) subject: String,
    /// Where that thing is — the cell view it is drawn in, or the position in
    /// the document the producer was reading. Empty where a finding is about
    /// the project rather than about a place in it.
    pub(in crate::workbench) location: String,
    pub(in crate::workbench) message: &'a str,
    /// What else the reader needs, for the row's hover.
    pub(in crate::workbench) details: &'a str,
    /// What the rule requires, in its own words. Empty where the producer has
    /// no rule prose to state.
    pub(in crate::workbench) rule: &'a str,
    /// The other places this finding names.
    pub(in crate::workbench) related: &'a [DiagnosticRelatedLocation],
    /// The edits the producer offered, where it offered any.
    pub(in crate::workbench) quick_fixes: &'a [CanonicalQuickFix],
    /// The waiver that covers it, with the scope and justification the rule
    /// deck recorded. A waived finding is still listed; it just stops
    /// counting against the design.
    pub(in crate::workbench) suppression: Option<&'a DiagnosticSuppression>,
    /// Where it is repaired, when it names somewhere.
    pub(in crate::workbench) route: Option<DiagnosticRoute>,
}

impl DiagnosticRow<'_> {
    pub(in crate::workbench) const fn current(&self) -> bool {
        self.standing.is_current()
    }

    pub(in crate::workbench) const fn waived(&self) -> bool {
        self.suppression.is_some()
    }
}

/// Everything about a row that can be read without building it.
///
/// Filtering, counting and ordering a release-scale list must not allocate a
/// string per row per frame, and these are the fields that decide all three.
/// A text filter is the one thing that cannot be answered from here, because
/// matching the subject means spelling it — so that, and only that, builds
/// rows.
#[derive(Debug, Clone, Copy)]
pub(in crate::workbench) struct RowFacet<'a> {
    pub(in crate::workbench) id: uuid::Uuid,
    pub(in crate::workbench) code: &'a str,
    pub(in crate::workbench) producer: &'a str,
    pub(in crate::workbench) level: DiagnosticLevel,
    pub(in crate::workbench) current: bool,
    pub(in crate::workbench) waived: bool,
}

/// Why the list has nothing in it, which is never one answer.
///
/// "Nothing is wrong" and "nobody has looked" are different claims, and a
/// tool that prints the first when it means the second is lying.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::workbench) enum EmptyReason {
    /// No project is open, so there is no design to have conditions about.
    NoProject,
    /// Nothing has produced evidence for this design yet.
    NothingRan,
    /// A producer cannot be evaluated at all, in its own words.
    Unavailable(String),
    /// Everything that ran found nothing.
    NoFindings,
}

enum Segment<'a> {
    /// A batch the store owns, with where it stands against the live design.
    Stored {
        batch: &'a DiagnosticBatch,
        standing: BatchStanding,
    },
    /// The netlist editor's own canonical collection, read in place.
    Netlist(&'a NetlistDiagnosticCollection),
    /// One code editor's canonical collection, read in place, and the source
    /// bundle whose editor opens it.
    Code {
        bundle: Option<crate::state::ProjectSourceId>,
        collection: &'a CodeDiagnosticCollection,
    },
}

impl Segment<'_> {
    fn len(&self) -> usize {
        match self {
            Self::Stored { batch, .. } => batch.records.len(),
            Self::Netlist(collection) => collection.len(),
            Self::Code { collection, .. } => collection.len(),
        }
    }
}

/// The counts behind every badge and status segment, derived from exactly the
/// rows the list holds, under the rule the rows are drawn by: a current
/// finding counts as an error or an advisory unless a waiver covers it, and
/// a finding that is not current counts as what it is instead.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::workbench) struct DiagnosticCounts {
    pub(in crate::workbench) errors: usize,
    pub(in crate::workbench) advisories: usize,
    /// The advisories that are warnings, rather than notes or hints.
    pub(in crate::workbench) warnings: usize,
    /// Findings about a design that has since been edited. Counted on their
    /// own: a stale finding is neither an error nor an advisory about the
    /// design in front of the reader.
    pub(in crate::workbench) stale: usize,
    /// Findings whose inputs cannot be read at all, so whether they still
    /// stand cannot be said. Not stale: nothing is known to have moved.
    pub(in crate::workbench) unavailable: usize,
}

impl DiagnosticCounts {
    fn tally(&mut self, standing: Tally, waived: bool, level: DiagnosticLevel) {
        match (standing, waived) {
            (Tally::Stale, _) => self.stale += 1,
            (Tally::Unavailable, _) => self.unavailable += 1,
            (Tally::Current, true) => {}
            (Tally::Current, false) if level.is_error() => self.errors += 1,
            (Tally::Current, false) => {
                self.advisories += 1;
                if level == DiagnosticLevel::Warning {
                    self.warnings += 1;
                }
            }
        }
    }
}

/// Where one finding stands, for counting it.
#[derive(Debug, Clone, Copy)]
enum Tally {
    Current,
    Stale,
    Unavailable,
}

impl Tally {
    const fn of_batch(standing: &BatchStanding) -> Self {
        match standing {
            BatchStanding::Current => Self::Current,
            BatchStanding::Stale => Self::Stale,
            BatchStanding::Unavailable(_) => Self::Unavailable,
        }
    }

    const fn of_record(current: bool) -> Self {
        if current { Self::Current } else { Self::Stale }
    }
}

pub(in crate::workbench) struct DiagnosticListing<'a> {
    segments: Vec<Segment<'a>>,
    counts: DiagnosticCounts,
    empty_reason: EmptyReason,
}

impl<'a> DiagnosticListing<'a> {
    pub(in crate::workbench) fn len(&self) -> usize {
        self.segments.iter().map(Segment::len).sum()
    }

    pub(in crate::workbench) const fn counts(&self) -> DiagnosticCounts {
        self.counts
    }

    /// Why there is nothing to list. Meaningful only when [`Self::len`] is
    /// zero; a non-empty list reports [`EmptyReason::NoFindings`].
    pub(in crate::workbench) const fn empty_reason(&self) -> &EmptyReason {
        &self.empty_reason
    }

    /// The reason a producer cannot be evaluated, where one said so, for a
    /// list that still has other rows to show.
    pub(in crate::workbench) fn unavailable(&self) -> Option<&str> {
        self.segments.iter().find_map(|segment| match segment {
            Segment::Stored {
                standing: BatchStanding::Unavailable(reason),
                ..
            } => Some(reason.as_str()),
            Segment::Stored { .. } | Segment::Netlist(_) | Segment::Code { .. } => None,
        })
    }

    /// Whether anything in the list is about a design that has since changed.
    pub(in crate::workbench) const fn has_stale(&self) -> bool {
        self.counts.stale > 0
    }

    pub(in crate::workbench) fn row(&self, index: usize) -> Option<DiagnosticRow<'a>> {
        let (segment, index) = self.locate(index)?;
        segment_row(segment, index)
    }

    /// What a row is, without spelling it out. See [`RowFacet`].
    pub(in crate::workbench) fn facet(&self, index: usize) -> Option<RowFacet<'a>> {
        let (segment, index) = self.locate(index)?;
        segment_facet(segment, index)
    }

    fn locate(&self, index: usize) -> Option<(&Segment<'a>, usize)> {
        let mut remaining = index;
        for segment in &self.segments {
            let length = segment.len();
            if remaining < length {
                return Some((segment, remaining));
            }
            remaining -= length;
        }
        None
    }
}

/// Where a finding from a source editor's own collection stands.
///
/// The editors decide currency per record rather than per batch, so the
/// reason is the record's own currentness rather than a digest comparison.
fn source_standing(currentness: DiagnosticCurrentness) -> RowStanding {
    match currentness {
        DiagnosticCurrentness::Current => RowStanding::Current,
        DiagnosticCurrentness::StaleSource => {
            RowStanding::Stale("the source has been edited since this was reported".to_owned())
        }
        DiagnosticCurrentness::StaleEnvironment => {
            RowStanding::Stale("the environment this was compiled in has changed since".to_owned())
        }
        DiagnosticCurrentness::StalePermissions => RowStanding::Stale(
            "the source is no longer readable under these permissions".to_owned(),
        ),
        DiagnosticCurrentness::SupersededValidation => {
            RowStanding::Stale("a later validation of this source has superseded it".to_owned())
        }
    }
}

fn segment_row<'a>(segment: &Segment<'a>, index: usize) -> Option<DiagnosticRow<'a>> {
    match segment {
        Segment::Stored { batch, standing } => {
            let record = batch.records.get(index)?;
            let current = standing.is_current();
            Some(DiagnosticRow {
                id: record.canonical.diagnostic_id,
                code: record.canonical.code.as_ref(),
                producer: record.canonical.source.as_ref(),
                level: record.level,
                standing: match standing {
                    BatchStanding::Current => RowStanding::Current,
                    BatchStanding::Stale => RowStanding::Stale(batch.inputs.description()),
                    BatchStanding::Unavailable(reason) => RowStanding::Unavailable(reason.clone()),
                },
                subject: stored_subject(record),
                location: record
                    .subject
                    .cell_view
                    .as_ref()
                    .map(|reference| reference.display_path())
                    .unwrap_or_default(),
                message: record.message.as_str(),
                details: record.details.as_str(),
                rule: record.rule.as_str(),
                related: record.canonical.related_locations.as_slice(),
                quick_fixes: record.canonical.quick_fixes.as_slice(),
                suppression: record.canonical.suppression.as_ref(),
                // A route captured when the finding was made addresses an
                // object that may have gone since. Offering it from a stale
                // batch would be offering a jump the finding cannot honour.
                route: current.then(|| record.route.clone()).flatten(),
            })
        }
        Segment::Netlist(collection) => {
            let diagnostic = collection.get(index)?;
            let current = diagnostic.is_current();
            Some(DiagnosticRow {
                id: diagnostic.canonical.diagnostic_id,
                code: diagnostic.canonical.code.as_ref(),
                producer: diagnostic.canonical.source.as_ref(),
                level: diagnostic.severity.into(),
                standing: source_standing(diagnostic.canonical.currentness),
                subject: netlist_subject(diagnostic),
                location: netlist_location(diagnostic),
                message: diagnostic.message.as_str(),
                details: diagnostic.details.as_str(),
                rule: "",
                related: diagnostic.canonical.related_locations.as_slice(),
                quick_fixes: diagnostic.canonical.quick_fixes.as_slice(),
                suppression: diagnostic.canonical.suppression.as_ref(),
                route: current.then_some(DiagnosticRoute::NetlistDiagnostic(
                    diagnostic.canonical.diagnostic_id,
                )),
            })
        }
        Segment::Code { bundle, collection } => {
            let diagnostic = collection.as_slice().get(index)?;
            Some(DiagnosticRow {
                id: diagnostic.canonical.diagnostic_id,
                code: diagnostic.canonical.code.as_ref(),
                producer: diagnostic.canonical.source.as_ref(),
                level: diagnostic.severity.into(),
                standing: source_standing(diagnostic.canonical.currentness),
                subject: diagnostic
                    .source_path
                    .as_deref()
                    .unwrap_or("workspace source")
                    .to_owned(),
                location: diagnostic
                    .line
                    .map(|line| format!("line {}", line + 1))
                    .unwrap_or_default(),
                message: diagnostic.message.as_ref(),
                details: diagnostic.detail.as_ref(),
                rule: "",
                related: diagnostic.canonical.related_locations.as_slice(),
                quick_fixes: diagnostic.canonical.quick_fixes.as_slice(),
                suppression: diagnostic.canonical.suppression.as_ref(),
                // A finding about source that has since moved names a line
                // the source may no longer have, so it offers no jump, like a
                // stale finding in every other segment.
                route: bundle
                    .filter(|_| diagnostic.canonical.is_current())
                    .zip(diagnostic.source_path.as_ref())
                    .map(|(bundle, path)| DiagnosticRoute::CodeDocument {
                        bundle,
                        logical_path: Arc::clone(path),
                        line: diagnostic.line,
                    }),
            })
        }
    }
}

fn segment_facet<'a>(segment: &Segment<'a>, index: usize) -> Option<RowFacet<'a>> {
    match segment {
        Segment::Stored { batch, standing } => {
            let record = batch.records.get(index)?;
            Some(RowFacet {
                id: record.canonical.diagnostic_id,
                code: record.canonical.code.as_ref(),
                producer: record.canonical.source.as_ref(),
                level: record.level,
                current: standing.is_current(),
                waived: record.waived(),
            })
        }
        Segment::Netlist(collection) => {
            let diagnostic = collection.get(index)?;
            Some(RowFacet {
                id: diagnostic.canonical.diagnostic_id,
                code: diagnostic.canonical.code.as_ref(),
                producer: diagnostic.canonical.source.as_ref(),
                level: diagnostic.severity.into(),
                current: diagnostic.is_current(),
                waived: diagnostic.canonical.suppression.is_some(),
            })
        }
        Segment::Code { collection, .. } => {
            let diagnostic = collection.as_slice().get(index)?;
            Some(RowFacet {
                id: diagnostic.canonical.diagnostic_id,
                code: diagnostic.canonical.code.as_ref(),
                producer: diagnostic.canonical.source.as_ref(),
                level: diagnostic.severity.into(),
                current: diagnostic.canonical.is_current(),
                waived: diagnostic.canonical.suppression.is_some(),
            })
        }
    }
}

/// What a rule finding is about: the object the producer named, without the
/// cell view that the row's own location column states.
fn stored_subject(record: &super::DiagnosticRecord) -> String {
    let object = record.subject.object.as_str();
    let Some(reference) = record.subject.cell_view.as_ref() else {
        return object.to_owned();
    };
    let path = reference.display_path();
    match object.strip_prefix(path.as_str()) {
        Some(remainder) if !remainder.trim().is_empty() => remainder.trim_start().to_owned(),
        Some(_) | None => object.to_owned(),
    }
}

/// What a netlist finding is about: the document the parser was reading. The
/// position inside it is the row's location.
fn netlist_subject(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    diagnostic.source_path.as_deref().map_or_else(
        || "netlist editor".to_owned(),
        |path| path.display().to_string(),
    )
}

/// Where in that document it is. An included source keeps its own line rather
/// than masquerading as a line of whatever file is open, which is why the
/// physical source line is read when the buffer has none.
fn netlist_location(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    let line = diagnostic.line.or(diagnostic.source_line);
    match (line, diagnostic.column) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => String::new(),
    }
}

/// Assemble the list for the design in front of the reader.
///
/// Every batch's standing is decided here, from the live design, which is
/// exact and is not free: a design-check digest encodes and hashes the
/// working set it covers. A surface drawn every frame reads
/// [`listing_in_pass`] instead, which decides them once per pass.
pub(in crate::workbench) fn listing(state: &AppState) -> DiagnosticListing<'_> {
    if !state.project_lifecycle.is_open() {
        return closed_listing();
    }
    let segments = segments(state, |batch| state.diagnostic_batch_standing(batch));
    let (counts, empty_reason) = tally(&segments);
    DiagnosticListing {
        segments,
        counts,
        empty_reason,
    }
}

/// The same list as [`listing`], with every standing and count decided at
/// most once per rendering pass.
///
/// Four surfaces state the list each frame — the tab badge, the page, and
/// the status bar's text and mark — and each used to decide every batch's
/// standing again, re-digesting the working set behind every checked view
/// and walking the configured hierarchy for the dispatch findings, four
/// times a frame. The first reader in a pass decides them, and every reader
/// after it in that pass reads that answer.
///
/// The memo is keyed by the pass and by the store's own generation, which
/// moves on every publication and withdrawal, so a batch published
/// mid-pass is never read with another batch's standing. Within one pass an
/// edit to the design is not reflected until the next pass, which every
/// edit requests; that is the price, and a frame that agrees with itself is
/// what it buys. Across passes nothing is reused, because nothing in the
/// session counts every change to every input a digest covers, and a memo
/// whose key missed one would be a staleness bug.
pub(in crate::workbench) fn listing_in_pass(state: &AppState, pass: u64) -> DiagnosticListing<'_> {
    if !state.project_lifecycle.is_open() {
        return closed_listing();
    }
    let generation = state.diagnostics.generation();
    if let Some(memo) = state.diagnostics.pass_memo(pass, generation) {
        let segments = segments(state, |batch| {
            memo.standings
                .get(&batch.inputs.key())
                .cloned()
                .unwrap_or_else(|| state.diagnostic_batch_standing(batch))
        });
        return DiagnosticListing {
            segments,
            counts: memo.counts,
            empty_reason: memo.empty_reason,
        };
    }
    let listed = listing(state);
    state.diagnostics.remember_pass(PassMemo {
        pass,
        generation,
        standings: listed
            .segments
            .iter()
            .filter_map(|segment| match segment {
                Segment::Stored { batch, standing } => Some((batch.inputs.key(), standing.clone())),
                Segment::Netlist(_) | Segment::Code { .. } => None,
            })
            .collect(),
        counts: listed.counts,
        empty_reason: listed.empty_reason.clone(),
    });
    listed
}

/// What one rendering pass decided about the list, for the readers after
/// the first in that pass.
#[derive(Debug, Clone)]
pub(in crate::workbench) struct PassMemo {
    pub(super) pass: u64,
    pub(super) generation: u64,
    standings: std::collections::BTreeMap<String, BatchStanding>,
    counts: DiagnosticCounts,
    empty_reason: EmptyReason,
}

fn closed_listing<'a>() -> DiagnosticListing<'a> {
    DiagnosticListing {
        segments: Vec::new(),
        counts: DiagnosticCounts::default(),
        empty_reason: EmptyReason::NoProject,
    }
}

/// Every segment of the list, with each stored batch's standing as
/// `standing_of` decides it.
fn segments<'a>(
    state: &'a AppState,
    mut standing_of: impl FnMut(&DiagnosticBatch) -> BatchStanding,
) -> Vec<Segment<'a>> {
    let mut segments = Vec::new();
    let mut stored: Vec<&DiagnosticBatch> = state.diagnostics.batches().collect();
    stored.sort_by_key(|batch| batch.inputs.owner());
    for batch in stored {
        let standing = standing_of(batch);
        segments.push(Segment::Stored { batch, standing });
    }

    // A validation that ran and found nothing is still a validation that
    // ran: an empty collection the editor holds a receipt for is a segment,
    // so the page says "No findings" rather than "Nothing has been checked".
    // The netlist editor publishes no such receipt outside its own module,
    // so a clean parse there cannot be told from no parse.
    let netlist = state.ui.netlist.diagnostics.as_ref();
    if !netlist.is_empty() {
        segments.push(Segment::Netlist(netlist));
    }
    for (bundle, collection) in code_collections(state) {
        segments.push(Segment::Code { bundle, collection });
    }
    segments
}

/// The counts the list states, and why it is empty when it is — tallied from
/// exactly what each row says about itself.
fn tally(segments: &[Segment<'_>]) -> (DiagnosticCounts, EmptyReason) {
    let mut counts = DiagnosticCounts::default();
    let mut unavailable = None;
    for segment in segments {
        match segment {
            Segment::Stored { batch, standing } => {
                if let BatchStanding::Unavailable(reason) = standing
                    && unavailable.is_none()
                {
                    unavailable = Some(reason.clone());
                }
                let standing = Tally::of_batch(standing);
                for record in &batch.records {
                    counts.tally(standing, record.waived(), record.level);
                }
            }
            Segment::Netlist(collection) => {
                for diagnostic in collection.iter() {
                    counts.tally(
                        Tally::of_record(diagnostic.is_current()),
                        diagnostic.canonical.suppression.is_some(),
                        diagnostic.severity.into(),
                    );
                }
            }
            Segment::Code { collection, .. } => {
                for diagnostic in collection.iter() {
                    counts.tally(
                        Tally::of_record(diagnostic.canonical.is_current()),
                        diagnostic.canonical.suppression.is_some(),
                        diagnostic.severity.into(),
                    );
                }
            }
        }
    }
    let empty_reason = match unavailable {
        Some(reason) if segments.iter().all(|segment| segment.len() == 0) => {
            EmptyReason::Unavailable(reason)
        }
        _ if segments.is_empty() => EmptyReason::NothingRan,
        _ => EmptyReason::NoFindings,
    };
    (counts, empty_reason)
}

/// The canonical collections the code editors publish, each with the source
/// bundle its findings are about.
///
/// The Verilog-A page keeps two: the diagnostics of the last compile that
/// produced a receipt, and the ones from the last compile that failed before
/// it could. Both describe the current source — the page clears them when the
/// source moves — so both stand, and each names its own bundle through the
/// token the compile was authorized under rather than through whichever
/// bundle happens to be selected now.
///
/// A collection is listed when it holds findings, or when the compile or
/// validation that produced it is on record as having run: a clean compile is
/// evidence that the source was looked at. A failure collection that is empty
/// is no failure, and says nothing.
fn code_collections(
    state: &AppState,
) -> Vec<(
    Option<crate::state::ProjectSourceId>,
    &CodeDiagnosticCollection,
)> {
    let code = &state.ui.code_workspace;
    let mut collections: Vec<(Option<crate::state::ProjectSourceId>, _)> = Vec::with_capacity(3);
    if let Some(receipt) = code.veriloga.receipt.as_ref() {
        collections.push((
            Some(receipt.compilation.token().bundle_id),
            receipt.diagnostics.as_ref(),
        ));
    }
    if !code.veriloga.last_failure.is_empty() {
        collections.push((
            code.veriloga
                .last_failure_token
                .as_ref()
                .map(|token| token.bundle_id),
            code.veriloga.last_failure.as_ref(),
        ));
    }
    if !code.automation.diagnostics.is_empty() || code.automation.receipt.is_some() {
        collections.push((
            crate::workbench::documents::code_workspace::active_source_bundle(
                &state.workspace,
                crate::state::ProjectSourceLanguage::RSpiceAutomation,
            )
            .map(crate::state::ProjectSourceBundle::id),
            code.automation.diagnostics.as_ref(),
        ));
    }
    collections
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app_state::diagnostic_store::{
        DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticSubject,
    };
    use rspice_app_types::diagnostics::CanonicalDiagnosticMetadata;

    fn open_state() -> AppState {
        let mut state = AppState::default();
        state.project_lifecycle.authority.open_session();
        state
    }

    fn record(state: &AppState, code: &str, level: DiagnosticLevel) -> DiagnosticRecord {
        DiagnosticRecord::new(
            CanonicalDiagnosticMetadata::current(
                "rspice.test",
                code,
                "doc",
                None,
                0,
                "validation",
                code,
            ),
            level,
            DiagnosticSubject::new(state.workspace.content.project.id(), "Node 'n1'"),
            format!("{code} stands"),
        )
    }

    #[test]
    fn a_closed_project_has_no_conditions_to_report() {
        let mut state = AppState::default();
        state.project_lifecycle.authority.close_project();
        let listed = listing(&state);
        assert_eq!(listed.len(), 0);
        assert_eq!(*listed.empty_reason(), EmptyReason::NoProject);
    }

    #[test]
    fn nothing_published_reads_as_nothing_having_looked() {
        let state = open_state();
        let listing = listing(&state);
        assert_eq!(listing.len(), 0);
        assert_eq!(*listing.empty_reason(), EmptyReason::NothingRan);
    }

    #[test]
    fn an_empty_batch_reads_as_no_findings_rather_than_nothing_having_looked() {
        let mut state = open_state();
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, Vec::new())
            .expect("the dispatch digest is available");
        let listing = listing(&state);
        assert_eq!(listing.len(), 0);
        assert_eq!(*listing.empty_reason(), EmptyReason::NoFindings);
    }

    /// The counts and the rows come from the same list, so they cannot
    /// disagree — which is the whole reason the list exists.
    #[test]
    fn the_counts_describe_exactly_the_rows_listed() {
        let mut state = open_state();
        let records = vec![
            record(&state, "T-ERROR", DiagnosticLevel::Error),
            record(&state, "T-CRITICAL", DiagnosticLevel::Critical),
            record(&state, "T-WARNING", DiagnosticLevel::Warning),
            record(&state, "T-INFO", DiagnosticLevel::Info),
        ];
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, records)
            .expect("the dispatch digest is available");

        let listing = listing(&state);
        assert_eq!(listing.len(), 4);
        let counts = listing.counts();
        assert_eq!(counts.errors, 2);
        assert_eq!(counts.advisories, 2);
        assert_eq!(counts.stale, 0);
        let codes: Vec<&str> = (0..listing.len())
            .filter_map(|index| listing.row(index))
            .map(|row| row.code)
            .collect();
        assert_eq!(
            codes,
            ["T-ERROR", "T-CRITICAL", "T-WARNING", "T-INFO"],
            "rows keep the order their producer reported them in",
        );
    }

    #[test]
    fn a_waived_finding_is_listed_without_counting_against_the_design() {
        let mut state = open_state();
        let mut waived = record(&state, "T-ERROR", DiagnosticLevel::Error);
        waived.canonical.suppression = Some(rspice_app_types::diagnostics::DiagnosticSuppression {
            rule: "T-ERROR".to_owned(),
            scope: "cell".to_owned(),
            justification_required: false,
        });
        state
            .publish_diagnostics(DiagnosticInputs::Dispatch, vec![waived])
            .expect("the dispatch digest is available");

        let listing = listing(&state);
        assert_eq!(listing.len(), 1, "a waiver is a decision, not a deletion");
        assert_eq!(listing.counts(), DiagnosticCounts::default());
        assert!(listing.row(0).expect("the row is listed").waived());
        let waiver = listing
            .row(0)
            .expect("the row is listed")
            .suppression
            .expect("the waiver is the decision someone recorded")
            .clone();
        assert_eq!(waiver.rule, "T-ERROR");
        assert_eq!(waiver.scope, "cell");
    }

    /// A stale finding keeps its row and loses its jump.
    #[test]
    fn a_stale_batch_offers_no_route() {
        let mut state = open_state();
        let subject = state.workspace.content.active_schematic_reference();
        let anchored = record(&state, "T-ERROR", DiagnosticLevel::Error).with_route(Some(
            DiagnosticRoute::Anchor(crate::diagnostics::LogAnchor::Schematic {
                cell_view: subject.clone(),
                x: 0,
                y: 0,
                component: None,
                wire: None,
            }),
        ));
        let inputs = DiagnosticInputs::DesignCheck {
            subject: subject.clone(),
        };
        state
            .publish_diagnostics(inputs, vec![anchored])
            .expect("the design-check digest is available");
        assert!(listing(&state).row(0).expect("listed").route.is_some());

        state.workspace.content.connectivity.policy.width_mismatch =
            crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

        let listing = listing(&state);
        assert_eq!(listing.len(), 1);
        assert_eq!(listing.counts().stale, 1);
        let row = listing.row(0).expect("a stale finding keeps its row");
        assert!(!row.current());
        assert!(row.route.is_none());
        assert_eq!(
            row.standing,
            RowStanding::Stale(format!(
                "the schematic working set behind {}",
                subject.display_path()
            )),
            "a stale row says what moved under it, not merely that it is stale",
        );
    }

    /// A rule finding's subject is the object; the cell view it is drawn in is
    /// the location. Before the grid had both columns the subject carried the
    /// cell view as well, so every row of one view repeated its path.
    #[test]
    fn a_rule_finding_separates_its_object_from_the_view_it_is_drawn_in() {
        let mut state = open_state();
        let reference = state.workspace.content.active_schematic_reference();
        let mut record = record(&state, "T-ERROR", DiagnosticLevel::Error);
        record.subject = DiagnosticSubject::new(
            state.workspace.content.project.id(),
            format!("{} pin A", reference.display_path()),
        )
        .in_cell_view(reference.clone());
        state
            .publish_diagnostics(
                DiagnosticInputs::DesignCheck {
                    subject: reference.clone(),
                },
                vec![record],
            )
            .expect("the design-check digest is available");

        let listing = listing(&state);
        let row = listing.row(0).expect("the finding is listed");
        assert_eq!(row.subject, "pin A");
        assert_eq!(row.location, reference.display_path());
    }

    fn waiver() -> rspice_app_types::diagnostics::DiagnosticSuppression {
        rspice_app_types::diagnostics::DiagnosticSuppression {
            rule: "TEST".to_owned(),
            scope: "project".to_owned(),
            justification_required: false,
        }
    }

    /// The netlist and code collections are counted by the rule their rows
    /// are drawn by. Their producers' summaries counted a waived finding as
    /// an error and a stale code finding as current, so the badge claimed
    /// findings the rows did not.
    #[test]
    fn source_collections_count_what_their_rows_say() {
        use crate::workbench::documents::netlist_document::{
            Diagnostic, DiagnosticSeverity as NetlistSeverity,
        };
        use rspice_app_types::diagnostics::{
            DiagnosticCurrentness, DiagnosticSeverity, SourceDiagnostic,
        };

        let mut state = open_state();
        let mut waived =
            Diagnostic::current("rspice.test", "T-WAIVED", NetlistSeverity::Error, "a");
        waived.canonical.suppression = Some(waiver());
        let mut stale = Diagnostic::current("rspice.test", "T-STALE", NetlistSeverity::Error, "b");
        stale
            .canonical
            .mark_currentness(DiagnosticCurrentness::StaleSource);
        let standing = Diagnostic::current("rspice.test", "T-ERROR", NetlistSeverity::Error, "c");
        state.ui.netlist.diagnostics = Arc::new(
            NetlistDiagnosticCollection::try_new(vec![waived, stale, standing], "")
                .expect("the fixture collection is well formed"),
        );

        let code = |code: &str| {
            SourceDiagnostic::current(
                "rspice.test",
                code,
                DiagnosticSeverity::Warning,
                code,
                "",
                Some("flow.py".to_owned()),
                None,
                None,
                Some(1),
                None,
            )
        };
        let mut stale_code = code("T-STALE-CODE");
        stale_code
            .canonical
            .mark_currentness(DiagnosticCurrentness::StaleSource);
        let mut waived_code = code("T-WAIVED-CODE");
        waived_code.canonical.suppression = Some(waiver());
        state.ui.code_workspace.automation.diagnostics = Arc::new(
            CodeDiagnosticCollection::try_new(vec![stale_code, waived_code, code("T-CODE")])
                .expect("the fixture collection is well formed"),
        );

        let listed = listing(&state);
        assert_eq!(listed.len(), 6, "every finding keeps its row");
        assert_eq!(
            listed.counts(),
            DiagnosticCounts {
                errors: 1,
                advisories: 1,
                warnings: 1,
                stale: 2,
                unavailable: 0,
            }
        );
        let mut from_rows = DiagnosticCounts::default();
        for index in 0..listed.len() {
            let facet = listed.facet(index).expect("every row has a facet");
            from_rows.tally(Tally::of_record(facet.current), facet.waived, facet.level);
        }
        assert_eq!(listed.counts(), from_rows, "the counts are the rows' own");
    }

    /// A code finding about source that has moved since offers no jump to
    /// the line it named, as no stale finding in another segment does.
    #[test]
    fn a_stale_code_finding_offers_no_route() {
        use rspice_app_types::diagnostics::{
            DiagnosticCurrentness, DiagnosticSeverity, SourceDiagnostic,
        };

        let finding = |code: &str| {
            SourceDiagnostic::current(
                "rspice.test",
                code,
                DiagnosticSeverity::Error,
                code,
                "",
                Some("flow.py".to_owned()),
                None,
                None,
                Some(3),
                None,
            )
        };
        let mut moved = finding("T-MOVED");
        moved
            .canonical
            .mark_currentness(DiagnosticCurrentness::StaleSource);
        let collection = CodeDiagnosticCollection::try_new(vec![finding("T-CURRENT"), moved])
            .expect("the fixture collection is well formed");
        let segment = Segment::Code {
            bundle: Some(crate::state::ProjectSourceId::new()),
            collection: &collection,
        };

        let current = segment_row(&segment, 0).expect("the current finding is a row");
        assert!(matches!(
            current.route,
            Some(DiagnosticRoute::CodeDocument { line: Some(3), .. })
        ));
        let stale = segment_row(&segment, 1).expect("the stale finding is a row");
        assert_eq!(stale.route, None);
    }

    /// A batch whose inputs cannot be read is counted as that, not as stale:
    /// nothing is known to have moved, and the row says the checks cannot run.
    #[test]
    fn an_unreadable_batch_counts_as_unavailable_rather_than_stale() {
        let state = open_state();
        let batch = DiagnosticBatch {
            inputs: DiagnosticInputs::Dispatch,
            input_digest: crate::product::ContentDigest::from_bytes([0; 32]),
            records: vec![record(&state, "T-ERROR", DiagnosticLevel::Error)],
        };
        let segments = vec![Segment::Stored {
            batch: &batch,
            standing: BatchStanding::Unavailable("the inputs cannot be read".to_owned()),
        }];

        let (counts, _) = tally(&segments);

        assert_eq!(counts.unavailable, 1);
        assert_eq!(counts.stale, 0);
        assert_eq!(counts.errors, 0);
    }

    /// A validation that ran clean is evidence that the source was looked at,
    /// so the page says there is nothing wrong rather than that nobody looked.
    #[test]
    fn a_clean_validation_reads_as_no_findings() {
        let mut state = open_state();
        assert_eq!(*listing(&state).empty_reason(), EmptyReason::NothingRan);

        use crate::workbench::documents::code_workspace::{
            AutomationValidationReceipt, SourceOperationToken,
        };
        state.ui.code_workspace.automation.receipt = Some(AutomationValidationReceipt {
            token: SourceOperationToken {
                project_id: state.workspace.content.project.id(),
                revision: 1,
                content_digest: crate::product::ContentDigest::from_bytes([7; 32]),
            },
        });

        let listed = listing(&state);
        assert_eq!(listed.len(), 0);
        assert_eq!(*listed.empty_reason(), EmptyReason::NoFindings);
    }

    /// Every reader in one pass reads one decision about every batch, and a
    /// publication in the middle of the pass is never read with the standing
    /// of the batch it replaced.
    #[test]
    fn one_pass_decides_each_standing_once_and_a_publication_invalidates_it() {
        let mut state = open_state();
        let subject = state.workspace.content.active_schematic_reference();
        state
            .publish_diagnostics(
                DiagnosticInputs::DesignCheck {
                    subject: subject.clone(),
                },
                vec![record(&state, "T-ERROR", DiagnosticLevel::Error)],
            )
            .expect("the design-check digest is available");

        assert_eq!(listing_in_pass(&state, 7).counts().errors, 1);
        // Moved under the batch mid-pass: the pass keeps the answer it
        // already gave, and the next pass sees the move.
        state.workspace.content.connectivity.policy.width_mismatch =
            crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
        assert_eq!(listing_in_pass(&state, 7).counts().errors, 1);
        assert_eq!(listing_in_pass(&state, 8).counts().stale, 1);
        assert_eq!(
            listing(&state).counts().stale,
            1,
            "the unmemoized list is exact"
        );

        // A publication moves the store's generation, so the same pass
        // decides again rather than reading the replaced batch's standing.
        state
            .publish_diagnostics(
                DiagnosticInputs::DesignCheck { subject },
                vec![record(&state, "T-ERROR", DiagnosticLevel::Error)],
            )
            .expect("the design-check digest is available");
        assert_eq!(listing_in_pass(&state, 8).counts().errors, 1);
        assert_eq!(listing_in_pass(&state, 8).counts().stale, 0);
    }

    /// Counting and filtering read facets, which spell nothing out. A facet
    /// that disagreed with its row would filter one list and draw another.
    #[test]
    fn a_facet_says_what_its_row_says() {
        let mut state = open_state();
        state
            .publish_diagnostics(
                DiagnosticInputs::Dispatch,
                vec![
                    record(&state, "T-ERROR", DiagnosticLevel::Error),
                    record(&state, "T-INFO", DiagnosticLevel::Info),
                ],
            )
            .expect("the dispatch digest is available");

        let listing = listing(&state);
        for index in 0..listing.len() {
            let row = listing.row(index).expect("the row is listed");
            let facet = listing.facet(index).expect("so is its facet");
            assert_eq!(facet.id, row.id);
            assert_eq!(facet.code, row.code);
            assert_eq!(facet.producer, row.producer);
            assert_eq!(facet.level, row.level);
            assert_eq!(facet.current, row.current());
            assert_eq!(facet.waived, row.waived());
        }
        assert!(listing.facet(listing.len()).is_none());
    }
}
