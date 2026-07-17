//! Transfer-function analysis runner.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, build_voltage_output_expr,
    generate_freq_points_with_abort, infer_primary_output_node_with_abort,
    infer_primary_source_name_with_abort, parse_runner_netlist_with_abort,
};
use crate::output_spec::{ac_output_value, parse_output_spec};
use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
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
    run_tf_analysis_with_config_and_abort(netlist_text, config, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run transfer-function analysis with explicit configuration and
/// cooperative cancellation.
pub fn run_tf_analysis_with_config_and_abort(
    netlist_text: &str,
    config: &TfRunConfig,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TfData> {
    run_tf_analysis_with_config_and_source_path_and_abort(netlist_text, config, None, abort)
}

/// Run transfer-function analysis with explicit configuration and a source path
/// used to resolve relative includes and model file references.
pub fn run_tf_analysis_with_config_and_source_path(
    netlist_text: &str,
    config: &TfRunConfig,
    source_path: Option<&Path>,
) -> Result<TfData, String> {
    run_tf_analysis_with_config_and_source_path_and_abort(
        netlist_text,
        config,
        source_path,
        &NoAbort,
    )
    .map_err(|error| error.to_string())
}

/// Run transfer-function analysis with source-path resolution and
/// cooperative cancellation through parsing, solving, and result conversion.
pub fn run_tf_analysis_with_config_and_source_path_and_abort(
    netlist_text: &str,
    config: &TfRunConfig,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TfData> {
    ensure_not_aborted(abort)?;
    config.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let parsed_netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;

    // Build a baseline netlist with all AC source magnitudes forced to zero.
    // We then explicitly excite only the requested input source to keep the
    // transfer denominator deterministic and independent of unrelated sources.
    let mut tf_netlist = parsed_netlist.clone();
    zero_all_source_ac(&mut tf_netlist, abort)?;
    set_source_ac_excitation(&mut tf_netlist, &config.input_source, 1.0, 0.0, abort)?;

    let engine = Engine::new(build_engine_config(&tf_netlist, None));
    let dc_result = engine
        .run_dc_op_with_abort(&tf_netlist, abort)
        .map_err(|error| ServiceRunError::from_core("DC OP error (required for TF)", error))?;
    ensure_not_aborted(abort)?;
    let circuit = engine.build_circuit(&tf_netlist).map_err(|error| {
        ServiceRunError::Failure(format!("Circuit build error (required for TF): {error}"))
    })?;
    ensure_not_aborted(abort)?;

    let output_expr =
        build_voltage_output_expr(config.output_node.trim(), config.output_ref.as_deref());
    let output_spec =
        parse_output_spec(&output_expr, &dc_result.node_names, &circuit).ok_or_else(|| {
            ServiceRunError::Failure(format!("TF output '{output_expr}' could not be resolved"))
        })?;

    let frequencies = generate_freq_points_with_abort(
        config.start_freq,
        config.stop_freq,
        config.points_per_unit,
        config.sweep.keyword(),
        abort,
    )?;

    let ac_results = engine
        .run_ac_with_abort(&tf_netlist, &frequencies, abort)
        .map_err(|error| ServiceRunError::from_core("TF AC analysis error", error))?;
    if ac_results.len() != frequencies.len() {
        return Err(ServiceRunError::Failure(format!(
            "TF AC analysis returned {} points for {} requested frequencies",
            ac_results.len(),
            frequencies.len()
        )));
    }

    let mut transfer = Vec::with_capacity(ac_results.len());
    let mut magnitude_db = Vec::with_capacity(ac_results.len());
    let mut phase_deg = Vec::with_capacity(ac_results.len());
    for (index, point) in ac_results.iter().enumerate() {
        poll_periodically(abort, index)?;
        let value = ac_output_value(point, &output_spec).map_err(|error| {
            ServiceRunError::Failure(format!("TF output extraction error: {error}"))
        })?;
        magnitude_db.push(20.0 * value.norm().max(1e-30).log10());
        phase_deg.push(value.arg().to_degrees());
        transfer.push(value);
    }

    let input_impedance = if config.input_impedance {
        let branch_ordinal = circuit
            .get_branch_by_name(config.input_source.trim())
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "TF input source '{}' does not expose an AC branch current; cannot compute Zin",
                    config.input_source
                ))
            })? as usize;
        let branch_idx = branch_ordinal.saturating_sub(1);
        let mut impedance = Vec::with_capacity(ac_results.len());
        for (index, point) in ac_results.iter().enumerate() {
            poll_periodically(abort, index)?;
            let iin = point.currents.get(branch_idx).copied().ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "TF input source '{}' branch index {} is unavailable in AC result",
                    config.input_source, branch_idx
                ))
            })?;
            impedance.push(if iin.norm() <= 1e-30 {
                Complex64::new(f64::INFINITY, 0.0)
            } else {
                Complex64::new(1.0, 0.0) / iin
            });
        }
        Some(impedance)
    } else {
        None
    };

    let output_impedance = if config.output_impedance {
        let mut zout_netlist = parsed_netlist.clone();
        zero_all_source_ac(&mut zout_netlist, abort)?;
        inject_tf_output_test_source(
            &mut zout_netlist,
            config.output_node.trim(),
            config.output_ref.as_deref(),
            abort,
        )?;

        let zout_engine = Engine::new(build_engine_config(&zout_netlist, None));
        let zout_dc = zout_engine
            .run_dc_op_with_abort(&zout_netlist, abort)
            .map_err(|error| {
                ServiceRunError::from_core("DC OP error (required for TF Zout)", error)
            })?;
        ensure_not_aborted(abort)?;
        let zout_circuit = zout_engine.build_circuit(&zout_netlist).map_err(|error| {
            ServiceRunError::Failure(format!(
                "Circuit build error (required for TF Zout): {error}"
            ))
        })?;
        ensure_not_aborted(abort)?;
        let zout_spec = parse_output_spec(&output_expr, &zout_dc.node_names, &zout_circuit)
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "TF output '{output_expr}' could not be resolved for Zout"
                ))
            })?;

        let zout_points = zout_engine
            .run_ac_with_abort(&zout_netlist, &frequencies, abort)
            .map_err(|error| {
                ServiceRunError::from_core("TF output-impedance AC analysis error", error)
            })?;
        let mut impedance = Vec::with_capacity(zout_points.len());
        for (index, point) in zout_points.iter().enumerate() {
            poll_periodically(abort, index)?;
            impedance.push(ac_output_value(point, &zout_spec).map_err(|error| {
                ServiceRunError::Failure(format!("TF output-impedance extraction error: {error}"))
            })?);
        }
        Some(impedance)
    } else {
        None
    };

    let group_delay = if config.group_delay && frequencies.len() >= 2 {
        use std::f64::consts::PI;
        let mut points = Vec::with_capacity(frequencies.len().saturating_sub(1));
        let mut prev_phase = transfer[0].arg();
        for idx in 1..frequencies.len() {
            poll_periodically(abort, idx)?;
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

    ensure_not_aborted(abort)?;
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
    run_tf_analysis_with_abort(netlist_text, &NoAbort).map_err(|error| error.to_string())
}

/// Run inferred/default transfer-function analysis with cancellation.
pub fn run_tf_analysis_with_abort(
    netlist_text: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TfData> {
    run_tf_analysis_with_source_path_and_abort(netlist_text, None, abort)
}

/// Run transfer-function analysis using inferred/default settings and a source
/// path used to resolve relative includes and model file references.
pub fn run_tf_analysis_with_source_path(
    netlist_text: &str,
    source_path: Option<&Path>,
) -> Result<TfData, String> {
    run_tf_analysis_with_source_path_and_abort(netlist_text, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run inferred/default transfer-function analysis with source-path
/// resolution and cancellation.
pub fn run_tf_analysis_with_source_path_and_abort(
    netlist_text: &str,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TfData> {
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));
    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for TF defaults)", error)
        })?;

    let cfg = infer_tf_run_config(&netlist, &dc_result.node_names, abort)?;
    run_tf_analysis_with_config_and_source_path_and_abort(netlist_text, &cfg, source_path, abort)
}

fn infer_tf_run_config(
    netlist: &rspice_core::Netlist,
    node_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<TfRunConfig> {
    let input_source = infer_primary_source_name_with_abort(netlist, abort)?.ok_or_else(|| {
        ServiceRunError::Failure(
            "TF requires at least one independent source in the netlist".to_string(),
        )
    })?;
    let output_node =
        infer_primary_output_node_with_abort(node_names, abort)?.ok_or_else(|| {
            ServiceRunError::Failure(
                "TF could not infer an output node; ensure at least one non-ground node exists"
                    .to_string(),
            )
        })?;
    Ok(TfRunConfig {
        input_source,
        output_node,
        ..TfRunConfig::default()
    })
}

fn source_with_ac_excitation(spec: &SourceSpec, magnitude: Value, phase_deg: Value) -> SourceSpec {
    // Preserve DC, transient, RF-port, and distortion annotations. SourceSpec
    // stores AC phase in radians while the UI contract is degrees.
    spec.clone().with_ac(magnitude, phase_deg.to_radians())
}

fn source_without_ac(spec: &SourceSpec) -> SourceSpec {
    source_with_ac_excitation(spec, 0.0, 0.0)
}

fn zero_all_source_ac(
    netlist: &mut rspice_core::Netlist,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    for (index, element) in netlist.elements.iter_mut().enumerate() {
        poll_periodically(abort, index)?;
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_without_ac(spec);
            }
            _ => {}
        }
    }
    ensure_not_aborted(abort)
}

fn set_source_ac_excitation(
    netlist: &mut rspice_core::Netlist,
    source_name: &str,
    magnitude: Value,
    phase_deg: Value,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    let source_name = source_name.trim();
    if source_name.is_empty() {
        return Err(ServiceRunError::Failure(
            "source name cannot be empty".to_string(),
        ));
    }

    let mut matched = false;
    for (index, element) in netlist.elements.iter_mut().enumerate() {
        poll_periodically(abort, index)?;
        if !element.name.eq_ignore_ascii_case(source_name) {
            continue;
        }
        match &mut element.kind {
            ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
                *spec = source_with_ac_excitation(spec, magnitude, phase_deg);
                matched = true;
            }
            _ => {
                return Err(ServiceRunError::Failure(format!(
                    "Element '{}' exists but is not an independent source",
                    source_name
                )));
            }
        }
    }

    if !matched {
        return Err(ServiceRunError::Failure(format!(
            "Independent source '{}' was not found in the netlist",
            source_name
        )));
    }

    Ok(())
}

fn unique_element_name(
    netlist: &rspice_core::Netlist,
    prefix: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<String> {
    let is_available = |candidate: &str| -> ServiceRunResult<bool> {
        for (index, element) in netlist.elements.iter().enumerate() {
            poll_periodically(abort, index)?;
            if element.name.eq_ignore_ascii_case(candidate) {
                return Ok(false);
            }
        }
        Ok(true)
    };
    if is_available(prefix)? {
        return Ok(prefix.to_string());
    }

    for idx in 1.. {
        ensure_not_aborted(abort)?;
        let candidate = format!("{}{}", prefix, idx);
        if is_available(&candidate)? {
            return Ok(candidate);
        }
    }

    unreachable!("monotonic suffix search should always find a free element name")
}

fn inject_tf_output_test_source(
    netlist: &mut rspice_core::Netlist,
    output_node: &str,
    output_ref: Option<&str>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    ensure_not_aborted(abort)?;
    let output_node = output_node.trim();
    if output_node.is_empty() {
        return Err(ServiceRunError::Failure(
            "TF output node must be non-empty".to_string(),
        ));
    }
    let output_ref = output_ref
        .map(str::trim)
        .filter(|node| !node.is_empty())
        .unwrap_or("0");
    let test_name = unique_element_name(netlist, "__TF_ZOUT_TEST", abort)?;
    netlist.elements.push(rspice_core::netlist::Element {
        name: test_name,
        kind: ElementKind::CurrentSource(SourceSpec::Ac {
            magnitude: 1.0,
            phase: 0.0,
        }),
        nodes: vec![output_node.to_string(), output_ref.to_string()],
        provenance: rspice_core::netlist::ElementProvenance::Authored,
    });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};
    use rspice_core::netlist::SourceRfPort;

    #[test]
    fn tf_service_preserves_typed_entry_abort() {
        let mut config = TfRunConfig::default();
        config.start_freq = 0.0;

        let result =
            run_tf_analysis_with_config_and_abort("not a netlist", &config, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn tf_source_preparation_honors_in_loop_abort() {
        let mut deck = String::from("many independent sources\n");
        for index in 0..130 {
            deck.push_str(&format!("V{index} n{index} 0 0 AC 1\n"));
        }
        deck.push_str(".end\n");
        let mut netlist = rspice_core::Netlist::parse(&deck).expect("test deck should parse");
        let abort = CountingAbort::new(1);

        let result = zero_all_source_ac(&mut netlist, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 1);
    }

    #[test]
    fn rf_port_sources_preserve_annotations_and_convert_phase_to_radians() {
        let spec = SourceSpec::RfPort {
            inner: Box::new(SourceSpec::DcAc {
                dc_value: 2.5,
                ac_magnitude: 1.0,
                ac_phase: 0.0,
            }),
            port: SourceRfPort {
                portnum: 1,
                z0: 50.0,
                power: None,
                frequency: None,
                phase: None,
            },
        };

        let excited = source_with_ac_excitation(&spec, 3.0, 90.0);
        let SourceSpec::RfPort { inner, port } = excited else {
            panic!("RF-port wrapper must be preserved");
        };
        assert_eq!(port.portnum, 1);
        let SourceSpec::DcAc {
            dc_value,
            ac_magnitude,
            ac_phase,
        } = *inner
        else {
            panic!("inner DC+AC source must be preserved");
        };
        assert_eq!(dc_value, 2.5);
        assert_eq!(ac_magnitude, 3.0);
        assert!((ac_phase - std::f64::consts::FRAC_PI_2).abs() < 1e-15);
    }
}
