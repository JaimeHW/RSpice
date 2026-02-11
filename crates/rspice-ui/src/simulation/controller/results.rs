use super::*;

impl SimulationController {
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
        use crate::state::WaveformData;

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

            SimulationResult::Transient { time, waveforms } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            time.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                // For AC analysis, store magnitude (not raw complex values)
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            format!("|{}|", name),
                            frequencies.clone(),
                            wf.magnitude(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::DcSweep {
                sweep_var: _,
                sweep_values,
                waveforms,
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            sweep_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                ..
            } => {
                let wf_data = vec![WaveformData::new(
                    "onoise".to_string(),
                    frequencies.clone(),
                    output_noise.clone(),
                    Self::color_for_index(0),
                )];
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::PoleZero { .. } => {
                // Pole-Zero results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::Sensitivity { .. } => {
                // Sensitivity results are displayed in console, not as waveforms
                AnalysisResult::new(1, analysis_type, label.to_string())
            }

            SimulationResult::MonteCarlo { variables, .. } => {
                let wf_data: Vec<WaveformData> = variables
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
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Parametric {
                sweep_values,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            sweep_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Corner {
                x_values,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            x_values.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Reliability {
                years, waveforms, ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            years.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            iterations.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Soa {
                time, waveforms, ..
            } => {
                let wf_data: Vec<WaveformData> = waveforms
                    .iter()
                    .enumerate()
                    .map(|(idx, (name, wf))| {
                        WaveformData::new(
                            name.clone(),
                            time.clone(),
                            wf.y_values.clone(),
                            Self::color_for_index(idx),
                        )
                    })
                    .collect();
                AnalysisResult::new(1, analysis_type, label.to_string()).with_waveforms(wf_data)
            }

            SimulationResult::Empty { .. } => {
                AnalysisResult::new(1, analysis_type, label.to_string())
            }
        }
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

    /// Poll for simulation completion
    ///
    /// Checks if the current analysis has completed. On success, adds result
    /// to the run and starts the next queued analysis. When all analyses are

    pub(super) fn update_waveforms(
        &self,
        state: &mut AppState,
        result: &crate::simulation::SimulationResult,
    ) {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

        // Color palette for commercial-grade visualization
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

        // Clear previous waveforms
        state.simulation.waveforms.clear();

        match result {
            SimulationResult::DcOp(dc_result) => {
                // DC OP: Display voltages in console
                // Note: DC annotation overlay requires position mapping from schematic
                // which is handled separately when the schematic state is available
                log::info!(
                    "DC OP result has {} node voltages",
                    dc_result.node_voltages.len()
                );
                for (node, voltage) in &dc_result.node_voltages {
                    log::info!("  V({}) = {:.6} V", node, voltage);
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "V({}) = {:.6} V",
                        node, voltage
                    )));
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "DC OP: {} node voltages computed",
                    dc_result.node_voltages.len()
                )));

                // Auto-show log panel so user sees results
                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Log;
            }

            SimulationResult::Transient { time, waveforms } => {
                // Transient: Create waveform traces with time as X-axis
                let time_vec: Vec<f64> = time.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let color = COLORS[idx % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        name.clone(),
                        time_vec.clone(),
                        wf_data.y_values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }
                self.populate_transient_post_views(state, time, waveforms);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Transient: {} points, {} waveforms",
                    time.len(),
                    waveforms.len()
                )));

                // Auto-show waveform panel for better visibility
                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                // AC: Create magnitude traces (log-log or semi-log typically)
                // Commercial simulators show |V(node)| in dB and phase separately
                let freq_vec: Vec<f64> = frequencies.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    // Magnitude trace - use magnitude() for complex data, not raw real values
                    let mag_name = format!("|{}|", name);
                    let color = COLORS[idx % COLORS.len()].to_string();

                    // For AC analysis, use the magnitude of complex waveform data
                    let magnitude = wf_data.magnitude();

                    let waveform = WaveformData::new(mag_name, freq_vec.clone(), magnitude, color);
                    state.simulation.waveforms.push(waveform);
                }
                self.populate_ac_post_views(state, frequencies, waveforms);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "AC: {} points, {} waveforms",
                    frequencies.len(),
                    waveforms.len()
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
            } => {
                // DC Sweep: sweep variable as X-axis
                let x_vec: Vec<f64> = sweep_values.clone();

                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let color = COLORS[idx % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        name.clone(),
                        x_vec.clone(),
                        wf_data.y_values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "DC Sweep ({}): {} points, {} waveforms",
                    sweep_var,
                    sweep_values.len(),
                    waveforms.len()
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise,
                contributors,
            } => {
                // Noise: frequency as X-axis, noise spectral density as Y
                let freq_vec: Vec<f64> = frequencies.clone();

                // Output noise trace
                if !output_noise.is_empty() {
                    let waveform = WaveformData::new(
                        "onoise".to_string(),
                        freq_vec.clone(),
                        output_noise.clone(),
                        COLORS[0].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                // Input-referred noise trace (if present)
                if let Some(inoise) = input_noise {
                    if !inoise.is_empty() {
                        let waveform = WaveformData::new(
                            "inoise".to_string(),
                            freq_vec.clone(),
                            inoise.clone(),
                            COLORS[1].to_string(),
                        );
                        state.simulation.waveforms.push(waveform);
                    }
                }

                // Per-source contributions
                for (idx, (source, values)) in contributors.iter().enumerate() {
                    let color = COLORS[(idx + 2) % COLORS.len()].to_string();
                    let waveform = WaveformData::new(
                        format!("noise({})", source),
                        freq_vec.clone(),
                        values.clone(),
                        color,
                    );
                    state.simulation.waveforms.push(waveform);
                }

                // Calculate integrated noise
                let integrated: f64 = output_noise.iter().sum::<f64>().sqrt();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Noise: {} points, integrated output: {:.3e} V/√Hz",
                    frequencies.len(),
                    integrated
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::PoleZero { poles, zeros, gain } => {
                self.populate_pole_zero_view(state, poles, zeros, *gain);
                // Pole-Zero: Display in console (and optionally s-plane plot)
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Pole-Zero Analysis: DC gain = {:.4}",
                    gain
                )));

                for (i, (re, im)) in poles.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        // Real pole
                        let freq = re.abs() / (2.0 * std::f64::consts::PI);
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  Pole {}: {:.3e} rad/s ({:.3e} Hz)",
                            i + 1,
                            re,
                            freq
                        )));
                    } else {
                        // Complex pole
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  Pole {}: {:.3e} ± j{:.3e} rad/s",
                            i + 1,
                            re,
                            im.abs()
                        )));
                    }
                }

                for (i, (re, im)) in zeros.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  Zero {}: {:.3e} rad/s",
                            i + 1,
                            re
                        )));
                    } else {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  Zero {}: {:.3e} ± j{:.3e} rad/s",
                            i + 1,
                            re,
                            im.abs()
                        )));
                    }
                }
            }

            SimulationResult::Sensitivity {
                sensitivities,
                normalized,
            } => {
                // Sensitivity: Display in console as table
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Sensitivity Analysis: {} parameters",
                    sensitivities.len()
                )));

                // Sort by normalized sensitivity magnitude
                let mut sorted: Vec<_> = normalized.iter().collect();
                sorted.sort_by(|a, b| {
                    b.1.abs()
                        .partial_cmp(&a.1.abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                });

                for (param, norm_sens) in sorted.iter().take(10) {
                    if let Some(sens) = sensitivities.get(*param) {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                            "  {}: dV/d{} = {:.3e}, norm = {:.2}%",
                            param,
                            param,
                            sens,
                            **norm_sens * 100.0
                        )));
                    }
                }
            }

            SimulationResult::MonteCarlo {
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
            } => {
                self.populate_monte_carlo_histograms(state, variables);
                for (idx, var) in variables.iter().enumerate() {
                    if var.histogram.is_empty() || var.bin_edges.len() < 2 {
                        continue;
                    }
                    let x: Vec<f64> = var
                        .bin_edges
                        .windows(2)
                        .map(|window| (window[0] + window[1]) * 0.5)
                        .collect();
                    let y: Vec<f64> = var.histogram.iter().map(|count| *count as f64).collect();
                    let waveform = WaveformData::new(
                        format!("hist({})", var.name),
                        x,
                        y,
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Monte Carlo: {}/{} runs converged ({} failed), all_converged={}",
                    runs_completed, runs_requested, num_failures, all_converged
                )));

                for var in variables.iter().take(8) {
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "  {}: mean={:.6e}, sigma={:.6e}, min={:.6e}, max={:.6e}",
                        var.name, var.mean, var.std_dev, var.min, var.max
                    )));
                }

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = if state.simulation.waveforms.is_empty() {
                    crate::common::app::BottomPanelTab::Log
                } else {
                    crate::common::app::BottomPanelTab::Waveform
                };
            }

            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        sweep_values.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Parametric ({}): {} points, {} waveforms, {} failed points",
                    target,
                    sweep_values.len(),
                    waveforms.len(),
                    num_failures
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Corner {
                x_values,
                waveforms,
                num_failures,
                ..
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        x_values.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Corner sweep: {} points, {} waveforms, {} failed corners",
                    x_values.len(),
                    waveforms.len(),
                    num_failures
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Reliability {
                years,
                waveforms,
                device_results,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        years.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.simulation.reliability_results = device_results.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Reliability: {} lifetime points, {} devices analyzed",
                    years.len(),
                    device_results.len()
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Optimization {
                iterations,
                waveforms,
                best_cost,
                best_variables,
                converged,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        iterations.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Optimization: {} iterations, best cost {:.6e}, converged={}",
                    iterations.len(),
                    best_cost,
                    converged
                )));
                for (name, value) in best_variables.iter().take(8) {
                    state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                        "  {} = {:.6e}",
                        name, value
                    )));
                }

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Soa {
                time,
                waveforms,
                violations,
            } => {
                for (idx, (name, wf_data)) in waveforms.iter().enumerate() {
                    let waveform = WaveformData::new(
                        name.clone(),
                        time.clone(),
                        wf_data.y_values.clone(),
                        COLORS[idx % COLORS.len()].to_string(),
                    );
                    state.simulation.waveforms.push(waveform);
                }
                state.simulation.soa_violations = violations.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "SOA: {} sampled points, {} violations",
                    time.len(),
                    violations.len()
                )));

                state.panels.bottom_panel = true;
                state.panels.active_bottom_tab = crate::common::app::BottomPanelTab::Waveform;
            }

            SimulationResult::Empty { .. } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(
                    "Analysis complete (no waveform data)".to_string(),
                ));
            }
        }

        // Build node-to-waveform mapping for cross-probing
        state.simulation.node_to_waveform.clear();
        for (idx, wf) in state.simulation.waveforms.iter().enumerate() {
            state
                .simulation
                .node_to_waveform
                .insert(wf.name.clone(), idx);
        }
    }

    fn populate_transient_post_views(
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

    fn populate_ac_post_views(
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

    fn populate_pole_zero_view(
        &self,
        state: &mut AppState,
        poles: &[(f64, f64)],
        zeros: &[(f64, f64)],
        gain: f64,
    ) {
        let mut data = crate::analysis::pole_zero::data::PoleZeroData::new("Pole-Zero");
        data.gain = gain;
        for &(re, im) in poles {
            data.roots
                .push(crate::analysis::pole_zero::data::ComplexRoot::pole(re, im));
        }
        for &(re, im) in zeros {
            data.roots
                .push(crate::analysis::pole_zero::data::ComplexRoot::zero(re, im));
        }
        state.pole_zero_state.load_data(data);
    }

    fn populate_monte_carlo_histograms(
        &self,
        state: &mut AppState,
        variables: &[crate::simulation::results::MonteCarloVariableResult],
    ) {
        state.histogram_state.clear();

        for variable in variables {
            if variable.histogram.is_empty()
                || variable.bin_edges.len() != variable.histogram.len() + 1
            {
                continue;
            }

            let mut bins = Vec::with_capacity(variable.histogram.len());
            for (idx, count) in variable.histogram.iter().enumerate() {
                bins.push(crate::analysis::histogram::data::HistogramBin {
                    lower: variable.bin_edges[idx],
                    upper: variable.bin_edges[idx + 1],
                    count: *count,
                    weight: *count as f64,
                });
            }
            let total_count: usize = variable.histogram.iter().sum();
            let histogram = crate::analysis::histogram::data::Histogram {
                name: variable.name.clone(),
                bins,
                total_count,
                total_weight: total_count as f64,
                underflow: 0,
                overflow: 0,
                data_min: *variable.bin_edges.first().unwrap_or(&0.0),
                data_max: *variable.bin_edges.last().unwrap_or(&0.0),
            };

            if state.histogram_state.is_empty() {
                state.histogram_state.load_histogram(histogram);
            } else {
                state.histogram_state.add_histogram(histogram);
            }
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
