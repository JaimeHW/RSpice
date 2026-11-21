//! A stability run's loop gain as the reader reads it in Results: the Bode
//! card and the Nyquist panel print one loop's margins, after the project
//! was saved, closed and opened again as well as straight after the run.

use crate::workbench::app_state::AppState;
use rspice_app_results::workbench::documents::result_document::{ResultViewer, right_panel};
use rspice_app_state::state::{
    ActiveViewer, AnalysisResult, AnalysisType, RetainedTrace, SimulationRun,
    SimulationRunProvenance,
};
use rspice_app_state::workbench::workspace::Workspace;

/// The loop gain L = 4/(1 + jf/1 kHz)³ at five points a decade from 1 Hz to
/// 1 MHz, retained as a stability analysis retains it: its Re and Im, and
/// its gain in decibels and its phase folded into one turn.
fn stability_traces() -> Vec<RetainedTrace> {
    let frequency: Vec<f64> = (0..=30).map(|i| 10f64.powf(f64::from(i) / 5.0)).collect();
    let polar: Vec<(f64, f64)> = frequency
        .iter()
        .map(|&f| {
            let x = f / 1.0e3;
            (4.0 / (1.0 + x * x).powf(1.5), -3.0 * x.atan())
        })
        .collect();
    let trace = |name: &str, values: Vec<f64>| {
        RetainedTrace::new(name, frequency.clone(), values, "#00aaff")
    };
    vec![
        trace(
            rspice_results::nyquist::LOOP_GAIN_REAL_TRACE,
            polar.iter().map(|(m, a)| m * a.cos()).collect(),
        ),
        trace(
            rspice_results::nyquist::LOOP_GAIN_IMAGINARY_TRACE,
            polar.iter().map(|(m, a)| m * a.sin()).collect(),
        ),
        trace(
            "Loop Gain (dB)",
            polar.iter().map(|(m, _)| 20.0 * m.log10()).collect(),
        ),
        trace(
            "Loop Phase (deg)",
            polar
                .iter()
                .map(|(_, a)| {
                    let degrees = a.to_degrees();
                    if degrees <= -180.0 {
                        degrees + 360.0
                    } else {
                        degrees
                    }
                })
                .collect(),
        ),
    ]
}

/// A session holding one stability run, Results in front.
fn session_with_a_stability_run() -> AppState {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Stb, "STB").with_waveforms(stability_traces()),
    );
    run.restore_provenance(SimulationRunProvenance::LegacyUnattributed)
        .expect("an imported run is unattributed");
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.retained.next_run_id = 2;
    assert!(state.model.simulation.select_run(0));
    state.workbench.activate(Workspace::Results);
    state
}

/// The rows of a loop's margins, and the frequencies they were read at.
const MARGIN_ROWS: [&str; 4] = [
    "Phase margin",
    "Gain crossover",
    "Gain margin",
    "Phase crossover",
];

/// Every margin `viewer`'s inspector prints, and each one's frequency: each
/// row's words and the value drawn after them.
fn margins(state: &mut AppState, viewer: ResultViewer) -> Vec<(String, String)> {
    fn collect(shape: &egui::Shape, text: &mut Vec<String>) {
        match shape {
            egui::Shape::Text(value) => text.push(value.galley.text().to_owned()),
            egui::Shape::Vec(shapes) => shapes.iter().for_each(|shape| collect(shape, text)),
            _ => {}
        }
    }
    state.results.results.session.viewer = viewer;
    let ctx = egui::Context::default();
    rspice_app_workbench::ui::Theme::default().apply(&ctx);
    let output = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1200.0, 900.0),
            )),
            ..Default::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .show(ctx, |ui| right_panel(ui, &mut state.results_parts()));
        },
    );
    let mut text = Vec::new();
    for shape in output.shapes {
        collect(&shape.shape, &mut text);
    }
    MARGIN_ROWS
        .iter()
        .filter_map(|row| {
            let at = text.iter().position(|value| value == row)?;
            Some(((*row).to_owned(), text.get(at + 1)?.clone()))
        })
        .collect()
}

/// Save `state`'s project to a scratch folder and open it again in a new
/// session, whose run reads its samples from the results folder beside the
/// project file.
fn save_and_reopen(mut state: AppState) -> (AppState, ScratchFolder) {
    let root = std::env::temp_dir().join(format!("rspice-stb-{}", uuid::Uuid::new_v4()));
    std::fs::create_dir_all(&root).expect("a scratch folder");
    let path = root.join("stability.rspiceproj");
    let saved = state.save_project_to_path(&path);
    let log: Vec<String> = state
        .model
        .log_buffer
        .entries()
        .map(|entry| entry.message.clone())
        .collect();
    assert!(saved, "the project saves: {log:?}");
    drop(state);
    let mut state = AppState::default();
    assert!(state.load_project_from_path(&path), "the project reopens");
    state.workbench.activate(Workspace::Results);
    (state, ScratchFolder(root))
}

/// A scratch folder removed when the test lets go of it.
struct ScratchFolder(std::path::PathBuf);

impl Drop for ScratchFolder {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// Both cards' margins, which must be the same words, read as the
/// closed-form margins of the loop within the interpolation's stated error.
fn assert_one_loops_margins(state: &mut AppState) {
    let bode = margins(state, ResultViewer::Bode);
    let nyquist = margins(state, ResultViewer::Nyquist);
    assert_eq!(
        bode, nyquist,
        "both cards print the same margins the same way"
    );
    let [phase, unity_at, gain, inversion_at] = bode.as_slice() else {
        panic!("each margin and its frequency: {bode:?}");
    };
    let (phase, unity_at, gain, inversion_at) = (&phase.1, &unity_at.1, &gain.1, &inversion_at.1);
    let leading = |text: &str| -> f64 {
        text.split(' ')
            .next()
            .and_then(|number| number.parse().ok())
            .unwrap_or_else(|| panic!("a number leads {text:?}"))
    };
    // |L| = 1 where (1 + x²)^1.5 = 4; the phase is −180° where x = √3,
    // and |L| is 1/2 there.
    let unity = (4f64.powf(2.0 / 3.0) - 1.0).sqrt();
    let phase_margin = 180.0 - 3.0 * unity.atan().to_degrees();
    let gain_margin = 20.0 * 2f64.log10();
    assert!(
        phase.ends_with(" \u{b0}")
            && gain.ends_with(" dB")
            && unity_at.ends_with(" kHz")
            && inversion_at.ends_with(" kHz"),
        "{bode:?}"
    );
    // The crossovers sit at x = unity and x = √3 of the 1 kHz pole; the
    // interpolant that finds the margins finds where they are read, within a
    // tenth of a percent at five points a decade.
    for (text, closed_form) in [(unity_at, unity), (inversion_at, 3f64.sqrt())] {
        assert!(
            (leading(text) / closed_form - 1.0).abs() <= 1.0e-3,
            "{text} against {closed_form} kHz"
        );
    }
    assert!(
        (leading(phase) - phase_margin).abs() <= rspice_results::nyquist::MARGIN_ERROR_DEGREES,
        "{phase} against {phase_margin}"
    );
    assert!(
        (leading(gain) - gain_margin).abs() <= rspice_results::nyquist::MARGIN_ERROR_DECIBELS,
        "{gain} against {gain_margin}"
    );
}

/// A stability run saved, closed and reopened opens its Nyquist sheet, and
/// the Bode card and the Nyquist panel print the loop's closed-form margins,
/// in the same words, read at five points a decade.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_reopened_stability_run_reads_one_loops_margins_on_bode_and_nyquist() {
    let mut state = session_with_a_stability_run();
    assert!(state.viewer_is_available(ActiveViewer::Nyquist));
    assert_one_loops_margins(&mut state);

    let (mut state, _folder) = save_and_reopen(state);
    assert!(
        state
            .model
            .simulation
            .active_run()
            .is_some_and(|run| run.analyses[0]
                .waveforms
                .iter()
                .all(|trace| trace.y.in_memory().is_none())),
        "the reopened run reads its samples from the results folder"
    );
    assert!(
        state.viewer_is_available(ActiveViewer::Nyquist),
        "the reopened stability run opens its Nyquist sheet"
    );
    assert_one_loops_margins(&mut state);
}
