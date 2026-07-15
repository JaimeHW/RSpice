use rspice_core::abort_signal::AbortSignal;

use super::{EngineBridge, ensure_not_aborted};
use crate::simulation::config::{PoleZeroConfig, PzAnalysisType};
use crate::simulation::results::SimulationResult;
use crate::simulation::runner::SimulationError;

impl EngineBridge {
    /// Run pole-zero analysis.
    pub(super) fn run_pz(
        &self,
        netlist: &rspice_core::Netlist,
        config: &PoleZeroConfig,
        abort: &dyn AbortSignal,
    ) -> Result<SimulationResult, SimulationError> {
        ensure_not_aborted(abort)?;
        let engine = self.engine_for_netlist(netlist);
        let dc = engine
            .run_dc_op_with_abort(netlist, abort)
            .map_err(|e| self.translate_error(e))?;
        ensure_not_aborted(abort)?;
        let node_names = &dc.node_names;

        let input_idx =
            resolve_node_or_ground(&config.input_node, node_names).ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Invalid input node '{}' for pole-zero analysis",
                    config.input_node
                ))
            })?;
        let input_ref_idx =
            resolve_node_or_ground(&config.input_ref, node_names).ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Invalid input reference '{}' for pole-zero analysis",
                    config.input_ref
                ))
            })?;
        let output_idx =
            resolve_node_or_ground(&config.output_node, node_names).ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Invalid output node '{}' for pole-zero analysis",
                    config.output_node
                ))
            })?;
        let output_ref_idx =
            resolve_node_or_ground(&config.output_ref, node_names).ok_or_else(|| {
                SimulationError::InvalidConfig(format!(
                    "Invalid output reference '{}' for pole-zero analysis",
                    config.output_ref
                ))
            })?;

        let (input_pos, input_neg, input_sign) =
            canonicalize_port(input_idx, input_ref_idx, "input")?;
        let (output_pos, output_neg, output_sign) =
            canonicalize_port(output_idx, output_ref_idx, "output")?;

        let input_is_current = config.transfer_type.trim().eq_ignore_ascii_case("CUR");
        let (compute_poles, compute_zeros) = match config.analysis_type {
            PzAnalysisType::PolesOnly => (true, false),
            PzAnalysisType::ZerosOnly => (false, true),
            PzAnalysisType::PoleZero => (true, true),
        };

        let pz_result = engine
            .run_pz_ports_with_abort(
                netlist,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                input_is_current,
                compute_poles,
                compute_zeros,
                abort,
            )
            .map_err(|e| self.translate_error(e))?;

        let mut poles = Vec::with_capacity(pz_result.poles.len());
        for pole in &pz_result.poles {
            ensure_not_aborted(abort)?;
            poles.push((pole.re, pole.im));
        }
        let mut zeros = Vec::with_capacity(pz_result.zeros.len());
        for zero in &pz_result.zeros {
            ensure_not_aborted(abort)?;
            zeros.push((zero.re, zero.im));
        }

        Ok(SimulationResult::PoleZero {
            poles,
            zeros,
            gain: input_sign * output_sign * pz_result.dc_gain,
        })
    }
}

fn resolve_node_or_ground(name: &str, node_names: &[String]) -> Option<usize> {
    let trimmed = name.trim();
    if trimmed.is_empty() {
        return None;
    }
    if matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    ) {
        return Some(0);
    }
    if let Ok(idx) = trimmed.parse::<usize>()
        && idx < node_names.len()
    {
        return Some(idx);
    }
    let upper = trimmed.to_ascii_uppercase();
    node_names
        .iter()
        .position(|n| n.to_ascii_uppercase() == upper)
}

fn canonicalize_port(
    pos: usize,
    neg: usize,
    label: &str,
) -> Result<(usize, Option<usize>, f64), SimulationError> {
    if pos == neg {
        return Err(SimulationError::InvalidConfig(format!(
            "Invalid {} port: positive and reference nodes are the same",
            label
        )));
    }
    if pos != 0 {
        return Ok((pos, if neg == 0 { None } else { Some(neg) }, 1.0));
    }
    if neg == 0 {
        return Err(SimulationError::InvalidConfig(format!(
            "Invalid {} port: ground-ground is not allowed",
            label
        )));
    }
    Ok((neg, None, -1.0))
}
