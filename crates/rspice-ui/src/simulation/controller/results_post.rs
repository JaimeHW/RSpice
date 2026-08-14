//! Populating the post-run viewers.
//!
//! After a run completes, seeds each analysis viewer with the data it needs
//! — transient traces, AC magnitude and phase, and the specialized viewers
//! behind the analysis workspace.

use super::*;

impl SimulationController {
    pub(super) fn in_flight_specialized_viewer_provenance(
        &self,
        state: &AppState,
    ) -> Option<SpecializedViewerCacheProvenance> {
        let run_sequence = self.current_run_id?;
        let run = state.simulation.run_by_sequence(run_sequence)?;
        let source_instance_id = self.current_provenance.as_ref()?.source_instance_id();
        Some(SpecializedViewerCacheProvenance::for_prepared_analysis(
            run.dataset_id,
            source_instance_id,
        ))
    }

    pub(super) fn populate_transient_post_views(
        &self,
        state: &mut AppState,
        analysis: &crate::state::AnalysisResult,
    ) {
        if !analysis.success || analysis.analysis_type != crate::state::AnalysisType::Transient {
            state.clear_transient_specialized_viewer_data();
            return;
        }

        let preferred_fft_source = state.analysis.fft_state.selected_source.as_deref();
        let Some((waveform_key, waveform)) =
            Self::fft_source_waveform_from_state(&analysis.waveforms, preferred_fft_source)
        else {
            // A specialized viewer must never outlive the retained source it
            // claims to represent.  In particular, an analysis with no saved
            // transient outputs cannot keep an FFT derived from the engine's
            // discarded working set.
            state.clear_transient_specialized_viewer_data();
            return;
        };
        let sample_count = waveform.x.len().min(waveform.y.len());
        let time = &waveform.x[..sample_count];
        let values = &waveform.y[..sample_count];

        state
            .analysis
            .fft_state
            .set_selected_source(Some(waveform_key.clone()));
        let input_options = state.analysis.fft_state.input_options_for_waveform(time);

        let provenance = self.in_flight_specialized_viewer_provenance(state);
        if let Some(bit_period) = Self::estimate_ui_period(time, values) {
            let eye_data = crate::analysis::eye_diagram::EyeDataBuilder::new()
                .bit_period(bit_period)
                .ui_count(2)
                .skip_initial(2)
                .build(time, values);
            if eye_data.trace_count() > 0 {
                state.analysis.eye_diagram_state.load_data(eye_data);
                if let Some(provenance) = provenance {
                    state.bind_specialized_viewer_cache(ActiveViewer::EyeDiagram, provenance);
                }
            }
        }

        if let Some(prepared) = crate::analysis::fft::prepare_fft_input_with_options(
            &waveform_key,
            time,
            values,
            input_options,
        ) {
            state.analysis.fft_state.load_prepared_input(prepared);
            if let Some(provenance) = provenance {
                state.bind_specialized_viewer_cache(ActiveViewer::Fft, provenance);
            }
        }
    }

    pub(super) fn populate_ac_post_views(
        &self,
        state: &mut AppState,
        frequencies: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let mut bode_data = crate::analysis::bode::BodeData::new();
        state.clear_specialized_viewer_cache_authority(ActiveViewer::BodePlot);
        state.clear_specialized_viewer_cache_authority(ActiveViewer::Nyquist);
        state.clear_specialized_viewer_cache_authority(ActiveViewer::SmithChart);
        state.analysis.nyquist_state.clear();
        state.analysis.smith_chart_state.clear_traces();

        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        let mut loaded_nyquist = false;
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            let Some(imag) = waveform.y_imag.as_ref() else {
                continue;
            };
            if waveform.y_values.len() != frequencies.len() || imag.len() != frequencies.len() {
                continue;
            }

            bode_data.add_response();

            let nyquist_curve = crate::analysis::nyquist::data::NyquistData::from_arrays(
                &name,
                frequencies,
                &waveform.y_values,
                imag,
            );
            if loaded_nyquist {
                state.analysis.nyquist_state.add_curve(nyquist_curve);
            } else {
                state.analysis.nyquist_state.load_data(nyquist_curve);
                loaded_nyquist = true;
            }
        }

        let has_bode = bode_data.response_count() > 0;
        if has_bode {
            state.analysis.bode_plot_state.load_data(bode_data);
        } else {
            state
                .analysis
                .bode_plot_state
                .load_data(crate::analysis::bode::BodeData::new());
        }

        if let Some(provenance) = self.in_flight_specialized_viewer_provenance(state) {
            if has_bode {
                state.bind_specialized_viewer_cache(ActiveViewer::BodePlot, provenance);
            }
            if loaded_nyquist {
                state.bind_specialized_viewer_cache(ActiveViewer::Nyquist, provenance);
            }
        }
    }

    pub(super) fn fft_source_waveform_from_state<'a>(
        waveforms: &'a [crate::state::WaveformData],
        preferred_name: Option<&str>,
    ) -> Option<(String, &'a crate::state::WaveformData)> {
        let mut candidates: Vec<&crate::state::WaveformData> = waveforms
            .iter()
            .filter(|wf| {
                let len = wf.x.len().min(wf.y.len());
                len >= crate::analysis::fft::MIN_FFT_SAMPLES
            })
            .collect();
        candidates.sort_by(|a, b| a.name.cmp(&b.name));

        if let Some(name) = preferred_name {
            if let Some(waveform) = candidates
                .iter()
                .copied()
                .find(|wf| wf.name == name || wf.name.eq_ignore_ascii_case(name))
            {
                return Some((waveform.name.clone(), waveform));
            }

            let preferred = Self::parse_fft_source_name(name);
            if let Some(waveform) = candidates
                .iter()
                .copied()
                .filter(|waveform| {
                    Self::parse_fft_source_name(&waveform.name).core == preferred.core
                })
                .min_by_key(|waveform| {
                    Self::fft_source_kind_rank(
                        preferred.kind,
                        Self::parse_fft_source_name(&waveform.name).kind,
                    )
                })
            {
                return Some((waveform.name.clone(), waveform));
            }
        }

        candidates
            .into_iter()
            .next()
            .map(|waveform| (waveform.name.clone(), waveform))
    }

    fn parse_fft_source_name(name: &str) -> ParsedFftSourceName {
        let trimmed = name.trim().trim_matches('|');
        if let Some(core) = trimmed.strip_prefix("V(").and_then(|s| s.strip_suffix(')')) {
            return ParsedFftSourceName {
                core: core.trim().to_ascii_lowercase(),
                kind: FftSourceKind::Voltage,
            };
        }
        if let Some(core) = trimmed.strip_prefix("I(").and_then(|s| s.strip_suffix(')')) {
            return ParsedFftSourceName {
                core: core.trim().to_ascii_lowercase(),
                kind: FftSourceKind::Current,
            };
        }
        ParsedFftSourceName {
            core: trimmed.trim().to_ascii_lowercase(),
            kind: FftSourceKind::Other,
        }
    }

    fn fft_source_kind_rank(preferred: FftSourceKind, candidate: FftSourceKind) -> i32 {
        match preferred {
            FftSourceKind::Voltage => {
                if candidate == FftSourceKind::Voltage {
                    0
                } else {
                    1
                }
            }
            FftSourceKind::Current => {
                if candidate == FftSourceKind::Current {
                    0
                } else {
                    1
                }
            }
            FftSourceKind::Other => match candidate {
                // For ambiguous untyped labels (e.g. "out"), prefer voltage
                // traces over current traces.
                FftSourceKind::Voltage => 0,
                FftSourceKind::Other => 1,
                FftSourceKind::Current => 2,
            },
        }
    }

    pub(super) fn estimate_ui_period(time: &[f64], signal: &[f64]) -> Option<f64> {
        let n = time.len().min(signal.len());
        if n < 8 {
            return None;
        }

        let mut v_min = f64::INFINITY;
        let mut v_max = f64::NEG_INFINITY;
        for &v in signal.iter().take(n) {
            if v.is_finite() {
                v_min = v_min.min(v);
                v_max = v_max.max(v);
            }
        }
        if !v_min.is_finite() || !v_max.is_finite() || (v_max - v_min) <= 0.0 {
            return None;
        }

        let threshold = (v_min + v_max) * 0.5;
        let edges = crate::analysis::eye_diagram::find_edges(&time[..n], &signal[..n], threshold);
        if edges.len() < 3 {
            return None;
        }

        let mut rising_times: Vec<f64> = edges
            .iter()
            .filter(|edge| edge.rising)
            .filter(|edge| edge.time.is_finite())
            .map(|edge| edge.time)
            .collect();
        rising_times.sort_by(|a, b| a.total_cmp(b));

        let edge_times: Vec<f64> = if rising_times.len() >= 3 {
            rising_times
        } else {
            let mut all: Vec<f64> = edges
                .iter()
                .map(|edge| edge.time)
                .filter(|time| time.is_finite())
                .collect();
            all.sort_by(|a, b| a.total_cmp(b));
            all
        };
        if edge_times.len() < 3 {
            return None;
        }

        let mut intervals = Vec::with_capacity(edge_times.len().saturating_sub(1));
        for pair in edge_times.windows(2) {
            let dt = pair[1] - pair[0];
            if dt.is_finite() && dt > 0.0 {
                intervals.push(dt);
            }
        }
        if intervals.is_empty() {
            return None;
        }
        intervals.sort_by(|a, b| a.total_cmp(b));
        let median = intervals[intervals.len() / 2];
        (median.is_finite() && median > 0.0).then_some(median)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FftSourceKind {
    Voltage,
    Current,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ParsedFftSourceName {
    core: String,
    kind: FftSourceKind,
}
