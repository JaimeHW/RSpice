use super::*;
use std::sync::Arc;

impl SimulationController {
    pub(super) fn waveforms_for_result(
        &self,
        sim_result: &crate::simulation::SimulationResult,
    ) -> Vec<crate::state::WaveformData> {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

        match sim_result {
            SimulationResult::Transient { time, waveforms } => self
                .build_sorted_waveforms_with_shared_x(time, waveforms, |name, waveform| {
                    (name, waveform.y_values.clone())
                }),

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => self.build_waveforms_with_shared_x(
                frequencies,
                waveforms
                    .iter()
                    .flat_map(|(name, waveform)| {
                        // Magnitude (linear) plus phase in degrees when the
                        // result is complex — the Bode viewer and the AC
                        // strip's right axis consume the phase trace.
                        let mut pair = vec![(format!("|{}|", name), waveform.magnitude())];
                        if let Some(phase) = waveform.phase_deg() {
                            pair.push((format!("phase({})", name), phase));
                        }
                        pair
                    })
                    .enumerate()
                    .map(|(idx, (name, values))| (idx, name, values)),
            ),

            SimulationResult::DcSweep {
                sweep_values,
                waveforms,
                ..
            } => self.build_waveforms_with_shared_x(
                sweep_values,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
            } => {
                let shared_freqs: Arc<[f64]> = Arc::from(frequencies.clone());
                let mut results = Vec::new();

                if !output_noise.is_empty() {
                    results.push(WaveformData::new(
                        "onoise".to_string(),
                        Arc::clone(&shared_freqs),
                        output_noise.clone(),
                        Self::color_for_index(0),
                    ));
                }

                if let Some(inoise) = input_noise
                    && !inoise.is_empty()
                {
                    results.push(WaveformData::new(
                        "inoise".to_string(),
                        Arc::clone(&shared_freqs),
                        inoise.clone(),
                        Self::color_for_index(1),
                    ));
                }

                for (idx, (source, values)) in contributors.iter().enumerate() {
                    results.push(WaveformData::new(
                        format!("noise({})", source),
                        Arc::clone(&shared_freqs),
                        values.clone(),
                        Self::color_for_index(idx + 2),
                    ));
                }

                results
            }

            SimulationResult::MonteCarlo { variables, .. } => variables
                .iter()
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
                    Some(WaveformData::new(
                        format!("hist({})", var.name),
                        x,
                        y,
                        Self::color_for_index(0),
                    ))
                })
                .collect(),

            SimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => self.build_waveforms_with_shared_x(
                sweep_values,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::Corner {
                x_values,
                waveforms,
                ..
            } => self.build_waveforms_with_shared_x(
                x_values,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::Reliability {
                years, waveforms, ..
            } => self.build_waveforms_with_shared_x(
                years,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => self.build_waveforms_with_shared_x(
                iterations,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::Soa {
                time, waveforms, ..
            } => self.build_waveforms_with_shared_x(
                time,
                waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, waveform))| (idx, name.clone(), waveform.y_values.clone())),
            ),

            SimulationResult::DcOp(_)
            | SimulationResult::PoleZero { .. }
            | SimulationResult::Sensitivity { .. }
            | SimulationResult::MeasurementsOnly { .. } => Vec::new(),
        }
    }

    pub(super) fn convert_to_analysis_result(
        &self,
        sim_result: &crate::simulation::SimulationResult,
        config: &AnalysisConfig,
    ) -> AnalysisResult {
        let analysis_type = self.config_to_analysis_type(config);
        let label = self.analysis_name(config).to_string();
        self.convert_to_analysis_result_with_metadata(sim_result, analysis_type, &label)
    }

    pub(super) fn convert_to_analysis_result_with_metadata(
        &self,
        sim_result: &crate::simulation::SimulationResult,
        analysis_type: AnalysisType,
        label: &str,
    ) -> AnalysisResult {
        use crate::simulation::SimulationResult;

        match sim_result {
            SimulationResult::DcOp(dc_result) => {
                // Convert engine DcOpResult to state DcOpResult
                let mut node_voltages = Vec::new();
                for (name, value) in &dc_result.node_voltages {
                    node_voltages.push(OperatingPointValue {
                        name: format!("V({})", name),
                        value: *value,
                        unit: "V".to_string(),
                    });
                }

                let mut branch_currents = Vec::new();
                for (name, value) in &dc_result.branch_currents {
                    branch_currents.push(OperatingPointValue {
                        name: format!("I({})", name),
                        value: *value,
                        unit: "A".to_string(),
                    });
                }

                let state_dc_op = DcOpResult {
                    node_voltages,
                    branch_currents,
                    power_dissipation: Vec::new(),
                };

                AnalysisResult::new(1, analysis_type, label.to_string()).with_dc_op(state_dc_op)
            }

            SimulationResult::Transient { .. }
            | SimulationResult::Ac { .. }
            | SimulationResult::DcSweep { .. }
            | SimulationResult::Noise { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.waveforms_for_result(sim_result))
            }

            SimulationResult::PoleZero { .. } => {
                // Pole-Zero results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::Sensitivity { .. } => {
                // Sensitivity results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::MonteCarlo { .. }
            | SimulationResult::Parametric { .. }
            | SimulationResult::Corner { .. }
            | SimulationResult::Reliability { .. }
            | SimulationResult::Optimization { .. }
            | SimulationResult::Soa { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
                    .with_waveforms(self.waveforms_for_result(sim_result))
            }

            SimulationResult::MeasurementsOnly { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }
        }
    }

    fn build_waveforms_with_shared_x<I>(
        &self,
        x_values: &[f64],
        traces: I,
    ) -> Vec<crate::state::WaveformData>
    where
        I: IntoIterator<Item = (usize, String, Vec<f64>)>,
    {
        let shared_x: Arc<[f64]> = Arc::from(x_values.to_vec());
        traces
            .into_iter()
            .map(|(idx, name, y_values)| {
                crate::state::WaveformData::new(
                    name,
                    Arc::clone(&shared_x),
                    y_values,
                    Self::color_for_index(idx),
                )
            })
            .collect()
    }

    fn build_sorted_waveforms_with_shared_x<F>(
        &self,
        x_values: &[f64],
        waveforms: &std::collections::HashMap<String, crate::simulation::WaveformData>,
        value_mapper: F,
    ) -> Vec<crate::state::WaveformData>
    where
        F: Fn(String, &crate::simulation::WaveformData) -> (String, Vec<f64>),
    {
        let mut names: Vec<_> = waveforms.keys().cloned().collect();
        names.sort();
        let shared_x: Arc<[f64]> = Arc::from(x_values.to_vec());

        names
            .into_iter()
            .enumerate()
            .filter_map(|(idx, name)| {
                waveforms.get(&name).map(|waveform| {
                    let (display_name, y_values) = value_mapper(name, waveform);
                    crate::state::WaveformData::new(
                        display_name,
                        Arc::clone(&shared_x),
                        y_values,
                        Self::color_for_index(idx),
                    )
                })
            })
            .collect()
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
