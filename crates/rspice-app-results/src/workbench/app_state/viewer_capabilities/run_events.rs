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
use crate::workbench::app_state::viewer_capabilities::AnalysisWorkspaceState;
use crate::workbench::documents::result_document::{
    AnalysisPresentationKey, ResultsParts, ResultsReads,
};
use rspice_results::result_presentation::SourceWaveformPresentationKey;

impl AnalysisWorkspaceState {
    /// What the transient viewers hold now.
    pub(crate) fn viewer_reading(&self) -> ViewerReading {
        ViewerReading {
            eye_has_traces: self.eye_diagram_state.trace_count() > 0,
            eye_timebase: self.eye_diagram_state.timebase_provenance().cloned(),
            fft_has_data: self.fft_state.has_data(),
            fft_failed: self.fft_state.last_error.is_some(),
            // The reader's choice, else the voltage plotted, else the signal
            // the last spectrum was of.
            fft_source: self
                .fft_state
                .source()
                .or(self.fft_state.selected_source.as_deref())
                .map(str::to_owned),
        }
    }

    /// How the FFT prepares the input it is given, as the reader
    /// set it.
    pub(crate) fn fft_input_options(&self) -> FftInputOptions {
        self.fft_state.input_options_for_waveform()
    }
}

impl ResultsReads<'_> {
    /// What the transient viewers hold now. The spectrum's source is the one
    /// the reader chose, else the first voltage plotted now.
    pub fn viewer_reading(self) -> ViewerReading {
        let mut reading = self.analysis.viewer_reading();
        if self.analysis.fft_state.source_choice.is_none()
            && let Some(plotted) = self.first_plotted_voltage()
        {
            reading.fft_source = Some(plotted);
        }
        reading
    }

    /// The first voltage the reader plots from the active analysis, in the
    /// order the run saved them: what the spectrum transforms when the reader
    /// chose no source.
    pub(crate) fn first_plotted_voltage(self) -> Option<String> {
        let run = self.model.simulation.active_run()?;
        let analysis = self.model.simulation.active_analysis()?;
        let key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        analysis
            .waveforms
            .iter()
            .find(|waveform| {
                is_voltage(&waveform.name)
                    && self.results.session.waveform_visibility(
                        &SourceWaveformPresentationKey::new(key, &waveform.name),
                        waveform.visible,
                    )
            })
            .map(|waveform| waveform.name.clone())
    }

    /// How the FFT prepares the input it is given, as the reader
    /// set it.
    pub fn fft_input_options(self) -> FftInputOptions {
        self.analysis.fft_input_options()
    }

    /// The bit period the reader chose for this result's eye.
    pub fn eye_timebase_for(self, owner: SpecializedViewerCacheProvenance) -> EyeTimebase {
        self.results.eye_timebase_for(owner)
    }
}

/// Whether a saved signal is a node voltage, `V(…)`.
fn is_voltage(name: &str) -> bool {
    let name = name.trim().trim_matches('|');
    name.len() > 3 && name[..2].eq_ignore_ascii_case("v(") && name.ends_with(')')
}

impl ViewerHost for ResultsParts<'_> {
    fn model(&self) -> &Model {
        self.model
    }

    fn viewer_reading(&self) -> ViewerReading {
        self.results_reads().viewer_reading()
    }

    fn fft_input_options(&self) -> FftInputOptions {
        self.results_reads().fft_input_options()
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
            RunEvent::FrequencyResponseReplaced { owner, bode } => {
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::BodePlot);
                self.analysis
                    .clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
                self.analysis.smith_chart_state.clear_traces();
                let has_bode = bode.response_count() > 0;
                if has_bode {
                    self.analysis.bode_plot_state.load_data(bode);
                } else {
                    self.analysis.bode_plot_state.load_data(BodeData::new());
                }
                if let Some(owner) = owner
                    && has_bode
                {
                    self.analysis
                        .bind_specialized_viewer_cache(ActiveViewer::BodePlot, owner);
                }
            }
            RunEvent::CompletionsCollected => self
                .analysis
                .synchronize_specialized_viewer_cache_authority(self.model),
            RunEvent::AnalysisLookedOver {
                analysis,
                index,
                verdict,
            } => self
                .results
                .adopt_looked_over(&self.model.simulation, analysis, index, verdict),
        }
    }
}
