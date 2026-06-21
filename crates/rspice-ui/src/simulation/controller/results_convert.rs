use super::*;
use std::collections::HashMap;
use std::sync::Arc;

impl SimulationController {
    pub(super) fn convert_to_analysis_result_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata_owned(sim_result, analysis_type, &label)
    }

    pub(super) fn convert_to_analysis_result_with_metadata_owned(
        &self,
        sim_result: crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;

        match sim_result {
            SimulationResult::DcOp(dc_result) => {
                let node_voltages = dc_result
                    .node_voltages
                    .into_iter()
                    .map(|(name, value)| OperatingPointValue {
                        name: format!("V({})", name),
                        value,
                        unit: "V".to_string(),
                    })
                    .collect();
                let branch_currents = dc_result
                    .branch_currents
                    .into_iter()
                    .map(|(name, value)| OperatingPointValue {
                        name: format!("I({})", name),
                        value,
                        unit: "A".to_string(),
                    })
                    .collect();

                let state_dc_op = DcOpResult {
                    node_voltages,
                    branch_currents,
                    power_dissipation: Vec::new(),
                };
                let mut result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_dc_op(state_dc_op);
                if let Some(report) = dc_result.device_report {
                    result = result.with_device_op(report);
                }
                result
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
                summary,
            } => {
                let mut result = AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_noise_waveforms_owned(
                        frequencies,
                        output_noise,
                        input_noise,
                        contributors,
                    ));
                if let Some(summary) = summary {
                    result = result.with_noise_summary(summary);
                }
                result
            }

            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_sorted_waveforms_with_shared_x_owned(
                    time,
                    waveforms,
                    |name, waveform| (name, waveform.y_values),
                ))
                .with_measurements(measurements),

            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_ac_waveforms_owned(frequencies, waveforms))
                .with_measurements(measurements),

            SimulationResult::DcSweep {
                sweep_values,
                waveforms,
                measurements,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(sweep_values, waveforms))
                .with_measurements(measurements),

            SimulationResult::PoleZero { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::Sensitivity { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::MonteCarlo { variables, .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.build_monte_carlo_waveforms_owned(variables))
            }

            SimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(sweep_values, waveforms)),

            SimulationResult::Corner {
                x_values,
                waveforms,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(x_values, waveforms)),

            SimulationResult::Reliability {
                years, waveforms, ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(years, waveforms)),

            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(iterations, waveforms)),

            SimulationResult::Soa {
                time, waveforms, ..
            } => AnalysisResult::new(1, analysis_type, label.to_string())
                .with_waveforms(self.build_waveforms_with_shared_x_owned(time, waveforms)),

            SimulationResult::MeasurementsOnly { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }
        }
    }

    fn build_waveforms_with_shared_x_owned(
        &self,
        x_values: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
    ) -> Vec<crate::state::WaveformData> {
        self.build_sorted_waveforms_with_shared_x_owned(x_values, waveforms, |name, waveform| {
            (name, waveform.y_values)
        })
    }

    fn build_noise_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: HashMap<String, Vec<f64>>,
    ) -> Vec<crate::state::WaveformData> {
        let shared_freqs = Arc::new(frequencies);
        let freq_len = shared_freqs.len();
        let mut results = Vec::new();

        if Self::samples_match_shared_axis(&output_noise, freq_len) {
            results.push(crate::state::WaveformData::new(
                "onoise".to_string(),
                Arc::clone(&shared_freqs),
                output_noise,
                Self::color_for_index(results.len()),
            ));
        }

        if let Some(inoise) = input_noise
            && Self::samples_match_shared_axis(&inoise, freq_len)
        {
            results.push(crate::state::WaveformData::new(
                "inoise".to_string(),
                Arc::clone(&shared_freqs),
                inoise,
                Self::color_for_index(results.len()),
            ));
        }

        let mut contributors: Vec<_> = contributors.into_iter().collect();
        contributors.sort_by(|a, b| a.0.cmp(&b.0));
        for (source, values) in contributors {
            if !Self::samples_match_shared_axis(&values, freq_len) {
                continue;
            }
            results.push(crate::state::WaveformData::new(
                format!("noise({})", source),
                Arc::clone(&shared_freqs),
                values,
                Self::color_for_index(results.len()),
            ));
        }

        results
    }

    fn build_monte_carlo_waveforms_owned(
        &self,
        variables: Vec<crate::simulation::results::MonteCarloVariableResult>,
    ) -> Vec<crate::state::WaveformData> {
        variables
            .into_iter()
            .filter_map(|var| {
                if var.histogram.is_empty() || var.bin_edges.len() < 2 {
                    return None;
                }
                let x: Vec<f64> = var
                    .bin_edges
                    .windows(2)
                    .map(|window| (window[0] + window[1]) * 0.5)
                    .collect();
                let y: Vec<f64> = var.histogram.iter().map(|count| *count as f64).collect();
                Some(crate::state::WaveformData::new(
                    format!("hist({})", var.name),
                    x,
                    y,
                    Self::color_for_index(0),
                ))
            })
            .collect()
    }

    fn build_ac_waveforms_owned(
        &self,
        frequencies: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
    ) -> Vec<crate::state::WaveformData> {
        let shared_freqs = Arc::new(frequencies);
        let freq_len = shared_freqs.len();
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            let real = waveform.y_values;
            if !Self::samples_match_shared_axis(&real, freq_len) {
                continue;
            }
            match waveform.y_imag {
                Some(imag) => {
                    if !Self::samples_match_shared_axis(&imag, freq_len) {
                        continue;
                    }
                    let magnitude_values: Vec<f64> = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| (r * r + i * i).sqrt())
                        .collect();
                    let phase = real
                        .iter()
                        .zip(imag.iter())
                        .map(|(r, i)| i.atan2(*r).to_degrees())
                        .collect::<Vec<_>>();

                    let magnitude = crate::state::WaveformData::new(
                        format!("|{}|", name),
                        Arc::clone(&shared_freqs),
                        magnitude_values,
                        Self::color_for_index(results.len()),
                    )
                    .with_complex_components(name.clone(), real, imag);
                    results.push(magnitude);

                    results.push(crate::state::WaveformData::new(
                        format!("phase({})", name),
                        Arc::clone(&shared_freqs),
                        phase,
                        Self::color_for_index(results.len()),
                    ));
                }
                None => {
                    results.push(crate::state::WaveformData::new(
                        format!("|{}|", name),
                        Arc::clone(&shared_freqs),
                        real,
                        Self::color_for_index(results.len()),
                    ));
                }
            }
        }

        results
    }

    fn build_sorted_waveforms_with_shared_x_owned<F>(
        &self,
        x_values: Vec<f64>,
        waveforms: HashMap<String, crate::simulation::WaveformData>,
        value_mapper: F,
    ) -> Vec<crate::state::WaveformData>
    where
        F: Fn(String, crate::simulation::WaveformData) -> (String, Vec<f64>),
    {
        let shared_x = Arc::new(x_values);
        let mut waveforms: Vec<_> = waveforms.into_iter().collect();
        waveforms.sort_by(|a, b| a.0.cmp(&b.0));

        let mut results = Vec::new();
        for (name, waveform) in waveforms {
            let (display_name, y_values) = value_mapper(name, waveform);
            if !Self::samples_match_shared_axis(&y_values, shared_x.len()) {
                continue;
            }
            results.push(crate::state::WaveformData::new(
                display_name,
                Arc::clone(&shared_x),
                y_values,
                Self::color_for_index(results.len()),
            ));
        }

        results
    }

    fn samples_match_shared_axis(samples: &[f64], axis_len: usize) -> bool {
        axis_len > 0 && samples.len() == axis_len
    }

    /// Get color for waveform trace by index
    pub(super) fn color_for_index(idx: usize) -> String {
        const COLORS: &[&str] = &[
            "#3B82F6", // Blue
            "#10B981", // Green
            "#F97316", // Orange
            "#8B5CF6", // Purple
            "#EC4899", // Pink
            "#EAB308", // Yellow
            "#14B8A6", // Teal
            "#EF4444", // Red
        ];
        COLORS[idx % COLORS.len()].to_string()
    }
}
