//! Pole-zero analysis runner.

use super::{build_engine_config, parse_runner_netlist};
use crate::output_spec::resolve_node_or_ground_index;
use rspice_core::Value;
use rspice_core::engine::Engine;
use std::path::Path;

/// Pole-zero analysis data
#[derive(Debug, Clone)]
pub struct PoleZeroData {
    /// Poles in the s-plane (real, imag)
    pub poles: Vec<(Value, Value)>,
    /// Zeros in the s-plane (real, imag)
    pub zeros: Vec<(Value, Value)>,
    /// DC transfer gain
    pub gain: Value,
}

/// Run pole-zero analysis.
pub fn run_pole_zero_analysis(
    netlist_text: &str,
    input_node: &str,
    input_ref: &str,
    output_node: &str,
    output_ref: &str,
    transfer_type: &str,
    analysis_type: &str,
) -> Result<PoleZeroData, String> {
    run_pole_zero_analysis_with_source_path(
        netlist_text,
        input_node,
        input_ref,
        output_node,
        output_ref,
        transfer_type,
        analysis_type,
        None,
    )
}

/// Run pole-zero analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_pole_zero_analysis_with_source_path(
    netlist_text: &str,
    input_node: &str,
    input_ref: &str,
    output_node: &str,
    output_ref: &str,
    transfer_type: &str,
    analysis_type: &str,
    source_path: Option<&Path>,
) -> Result<PoleZeroData, String> {
    let input_node = input_node.trim();
    let output_node = output_node.trim();
    if input_node.is_empty() {
        return Err("Pole-zero input_node is required".to_string());
    }
    if output_node.is_empty() {
        return Err("Pole-zero output_node is required".to_string());
    }

    let transfer_type = transfer_type.trim().to_ascii_uppercase();
    if transfer_type != "VOL" && transfer_type != "CUR" {
        return Err("Pole-zero transfer_type must be VOL or CUR".to_string());
    }

    let analysis_type = analysis_type.trim().to_ascii_uppercase();
    if analysis_type != "PZ" && analysis_type != "POL" && analysis_type != "ZER" {
        return Err("Pole-zero analysis_type must be PZ, POL, or ZER".to_string());
    }

    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for pole-zero): {}", e))?;

    let input_idx = resolve_node_or_ground_index(input_node, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero input node '{}' not found", input_node))?;
    let input_ref_idx = resolve_node_or_ground_index(input_ref, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero input reference '{}' not found", input_ref))?;
    let output_idx = resolve_node_or_ground_index(output_node, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero output node '{}' not found", output_node))?;
    let output_ref_idx = resolve_node_or_ground_index(output_ref, &dc_result.node_names)
        .ok_or_else(|| format!("Pole-zero output reference '{}' not found", output_ref))?;

    if input_idx == input_ref_idx {
        return Err("Pole-zero input_node and input_ref cannot be the same node".to_string());
    }
    if output_idx == output_ref_idx {
        return Err("Pole-zero output_node and output_ref cannot be the same node".to_string());
    }

    let (input_pos, input_neg, input_sign) = canonicalize_pz_port(input_idx, input_ref_idx)
        .map_err(|e| format!("Invalid pole-zero input port: {}", e))?;
    let (output_pos, output_neg, output_sign) = canonicalize_pz_port(output_idx, output_ref_idx)
        .map_err(|e| format!("Invalid pole-zero output port: {}", e))?;

    let input_is_current = transfer_type == "CUR";
    let (compute_poles, compute_zeros) = match analysis_type.as_str() {
        "POL" => (true, false),
        "ZER" => (false, true),
        _ => (true, true),
    };

    let pz_result = engine
        .run_pz_ports(
            &netlist,
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            input_is_current,
            compute_poles,
            compute_zeros,
        )
        .map_err(|e| {
            format!(
                "Pole-zero analysis error (input={:?}->{:?}, output={:?}->{:?}): {}",
                input_idx, input_ref_idx, output_idx, output_ref_idx, e
            )
        })?;

    let mut poles: Vec<(Value, Value)> = pz_result.poles.iter().map(|p| (p.re, p.im)).collect();
    let mut zeros: Vec<(Value, Value)> = pz_result.zeros.iter().map(|z| (z.re, z.im)).collect();

    match analysis_type.as_str() {
        "POL" => zeros.clear(),
        "ZER" => poles.clear(),
        _ => {}
    }

    Ok(PoleZeroData {
        poles,
        zeros,
        gain: input_sign * output_sign * pz_result.dc_gain,
    })
}

fn canonicalize_pz_port(pos: usize, neg: usize) -> Result<(usize, Option<usize>, Value), String> {
    if pos == neg {
        return Err("positive and reference nodes cannot be the same".to_string());
    }

    if pos != 0 {
        return Ok((pos, if neg == 0 { None } else { Some(neg) }, 1.0));
    }

    if neg == 0 {
        return Err("port cannot be ground-ground".to_string());
    }

    // Canonicalize V(0, n) or I(0, n) to -(V(n,0) / I(n,0)).
    Ok((neg, None, -1.0))
}
