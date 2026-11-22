//! Transient viewer data.
//!
//! Prepares and caches what the transient viewers draw, so switching between
//! viewers after a run does not re-derive the same traces. The spectrum's
//! input and the folded eye are prepared on Results jobs beside the window.

use super::*;
use crate::simulation::ViewerHost;
use crate::state::{
    ActiveViewer, AnalysisType, RunEvent, SharedWaveformValues, SpecializedViewerCacheProvenance,
};
use rspice_app_services::workbench::results_job::{JobOutcome, JobStop, ResultsJob};
use rspice_results::waveform::HeldColumn;
use rspice_results_ui::eye_diagram::{
    EyeData, EyeDataBuilder, EyeFold, EyeTimebase, EyeTimebaseProvenance, centring_delay,
    clock_trigger, crossing_phase_at, estimate_unit_interval, fold_anchor,
};
use rspice_results_ui::fft::{FftInputError, FftInputOptions, PreparedFftInput};

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
    pub analysis: SpecializedViewerCacheProvenance,
    pub availability: DerivedViewAvailability,
}

/// A derived view being prepared for the analysis it was asked for.
#[derive(Debug)]
pub struct PendingDerivedViewTask {
    pub analysis: SpecializedViewerCacheProvenance,
    /// The preparation. A job that fails is a preparation that stopped unexpectedly.
    pub job: ResultsJob<DerivedViewTaskResult>,
}

#[derive(Debug, Clone)]
struct DerivedWaveformSource {
    analysis: SpecializedViewerCacheProvenance,
    source_name: String,
    time: SharedWaveformValues,
    values: SharedWaveformValues,
    fft_options: FftInputOptions,
    eye_fold: EyeFold,
    /// The time and value columns of the clock signal the eye is triggered
    /// from, when it is; `Err` names a clock the result does not hold.
    eye_clock: Option<Result<(SharedWaveformValues, SharedWaveformValues), String>>,
}

/// A folded eye and the period it was folded at — including the case where
/// nothing could be folded, which the sheet still has to explain.
#[derive(Debug)]
pub struct EyeBuild {
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
    pub analysis: SpecializedViewerCacheProvenance,
    pub payload: DerivedViewResultPayload,
}

#[derive(Debug, Default)]
pub struct TransientPostCoordinator {
    eye_task: Option<PendingDerivedViewTask>,
    pub fft_task: Option<PendingDerivedViewTask>,
    pub(super) eye_loaded: Option<LoadedDerivedView>,
    pub fft_loaded: Option<LoadedDerivedView>,
    /// What the eye being folded, or on screen, was asked to be folded by.
    eye_fold: Option<EyeFold>,
}

impl SimulationController {
    pub fn ensure_transient_viewer_data(
        &mut self,
        state: &mut impl ViewerHost,
        viewer: ActiveViewer,
    ) -> DerivedViewerLoadState {
        self.sync_transient_post_views(state);
        match viewer {
            ActiveViewer::EyeDiagram => self.ensure_eye_diagram_data(state),
            ActiveViewer::Fft => self.ensure_fft_data(state),
            _ => DerivedViewerLoadState::Ready,
        }
    }

    pub fn mark_transient_view_ready(&mut self, state: &mut impl ViewerHost, viewer: ActiveViewer) {
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

    pub fn sync_transient_post_views(&mut self, state: &mut impl ViewerHost) {
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

    /// Retire the spectrum and the eye a finished transient replaces. Neither is
    /// prepared here: each is prepared when its sheet is opened, as a job reading
    /// windows of the record, so a run's completion reads none of its columns.
    pub fn invalidate_transient_post_views(&mut self, state: &mut impl ViewerHost) {
        self.cancel_task_slot(DerivedViewKind::EyeDiagram);
        self.cancel_task_slot(DerivedViewKind::Fft);
        self.transient_post.eye_loaded = None;
        self.transient_post.fft_loaded = None;
        self.transient_post.eye_fold = None;
        state.apply_run_event(RunEvent::TransientResultReplaced);
    }

    fn ensure_eye_diagram_data(&mut self, state: &mut impl ViewerHost) -> DerivedViewerLoadState {
        // The reader's bit period is checked before anything else, including
        // whether an eye is already loaded: an eye folded at the old period
        // is not this eye at a different zoom, it is a different measurement.
        // Editing the rate has to rebuild, not be swallowed by the
        // already-have-data shortcut below.
        // The same holds for the signal and the clock it is folded by.
        if let Some(analysis) = self.active_transient_analysis_key(state.model()) {
            let requested = state.eye_fold_for(analysis);
            if !eye_answers_fold(
                state.viewer_reading().eye_timebase.as_ref(),
                self.transient_post.eye_fold.as_ref(),
                &requested,
            ) {
                self.cancel_task_slot(DerivedViewKind::EyeDiagram);
                self.transient_post.eye_loaded = None;
                self.transient_post.eye_fold = None;
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
        self.transient_post.eye_fold = Some(source.eye_fold.clone());
        self.spawn_derived_view_task(DerivedViewKind::EyeDiagram, source);
        DerivedViewerLoadState::Loading
    }

    fn ensure_fft_data(&mut self, state: &mut impl ViewerHost) -> DerivedViewerLoadState {
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
        state: &mut impl ViewerHost,
        view: DerivedViewKind,
        active_analysis: Option<SpecializedViewerCacheProvenance>,
    ) {
        let task_slot = self.task_slot_mut(view);
        let Some(task) = task_slot.as_mut() else {
            return;
        };
        if !task.job.is_finished() {
            return;
        }

        let message = match task.job.take_outcome() {
            Some(JobOutcome::Done(message)) => message,
            Some(JobOutcome::Cancelled) | None => {
                *task_slot = None;
                return;
            }
            Some(JobOutcome::Failed(_)) => {
                let disconnected_analysis = task.analysis;
                *task_slot = None;
                // Cancellation removes the task slot before it cancels the
                // job, so a job that failed while still installed is an
                // unexpected stop of the preparation, not cancellation.
                // Attribute it only to the still-active FFT provenance.
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

        let analysis = source.analysis;
        let (name, step) = match view {
            DerivedViewKind::EyeDiagram => ("eye-fold", "Folding the eye"),
            DerivedViewKind::Fft => ("fft-preparation", "Preparing the spectrum"),
        };
        let job = ResultsJob::start(name, move |job| {
            job.step(step, 0);
            // A preparation that panics ends as a failed job, which the
            // poll reports as a preparation that stopped unexpectedly.
            let payload = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match view {
                DerivedViewKind::EyeDiagram => {
                    DerivedViewResultPayload::Eye(build_eye_diagram_data(&source))
                }
                DerivedViewKind::Fft => {
                    DerivedViewResultPayload::Fft(build_fft_prepared_input(&source))
                }
            }))
            .map_err(|_| JobStop::Failed(format!("{step} stopped unexpectedly")))?;
            job.check()?;
            Ok(DerivedViewTaskResult {
                analysis: source.analysis,
                payload,
            })
        });

        *self.task_slot_mut(view) = Some(PendingDerivedViewTask { analysis, job });
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
            task.job.cancel();
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
        state: &impl ViewerHost,
    ) -> Option<DerivedWaveformSource> {
        let analysis = state.model().simulation.active_analysis()?;
        let analysis_key = self.active_transient_analysis_key(state.model())?;
        let eye_fold = state.eye_fold_for(analysis_key);
        // The eye folds the signal its sheet names, or the spectrum's.
        let preferred_source = eye_fold
            .source
            .clone()
            .or_else(|| state.viewer_reading().fft_source);
        let (source_name, waveform) =
            Self::fft_source_waveform_from_state(&analysis.waveforms, preferred_source.as_deref())?;
        let eye_clock = eye_fold.clock.as_ref().map(|clock| {
            analysis
                .waveforms
                .iter()
                .find(|waveform| waveform.name == clock.signal)
                .map(|waveform| (waveform.x.clone(), waveform.y.clone()))
                .ok_or_else(|| format!("The clock signal {} is not in this result", clock.signal))
        });

        Some(DerivedWaveformSource {
            analysis: analysis_key,
            source_name,
            time: waveform.x.clone(),
            values: waveform.y.clone(),
            fft_options: state.fft_input_options(),
            eye_fold,
            eye_clock,
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
        // A record that could not be read is not read again for another rate.
        (EyeTimebaseProvenance::Unreadable(_), _) => true,
        _ => false,
    }
}

/// Does the eye on screen, or the one being folded, answer the fold being
/// asked for: the same signal, clock and offset it was asked to be folded
/// by, and the period [`eye_matches_timebase`] reads off its provenance? An
/// eye triggered from a clock signal takes its period from the clock, so
/// only the clock is compared.
pub fn eye_answers_fold(
    loaded: Option<&EyeTimebaseProvenance>,
    folded_by: Option<&EyeFold>,
    requested: &EyeFold,
) -> bool {
    if let Some(folded_by) = folded_by {
        if !folded_by.same_trigger(requested) {
            return false;
        }
        if requested.clock.is_some() {
            return true;
        }
    }
    eye_matches_timebase(loaded, requested.timebase)
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

/// A source's time and value columns, as the reader service holds them.
type SourceColumns = (HeldColumn, HeldColumn);

/// Both columns of an eye's source. An eye folds every sample of the record, so the job asks the
/// reader service for them whole and waits, off the frame; a record past the loaded-results
/// budget is refused.
fn whole_source(source: &DerivedWaveformSource) -> Result<SourceColumns, String> {
    Ok((
        source.time.fetch_whole_waiting()?,
        source.values.fetch_whole_waiting()?,
    ))
}

fn build_eye_diagram_data(source: &DerivedWaveformSource) -> EyeBuild {
    let (data, provenance) = match whole_source(source) {
        Ok((time, values)) => match &source.eye_clock {
            None => build_eye_on_ideal_clock(&time, &values, &source.eye_fold),
            Some(Err(reason)) => (None, EyeTimebaseProvenance::ClockRejected(reason.clone())),
            Some(Ok((clock_time, clock_values))) => {
                match (
                    clock_time.fetch_whole_waiting(),
                    clock_values.fetch_whole_waiting(),
                ) {
                    (Ok(clock_time), Ok(clock_values)) => build_eye_on_clock_signal(
                        &time,
                        &values,
                        (&*clock_time, &*clock_values),
                        &source.eye_fold,
                    ),
                    (Err(reason), _) | (_, Err(reason)) => {
                        (None, EyeTimebaseProvenance::Unreadable(reason))
                    }
                }
            }
        },
        Err(reason) => (None, EyeTimebaseProvenance::Unreadable(reason)),
    };
    EyeBuild { data, provenance }
}

/// Fold on an ideal clock: recovered or stated, from the offset the reader
/// stated or with the opening centred.
pub fn build_eye_on_ideal_clock(
    time: &[f64],
    values: &[f64],
    fold: &EyeFold,
) -> (Option<EyeData>, EyeTimebaseProvenance) {
    let Some(offset) = fold.offset else {
        return build_eye_from_waveform(time, values, fold.timebase);
    };
    let unit_interval = match fold.timebase {
        EyeTimebase::Explicit { unit_interval } => unit_interval,
        EyeTimebase::Auto => return build_eye_from_waveform(time, values, fold.timebase),
    };
    let incoherent =
        crossing_phase_at(time, values, unit_interval).is_some_and(|fit| !fit.is_coherent());
    let eye_data = EyeDataBuilder::new()
        .bit_period(unit_interval)
        .ui_count(EYE_WINDOW_UI)
        .skip_initial(EYE_SKIP_INITIAL)
        .fold_anchor(Some(offset))
        .build(time, values);
    (
        (eye_data.trace_count() > 0).then_some(eye_data),
        EyeTimebaseProvenance::Explicit {
            unit_interval,
            incoherent,
        },
    )
}

/// Fold from a clock signal: every acquisition starts at one of its rising
/// crossings of the threshold, delayed by the offset the reader stated or
/// by what centres the data's crossings.
pub fn build_eye_on_clock_signal(
    time: &[f64],
    values: &[f64],
    clock: (&[f64], &[f64]),
    fold: &EyeFold,
) -> (Option<EyeData>, EyeTimebaseProvenance) {
    let Some(signal) = fold.clock.as_ref() else {
        return build_eye_on_ideal_clock(time, values, fold);
    };
    let trigger = match clock_trigger(clock.0, clock.1, signal.threshold) {
        Ok(trigger) => trigger,
        Err(rejection) => {
            return (
                None,
                EyeTimebaseProvenance::ClockRejected(format!(
                    "No eye from the clock {}: {rejection}",
                    signal.signal
                )),
            );
        }
    };
    let (low, high) = values
        .iter()
        .copied()
        .filter(|value| value.is_finite())
        .fold((f64::INFINITY, f64::NEG_INFINITY), |(low, high), value| {
            (low.min(value), high.max(value))
        });
    let level = 0.5 * (low + high);
    let delay = fold
        .offset
        .or_else(|| centring_delay(&trigger, time, values, level))
        .unwrap_or(0.0);
    let edge_count = trigger.edges.len();
    let starts: Vec<f64> = trigger.edges.iter().map(|edge| edge + delay).collect();
    let eye_data = EyeDataBuilder::new()
        .bit_period(trigger.period)
        .ui_count(EYE_WINDOW_UI)
        .skip_initial(EYE_SKIP_INITIAL)
        .triggers(starts)
        .build(time, values);
    (
        (eye_data.trace_count() > 0).then_some(eye_data),
        EyeTimebaseProvenance::Triggered {
            unit_interval: trigger.period,
            edge_count,
        },
    )
}

/// The spectrum's input: the record validated a block at a time as it is read, and only the rows
/// of its time window held, so a record kept in a results store is prepared whatever its length.
fn build_fft_prepared_input(
    source: &DerivedWaveformSource,
) -> Result<PreparedFftInput, FftInputError> {
    rspice_results_ui::fft::pipeline::prepare_fft_input_from_columns(
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
