//! `workbench/app_state/diagnostic_store/listing` observed through a whole session.

use crate::state::code_diagnostics::CodeDiagnosticCollection;
use crate::state::netlist_diagnostics::NetlistDiagnosticCollection;
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::BatchStanding;
use crate::workbench::app_state::diagnostic_store::DiagnosticBatch;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::app_state::diagnostic_store::listing::*;
use crate::workbench::app_state::diagnostic_store::{
    DiagnosticInputs, DiagnosticLevel, DiagnosticRecord, DiagnosticSubject,
};
use rspice_vocabulary::diagnostics::CanonicalDiagnosticMetadata;
use std::sync::Arc;

fn open_state() -> AppState {
    let mut state = AppState::default();
    state.open_project_session();
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
        DiagnosticSubject::new(state.model.workspace.content.project.id(), "Node 'n1'"),
        format!("{code} stands"),
    )
}

#[test]
fn a_closed_project_has_no_conditions_to_report() {
    let mut state = AppState::default();
    state.close_project_session();
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

/// A check that found nothing in a design that has since been edited
/// says nothing about the design in front of the reader, so the page
/// does not call that design clean until the check runs again.
#[test]
fn an_empty_batch_whose_inputs_moved_reads_as_out_of_date() {
    let mut state = open_state();
    let subject = state.model.workspace.content.active_schematic_reference();
    let inputs = DiagnosticInputs::DesignCheck { subject };
    state
        .publish_diagnostics(inputs.clone(), Vec::new())
        .expect("the design-check digest is available");
    assert_eq!(*listing(&state).empty_reason(), EmptyReason::NoFindings);

    state
        .model
        .workspace
        .content
        .connectivity
        .policy
        .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

    let listed = listing(&state);
    assert_eq!(listed.len(), 0);
    assert_eq!(*listed.empty_reason(), EmptyReason::OutOfDate);
    assert_eq!(
        *listing_in_pass(&state, 3).empty_reason(),
        EmptyReason::OutOfDate,
        "the memoized list states the same reason",
    );

    state
        .publish_diagnostics(inputs, Vec::new())
        .expect("the design-check digest is available");
    assert_eq!(
        *listing(&state).empty_reason(),
        EmptyReason::NoFindings,
        "a check run over the design as it stands is evidence again",
    );
}

/// One stale empty result is enough to withhold "no findings", and a
/// producer that cannot run at all is said before any of it.
#[test]
fn an_empty_list_states_the_weakest_evidence_behind_it() {
    let state = open_state();
    let empty = |inputs| DiagnosticBatch {
        inputs,
        input_digest: crate::product::ContentDigest::from_bytes([0; 32]),
        records: Vec::new(),
        fingerprint: 0,
    };
    let dispatch = empty(DiagnosticInputs::Dispatch);
    let run = empty(DiagnosticInputs::Run { run_sequence: 4 });
    let design = empty(DiagnosticInputs::DesignCheck {
        subject: state.model.workspace.content.active_schematic_reference(),
    });
    let stored = |batch, standing| Segment::Stored { batch, standing };

    let (_, reason) = tally(&[
        stored(&dispatch, BatchStanding::Current),
        stored(&run, BatchStanding::Stale),
    ]);
    assert_eq!(reason, EmptyReason::OutOfDate);

    let (_, reason) = tally(&[
        stored(&dispatch, BatchStanding::Current),
        stored(&run, BatchStanding::Stale),
        stored(
            &design,
            BatchStanding::Unavailable("the inputs cannot be read".to_owned()),
        ),
    ]);
    assert_eq!(
        reason,
        EmptyReason::Unavailable("the inputs cannot be read".to_owned())
    );

    let (_, reason) = tally(&[
        stored(&dispatch, BatchStanding::Current),
        stored(&run, BatchStanding::Current),
    ]);
    assert_eq!(reason, EmptyReason::NoFindings);
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
    waived.canonical.suppression = Some(rspice_vocabulary::diagnostics::DiagnosticSuppression {
        rule: "T-ERROR".to_owned(),
        reason: "accepted at review".to_owned(),
        author: "ana".to_owned(),
        waived_at_ms: 1,
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
    assert_eq!(waiver.reason, "accepted at review");
}

/// A stale finding keeps its row and loses its jump.
#[test]
fn a_stale_batch_offers_no_route() {
    let mut state = open_state();
    let subject = state.model.workspace.content.active_schematic_reference();
    let anchored = record(&state, "T-ERROR", DiagnosticLevel::Error).with_route(Some(
        DiagnosticRoute::Anchor(crate::diagnostics::LogAnchor::Schematic {
            cell_view: subject.clone(),
            x: 0,
            y: 0,
            component: None,
            wire: None,
            sheet: None,
        }),
    ));
    let inputs = DiagnosticInputs::DesignCheck {
        subject: subject.clone(),
    };
    state
        .publish_diagnostics(inputs, vec![anchored])
        .expect("the design-check digest is available");
    assert!(listing(&state).row(0).expect("listed").route.is_some());

    state
        .model
        .workspace
        .content
        .connectivity
        .policy
        .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;

    let listing = listing(&state);
    assert_eq!(listing.len(), 1);
    assert_eq!(listing.counts().stale, 1);
    let row = listing.row(0).expect("a stale finding keeps its row");
    assert!(!row.current());
    assert!(row.route.is_none());
    assert_eq!(
        row.standing,
        RowStanding::Stale(format!(
            "a cellview or binding the check of {} read has changed since this was reported",
            subject.display_path()
        )),
        "a stale row says in one clause what moved under it, not merely that it is stale",
    );
}

/// A rule finding's subject is the object; the cell view it is drawn in is
/// the location. Before the grid had both columns the subject carried the
/// cell view as well, so every row of one view repeated its path.
#[test]
fn a_rule_finding_separates_its_object_from_the_view_it_is_drawn_in() {
    let mut state = open_state();
    let reference = state.model.workspace.content.active_schematic_reference();
    let mut record = record(&state, "T-ERROR", DiagnosticLevel::Error);
    record.subject = DiagnosticSubject::new(
        state.model.workspace.content.project.id(),
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

fn waiver() -> rspice_vocabulary::diagnostics::DiagnosticSuppression {
    rspice_vocabulary::diagnostics::DiagnosticSuppression {
        rule: "TEST".to_owned(),
        reason: "accepted at review".to_owned(),
        author: "ana".to_owned(),
        waived_at_ms: 1,
    }
}

/// The netlist and code collections are counted by the rule their rows
/// are drawn by. Their producers' summaries counted a waived finding as
/// an error and a stale code finding as current, so the badge claimed
/// findings the rows did not.
#[test]
fn source_collections_count_what_their_rows_say() {
    use crate::state::netlist_diagnostics::Diagnostic;
    use rspice_vocabulary::diagnostics::DiagnosticSeverity as NetlistSeverity;
    use rspice_vocabulary::diagnostics::{
        DiagnosticCurrentness, DiagnosticSeverity, SourceDiagnostic,
    };

    let mut state = open_state();
    let mut waived = Diagnostic::current("rspice.test", "T-WAIVED", NetlistSeverity::Error, "a");
    waived.canonical.suppression = Some(waiver());
    let mut stale = Diagnostic::current("rspice.test", "T-STALE", NetlistSeverity::Error, "b");
    stale
        .canonical
        .mark_currentness(DiagnosticCurrentness::StaleSource);
    let standing = Diagnostic::current("rspice.test", "T-ERROR", NetlistSeverity::Error, "c");
    state.code.netlist.diagnostics = Arc::new(
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
    state.code.code_workspace.automation.diagnostics = Arc::new(
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
    use rspice_vocabulary::diagnostics::{
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
        source_current: true,
    };

    let current = segment_row(&segment, 0).expect("the current finding is a row");
    assert!(matches!(
        current.route,
        Some(DiagnosticRoute::CodeDocument { line: Some(3), .. })
    ));
    let stale = segment_row(&segment, 1).expect("the stale finding is a row");
    assert_eq!(stale.route, None);
}

/// The netlist editor marks its findings stale only while it draws. A
/// finding parsed from an older buffer than the one the editor holds is
/// stale on the page whether or not the editor has been drawn since.
#[test]
fn a_netlist_finding_older_than_the_buffer_reads_stale() {
    use crate::state::netlist_diagnostics::Diagnostic;

    let mut state = open_state();
    let finding = Diagnostic::error("unknown model nchh").with_line(Some(0));
    state.code.netlist.revision = finding.canonical.revision;
    state.code.netlist.diagnostics = Arc::new(
        NetlistDiagnosticCollection::try_new(vec![finding], "nchh\n")
            .expect("the fixture collection is well formed"),
    );
    let listed = listing(&state);
    assert_eq!(listed.counts().errors, 1);
    assert!(listed.row(0).expect("listed").current());

    // An edit made with the editor off screen: nothing re-parsed.
    state.code.netlist.revision = state.code.netlist.revision.wrapping_add(1);

    let listed = listing(&state);
    assert_eq!(listed.counts().errors, 0);
    assert_eq!(listed.counts().stale, 1);
    let row = listed.row(0).expect("a stale finding keeps its row");
    assert!(!row.current());
    assert!(row.route.is_none(), "a stale finding offers no jump");
    assert!(!listed.facet(0).expect("and its facet").current);
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
        fingerprint: 0,
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
    let bundle = state
        .model
        .workspace
        .active_source_bundle(crate::state::ProjectSourceLanguage::RSpiceAutomation)
        .expect("a project holds its automation bundle");
    let (bundle_id, root) = (bundle.id(), bundle.root().logical_path().to_owned());
    let root_content = bundle.root().content().to_owned();
    state.code.code_workspace.automation.receipt = Some(AutomationValidationReceipt {
        token: SourceOperationToken {
            project_id: state.model.workspace.content.project.id(),
            revision: bundle.revision().get(),
            content_digest: bundle.closure_digest(),
        },
    });

    let listed = listing(&state);
    assert_eq!(listed.len(), 0);
    assert_eq!(*listed.empty_reason(), EmptyReason::NoFindings);

    // An edit the page did not see: the clean validation is of older
    // source than the bundle holds now.
    state
        .model
        .workspace
        .content
        .project_sources
        .replace_bundle_file_content(bundle_id, &root, format!("{root_content}\n"))
        .expect("the root accepts an added line");
    assert_eq!(*listing(&state).empty_reason(), EmptyReason::OutOfDate);
}

/// A netlist that parsed clean is evidence the deck was looked at, and
/// stops being evidence about the deck once the buffer moves past the
/// revision it was parsed at. The editor re-parses only while it draws,
/// so an edit made with it off screen leaves exactly that.
#[test]
fn a_clean_netlist_validation_goes_out_of_date_with_its_buffer() {
    let mut state = open_state();
    assert_eq!(*listing(&state).empty_reason(), EmptyReason::NothingRan);

    let revision = state.code.netlist.revision;
    state.code.netlist.mark_validated_for_test(revision);
    let listed = listing(&state);
    assert_eq!(listed.len(), 0);
    assert_eq!(*listed.empty_reason(), EmptyReason::NoFindings);

    state.code.netlist.revision = revision.wrapping_add(1);
    assert_eq!(*listing(&state).empty_reason(), EmptyReason::OutOfDate);
    assert_eq!(
        *listing_in_pass(&state, 5).empty_reason(),
        EmptyReason::OutOfDate,
        "the memoized list states the same reason",
    );
}

/// Every reader in one pass reads one decision about every batch, and a
/// publication in the middle of the pass is never read with the standing
/// of the batch it replaced.
#[test]
fn one_pass_decides_each_standing_once_and_a_publication_invalidates_it() {
    let mut state = open_state();
    let subject = state.model.workspace.content.active_schematic_reference();
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
    state
        .model
        .workspace
        .content
        .connectivity
        .policy
        .width_mismatch = crate::state::BundleWidthMismatchPolicy::ExplicitSliceOrExtend;
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
