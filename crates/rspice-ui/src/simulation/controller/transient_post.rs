//! Transient viewer data.
//!
//! Prepares and caches what the transient viewers draw, so switching between
//! viewers after a run does not re-derive the same traces.

use super::*;
use crate::analysis::eye_diagram::{
    EyeData, EyeDataBuilder, EyeTimebase, EyeTimebaseProvenance, crossing_phase_at,
    estimate_unit_interval, fold_anchor,
};
use crate::analysis::fft::{FftInputOptions, PreparedFftInput};
use crate::state::{AnalysisType, SharedWaveformValues};
use crate::workbench::app_state::{ActiveViewer, AppState, SpecializedViewerCacheProvenance};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, mpsc};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum DerivedViewerLoadState {
    Ready,
    Loading,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivedViewKind {
    EyeDiagram,
    Fft,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DerivedViewAvailability {
    Ready,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoadedDerivedView {
    analysis: SpecializedViewerCacheProvenance,
    availability: DerivedViewAvailability,
}

#[derive(Debug)]
struct PendingDerivedViewTask {
    analysis: SpecializedViewerCacheProvenance,
    receiver: mpsc::Receiver<DerivedViewTaskResult>,
    cancel_flag: Arc<AtomicBool>,
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
struct EyeBuild {
    data: Option<EyeData>,
    provenance: EyeTimebaseProvenance,
}

#[derive(Debug)]
enum DerivedViewResultPayload {
    Eye(EyeBuild),
    Fft(Option<PreparedFftInput>),
}

#[derive(Debug)]
struct DerivedViewTaskResult {
    analysis: SpecializedViewerCacheProvenance,
    payload: DerivedViewResultPayload,
}

#[derive(Debug, Default)]
pub(super) struct TransientPostCoordinator {
    eye_task: Option<PendingDerivedViewTask>,
    fft_task: Option<PendingDerivedViewTask>,
    eye_loaded: Option<LoadedDerivedView>,
    fft_loaded: Option<LoadedDerivedView>,
}

impl SimulationController {
    pub(crate) fn ensure_transient_viewer_data(
        &mut self,
        state: &mut AppState,
        viewer: ActiveViewer,
    ) -> DerivedViewerLoadState {
        self.sync_transient_post_views(state);
        match viewer {
            ActiveViewer::EyeDiagram => self.ensure_eye_diagram_data(state),
            ActiveViewer::Fft => self.ensure_fft_data(state),
            _ => DerivedViewerLoadState::Ready,
        }
    }

    pub(crate) fn mark_transient_view_ready(&mut self, state: &mut AppState, viewer: ActiveViewer) {
        let Some(analysis) = self.active_transient_analysis_key(state) else {
            return;
        };

        match viewer {
            ActiveViewer::EyeDiagram if state.analysis.eye_diagram_state.trace_count() > 0 => {
                state.bind_specialized_viewer_cache(viewer, analysis);
                self.transient_post.eye_loaded = Some(LoadedDerivedView {
                    analysis,
                    availability: DerivedViewAvailability::Ready,
                });
            }
            ActiveViewer::Fft if state.analysis.fft_state.has_data() => {
                state.bind_specialized_viewer_cache(viewer, analysis);
                self.transient_post.fft_loaded = Some(LoadedDerivedView {
                    analysis,
                    availability: DerivedViewAvailability::Ready,
                });
            }
            _ => {}
        }
    }

    pub(super) fn sync_transient_post_views(&mut self, state: &mut AppState) {
        let active_analysis = self.active_transient_analysis_key(state);
        self.cancel_stale_transient_task(DerivedViewKind::EyeDiagram, active_analysis);
        self.cancel_stale_transient_task(DerivedViewKind::Fft, active_analysis);

        if self
            .transient_post
            .eye_loaded
            .map(|loaded| Some(loaded.analysis) != active_analysis)
            .unwrap_or(false)
        {
            state
                .analysis
                .eye_diagram_state
                .load_data(EyeData::default());
            self.transient_post.eye_loaded = None;
        }

        if self
            .transient_post
            .fft_loaded
            .map(|loaded| Some(loaded.analysis) != active_analysis)
            .unwrap_or(false)
        {
            state.analysis.fft_state.clear();
            self.transient_post.fft_loaded = None;
        }

        self.poll_transient_task(state, DerivedViewKind::EyeDiagram, active_analysis);
        self.poll_transient_task(state, DerivedViewKind::Fft, active_analysis);
    }

    pub(super) fn invalidate_transient_post_views(&mut self, state: &mut AppState) {
        self.cancel_task_slot(DerivedViewKind::EyeDiagram);
        self.cancel_task_slot(DerivedViewKind::Fft);
        self.transient_post.eye_loaded = None;
        self.transient_post.fft_loaded = None;
        state.clear_transient_specialized_viewer_data();
    }

    pub(super) fn prime_transient_fft_source_selection(&mut self, state: &mut AppState) {
        let Some(source) = self.current_transient_waveform_source(state) else {
            return;
        };
        state
            .analysis
            .fft_state
            .set_selected_source(Some(source.source_name.clone()));
    }

    fn ensure_eye_diagram_data(&mut self, state: &mut AppState) -> DerivedViewerLoadState {
        // The reader's bit period is checked before anything else, including
        // whether an eye is already loaded: an eye folded at the old period
        // is not this eye at a different zoom, it is a different measurement.
        // Editing the rate has to rebuild, not be swallowed by the
        // already-have-data shortcut below.
        if let Some(analysis) = self.active_transient_analysis_key(state) {
            let requested = state.eye_timebase_for(analysis);
            if !eye_matches_timebase(state, requested) {
                self.cancel_task_slot(DerivedViewKind::EyeDiagram);
                self.transient_post.eye_loaded = None;
                state
                    .analysis
                    .eye_diagram_state
                    .load_data_with_timebase(EyeData::default(), None);
                state.clear_specialized_viewer_cache_authority(ActiveViewer::EyeDiagram);
            }
        }

        if state.analysis.eye_diagram_state.trace_count() > 0 {
            self.mark_transient_view_ready(state, ActiveViewer::EyeDiagram);
            return DerivedViewerLoadState::Ready;
        }

        let Some(active_analysis) = self.active_transient_analysis_key(state) else {
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

    fn ensure_fft_data(&mut self, state: &mut AppState) -> DerivedViewerLoadState {
        if state.analysis.fft_state.has_data() {
            self.mark_transient_view_ready(state, ActiveViewer::Fft);
            return DerivedViewerLoadState::Ready;
        }

        let Some(active_analysis) = self.active_transient_analysis_key(state) else {
            return DerivedViewerLoadState::Unavailable;
        };
        if let Some(loaded) = self.transient_post.fft_loaded
            && loaded.analysis == active_analysis
        {
            return match loaded.availability {
                DerivedViewAvailability::Ready => DerivedViewerLoadState::Ready,
                DerivedViewAvailability::Unavailable => DerivedViewerLoadState::Unavailable,
            };
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
        state
            .analysis
            .fft_state
            .set_selected_source(Some(source.source_name.clone()));
        self.spawn_derived_view_task(DerivedViewKind::Fft, source);
        DerivedViewerLoadState::Loading
    }

    fn poll_transient_task(
        &mut self,
        state: &mut AppState,
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
                *task_slot = None;
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
                // The provenance is recorded either way: when nothing could
                // be folded it is the only thing the sheet can tell the
                // reader about why.
                state.analysis.eye_diagram_state.load_data_with_timebase(
                    build.data.unwrap_or_default(),
                    Some(build.provenance),
                );
                if available {
                    state.bind_specialized_viewer_cache(ActiveViewer::EyeDiagram, message.analysis);
                }
                self.transient_post.eye_loaded = Some(LoadedDerivedView {
                    analysis: message.analysis,
                    availability: if available {
                        DerivedViewAvailability::Ready
                    } else {
                        DerivedViewAvailability::Unavailable
                    },
                });
            }
            (DerivedViewKind::Fft, DerivedViewResultPayload::Fft(result)) => {
                if let Some(prepared) = result {
                    state.analysis.fft_state.load_prepared_input(prepared);
                    state.bind_specialized_viewer_cache(ActiveViewer::Fft, message.analysis);
                    self.transient_post.fft_loaded = Some(LoadedDerivedView {
                        analysis: message.analysis,
                        availability: DerivedViewAvailability::Ready,
                    });
                } else {
                    state.analysis.fft_state.clear();
                    self.transient_post.fft_loaded = Some(LoadedDerivedView {
                        analysis: message.analysis,
                        availability: DerivedViewAvailability::Unavailable,
                    });
                }
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

        crate::workbench::spawn_or_inline(move || {
            let payload = match view {
                DerivedViewKind::EyeDiagram => {
                    if task_cancel_flag.load(Ordering::Relaxed) {
                        return;
                    }
                    DerivedViewResultPayload::Eye(build_eye_diagram_data(&task_source))
                }
                DerivedViewKind::Fft => {
                    let prepared = build_fft_prepared_input(&task_source, &task_cancel_flag);
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

    fn cancel_task_slot(&mut self, view: DerivedViewKind) {
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
        state: &AppState,
    ) -> Option<SpecializedViewerCacheProvenance> {
        let analysis = state.simulation.active_analysis()?;
        Self::analysis_supports_transient_derivation(analysis.analysis_type)
            .then(|| state.active_specialized_viewer_cache_provenance())
            .flatten()
    }

    fn current_transient_waveform_source(&self, state: &AppState) -> Option<DerivedWaveformSource> {
        let analysis = state.simulation.active_analysis()?;
        let analysis_key = self.active_transient_analysis_key(state)?;
        let preferred_source = state.analysis.fft_state.selected_source.as_deref();
        let (source_name, waveform) =
            Self::fft_source_waveform_from_state(&analysis.waveforms, preferred_source)?;

        Some(DerivedWaveformSource {
            analysis: analysis_key,
            source_name,
            time: Arc::clone(&waveform.x),
            values: Arc::clone(&waveform.y),
            fft_options: state
                .analysis
                .fft_state
                .input_options_for_waveform(&waveform.x),
            eye_timebase: state.eye_timebase_for(analysis_key),
        })
    }

    pub(crate) fn analysis_supports_transient_derivation(analysis_type: AnalysisType) -> bool {
        analysis_type.is_time_domain()
    }
}

/// Does the eye currently on screen answer the bit period being asked for?
///
/// The loaded data's own provenance is the source of truth rather than any
/// bookkeeping beside it, because it is what the sheet is quoting to the
/// reader. An eye with no recorded provenance makes no claim and is left
/// alone.
fn eye_matches_timebase(state: &AppState, requested: EyeTimebase) -> bool {
    let Some(provenance) = state.analysis.eye_diagram_state.timebase_provenance() else {
        return true;
    };
    match (provenance, requested) {
        (
            EyeTimebaseProvenance::Auto { .. } | EyeTimebaseProvenance::AutoRejected(_),
            EyeTimebase::Auto,
        ) => true,
        (
            EyeTimebaseProvenance::Explicit { unit_interval: had },
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
pub(crate) fn build_eye_from_waveform(
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
        EyeTimebase::Explicit { unit_interval } => (
            unit_interval,
            crossing_phase_at(time, values, unit_interval)
                .and_then(|fit| fold_anchor(fit.phase, unit_interval)),
            EyeTimebaseProvenance::Explicit { unit_interval },
        ),
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
    cancel_flag: &AtomicBool,
) -> Option<PreparedFftInput> {
    if cancel_flag.load(Ordering::Relaxed) {
        return None;
    }
    crate::analysis::fft::prepare_fft_input_with_options(
        &source.source_name,
        &source.time,
        &source.values,
        source.fft_options,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{AnalysisInstanceId, DatasetId};

    /// Trapezoidal 1010 clock at 1 ns, 20-80 % edge of 50 ps.
    fn clock(bits: usize, t_start: f64) -> (Vec<f64>, Vec<f64>) {
        const UI: f64 = 1e-9;
        const DT: f64 = 10e-12;
        const RAMP: f64 = 50e-12 / 0.6;

        let events: Vec<(f64, f64)> = (0..bits)
            .map(|n| {
                (
                    t_start + n as f64 * UI,
                    if n.is_multiple_of(2) { 1.0 } else { 0.0 },
                )
            })
            .collect();
        let t_end = t_start + (bits as f64 + 1.0) * UI;
        let count = (t_end / DT) as usize;
        let mut time = Vec::with_capacity(count);
        let mut signal = Vec::with_capacity(count);
        let mut cursor = 0usize;
        let mut held = 0.0;
        for index in 0..count {
            let t = index as f64 * DT;
            while cursor < events.len() && t >= events[cursor].0 + 0.5 * RAMP {
                held = events[cursor].1;
                cursor += 1;
            }
            let value = match events.get(cursor) {
                Some(&(edge, next)) if t > edge - 0.5 * RAMP => {
                    let alpha = ((t - (edge - 0.5 * RAMP)) / RAMP).clamp(0.0, 1.0);
                    held + alpha * (next - held)
                }
                _ => held,
            };
            time.push(t);
            signal.push(value);
        }
        (time, signal)
    }

    fn owner() -> SpecializedViewerCacheProvenance {
        SpecializedViewerCacheProvenance::for_prepared_analysis(
            DatasetId::new(),
            AnalysisInstanceId::new(),
        )
    }

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
        assert_eq!(
            stated_provenance,
            EyeTimebaseProvenance::Explicit {
                unit_interval: 2e-9
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
            .analysis
            .eye_diagram_state
            .load_data_with_timebase(data.expect("folds"), Some(provenance));

        assert!(eye_matches_timebase(&state, stated));
        assert!(!eye_matches_timebase(&state, EyeTimebase::Auto));
        assert!(!eye_matches_timebase(
            &state,
            EyeTimebase::Explicit {
                unit_interval: 2e-9
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
        assert!(state.ui.results.eye_timebase.is_empty());
    }
}
