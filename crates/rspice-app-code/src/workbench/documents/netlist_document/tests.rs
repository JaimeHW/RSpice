//! The open deck and its includes are files edited in place, and a
//! replacement across them is atomic.
//!
//! An include of the deck edits its own file, never the deck; a replacement
//! commits the deck and an include together as one undo step, or refuses
//! without a partial edit. The outline is parsed once per change, which is
//! what keeps a large deck off the frame budget.

use super::fixtures::{
    DECK_INCLUDE, INCLUDE_NATIVE_ORIGIN, ORIGINAL_INCLUDE, ROOT, seed_deck_with_include,
};
use super::*;
use crate::product::ObjectRevision;
use crate::state::netlist_diagnostics::Diagnostic;
use crate::state::{NetlistDocumentId, NetlistValidationReceipt};
use crate::workbench::code::tests::{CodeHarness, harness};

const EDITED_INCLUDE: &str = ".param rmodel=2.2k\n";

/// The deck-with-include fixture over code's harness.
fn deck_state() -> CodeHarness {
    let mut harness = harness();
    seed_deck_with_include(&mut harness.model, &mut harness.state.netlist);
    harness
}

/// The open deck's text and its include's, as the session holds them.
fn deck_texts(harness: &CodeHarness) -> (String, String) {
    let deck = harness.state.netlist.open_deck.as_ref().unwrap();
    (
        deck.root.text.clone(),
        deck.include(DECK_INCLUDE).unwrap().text.clone(),
    )
}

/// Bring the deck's include to the front.
fn show_deck_include(harness: &mut CodeHarness) {
    open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        DECK_INCLUDE,
    )
    .unwrap();
}

/// The navigator asks for the outline on every frame. Building it costs the
/// deck, so the answer has to be the same object until the bytes change, and
/// a different one once they do: a navigator listing declarations the buffer
/// no longer contains is worse than a slow one. A frame gets the last outline
/// built while the next one builds off the editor's thread.
#[test]
fn the_visible_outline_is_built_once_per_change_and_never_reused_across_one() {
    let mut harness = deck_state();

    let first = visible_source_index_now(&harness.model, &mut harness.state.netlist);
    let ctx = egui::Context::default();
    let frame = visible_source_index_in(&ctx, &harness.model, &mut harness.state.netlist);
    assert!(
        std::sync::Arc::ptr_eq(&first, &frame),
        "an unchanged buffer must not be built twice"
    );
    assert!(first.describes(ROOT));
    assert_eq!(first.card(3), "V1 out 0 1");

    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        format!("{ROOT}* one more card\n")
    ));
    let replaced = visible_source_index_in(&ctx, &harness.model, &mut harness.state.netlist);
    assert!(
        replaced.describes(&format!("{ROOT}* one more card\n")),
        "text put in the buffer without typing is outlined before a frame shows it"
    );

    let typed = format!("{ROOT}* one more card\n* and one typed\n");
    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        typed.clone()
    ));
    harness.state.netlist.typed_revision = Some(harness.state.netlist.revision);
    let stale = visible_source_index_in(&ctx, &harness.model, &mut harness.state.netlist);
    assert!(
        std::sync::Arc::ptr_eq(&replaced, &stale) || stale.describes(&typed),
        "while the writer types, a frame shows the last outline built or the new one, never another"
    );
    let edited = visible_source_index_now(&harness.model, &mut harness.state.netlist);
    assert!(edited.describes(&typed));
    assert_eq!(edited.line_count(), first.line_count() + 2);

    // Switching documents changes the buffer without editing it.
    assert!(open_generated_primary_in(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(visible_source_index_now(&harness.model, &mut harness.state.netlist).describes(ROOT));
}

#[test]
fn reactivating_the_open_deck_preserves_validation_evidence() {
    let mut harness = deck_state();
    harness.state.netlist.revision = 9;
    let digest = crate::state::content_digest(ROOT);
    harness.model.netlist_artifacts.validation = Some(NetlistValidationReceipt {
        visible_content_digest: digest,
        executable_source_digest: crate::product::ContentDigest::from_bytes([0x51; 32]),
        prepared_snapshot_digest: crate::product::ContentDigest::from_bytes([0x52; 32]),
        project_revision: harness.model.workspace.content.project.revision().get(),
        task_count: 1,
        advisory_count: 0,
    });
    let validation = harness.model.netlist_artifacts.validation.clone();

    assert!(crate::workbench::documents::deck_files::show_deck(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(crate::workbench::documents::deck_files::show_deck(
        &mut harness.model,
        &mut harness.state.netlist
    ));

    assert_eq!(harness.state.netlist.revision, 9);
    assert_eq!(harness.model.netlist_artifacts.validation, validation);
    assert_eq!(harness.model.simulation.source.netlist_content, ROOT);
}

/// An include of the deck is a file of its own: an edit made on its tab
/// changes that file and marks it unsaved, and leaves the deck as it was.
#[test]
fn an_include_of_the_deck_edits_its_own_file_and_never_the_deck() {
    let mut harness = deck_state();
    show_deck_include(&mut harness);
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        ORIGINAL_INCLUDE
    );
    assert!(active_netlist_source_is_editable_in(
        &harness.model,
        &harness.state.netlist
    ));

    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        EDITED_INCLUDE.to_owned()
    ));

    assert_eq!(
        deck_texts(&harness),
        (ROOT.to_owned(), EDITED_INCLUDE.to_owned()),
        "include edits never rewrite the deck"
    );
    let deck = harness.state.netlist.open_deck.as_ref().unwrap();
    assert!(!deck.root.dirty);
    assert!(deck.include(DECK_INCLUDE).unwrap().dirty);
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        EDITED_INCLUDE
    );
}

#[test]
fn canonical_diagnostic_navigation_opens_the_exact_include_and_line() {
    let mut harness = deck_state();
    let mut diagnostic = Diagnostic::error("included model is malformed");
    diagnostic.source_path = Some(std::path::PathBuf::from(INCLUDE_NATIVE_ORIGIN));
    diagnostic.source_line = Some(17);
    diagnostic.line = None;
    let diagnostic_id = diagnostic.canonical.diagnostic_id;
    harness.state.netlist.diagnostics =
        std::sync::Arc::new(NetlistDiagnosticCollection::try_new(vec![diagnostic], ROOT).unwrap());

    open_diagnostic_location(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        &mut harness.wb,
        diagnostic_id,
    )
    .unwrap();

    assert_eq!(harness.state.netlist.front.include(), Some(DECK_INCLUDE));
    assert_eq!(harness.state.netlist.cursor_line, 17);
    assert_eq!(harness.state.netlist.requested_line, Some(18));
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        ORIGINAL_INCLUDE
    );
}

#[test]
fn secondary_document_limit_refusal_leaves_the_active_document_unchanged() {
    let mut harness = deck_state();
    for index in 0..MAX_OPEN_NETLIST_SECONDARY_DOCUMENTS {
        harness.state.netlist_open_documents.insert(
            crate::workbench::state::WorkspaceDocumentId::NetlistDependency {
                root: NetlistDocumentId::new(),
                logical_identity: format!("fixture-{index}.inc"),
            },
        );
    }
    let before_source = harness.model.simulation.source.netlist_content.clone();
    let before_document = harness.state.netlist.front.root_kind();

    let error = open_netlist_dependency_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &mut harness.state.netlist_open_documents,
        &mut harness.runs,
        DECK_INCLUDE,
    )
    .unwrap_err();
    assert!(error.contains("Close an include tab"));
    assert_eq!(harness.state.netlist.front.root_kind(), before_document);
    assert!(harness.state.netlist.front.include().is_none());
    assert_eq!(
        harness.model.simulation.source.netlist_content,
        before_source
    );
}

/// The deck's edit and its include's, as a replace across both prepares them.
fn deck_and_include_edits() -> (String, Vec<OwnedNetlistReplacement>) {
    let edited_root = ROOT.replace("V1 out 0 1", "VINPUT out 0 1");
    let edits = vec![
        OwnedNetlistReplacement::root(ROOT, edited_root.clone(), 1),
        OwnedNetlistReplacement::dependency(
            DECK_INCLUDE,
            ORIGINAL_INCLUDE,
            EDITED_INCLUDE.to_owned(),
            1,
        ),
    ];
    (edited_root, edits)
}

#[test]
fn workspace_replacement_commits_deck_and_include_together() {
    let mut harness = deck_state();
    show_deck_include(&mut harness);
    let (edited_root, edits) = deck_and_include_edits();

    assert_eq!(
        replace_deck_files(&mut harness.model, &mut harness.state.netlist, edits).unwrap(),
        2
    );
    assert_eq!(
        deck_texts(&harness),
        (edited_root, EDITED_INCLUDE.to_owned())
    );
    assert_eq!(
        harness.model.simulation.source.netlist_content, EDITED_INCLUDE,
        "the include in front follows the atomic commit"
    );
}

#[test]
fn workspace_replacement_is_one_atomic_undo_and_redo_step() {
    let mut harness = deck_state();
    show_deck_include(&mut harness);
    let (edited_root, edits) = deck_and_include_edits();
    replace_deck_files(&mut harness.model, &mut harness.state.netlist, edits).unwrap();
    assert!(can_undo_netlist_edit(&harness.state.netlist));

    assert!(
        undo_netlist_edit(&mut harness.model, &mut harness.state.netlist)
            .unwrap()
            .is_some()
    );
    assert_eq!(
        deck_texts(&harness),
        (ROOT.to_owned(), ORIGINAL_INCLUDE.to_owned())
    );
    assert!(can_redo_netlist_edit(&harness.state.netlist));

    assert!(
        redo_netlist_edit(&mut harness.model, &mut harness.state.netlist)
            .unwrap()
            .is_some()
    );
    assert_eq!(
        deck_texts(&harness),
        (edited_root, EDITED_INCLUDE.to_owned())
    );
}

#[test]
fn workspace_replacement_rejects_stale_include_without_partial_root_edit() {
    let mut harness = deck_state();
    show_deck_include(&mut harness);
    let (_, stale_edits) = deck_and_include_edits();
    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        ".param rmodel=3.3k\n".to_owned()
    ));

    let error = replace_deck_files(&mut harness.model, &mut harness.state.netlist, stale_edits)
        .unwrap_err();

    assert!(error.contains("nothing was changed"), "{error}");
    assert_eq!(
        deck_texts(&harness),
        (ROOT.to_owned(), ".param rmodel=3.3k\n".to_owned())
    );
}

#[test]
fn run_deck_snapshot_comparison_reports_the_edits_made_since_the_run() {
    const RAN: &str = "ran deck\nR1 out 0 1k\n.op\n.end\n";
    const EDITED: &str = "ran deck\nR1 out 0 2k\n.op\n.end\n";

    let mut harness = deck_state();
    assert!(edit_deck_file(
        &mut harness.model,
        &mut harness.state.netlist,
        EDITED.to_owned()
    ));
    harness.runs.manual_run.last_run_buffer = Some(RAN.to_owned());
    harness.runs.manual_run.last_run_id = Some(3);
    assert!(open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));

    compare_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
    )
    .unwrap();

    assert_eq!(
        harness.state.netlist.front.root_kind(),
        NetlistRoot::Comparison
    );
    let diff = harness.state.netlist.generated_diff_source.as_str();
    assert!(diff.contains("--- run-3"), "{diff}");
    assert!(diff.contains("-R1 out 0 1k"), "{diff}");
    assert!(diff.contains("+R1 out 0 2k"), "{diff}");

    assert!(close_revision_comparison(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert_eq!(harness.state.netlist.front.root_kind(), NetlistRoot::Deck);
    assert_eq!(harness.model.simulation.source.netlist_content, EDITED);
}

#[test]
fn run_deck_snapshot_needs_both_the_run_and_its_deck() {
    let mut harness = deck_state();
    harness.runs.manual_run.last_run_buffer = Some("ran\n.end\n".to_owned());
    assert!(run_deck_snapshot_run_id_in(&harness.runs).is_none());
    assert!(!open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));

    harness.runs.manual_run.last_run_id = Some(9);
    harness.runs.manual_run.last_run_buffer = None;
    assert!(run_deck_snapshot_run_id_in(&harness.runs).is_none());
    assert!(!open_run_deck_snapshot_in(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs
    ));
}

/// The executed-deck viewer: read-only, per point, and never substituting.
///
/// The retention half — that every run seals these at dispatch — is proved in
/// `simulation::controller::tests`. What is proved here is what the document
/// then is: the immutable run-snapshot document, showing the point that was
/// asked for, refusing a comparison it cannot honestly make, and refusing a
/// run whose deck this session does not hold rather than showing another one.
#[test]
fn an_executed_deck_opens_read_only_at_the_point_it_was_asked_for() {
    use crate::state::{ExecutedDeck, ExecutedDeckPoint, sealed_model_sources};

    const NOMINAL: &str =
        "run deck\n* RSpice sealed model source: pack rspice-opamps 2.1.0\nV1 out 0 1\n.op\n.end\n";
    const HOT: &str = "run deck\n* RSpice sealed model source: pack rspice-opamps 2.1.0\n.OPTIONS TEMP=125\nV1 out 0 1\n.op\n.end\n";

    let mut harness = deck_state();
    let point = |label: &str, text: &str| {
        let deck: std::sync::Arc<str> = std::sync::Arc::from(text);
        ExecutedDeckPoint {
            label: label.to_owned(),
            model_sources: sealed_model_sources(&deck),
            deck,
        }
    };
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: 4,
            points: vec![point("TT 27C", NOMINAL), point("TT 125C", HOT)],
        });

    assert!(reveal_executed_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
        &mut harness.wb,
        4,
        1
    ));
    assert_eq!(
        harness.state.netlist.front.root_kind(),
        NetlistRoot::RunInput
    );
    assert_eq!(harness.model.simulation.source.netlist_content, HOT);
    assert!(
        !active_netlist_source_is_editable_in(&harness.model, &harness.state.netlist),
        "an executed deck is a viewer, never a buffer"
    );
    assert_eq!(
        harness
            .model
            .simulation
            .retained
            .executed_decks
            .get(4)
            .map(crate::state::ExecutedDeck::model_sources),
        Some(vec!["pack rspice-opamps 2.1.0".to_owned()]),
        "and its own comments name the release it was given"
    );
    assert!(
        compare_run_deck_snapshot_in(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs
        )
        .is_err(),
        "there is no working copy of a deck nobody typed"
    );

    // A point past the end resolves to one the run actually has rather than
    // to an empty document under a header naming the run.
    assert!(reveal_executed_deck(
        &mut harness.model,
        &mut harness.state.netlist,
        &harness.runs,
        &mut harness.wb,
        4,
        40
    ));
    assert_eq!(harness.model.simulation.source.netlist_content, HOT);

    assert!(
        !reveal_executed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs,
            &mut harness.wb,
            5,
            0
        ),
        "a run whose deck is not held refuses rather than showing another one"
    );

    assert!(close_run_deck_snapshot(
        &mut harness.model,
        &mut harness.state.netlist
    ));
    assert!(
        harness.state.netlist.executed_deck_view.is_none(),
        "leaving the viewer releases the run it was bound to"
    );
}

/// The badge over an executed deck is earned or it is not made.
///
/// `source_content_digest` is the one receipt field that authenticates deck
/// text, so it is the one thing a retained deck can be checked against, and it
/// covers the run-level source only. What is proved here is that a real
/// recomputation decides the verdict — a point that solved the sealed source
/// verbatim verifies, a corner's materialized variant of it is named as
/// unsealed rather than as tampered, a rewritten deck is named as matching
/// nothing, and a run with no receipt claims nothing at all.
#[test]
fn an_executed_deck_claims_verification_only_when_it_reproduces_the_sealed_digest() {
    use crate::state::{
        AnalysisResultSourceDomain, ExecutedDeck, ExecutedDeckPoint, PreparedRunReceipt,
        PreparedRunTaskReceipt, PreparedSourceCheckReceipt, SimulationRunProvenance,
        sealed_model_sources,
    };
    use crate::workbench::documents::netlist_document::ExecutedDeckVerification;

    const SEALED: &str = "authorized deck\nV1 out 0 1\n.op\n.end\n";
    const CORNER: &str = "authorized deck\n.lib cmos.lib ss\nV1 out 0 1\n.op\n.end\n";

    let digest = |value: u8| crate::product::ContentDigest::from_bytes([value; 32]);
    let point = |label: &str, text: &str| {
        let deck: std::sync::Arc<str> = std::sync::Arc::from(text);
        ExecutedDeckPoint {
            label: label.to_owned(),
            model_sources: sealed_model_sources(&deck),
            deck,
        }
    };
    let seal = |harness: &mut CodeHarness, source: &str| {
        let receipt = PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
            source_domain: AnalysisResultSourceDomain::SimulationPlan,
            simulation_plan_id: Some(crate::product::SimulationPlanId::new()),
            project_revision: ObjectRevision::INITIAL,
            prepared_snapshot_digest: digest(0xa0),
            source_content_digest:
                rspice_simulation::sealed_source::sealed_executable_source_digest(
                    AnalysisResultSourceDomain::SimulationPlan,
                    source,
                )
                .expect("a plan run seals its executable source"),
            source_check_receipt: PreparedSourceCheckReceipt::SchematicDrc(digest(0xa2)),
            project_model_sources: Vec::new(),
            specifications: Vec::new(),
            specification_policy:
                rspice_results::specification::PreparedSpecificationPolicy::default(),
            tasks: vec![
                PreparedRunTaskReceipt::new(
                    crate::product::AnalysisInstanceId::new(),
                    ObjectRevision::INITIAL,
                    Vec::new(),
                    0,
                    digest(0xa3),
                )
                .expect("task receipt"),
            ],
        })
        .expect("prepared run receipt");
        let run = harness
            .model
            .simulation
            .start_prepared_run(receipt)
            .unwrap();
        run.id
    };

    let mut harness = deck_state();
    let sequence = seal(&mut harness, SEALED);
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: sequence,
            points: vec![point("TT 27C", SEALED), point("SS 27C", CORNER)],
        });

    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 0),
        Some(ExecutedDeckVerification::Verified),
        "the point that solved the sealed source reproduces its digest"
    );
    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 1),
        Some(ExecutedDeckVerification::PointVariant),
        "a corner's materialized source is unsealed, which is not the same as wrong"
    );
    assert!(
        reveal_executed_deck(
            &mut harness.model,
            &mut harness.state.netlist,
            &harness.runs,
            &mut harness.wb,
            sequence,
            0
        ),
        "the deck opens at the point asked for"
    );
    assert_eq!(
        harness
            .state
            .netlist
            .executed_deck_view
            .map(|selection| selection.verification),
        Some(ExecutedDeckVerification::Verified),
        "and the verdict travels with the selection the strip reads"
    );

    // Every retained byte rewritten: nothing this run holds reproduces what
    // its receipt sealed, and the viewer must not soften that into "unsealed".
    harness
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: sequence,
            points: vec![point("TT 27C", "rewritten deck\n.end\n")],
        });
    assert_eq!(
        ExecutedDeckVerification::of(&harness.model, sequence, 0),
        Some(ExecutedDeckVerification::Unmatched),
        "a deck the run was never authorized over says exactly that"
    );

    let mut unsealed = deck_state();
    let mut legacy = crate::state::SimulationRun::new(3);
    legacy
        .restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("legacy history is explicitly classified");
    unsealed.model.simulation.retained.runs = vec![legacy].into();
    unsealed
        .model
        .simulation
        .retained
        .executed_decks
        .retain(ExecutedDeck {
            run_id: 3,
            points: vec![point("TT 27C", SEALED)],
        });
    assert_eq!(
        ExecutedDeckVerification::of(&unsealed.model, 3, 0),
        Some(ExecutedDeckVerification::NotRecorded),
        "a run with no receipt digest has nothing to be checked against"
    );
    assert_eq!(
        ExecutedDeckVerification::of(&unsealed.model, 4, 0),
        None,
        "and a run whose decks are not held is not a verdict at all"
    );
}
