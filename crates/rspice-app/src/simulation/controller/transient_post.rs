//! Transient viewer data.
//!
//! Prepares and caches what the transient viewers draw, so switching between
//! viewers after a run does not re-derive the same traces.

use super::*;
use crate::state::{
    ActiveViewer, AnalysisType, RunEvent, SharedWaveformValues, SpecializedViewerCacheProvenance,
};
use rspice_results_ui::eye_diagram::{
    EyeData, EyeDataBuilder, EyeTimebase, EyeTimebaseProvenance, crossing_phase_at,
    estimate_unit_interval, fold_anchor,
};
use rspice_results_ui::fft::{FftInputError, FftInputOptions, PreparedFftInput};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedViewerLoadState {
    Ready,
    Loading,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedViewKind {
    EyeDiagram,
    Fft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DerivedViewAvailability {
    Ready,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadedDerivedView {
    pub(crate) analysis: SpecializedViewerCacheProvenance,
    pub(crate) availability: DerivedViewAvailability,
}

#[derive(Debug)]
pub struct PendingDerivedViewTask {
    pub(crate) analysis: SpecializedViewerCacheProvenance,
    pub(crate) receiver: mpsc::Receiver<DerivedViewTaskResult>,
    pub(crate) cancel_flag: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
struct DerivedWaveformSource {
    analysis: SpecializedViewerCacheProvenance,
    source_name: String,
    time: SharedWaveformValues,
    values: SharedWaveformValues,
    fft_options: FftInputOptions,
    eye_timebase: EyeTimebase,
}

/// A folded eye and the period it was folded at — including the case where
/// nothing could be folded, which the sheet still has to explain.
#[derive(Debug)]
pub(crate) struct EyeBuild {
    data: Option<EyeData>,
    provenance: EyeTimebaseProvenance,
}

#[derive(Debug)]
pub enum DerivedViewResultPayload {
    Eye(EyeBuild),
    Fft(Result<PreparedFftInput, FftInputError>),
}

#[derive(Debug)]
pub struct DerivedViewTaskResult {
    pub(crate) analysis: SpecializedViewerCacheProvenance,
    pub(crate) payload: DerivedViewResultPayload,
}

#[derive(Debug, Default)]
pub(crate) struct TransientPostCoordinator {
    eye_task: Option<PendingDerivedViewTask>,
    pub fft_task: Option<PendingDerivedViewTask>,
    pub(super) eye_loaded: Option<LoadedDerivedView>,
    pub fft_loaded: Option<LoadedDerivedView>,
}

impl SimulationController {
    pub fn ensure_transient_viewer_data(
        &mut self,
        state: &mut impl RunHost,
        viewer: ActiveViewer,
    ) -> DerivedViewerLoadState {
        self.sync_transient_post_views(state);
        match viewer {
            ActiveViewer::EyeDiagram => self.ensure_eye_diagram_data(state),
            ActiveViewer::Fft => self.ensure_fft_data(state),
            _ => DerivedViewerLoadState::Ready,
        }
    }

    pub fn mark_transient_view_ready(&mut self, state: &mut impl RunHost, viewer: ActiveViewer) {
        let Some(analysis) = self.active_transient_analysis_key(state.model()) else {
            return;
        };

        match viewer {
            ActiveViewer::EyeDiagram if state.viewer_reading().eye_has_traces => {
                state.apply_run_event(RunEvent::ViewerCacheConfirmed {
                    viewer,
                    owner: analysis,
                });
                self.transient_post.eye_loaded = Some(LoadedDerivedView {
                    analysis,
                    availability: DerivedViewAvailability::Ready,
                });
            }
            ActiveViewer::Fft if state.viewer_reading().fft_has_data => {
                state.apply_run_event(RunEvent::ViewerCacheConfirmed {
                    viewer,
                    owner: analysis,
                });
                self.transient_post.fft_loaded = Some(LoadedDerivedView {
                    analysis,
                    availability: DerivedViewAvailability::Ready,
                });
            }
            _ => {}
        }
    }

    pub fn sync_transient_post_views(&mut self, state: &mut impl RunHost) {
        let active_analysis = self.active_transient_analysis_key(state.model());
        self.cancel_stale_transient_task(DerivedViewKind::EyeDiagram, active_analysis);
        self.cancel_stale_transient_task(DerivedViewKind::Fft, active_analysis);

        if self
            .transient_post
            .eye_loaded
            .map(|loaded| Some(loaded.analysis) != active_analysis)
            .unwrap_or(false)
        {
            state.apply_run_event(RunEvent::EyeOutdated);
            self.transient_post.eye_loaded = None;
        }

        if self
            .transient_post
            .fft_loaded
            .map(|loaded| Some(loaded.analysis) != active_analysis)
            .unwrap_or(false)
        {
            state.apply_run_event(RunEvent::FftOutdated);
            self.transient_post.fft_loaded = None;
        }

        self.poll_transient_task(state, DerivedViewKind::EyeDiagram, active_analysis);
        self.poll_transient_task(state, DerivedViewKind::Fft, active_analysis);
    }

    pub(super) fn invalidate_transient_post_views(&mut self, state: &mut impl RunHost) {
        self.cancel_task_slot(DerivedViewKind::EyeDiagram);
        self.cancel_task_slot(DerivedViewKind::Fft);
        self.transient_post.eye_loaded = None;
        self.transient_post.fft_loaded = None;
        state.apply_run_event(RunEvent::TransientResultReplaced);
    }

    pub(super) fn prime_transient_fft_source_selection(&mut self, state: &mut impl RunHost) {
        let Some(source) = self.current_transient_waveform_source(state) else {
            return;
        };
        state.apply_run_event(RunEvent::FftSourceChosen {
            source: source.source_name,
        });
    }

    fn ensure_eye_diagram_data(&mut self, state: &mut impl RunHost) -> DerivedViewerLoadState {
        // The reader's bit period is checked before anything else, including
        // whether an eye is already loaded: an eye folded at the old period
        // is not this eye at a different zoom, it is a different measurement.
        // Editing the rate has to rebuild, not be swallowed by the
        // already-have-data shortcut below.
        if let Some(analysis) = self.active_transient_analysis_key(state.model()) {
            let requested = state.eye_timebase_for(analysis);
            if !eye_matches_timebase(state.viewer_reading().eye_timebase.as_ref(), requested) {
                self.cancel_task_slot(DerivedViewKind::EyeDiagram);
                self.transient_post.eye_loaded = None;
                state.apply_run_event(RunEvent::EyeTimebaseChanged);
            }
        }

        if state.viewer_reading().eye_has_traces {
            self.mark_transient_view_ready(state, ActiveViewer::EyeDiagram);
            return DerivedViewerLoadState::Ready;
        }

        let Some(active_analysis) = self.active_transient_analysis_key(state.model()) else {
            return DerivedViewerLoadState::Unavailable;
        };
        if let Some(loaded) = self.transient_post.eye_loaded
            && loaded.analysis == active_analysis
        {
            return match loaded.availability {
                DerivedViewAvailability::Ready => DerivedViewerLoadState::Ready,
                DerivedViewAvailability::Unavailable => DerivedViewerLoadState::Unavailable,
            };
        }
        if self
            .transient_post
            .eye_task
            .as_ref()
            .map(|task| task.analysis == active_analysis)
            .unwrap_or(false)
        {
            return DerivedViewerLoadState::Loading;
        }

        let Some(source) = self.current_transient_waveform_source(state) else {
            self.transient_post.eye_loaded = Some(LoadedDerivedView {
                analysis: active_analysis,
                availability: DerivedViewAvailability::Unavailable,
            });
            return DerivedViewerLoadState::Unavailable;
        };
        self.spawn_derived_view_task(DerivedViewKind::EyeDiagram, source);
        DerivedViewerLoadState::Loading
    }

    fn ensure_fft_data(&mut self, state: &mut impl RunHost) -> DerivedViewerLoadState {
        if state.viewer_reading().fft_has_data {
            self.mark_transient_view_ready(state, ActiveViewer::Fft);
            return DerivedViewerLoadState::Ready;
        }

        let Some(active_analysis) = self.active_transient_analysis_key(state.model()) else {
            return DerivedViewerLoadState::Unavailable;
        };
        if let Some(loaded) = self.transient_post.fft_loaded
            && loaded.analysis == active_analysis
        {
            match loaded.availability {
                DerivedViewAvailability::Ready => {
                    // `has_data` was false above, so this marker became stale
                    // after an in-place viewer recompute or explicit clear.
                    state.apply_run_event(RunEvent::FftEmptied);
                    if state.viewer_reading().fft_failed {
                        self.transient_post.fft_loaded = Some(LoadedDerivedView {
                            analysis: active_analysis,
                            availability: DerivedViewAvailability::Unavailable,
                        });
                        return DerivedViewerLoadState::Unavailable;
                    }
                    self.transient_post.fft_loaded = None;
                }
                DerivedViewAvailability::Unavailable => {
                    return DerivedViewerLoadState::Unavailable;
                }
            }
        }
        if self
            .transient_post
            .fft_task
            .as_ref()
            .map(|task| task.analysis == active_analysis)
            .unwrap_or(false)
        {
            return DerivedViewerLoadState::Loading;
        }

        let Some(source) = self.current_transient_waveform_source(state) else {
            self.transient_post.fft_loaded = Some(LoadedDerivedView {
                analysis: active_analysis,
                availability: DerivedViewAvailability::Unavailable,
            });
            return DerivedViewerLoadState::Unavailable;
        };
        state.apply_run_event(RunEvent::FftSourceChosen {
            source: source.source_name.clone(),
        });
        self.spawn_derived_view_task(DerivedViewKind::Fft, source);
        DerivedViewerLoadState::Loading
    }

    pub fn poll_transient_task(
        &mut self,
        state: &mut impl RunHost,
        view: DerivedViewKind,
        active_analysis: Option<SpecializedViewerCacheProvenance>,
    ) {
        let task_slot = self.task_slot_mut(view);
        let Some(task) = task_slot.as_mut() else {
            return;
        };

        let message = match task.receiver.try_recv() {
            Ok(message) => message,
            Err(mpsc::TryRecvError::Empty) => return,
            Err(mpsc::TryRecvError::Disconnected) => {
                let disconnected_analysis = task.analysis;
                *task_slot = None;
                // Cancellation removes the task slot before setting its flag,
                // so a disconnected receiver that is still installed is an
                // unexpected worker termination, not cancellation. Attribute
                // it only to the still-active FFT provenance.
                if view == DerivedViewKind::Fft && Some(disconnected_analysis) == active_analysis {
                    state.apply_run_event(RunEvent::FftWorkerLost);
                    self.transient_post.fft_loaded = Some(LoadedDerivedView {
                        analysis: disconnected_analysis,
                        availability: DerivedViewAvailability::Unavailable,
                    });
                }
                return;
            }
        };
        *task_slot = None;

        if Some(message.analysis) != active_analysis {
            return;
        }

        match (view, message.payload) {
            (DerivedViewKind::EyeDiagram, DerivedViewResultPayload::Eye(build)) => {
                let available = build.data.is_some();
                state.apply_run_event(RunEvent::EyeFolded {
                    owner: Some(message.analysis),
                    eye: build.data,
                    timebase: build.provenance,
                });
                self.transient_post.eye_loaded = Some(LoadedDerivedView {
                    analysis: message.analysis,
                    availability: if available {
                        DerivedViewAvailability::Ready
                    } else {
                        DerivedViewAvailability::Unavailable
                    },
                });
            }
            (DerivedViewKind::Fft, DerivedViewResultPayload::Fft(input)) => {
                state.apply_run_event(RunEvent::FftInputPrepared {
                    owner: Some(message.analysis),
                    input,
                });
                // The viewer holds a spectrum exactly when it accepted the input.
                let available = state.viewer_reading().fft_has_data;
                self.transient_post.fft_loaded = Some(LoadedDerivedView {
                    analysis: message.analysis,
                    availability: if available {
                        DerivedViewAvailability::Ready
                    } else {
                        DerivedViewAvailability::Unavailable
                    },
                });
            }
            _ => {}
        }
    }

    fn spawn_derived_view_task(&mut self, view: DerivedViewKind, source: DerivedWaveformSource) {
        self.cancel_task_slot(view);

        let (sender, receiver) = mpsc::channel();
        let cancel_flag = Arc::new(AtomicBool::new(false));
        let task_cancel_flag = Arc::clone(&cancel_flag);
        let task_source = source.clone();

        spawn_or_inline(move || {
            let payload = match view {
                DerivedViewKind::EyeDiagram => {
                    if task_cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }
                    DerivedViewResultPayload::Eye(build_eye_diagram_data(&task_source))
                }
                DerivedViewKind::Fft => {
                    if task_cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }
                    let prepared = build_fft_prepared_input(&task_source);
                    DerivedViewResultPayload::Fft(prepared)
                }
            };
            if task_cancel_flag.load(Ordering::Relaxed) {
                return;
            }
            let _ = sender.send(DerivedViewTaskResult {
                analysis: task_source.analysis,
                payload,
            });
        });

        *self.task_slot_mut(view) = Some(PendingDerivedViewTask {
            analysis: source.analysis,
            receiver,
            cancel_flag,
        });
    }

    fn cancel_stale_transient_task(
        &mut self,
        view: DerivedViewKind,
        active_analysis: Option<SpecializedViewerCacheProvenance>,
    ) {
        let should_cancel = self
            .task_slot(view)
            .map(|task| Some(task.analysis) != active_analysis)
            .unwrap_or(false);
        if should_cancel {
            self.cancel_task_slot(view);
        }
    }

    pub fn cancel_task_slot(&mut self, view: DerivedViewKind) {
        if let Some(task) = self.task_slot_mut(view).take() {
            task.cancel_flag.store(true, Ordering::Relaxed);
        }
    }

    fn task_slot(&self, view: DerivedViewKind) -> Option<&PendingDerivedViewTask> {
        match view {
            DerivedViewKind::EyeDiagram => self.transient_post.eye_task.as_ref(),
            DerivedViewKind::Fft => self.transient_post.fft_task.as_ref(),
        }
    }

    fn task_slot_mut(&mut self, view: DerivedViewKind) -> &mut Option<PendingDerivedViewTask> {
        match view {
            DerivedViewKind::EyeDiagram => &mut self.transient_post.eye_task,
            DerivedViewKind::Fft => &mut self.transient_post.fft_task,
        }
    }

    fn active_transient_analysis_key(
        &self,
        model: &Model,
    ) -> Option<SpecializedViewerCacheProvenance> {
        let analysis = model.simulation.active_analysis()?;
        Self::analysis_supports_transient_derivation(analysis.analysis_type)
            .then(|| model.active_specialized_viewer_cache_provenance())
            .flatten()
    }

    fn current_transient_waveform_source(
        &self,
        state: &impl RunHost,
    ) -> Option<DerivedWaveformSource> {
        let analysis = state.model().simulation.active_analysis()?;
        let analysis_key = self.active_transient_analysis_key(state.model())?;
        let preferred_source = state.viewer_reading().fft_source;
        let (source_name, waveform) =
            Self::fft_source_waveform_from_state(&analysis.waveforms, preferred_source.as_deref())?;

        Some(DerivedWaveformSource {
            analysis: analysis_key,
            source_name,
            time: Arc::clone(&waveform.x),
            values: Arc::clone(&waveform.y),
            fft_options: state.fft_input_options(&waveform.x),
            eye_timebase: state.eye_timebase_for(analysis_key),
        })
    }

    pub fn analysis_supports_transient_derivation(analysis_type: AnalysisType) -> bool {
        analysis_type.is_time_domain()
    }
}

/// Does the eye currently on screen answer the bit period being asked for?
///
/// The loaded data's own provenance is the source of truth rather than any
/// bookkeeping beside it, because it is what the sheet is quoting to the
/// reader. An eye with no recorded provenance makes no claim and is left
/// alone.
///
/// Only what was *asked for* is compared — the variant and the unit interval.
/// What the fold then discovered about the data (that its crossings do not
/// agree at the stated rate) is an answer, not a request: keying on it would
/// make every incoherent eye disagree with the rate that produced it and
/// rebuild itself on every frame, forever.
pub fn eye_matches_timebase(
    loaded: Option<&EyeTimebaseProvenance>,
    requested: EyeTimebase,
) -> bool {
    let Some(provenance) = loaded else {
        return true;
    };
    match (provenance, requested) {
        (
            EyeTimebaseProvenance::Auto { .. } | EyeTimebaseProvenance::AutoRejected(_),
            EyeTimebase::Auto,
        ) => true,
        (
            EyeTimebaseProvenance::Explicit {
                unit_interval: had, ..
            },
            EyeTimebase::Explicit {
                unit_interval: want,
            },
        ) => had.to_bits() == want.to_bits(),
        _ => false,
    }
}

/// Number of unit intervals the folded window spans.
const EYE_WINDOW_UI: u32 = 2;
/// Bits skipped at the head of the record while the circuit settles.
const EYE_SKIP_INITIAL: usize = 2;

/// Fold a transient waveform into an eye, and say what it was folded at.
///
/// The one place the eye is built. Both the eager path that seeds the viewer
/// when a run completes and the lazy path that builds it on first view go
/// through here, so the two cannot drift into folding the same waveform at
/// different periods — or, as they did, into two copies of the same estimator
/// with the same defect.
pub fn build_eye_from_waveform(
    time: &[f64],
    values: &[f64],
    timebase: EyeTimebase,
) -> (Option<EyeData>, EyeTimebaseProvenance) {
    // The anchor puts the crossings at half-integer phases so the opening
    // lands at the centre of the window, where the height, the noise slice
    // and every compliance mask are measured.
    let (unit_interval, anchor, provenance) = match timebase {
        EyeTimebase::Auto => match estimate_unit_interval(time, values) {
            Ok(estimate) => (
                estimate.unit_interval,
                fold_anchor(estimate.mean_crossing_phase, estimate.unit_interval),
                EyeTimebaseProvenance::Auto {
                    unit_interval: estimate.unit_interval,
                    edge_count: estimate.crossing_count,
                    rms_residual_ui: estimate.rms_residual_ui,
                    low_confidence: estimate.low_confidence,
                },
            ),
            // Nothing is drawn: an eye folded at a period the waveform does
            // not have is a picture of the fold, not of the signal.
            Err(rejection) => {
                return (None, EyeTimebaseProvenance::AutoRejected(rejection));
            }
        },
        EyeTimebase::Explicit { unit_interval } => {
            // The stated rate is folded at either way. Whether the crossings
            // agree with it is a separate fact and the reader's to keep: an
            // eye folded at a rate the data does not run at still draws, and
            // nothing about the picture admits it.
            let fit = crossing_phase_at(time, values, unit_interval);
            (
                unit_interval,
                fit.and_then(|fit| fold_anchor(fit.phase, unit_interval)),
                EyeTimebaseProvenance::Explicit {
                    unit_interval,
                    incoherent: fit.is_some_and(|fit| !fit.is_coherent()),
                },
            )
        }
    };

    let eye_data = EyeDataBuilder::new()
        .bit_period(unit_interval)
        .ui_count(EYE_WINDOW_UI)
        .skip_initial(EYE_SKIP_INITIAL)
        .fold_anchor(anchor)
        .build(time, values);
    ((eye_data.trace_count() > 0).then_some(eye_data), provenance)
}

fn build_eye_diagram_data(source: &DerivedWaveformSource) -> EyeBuild {
    let (data, provenance) =
        build_eye_from_waveform(&source.time, &source.values, source.eye_timebase);
    EyeBuild { data, provenance }
}

fn build_fft_prepared_input(
    source: &DerivedWaveformSource,
) -> Result<PreparedFftInput, FftInputError> {
    rspice_results_ui::fft::prepare_fft_input_with_options(
        &source.source_name,
        &source.time,
        &source.values,
        source.fft_options,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::controller::test_execution::clock;

    /// A stated rate is the rate. The builder must fold at it rather than at
    /// whatever it would have recovered, or the control is decorative.
    #[test]
    fn an_explicit_rate_overrides_what_the_waveform_would_have_said() {
        let (time, signal) = clock(200, 0.137e-9);

        let (auto, auto_provenance) = build_eye_from_waveform(&time, &signal, EyeTimebase::Auto);
        let auto = auto.expect("a clock folds");
        assert!((auto.bit_period - 1e-9).abs() <= 1e-12);
        assert!(matches!(
            auto_provenance,
            EyeTimebaseProvenance::Auto { .. }
        ));

        let (stated, stated_provenance) = build_eye_from_waveform(
            &time,
            &signal,
            EyeTimebase::Explicit {
                unit_interval: 2e-9,
            },
        );
        let stated = stated.expect("an explicit rate folds");
        assert!((stated.bit_period - 2e-9).abs() <= 1e-18);
        // Twice the true rate is the two-bit fold this estimator exists to
        // avoid: the 1010 clock's crossings land at two opposite phases of
        // the doubled window and agree on neither. The reader gets the rate
        // they asked for, marked.
        assert_eq!(
            stated_provenance,
            EyeTimebaseProvenance::Explicit {
                unit_interval: 2e-9,
                incoherent: true,
            }
        );
    }

    /// A stated rate the waveform does not run at still folds — the reader
    /// asked for it — but the disagreement is the reader's to see. The fit
    /// that produces the fold anchor already measures it; discarding it left
    /// an eye whose every figure was a property of the fold, presented
    /// exactly like one measured at the signal's own rate.
    #[test]
    fn a_stated_rate_the_crossings_reject_folds_and_carries_the_disagreement() {
        let (time, signal) = clock(200, 0.137e-9);

        // 1.37 ns against a 1 ns clock: 37 % off, and the crossings walk the
        // whole period rather than sitting at one phase within it.
        let (data, provenance) = build_eye_from_waveform(
            &time,
            &signal,
            EyeTimebase::Explicit {
                unit_interval: 1.37e-9,
            },
        );
        assert!(data.is_some(), "the rate the reader stated still folds");
        assert_eq!(
            provenance,
            EyeTimebaseProvenance::Explicit {
                unit_interval: 1.37e-9,
                incoherent: true,
            }
        );

        // The rate the clock actually runs at carries no such mark.
        let (data, provenance) = build_eye_from_waveform(
            &time,
            &signal,
            EyeTimebase::Explicit {
                unit_interval: 1e-9,
            },
        );
        assert!(data.is_some());
        assert_eq!(
            provenance,
            EyeTimebaseProvenance::Explicit {
                unit_interval: 1e-9,
                incoherent: false,
            }
        );
    }

    /// A waveform with no recoverable bit period folds nothing, and says so
    /// rather than leaving the sheet to guess why it is empty.
    #[test]
    fn an_unrecoverable_bit_period_is_reported_rather_than_guessed() {
        let time: Vec<f64> = (0..2000).map(|i| i as f64 * 10e-12).collect();
        let signal = vec![0.5; time.len()];

        let (data, provenance) = build_eye_from_waveform(&time, &signal, EyeTimebase::Auto);
        assert!(data.is_none());
        assert!(matches!(provenance, EyeTimebaseProvenance::AutoRejected(_)));
        assert!(
            provenance
                .rejection_hint("V(out)")
                .is_some_and(|hint| hint.contains("V(out)"))
        );
    }
}
