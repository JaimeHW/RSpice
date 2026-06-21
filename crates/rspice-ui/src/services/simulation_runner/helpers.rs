use std::path::{Path, PathBuf};

use rspice_core::Value;
use rspice_core::netlist::ElementKind;

pub(super) fn parse_runner_netlist(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<rspice_core::Netlist, String> {
    let parse_source = runner_parse_source(source_path);
    rspice_core::Netlist::parse_with_path(netlist_text, &parse_source)
        .map_err(|e| format!("Parse error: {}", e))
}

fn runner_parse_source(source_path: Option<&Path>) -> PathBuf {
    const GENERATED_NETLIST_NAME: &str = "__rspice_ui_runner_generated__.cir";

    match source_path {
        Some(path) if path.is_dir() => path.join(GENERATED_NETLIST_NAME),
        Some(path) => path.to_path_buf(),
        None => std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join(GENERATED_NETLIST_NAME),
    }
}

pub(super) fn build_voltage_output_expr(output_node: &str, output_ref: Option<&str>) -> String {
    let output_node = output_node.trim();
    let output_ref = output_ref
        .map(str::trim)
        .filter(|name| !name.is_empty() && !is_ground_like(name));
    match output_ref {
        Some(reference) => format!("V({},{})", output_node, reference),
        None => format!("V({})", output_node),
    }
}

pub(super) fn is_ground_like(name: &str) -> bool {
    matches!(
        name.trim().to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    )
}

pub(super) fn infer_primary_source_name(netlist: &rspice_core::Netlist) -> Option<String> {
    netlist
        .elements
        .iter()
        .find_map(|element| match &element.kind {
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_) => {
                Some(element.name.clone())
            }
            _ => None,
        })
}

pub(super) fn netlist_has_independent_source_named(
    netlist: &rspice_core::Netlist,
    source_name: &str,
) -> bool {
    netlist.elements.iter().any(|element| {
        (matches!(
            &element.kind,
            ElementKind::VoltageSource(_) | ElementKind::CurrentSource(_)
        )) && element.name.eq_ignore_ascii_case(source_name)
    })
}

pub(super) fn infer_primary_output_node(node_names: &[String]) -> Option<String> {
    node_names
        .iter()
        .rev()
        .find(|name| !is_ground_like(name))
        .cloned()
}

pub(super) fn normalize_voltage_signal_name(name: &str) -> String {
    let trimmed = name.trim();
    if trimmed.len() > 3 && trimmed[..2].eq_ignore_ascii_case("V(") && trimmed.ends_with(')') {
        return trimmed[2..trimmed.len() - 1].trim().to_ascii_uppercase();
    }
    trimmed.to_ascii_uppercase()
}

/// Generate frequency sweep points
pub(super) fn generate_freq_points(
    start: Value,
    stop: Value,
    points: usize,
    sweep_type: &str,
) -> Result<Vec<Value>, String> {
    if points == 0 {
        return Err("frequency sweep must request at least one point".to_string());
    }
    if !start.is_finite() || !stop.is_finite() || start <= 0.0 || stop <= 0.0 {
        return Err(format!(
            "frequency sweep bounds must be finite and positive (start={start}, stop={stop})"
        ));
    }
    if stop < start {
        return Err(format!(
            "frequency sweep stop frequency ({stop}) must be greater than or equal to start frequency ({start})"
        ));
    }

    match sweep_type.to_lowercase().as_str() {
        "dec" | "decade" => {
            let num_decades = (stop / start).log10();
            let total_points = ((points as f64) * num_decades).round() as usize;
            let total_points = total_points.max(2);
            Ok((0..total_points)
                .map(|idx| {
                    let t = idx as f64 / (total_points - 1) as f64;
                    start * (stop / start).powf(t)
                })
                .collect())
        }
        "oct" | "octave" => {
            let num_octaves = (stop / start).log2();
            let total_points = ((points as f64) * num_octaves).round() as usize;
            let total_points = total_points.max(2);
            Ok((0..total_points)
                .map(|idx| {
                    let t = idx as f64 / (total_points - 1) as f64;
                    start * (stop / start).powf(t)
                })
                .collect())
        }
        "lin" | "linear" => Ok((0..points)
            .map(|idx| {
                let t = idx as f64 / (points - 1).max(1) as f64;
                start + t * (stop - start)
            })
            .collect()),
        _ => Err(format!(
            "unknown frequency sweep type '{sweep_type}'; expected lin, dec, or oct"
        )),
    }
}
