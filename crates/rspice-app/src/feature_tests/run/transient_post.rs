//! The transient coordinator's derived views, observed through the results
//! viewer they load.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

use rspice_results_ui::eye_diagram::{EyeTimebase, EyeTimebaseProvenance};
use rspice_results_ui::fft::{
    FftFailure, FftInputError, PreparedFftInput, WindowFunction,
    data::{FftBuildError, SpectrumAnalysisError},
};

use crate::product::{AnalysisInstanceId, DatasetId};
use crate::simulation::controller::SimulationController;
use crate::simulation::controller::tests::synthetic_result_provenance;
use crate::simulation::controller::transient_post::tests::clock;
use crate::simulation::controller::transient_post::{
    DerivedViewAvailability, DerivedViewKind, DerivedViewResultPayload, DerivedViewTaskResult,
    DerivedViewerLoadState, LoadedDerivedView, PendingDerivedViewTask, build_eye_from_waveform,
    eye_matches_timebase,
};
use crate::state::{
    ActiveViewer, AnalysisResult, AnalysisType, SimulationRun, SpecializedViewerCacheProvenance,
};
use crate::workbench::app_state::AppState;

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
        })
        .expect("qualified FFT ownership fixture");
    state.bind_specialized_viewer_cache(ActiveViewer::Fft, analysis);
}

fn install_fft_task(
    controller: &mut SimulationController,
    analysis: SpecializedViewerCacheProvenance,
) -> (mpsc::Sender<DerivedViewTaskResult>, Arc<AtomicBool>) {
    let (sender, receiver) = mpsc::channel();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    controller.transient_post.fft_task = Some(PendingDerivedViewTask {
        analysis,
        receiver,
        cancel_flag: Arc::clone(&cancel_flag),
    });
    (sender, cancel_flag)
}

#[test]
fn matching_fft_input_failure_owns_and_clears_only_its_active_cache() {
    let analysis = owner();
    let mut state = AppState::default();
    seed_fft(&mut state, analysis);
    let mut controller = SimulationController::new();
    let (sender, _) = install_fft_task(&mut controller, analysis);
    sender
        .send(DerivedViewTaskResult {
            analysis,
            payload: DerivedViewResultPayload::Fft(Err(FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16,
            })),
        })
        .expect("installed local receiver");

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
    let (sender, _) = install_fft_task(&mut controller, stale);
    sender
        .send(DerivedViewTaskResult {
            analysis: stale,
            payload: DerivedViewResultPayload::Fft(Err(FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16,
            })),
        })
        .expect("installed local receiver");

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
    let (sender, cancelled) = install_fft_task(&mut controller, stale);

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
            })),
        })
        .expect("completion arrives before the next application poll");
    controller.sync_transient_post_views(&mut state);

    assert!(cancelled.load(Ordering::Relaxed));
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
    let (sender, cancel_flag) = install_fft_task(&mut controller, analysis);

    controller.cancel_task_slot(DerivedViewKind::Fft);

    assert!(cancel_flag.load(Ordering::Relaxed));
    assert!(controller.transient_post.fft_task.is_none());
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
    let (sender, _) = install_fft_task(&mut controller, analysis);
    drop(sender);

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
    assert_eq!(state.eye_timebase_for(first), EyeTimebase::Auto);

    let stated = EyeTimebase::Explicit {
        unit_interval: 400e-12,
    };
    state.set_eye_timebase(first, stated);
    assert_eq!(state.eye_timebase_for(first), stated);
    assert_eq!(state.eye_timebase_for(second), EyeTimebase::Auto);

    state.set_eye_timebase(first, EyeTimebase::Auto);
    assert_eq!(state.eye_timebase_for(first), EyeTimebase::Auto);
    assert!(state.results.results.session.eye_timebase.is_empty());
}

#[test]
fn transient_specialized_views_never_outlive_their_retained_source() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    let x = (0..32)
        .map(|index| index as f64 * 1.0e-9)
        .collect::<Vec<_>>();
    let y = (0..32)
        .map(|index| ((index as f64) * 0.4).sin())
        .collect::<Vec<_>>();
    let retained = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::RetainedTrace::new("V(out)", x, y, "#ffffff"),
    ]);

    controller.populate_transient_post_views(&mut state, &retained);
    assert!(state.results.analysis.fft_state.has_data());

    let no_outputs = AnalysisResult::new(1, AnalysisType::Transient, "TRAN");
    controller.populate_transient_post_views(&mut state, &no_outputs);
    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.cache_authority.fft.is_none());
}

#[test]
fn malformed_eager_fft_input_clears_the_prior_spectrum_and_keeps_the_typed_failure() {
    let mut controller = SimulationController::new();
    let mut state = AppState::default();
    let run_sequence = state.model.simulation.start_run().id;
    controller.current_run_id = Some(run_sequence);
    controller.current_provenance = Some(synthetic_result_provenance());
    let expected_owner = controller
        .in_flight_specialized_viewer_provenance(&state.model)
        .expect("the active prepared analysis owns specialized-viewer caches");
    let valid_time = (0..17).map(|index| index as f64).collect::<Vec<_>>();
    let valid_values = (0..17)
        .map(|index| (index as f64 * 0.25).sin())
        .collect::<Vec<_>>();
    let valid = AnalysisResult::new(1, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::RetainedTrace::new("V(out)", valid_time.clone(), valid_values, "#ffffff"),
    ]);
    controller.populate_transient_post_views(&mut state, &valid);
    assert!(state.results.analysis.fft_state.has_data());
    assert_eq!(
        state.results.analysis.cache_authority.fft,
        Some(expected_owner)
    );
    assert!(controller.transient_post.fft_loaded.is_some_and(|loaded| {
        loaded.analysis == expected_owner
            && matches!(loaded.availability, DerivedViewAvailability::Ready)
    }));

    let malformed = AnalysisResult::new(2, AnalysisType::Transient, "TRAN").with_waveforms(vec![
        crate::state::RetainedTrace::new("V(out)", valid_time, vec![0.0; 16], "#ffffff"),
    ]);
    controller.populate_transient_post_views(&mut state, &malformed);

    assert!(!state.results.analysis.fft_state.has_data());
    assert!(state.results.analysis.fft_state.source_cache.is_none());
    assert!(state.results.analysis.cache_authority.fft.is_none());
    assert!(controller.transient_post.fft_loaded.is_some_and(|loaded| {
        loaded.analysis == expected_owner
            && matches!(loaded.availability, DerivedViewAvailability::Unavailable)
    }));
    assert!(matches!(
        state.results.analysis.fft_state.last_error,
        Some(rspice_results_ui::fft::FftFailure::Input(
            rspice_results_ui::fft::FftInputError::LengthMismatch {
                time_count: 17,
                value_count: 16
            }
        ))
    ));
}
