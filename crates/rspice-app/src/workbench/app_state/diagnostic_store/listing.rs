//! Every standing condition, as one addressable list.
//!
//! The Problems page, the tab badge and the status bar all state the same
//! thing, so they read the same list. Three surfaces each deriving their own
//! count from four producers is how the badge came to claim findings the body
//! under it did not list.
//!
//! The list is built from segment descriptors rather than from a vector of
//! rows: the netlist and code editors publish collections sized for a million
//! records, and a page that materialized them to count them would do it sixty
//! times a second. A segment reports its own counts from the summary its
//! producer already keeps, and a row is built only when it is about to be
//! drawn.

use std::sync::Arc;

use super::{BatchStanding, DiagnosticBatch, DiagnosticLevel, DiagnosticRoute};
use crate::workbench::app_state::AppState;
use crate::workbench::documents::code_workspace::CodeDiagnosticCollection;
use crate::workbench::documents::netlist_document::NetlistDiagnosticCollection;

/// One row, assembled when it is drawn.
pub(in crate::workbench) struct DiagnosticRow<'a> {
    /// The rule this finding is an instance of, which is how a reader names
    /// it when they go looking for why it fired.
    pub(in crate::workbench) code: &'a str,
    pub(in crate::workbench) level: DiagnosticLevel,
    /// Whether this finding describes the design as it stands.
    pub(in crate::workbench) current: bool,
    /// Whether a recorded waiver covers it. A waived finding is still listed;
    /// it just stops counting against the design.
    pub(in crate::workbench) waived: bool,
    /// What the finding is about, in one line.
    pub(in crate::workbench) subject: String,
    pub(in crate::workbench) message: &'a str,
    /// What else the reader needs, for the row's hover.
    pub(in crate::workbench) details: &'a str,
    /// Where it is repaired, when it names somewhere.
    pub(in crate::workbench) route: Option<DiagnosticRoute>,
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
/// rows the list holds.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(in crate::workbench) struct DiagnosticCounts {
    pub(in crate::workbench) errors: usize,
    pub(in crate::workbench) advisories: usize,
    /// Findings about a design that has since been edited. Counted on their
    /// own: a stale finding is neither an error nor an advisory about the
    /// design in front of the reader.
    pub(in crate::workbench) stale: usize,
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
        let mut remaining = index;
        for segment in &self.segments {
            let length = segment.len();
            if remaining < length {
                return segment_row(segment, remaining);
            }
            remaining -= length;
        }
        None
    }
}

fn segment_row<'a>(segment: &Segment<'a>, index: usize) -> Option<DiagnosticRow<'a>> {
    match segment {
        Segment::Stored { batch, standing } => {
            let record = batch.records.get(index)?;
            let current = standing.is_current();
            Some(DiagnosticRow {
                code: record.canonical.code.as_ref(),
                level: record.level,
                current,
                waived: record.waived(),
                subject: record.subject.display(),
                message: record.message.as_str(),
                details: record.details.as_str(),
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
                code: diagnostic.canonical.code.as_ref(),
                level: diagnostic.severity.into(),
                current,
                waived: diagnostic.canonical.suppression.is_some(),
                subject: netlist_subject(diagnostic),
                message: diagnostic.message.as_str(),
                details: diagnostic.details.as_str(),
                route: current.then_some(DiagnosticRoute::NetlistDiagnostic(
                    diagnostic.canonical.diagnostic_id,
                )),
            })
        }
        Segment::Code { bundle, collection } => {
            let diagnostic = collection.as_slice().get(index)?;
            let current = diagnostic.canonical.is_current();
            Some(DiagnosticRow {
                code: diagnostic.canonical.code.as_ref(),
                level: diagnostic.severity.into(),
                current,
                waived: diagnostic.canonical.suppression.is_some(),
                subject: code_subject(diagnostic),
                message: diagnostic.message.as_ref(),
                details: diagnostic.detail.as_ref(),
                route: bundle
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

/// Where a netlist finding is, in the words the deck uses.
fn netlist_subject(
    diagnostic: &crate::workbench::documents::netlist_document::Diagnostic,
) -> String {
    let line = diagnostic.line.or(diagnostic.source_line);
    let location = match (line, diagnostic.column) {
        (Some(line), Some(column)) => format!("line {} · column {}", line + 1, column + 1),
        (Some(line), None) => format!("line {}", line + 1),
        (None, _) => "document scope".to_owned(),
    };
    match diagnostic.source_path.as_deref() {
        Some(path) => format!("{} · {location}", path.display()),
        None => location,
    }
}

/// Where a source finding is. The logical path is the identity the compile
/// receipt used, so a finding in an included header says so rather than
/// pointing at a line of whatever file is open.
fn code_subject(diagnostic: &rspice_app_types::diagnostics::SourceDiagnostic) -> String {
    let path = diagnostic
        .source_path
        .as_deref()
        .unwrap_or("workspace source");
    match diagnostic.line {
        Some(line) => format!("{path} · line {}", line + 1),
        None => path.to_owned(),
    }
}

/// Assemble the list for the design in front of the reader.
pub(in crate::workbench) fn listing(state: &AppState) -> DiagnosticListing<'_> {
    if !state.project_lifecycle.is_open() {
        return DiagnosticListing {
            segments: Vec::new(),
            counts: DiagnosticCounts::default(),
            empty_reason: EmptyReason::NoProject,
        };
    }

    let mut segments = Vec::new();
    let mut counts = DiagnosticCounts::default();
    let mut unavailable = None;

    let mut stored: Vec<&DiagnosticBatch> = state.diagnostics.batches().collect();
    stored.sort_by_key(|batch| batch.inputs.owner());
    for batch in stored {
        let standing = state.diagnostic_batch_standing(batch);
        if let BatchStanding::Unavailable(reason) = &standing
            && unavailable.is_none()
        {
            unavailable = Some(reason.clone());
        }
        for record in &batch.records {
            match (standing.is_current(), record.waived()) {
                (false, _) => counts.stale += 1,
                (true, true) => {}
                (true, false) if record.level.is_error() => counts.errors += 1,
                (true, false) => counts.advisories += 1,
            }
        }
        segments.push(Segment::Stored { batch, standing });
    }

    let netlist = state.ui.netlist.diagnostics.as_ref();
    if !netlist.is_empty() {
        let summary = netlist.summary();
        counts.errors += summary.current_errors;
        counts.advisories += summary.current_advisories();
        counts.stale += summary.stale;
        segments.push(Segment::Netlist(netlist));
    }

    for (bundle, collection) in code_collections(state) {
        let summary = collection.summary();
        counts.errors += summary.errors;
        counts.advisories += summary.warnings + summary.information + summary.hints;
        segments.push(Segment::Code { bundle, collection });
    }

    let empty_reason = match unavailable {
        Some(reason) if segments.iter().all(|segment| segment.len() == 0) => {
            EmptyReason::Unavailable(reason)
        }
        _ if segments.is_empty() => EmptyReason::NothingRan,
        _ => EmptyReason::NoFindings,
    };

    DiagnosticListing {
        segments,
        counts,
        empty_reason,
    }
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
    collections.push((
        code.veriloga
            .last_failure_token
            .as_ref()
            .map(|token| token.bundle_id),
        code.veriloga.last_failure.as_ref(),
    ));
    collections.push((
        crate::workbench::documents::code_workspace::active_source_bundle(
            &state.workspace,
            crate::state::ProjectSourceLanguage::RSpiceAutomation,
        )
        .map(crate::state::ProjectSourceBundle::id),
        code.automation.diagnostics.as_ref(),
    ));
    collections.retain(|(_, collection)| !collection.is_empty());
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
        assert!(listing.row(0).expect("the row is listed").waived);
    }

    /// A stale finding keeps its row and loses its jump.
    #[test]
    fn a_stale_batch_offers_no_route() {
        let mut state = open_state();
        let anchored = record(&state, "T-ERROR", DiagnosticLevel::Error).with_route(Some(
            DiagnosticRoute::Anchor(crate::diagnostics::LogAnchor::Schematic {
                x: 0,
                y: 0,
                component: None,
                wire: None,
            }),
        ));
        let inputs = DiagnosticInputs::DesignCheck {
            subject: state.workspace.content.active_schematic_reference(),
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
        assert!(!row.current);
        assert!(row.route.is_none());
    }
}
