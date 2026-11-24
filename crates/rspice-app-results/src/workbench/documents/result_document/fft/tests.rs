//! The spectrum sheet over a stored run: what it transforms, and what it reads
//! of the store to prepare it.

use std::sync::Arc;
use std::time::{Duration, Instant};

use super::follow_plotted;
use crate::simulation::SimulationController;
use crate::simulation::controller::DerivedViewerLoadState;
use crate::state::{ActiveViewer, SimulationRun};
use crate::workbench::documents::result_document::AnalysisPresentationKey;
use crate::workbench::documents::result_document::test_fixtures::{
    CountedColumn, stored_transient,
};
use crate::workbench::documents::result_document::test_host::ResultsTestHost;
use rspice_results::result_presentation::SourceWaveformPresentationKey;

/// A stored transient over 1 ms of 2^17 steps, more than one block of the
/// store, that saved, in this order, a 2 kHz current I(V1), a 4 kHz V(a) the
/// plan does not plot and a 16 kHz V(out) it does: the store's columns come
/// back, axis first, to count what is read of them.
fn stored_run() -> (ResultsTestHost, Vec<Arc<CountedColumn>>) {
    let steps = 1 << 17;
    let time: Vec<f64> = (0..=steps)
        .map(|row| row as f64 * 1.0e-3 / steps as f64)
        .collect();
    let tone = |frequency: f64, amplitude: f64| -> Vec<f64> {
        time.iter()
            .map(|t| amplitude * (std::f64::consts::TAU * frequency * t).sin())
            .collect()
    };
    let signals = vec![
        ("I(V1)".to_owned(), tone(2.0e3, 1.0e-3)),
        ("V(a)".to_owned(), tone(4.0e3, 0.5)),
        ("V(out)".to_owned(), tone(16.0e3, 1.0)),
    ];
    let (mut analysis, columns) = stored_transient(time.clone(), signals, Some([3; 32]));
    analysis.waveforms[1].visible = false;
    let mut run = SimulationRun::new(1);
    run.add_analysis(analysis);
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![run].into();
    state.model.simulation.view.active_run_idx = Some(0);
    state.model.simulation.view.active_analysis_idx = Some(0);
    (state, columns)
}

/// Open the spectrum sheet and wait for its preparation.
fn open_spectrum(state: &mut ResultsTestHost, controller: &mut SimulationController) {
    let deadline = Instant::now() + Duration::from_secs(60);
    loop {
        let mut parts = state.results_parts();
        follow_plotted(&mut parts);
        match controller.ensure_transient_viewer_data(&mut parts, ActiveViewer::Fft) {
            DerivedViewerLoadState::Ready => return,
            DerivedViewerLoadState::Loading => {
                assert!(Instant::now() < deadline, "the spectrum was never prepared");
                std::thread::sleep(Duration::from_millis(5));
            }
            DerivedViewerLoadState::Unavailable => panic!(
                "no spectrum: {:?}",
                state.results.analysis.fft_state.last_error
            ),
        }
    }
}

fn fundamental(state: &ResultsTestHost) -> f64 {
    state
        .results
        .analysis
        .fft_state
        .analysis
        .as_ref()
        .and_then(|analysis| analysis.fundamental_frequency)
        .expect("a fundamental")
}

/// With a current saved before it and an unplotted voltage between, the
/// sheet transforms the first voltage plotted, V(out), not the first name
/// that sorts; plotting V(a) moves the sheet to it. The run's completion
/// reads none of the store, and the spectrum reads it a window at a time,
/// never a column whole.
#[test]
fn the_spectrum_is_of_the_first_voltage_plotted_and_reads_the_store_by_windows() {
    let (mut state, columns) = stored_run();
    let mut controller = SimulationController::new();

    controller.invalidate_transient_post_views(&mut state.results_parts());
    let rows_read: u64 = columns.iter().map(|column| column.rows_read()).sum();
    assert_eq!(rows_read, 0, "the run's completion read its store");

    open_spectrum(&mut state, &mut controller);
    let fft = &state.results.analysis.fft_state;
    assert_eq!(fft.selected_source.as_deref(), Some("V(out)"));
    assert!(
        (fundamental(&state) - 16.0e3).abs() < 1.0,
        "{}",
        fundamental(&state)
    );

    let run = state.model.simulation.active_run().expect("a run");
    let key = AnalysisPresentationKey::new(run.dataset_id, &run.analyses[0]);
    state
        .results
        .results
        .session
        .waveform_visibility
        .insert(SourceWaveformPresentationKey::new(key, "V(a)"), true);
    open_spectrum(&mut state, &mut controller);
    assert_eq!(
        state.results.analysis.fft_state.selected_source.as_deref(),
        Some("V(a)")
    );
    assert!(
        (fundamental(&state) - 4.0e3).abs() < 1.0,
        "{}",
        fundamental(&state)
    );

    let whole: Vec<u64> = columns.iter().map(|column| column.whole_reads()).collect();
    assert_eq!(whole, [0, 0, 0, 0], "a stored column was read whole");
}

/// A finished run of `plan` whose transient `analysis` holds 1 ms of a 16 kHz
/// V(out) in 1024 steps.
fn plan_run(
    plan: crate::product::SimulationPlanId,
    analysis: crate::product::AnalysisInstanceId,
) -> SimulationRun {
    use crate::product::ObjectRevision;
    use crate::state::{AnalysisResult, AnalysisResultProvenance, AnalysisType, RetainedTrace};
    use crate::workbench::documents::result_document::test_fixtures::{
        completed_prepared_run, digest,
    };
    let time: Vec<f64> = (0..=1024).map(|row| row as f64 * 1.0e-3 / 1024.0).collect();
    let out: Vec<f64> = time
        .iter()
        .map(|t| (std::f64::consts::TAU * 16.0e3 * t).sin())
        .collect();
    let mut run = completed_prepared_run(plan, ObjectRevision::INITIAL, digest(0x50));
    run.add_analysis(
        AnalysisResult::new(1, AnalysisType::Transient, "TRAN")
            .with_waveforms(vec![RetainedTrace::new("V(out)", time, out, "#ffbd2e")])
            .with_provenance(
                AnalysisResultProvenance::new(
                    analysis,
                    ObjectRevision::INITIAL,
                    digest(0x51),
                    Vec::new(),
                )
                .expect("analysis provenance"),
            ),
    );
    run
}

/// The window, β and harmonic count the reader sets are kept by the plan's
/// working view, through its one writer: the next run of the plan is
/// transformed the same way even when nothing but the view remembers them,
/// and Undo takes the change back.
#[test]
fn spectrum_settings_are_kept_by_the_view_across_a_rerun_and_undone() {
    use super::{adopt_kept_settings, keep_settings};
    use crate::workbench::documents::result_document::view_edits::{self, Step};
    use rspice_results::fft::settings::{SpectrumSettings, SpectrumWindow};
    use rspice_results_ui::fft::{FftState, WindowFunction, window::KaiserBeta};

    let (plan, analysis) = (
        crate::product::SimulationPlanId::new(),
        crate::product::AnalysisInstanceId::new(),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![plan_run(plan, analysis)].into();
    assert!(state.model.simulation.select_run(0));
    state.quick_frame();
    let settings = SpectrumSettings {
        window: SpectrumWindow::Kaiser,
        kaiser_beta: 20.0,
        harmonics: 40,
        ..SpectrumSettings::default()
    };
    keep_settings(&mut state.results_parts(), settings.clone());
    let view = state.working_view().expect("the run's view");
    let pane = view.panes()[0].id;
    assert_eq!(view.spectrum_settings(pane), Some(&settings));
    assert_eq!(
        state.results.analysis.fft_state.window,
        WindowFunction::Kaiser(KaiserBeta::new(20.0).unwrap())
    );

    // The session forgets the sheet, as a restart does; then the plan runs
    // again.
    crate::workbench::documents::result_document::working_view::release(&mut state.results_parts());
    state.results.analysis.fft_state = FftState::default();
    state
        .model
        .simulation
        .retained
        .runs
        .insert(0, plan_run(plan, analysis));
    assert!(state.model.simulation.select_run(0));
    state.quick_frame();
    adopt_kept_settings(&mut state.results_parts());
    assert_eq!(state.results.analysis.fft_state.settings(), settings);

    let document = state.working_view().expect("the run's view").id();
    view_edits::step(&mut state.results_parts(), document, Step::Undo).expect("an undo");
    adopt_kept_settings(&mut state.results_parts());
    assert_eq!(
        state.results.analysis.fft_state.settings(),
        SpectrumSettings::default()
    );
}

/// A trace's Spectrum entry chooses the sheet's source, and the view keeps
/// the choice: the sheet's next frame shows that trace's spectrum, not the
/// first voltage plotted, and Undo puts the plotted one back.
#[test]
fn a_trace_spectrum_entry_chooses_the_source_the_view_keeps() {
    use super::adopt_kept_settings;
    use crate::workbench::documents::result_document::view_edits::{self, Step};

    let (plan, analysis) = (
        crate::product::SimulationPlanId::new(),
        crate::product::AnalysisInstanceId::new(),
    );
    let mut state = ResultsTestHost::default();
    state.model.simulation.retained.runs = vec![plan_run(plan, analysis)].into();
    assert!(state.model.simulation.select_run(0));
    state.quick_frame();
    state
        .results
        .analysis
        .fft_state
        .choose_source_from_trace("I(V1)".to_owned());
    adopt_kept_settings(&mut state.results_parts());
    let view = state.working_view().expect("the run's view");
    let pane = view.panes()[0].id;
    let document = view.id();
    assert_eq!(
        view.spectrum_settings(pane)
            .and_then(|settings| settings.source.as_deref()),
        Some("I(V1)")
    );
    follow_plotted(&mut state.results_parts());
    assert_eq!(state.results.analysis.fft_state.source(), Some("I(V1)"));

    view_edits::step(&mut state.results_parts(), document, Step::Undo).expect("an undo");
    adopt_kept_settings(&mut state.results_parts());
    follow_plotted(&mut state.results_parts());
    assert_eq!(state.results.analysis.fft_state.source(), Some("V(out)"));
}
