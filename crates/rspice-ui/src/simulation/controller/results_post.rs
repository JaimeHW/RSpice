//! Populating the post-run viewers.
//!
//! After a run completes, seeds each analysis viewer with the data it needs
//! — transient traces, AC magnitude and phase, and the specialized viewers
//! behind the analysis workspace.

use super::*;

use crate::simulation::results::STB_NYQUIST_CONTOUR_WAVEFORM;

/// What the locus is, named for the reader rather than for the transport.
const LOOP_GAIN_LOCUS_LABEL: &str = "L(jω)";

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
        // Same builder as the lazy path in `transient_post`, at the same
        // reader-chosen bit period. Seeding the viewer eagerly must not mean
        // seeding it from a second, differently-behaved estimator.
        let timebase = provenance
            .map(|owner| state.eye_timebase_for(owner))
            .unwrap_or_default();
        let (eye_data, eye_provenance) = build_eye_from_waveform(time, values, timebase);
        let folded = eye_data.is_some();
        state
            .analysis
            .eye_diagram_state
            .load_data_with_timebase(eye_data.unwrap_or_default(), Some(eye_provenance));
        if folded && let Some(provenance) = provenance {
            state.bind_specialized_viewer_cache(ActiveViewer::EyeDiagram, provenance);
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

    /// The waveform a stability run retains its loop-gain contour under, when
    /// the analysis that just finished is one.
    ///
    /// A stability result reaches this seam spelled as an AC result — that is
    /// how its swept response is retained — so the analysis kind has to come
    /// from the prepared task, resolved exactly the way the retention site
    /// resolves it. Anything else has no loop gain to hand the sheet.
    fn loop_gain_contour_name(&self) -> Option<&'static str> {
        let analysis_type = self
            .current_spec
            .as_ref()
            .map(|spec| self.spec_to_analysis_type(spec))
            .or_else(|| {
                self.current_config
                    .as_ref()
                    .map(|config| self.config_to_analysis_type(config))
            })?;
        (analysis_type == AnalysisType::Stb).then_some(STB_NYQUIST_CONTOUR_WAVEFORM)
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
        // The Nyquist sheet is a loop-gain instrument: its encirclements and
        // margins are the stability criterion applied to a return ratio. Only
        // a stability run measures one, and it retains exactly one contour for
        // it, so that contour is the only thing the sheet is given. Every
        // other complex response here — an AC node response, a harmonic
        // balance spectrum — is a perfectly good curve that is not a loop
        // gain, and drawing it there would attach a stability verdict to a
        // quantity that has none.
        let loop_gain = self
            .loop_gain_contour_name()
            .filter(|name| waveforms.contains_key(*name));
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

            if loop_gain == Some(name.as_str()) {
                state.analysis.nyquist_state.load_data(
                    crate::analysis::nyquist::NyquistData::from_arrays(
                        LOOP_GAIN_LOCUS_LABEL,
                        frequencies,
                        &waveform.y_values,
                        imag,
                    ),
                );
            }
        }
        let loaded_nyquist = !state.analysis.nyquist_state.is_empty();

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

#[cfg(test)]
mod contour_name_tests {
    /// The contour's name is a contract between the run and the sheet, and it
    /// was two independent string literals in two modules with nothing that
    /// compiled or tested the pair together. Renaming the writer's spelling
    /// leaves the reader looking for a waveform that no longer exists, and
    /// the Nyquist sheet quietly shows nothing at all.
    #[test]
    fn the_stability_contour_has_exactly_one_spelling_in_the_crate() {
        let writer =
            crate::source_guard::without_test_items(include_str!("../runner/spec/frequency.rs"));
        assert!(
            !writer.contains("\"Nyquist L(jw)\""),
            "the stability run writes the contour name as its own literal instead of the \
             constant the reader resolves it through"
        );
        let reader = crate::source_guard::without_test_items(include_str!("results_post.rs"));
        assert!(
            !reader.contains("\"Nyquist L(jw)\""),
            "the post-run population resolves the contour name from its own literal"
        );
    }
}

#[cfg(test)]
mod nyquist_scope_tests {
    use super::*;
    use crate::simulation::WaveformData;
    use std::collections::HashMap;

    fn stb_spec() -> AnalysisSpec {
        AnalysisSpec::Stb {
            probe_node: "VLOOP1".to_owned(),
            start_freq: 1.0,
            stop_freq: 1.0e6,
            sweep: Default::default(),
            points_per_decade: 10,
            compute_nyquist: true,
        }
    }

    /// A stability result and an AC result arrive at this seam spelled the
    /// same way. The frequencies and both responses are identical here, so
    /// only the analysis kind can separate them.
    fn frequency_response() -> (Vec<f64>, HashMap<String, WaveformData>) {
        let frequencies = vec![1.0, 10.0, 100.0];
        let mut waveforms = HashMap::new();
        waveforms.insert(
            STB_NYQUIST_CONTOUR_WAVEFORM.to_owned(),
            WaveformData::new_complex(
                STB_NYQUIST_CONTOUR_WAVEFORM,
                frequencies.clone(),
                vec![2.0, 0.5, -0.1],
                vec![-0.3, -0.8, -0.2],
            ),
        );
        waveforms.insert(
            "V(out)".to_owned(),
            WaveformData::new_complex(
                "V(out)",
                frequencies.clone(),
                vec![1.0, 0.7, 0.2],
                vec![0.0, -0.4, -0.6],
            ),
        );
        (frequencies, waveforms)
    }

    #[test]
    fn a_stability_run_loads_its_loop_gain_contour_and_nothing_else() {
        let mut controller = SimulationController::new();
        controller.current_spec = Some(stb_spec());
        let mut state = AppState::default();
        let (frequencies, waveforms) = frequency_response();

        controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

        let curve = state
            .analysis
            .nyquist_state
            .curve()
            .expect("the stability run's contour reaches the sheet");
        assert_eq!(curve.name, LOOP_GAIN_LOCUS_LABEL);
        assert_eq!(curve.len(), frequencies.len());
        assert_eq!(curve.points[0].real, 2.0);
    }

    /// An AC node response is not a loop gain, so it must not reach a sheet
    /// that puts a stability criterion on what it draws.
    #[test]
    fn an_ac_run_loads_no_loop_gain_locus() {
        let mut controller = SimulationController::new();
        controller.current_spec = Some(AnalysisSpec::Ac {
            start_freq: 1.0,
            stop_freq: 100.0,
            points_per_unit: 10,
            sweep: Default::default(),
        });
        let mut state = AppState::default();
        let (frequencies, waveforms) = frequency_response();

        controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

        assert!(state.analysis.nyquist_state.is_empty());
        assert!(state.analysis.cache_authority.nyquist.is_none());
        // The Bode sheet still receives every complex response.
        assert!(!state.analysis.bode_plot_state.is_empty());
    }

    /// And an analysis whose kind cannot be resolved gets no locus either:
    /// the sheet is opened by evidence, never by the shape of the data.
    #[test]
    fn an_unattributed_result_loads_no_loop_gain_locus() {
        let controller = SimulationController::new();
        let mut state = AppState::default();
        let (frequencies, waveforms) = frequency_response();

        controller.populate_ac_post_views(&mut state, &frequencies, &waveforms);

        assert!(state.analysis.nyquist_state.is_empty());
    }
}
