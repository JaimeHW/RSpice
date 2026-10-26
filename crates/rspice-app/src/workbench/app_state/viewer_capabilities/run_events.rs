//! The Results workspace's side of what the run tier reads and reports.
//!
//! The run tier describes what happened to a result; this is the one place
//! that turns each report into viewer data and cache ownership. Every event
//! is applied before `apply_run_event` returns. What it reads of the viewers
//! is answered here too, from the same parts.

#[cfg(test)]
mod tests;

use rspice_results_ui::bode::BodeData;
use rspice_results_ui::eye_diagram::{EyeData, EyeTimebase};
use rspice_results_ui::fft::FftInputOptions;

use crate::simulation::{ViewerHost, ViewerReading};
use crate::state::{ActiveViewer, RunEvent, RunEventSink, SpecializedViewerCacheProvenance};
use crate::workbench::app_state::Model;
use crate::workbench::documents::result_document::{ResultsParts, ResultsReads};

impl ResultsReads<'_> {
    /// What the transient viewers hold now.
    pub(crate) fn viewer_reading(self) -> ViewerReading {
        let analysis = self.analysis;
        ViewerReading {
            eye_has_traces: analysis.eye_diagram_state.trace_count() > 0,
            eye_timebase: analysis.eye_diagram_state.timebase_provenance().cloned(),
            fft_has_data: analysis.fft_state.has_data(),
            fft_failed: analysis.fft_state.last_error.is_some(),
            fft_source: analysis.fft_state.selected_source.clone(),
        }
    }

    /// How the FFT prepares the input it is given over `time`, as the reader
    /// set it.
    pub(crate) fn fft_input_options(self, time: &[f64]) -> FftInputOptions {
        self.analysis.fft_state.input_options_for_waveform(time)
    }

    /// The bit period the reader chose for this result's eye.
    pub(crate) fn eye_timebase_for(self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results.eye_timebase_for(owner)
    }
}

impl ViewerHost for ResultsParts<'_> {
    fn model(&self) -> &Model {
        self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        self.results_reads().viewer_reading()
    }

    fn fft_input_options(&self, time: &[f64]) -> FftInputOptions {
        self.results_reads().fft_input_options(time)
    }

    fn eye_timebase_for(&self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results_reads().eye_timebase_for(owner)
    }
}

impl RunEventSink for ResultsParts<'_> {
    fn apply_run_event(&mut self, event: RunEvent) {
        match event {
            RunEvent::TransientResultReplaced => {
                self.analysis.clear_transient_specialized_viewer_data()
            }
            RunEvent::FftSourceChosen { source } => {
                self.analysis.fft_state.set_selected_source(Some(source));
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
                self.analysis
                    .eye_diagram_state
                    .load_data_with_timebase(eye.unwrap_or_default(), Some(timebase));
                if folded && let Some(owner) = owner {
                    self.analysis
                        .bind_specialized_viewer_cache(ActiveViewer::EyeDiagram, owner);
                }
            }
            RunEvent::EyeTimebaseChanged => {
                self.analysis
                    .eye_diagram_state
                    .load_data_with_timebase(EyeData::default(), None);
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::EyeDiagram);
            }
            RunEvent::EyeOutdated => {
                self.analysis
                    .eye_diagram_state
                    .load_data(EyeData::default());
            }
            RunEvent::FftInputPrepared { owner, input } => match input {
                Ok(prepared) => {
                    if self
                        .analysis
                        .fft_state
                        .load_prepared_input(prepared)
                        .is_ok()
                    {
                        if let Some(owner) = owner {
                            self.analysis
                                .bind_specialized_viewer_cache(ActiveViewer::Fft, owner);
                        }
                    } else {
                        self.analysis
                            .clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
                    }
                }
                Err(error) => {
                    self.analysis.fft_state.record_input_error(error);
                    self.analysis
                        .clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
                }
            },
            RunEvent::FftWorkerLost => {
                self.analysis.fft_state.record_worker_disconnect();
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
            }
            RunEvent::FftOutdated => self.analysis.fft_state.clear(),
            RunEvent::FftEmptied => {
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::Fft);
            }
            RunEvent::ViewerCacheConfirmed { viewer, owner } => {
                self.analysis.bind_specialized_viewer_cache(viewer, owner);
            }
            RunEvent::FrequencyResponseReplaced {
                owner,
                bode,
                loop_gain,
            } => {
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::BodePlot);
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::Nyquist);
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
                self.analysis.nyquist_state.clear();
                self.analysis.smith_chart_state.clear_traces();
                if let Some(locus) = loop_gain {
                    self.analysis.nyquist_state.load_data(locus);
                }
                let loaded_nyquist = !self.analysis.nyquist_state.is_empty();
                let has_bode = bode.response_count() > 0;
                if has_bode {
                    self.analysis.bode_plot_state.load_data(bode);
                } else {
                    self.analysis.bode_plot_state.load_data(BodeData::new());
                }
                if let Some(owner) = owner {
                    if has_bode {
                        self.analysis
                            .bind_specialized_viewer_cache(ActiveViewer::BodePlot, owner);
                    }
                    if loaded_nyquist {
                        self.analysis
                            .bind_specialized_viewer_cache(ActiveViewer::Nyquist, owner);
                    }
                }
            }
            RunEvent::CompletionsCollected => self
                .analysis
                .synchronize_specialized_viewer_cache_authority(self.model),
        }
    }
}
