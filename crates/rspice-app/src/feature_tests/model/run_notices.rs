//! `workbench/app_state/diagnostic_store/run_notices` observed through a whole session.

use crate::diagnostics::LogAnchor;
use crate::state::CellViewRef;
use crate::state::{
    Cell, Component, ComponentType, Library, LibraryCellInstance, Point, SchematicState, View,
    ViewType,
};
use crate::workbench::app_state::AppState;
use crate::workbench::app_state::diagnostic_store::DiagnosticInputs;
use crate::workbench::app_state::diagnostic_store::DiagnosticLevel;
use crate::workbench::app_state::diagnostic_store::DiagnosticRecord;
use crate::workbench::app_state::diagnostic_store::DiagnosticRoute;
use crate::workbench::app_state::diagnostic_store::run_notices::*;
use crate::workbench::app_state::diagnostic_store::{BatchStanding, listing};
use rspice_core::ContinuationAid;
use rspice_core::notice::BoundarySupplyKind;
use rspice_core::notice::EngineNotice;
use rspice_core::notice::ParameterReplacement;
use rspice_core::notice::ParameterScope;
use rspice_core::notice::RecordedNotice;
use rspice_results::convergence_quality::TransientConvergenceEvidence;
use std::hash::Hash;

fn open_state(runs: &[u64]) -> AppState {
    let mut state = AppState::default();
    state.open_project_session();
    state.model.simulation.retained.runs = runs
        .iter()
        .map(|run| crate::state::SimulationRun::new(*run))
        .collect::<Vec<_>>()
        .into();
    state
}

/// What identifies `point` in a record. Any digest the records of one run
/// share will do, as the collector's do.
fn identity(point: &str) -> u64 {
    use std::hash::Hasher;
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    point.hash(&mut hasher);
    hasher.finish()
}

/// `parameter` of `element` replaced at each of `points`.
fn clamp(element: &str, parameter: &str, points: &[&str]) -> RecordedNotice {
    RecordedNotice {
        notice: EngineNotice::ParameterReplaced(ParameterReplacement {
            family: "BSIM4".to_owned(),
            model: Some("nch".to_owned()),
            element: Some(element.to_owned()),
            scope: ParameterScope::Instance,
            parameter: parameter.to_owned(),
            requested: -5.0,
            applied: 0.0,
            limit: Some(0.0),
        }),
        occurrences: points.len().max(1),
        points: points.iter().map(|point| (*point).to_owned()).collect(),
        point_identities: points.iter().map(|point| identity(point)).collect(),
        line: None,
    }
}

/// `notices`, which no line of the run's log states.
fn unstated(
    notices: impl IntoIterator<Item = RecordedNotice>,
) -> Vec<(RecordedNotice, Option<usize>)> {
    notices.into_iter().map(|notice| (notice, None)).collect()
}

/// The notice records the run's batch holds, as (code, subject, details).
fn notices(state: &AppState) -> Vec<(String, String, String)> {
    state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .filter(|record| is_notice(record))
        .map(|record| {
            (
                record.canonical.code.to_string(),
                record.subject.object.clone(),
                record.details.clone(),
            )
        })
        .collect()
}

/// The next run of the design is a new measurement of it: what the last
/// run raised stops standing when it is dispatched, and the new run's
/// notices stand in its place.
#[test]
fn a_rerun_replaces_the_runs_warnings() {
    let mut state = open_state(&[1, 2]);
    state.open_run_outcome(1);
    state.publish_run_notices(
        "Transient",
        unstated([clamp("M1", "rdsw", &[])]),
        None,
        &[],
        1,
    );
    assert_eq!(notices(&state).len(), 1);

    state.model.supersede_run_diagnostics(2);
    state.open_run_outcome(2);
    state.publish_run_notices(
        "Transient",
        unstated([clamp("M2", "rdsw", &[])]),
        None,
        &[],
        2,
    );

    let standing = notices(&state);
    assert_eq!(standing.len(), 1, "{standing:?}");
    assert_eq!(standing[0].1, "M2", "the rerun's own warning stands");
}

/// An edit to the design the run was dispatched against does not take
/// its warnings away: they say the run was measured on another design.
#[test]
fn an_edit_stales_the_runs_warnings_rather_than_withdrawing_them() {
    let mut state = open_state(&[1]);
    state.open_run_outcome(1);
    state.publish_run_notices(
        "Transient",
        unstated([clamp("M1", "rdsw", &[])]),
        None,
        &[],
        1,
    );
    let batch = state
        .model
        .diagnostics
        .batch(&DiagnosticInputs::Run { run_sequence: 1 })
        .expect("the run's batch")
        .clone();
    assert!(state.diagnostic_batch_standing(&batch).is_current());

    state
        .model
        .schematic
        .add_component(ComponentType::Resistor, Point::new(0, 0));

    assert_eq!(
        state.diagnostic_batch_standing(&batch),
        BatchStanding::Stale
    );
    assert_eq!(notices(&state).len(), 1, "the warning is kept, stale");
    let listed = listing::listing(&state);
    assert!(listed.counts().stale > 0, "{:?}", listed.counts());
}

/// A clamp raised at every point of a sweep and in every analysis of the
/// run is one record that counts them and names where, not a row per
/// point; a second parameter is a second condition.
#[test]
fn a_sweep_raises_one_record_per_condition_with_the_points_it_occurred_at() {
    let mut state = open_state(&[1]);
    state.open_run_outcome(1);
    let trials: Vec<String> = (1..=10).map(|trial| format!("trial {trial}")).collect();
    let trials: Vec<&str> = trials.iter().map(String::as_str).collect();
    state.publish_run_notices(
        "Monte Carlo",
        unstated([
            clamp("M1", "rdsw", &trials),
            RecordedNotice {
                notice: EngineNotice::ConvergenceAid(ContinuationAid::GminStepping),
                occurrences: 3,
                points: Vec::new(),
                point_identities: Vec::new(),
                line: None,
            },
        ]),
        None,
        &[],
        1,
    );
    state.publish_run_notices(
        "Transient",
        unstated([clamp("m1", "rdsw", &[]), clamp("M1", "prwg", &[])]),
        None,
        &[],
        1,
    );

    let standing = notices(&state);
    assert_eq!(standing.len(), 3, "{standing:#?}");
    let (code, subject, details) = &standing[0];
    assert_eq!(code, PARAMETER_CODE);
    assert_eq!(subject, "M1");
    assert_eq!(
        details,
        "Raised 11 times in Monte Carlo, Transient · at trial 1, trial 2, trial 3, trial 4, \
         trial 5, trial 6, trial 7, trial 8 and 2 more · engine element M1"
    );
    assert_eq!(standing[1].0, AID_CODE);
    assert_eq!(standing[2].0, PARAMETER_CODE);
    let listed = listing::listing(&state);
    assert_eq!(listed.len(), 3);
    assert_eq!(listed.counts().warnings, 2, "{:?}", listed.counts());
}

/// `parameter` of `element` replaced at each of `points`, in order, as
/// one analysis's collector delivers it: the first eight named, and every
/// one identified.
fn delivered(element: &str, parameter: &str, points: &[String]) -> RecordedNotice {
    let named: Vec<&str> = points.iter().take(8).map(String::as_str).collect();
    let mut notice = clamp(element, parameter, &named);
    notice.occurrences = points.len();
    notice.point_identities = points.iter().map(|point| identity(point)).collect();
    notice
}

/// The samples `first..=last`.
fn samples(first: usize, last: usize) -> Vec<String> {
    (first..=last)
        .map(|sample| format!("sample {sample}"))
        .collect()
}

/// Points are counted once however many analyses raised the condition at
/// them, whether the record names them or counts them. Two Monte Carlo
/// analyses that clamp at samples 1 to 12 clamp at twelve samples, not
/// sixteen; one at 1 to 12 and one at 9 to 16, at sixteen, not twenty.
#[test]
fn a_point_two_analyses_raise_a_condition_at_is_counted_once() {
    let named = samples(1, 8).join(", ");
    for (second, raised, more) in [(samples(1, 12), 24, 4), (samples(9, 16), 20, 8)] {
        let mut state = open_state(&[1]);
        state.open_run_outcome(1);
        state.publish_run_notices(
            "Monte Carlo A",
            unstated([delivered("M1", "rdsw", &samples(1, 12))]),
            None,
            &[],
            1,
        );
        state.publish_run_notices(
            "Monte Carlo B",
            unstated([delivered("M1", "rdsw", &second)]),
            None,
            &[],
            1,
        );

        let standing = notices(&state);
        assert_eq!(standing.len(), 1, "{standing:#?}");
        assert_eq!(
            standing[0].2,
            format!(
                "Raised {raised} times in Monte Carlo A, Monte Carlo B · at {named} and \
                 {more} more · engine element M1"
            )
        );
    }
}

/// A boundary whose supply defaulted is one warning naming its net,
/// however many analyses of the run built the circuit and raised it.
#[test]
fn a_defaulted_boundary_supply_is_one_warning_naming_the_net() {
    let mut state = open_state(&[1]);
    state.open_run_outcome(1);
    let defaulted = || RecordedNotice {
        notice: EngineNotice::BoundarySupply {
            boundary: "dout".to_owned(),
            kind: BoundarySupplyKind::Defaulted,
            level: 3.3,
            from: None,
        },
        occurrences: 1,
        points: Vec::new(),
        point_identities: Vec::new(),
        line: None,
    };
    state.publish_run_notices("Operating Point", unstated([defaulted()]), None, &[], 1);
    state.publish_run_notices("Transient", unstated([defaulted()]), None, &[], 1);

    let standing = notices(&state);
    assert_eq!(standing.len(), 1, "{standing:#?}");
    let (code, subject, _) = &standing[0];
    assert_eq!(code, SUPPLY_DEFAULTED_CODE);
    assert_eq!(subject, "dout");
    let listed = listing::listing(&state);
    assert_eq!(listed.counts().warnings, 1, "{:?}", listed.counts());
}

/// A `.MEASURE` that produced no value is a warning about the run, in
/// the engine's words; one that produced a value is not.
#[test]
fn a_measurement_that_never_triggered_is_a_warning() {
    let mut state = open_state(&[1]);
    state.open_run_outcome(1);
    let failed = rspice_core::MeasureResult::failed("tdelay", "TRIG never crossed 0.5");
    let mut measured = rspice_core::MeasureResult::failed("tpeak", "");
    measured.error = None;
    measured.value = Some(1.0);
    state.publish_run_notices("Transient", Vec::new(), None, &[failed, measured], 1);

    let standing = notices(&state);
    assert_eq!(standing.len(), 1, "{standing:?}");
    assert_eq!(standing[0].0, MEASURE_CODE);
    assert_eq!(standing[0].1, "tdelay");
}

/// A clamp states the bound the value crossed; a reset with no bound, A1
/// zeroed because A2 was out of range, says only that the value was
/// replaced, because its own value was not out of anything.
#[test]
fn a_replacement_states_its_bound_only_where_the_model_states_one() {
    let replacement = |parameter: &str, requested: f64, applied: f64, limit: Option<f64>| {
        replacement_message(&ParameterReplacement {
            family: "BSIM3".to_owned(),
            model: Some("nch".to_owned()),
            element: Some("M1".to_owned()),
            scope: ParameterScope::Instance,
            parameter: parameter.to_owned(),
            requested,
            applied,
            limit,
        })
    };
    assert_eq!(
        replacement("a2", 2.0, 1.0, Some(1.0)),
        "BSIM3 a2 = 2 is above 1; the model uses 1"
    );
    assert_eq!(
        replacement("a1", 0.5, 0.0, None),
        "BSIM3 a1 = 500m is replaced; the model uses 0"
    );
}

/// The convergence evidence of a transient whose stepper accepted the
/// points at `accepted` (ascending indices into a 1 ns grid) past the
/// truncation-error test.
fn transient_accepting(accepted: &[usize]) -> TransientConvergenceEvidence {
    let mut quality = rspice_core::diagnostics::ConvergenceQuality::default();
    for &index in accepted {
        quality.record_force_accept(index);
    }
    let time: Vec<f64> = (0..10).map(|step| f64::from(step) * 1e-9).collect();
    TransientConvergenceEvidence::capture(quality, &time, &rspice_core::abort_signal::NoAbort)
        .expect("the fixture's evidence captures")
}

/// Points a transient accepted past its truncation-error tolerance are a
/// warning about the run's accuracy, counted over every transient that
/// accepted any and dated by the first; a transient that accepted none
/// raises nothing.
#[test]
fn points_accepted_past_the_truncation_tolerance_are_one_warning_for_the_run() {
    let mut state = open_state(&[1]);
    state.open_run_outcome(1);
    state.publish_run_notices(
        "Transient",
        Vec::new(),
        Some((&transient_accepting(&[]), None)),
        &[],
        1,
    );
    assert!(notices(&state).is_empty(), "nothing was accepted past it");

    state.publish_run_notices(
        "Transient",
        Vec::new(),
        Some((&transient_accepting(&[3, 7]), Some(4))),
        &[],
        1,
    );
    state.publish_run_notices(
        "Transient 2",
        Vec::new(),
        Some((&transient_accepting(&[5]), Some(9))),
        &[],
        1,
    );

    let records: Vec<&DiagnosticRecord> = state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .filter(|record| is_notice(record))
        .collect();
    assert_eq!(records.len(), 1, "{records:#?}");
    let record = records[0];
    assert_eq!(record.canonical.code.as_ref(), TRUNCATION_CODE);
    assert_eq!(record.level, DiagnosticLevel::Warning);
    assert_eq!(record.subject.object, "Transient, Transient 2");
    assert_eq!(
        record.message,
        "Time points were accepted past the local truncation-error tolerance, the first near \
         t = 3ns"
    );
    assert_eq!(record.details, "Raised 3 times in Transient, Transient 2");
    assert_eq!(
        record.log_place,
        Some(crate::workbench::app_state::diagnostic_store::RunLogPlace {
            run_sequence: 1,
            line: 4
        }),
        "the first line of the run's log that says so"
    );
}

/// A root drawing that places `amp` twice, as `amp` and `amp2`, and a
/// memristor, over an `amp` master that draws `M1` and a memristor of its
/// own.
fn hierarchical_design() -> AppState {
    let mut state = open_state(&[1]);
    let mut amp = SchematicState::default();
    amp.document_mut_for_test().components.extend([
        Component::new(10, ComponentType::Nmos, Point::new(40, 60)).with_name_value("M1", "nch"),
        Component::new(11, ComponentType::Memristor, Point::new(80, 60))
            .with_name_value("MR1", "1"),
    ]);
    let mut work = Library::new("work");
    let mut cell = Cell::new("amp");
    cell.add_view(View::new("schematic", ViewType::Schematic));
    work.add_cell(cell);
    state.model.library_manager.add_library(work);
    state
        .model
        .workspace
        .insert_schematic_editor(CellViewRef::new("work", "amp", "schematic").key(), amp);
    let active = state.model.workspace.content.active_view.clone();
    if state
        .model
        .library_manager
        .get_library(&active.library)
        .is_none()
    {
        let mut top = Library::new(&active.library);
        let mut top_cell = Cell::new(&active.cell);
        top_cell.add_view(View::new(&active.view, ViewType::Schematic));
        top.add_cell(top_cell);
        state.model.library_manager.add_library(top);
    }
    let placed = |id: u64, name: &str, x: i32| {
        let mut instance = Component::new(id, ComponentType::CellInstance, Point::new(x, 0))
            .with_library_cell(LibraryCellInstance::new("work", "amp", "schematic"));
        instance.name = name.to_owned();
        instance
    };
    state
        .model
        .schematic
        .document_mut_for_test()
        .components
        .extend([
            placed(1, "amp", 0),
            placed(3, "amp2", 400),
            Component::new(2, ComponentType::Memristor, Point::new(200, 60))
                .with_name_value("MR1", "1"),
        ]);
    state.sync_active_schematic_to_workspace();
    state
}

/// The engine names an element inside a sub-cell by the written names
/// above it; the record finds the component the `amp` master draws, and
/// routes to it in that master rather than in the root. The master is
/// placed twice, so each occurrence's clamp is its own record, named by
/// its occurrence, and both lead to the one `M1` the master draws.
#[test]
fn an_engine_name_in_a_sub_cell_locates_the_drawn_instance_under_its_occurrence() {
    let mut state = hierarchical_design();
    let amp = CellViewRef::new("work", "amp", "schematic");

    let located = state
        .model
        .locate_engine_element("xamp.m1")
        .expect("XAMP.M1 is the M1 that /amp draws");
    assert_eq!(located.occurrence.to_string(), "/amp");
    assert_eq!(located.cell_view.key(), amp.key());
    assert_eq!(located.component, 10);
    assert_eq!(located.name, "M1");
    assert_eq!(
        state
            .model
            .locate_engine_element("XAMP2.M1")
            .map(|located| (located.occurrence.to_string(), located.component)),
        Some(("/amp2".to_owned(), 10))
    );
    assert_eq!(
        state.model.locate_engine_element("m1"),
        None,
        "no M1 at the root"
    );

    state.open_run_outcome(1);
    state.publish_run_notices(
        "Operating point",
        unstated([
            clamp("XAMP.M1", "rdsw", &[]),
            clamp("XAMP2.M1", "rdsw", &[]),
        ]),
        None,
        &[],
        1,
    );
    let records: Vec<&DiagnosticRecord> = state
        .model
        .diagnostics
        .batches()
        .flat_map(|batch| batch.records.iter())
        .filter(|record| is_notice(record))
        .collect();
    let subjects: Vec<&str> = records
        .iter()
        .map(|record| record.subject.object.as_str())
        .collect();
    assert_eq!(subjects, ["/amp/M1", "/amp2/M1"]);
    for record in records {
        assert_eq!(
            record.subject.cell_view.as_ref().map(CellViewRef::key),
            Some(amp.key())
        );
        assert!(
            matches!(
                &record.route,
                Some(DiagnosticRoute::Anchor(LogAnchor::Schematic {
                    cell_view,
                    component: Some(10),
                    ..
                })) if cell_view.key() == amp.key()
            ),
            "{:?}",
            record.route
        );
    }
}

/// A memristor is known to the engine under its keyword, `YMEMRISTOR!MR1`,
/// and `MR1` alone would name a MOSFET; the drawn `MR1` is found either
/// way it is reached.
#[test]
fn a_memristor_is_located_by_the_name_the_engine_knows_it_by() {
    let state = hierarchical_design();
    assert_eq!(
        state
            .model
            .locate_engine_element("YMEMRISTOR!MR1")
            .map(|located| located.component),
        Some(2)
    );
    assert_eq!(
        state
            .model
            .locate_engine_element("xamp.ymemristor!mr1")
            .map(|located| located.component),
        Some(11)
    );
    assert_eq!(state.model.locate_engine_element("MR1"), None);
}
