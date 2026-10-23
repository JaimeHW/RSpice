//! The Results workspace's side of what the run tier reports.
//!
//! The run tier describes what happened to a result; this is the one place
//! that turns each report into viewer data and cache ownership. Every event
//! is applied before `apply_run_event` returns.

#[cfg(test)]
mod tests;

use rspice_results_ui::bode::BodeData;
use rspice_results_ui::eye_diagram::EyeData;

use crate::state::{ActiveViewer, RunEvent, RunEventSink};
use crate::workbench::app_state::AppState;

impl RunEventSink for AppState {
    fn apply_run_event(&mut self, event: RunEvent) {
        match event {
            RunEvent::TransientResultReplaced => self.clear_transient_specialized_viewer_data(),
            RunEvent::FftSourceChosen { source } => {
                self.results
                    .analysis
                    .fft_state
                    .set_selected_source(Some(source));
            }
            RunEvent::EyeFolded {
                owner,
                eye,
                timebase,
            } => {
                let folded = eye.is_some();
                // The provenance is recorded either way: when nothing could be
                // folded it is the only thing the sheet can tell the reader
                // about why.
                self.results
                    .analysis
                    .eye_diagram_state
                    .load_data_with_timebase(eye.unwrap_or_default(), Some(timebase));
                if folded && let Some(owner) = owner {
                    self.bind_specialized_viewer_cache(ActiveViewer::EyeDiagram, owner);
                }
            }
            RunEvent::EyeTimebaseChanged => {
                self.results
                    .analysis
                    .eye_diagram_state
                    .load_data_with_timebase(EyeData::default(), None);
                self.clear_specialized_viewer_cache_authority(ActiveViewer::EyeDiagram);
            }
            RunEvent::EyeOutdated => {
                self.results
                    .analysis
                    .eye_diagram_state
                    .load_data(EyeData::default());
            }
            RunEvent::FftInputPrepared { owner, input } => match input {
                Ok(prepared) => {
                    if self
                        .results
                        .analysis
                        .fft_state
                        .load_prepared_input(prepared)
                        .is_ok()
                    {
                        if let Some(owner) = owner {
                            self.bind_specialized_viewer_cache(ActiveViewer::Fft, owner);
                        }
                    } else {
                        self.clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
                    }
                }
                Err(error) => {
                    self.results.analysis.fft_state.record_input_error(error);
                    self.clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
                }
            },
            RunEvent::FftWorkerLost => {
                self.results.analysis.fft_state.record_worker_disconnect();
                self.clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
            }
            RunEvent::FftOutdated => self.results.analysis.fft_state.clear(),
            RunEvent::FftEmptied => {
                self.clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
            }
            RunEvent::ViewerCacheConfirmed { viewer, owner } => {
                self.bind_specialized_viewer_cache(viewer, owner);
            }
            RunEvent::FrequencyResponseReplaced {
                owner,
                bode,
                loop_gain,
            } => {
                self.clear_specialized_viewer_cache_authority(ActiveViewer::BodePlot);
                self.clear_specialized_viewer_cache_authority(ActiveViewer::Nyquist);
                self.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
                self.results.analysis.nyquist_state.clear();
                self.results.analysis.smith_chart_state.clear_traces();
                if let Some(locus) = loop_gain {
                    self.results.analysis.nyquist_state.load_data(locus);
                }
                let loaded_nyquist = !self.results.analysis.nyquist_state.is_empty();
                let has_bode = bode.response_count() > 0;
                if has_bode {
                    self.results.analysis.bode_plot_state.load_data(bode);
                } else {
                    self.results
                        .analysis
                        .bode_plot_state
                        .load_data(BodeData::new());
                }
                if let Some(owner) = owner {
                    if has_bode {
                        self.bind_specialized_viewer_cache(ActiveViewer::BodePlot, owner);
                    }
                    if loaded_nyquist {
                        self.bind_specialized_viewer_cache(ActiveViewer::Nyquist, owner);
                    }
                }
            }
            RunEvent::CompletionsCollected => self.synchronize_specialized_viewer_cache_authority(),
        }
    }
}
