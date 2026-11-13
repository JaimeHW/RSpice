//! Properties of the target-neutral execution contracts.
//!
//! Each test states one *law* — a claim that must hold for every input in a
//! generated family, not just for the hand-picked cases the unit tests carry.
//! The generators are driven by a fixed ChaCha seed and failure persistence is
//! disabled, so a run is reproducible from the source alone and CI never
//! writes a regression file.
//!
//! The laws checked here are:
//!
//! * run-axis cardinality is the product of the axis lengths, and the first
//!   axis varies fastest;
//! * a coordinate ID is a function of the coordinate's assignments alone, so
//!   replanning the same deck reproduces it and an external contract can
//!   recompute it with [`numeric_run_coordinate_id`];
//! * [`SignalSchema::union`] is idempotent, is commutative up to source
//!   bookkeeping, and never fabricates a value for a signal a coordinate did
//!   not produce;
//! * a bounded writer accepts exactly the declared budget and reports why it
//!   stopped.

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;

use proptest::prelude::*;
use proptest::test_runner::{Config, RngAlgorithm, TestRng, TestRunner};

use rspice_core::ResourceLimits;
use rspice_core::abort_signal::{ImmediateAbort, NoAbort};
use rspice_core::execution::bounded_io::{BoundedAbortWriter, BoundedWriteFailure};
use rspice_core::execution::{
    AxisKind, CoordinateSchema, DataBinding, DeckPlan, RunAxis, RunAxisValue, RunCoordinate,
    RunCoordinateId, SignalDescriptor, SignalKind, SignalOwner, SignalSchema, SignalShape,
    SignalUnit, SignalValueType, numeric_run_coordinate_id,
};

/// A deterministic runner: fixed algorithm, fixed seed, no persistence file.
///
/// Property tests are release gates here, so "it failed on the CI machine and
/// nowhere else" is not an acceptable outcome. Every case this crate explores
/// is a pure function of the seed written in the test.
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

//=============================================================================
// Run-axis generators
//=============================================================================

/// A finite, distinctly spelled axis name.
fn axis_name(prefix: &'static str) -> impl Strategy<Value = String> {
    (0_u32..6).prop_map(move |index| format!("{prefix}{index}"))
}

fn numeric_values() -> impl Strategy<Value = Vec<RunAxisValue>> {
    prop::collection::vec(-40.0_f64..40.0, 1..4).prop_map(|values| {
        // A `.STEP` sweep never repeats a value inside one axis; the planner
        // accepts repeats but the identity law under test is about distinct
        // points, so duplicates are removed rather than generated.
        let mut seen = BTreeSet::new();
        values
            .into_iter()
            .filter(|value| seen.insert(value.to_bits()))
            .map(RunAxisValue::Numeric)
            .collect()
    })
}

fn numeric_axis(kind: AxisKind, prefix: &'static str) -> impl Strategy<Value = RunAxis> {
    (axis_name(prefix), numeric_values())
        .prop_filter_map("axis values must not be empty after de-duplication", {
            move |(name, values)| RunAxis::new(kind, name, values).ok()
        })
}

fn data_axis() -> impl Strategy<Value = RunAxis> {
    (
        axis_name("tbl"),
        prop::collection::vec(prop::collection::vec(-9.0_f64..9.0, 1..3), 1..3),
    )
        .prop_filter_map(
            "DATA rows must be consistent and non-empty",
            |(name, rows)| {
                let width = rows.first()?.len();
                let values = rows
                    .into_iter()
                    .map(|row| {
                        if row.len() != width {
                            return None;
                        }
                        let bindings = row
                            .into_iter()
                            .enumerate()
                            .map(|(column, value)| {
                                DataBinding::new(format!("p{column}"), value).ok()
                            })
                            .collect::<Option<Vec<_>>>()?;
                        Some(RunAxisValue::DataRow(bindings))
                    })
                    .collect::<Option<Vec<_>>>()?;
                RunAxis::new(AxisKind::Data, name, values).ok()
            },
        )
}

fn alter_axis() -> impl Strategy<Value = RunAxis> {
    (axis_name("alt"), prop::collection::vec(1_u8..250, 1..3)).prop_filter_map(
        "ALTER variants need a non-empty label and a non-zero digest",
        |(name, seeds)| {
            let values = seeds
                .into_iter()
                .enumerate()
                .map(|(index, seed)| RunAxisValue::AlterVariant {
                    label: format!("variant{index}"),
                    materialization_digest: [seed; 32],
                })
                .collect();
            RunAxis::new(AxisKind::Alter, name, values).ok()
        },
    )
}

/// Axes in the declared order the planner requires: alter, data, step, temp.
fn axis_set() -> impl Strategy<Value = Vec<RunAxis>> {
    (
        prop::option::of(alter_axis()),
        prop::option::of(data_axis()),
        prop::option::of(numeric_axis(AxisKind::Step, "step")),
        prop::option::of(numeric_axis(AxisKind::Temperature, "temperature")),
    )
        .prop_map(|(alter, data, step, temperature)| {
            [alter, data, step, temperature]
                .into_iter()
                .flatten()
                .collect()
        })
}

/// Axes whose every value is numeric, so the coordinate identity can be
/// recomputed from an external typed contract.
fn numeric_axis_set() -> impl Strategy<Value = Vec<RunAxis>> {
    (
        prop::option::of(numeric_axis(AxisKind::Step, "step")),
        prop::option::of(numeric_axis(AxisKind::Temperature, "temperature")),
    )
        .prop_map(|(step, temperature)| [step, temperature].into_iter().flatten().collect())
}

fn coordinates(axes: Vec<RunAxis>) -> Vec<RunCoordinate> {
    DeckPlan::new(axes, Vec::new())
        .expect("generated axes satisfy the planner's ordering and binding rules")
        .coordinates_with_abort(&ResourceLimits::default(), &NoAbort)
        .expect("a generated plan stays far inside the default resource limits")
}

//=============================================================================
// Laws: run-axis cardinality and coordinate identity
//=============================================================================

#[test]
fn law_coordinate_count_is_the_product_of_axis_cardinalities() {
    runner(0x0C00_0001, 96)
        .run(&axis_set(), |axes| {
            let expected = axes
                .iter()
                .map(|axis| axis.values().len())
                .product::<usize>();
            let coordinates = coordinates(axes);
            prop_assert_eq!(coordinates.len(), expected);
            Ok(())
        })
        .expect("run-axis cardinality is the product of the axis lengths");
}

#[test]
fn law_coordinate_ordinals_are_dense_and_first_axis_varies_fastest() {
    runner(0x0C00_0002, 96)
        .run(&axis_set(), |axes| {
            let lengths = axes
                .iter()
                .map(|axis| axis.values().len())
                .collect::<Vec<_>>();
            let coordinates = coordinates(axes);
            for (ordinal, coordinate) in coordinates.iter().enumerate() {
                prop_assert_eq!(coordinate.ordinal(), ordinal);
                let mut stride = 1_usize;
                for (assignment, length) in coordinate.assignments().iter().zip(&lengths) {
                    prop_assert_eq!(assignment.value_index(), (ordinal / stride) % length);
                    stride *= length;
                }
            }
            Ok(())
        })
        .expect("coordinates are the dense first-axis-fastest Cartesian enumeration");
}

#[test]
fn law_coordinate_ids_depend_only_on_their_assignments() {
    runner(0x0C00_0003, 96)
        .run(&axis_set(), |axes| {
            let first = coordinates(axes.clone());
            let second = coordinates(axes);
            prop_assert_eq!(first.len(), second.len());
            for (left, right) in first.iter().zip(&second) {
                prop_assert_eq!(left.stable_id(), right.stable_id());
                prop_assert_eq!(left.stable_tag(), right.stable_tag());
            }

            // Distinct assignment tuples must not share an identity. Equal
            // semantic digests are allowed only when the occurrence counter
            // separates them, which is the planner's collision escape hatch.
            let mut by_id = BTreeMap::new();
            for coordinate in &first {
                let key = coordinate
                    .assignments()
                    .iter()
                    .map(|assignment| {
                        (
                            assignment.kind(),
                            assignment.name().to_ascii_lowercase(),
                            format!("{:?}", assignment.value()),
                        )
                    })
                    .collect::<Vec<_>>();
                prop_assert!(
                    by_id.insert(coordinate.stable_id(), key).is_none(),
                    "two coordinates shared one stable ID"
                );
            }
            Ok(())
        })
        .expect("a coordinate ID is a function of its assignments alone");
}

#[test]
fn law_numeric_coordinate_ids_are_recomputable_from_a_typed_contract() {
    runner(0x0C00_0004, 96)
        .run(&numeric_axis_set(), |axes| {
            let coordinates = coordinates(axes);
            let mut occurrences = BTreeMap::<[u8; 16], u32>::new();
            for coordinate in &coordinates {
                let assignments = coordinate
                    .assignments()
                    .iter()
                    .map(|assignment| {
                        let RunAxisValue::Numeric(value) = assignment.value() else {
                            unreachable!("numeric axes carry only numeric values")
                        };
                        (assignment.kind(), assignment.name(), *value)
                    })
                    .collect::<Vec<_>>();
                let semantic = numeric_run_coordinate_id(&assignments, 0)
                    .expect("finite numeric assignments have an identity")
                    .semantic_bytes();
                let occurrence = occurrences.entry(semantic).or_default();
                let recomputed = numeric_run_coordinate_id(&assignments, *occurrence)
                    .expect("finite numeric assignments have an identity");
                *occurrence += 1;
                prop_assert_eq!(recomputed, coordinate.stable_id());
            }
            Ok(())
        })
        .expect("an external contract recomputes the planner's numeric coordinate identity");
}

#[test]
fn law_an_aborted_plan_never_materializes_coordinates() {
    runner(0x0C00_0005, 48)
        .run(&axis_set(), |axes| {
            let plan =
                DeckPlan::new(axes, Vec::new()).expect("generated axes satisfy the planner rules");
            let refused = plan.coordinates_with_abort(&ResourceLimits::default(), &ImmediateAbort);
            prop_assert!(refused.is_err(), "an aborted plan produced coordinates");
            Ok(())
        })
        .expect("cancellation is observed before any coordinate is materialized");
}

//=============================================================================
// Laws: signal-schema union and alignment
//=============================================================================

fn descriptor(name: &str) -> SignalDescriptor {
    SignalDescriptor::new(
        format!("v({name})"),
        format!("V({name})"),
        SignalKind::Voltage,
        SignalUnit::Volt,
        SignalValueType::Real,
        SignalShape::Vector,
        SignalOwner::Node(name.to_owned()),
    )
    .expect("a node voltage descriptor is structurally valid")
}

fn schema() -> impl Strategy<Value = SignalSchema> {
    prop::collection::btree_set("n[0-9]", 1..6).prop_filter_map("schema must be constructible", {
        |names| SignalSchema::new(names.iter().map(|name| descriptor(name)).collect()).ok()
    })
}

/// Two distinct planner coordinate identities.
///
/// `RunCoordinateId` has no public constructor — an identity is minted by the
/// planner and nowhere else — so the schema-union laws borrow the two points
/// of a real two-value axis instead of fabricating IDs.
fn two_coordinate_ids() -> (RunCoordinateId, RunCoordinateId) {
    let axis = RunAxis::new(
        AxisKind::Step,
        "stepq",
        vec![RunAxisValue::Numeric(1.0), RunAxisValue::Numeric(2.0)],
    )
    .expect("a two-point numeric axis is valid");
    let coordinates = coordinates(vec![axis]);
    let first = coordinates[0].stable_id();
    let second = coordinates[1].stable_id();
    assert_ne!(first, second, "two axis points must have distinct IDs");
    (first, second)
}

#[test]
fn law_schema_union_with_itself_adds_nothing() {
    runner(0x0C00_0010, 96)
        .run(&schema(), |schema| {
            let (first, second) = two_coordinate_ids();
            let union = SignalSchema::union([
                CoordinateSchema::new(first, &schema),
                CoordinateSchema::new(second, &schema),
            ])
            .expect("a schema is compatible with itself");

            let mut expected = schema
                .descriptors()
                .iter()
                .map(|descriptor| descriptor.canonical_name().to_owned())
                .collect::<Vec<_>>();
            expected.sort();
            let actual = union
                .schema()
                .descriptors()
                .iter()
                .map(|descriptor| descriptor.canonical_name().to_owned())
                .collect::<Vec<_>>();
            prop_assert_eq!(&actual, &expected);

            // Idempotence: unioning the union back in changes nothing.
            let again = SignalSchema::union([
                CoordinateSchema::new(first, union.schema()),
                CoordinateSchema::new(second, &schema),
            ])
            .expect("the union is compatible with its own sources");
            prop_assert_eq!(again.schema(), union.schema());

            // Every union column is present in both sources.
            for indices in union.source_indices().values() {
                prop_assert!(indices.iter().all(Option::is_some));
            }
            Ok(())
        })
        .expect("union with an identical schema is the identity on the descriptor set");
}

#[test]
fn law_schema_union_is_commutative_and_never_fabricates_values() {
    runner(0x0C00_0011, 128)
        .run(&(schema(), schema()), |(left, right)| {
            let (left_id, right_id) = two_coordinate_ids();
            let forward = SignalSchema::union([
                CoordinateSchema::new(left_id, &left),
                CoordinateSchema::new(right_id, &right),
            ])
            .expect("two node-voltage schemas are always compatible");
            let backward = SignalSchema::union([
                CoordinateSchema::new(right_id, &right),
                CoordinateSchema::new(left_id, &left),
            ])
            .expect("two node-voltage schemas are always compatible");
            prop_assert_eq!(forward.schema(), backward.schema());
            prop_assert_eq!(forward.source_indices(), backward.source_indices());

            // Alignment carries a source's own values into the union columns
            // it owns and leaves every other column explicitly absent. A
            // fabricated `0.0` would show up here as a `Some`.
            for (id, schema) in [(left_id, &left), (right_id, &right)] {
                let values = (0..schema.descriptors().len())
                    .map(|index| index as f64 + 0.5)
                    .collect::<Vec<_>>();
                let aligned = forward
                    .align_values(id, &values)
                    .expect("a source aligns against the union it belongs to");
                prop_assert_eq!(aligned.len(), forward.schema().descriptors().len());
                let owned = schema
                    .descriptors()
                    .iter()
                    .map(|descriptor| descriptor.canonical_name().to_owned())
                    .collect::<BTreeSet<_>>();
                for (column, sample) in aligned.iter().enumerate() {
                    let name = forward.schema().descriptors()[column].canonical_name();
                    prop_assert_eq!(sample.is_some(), owned.contains(name));
                }
                let present = aligned.iter().flatten().copied().collect::<Vec<_>>();
                prop_assert_eq!(present, values);
            }
            Ok(())
        })
        .expect("union is commutative and alignment never invents a sample");
}

#[test]
fn law_alignment_refuses_a_source_of_the_wrong_width() {
    runner(0x0C00_0012, 64)
        .run(&(schema(), 0_usize..4), |(schema, extra)| {
            let (id, _) = two_coordinate_ids();
            let union = SignalSchema::union([CoordinateSchema::new(id, &schema)])
                .expect("a single-source union is always valid");
            let wrong_width = vec![1.0_f64; schema.descriptors().len() + extra + 1];
            prop_assert!(union.align_values(id, &wrong_width).is_err());
            Ok(())
        })
        .expect("alignment refuses a source value count the schema does not describe");
}

//=============================================================================
// Laws: bounded, cancellable serialization sinks
//=============================================================================

#[test]
fn law_bounded_writer_accepts_exactly_the_declared_budget() {
    let chunks = prop::collection::vec(prop::collection::vec(any::<u8>(), 0..24), 1..12);
    runner(0x0C00_0020, 192)
        .run(&(chunks, 0_u64..96), |(chunks, limit)| {
            let mut writer = BoundedAbortWriter::new(&NoAbort, limit);
            let mut accepted = Vec::new();
            let mut refused = false;
            for chunk in &chunks {
                match writer.write(chunk) {
                    Ok(count) => {
                        prop_assert_eq!(count, chunk.len(), "a bounded writer never writes short");
                        accepted.extend_from_slice(chunk);
                    }
                    Err(_) => {
                        refused = true;
                        break;
                    }
                }
            }
            prop_assert_eq!(writer.byte_limit(), limit);
            prop_assert_eq!(writer.len(), accepted.len());
            prop_assert!(accepted.len() as u64 <= limit);
            if refused {
                prop_assert_eq!(
                    writer.failure(),
                    Some(BoundedWriteFailure::ByteLimitExceeded { limit_bytes: limit })
                );
            } else {
                prop_assert_eq!(writer.failure(), None);
            }
            prop_assert_eq!(writer.into_bytes(), accepted);
            Ok(())
        })
        .expect("a bounded writer accepts a prefix of its input up to the declared limit");
}

#[test]
fn law_bounded_writer_reports_cancellation_before_accepting_a_byte() {
    let chunks = prop::collection::vec(prop::collection::vec(any::<u8>(), 1..16), 1..6);
    runner(0x0C00_0021, 96)
        .run(&chunks, |chunks| {
            let mut writer = BoundedAbortWriter::new(&ImmediateAbort, u64::MAX);
            for chunk in &chunks {
                prop_assert!(writer.write(chunk).is_err());
            }
            prop_assert_eq!(writer.failure(), Some(BoundedWriteFailure::Aborted));
            prop_assert!(writer.is_empty());
            Ok(())
        })
        .expect("cancellation is reported before any byte is accepted");
}
