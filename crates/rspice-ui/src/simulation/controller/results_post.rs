use super::*;

impl SimulationController {
    pub(super) fn populate_transient_post_views(
        &self,
        state: &mut AppState,
        time: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
    ) {
        let Some((_name, waveform)) = Self::primary_waveform(waveforms, time.len()) else {
            return;
        };

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

        if let Some((samples, sample_rate)) =
            Self::downsample_for_fft(time, &waveform.y_values, 4096)
        {
            let fft_data = crate::analysis::fft::FftData::from_time_domain(
                &format!("FFT({})", waveform.name),
                &samples,
                sample_rate,
                state.fft_state.window,
            );
            if !fft_data.is_empty() {
                state.fft_state.load_data(fft_data);
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

    fn primary_waveform<'a>(
        waveforms: &'a std::collections::HashMap<String, crate::simulation::WaveformData>,
        expected_len: usize,
    ) -> Option<(&'a str, &'a crate::simulation::WaveformData)> {
        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        for name in names {
            let Some(waveform) = waveforms.get(&name) else {
                continue;
            };
            if waveform.y_values.len() == expected_len {
                return Some((waveform.name.as_str(), waveform));
            }
        }
        None
    }

    fn estimate_ui_period(time: &[f64], signal: &[f64]) -> Option<f64> {
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

    fn downsample_for_fft(
        time: &[f64],
        signal: &[f64],
        max_points: usize,
    ) -> Option<(Vec<f64>, f64)> {
        let n = time.len().min(signal.len());
        if n < 16 || max_points < 16 {
            return None;
        }
        let step = (n / max_points).max(1);

        let mut values = Vec::with_capacity((n / step) + 1);
        let mut times = Vec::with_capacity((n / step) + 1);
        for idx in (0..n).step_by(step) {
            let t = time[idx];
            let y = signal[idx];
            if t.is_finite() && y.is_finite() {
                times.push(t);
                values.push(y);
            }
        }
        if values.len() < 16 {
            return None;
        }

        let duration = times[times.len() - 1] - times[0];
        if !duration.is_finite() || duration <= 0.0 {
            return None;
        }
        let sample_rate = (values.len().saturating_sub(1) as f64) / duration;
        if !sample_rate.is_finite() || sample_rate <= 0.0 {
            return None;
        }
        Some((values, sample_rate))
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

    pub(super) fn preferred_viewer_for_analysis(
        analysis_type: AnalysisType,
    ) -> crate::viewers::ActiveViewer {
        crate::common::analysis_navigation::preferred_viewer(analysis_type)
    }
}
