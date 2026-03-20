use super::*;

impl SimulationController {
    pub(super) fn populate_transient_post_views(
        &self,
        state: &mut AppState,
        time: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let preferred_fft_source = state.fft_state.selected_source.as_deref();
        let Some((waveform_key, waveform)) =
            Self::fft_source_waveform(waveforms, time.len(), preferred_fft_source)
        else {
            return;
        };

        state
            .fft_state
            .set_selected_source(Some(waveform_key.clone()));
        let input_options = state.fft_state.input_options_for_waveform(time);

        if let Some(bit_period) = Self::estimate_ui_period(time, &waveform.y_values) {
            let eye_data = crate::analysis::eye_diagram::data::EyeDataBuilder::new()
                .bit_period(bit_period)
                .ui_count(2)
                .skip_initial(2)
                .build(time, &waveform.y_values);
            if eye_data.trace_count() > 0 {
                state.eye_diagram_state.load_data(eye_data);
            }
        }

        if let Some(prepared) = crate::analysis::fft::prepare_fft_input_with_options(
            &waveform_key,
            time,
            &waveform.y_values,
            input_options,
        ) {
            state.fft_state.load_prepared_input(prepared);
        }
    }

    pub(super) fn populate_ac_post_views(
        &self,
        state: &mut AppState,
        frequencies: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let mut bode_data = crate::analysis::bode::BodeData::new();
        state.nyquist_state.clear();
        state.smith_chart_state.clear_traces();

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

            let response = crate::analysis::bode::data::FrequencyResponse::from_complex_arrays(
                &name,
                frequencies,
                &waveform.y_values,
                imag,
            );
            bode_data.add_response(response);

            let nyquist_curve = crate::analysis::nyquist::data::NyquistData::from_arrays(
                &name,
                frequencies,
                &waveform.y_values,
                imag,
            );
            if loaded_nyquist {
                state.nyquist_state.add_curve(nyquist_curve);
            } else {
                state.nyquist_state.load_data(nyquist_curve);
                loaded_nyquist = true;
            }

            if Self::is_sparameter_trace_name(&name) {
                state.smith_chart_state.load_sparam_data(
                    &name,
                    frequencies,
                    &waveform.y_values,
                    imag,
                );
            }
        }

        if bode_data.response_count() > 0 {
            bode_data.calculate_margins();
            state.bode_plot_state.load_data(bode_data);
        } else {
            state
                .bode_plot_state
                .load_data(crate::analysis::bode::BodeData::new());
        }
    }

    fn fft_source_waveform<'a>(
        waveforms: &'a std::collections::HashMap<String, crate::simulation::WaveformData>,
        expected_len: usize,
        preferred_name: Option<&str>,
    ) -> Option<(String, &'a crate::simulation::WaveformData)> {
        if let Some(name) = preferred_name {
            if let Some(wf) = waveforms
                .get(name)
                .filter(|wf| wf.y_values.len() == expected_len)
            {
                return Some((name.to_string(), wf));
            }

            let mut sorted_names: Vec<_> = waveforms.keys().cloned().collect();
            sorted_names.sort();
            for key in sorted_names {
                let Some(wf) = waveforms.get(&key) else {
                    continue;
                };
                if wf.y_values.len() == expected_len && wf.name == name {
                    return Some((key, wf));
                }
            }

            if let Some((key, wf)) =
                Self::match_preferred_fft_source_normalized(waveforms, expected_len, name)
            {
                return Some((key, wf));
            }
        }

        // Keep fallback deterministic and predictable so the same waveform is
        // analyzed run-to-run when no explicit source is available.
        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        names.into_iter().find_map(|name| {
            waveforms
                .get(&name)
                .filter(|wf| wf.y_values.len() == expected_len)
                .map(|wf| (name, wf))
        })
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

            if let Some(waveform) = candidates
                .iter()
                .copied()
                .find(|wf| Self::parse_fft_source_name(&wf.name).core == Self::parse_fft_source_name(name).core)
            {
                return Some((waveform.name.clone(), waveform));
            }
        }

        candidates
            .into_iter()
            .next()
            .map(|waveform| (waveform.name.clone(), waveform))
    }

    fn match_preferred_fft_source_normalized<'a>(
        waveforms: &'a std::collections::HashMap<String, crate::simulation::WaveformData>,
        expected_len: usize,
        preferred_name: &str,
    ) -> Option<(String, &'a crate::simulation::WaveformData)> {
        let preferred = Self::parse_fft_source_name(preferred_name);
        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();

        let mut best: Option<(i32, String, &'a crate::simulation::WaveformData)> = None;
        for name in names {
            let Some(wf) = waveforms.get(&name) else {
                continue;
            };
            if wf.y_values.len() != expected_len {
                continue;
            }

            let key_name = Self::parse_fft_source_name(&name);
            let wf_name = Self::parse_fft_source_name(&wf.name);
            if preferred.core != key_name.core && preferred.core != wf_name.core {
                continue;
            }

            let candidate_kind = match wf_name.kind {
                FftSourceKind::Other => key_name.kind,
                kind => kind,
            };
            let rank = Self::fft_source_kind_rank(preferred.kind, candidate_kind);
            match best {
                Some((best_rank, _, _)) if rank >= best_rank => {}
                _ => {
                    best = Some((rank, name, wf));
                }
            }
        }

        best.map(|(_, key, wf)| (key, wf))
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
        let edges =
            crate::analysis::eye_diagram::data::find_edges(&time[..n], &signal[..n], threshold);
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

    fn is_sparameter_trace_name(name: &str) -> bool {
        let normalized = name.trim_matches('|').to_ascii_uppercase();
        if !normalized.starts_with('S') {
            return false;
        }
        normalized[1..]
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .count()
            >= 2
    }

    #[cfg(test)]
    pub(super) fn preferred_viewer_for_analysis(
        analysis_type: AnalysisType,
    ) -> crate::viewers::ActiveViewer {
        crate::common::analysis_navigation::preferred_viewer(analysis_type)
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
