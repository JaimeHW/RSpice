//! What the decks a project's runs executed do to a project file.
//!
//! Three facts are pinned here. The exact bytes an engine read survive a save
//! and a reload, still sharing one allocation between the points that solved
//! one source — writing a sweep's shared deck per point would multiply the
//! largest text this application holds by the point count. A deck belongs to
//! a dataset, so a run the history no longer contains contributes none. And
//! the archive's own ceilings are re-applied on the way in: a file carrying
//! more than a session could have retained was not written by one, and is
//! refused rather than trimmed.

use super::*;

use crate::state::{ExecutedDeck, ExecutedDeckPoint, sealed_model_sources};

const NOMINAL: &str = "sweep deck\n* RSpice sealed model source: pack rspice-cmos 3.0.0\n\
                       V1 out 0 1\n.op\n.end\n";
const HOT: &str = "sweep deck\n* RSpice sealed model source: pack rspice-cmos 3.0.0\n\
                   .OPTIONS TEMP=125\nV1 out 0 1\n.op\n.end\n";

fn point(label: &str, deck: &std::sync::Arc<str>) -> ExecutedDeckPoint {
    ExecutedDeckPoint {
        label: label.to_owned(),
        model_sources: sealed_model_sources(deck),
        deck: std::sync::Arc::clone(deck),
    }
}

/// One sealed plan run, with an id the executed-deck archive can key on.
fn sealed_run(sequence: u64, byte: u8) -> SimulationRun {
    let snapshot = ContentDigest::from_bytes([byte; 32]);
    let instance = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(sequence);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Ac, "AC").with_provenance(
            AnalysisResultProvenance::new(instance, ObjectRevision::INITIAL, snapshot, Vec::new())
                .expect("prepared provenance"),
        ),
    );
    let task = PreparedRunTaskReceipt::new(
        instance,
        ObjectRevision::INITIAL,
        Vec::new(),
        analysis_kind_tag_for_plan_kind(AnalysisKind::Ac),
        ContentDigest::from_bytes([byte.wrapping_add(3); 32]),
    )
    .expect("prepared task receipt");
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        snapshot,
        ContentDigest::from_bytes([byte.wrapping_add(1); 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes(
            [byte.wrapping_add(2); 32],
        )),
        vec![task],
    )
    .expect("prepared run receipt");
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("the fixture run seals explicitly");
    run
}

/// A three-point sweep whose first two points solved one shared source.
fn swept_state() -> SimulationState {
    let nominal: std::sync::Arc<str> = std::sync::Arc::from(NOMINAL);
    let hot: std::sync::Arc<str> = std::sync::Arc::from(HOT);
    let mut simulation = SimulationState::default();
    simulation.runs = vec![sealed_run(7, 0xc0)];
    simulation.next_run_id = 7;
    simulation.executed_decks.retain(ExecutedDeck {
        run_id: 7,
        points: vec![
            point("TT 27C", &nominal),
            point("SS 27C", &nominal),
            point("TT 125C", &hot),
        ],
    });
    simulation
}

fn round_trip(written: &ProjectSimulationResults) -> ProjectSimulationResults {
    let encoded = serde_json::to_string(written).expect("results serialize");
    let decoded =
        serde_json::from_str::<ProjectSimulationResults>(&encoded).expect("results deserialize");
    decoded.validate().expect("the written document validates");
    decoded
}

#[test]
fn a_runs_executed_decks_round_trip_and_stay_shared_between_its_points() {
    let written = ProjectSimulationResults::from_state(&swept_state());
    assert_eq!(
        written.executed_decks.sources.len(),
        2,
        "three points over two distinct sources are two sources on disk"
    );

    let decoded = round_trip(&written);
    let mut reloaded = SimulationState::default();
    decoded
        .apply_to_state(&mut reloaded)
        .expect("the persisted results restore");

    let record = reloaded
        .executed_decks
        .get(7)
        .expect("the reopened project holds the run's decks");
    assert_eq!(
        record
            .points
            .iter()
            .map(|point| (point.label.as_str(), point.deck.as_ref()))
            .collect::<Vec<_>>(),
        vec![("TT 27C", NOMINAL), ("SS 27C", NOMINAL), ("TT 125C", HOT),],
        "every point reopens with the exact bytes its engine read"
    );
    assert!(
        std::sync::Arc::ptr_eq(&record.points[0].deck, &record.points[1].deck),
        "the points that solved one source still share one allocation"
    );
    assert!(
        !std::sync::Arc::ptr_eq(&record.points[0].deck, &record.points[2].deck),
        "and a point that solved a different source is not folded into it"
    );
    assert_eq!(
        record.model_sources(),
        vec!["pack rspice-cmos 3.0.0".to_owned()],
        "the sealed sources are read back out of the deck's own comments"
    );
}

/// A project written before executed decks were retained states no key, and
/// must load with none rather than being refused.
///
/// The load direction is the one that matters here: every project saved before
/// schema v15 carries no `executed_decks` member at all, and `deny_unknown_fields`
/// elsewhere in this document makes a missing `serde(default)` a hard refusal
/// to open the file. Removing the key from a real serialized document is the
/// only way to exercise that; writing an empty archive exercises a present
/// field, which is a different question.
#[test]
fn a_project_stating_no_executed_decks_loads_with_none() {
    let written = ProjectSimulationResults::from_state(&swept_state());
    let mut document: serde_json::Value =
        serde_json::to_value(&written).expect("results serialize");
    let object = document
        .as_object_mut()
        .expect("results are written as an object");
    assert!(
        object.remove("executed_decks").is_some(),
        "the fixture must actually carry decks for their removal to mean anything"
    );

    let decoded: ProjectSimulationResults =
        serde_json::from_value(document).expect("a project written before v15 still opens");
    decoded
        .validate()
        .expect("a document carrying no deck is internally consistent");
    assert!(
        decoded.executed_decks.is_empty(),
        "no decks retained, and none invented"
    );
    assert_eq!(
        decoded.runs.len(),
        written.runs.len(),
        "and the datasets it does carry are unaffected"
    );
}

#[test]
fn a_run_the_history_no_longer_holds_writes_no_deck() {
    let mut simulation = swept_state();
    let orphan: std::sync::Arc<str> = std::sync::Arc::from("dropped deck\n.end\n");
    simulation.executed_decks.retain(ExecutedDeck {
        run_id: 9,
        points: vec![point("TT 27C", &orphan)],
    });

    let written = ProjectSimulationResults::from_state(&simulation);
    assert_eq!(
        written
            .executed_decks
            .runs
            .iter()
            .map(|deck| deck.run_id)
            .collect::<Vec<_>>(),
        vec![7],
        "a deck whose dataset is not in the file is not in the file either"
    );
    assert_eq!(
        written.executed_decks.sources.len(),
        2,
        "and its source is not carried along unreferenced"
    );
}

#[test]
fn a_persisted_deck_naming_no_retained_run_is_refused() {
    let mut written = ProjectSimulationResults::from_state(&swept_state());
    written.executed_decks.runs[0].run_id = 8;

    let error = written
        .validate()
        .expect_err("a deck must belong to a dataset the file contains");
    assert!(error.contains("simulation run 8"), "{error}");
}

#[test]
fn a_deck_source_no_retained_point_references_is_refused() {
    let mut written = ProjectSimulationResults::from_state(&swept_state());
    written
        .executed_decks
        .sources
        .push("smuggled deck\n.end\n".to_owned());

    let error = written
        .validate()
        .expect_err("deck text nothing shows is deck text nothing accounts for");
    assert!(error.contains("referenced by no retained point"), "{error}");
}

#[test]
fn a_point_referencing_a_source_the_file_does_not_carry_is_refused() {
    let mut written = ProjectSimulationResults::from_state(&swept_state());
    written.executed_decks.runs[0].points[0].source = 9;

    let error = written
        .validate()
        .expect_err("a point must name a source the file carries");
    assert!(error.contains("references source 9 of 2"), "{error}");
}

#[test]
fn more_retained_decks_than_a_session_could_hold_fails_closed_on_load() {
    let mut simulation = SimulationState::default();
    simulation.runs = (1..=5)
        .map(|sequence| sealed_run(sequence, 0x10 * sequence as u8))
        .collect();
    simulation.next_run_id = 5;
    let mut written = ProjectSimulationResults::from_state(&simulation);
    // Written by hand rather than by the archive, which would have evicted
    // the oldest before a fifth could be retained. This is the shape of the
    // file a tamperer produces, not one a session can.
    written.executed_decks.sources = (1..=5)
        .map(|sequence| format!("deck {sequence}\n.end\n"))
        .collect();
    written.executed_decks.runs = (1..=5)
        .map(|sequence| ProjectExecutedDeck {
            run_id: sequence,
            points: vec![ProjectExecutedDeckPoint {
                label: "TT 27C".to_owned(),
                source: sequence as usize - 1,
            }],
        })
        .collect();
    written
        .validate()
        .expect("every deck is internally well formed");

    let mut reloaded = SimulationState::default();
    let error = written
        .apply_to_state(&mut reloaded)
        .expect_err("a file over the archive's ceiling was not written by a session");
    assert!(error.contains("exceed the archive limit of 4"), "{error}");
    assert!(
        reloaded.executed_decks.get(1).is_none(),
        "and nothing from a refused file is installed"
    );
}

#[test]
fn executed_decks_on_a_file_claiming_an_older_schema_are_refused() {
    let mut written = ProjectSimulationResults::from_state(&swept_state());
    written.schema_version = GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION;

    let error = written
        .migrate_to_current(ProjectId::new())
        .expect_err("a schema that never retained decks cannot be carrying them");
    assert!(
        error.contains("executed decks introduced by schema v15"),
        "{error}"
    );
}
