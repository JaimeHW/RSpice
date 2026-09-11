//! Result side effects.
//!
//! What a completed run changes beyond the waveforms themselves: run
//! history, the active viewer selection, and the console and log entries
//! that record what happened.

use super::*;

/// Echo evaluated `.MEAS` results to the console, the way the CLI prints
/// them — value lines for successes, warnings for failures.
fn echo_measurements(state: &mut AppState, measurements: &[rspice_core::MeasureResult]) {
    if measurements.is_empty() {
        return;
    }
    state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
        "Measurements ({}):",
        measurements.len()
    )));
    for m in measurements {
        match (m.value, m.passed) {
            (Some(value), true) => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "  {} = {:.6e}",
                    m.name, value
                )));
            }
            (Some(value), false) => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::warning(format!(
                    "  {} = {:.6e} FAILED ({})",
                    m.name,
                    value,
                    m.error.as_deref().unwrap_or("verification contract failed")
                )));
            }
            (None, _) => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::warning(format!(
                    "  {} = FAILED ({})",
                    m.name,
                    m.error.as_deref().unwrap_or("unknown")
                )));
            }
        }
    }
}

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
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "V({}) = {:.6} V",
                            node, voltage
                        )));
                    }
                }

                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "DC OP: {} node voltages computed",
                    dc_result.node_voltages.len()
                )));
            }

            SimulationResult::Transient {
                time,
                waveforms,
                measurements,
                ..
            } => {
                self.invalidate_transient_post_views(state);
                self.prime_transient_fft_source_selection(state);

                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Transient: {} points, {} waveforms",
                    time.len(),
                    waveforms.len()
                )));
                echo_measurements(state, measurements);
            }

            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
                ..
            } => {
                self.populate_ac_post_views(state, frequencies, waveforms);

                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "AC: {} points, {} waveforms",
                    frequencies.len(),
                    waveforms.len()
                )));
                echo_measurements(state, measurements);
            }

            SimulationResult::Pstb {
                modes,
                floquet_evidence,
                orbit_kind,
                stability_threshold,
                probe_instance,
                detect_subharmonics,
                trivial_multiplier_index,
                stability_verdict,
                stability_classification,
                min_stability_margin_db,
                num_unstable,
                subharmonics,
                converged,
                iterations,
                mode_indices,
                ..
            } => {
                let margin = min_stability_margin_db
                    .map(|value| format!("{value:.3} dB"))
                    .unwrap_or_else(|| "not applicable".to_owned());
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "PSTB: {} authenticated modes ({} displayed), {} unstable, margin {margin}",
                    modes.len(),
                    mode_indices.len(),
                    num_unstable,
                )));
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "  verdict={stability_verdict:?}, classification={stability_classification:?}, orbit={orbit_kind:?}, threshold={stability_threshold:.9}, probe={probe_instance}, detect_subharmonics={detect_subharmonics}, trivial_mode={trivial_multiplier_index:?}, subharmonics={subharmonics:?}, converged={converged}, iterations={iterations}, evidence={floquet_evidence:?}"
                )));
            }

            SimulationResult::HarmonicBalance {
                frequencies,
                waveforms,
                measurements,
                ..
            } => {
                self.populate_ac_post_views(state, frequencies, waveforms);
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Harmonic Balance: {} components, {} waveforms",
                    frequencies.len(),
                    waveforms.len()
                )));
                echo_measurements(state, measurements);
            }

            SimulationResult::DcSweep {
                sweep_var,
                sweep_values,
                waveforms,
                measurements,
                ..
            } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "DC Sweep ({}): {} points, {} waveforms",
                    sweep_var,
                    sweep_values.len(),
                    waveforms.len()
                )));
                echo_measurements(state, measurements);
            }

            SimulationResult::Noise {
                frequencies,
                output_noise,
                measurements,
                ..
            } => {
                // Calculate integrated noise
                let integrated: f64 = output_noise.iter().sum::<f64>().sqrt();
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Noise: {} points, integrated output: {:.3e} V/sqrt(Hz)",
                    frequencies.len(),
                    integrated
                )));
                echo_measurements(state, measurements);
            }

            SimulationResult::PoleZero {
                poles,
                zeros,
                pole_evidence,
                zero_evidence,
                gain,
            } => {
                // The immutable retained result is the sole plot authority.
                // Console output is a secondary execution log only.
                let gain = gain
                    .map(|gain| format!("{gain:.4}"))
                    .unwrap_or_else(|| "unavailable".to_owned());
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Pole-Zero Analysis: DC gain = {gain}"
                )));
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Evidence: poles {} · zeros {}",
                    pole_evidence.label(),
                    zero_evidence.label()
                )));

                for (i, (re, im)) in poles.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        // Real pole
                        let freq = re.abs() / (2.0 * std::f64::consts::PI);
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "  Pole {}: {:.3e} rad/s ({:.3e} Hz)",
                            i + 1,
                            re,
                            freq
                        )));
                    } else {
                        // Complex pole
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "  Pole {}: {:.3e} +/- j{:.3e} rad/s",
                            i + 1,
                            re,
                            im.abs()
                        )));
                    }
                }

                for (i, (re, im)) in zeros.iter().enumerate() {
                    if im.abs() < 1e-10 {
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "  Zero {}: {:.3e} rad/s",
                            i + 1,
                            re
                        )));
                    } else {
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                ..
            } => {
                use rspice_core::analysis::sensitivity::SensitivityValue;
                let unavailable = normalized
                    .values()
                    .filter(|value| value.value().is_none())
                    .count();
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Sensitivity Analysis: {} parameters, {unavailable} normalized values unavailable", sensitivities.len()
                )));
                let mut sorted: Vec<_> = normalized.iter().collect();
                sorted.sort_by(|a, b| {
                    use std::cmp::Ordering;
                    match (a.1.value(), b.1.value()) {
                        (Some(a), Some(b)) => b.abs().total_cmp(&a.abs()),
                        (Some(_), None) => Ordering::Less,
                        (None, Some(_)) => Ordering::Greater,
                        _ => Ordering::Equal,
                    }
                    .then_with(|| a.0.cmp(b.0))
                });
                let format_quantity = |quantity: SensitivityValue<f64>| match quantity {
                    SensitivityValue::Available(value) => format!("{value:.3e}"),
                    SensitivityValue::Unavailable { unavailable } => {
                        format!("unavailable ({})", unavailable.as_str())
                    }
                };
                for (parameter, normalized) in sorted.into_iter().take(10) {
                    if let Some(raw) = sensitivities.get(parameter) {
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "  {parameter}: derivative = {}, normalized = {}",
                            format_quantity(*raw),
                            format_quantity(*normalized)
                        )));
                    }
                }
            }

            SimulationResult::TransferFunction {
                input_source,
                output_expression,
                gain,
                input_resistance,
                output_resistance,
                ..
            } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Transfer function: {} -> {}",
                    input_source, output_expression
                )));
                for (label, value) in [
                    ("gain", gain.as_ref()),
                    ("input resistance", input_resistance.as_ref()),
                    ("output resistance", output_resistance.as_ref()),
                ] {
                    if let Some(value) = value {
                        let rendered = match value {
                            crate::simulation::results::TransferFunctionScalar::Finite(value) => {
                                format!("{value:.6e}")
                            }
                            crate::simulation::results::TransferFunctionScalar::PositiveInfinity => {
                                "+infinity".to_owned()
                            }
                            crate::simulation::results::TransferFunctionScalar::NegativeInfinity => {
                                "-infinity".to_owned()
                            }
                        };
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                            "  {label} = {rendered}"
                        )));
                    }
                }
            }

            SimulationResult::MonteCarlo {
                seed,
                runs_requested,
                runs_completed,
                num_failures,
                all_converged,
                variables,
                ..
            } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Monte Carlo: {}/{} runs converged ({} failed), seed={}, all_converged={}",
                    runs_completed, runs_requested, num_failures, seed, all_converged
                )));

                for var in variables.iter().take(8) {
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                ..
            } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
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
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "Optimization: {} iterations, best cost {:.6e}, converged={}",
                    iterations.len(),
                    best_cost,
                    converged
                )));
                for (name, value) in best_variables.iter().take(8) {
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                        "  {} = {:.6e}",
                        name, value
                    )));
                }
            }

            SimulationResult::Soa {
                time,
                waveforms: _,
                violations,
                ..
            } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(format!(
                    "SOA: {} sampled points, {} violations",
                    time.len(),
                    violations.len()
                )));
            }

            SimulationResult::MeasurementsOnly { .. } => {
                state.push_sim_message(crate::diagnostics::ConsoleMessage::info(
                    "Analysis complete (scalar result evidence retained)".to_string(),
                ));
            }
        }
    }
}
