use super::*;
use crate::analysis::eye_diagram::data::{EyeData, EyeDataBuilder};
use crate::analysis::fft::{FftInputOptions, PreparedFftInput};
use crate::common::app::AppState;
use crate::state::AnalysisType;
use crate::viewers::ActiveViewer;
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
struct ActiveTransientAnalysisKey {
    run_id: u64,
    analysis_index: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct LoadedDerivedView {
    analysis: ActiveTransientAnalysisKey,
    availability: DerivedViewAvailability,
}

#[derive(Debug)]
struct PendingDerivedViewTask {
    analysis: ActiveTransientAnalysisKey,
    receiver: mpsc::Receiver<DerivedViewTaskResult>,
    cancel_flag: Arc<AtomicBool>,
}

#[derive(Debug, Clone)]
struct DerivedWaveformSource {
    analysis: ActiveTransientAnalysisKey,
    source_name: String,
    time: Arc<[f64]>,
    values: Arc<[f64]>,
    fft_options: FftInputOptions,
}

#[derive(Debug)]
enum DerivedViewResultPayload {
    Eye(Option<EyeData>),
    Fft(Option<PreparedFftInput>),
}

#[derive(Debug)]
struct DerivedViewTaskResult {
    analysis: ActiveTransientAnalysisKey,
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

    pub(crate) fn mark_transient_view_ready(&mut self, state: &AppState, viewer: ActiveViewer) {
        let Some(analysis) = self.active_transient_analysis_key(state) else {
            return;
        };

        match viewer {
            ActiveViewer::EyeDiagram if state.analysis.eye_diagram_state.trace_count() > 0 => {
                self.transient_post.eye_loaded = Some(LoadedDerivedView {
                    analysis,
                    availability: DerivedViewAvailability::Ready,
                });
            }
            ActiveViewer::Fft if state.analysis.fft_state.has_data() => {
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
        state
            .analysis
            .eye_diagram_state
            .load_data(EyeData::default());
        state.analysis.fft_state.clear();
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
        if state.analysis.eye_diagram_state.trace_count() > 0 {
            self.mark_transient_view_ready(state, ActiveViewer::EyeDiagram);
            return DerivedViewerLoadState::Ready;
        }

        let Some(active_analysis) = self.active_transient_analysis_key(state) else {
            return DerivedViewerLoadState::Unavailable;
        };
        if let Some(loaded) = self.transient_post.eye_loaded
            && loaded.analysis == active_analysis {
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
            && loaded.analysis == active_analysis {
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
        active_analysis: Option<ActiveTransientAnalysisKey>,
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
            (DerivedViewKind::EyeDiagram, DerivedViewResultPayload::Eye(result)) => {
                if let Some(eye_data) = result {
                    state.analysis.eye_diagram_state.load_data(eye_data);
                    self.transient_post.eye_loaded = Some(LoadedDerivedView {
                        analysis: message.analysis,
                        availability: DerivedViewAvailability::Ready,
                    });
                } else {
                    state
                        .analysis
                        .eye_diagram_state
                        .load_data(EyeData::default());
                    self.transient_post.eye_loaded = Some(LoadedDerivedView {
                        analysis: message.analysis,
                        availability: DerivedViewAvailability::Unavailable,
                    });
                }
            }
            (DerivedViewKind::Fft, DerivedViewResultPayload::Fft(result)) => {
                if let Some(prepared) = result {
                    state.analysis.fft_state.load_prepared_input(prepared);
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

        std::thread::spawn(move || {
            let payload = match view {
                DerivedViewKind::EyeDiagram => {
                    let eye_data = build_eye_diagram_data(&task_source, &task_cancel_flag);
                    DerivedViewResultPayload::Eye(eye_data)
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
        active_analysis: Option<ActiveTransientAnalysisKey>,
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
    ) -> Option<ActiveTransientAnalysisKey> {
        let run_idx = state.simulation.active_run_idx?;
        let analysis_idx = state.simulation.active_analysis_idx?;
        let run = state.simulation.runs.get(run_idx)?;
        let analysis = run.analyses.get(analysis_idx)?;
        Self::analysis_supports_transient_derivation(analysis.analysis_type).then_some(
            ActiveTransientAnalysisKey {
                run_id: run.id,
                analysis_index: analysis_idx,
            },
        )
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
        })
    }

    pub(crate) fn analysis_supports_transient_derivation(analysis_type: AnalysisType) -> bool {
        matches!(
            analysis_type,
            AnalysisType::Transient
                | AnalysisType::Envelope
                | AnalysisType::Soa
                | AnalysisType::Pss
        )
    }
}

fn build_eye_diagram_data(
    source: &DerivedWaveformSource,
    cancel_flag: &AtomicBool,
) -> Option<EyeData> {
    if cancel_flag.load(Ordering::Relaxed) {
        return None;
    }
    let bit_period = SimulationController::estimate_ui_period(&source.time, &source.values)?;
    if cancel_flag.load(Ordering::Relaxed) {
        return None;
    }

    let eye_data = EyeDataBuilder::new()
        .bit_period(bit_period)
        .ui_count(2)
        .skip_initial(2)
        .build(&source.time, &source.values);
    (eye_data.trace_count() > 0).then_some(eye_data)
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
