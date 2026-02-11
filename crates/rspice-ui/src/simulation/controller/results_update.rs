use super::*;

impl SimulationController {
    pub(super) fn update_waveforms(
        &self,
        state: &mut AppState,
        result: &crate::simulation::SimulationResult,
    ) {
        use crate::simulation::SimulationResult;
        use crate::state::WaveformData;

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
                    let color = Self::color_for_index(idx);
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
                    let color = Self::color_for_index(idx);

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
                    let color = Self::color_for_index(idx);
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
                        Self::color_for_index(0),
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
                            Self::color_for_index(1),
                        );
                        state.simulation.waveforms.push(waveform);
                    }
                }

                // Per-source contributions
                for (idx, (source, values)) in contributors.iter().enumerate() {
                    let color = Self::color_for_index(idx + 2);
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
                        Self::color_for_index(idx),
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
                        Self::color_for_index(idx),
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
                        Self::color_for_index(idx),
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
                        Self::color_for_index(idx),
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
                        Self::color_for_index(idx),
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
                        Self::color_for_index(idx),
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
}
