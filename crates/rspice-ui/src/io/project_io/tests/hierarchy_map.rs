//! What the run receipt's hierarchy map does to a project file.
//!
//! Two facts are pinned here, and they pull in opposite directions. A run
//! sealed today records the exact occurrence map its deck emitted, down to the
//! engine scope each occurrence was spelled with, because that map is the only
//! way a stored result can be read back as a design instance. A run recorded
//! before schema v15 records none, and migration leaves it that way: rows
//! reconstructed from today's design would attribute a deck to a run that
//! never emitted it.

use super::*;

use crate::state::HierarchyMapRow;

const OUTER_OCCURRENCE: &str = "/X1";
const INNER_OCCURRENCE: &str = "/X1/X2";

fn amp_reference() -> CellViewRef {
    CellViewRef::new("user", "amp", "schematic")
}

fn hierarchy_row(occurrence: &str, master: &str, engine_prefix: &str) -> HierarchyMapRow {
    HierarchyMapRow::new(occurrence, master, engine_prefix, amp_reference())
        .expect("a fixture row names an occurrence the engine can spell")
}

fn reference_map() -> Vec<HierarchyMapRow> {
    vec![
        hierarchy_row(OUTER_OCCURRENCE, "amp_1", "X1"),
        hierarchy_row(INNER_OCCURRENCE, "amp_2", "X1.X2"),
    ]
}

/// One sealed plan run whose receipt carries `hierarchy_map`, as
/// `ProjectSimulationResults` writes it at the current schema.
fn sealed_results(hierarchy_map: Vec<HierarchyMapRow>) -> ProjectSimulationResults {
    let snapshot = ContentDigest::from_bytes([0xb0; 32]);
    let instance = AnalysisInstanceId::new();
    let mut run = SimulationRun::new(41);
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
        ContentDigest::from_bytes([0xb3; 32]),
    )
    .expect("prepared task receipt");
    let receipt = PreparedRunReceipt::new(
        AnalysisResultSourceDomain::SimulationPlan,
        Some(SimulationPlanId::new()),
        ObjectRevision::INITIAL,
        snapshot,
        ContentDigest::from_bytes([0xb1; 32]),
        PreparedSourceCheckReceipt::SchematicDrc(ContentDigest::from_bytes([0xb2; 32])),
        vec![task],
    )
    .expect("prepared run receipt")
    .with_hierarchy_map(hierarchy_map)
    .expect("the fixture map names each occurrence once");
    run.restore_provenance(SimulationRunProvenance::Prepared(Box::new(receipt)))
        .expect("the fixture run seals explicitly");

    let mut simulation = SimulationState::default();
    simulation.runs = vec![run];
    simulation.next_run_id = 41;
    ProjectSimulationResults::from_state(&simulation)
}

fn restored_receipt(results: &ProjectSimulationResults) -> PreparedRunReceipt {
    let PersistedField::Value(receipt) = &results.runs[0].prepared_receipt else {
        panic!("the fixture run retains a prepared receipt");
    };
    receipt
        .clone()
        .into_receipt()
        .expect("the retained receipt restores")
}

#[test]
fn a_sealed_hierarchy_map_round_trips_through_the_project_file() {
    let written = sealed_results(reference_map());
    let encoded = serde_json::to_string(&written).expect("results serialize");
    let decoded =
        serde_json::from_str::<ProjectSimulationResults>(&encoded).expect("results deserialize");
    decoded.validate().expect("the written document validates");

    let rows = restored_receipt(&decoded)
        .hierarchy_map()
        .iter()
        .map(|row| {
            (
                row.occurrence().to_owned(),
                row.master().to_owned(),
                row.engine_prefix().to_owned(),
                row.master_reference().key(),
            )
        })
        .collect::<Vec<_>>();
    assert_eq!(
        rows,
        vec![
            (
                OUTER_OCCURRENCE.to_owned(),
                "amp_1".to_owned(),
                "X1".to_owned(),
                "user/amp/schematic".to_owned()
            ),
            (
                INNER_OCCURRENCE.to_owned(),
                "amp_2".to_owned(),
                "X1.X2".to_owned(),
                "user/amp/schematic".to_owned()
            ),
        ],
        "the receipt records what the deck named, not a re-derivation of it"
    );
}

#[test]
fn a_persisted_engine_prefix_that_does_not_name_its_occurrence_is_refused() {
    let mut document =
        serde_json::to_value(sealed_results(reference_map())).expect("results serialize");
    document["runs"][0]["prepared_receipt"]["hierarchy_map"][1]["engine_prefix"] =
        serde_json::Value::from("X9.X9");

    let decoded = serde_json::from_value::<ProjectSimulationResults>(document)
        .expect("an edited row is still well-formed JSON");
    let error = decoded
        .validate()
        .expect_err("a prefix that names another occurrence cannot be trusted");
    assert!(error.contains("rather than 'X1.X2'"), "{error}");
}

#[test]
fn results_written_before_the_hierarchy_map_load_and_reserialize_without_one() {
    let mut document = serde_json::to_value(sealed_results(Vec::new())).expect("results serialize");
    assert!(
        !document.to_string().contains("hierarchy_map"),
        "a run with no occurrence map writes no map field at all"
    );
    document["schema_version"] =
        serde_json::Value::from(GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION);

    let mut persisted = serde_json::from_value::<ProjectSimulationResults>(document)
        .expect("a schema-v14 document deserializes");
    persisted
        .migrate_to_current(ProjectId::new())
        .expect("an authentic schema-v14 document migrates");

    assert_eq!(
        persisted.schema_version,
        PROJECT_SIMULATION_RESULTS_SCHEMA_VERSION
    );
    assert!(
        restored_receipt(&persisted).hierarchy_map().is_empty(),
        "migration never invents an occurrence the run did not emit"
    );
    let rewritten = serde_json::to_string(&persisted).expect("the migrated document serializes");
    assert!(
        !rewritten.contains("hierarchy_map"),
        "an empty map is written as absence, exactly as the older build wrote it:\n{rewritten}"
    );
}

#[test]
fn a_schema_v14_document_cannot_smuggle_a_hierarchy_map() {
    let mut smuggled = sealed_results(reference_map());
    smuggled.schema_version = GOVERNED_SPECIFICATION_RESULTS_SCHEMA_VERSION;

    let error = smuggled
        .migrate_to_current(ProjectId::new())
        .expect_err("schema v14 predates the hierarchy map");
    assert!(
        error.contains("contains a hierarchy map introduced by schema v15"),
        "{error}"
    );
}
