//! Transfer-function analysis runner.

use super::{
    build_engine_config, build_voltage_output_expr, generate_freq_points,
    infer_primary_output_node, infer_primary_source_name, parse_runner_netlist,
};
use crate::output_spec::{ac_output_value, parse_output_spec};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::engine::Engine;
use rspice_core::netlist::{ElementKind, SourceSpec};
use std::path::Path;

/// Frequency sweep type for transfer function analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TfFrequencySweep {
    Decade,
    Octave,
    Linear,
}

impl TfFrequencySweep {
    fn keyword(self) -> &'static str {
        match self {
            Self::Decade => "dec",
            Self::Octave => "oct",
            Self::Linear => "lin",
        }
    }
}

/// Explicit configuration for transfer-function execution.
#[derive(Debug, Clone)]
pub struct TfRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: TfFrequencySweep,
    pub input_source: String,
    pub output_node: String,
    pub output_ref: Option<String>,
    pub group_delay: bool,
    pub input_impedance: bool,
    pub output_impedance: bool,
}

impl Default for TfRunConfig {
    fn default() -> Self {
        Self {
            start_freq: 1.0,
            stop_freq: 1e9,
            points_per_unit: 10,
            sweep: TfFrequencySweep::Decade,
            input_source: "VIN".to_string(),
            output_node: "VOUT".to_string(),
            output_ref: None,
            group_delay: false,
            input_impedance: false,
            output_impedance: false,
        }
    }
}

impl TfRunConfig {
    fn validate(&self) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err("TF start frequency must be positive".to_string());
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err("TF stop frequency must be >= start frequency".to_string());
        }
        if self.points_per_unit == 0 {
            return Err("TF points per unit must be greater than zero".to_string());
        }
        if self.input_source.trim().is_empty() {
            return Err("TF input source must be specified".to_string());
        }
        if self.output_node.trim().is_empty() {
            return Err("TF output node must be specified".to_string());
        }
        Ok(())
    }
}

/// Transfer-function analysis data.
#[derive(Debug, Clone)]
pub struct TfData {
    /// Frequency points (Hz).
    pub frequencies: Vec<Value>,
    /// Complex transfer function H(jw).
    pub transfer: Vec<Complex64>,
    /// Magnitude response in dB.
    pub magnitude_db: Vec<Value>,
    /// Phase response in degrees.
    pub phase_deg: Vec<Value>,
    /// Group delay curve: (frequency_hz, delay_s).
    pub group_delay: Option<Vec<(Value, Value)>>,
    /// Input impedance vs frequency (Ohms), if requested.
    pub input_impedance: Option<Vec<Complex64>>,
    /// Output impedance vs frequency (Ohms), if requested.
    pub output_impedance: Option<Vec<Complex64>>,
    /// Output trace label (for display).
    pub output_label: String,
    /// Input source name (for display).
    pub input_source: String,
    /// Low-frequency gain magnitude (linear).
    pub dc_gain: Option<Value>,
}

/// Run transfer-function analysis with explicit configuration.
pub fn run_tf_analysis_with_config(
    netlist_text: &str,
    config: &TfRunConfig,
) -> Result<TfData, String> {
    run_tf_analysis_with_config_and_source_path(netlist_text, config, None)
}

/// Run transfer-function analysis with explicit configuration and a source path
/// used to resolve relative includes and model file references.
pub fn run_tf_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &TfRunConfig,
    source_path: Option<&Path>,
) -> Result<TfData, String> {
    config.validate()?;

    let parsed_netlist = parse_runner_netlist(netlist_text, source_path)?;

    // Build a baseline netlist with all AC source magnitudes forced to zero.
    // We then explicitly excite only the requested input source to keep the
    // transfer denominator deterministic and independent of unrelated sources.
    let mut tf_netlist = parsed_netlist.clone();
    zero_all_source_ac(&mut tf_netlist);
    set_source_ac_excitation(&mut tf_netlist, &config.input_source, 1.0, 0.0)?;

    let engine = Engine::new(build_engine_config(&tf_netlist, None));
    let dc_result = engine
        .run_dc_op(&tf_netlist)
        .map_err(|e| format!("DC OP error (required for TF): {}", e))?;
    let circuit = engine
        .build_circuit(&tf_netlist)
        .map_err(|e| format!("Circuit build error (required for TF): {}", e))?;

    let output_expr =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());
    let output_spec = parse_output_spec(&output_expr, &dc_result.node_names, &circuit)
        .ok_or_else(|| format!("TF output '{}' could not be resolved", output_expr))?;

    let frequencies = generate_freq_points(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
    );
    if frequencies.is_empty() {
        return Err("TF frequency sweep produced no points".to_string());
    }

    let ac_results = engine
        .run_ac(&tf_netlist, &frequencies)
        .map_err(|e| format!("TF AC analysis error: {}", e))?;
    if ac_results.len() != frequencies.len() {
        return Err(format!(
            "TF AC analysis returned {} points for {} requested frequencies",
            ac_results.len(),
            frequencies.len()
        ));
    }

    let transfer: Vec<Complex64> = ac_results
        .iter()
        .map(|point| ac_output_value(point, &output_spec))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("TF output extraction error: {}", e))?;
    let magnitude_db: Vec<Value> = transfer
        .iter()
        .map(|h| 20.0 * h.norm().max(1e-30).log10())
        .collect();
    let phase_deg: Vec<Value> = transfer.iter().map(|h| h.arg().to_degrees()).collect();

    let input_impedance = if config.input_impedance {
        let branch_ordinal = circuit
            .get_branch_by_name(config.input_source.trim())
            .ok_or_else(|| {
                format!(
                    "TF input source '{}' does not expose an AC branch current; cannot compute Zin",
                    config.input_source
                )
            })? as usize;
        let branch_idx = branch_ordinal.saturating_sub(1);
        Some(
            ac_results
                .iter()
                .map(|point| {
                    let iin = point.currents.get(branch_idx).copied().ok_or_else(|| {
                        format!(
                            "TF input source '{}' branch index {} is unavailable in AC result",
                            config.input_source, branch_idx
                        )
                    })?;
                    if iin.norm() <= 1e-30 {
                        Ok(Complex64::new(f64::INFINITY, 0.0))
                    } else {
                        Ok(Complex64::new(1.0, 0.0) / iin)
                    }
                })
                .collect::<Result<Vec<_>, String>>()?,
        )
    } else {
        None
    };

    let output_impedance = if config.output_impedance {
        let mut zout_netlist = parsed_netlist.clone();
        zero_all_source_ac(&mut zout_netlist);
        inject_tf_output_test_source(
            &mut zout_netlist,
            config.output_node.trim(),
            config.output_ref.as_deref(),
        )?;

        let zout_engine = Engine::new(build_engine_config(&zout_netlist, None));
        let zout_dc = zout_engine
            .run_dc_op(&zout_netlist)
            .map_err(|e| format!("DC OP error (required for TF Zout): {}", e))?;
        let zout_circuit = zout_engine
            .build_circuit(&zout_netlist)
            .map_err(|e| format!("Circuit build error (required for TF Zout): {}", e))?;
        let zout_spec = parse_output_spec(&output_expr, &zout_dc.node_names, &zout_circuit)
            .ok_or_else(|| format!("TF output '{}' could not be resolved for Zout", output_expr))?;

        let zout_points = zout_engine
            .run_ac(&zout_netlist, &frequencies)
            .map_err(|e| format!("TF output-impedance AC analysis error: {}", e))?;
        Some(
            zout_points
                .iter()
                .map(|point| {
                    ac_output_value(point, &zout_spec)
                        .map_err(|e| format!("TF output-impedance extraction error: {}", e))
                })
                .collect::<Result<Vec<_>, String>>()?,
        )
    } else {
        None
    };

    let group_delay = if config.group_delay && frequencies.len() >= 2 {
        use std::f64::consts::PI;
        let mut points = Vec::with_capacity(frequencies.len().saturating_sub(1));
        let mut prev_phase = transfer[0].arg();
        for idx in 1..frequencies.len() {
            let df = frequencies[idx] - frequencies[idx - 1];
            if df <= 0.0 {
                prev_phase = transfer[idx].arg();
                continue;
            }
            let mut phase = transfer[idx].arg();
            while phase - prev_phase > PI {
                phase -= 2.0 * PI;
            }
            while phase - prev_phase < -PI {
                phase += 2.0 * PI;
            }
            let delay = -(phase - prev_phase) / (2.0 * PI * df);
            let mid = (frequencies[idx - 1] + frequencies[idx]) * 0.5;
            points.push((mid, delay));
            prev_phase = phase;
        }
        Some(points)
    } else {
        None
    };

    Ok(TfData {
        dc_gain: transfer.first().map(|h| h.norm()),
        frequencies,
        transfer,
        magnitude_db,
        phase_deg,
        group_delay,
        input_impedance,
        output_impedance,
        output_label: output_expr,
        input_source: config.input_source.clone(),
    })
}

/// Run transfer-function analysis using inferred/default settings.
pub fn run_tf_analysis(netlist_text: &str) -> Result<TfData, String> {
    run_tf_analysis_with_source_path(netlist_text, None)
}

/// Run transfer-function analysis using inferred/default settings and a source
/// path used to resolve relative includes and model file references.
pub fn run_tf_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<TfData, String> {
    let netlist = parse_runner_netlist(netlist_text, source_path)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op(&netlist)
        .map_err(|e| format!("DC OP error (required for TF defaults): {}", e))?;

    let cfg = infer_tf_run_config(&netlist, &dc_result.node_names)?;
    run_tf_analysis_with_config_and_source_path(netlist_text, &cfg, source_path)
}

fn infer_tf_run_config(
    netlist: &rspice_core::Netlist,
    node_names: &[String],
) -> Result<TfRunConfig, String> {
    let input_source = infer_primary_source_name(netlist)
        .ok_or_else(|| "TF requires at least one independent source in the netlist".to_string())?;
    let output_node = infer_primary_output_node(node_names).ok_or_else(|| {
        "TF could not infer an output node; ensure at least one non-ground node exists".to_string()
    })?;
    Ok(TfRunConfig {
        input_source,
        output_node,
        ..TfRunConfig::default()
    })
}

fn source_dc_bias(spec: &SourceSpec) -> Value {
    match spec {
        SourceSpec::Dc(v) => *v,
        SourceSpec::Ac { .. } => 0.0,
        SourceSpec::DcAc { dc_value, .. } => *dc_value,
        SourceSpec::DcTransient { dc_value, .. } => *dc_value,
        SourceSpec::DcAcTransient { dc_value, .. } => *dc_value,
        SourceSpec::Pulse { v1, .. } => *v1,
        SourceSpec::Sin { offset, .. } => *offset,
        SourceSpec::Pwl { points } => points.first().map(|(_, value)| *value).unwrap_or(0.0),
        SourceSpec::PwlFile { .. } => 0.0,
        SourceSpec::Exp { v1, .. } => *v1,
        SourceSpec::Sffm { offset, .. } => *offset,
        SourceSpec::Am { offset, .. } => *offset,
        // Zero-mean noise contributes nothing to the operating point.
        SourceSpec::TrNoise { .. } => 0.0,
    }
}

fn source_with_ac_excitation(spec: &SourceSpec, magnitude: Value, phase_deg: Value) -> SourceSpec {
    SourceSpec::DcAc {
        dc_value: source_dc_bias(spec),
        ac_magnitude: magnitude,
        ac_phase: phase_deg,
    }
}

fn source_without_ac(spec: &SourceSpec) -> SourceSpec {
    source_with_ac_excitation(spec, 0.0, 0.0)
}

fn zero_all_source_ac(netlist: &mut rspice_core::Netlist) {
    for element in &mut netlist.elements {
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_without_ac(spec);
            }
            _ => {}
        }
    }
}

fn set_source_ac_excitation(
    netlist: &mut rspice_core::Netlist,
    source_name: &str,
    magnitude: Value,
    phase_deg: Value,
) -> Result<(), String> {
    let source_name = source_name.trim();
    if source_name.is_empty() {
        return Err("source name cannot be empty".to_string());
    }

    let mut matched = false;
    for element in &mut netlist.elements {
        if !element.name.eq_ignore_ascii_case(source_name) {
            continue;
        }
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_with_ac_excitation(spec, magnitude, phase_deg);
                matched = true;
            }
            _ => {
                return Err(format!(
                    "Element '{}' exists but is not an independent source",
                    source_name
                ));
            }
        }
    }

    if !matched {
        return Err(format!(
            "Independent source '{}' was not found in the netlist",
            source_name
        ));
    }

    Ok(())
}

fn unique_element_name(netlist: &rspice_core::Netlist, prefix: &str) -> String {
    if netlist
        .elements
        .iter()
        .all(|element| !element.name.eq_ignore_ascii_case(prefix))
    {
        return prefix.to_string();
    }

    for idx in 1.. {
        let candidate = format!("{}{}", prefix, idx);
        if netlist
            .elements
            .iter()
            .all(|element| !element.name.eq_ignore_ascii_case(&candidate))
        {
            return candidate;
        }
    }

    unreachable!("monotonic suffix search should always find a free element name")
}

fn inject_tf_output_test_source(
    netlist: &mut rspice_core::Netlist,
    output_node: &str,
    output_ref: Option<&str>,
) -> Result<(), String> {
    let output_node = output_node.trim();
    if output_node.is_empty() {
        return Err("TF output node must be non-empty".to_string());
    }
    let output_ref = output_ref
        .map(str::trim)
        .filter(|node| !node.is_empty())
        .unwrap_or("0");
    let test_name = unique_element_name(netlist, "__TF_ZOUT_TEST");
    netlist.elements.push(rspice_core::netlist::Element {
        name: test_name,
        kind: ElementKind::CurrentSource(SourceSpec::Ac {
            magnitude: 1.0,
            phase: 0.0,
        }),
        nodes: vec![output_node.to_string(), output_ref.to_string()],
    });
    Ok(())
}
