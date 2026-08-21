//! What a temperature step's family says about the points it declared.

use super::*;
use crate::services::simulation_runner::{CornerBaseMode, CornerFrequencySweep, TempRunConfig};
use crate::state::{AnalysisResultFamilyMetadata, AnalysisType};

/// A divider whose upper leg carries a linear temperature coefficient, so the
/// axis is the only thing that moves the answer and the answer can be written
/// down: `R1` is 1 kΩ at 27 °C and 1 % per degree either side of it, so
/// `V(out)` falls monotonically as the deck heats up while the supply stands
/// still.
///
/// The `.step temp` card is there because a planned run really does carry one:
/// the declaration's own analysis line is written into the run-level deck
/// before any point exists. A point's temperature reaches the engine as the
/// `.OPTIONS TEMP` card spliced after it, so if the standing card were the one
/// being honoured every point below would read alike.
const DERATED_DIVIDER: &str = "temperature family\n\
     VDD vdd 0 DC 1.8 AC 1\n\
     R1 vdd out 1k TC1=0.01\n\
     R2 out 0 1k\n\
     C1 out 0 1p\n\
     .step temp list -40 27 125\n\
     .op\n\
     .end\n";

const DECLARED: [f64; 3] = [-40.0, 27.0, 125.0];

fn contract(base_mode: CornerBaseMode) -> TempRunConfig {
    TempRunConfig {
        temperatures_c: DECLARED.to_vec(),
        base_mode,
    }
}

/// The declaration's family and its traces, sorted by trace name.
fn family(base_mode: CornerBaseMode) -> (AnalysisResultFamilyMetadata, Vec<(String, Vec<f64>)>) {
    let run = crate::simulation::runner::pvt_point_evidence::run_temperature_declaration(
        DERATED_DIVIDER,
        contract(base_mode),
        27.0,
    )
    .expect("the temperature declaration prepares, authorizes and runs");
    let analysis = run
        .analyses
        .iter()
        .find(|analysis| analysis.analysis_type == AnalysisType::Parametric)
        .expect("the declaration produces a parametric family");
    assert!(
        analysis.success,
        "temperature family failed: {:?}",
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
            .expect("a parametric family carries its axis"),
        traces,
    )
}

fn trace_names(traces: &[(String, Vec<f64>)]) -> Vec<&str> {
    traces.iter().map(|(name, _)| name.as_str()).collect()
}

fn trace<'a>(traces: &'a [(String, Vec<f64>)], name: &str) -> &'a [f64] {
    traces
        .iter()
        .find(|(candidate, _)| candidate == name)
        .map(|(_, values)| values.as_slice())
        .unwrap_or_else(|| panic!("no trace named {name} in {:?}", trace_names(traces)))
}

/// The axis is the declared temperatures themselves, in the order declared.
#[test]
fn the_axis_is_the_declared_temperatures() {
    let (metadata, _) = family(CornerBaseMode::Op);

    assert_eq!(
        metadata,
        AnalysisResultFamilyMetadata::Parametric {
            member_measurements: Vec::new(),
            target: "TEMP".to_owned(),
            sweep_values: DECLARED.to_vec(),
            failed_points: 0,
        }
    );
}

/// The whole reason a temperature step exists: the answer has to move with
/// temperature. If these three read alike the deck was solved three times at
/// one temperature, which is what an unexpanded declaration would produce.
#[test]
fn an_operating_point_step_contributes_a_node_voltage_per_temperature() {
    let (_, traces) = family(CornerBaseMode::Op);

    assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
    let out = trace(&traces, "V(OUT)");
    assert_eq!(out.len(), DECLARED.len());
    assert!(
        out[0] > out[1] && out[1] > out[2],
        "a hotter upper leg drops more of the supply across itself: {out:?}"
    );
    // The supply is untouched by the axis, so it is the control.
    for value in trace(&traces, "V(VDD)") {
        assert!((value - 1.8).abs() < 1.0e-9, "supply moved: {value}");
    }
}

#[test]
fn a_transient_step_contributes_its_terminal_sample_for_every_node() {
    let (_, traces) = family(CornerBaseMode::Transient {
        stop_time: 100.0e-9,
        step_time: 1.0e-9,
    });

    assert_eq!(trace_names(&traces), vec!["V(OUT)", "V(VDD)"]);
    let out = trace(&traces, "V(OUT)");
    assert!(out[0] > out[1] && out[1] > out[2], "{out:?}");
}

/// AC names its traces as magnitudes, which is the one place a temperature
/// family's trace names depend on the base analysis rather than on the axis.
#[test]
fn an_ac_step_contributes_terminal_frequency_magnitude() {
    let (metadata, traces) = family(CornerBaseMode::Ac {
        start_freq: 1.0e3,
        stop_freq: 1.0e6,
        points_per_unit: 5,
        sweep: CornerFrequencySweep::Decade,
    });

    assert_eq!(trace_names(&traces), vec!["|V(OUT)|", "|V(VDD)|"]);
    assert!(matches!(
        metadata,
        AnalysisResultFamilyMetadata::Parametric {
            failed_points: 0,
            ..
        }
    ));
}

/// Three declared temperatures, three solves, and a family that costs no solve
/// of its own — it carries no PVT point because it did not solve one. This is
/// the duplicate-solve check: an expansion that also solved the declaration
/// would show four attributed results or a fourth engine call.
#[test]
fn the_family_is_assembled_from_the_points_rather_than_solved_again() {
    let run = crate::simulation::runner::pvt_point_evidence::run_temperature_declaration(
        DERATED_DIVIDER,
        contract(CornerBaseMode::Op),
        27.0,
    )
    .expect("the temperature declaration prepares, authorizes and runs");

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
    assert_eq!(
        attributed,
        DECLARED.len(),
        "one solve per declared temperature, and no more"
    );
    assert_eq!(
        run.analyses.len(),
        DECLARED.len() + 1,
        "the three points and the one family they add up to"
    );
    let families = run
        .analyses
        .iter()
        .filter(|analysis| analysis.analysis_type == AnalysisType::Parametric)
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
    // Every solve is a base-analysis result. A declaration that reached the
    // sweep executor would have retained a second parametric result beside
    // this one.
    assert_eq!(
        run.analyses
            .iter()
            .filter(|analysis| analysis.analysis_type == AnalysisType::DcOp)
            .count(),
        DECLARED.len()
    );
}

/// A declaration none of whose points converged is a failed parametric result,
/// not a missing one.
#[test]
fn a_declaration_with_no_converged_point_is_a_failure() {
    let outcome = temperature_family_of_points(&CornerBaseMode::Op, 3, &[]);

    assert_eq!(
        outcome.expect_err("nothing converged"),
        "Parametric analysis produced no converged sweep points"
    );
}

fn temperature_run() -> crate::state::SimulationRun {
    crate::simulation::runner::pvt_point_evidence::run_temperature_declaration(
        DERATED_DIVIDER,
        contract(CornerBaseMode::Op),
        27.0,
    )
    .expect("the temperature declaration prepares, authorizes and runs")
}

/// A run's retained results have to be an exact ordered prefix of the task
/// graph its receipt authenticated, and that is checked on project load as a
/// hard error rather than a warning. The family is a result, so it has to be a
/// task: a family assembled outside the task list would leave a saved run
/// carrying one more result than it had authenticated tasks, and the project
/// would refuse to reopen.
#[test]
fn a_temperature_run_is_an_authentic_prefix_of_its_receipt_and_survives_a_project_round_trip() {
    use crate::io::project_io::ProjectSimulationResults;

    let run = temperature_run();
    run.validate_provenance()
        .expect("every retained result answers for an authenticated task");
    assert_eq!(
        run.prepared_receipt()
            .expect("the run is sealed by its dispatch")
            .tasks()
            .len(),
        run.analyses.len(),
        "three points and the family they add up to, against four authenticated tasks"
    );

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
    persisted
        .validate()
        .expect("a temperature run is persistable");

    let mut reloaded = crate::state::SimulationState::default();
    persisted
        .apply_to_state(&mut reloaded)
        .expect("a saved temperature run reopens");

    let restored = reloaded.runs.first().expect("the run survives the reload");
    assert_eq!(restored.analyses.len(), simulation.runs[0].analyses.len());
    assert!(
        restored.analyses.iter().any(
            |analysis| analysis.analysis_type == AnalysisType::Parametric
                && matches!(
                    analysis.family_metadata,
                    Some(AnalysisResultFamilyMetadata::Parametric { .. })
                )
        ),
        "the parametric plot's axis survives the round trip"
    );
    restored
        .validate_provenance()
        .expect("the reloaded run is still an authentic prefix");
}

/// A run that stopped part-way keeps the same property. This is why the
/// assembly is ordered last: every truncation of the result list is still a
/// prefix of the task list, whereas an assembly ordered first could not produce
/// its result until its points had and would leave a hole.
#[test]
fn a_temperature_run_that_stopped_part_way_is_still_an_authentic_prefix() {
    let complete = temperature_run();
    assert_eq!(complete.analyses.len(), DECLARED.len() + 1);

    for retained in 0..complete.analyses.len() {
        let mut partial = complete.clone();
        partial.analyses.truncate(retained);
        partial.validate_provenance().unwrap_or_else(|error| {
            panic!("a run that stopped after {retained} result(s) must still validate: {error}")
        });
    }
}
