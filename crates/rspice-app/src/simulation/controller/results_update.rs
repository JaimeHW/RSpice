//! Result side effects.
//!
//! What a completed analysis changes beyond the waveforms themselves: the
//! derived viewers that read it, and the `.MEAS` results the log prints.
//! Result values are the Results workspace's to show; the log says what was
//! measured, the way a Spectre log does, and nothing else about them.

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
            SimulationResult::Transient { measurements, .. } => {
                self.invalidate_transient_post_views(state);
                self.prime_transient_fft_source_selection(state);
                echo_measurements(state, measurements);
            }
            SimulationResult::Stb {
                response,
                measurements,
            } => {
                let frequencies: Vec<_> =
                    response.bode_points.iter().map(|p| p.frequency).collect();
                match SimulationResult::stb_waveforms(response) {
                    Ok(waveforms) => self.populate_ac_post_views(state, &frequencies, &waveforms),
                    Err(error) => {
                        state.push_sim_message(crate::diagnostics::ConsoleMessage::error(error))
                    }
                }
                for warning in &response.warnings {
                    state.push_sim_message(crate::diagnostics::ConsoleMessage::warning(
                        warning.clone(),
                    ));
                }
                echo_measurements(state, measurements);
            }
            SimulationResult::Ac {
                frequencies,
                waveforms,
                measurements,
                ..
            }
            | SimulationResult::HarmonicBalance {
                frequencies,
                waveforms,
                measurements,
                ..
            } => {
                self.populate_ac_post_views(state, frequencies, waveforms);
                echo_measurements(state, measurements);
            }
            SimulationResult::Qpnoise {
                frequencies,
                waveforms,
                ..
            }
            | SimulationResult::Qpxf {
                frequencies,
                waveforms,
                ..
            }
            | SimulationResult::Qpac {
                frequencies,
                waveforms,
                ..
            }
            | SimulationResult::Qpss {
                frequencies,
                waveforms,
                ..
            } => self.populate_ac_post_views(state, frequencies, waveforms),
            SimulationResult::DcSweep { measurements, .. }
            | SimulationResult::Noise { measurements, .. } => {
                echo_measurements(state, measurements);
            }
            SimulationResult::DcOp(_)
            | SimulationResult::Pstb { .. }
            | SimulationResult::PoleZero { .. }
            | SimulationResult::SensitivityStudy { .. }
            | SimulationResult::DcMismatch { .. }
            | SimulationResult::TransferFunction { .. }
            | SimulationResult::MonteCarlo { .. }
            | SimulationResult::Parametric { .. }
            | SimulationResult::Corner { .. }
            | SimulationResult::Optimization { .. }
            | SimulationResult::Soa { .. }
            | SimulationResult::Fft { .. }
            | SimulationResult::MeasurementsOnly { .. } => {}
        }
    }
}
