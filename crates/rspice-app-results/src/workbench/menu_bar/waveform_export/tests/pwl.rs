//! SPICE PWL export: a trace written as a source drives another run through
//! its retained samples.

use super::*;
use crate::state::SimulationRunLifecycle;
use rspice_app_runs::simulation::controller::test_execution::{
    run_manual_batch, run_manual_batch_at,
};

const STEP_DECK: &str = "rc step
V1 in 0 PULSE(0 1 0 1n 1n 5u 10u)
R1 in out 1k
C1 out 0 1n
.tran 10n 2u
.end
";

/// Index of the PWL source in the stored export choice.
const PWL: usize = 12;

/// The source file "Export dataset…" writes for `trace`, and its name.
fn exported_pwl(state: &mut ResultsTestHost, trace: Option<&str>) -> (PathBuf, String) {
    state
        .model
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, PWL)
        .expect("the PWL preference");
    let io = MockExportWorkflowIo::default();
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &io,
        &DatasetExportChoice {
            trace: trace.map(str::to_owned),
            ..DatasetExportChoice::default()
        },
    );
    let files = io.byte_files.borrow();
    assert_eq!(files.len(), 1, "{:?}", state.prompt_text());
    assert_eq!(files[0].2, "text/plain;charset=utf-8");
    (
        files[0].0.clone(),
        String::from_utf8(files[0].1.clone()).expect("a PWL source is text"),
    )
}

fn scratch_folder(name: &str) -> std::path::PathBuf {
    let folder = std::env::temp_dir().join(format!("rspice-pwl-{name}-{}", std::process::id()));
    std::fs::create_dir_all(&folder).expect("a scratch folder");
    folder
}

/// The voltage at the output of an RC step, exported as a PWL source and
/// replayed into a resistor by another run, passes through every retained
/// sample: the replay lands on each breakpoint and holds the value there.
#[test]
fn a_pwl_export_drives_a_run_through_the_retained_samples() {
    let run = run_manual_batch(STEP_DECK);
    let original = run.analyses[0]
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "OUT")
        .expect("the step retains OUT")
        .clone();
    let mut state = state_with_typed_result(run.analyses[0].clone());
    let (path, source) = exported_pwl(&mut state, Some("OUT"));
    assert_eq!(path, PathBuf::from("OUT.inc"));
    assert!(
        source.contains("\n.subckt PWL_OUT p n\nV1 p n PWL(\n"),
        "{source}"
    );
    assert!(source.ends_with("+ )\n.ends PWL_OUT\n"), "{source}");

    let folder = scratch_folder("replay");
    std::fs::write(folder.join("OUT.inc"), &source).unwrap();
    let stop = original.x.last().copied().unwrap();
    let deck = format!(
        "pwl replay\n.include \"OUT.inc\"\nX1 a 0 PWL_OUT\nR1 a 0 1k\n.tran 10n {stop:e}\n.end\n"
    );
    let replay = run_manual_batch_at(
        &deck,
        &folder.join("replay.cir"),
        SimulationRunLifecycle::Completed,
    );
    let _ = std::fs::remove_dir_all(&folder);
    let replayed = replay.analyses[0]
        .waveforms
        .iter()
        .find(|waveform| waveform.name == "A")
        .expect("the replay retains A");
    let mut worst = 0.0_f64;
    for (time, value) in original.x.iter().zip(original.y.iter()) {
        let at = replayed
            .x
            .iter()
            .position(|replay_time| replay_time == time)
            .unwrap_or_else(|| panic!("the replay steps onto the breakpoint at {time:e} s"));
        worst = worst.max((replayed.y[at] - value).abs());
    }
    // Bit-exact on the reference box; a few ulps leave room for another
    // platform's solver arithmetic, not for a misread number.
    assert!(
        worst <= 4.0 * f64::EPSILON,
        "the replay leaves a retained sample by {worst:e} V"
    );
}

/// A current trace becomes a current source, and with no trace named the
/// first one on the plot is written.
#[test]
fn a_current_trace_is_a_current_source_and_no_name_takes_the_first_plotted() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform("I(R1)", vec![0.0, 1e-9, 2e-9], vec![0.0, 1e-3, -2.5e-4]).with_unit("A"),
            waveform("V(out)", vec![0.0, 1e-9, 2e-9], vec![0.0, 1.0, 0.5]).with_unit("V"),
        ]),
    );
    let (path, source) = exported_pwl(&mut state, None);
    assert_eq!(path, PathBuf::from("I_R1.inc"));
    assert!(
        source.contains(
            ".subckt PWL_I_R1 p n\nI1 p n PWL(\n+ 0e0 0e0\n+ 1e-9 1e-3\n+ 2e-9 -2.5e-4\n+ )\n"
        ),
        "{source}"
    );
}

/// A sample that repeats its predecessor exactly is dropped; two values at
/// one instant cannot be a PWL source and are refused at that instant,
/// before a destination is asked for.
#[test]
fn a_repeated_sample_is_dropped_and_two_values_at_one_time_are_refused() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Transient, "Transient").with_waveforms(vec![
            waveform(
                "V(a)",
                vec![0.0, 1e-9, 1e-9, 2e-9],
                vec![0.0, 1.0, 1.0, 0.5],
            ),
            waveform(
                "V(b)",
                vec![0.0, 1e-9, 1e-9, 2e-9],
                vec![0.0, 1.0, 0.0, 0.5],
            ),
        ]),
    );
    let (_, source) = exported_pwl(&mut state, Some("V(a)"));
    assert_eq!(source.matches("\n+ 1e-9 ").count(), 1, "{source}");

    state
        .model
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, PWL)
        .unwrap();
    let io = MockExportWorkflowIo::default();
    action_export_dataset_with_io(
        &mut state.results_parts(),
        &io,
        &DatasetExportChoice {
            trace: Some("V(b)".to_owned()),
            ..DatasetExportChoice::default()
        },
    );
    assert!(io.dialog_titles.borrow().is_empty());
    let message = refusal(&state);
    assert!(message.contains("two values at t = 1e-9 s"), "{message}");
}

/// A PWL source is a waveform in time; an AC sweep is refused by name.
#[test]
fn a_frequency_sweep_is_refused_as_a_pwl_source() {
    let mut state = state_with_typed_result(
        AnalysisResult::new(1, AnalysisType::Ac, "AC Analysis").with_waveforms(vec![
            complex_waveform(
                "|V(out)|",
                "V(out)",
                vec![1.0e3, 1.0e4],
                vec![1.0, 0.5],
                vec![1.0, 0.3],
                vec![0.0, -0.4],
            ),
        ]),
    );
    state
        .model
        .preferences
        .set_choice(crate::workbench::ChoicePreference::EngineeringExport, PWL)
        .unwrap();
    let io = MockExportWorkflowIo::default();
    action_export_csv_with_io(&mut state.results_parts(), &io);
    assert!(io.byte_files.borrow().is_empty());
    let message = refusal(&state);
    assert!(message.contains("waveform in time"), "{message}");
}
