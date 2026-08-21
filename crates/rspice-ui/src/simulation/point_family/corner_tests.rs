//! What a corner declaration's family says about the points it declared.

use super::*;
use crate::services::simulation_runner::{
    CornerBaseMode, CornerFrequencySweep, CornerProcess, CornerRunConfig,
};
use crate::state::{AnalysisResultFamilyMetadata, AnalysisType};

/// A 1:1 resistive divider on a supply the corner axis derates by 10%, so
/// every reduction below has an answer that can be written down: the output
/// is half the supply its point was solved at.
const DIVIDER: &str = "corner family\n\
     VDD vdd 0 DC 1.8 AC 1\n\
     R1 vdd out 1k\n\
     R2 out 0 1k\n\
     C1 out 0 1p\n\
     .op\n\
     .end\n";

fn contract(base_mode: CornerBaseMode) -> CornerRunConfig {
    CornerRunConfig {
        process_corners: vec![CornerProcess::TT],
        voltages: vec![1.8, 1.62],
        supply_source_names: vec!["VDD".to_owned()],
        temperatures_c: vec![27.0, 85.0],
        full_matrix: true,
        nominal_voltage: Some(1.8),
        base_mode,
        model_bindings: Vec::new(),
        points: Vec::new(),
    }
}

/// The declaration's family and its traces, sorted by trace name.
fn family(base_mode: CornerBaseMode) -> (AnalysisResultFamilyMetadata, Vec<(String, Vec<f64>)>) {
    let run = crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
        DIVIDER,
        contract(base_mode),
        27.0,
    )
    .expect("the corner declaration prepares, authorizes and runs");
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Corner)
        .expect("the declaration produces a corner family");
    assert!(
        analysis.success,
        "corner family failed: {:?}",
        analysis.error_message
    );
    let mut traces = analysis
        .waveforms
        .iter()
        .map(|waveform| (waveform.name.clone(), waveform.y.as_ref().clone()))
        .collect::<Vec<_>>();
    traces.sort_by(|left, right| left.0.cmp(&right.0));
    (
        analysis
            .family_metadata
            .clone()
            .expect("a corner family carries its axis"),
        traces,
    )
}

fn assert_close(actual: &[f64], expected: &[f64], what: &str) {
    assert_eq!(actual.len(), expected.len(), "{what}: {actual:?}");
    for (index, (actual, expected)) in actual.iter().zip(expected).enumerate() {
        assert!(
            (actual - expected).abs() < 1.0e-6,
            "{what} point {index}: expected {expected}, got {actual}"
        );
    }
}

fn trace_names(traces: &[(String, Vec<f64>)]) -> Vec<&str> {
    traces.iter().map(|(name, _)| name.as_str()).collect()
}

/// The four declared points share a process and differ in supply, so the
/// axis cannot be temperature and is the corner index instead.
#[test]
fn the_axis_is_the_declared_space_itself() {
    let (metadata, _) = family(CornerBaseMode::Op);

    assert_eq!(
        metadata,
        AnalysisResultFamilyMetadata::Corner {
            member_measurements: Vec::new(),
            x_values: vec![0.0, 1.0, 2.0, 3.0],
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: vec![27.0, 85.0, 27.0, 85.0],
            corner_labels: vec![
                "TT_1.800000V_27.000000C".to_owned(),
                "TT_1.800000V_85.000000C".to_owned(),
                "TT_1.620000V_27.000000C".to_owned(),
                "TT_1.620000V_85.000000C".to_owned(),
            ],
            failed_corners: 0,
        }
    );
}

#[test]
fn an_operating_point_corner_contributes_its_converged_node_voltages() {
    let (_, traces) = family(CornerBaseMode::Op);

    assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
    assert_close(&traces[0].1, &[0.9, 0.9, 0.81, 0.81], "V(OUT)");
    assert_close(&traces[1].1, &[1.8, 1.8, 1.62, 1.62], "V(VDD)");
}

/// The sweep drives the same supply the corner derates, so the terminal
/// swept point overrides the corner value and every point answers alike.
/// That is exactly the reduction under test: the last swept point.
#[test]
fn a_dc_sweep_corner_contributes_its_terminal_swept_point() {
    let (_, traces) = family(CornerBaseMode::DcSweep {
        source_name: "VDD".to_owned(),
        start: 0.0,
        stop: 1.0,
        step: 0.5,
    });

    assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
    assert_close(&traces[0].1, &[0.5, 0.5, 0.5, 0.5], "V(OUT)");
    assert_close(&traces[1].1, &[1.0, 1.0, 1.0, 1.0], "V(VDD)");
}

/// The divider settles long before 100 ns, so the terminal sample is the
/// DC answer. Both nodes appear: the corner executor used to drop whichever
/// node a transient result listed first, because it skipped index zero on
/// the one base mode whose node vector carries no ground.
#[test]
fn a_transient_corner_contributes_its_terminal_sample_for_every_node() {
    let (_, traces) = family(CornerBaseMode::Transient {
        stop_time: 100.0e-9,
        step_time: 1.0e-9,
    });

    assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
    assert_close(&traces[0].1, &[0.9, 0.9, 0.81, 0.81], "V(OUT)");
    assert_close(&traces[1].1, &[1.8, 1.8, 1.62, 1.62], "V(VDD)");
}

/// AC contributes magnitude at the terminal frequency and names its traces
/// as magnitudes. The terminal frequency is what makes this assertable: the
/// divider is loaded by 1 pF against 500 ohm of source resistance, so the
/// answer at 1 MHz is the divider ratio rolled off by that pole and nothing
/// like the answer at 1 kHz. The supply corner scales DC values and leaves
/// the AC drive alone, so every point reads the same.
#[test]
fn an_ac_corner_contributes_terminal_frequency_magnitude() {
    let terminal = 2.0 * std::f64::consts::PI * 1.0e6 * 500.0 * 1.0e-12;
    let rolled_off = 0.5 / (1.0 + terminal * terminal).sqrt();

    let (_, traces) = family(CornerBaseMode::Ac {
        start_freq: 1.0e3,
        stop_freq: 1.0e6,
        points_per_unit: 5,
        sweep: CornerFrequencySweep::Decade,
    });

    assert_eq!(trace_names(&traces), vec!["|V(OUT)|", "|V(VDD)|"]);
    assert_close(&traces[0].1, &[rolled_off; 4], "|V(OUT)|");
    assert_close(&traces[1].1, &[1.0, 1.0, 1.0, 1.0], "|V(VDD)|");
    assert!(
        (traces[0].1[0] - 0.5).abs() > 1.0e-9,
        "the terminal frequency must not read like the first one"
    );
}

/// Four declared points, four solves, and a family that costs no solve of
/// its own — it carries no PVT point because it did not solve one.
#[test]
fn the_family_is_assembled_from_the_points_rather_than_solved_again() {
    let run = crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
        DIVIDER,
        contract(CornerBaseMode::Op),
        27.0,
    )
    .expect("the corner declaration prepares, authorizes and runs");

    let attributed = run
        .analyses
        .iter()
        .filter(|analysis| {
            analysis
                .provenance
                .as_ref()
                .and_then(crate::state::AnalysisResultProvenance::pvt_point)
                .is_some()
        })
        .count();
    assert_eq!(attributed, 4, "one solve per declared point, and no more");
    assert_eq!(
        run.analyses.len(),
        5,
        "the four points and the one family they add up to"
    );
    let families = run
        .analyses
        .iter()
        .filter(|analysis| analysis.analysis_type == AnalysisType::Corner)
        .collect::<Vec<_>>();
    assert_eq!(families.len(), 1);
    assert!(
        families[0]
            .provenance
            .as_ref()
            .expect("the family answers for the declaration")
            .pvt_point()
            .is_none(),
        "the family is not a point and must never claim one"
    );
}

/// A declaration none of whose points converged is a failed corner result,
/// not a missing one.
#[test]
fn a_declaration_with_no_converged_point_is_a_failure() {
    let outcome = corner_family_of_points(&CornerBaseMode::Op, 3, &[]);

    assert_eq!(
        outcome.expect_err("nothing converged"),
        "Corner analysis produced no converged corner points"
    );
}

fn corner_run() -> crate::state::SimulationRun {
    crate::simulation::runner::pvt_point_evidence::run_corner_declaration(
        DIVIDER,
        contract(CornerBaseMode::Op),
        27.0,
    )
    .expect("the corner declaration prepares, authorizes and runs")
}

/// A run's retained results have to be an exact ordered prefix of the task
/// graph its receipt authenticated, and that is checked on project load as a
/// hard error rather than a warning. The family is a result, so it has to be
/// a task: while it was assembled outside the task list, a saved corner run
/// carried one more result than it had authenticated tasks and the project
/// refused to reopen.
#[test]
fn a_corner_run_is_an_authentic_prefix_of_its_receipt_and_survives_a_project_round_trip() {
    use crate::io::project_io::ProjectSimulationResults;

    let run = corner_run();
    run.validate_provenance()
        .expect("every retained result answers for an authenticated task");
    assert_eq!(
        run.prepared_receipt()
            .expect("the run is sealed by its dispatch")
            .tasks()
            .len(),
        run.analyses.len(),
        "four points and the family they add up to, against five authenticated tasks"
    );

    // The exact shape that broke project load: one result more than the
    // receipt authenticates is refused outright, and a family assembled
    // outside the task list is precisely that result.
    let mut surplus = run.clone();
    let duplicate = surplus.analyses[0].clone();
    surplus.analyses.push(duplicate);
    surplus
        .validate_provenance()
        .expect_err("a result with no authenticated task cannot be retained");

    let mut simulation = crate::state::SimulationState::default();
    simulation.next_run_id = run.id;
    simulation.runs = vec![run];

    let persisted = ProjectSimulationResults::from_state(&simulation);
    persisted.validate().expect("a corner run is persistable");

    let mut reloaded = crate::state::SimulationState::default();
    persisted
        .apply_to_state(&mut reloaded)
        .expect("a saved corner run reopens");

    let restored = reloaded.runs.first().expect("the run survives the reload");
    assert_eq!(restored.analyses.len(), simulation.runs[0].analyses.len());
    assert!(
        restored
            .analyses
            .iter()
            .any(|analysis| analysis.analysis_type == AnalysisType::Corner
                && matches!(
                    analysis.family_metadata,
                    Some(AnalysisResultFamilyMetadata::Corner { .. })
                )),
        "the corner plot's axis survives the round trip"
    );
    restored
        .validate_provenance()
        .expect("the reloaded run is still an authentic prefix");
}

/// A run that stopped part-way keeps the same property. This is why the
/// assembly is ordered last: every truncation of the result list is still a
/// prefix of the task list, whereas an assembly ordered first could not
/// produce its result until its points had and would leave a hole.
#[test]
fn a_corner_run_that_stopped_part_way_is_still_an_authentic_prefix() {
    let complete = corner_run();
    assert_eq!(complete.analyses.len(), 5);

    for retained in 0..complete.analyses.len() {
        let mut partial = complete.clone();
        partial.analyses.truncate(retained);
        partial.validate_provenance().unwrap_or_else(|error| {
            panic!("a run that stopped after {retained} result(s) must still validate: {error}")
        });
    }
}
