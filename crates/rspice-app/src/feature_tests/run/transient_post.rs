//! The transient coordinator's derived views, observed through the results
//! viewer they load.

use std::sync::{Arc, mpsc};
use std::time::Duration;

use rspice_results_ui::eye_diagram::{EyeTimebase, EyeTimebaseProvenance};
use rspice_results_ui::fft::{
    FftFailure, FftInputError, PreparedFftInput, WindowFunction,
    data::{FftBuildError, SpectrumAnalysisError},
};

use crate::workbench::app_state::AppState;
use rspice_app_runs::simulation::controller::SimulationController;
use rspice_app_runs::simulation::controller::test_execution::clock;
use rspice_app_runs::simulation::controller::transient_post::{
    DerivedViewAvailability, DerivedViewKind, DerivedViewResultPayload, DerivedViewTaskResult,
    DerivedViewerLoadState, LoadedDerivedView, PendingDerivedViewTask, build_eye_from_waveform,
    eye_matches_timebase,
};
use rspice_app_services::workbench::results_job::{JobStop, ResultsJob};
use rspice_app_state::product::{AnalysisInstanceId, DatasetId};
use rspice_app_state::state::{
    ActiveViewer, AnalysisResult, AnalysisType, SimulationRun, SpecializedViewerCacheProvenance,
};

fn owner() -> SpecializedViewerCacheProvenance {
    SpecializedViewerCacheProvenance::for_prepared_analysis(
        DatasetId::new(),
        AnalysisInstanceId::new(),
    )
}

fn seed_fft(state: &mut AppState, analysis: SpecializedViewerCacheProvenance) {
    state.results.analysis.fft_state.window = WindowFunction::Rectangular;
    state
        .results
        .analysis
        .fft_state
        .load_prepared_input(PreparedFftInput {
            name: "V(out)".to_owned(),
            samples: vec![0.0; 16],
            sample_rate: 16.0,
            original_count: 16,
            decimation_factor: 1,
            dc_level: 0.0,
            record: rspice_results_ui::fft::pipeline::FftTimeWindow::new(0.0, 1.0),
        })
        .expect("qualified FFT ownership fixture");
    state
        .results
        .analysis
        .bind_specialized_viewer_cache(ActiveViewer::Fft, analysis);
}

/// An FFT preparation for `analysis` that ends with what the returned sender sends, or stops
/// unexpectedly when the sender is dropped.
fn install_fft_task(
    controller: &mut SimulationController,
    analysis: SpecializedViewerCacheProvenance,
) -> (
    mpsc::Sender<DerivedViewTaskResult>,
    ResultsJob<DerivedViewTaskResult>,
) {
    let (sender, receiver) = mpsc::channel();
    let job = ResultsJob::start("fft-test", move |job| {
        loop {
            match receiver.recv_timeout(Duration::from_millis(5)) {
                Ok(message) => return Ok(message),
                Err(mpsc::RecvTimeoutError::Timeout) => job.check()?,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    return Err(JobStop::Failed("the preparation stopped".to_owned()));
                }
            }
        }
    });
    controller.transient_post.fft_task = Some(PendingDerivedViewTask {
        analysis,
        job: job.clone(),
    });
    (sender, job)
}

#[test]
fn matching_fft_input_failure_owns_and_clears_only_its_active_cache() {
    let analysis = owner();
    let mut state = AppState::default();
    seed_fft(&mut state, analysis);
    let mut controller = SimulationController::new();
    let (sender, job) = install_fft_task(&mut controller, analysis);
    sender
        .send(DerivedViewTaskResult {
            analysis,
            payload: DerivedViewResultPayload::Fft(Err(FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16,
            })),
        })
        .expect("installed local receiver");
    assert!(job.wait(Duration::from_secs(10)));

    controller.poll_transient_task(&mut state, DerivedViewKind::Fft, Some(analysis));

    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.source_cache.is_none());
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert!(matches!(
        state.results.analysis.fft_state.last_error,
        Some(FftFailure::Input(FftInputError::LengthMismatch {
            time_count: 17,
            value_count: 16
        }))
    ));
    assert_eq!(
        controller.transient_post.fft_loaded,
        Some(LoadedDerivedView {
            analysis,
            availability: DerivedViewAvailability::Unavailable,
        })
    );
}

#[test]
fn stale_fft_input_failure_cannot_mutate_the_active_cache() {
    let active = owner();
    let stale = owner();
    let mut state = AppState::default();
    seed_fft(&mut state, active);
    let mut controller = SimulationController::new();
    let (sender, job) = install_fft_task(&mut controller, stale);
    sender
        .send(DerivedViewTaskResult {
            analysis: stale,
            payload: DerivedViewResultPayload::Fft(Err(FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16,
            })),
        })
        .expect("installed local receiver");
    assert!(job.wait(Duration::from_secs(10)));

    controller.poll_transient_task(&mut state, DerivedViewKind::Fft, Some(active));

    assert!(state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.source_cache.is_some());
    assert!(state.results.analysis.fft_state.last_error.is_none());
    assert_eq!(state.results.analysis.cache_authority.fft, Some(active));
    assert!(controller.transient_post.fft_loaded.is_none());
}

#[test]
fn design_replacement_rejects_a_late_successful_fft_completion() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));
    let stale = state
        .model
        .active_specialized_viewer_cache_provenance()
        .unwrap();
    let mut controller = SimulationController::new();
    let (sender, job) = install_fft_task(&mut controller, stale);

    state.clear_design_execution_context();
    // Reuse the display indices and sequence numbers in the replacement design.
    let mut replacement = SimulationRun::new(1);
    replacement.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    state.model.simulation.retained.runs = vec![replacement].into();
    assert!(state.model.simulation.select_run(0));
    let active = state
        .model
        .active_specialized_viewer_cache_provenance()
        .unwrap();
    assert_ne!(active, stale);
    seed_fft(&mut state, active);
    let retained = state
        .results
        .analysis
        .fft_state
        .source_cache
        .clone()
        .unwrap();

    sender
        .send(DerivedViewTaskResult {
            analysis: stale,
            payload: DerivedViewResultPayload::Fft(Ok(PreparedFftInput {
                name: "previous design".to_owned(),
                samples: vec![1.0; 32],
                sample_rate: 32.0,
                original_count: 32,
                decimation_factor: 1,
                dc_level: 0.0,
                record: rspice_results_ui::fft::pipeline::FftTimeWindow::new(0.0, 1.0),
            })),
        })
        .expect("completion arrives before the next application poll");
    controller.sync_transient_post_views(&mut state);

    assert!(job.is_cancelled());
    assert!(controller.transient_post.fft_task.is_none());
    assert!(controller.transient_post.fft_loaded.is_none());
    assert_eq!(state.results.analysis.cache_authority.fft, Some(active));
    assert!(state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.last_error.is_none());
    let current = state
        .results
        .analysis
        .fft_state
        .source_cache
        .as_ref()
        .unwrap();
    assert_eq!(current.name, retained.name);
    assert_eq!(current.sample_rate, retained.sample_rate);
    assert!(Arc::ptr_eq(&current.samples, &retained.samples));
}

#[test]
fn cancelling_an_fft_task_is_silent_and_drops_its_result_channel() {
    let analysis = owner();
    let mut state = AppState::default();
    seed_fft(&mut state, analysis);
    let mut controller = SimulationController::new();
    let (sender, job) = install_fft_task(&mut controller, analysis);

    controller.cancel_task_slot(DerivedViewKind::Fft);

    assert!(job.is_cancelled());
    assert!(controller.transient_post.fft_task.is_none());
    assert!(job.wait(Duration::from_secs(10)));
    assert!(
        sender
            .send(DerivedViewTaskResult {
                analysis,
                payload: DerivedViewResultPayload::Fft(Err(FftInputError::LengthMismatch {
                    time_count: 17,
                    value_count: 16,
                })),
            })
            .is_err()
    );
    assert!(state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.last_error.is_none());
    assert_eq!(state.results.analysis.cache_authority.fft, Some(analysis));
    assert!(controller.transient_post.fft_loaded.is_none());
}

#[test]
fn active_fft_worker_disconnect_fails_closed_with_typed_ownership() {
    let analysis = owner();
    let mut state = AppState::default();
    seed_fft(&mut state, analysis);
    let mut controller = SimulationController::new();
    let (sender, job) = install_fft_task(&mut controller, analysis);
    drop(sender);
    assert!(job.wait(Duration::from_secs(10)));

    controller.poll_transient_task(&mut state, DerivedViewKind::Fft, Some(analysis));

    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.source_cache.is_none());
    assert_eq!(
        state.results.analysis.fft_state.last_error,
        Some(FftFailure::WorkerDisconnected)
    );
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert_eq!(
        controller.transient_post.fft_loaded,
        Some(LoadedDerivedView {
            analysis,
            availability: DerivedViewAvailability::Unavailable,
        })
    );
}

#[test]
fn failed_fft_recompute_cannot_leave_the_coordinator_ready() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    state.results.analysis.fft_state.window = WindowFunction::Rectangular;
    let mut samples = vec![0.0; 16];
    samples[0] = 1.0;
    samples[1] = f64::from_bits(1);
    state
        .results
        .analysis
        .fft_state
        .load_prepared_input(PreparedFftInput {
            name: "V(out)".to_owned(),
            samples,
            sample_rate: 16.0,
            original_count: 16,
            decimation_factor: 1,
            dc_level: 0.0,
            record: rspice_results_ui::fft::pipeline::FftTimeWindow::new(0.0, 1.0),
        })
        .expect("the rectangular window preserves every authored sample");

    let mut controller = SimulationController::new();
    controller.mark_transient_view_ready(&mut state, ActiveViewer::Fft);
    assert_eq!(
        controller.ensure_transient_viewer_data(&mut state, ActiveViewer::Fft),
        DerivedViewerLoadState::Ready
    );
    assert!(state.results.analysis.cache_authority.fft.is_some());

    assert!(matches!(
        state
            .results
            .analysis
            .fft_state
            .set_window(WindowFunction::Hanning),
        Err(FftFailure::Build(FftBuildError::ErasedWindowedSample {
            index: 1,
            ..
        }))
    ));
    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.last_error.is_some());

    assert_eq!(
        controller.ensure_transient_viewer_data(&mut state, ActiveViewer::Fft),
        DerivedViewerLoadState::Unavailable
    );
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert!(matches!(
        state.results.analysis.fft_state.last_error,
        Some(FftFailure::Build(FftBuildError::ErasedWindowedSample {
            index: 1,
            ..
        }))
    ));
}

#[test]
fn failed_fft_analysis_cannot_leave_the_coordinator_ready() {
    let mut state = AppState::default();
    let mut run = SimulationRun::new(1);
    run.add_analysis(AnalysisResult::new(1, AnalysisType::Transient, "TRAN"));
    state.model.simulation.retained.runs = vec![run].into();
    assert!(state.model.simulation.select_run(0));

    state.results.analysis.fft_state.window = WindowFunction::Rectangular;
    state
        .results
        .analysis
        .fft_state
        .load_prepared_input(PreparedFftInput {
            name: "V(out)".to_owned(),
            samples: (0..64)
                .map(|index| (index as f64 * std::f64::consts::TAU / 8.0).sin())
                .collect(),
            sample_rate: 64.0,
            original_count: 64,
            decimation_factor: 1,
            dc_level: 0.0,
            record: rspice_results_ui::fft::pipeline::FftTimeWindow::new(0.0, 1.0),
        })
        .expect("valid FFT analysis fixture");

    let mut controller = SimulationController::new();
    controller.mark_transient_view_ready(&mut state, ActiveViewer::Fft);
    assert_eq!(
        controller.ensure_transient_viewer_data(&mut state, ActiveViewer::Fft),
        DerivedViewerLoadState::Ready
    );
    assert!(state.results.analysis.cache_authority.fft.is_some());

    state.results.analysis.fft_state.num_harmonics = 0;
    assert!(matches!(
        state.results.analysis.fft_state.recompute_from_source(),
        Err(FftFailure::Analysis(
            SpectrumAnalysisError::InvalidHarmonicOrder { value: 0, .. }
        ))
    ));
    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.source_cache.is_some());

    assert_eq!(
        controller.ensure_transient_viewer_data(&mut state, ActiveViewer::Fft),
        DerivedViewerLoadState::Unavailable
    );
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert!(matches!(
        state.results.analysis.fft_state.last_error,
        Some(FftFailure::Analysis(
            SpectrumAnalysisError::InvalidHarmonicOrder { value: 0, .. }
        ))
    ));
}

/// Editing the rate has to invalidate the eye on screen. The comparison
/// runs against the loaded data's own provenance — what the sheet is
/// quoting — so it cannot be defeated by the already-have-data shortcut.
#[test]
fn a_changed_rate_makes_the_loaded_eye_stale() {
    let mut state = AppState::default();
    let (time, signal) = clock(60, 0.137e-9);
    let stated = EyeTimebase::Explicit {
        unit_interval: 1e-9,
    };
    let (data, provenance) = build_eye_from_waveform(&time, &signal, stated);
    state
        .results
        .analysis
        .eye_diagram_state
        .load_data_with_timebase(data.expect("folds"), Some(provenance));

    let loaded = state
        .results
        .analysis
        .eye_diagram_state
        .timebase_provenance();
    assert!(eye_matches_timebase(loaded, stated));
    assert!(!eye_matches_timebase(loaded, EyeTimebase::Auto));
    assert!(!eye_matches_timebase(
        loaded,
        EyeTimebase::Explicit {
            unit_interval: 2e-9
        }
    ));
}

/// The incoherence mark rides on the provenance the staleness comparison
/// reads, so it has to be invisible to it. If it were not, the eye would
/// disagree with the rate that produced it, be discarded, be rebuilt with
/// the same mark, and disagree again — a rebuild every frame for as long
/// as the reader leaves the rate set.
#[test]
fn the_incoherence_mark_never_makes_the_loaded_eye_stale() {
    let mut state = AppState::default();
    let (time, signal) = clock(60, 0.137e-9);
    let stated = EyeTimebase::Explicit {
        unit_interval: 1.37e-9,
    };
    let (data, provenance) = build_eye_from_waveform(&time, &signal, stated);
    assert_eq!(
        provenance,
        EyeTimebaseProvenance::Explicit {
            unit_interval: 1.37e-9,
            incoherent: true,
        }
    );
    state
        .results
        .analysis
        .eye_diagram_state
        .load_data_with_timebase(data.expect("folds"), Some(provenance));

    let loaded = state
        .results
        .analysis
        .eye_diagram_state
        .timebase_provenance();
    // Every frame asks the same question and must get the same answer.
    for frame in 0..3 {
        assert!(
            eye_matches_timebase(loaded, stated),
            "the incoherent eye went stale against its own rate on frame {frame}"
        );
    }
    // A different rate is still a different measurement.
    assert!(!eye_matches_timebase(
        loaded,
        EyeTimebase::Explicit {
            unit_interval: 1e-9
        }
    ));
}

/// The reader's rate is remembered per result and forgotten when it goes
/// back to automatic, so a session file carries only stated rates.
#[test]
fn a_stated_rate_is_kept_against_its_own_result() {
    let mut state = AppState::default();
    let (first, second) = (owner(), owner());
    assert_eq!(
        state.results.results.eye_timebase_for(first),
        EyeTimebase::Auto
    );

    let stated = EyeTimebase::Explicit {
        unit_interval: 400e-12,
    };
    state.results.results.set_eye_timebase(first, stated);
    assert_eq!(state.results.results.eye_timebase_for(first), stated);
    assert_eq!(
        state.results.results.eye_timebase_for(second),
        EyeTimebase::Auto
    );

    state
        .results
        .results
        .set_eye_timebase(first, EyeTimebase::Auto);
    assert_eq!(
        state.results.results.eye_timebase_for(first),
        EyeTimebase::Auto
    );
    assert!(state.results.results.session.eye_timebase.is_empty());
}

/// A finished transient retires the spectrum and the eye of the result it
/// replaces and frees their owners; neither is prepared until its sheet is
/// opened.
#[test]
fn a_finished_transient_retires_the_spectrum_and_eye_it_replaces() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    seed_fft(&mut state, owner());
    assert!(state.results.analysis.fft_state.has_data());

    controller.invalidate_transient_post_views(&mut state);

    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert!(controller.transient_post.fft_task.is_none());
    assert!(controller.transient_post.fft_loaded.is_none());
}
