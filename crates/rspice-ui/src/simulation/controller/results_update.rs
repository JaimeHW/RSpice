use super::*;

impl SimulationController {
    pub(super) fn apply_result_side_effects(
        &mut self,
        state: &mut AppState,
        result: &crate::simulation::SimulationResult,
    ) {
        use crate::simulation::SimulationResult;

        match result {
            SimulationResult::DcOp(dc_result) => {
                // DC OP: Display voltages in console
                // Note: DC annotation overlay requires position mapping from schematic
                // which is handled separately when the schematic state is available
                log::info!(
                    "DC OP result has {} node voltages",
                    dc_result.node_voltages.len()
                );
                // Node voltages live in the DC-OP results view; echo them
                // to the console only for small circuits so one large run
                // doesn't flood the whole log buffer.
                const DC_OP_CONSOLE_ECHO_LIMIT: usize = 24;
                if dc_result.node_voltages.len() <= DC_OP_CONSOLE_ECHO_LIMIT {
                    for (node, voltage) in &dc_result.node_voltages {
                        state.push_sim_message(crate::common::app::ConsoleMessage::info(
                            format!("V({}) = {:.6} V", node, voltage),
                        ));
                    }
                }

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "DC OP: {} node voltages computed",
                    dc_result.node_voltages.len()
                )));

            }

            SimulationResult::Transient { time, waveforms } => {
                self.invalidate_transient_post_views(state);
                self.prime_transient_fft_source_selection(state);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Transient: {} points, {} waveforms",
                    time.len(),
                    waveforms.len()
                )));

            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
            } => {
                self.populate_ac_post_views(state, frequencies, waveforms);

                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "AC: {} points, {} waveforms",
                    frequencies.len(),
                    waveforms.len()
                )));

            }

            SimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
            } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "DC Sweep ({}): {} points, {} waveforms",
                    sweep_var,
                    sweep_values.len(),
                    waveforms.len()
                )));

            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                input_noise: _,
                contributors: _,
            } => {
                // Calculate integrated noise
                let integrated: f64 = output_noise.iter().sum::<f64>().sqrt();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Noise: {} points, integrated output: {:.3e} V/sqrt(Hz)",
                    frequencies.len(),
                    integrated
                )));

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
                            "  Pole {}: {:.3e} +/- j{:.3e} rad/s",
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
                            "  Zero {}: {:.3e} +/- j{:.3e} rad/s",
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

            }

            SimulationResult::Parametric {
                target,
                sweep_values,
                waveforms,
                num_failures,
            } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Parametric ({}): {} points, {} waveforms, {} failed points",
                    target,
                    sweep_values.len(),
                    waveforms.len(),
                    num_failures
                )));

            }

            SimulationResult::Corner {
                x_values,
                waveforms,
                num_failures,
                ..
            } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Corner sweep: {} points, {} waveforms, {} failed corners",
                    x_values.len(),
                    waveforms.len(),
                    num_failures
                )));

            }

            SimulationResult::Reliability {
                years,
                waveforms: _,
                device_results,
            } => {
                state.simulation.reliability_results = device_results.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "Reliability: {} lifetime points, {} devices analyzed",
                    years.len(),
                    device_results.len()
                )));

            }

            SimulationResult::Optimization {
                iterations,
                waveforms: _,
                best_cost,
                best_variables,
                converged,
            } => {
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

            }

            SimulationResult::Soa {
                time,
                waveforms: _,
                violations,
            } => {
                state.simulation.soa_violations = violations.clone();
                state.push_sim_message(crate::common::app::ConsoleMessage::info(format!(
                    "SOA: {} sampled points, {} violations",
                    time.len(),
                    violations.len()
                )));

            }

            SimulationResult::MeasurementsOnly { .. } => {
                state.push_sim_message(crate::common::app::ConsoleMessage::info(
                    "Analysis complete (no waveform data)".to_string(),
                ));
            }
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
        state.analysis.pole_zero_state.load_data(data);
    }

    fn populate_monte_carlo_histograms(
        &self,
        state: &mut AppState,
        variables: &[crate::simulation::results::MonteCarloVariableResult],
    ) {
        state.analysis.histogram_state.clear();

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

            if state.analysis.histogram_state.is_empty() {
                state.analysis.histogram_state.load_histogram(histogram);
            } else {
                state.analysis.histogram_state.add_histogram(histogram);
            }
        }
    }
}
