//! Properties of the adapter's STEP/TEMP axis-execution manifest.
//!
//! The manifest is an orchestration record whose whole job is to be checkable:
//! a reader must be able to prove that the coordinate identities it names were
//! produced by the planner from the axis assignments beside them, and that the
//! coordinate set is the complete Cartesian product in canonical order. The
//! laws stated here are therefore:
//!
//! * **identity** — every accepted manifest recomputes each `coordinate_id`
//!   with `rspice_core::execution::numeric_run_coordinate_id`, so a coordinate
//!   ID detached from its assignments is refused;
//! * **round trip** — a manifest that validates encodes and decodes back to an
//!   equal manifest;
//! * **completeness** — an incomplete Cartesian product, a permuted coordinate
//!   order, or a duplicated artifact path is refused with a typed error.
//!
//! Generation is seeded and failure persistence is off, so every case here is
//! reproducible from this source alone.

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use rspice_core::execution::{AxisKind, numeric_run_coordinate_id};
use rspice_engine_adapter::axis_execution_document::{
    AnalysisExecution, AxisAnalysisKind, AxisAssignmentDocument, AxisAssignmentKind,
    AxisExecutionDocument, AxisExecutionDocumentError, CoordinateExecution,
    OutputNamespaceDocument, ResultDocumentReference, StepTargetDocument,
};
use rspice_engine_adapter::measure::canonical_decimal;
use rspice_engine_adapter::wire::MAX_ENGINE_RESULT_MANIFEST_BYTES;

fn runner(seed: u64, cases: u32) -> TestRunner {
    let mut entropy = [0_u8; 32];
    entropy[..8].copy_from_slice(&seed.to_le_bytes());
    TestRunner::new_with_rng(
        Config {
            cases,
            failure_persistence: None,
            ..Config::default()
        },
        TestRng::from_seed(RngAlgorithm::ChaCha, &entropy),
    )
}

/// A generated two-axis plan: a `.STEP PARAM r` sweep and a `.TEMP` sweep.
#[derive(Debug, Clone)]
struct Plan {
    resistances: Vec<f64>,
    temperatures: Vec<f64>,
}

impl Plan {
    fn axis_values(&self) -> [&[f64]; 2] {
        [&self.resistances, &self.temperatures]
    }
}

fn distinct_values(
    low: f64,
    high: f64,
    count: std::ops::Range<usize>,
) -> impl Strategy<Value = Vec<f64>> {
    prop::collection::vec(low..high, count).prop_filter_map(
        "axis values must be distinct",
        |values| {
            let mut canonical = values
                .iter()
                .map(|value| canonical_decimal(*value))
                .collect::<Option<Vec<_>>>()?;
            canonical.sort();
            let unique = canonical.len();
            canonical.dedup();
            (canonical.len() == unique).then_some(values)
        },
    )
}

fn plan() -> impl Strategy<Value = Plan> {
    (
        distinct_values(1.0, 1e5, 1..4),
        distinct_values(-40.0, 125.0, 1..4),
    )
        .prop_map(|(resistances, temperatures)| Plan {
            resistances,
            temperatures,
        })
}

/// Build the manifest a correct executor would publish for `plan`.
///
/// Coordinates are enumerated first-axis-fastest, exactly as
/// `DeckPlan::coordinates_with_abort` does, and each coordinate ID is the
/// planner's own identity for its numeric assignments.
fn manifest(plan: &Plan) -> AxisExecutionDocument {
    let [resistances, temperatures] = plan.axis_values();
    let mut runs = Vec::new();
    for ordinal in 0..resistances.len() * temperatures.len() {
        let resistance = resistances[ordinal % resistances.len()];
        let temperature = temperatures[(ordinal / resistances.len()) % temperatures.len()];
        let coordinate_id = numeric_run_coordinate_id(
            &[
                (AxisKind::Step, "param:r", resistance),
                (AxisKind::Temperature, "temperature", temperature),
            ],
            0,
        )
        .expect("finite numeric assignments have a planner identity")
        .to_string();
        let namespace = format!("run-{coordinate_id}");
        runs.push(CoordinateExecution {
            ordinal: ordinal + 1,
            coordinate_id: coordinate_id.clone(),
            coordinate_namespace: namespace.clone(),
            assignments: vec![
                AxisAssignmentDocument {
                    kind: AxisAssignmentKind::Step,
                    name: "param:r".to_owned(),
                    value_index: ordinal % resistances.len(),
                    value_decimal: canonical_decimal(resistance).expect("finite axis value"),
                    target: Some(StepTargetDocument::Parameter {
                        name: "r".to_owned(),
                    }),
                },
                AxisAssignmentDocument {
                    kind: AxisAssignmentKind::Temperature,
                    name: "temperature".to_owned(),
                    value_index: (ordinal / resistances.len()) % temperatures.len(),
                    value_decimal: canonical_decimal(temperature).expect("finite axis value"),
                    target: Some(StepTargetDocument::Temperature),
                },
            ],
            analyses: vec![AnalysisExecution {
                analysis_id: "tran-001".to_owned(),
                output_namespace: OutputNamespaceDocument {
                    coordinate: namespace,
                    analysis: "tran-001".to_owned(),
                },
                artifacts: vec![ResultDocumentReference {
                    path: format!("results/coordinate-{ordinal}__tran-001.result.json"),
                    content_type: "application/vnd.rspice.analysis-result+json;version=1"
                        .to_owned(),
                    schema: "rspice-analysis-result".to_owned(),
                    schema_version: 1,
                    result_kind: "tran".to_owned(),
                }],
                measurements: Vec::new(),
            }],
        });
    }
    AxisExecutionDocument::new(AxisAnalysisKind::Transient, runs)
        .expect("a canonically enumerated manifest is valid")
}

fn invalid_document(result: Result<AxisExecutionDocument, AxisExecutionDocumentError>) -> bool {
    matches!(result, Err(AxisExecutionDocumentError::InvalidDocument(_)))
}

#[test]
fn law_a_valid_manifest_round_trips_through_its_wire_form() {
    runner(0x0A17_0001, 64)
        .run(&plan(), |plan| {
            let document = manifest(&plan);
            let value = document.to_value().expect("a valid manifest encodes");
            let decoded = AxisExecutionDocument::from_value(value.clone())
                .expect("its own wire form decodes");
            prop_assert_eq!(&decoded, &document);

            let json = serde_json::to_string(&value).expect("a JSON value renders");
            let decoded = AxisExecutionDocument::from_json_with_abort(
                &json,
                &rspice_core::abort_signal::NoAbort,
                MAX_ENGINE_RESULT_MANIFEST_BYTES as u64,
            )
            .expect("the rendered manifest decodes");
            prop_assert_eq!(decoded, document);
            Ok(())
        })
        .expect("the axis manifest is a lossless wire form of itself");
}

#[test]
fn law_every_coordinate_id_is_recomputed_from_its_assignments() {
    runner(0x0A17_0002, 64)
        .run(&plan(), |plan| {
            let document = manifest(&plan);
            prop_assert_eq!(
                document.coordinate_count,
                plan.resistances.len() * plan.temperatures.len()
            );
            for run in &document.runs {
                let assignments = run
                    .assignments
                    .iter()
                    .map(|assignment| {
                        (
                            match assignment.kind {
                                AxisAssignmentKind::Step => AxisKind::Step,
                                AxisAssignmentKind::Temperature => AxisKind::Temperature,
                            },
                            assignment.name.as_str(),
                            assignment
                                .value_decimal
                                .parse::<f64>()
                                .expect("a canonical decimal parses"),
                        )
                    })
                    .collect::<Vec<_>>();
                let expected = numeric_run_coordinate_id(&assignments, 0)
                    .expect("finite assignments have an identity")
                    .to_string();
                prop_assert_eq!(&run.coordinate_id, &expected);
                prop_assert_eq!(&run.coordinate_namespace, &format!("run-{expected}"));
            }
            Ok(())
        })
        .expect("a manifest coordinate ID is the planner's identity for its assignments");
}

#[test]
fn law_a_detached_coordinate_id_is_refused() {
    runner(0x0A17_0003, 64)
        .run(&(plan(), 0_usize..16), |(plan, pick)| {
            let mut document = manifest(&plan);
            let index = pick % document.runs.len();
            // Swap in a well-formed but foreign identity: the shape checks
            // pass, so only the recomputation can catch it.
            let foreign = "fedcba9876543210fedcba9876543210-001".to_owned();
            let namespace = format!("run-{foreign}");
            {
                let run = &mut document.runs[index];
                run.coordinate_id = foreign;
                run.coordinate_namespace = namespace.clone();
                for analysis in &mut run.analyses {
                    analysis.output_namespace.coordinate = namespace.clone();
                }
            }
            prop_assert!(
                matches!(
                    document.validate(),
                    Err(AxisExecutionDocumentError::InvalidDocument(_))
                ),
                "a coordinate ID detached from its assignments must be refused"
            );
            Ok(())
        })
        .expect("coordinate identity is verified, not trusted");
}

#[test]
fn law_an_incomplete_or_permuted_coordinate_set_is_refused() {
    runner(0x0A17_0004, 64)
        .run(&plan(), |plan| {
            let complete = manifest(&plan);
            if complete.runs.len() < 2 {
                return Ok(());
            }

            // Dropping the *first* coordinate leaves a set that is no longer a
            // complete product: its axis value indices no longer start at
            // zero. (Dropping the last one can leave a smaller but still
            // complete product, which is a different, legitimate plan.)
            let mut truncated = complete.clone();
            truncated.runs.remove(0);
            for (index, run) in truncated.runs.iter_mut().enumerate() {
                run.ordinal = index + 1;
            }
            prop_assert!(
                invalid_document(AxisExecutionDocument::new(
                    truncated.analysis_kind,
                    truncated.runs
                )),
                "a partial Cartesian product must be refused"
            );

            let mut permuted = complete.clone();
            permuted.runs.swap(0, 1);
            for (index, run) in permuted.runs.iter_mut().enumerate() {
                run.ordinal = index + 1;
            }
            prop_assert!(
                invalid_document(AxisExecutionDocument::new(
                    permuted.analysis_kind,
                    permuted.runs
                )),
                "coordinates out of canonical order must be refused"
            );

            let mut duplicated = complete;
            let borrowed = duplicated.runs[0].analyses[0].artifacts[0].path.clone();
            duplicated.runs[1].analyses[0].artifacts[0].path = borrowed;
            prop_assert!(
                invalid_document(AxisExecutionDocument::new(
                    duplicated.analysis_kind,
                    duplicated.runs
                )),
                "two coordinates must not publish the same artifact path"
            );
            Ok(())
        })
        .expect("the manifest is the complete Cartesian product in canonical order");
}

#[test]
fn law_a_manifest_past_its_byte_budget_is_a_typed_refusal() {
    runner(0x0A17_0005, 32)
        .run(&plan(), |plan| {
            let document = manifest(&plan);
            let json =
                serde_json::to_string(&document.to_value().expect("a valid manifest encodes"))
                    .expect("a JSON value renders");
            let budget = (json.len() - 1) as u64;
            prop_assert!(
                matches!(
                    AxisExecutionDocument::from_json_with_abort(
                        &json,
                        &rspice_core::abort_signal::NoAbort,
                        budget
                    ),
                    Err(AxisExecutionDocumentError::DocumentTooLarge { .. })
                ),
                "a manifest larger than its budget must be refused by size"
            );
            Ok(())
        })
        .expect("the manifest byte budget is enforced before decoding");
}
