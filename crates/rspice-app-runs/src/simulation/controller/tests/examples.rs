//! Every built-in example runs the plan it opens with to the end, and the
//! run shows what the example's summary says, checked against the circuit's
//! closed form.

use rspice_app_state::workbench::examples::{EXAMPLES, load_example, load_example_plan};
use rspice_results::run::SimulationRunLifecycle;
use rspice_results::waveform::RetainedWaveform;

use super::*;
use crate::simulation::controller::test_execution::run_generated_batch;

/// The most values a browser run may hold; a run under it runs in both builds.
const BROWSER_RESULT_VALUES: usize = 25_000_000;

/// The run of the example `name`, opened as Help > Examples opens it.
fn example_run(name: &str) -> crate::state::SimulationRun {
    let mut state = RunTestHost::default();
    load_example(name, &mut state.model.schematic);
    load_example_plan(name, &mut state.model.sim_setup);
    run_generated_batch(state, SimulationRunLifecycle::Completed)
}

/// The trace `name` of the run's last analysis, the one the example shows, as
/// magnitudes when it is complex.
fn trace(run: &crate::state::SimulationRun, name: &str) -> (Vec<f64>, Vec<f64>) {
    let analysis = run.analyses.last().expect("the run has an analysis");
    let wave: &RetainedWaveform = analysis
        .waveforms
        .iter()
        .find(|wave| wave.name.eq_ignore_ascii_case(name))
        .unwrap_or_else(|| {
            let names: Vec<_> = analysis.waveforms.iter().map(|wave| &wave.name).collect();
            panic!("{name} is not among {names:?}")
        });
    let y = match &wave.complex {
        Some(complex) => complex
            .real
            .iter()
            .zip(&complex.imag)
            .map(|(re, im)| re.hypot(*im))
            .collect(),
        None => wave.y.to_vec(),
    };
    (wave.x.to_vec(), y)
}

/// The value of `(x, y)` at the sample nearest `at`.
fn at(trace: &(Vec<f64>, Vec<f64>), at: f64) -> f64 {
    let nearest = trace
        .0
        .iter()
        .enumerate()
        .min_by(|a, b| (a.1 - at).abs().total_cmp(&(b.1 - at).abs()))
        .map(|(index, _)| index)
        .expect("the trace has samples");
    trace.1[nearest]
}

#[test]
fn every_example_runs_its_plan_and_shows_what_its_summary_says() {
    for example in EXAMPLES {
        let run = example_run(example.name);
        assert!(
            run.analyses
                .iter()
                .all(|analysis| analysis.success && analysis.has_data()),
            "{}",
            example.name
        );
        let values: usize = run
            .analyses
            .iter()
            .flat_map(|analysis| &analysis.waveforms)
            .map(|wave| wave.x.len() + wave.y.len())
            .sum();
        assert!(
            values < BROWSER_RESULT_VALUES / 100,
            "{} holds {values} values",
            example.name
        );

        match example.name {
            "RC Lowpass Filter" => {
                // R1 1k and C1 159n put the corner at 1001 Hz.
                let out = trace(&run, "V(out)");
                let corner = 1.0 / (2.0 * std::f64::consts::PI * 1.0e3 * 159.0e-9);
                let expected = 1.0 / (1.0 + (1.0e3 / corner).powi(2)).sqrt();
                assert!((at(&out, 1.0e3) - expected).abs() < 1.0e-3);
                assert!(at(&out, 1.0) > 0.999 && at(&out, 1.0e6) < 2.0e-3);
            }
            "Voltage Divider" => {
                let (_, out) = trace(&run, "V(out)");
                assert!(out.iter().all(|volts| (volts - 2.5).abs() < 1.0e-9));
            }
            "Common Emitter Amplifier" => {
                // A 10 mV input comes out larger in mid-band, and the input
                // coupling capacitor rolls it off at the low end.
                let out = trace(&run, "V(out)");
                let mid_band = at(&out, 10.0e3);
                assert!(mid_band > 10.0e-3, "mid-band output {mid_band} V");
                assert!(at(&out, 1.0) < mid_band / 2.0);
            }
            "CMOS Inverter" => {
                // Rail to rail, five times over.
                let (time, out) = trace(&run, "V(out)");
                assert!(
                    time.last()
                        .is_some_and(|end| (end - 50.0e-9).abs() < 1.0e-12)
                );
                assert!(out.iter().any(|volts| *volts > 3.2));
                assert!(out.iter().any(|volts| *volts < 0.1));
            }
            "Differential Pair" => {
                // The 1 mA tail is steered whole to one side at each end of
                // the sweep: the outputs swap 1 V across 1k collectors.
                let outp = trace(&run, "V(outp)");
                let outn = trace(&run, "V(outn)");
                let low = at(&outp, -0.1) - at(&outn, -0.1);
                let high = at(&outp, 0.1) - at(&outn, 0.1);
                assert!(low * high < 0.0, "the outputs cross: {low} V, {high} V");
                assert!(low.abs() > 0.9 && high.abs() > 0.9);
            }
            "Opamp Inverting Amplifier" => {
                // -RF/RIN = -10, less the open-loop gain's 1e6 shortfall.
                let out = trace(&run, "V(vout)");
                let gain = -10.0 / (1.0 + 11.0 / 1.0e6);
                assert!((at(&out, 1.0) - gain).abs() < 1.0e-6);
                assert!((at(&out, -1.0) + gain).abs() < 1.0e-6);
            }
            other => panic!("{other} has no check of its own"),
        }
    }
}
