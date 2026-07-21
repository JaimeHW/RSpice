//! DC small-signal transfer-function (`.TF`) analysis runner.
//!
//! A transfer-function request is evaluated at the converged DC operating
//! point. It is not an AC sweep: gain, input resistance, and output
//! resistance are produced by the engine's zero-hertz linearized solves.

use super::error::ensure_not_aborted;
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, infer_primary_output_node_with_abort,
    infer_primary_source_name_with_abort, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::engine::{DampingStrategy, Engine, SimulationConfig};
use rspice_core::netlist::ElementKind;
use std::path::Path;

/// Post-solve normalization applied to the signed transfer derivative.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TfNormalization {
    /// Return the native engineering derivative (for example V/A).
    #[default]
    None,
    /// Return `(dY / dX) * (Xnom / Ynom)`, a signed dimensionless ratio.
    RelativeToNominal,
    /// Return the signed response to one engineering unit of source stimulus.
    PerSourceUnit,
}

/// Numerical policy applied after source-authored `.OPTIONS` are resolved.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TfAccuracy {
    /// Direct Newton, 25 iterations, 1e-5 absolute voltage tolerance.
    Fast,
    /// Preserve the resolved project/netlist policy.
    #[default]
    Balanced,
    /// At least 100 iterations with 1e-9/1e-5 absolute/relative tolerances.
    Accurate,
    /// At least 200 iterations and every nonlinear continuation aid enabled.
    Robust,
}

/// Physical quantity at one side of the transfer derivative.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TfQuantity {
    Voltage,
    Current,
}

/// Engineering unit carried by a retained gain value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TfGainUnit {
    Dimensionless,
    VoltsPerVolt,
    AmpsPerVolt,
    VoltsPerAmpere,
    AmpsPerAmpere,
}

/// Mathematical basis of the retained gain value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TfGainBasis {
    AbsoluteDerivative,
    NominalRelative,
    PerSourceUnit,
}

/// Typed metadata needed to display and export a signed transfer gain safely.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TfGainMetadata {
    pub input_quantity: TfQuantity,
    pub output_quantity: TfQuantity,
    pub unit: TfGainUnit,
    pub basis: TfGainBasis,
}

/// Explicit configuration for transfer-function execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TfRunConfig {
    pub input_source: String,
    pub output_expression: String,
    pub transfer_gain: bool,
    pub input_resistance: bool,
    pub output_resistance: bool,
    pub normalization: TfNormalization,
    pub accuracy: TfAccuracy,
}

impl Default for TfRunConfig {
    fn default() -> Self {
        Self {
            input_source: "VIN_DIFF".to_string(),
            output_expression: "V(afe_out)".to_string(),
            transfer_gain: true,
            input_resistance: true,
            output_resistance: true,
            normalization: TfNormalization::None,
            accuracy: TfAccuracy::Balanced,
        }
    }
}

impl TfRunConfig {
    fn validate(&self) -> Result<(), String> {
        let input_source = self.input_source.trim();
        if input_source.is_empty() {
            return Err("TF input source must be specified".to_string());
        }
        if self.input_source != input_source || input_source.chars().any(char::is_whitespace) {
            return Err(
                "TF input source must be one canonical independent-source name".to_string(),
            );
        }
        let output_expression = self.output_expression.trim();
        if output_expression.is_empty() {
            return Err("TF output expression must be specified".to_string());
        }
        if self.output_expression != output_expression {
            return Err("TF output expression must not contain surrounding whitespace".to_string());
        }
        if !self.transfer_gain && !self.input_resistance && !self.output_resistance {
            return Err(
                "TF must retain transfer gain, input resistance, or output resistance".to_string(),
            );
        }
        Ok(())
    }
}

/// Transfer-function analysis data.
#[derive(Debug, Clone, PartialEq)]
pub struct TfData {
    pub input_source: String,
    pub output_label: String,
    pub gain: Option<Value>,
    pub input_resistance: Option<Value>,
    pub output_resistance: Option<Value>,
    pub normalization: TfNormalization,
    pub gain_metadata: TfGainMetadata,
    /// Nominal source value used by relative normalization, otherwise absent.
    pub nominal_input: Option<Value>,
    /// Nominal output value used by relative normalization, otherwise absent.
    pub nominal_output: Option<Value>,
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
    let probe = TfOutputProbe::parse(&config.output_expression)?;
    ensure_not_aborted(abort)?;

    let input_quantity = input_quantity(&parsed_netlist, config.input_source.trim())?;
    let output_quantity = probe.quantity();
    let engine_config =
        apply_accuracy_policy(build_engine_config(&parsed_netlist, None), config.accuracy);
    let engine = Engine::try_new(engine_config).map_err(|error| {
        ServiceRunError::Failure(format!("Invalid TF numerical policy: {error}"))
    })?;
    let result = engine
        .run_transfer_function_with_abort(
            &parsed_netlist,
            probe.engine_target(),
            probe.reference_node(),
            probe.is_current(),
            config.input_source.trim(),
            abort,
        )
        .map_err(|error| {
            ServiceRunError::from_core("DC transfer-function analysis error", error)
        })?;
    ensure_not_aborted(abort)?;

    if !result.gain.is_finite() {
        return Err(ServiceRunError::Failure(format!(
            "TF gain from {} to {} is non-finite",
            result.input, result.output
        )));
    }

    let (gain, nominal_input, nominal_output) = if config.transfer_gain {
        normalize_gain(
            &engine,
            &parsed_netlist,
            &probe,
            config.input_source.trim(),
            result.gain,
            config.normalization,
            abort,
        )?
    } else {
        (None, None, None)
    };
    let input_resistance = retain_resistance(
        config.input_resistance,
        result.input_impedance,
        "input resistance",
    )?;
    let output_resistance = retain_resistance(
        config.output_resistance,
        result.output_impedance,
        "output resistance",
    )?;

    Ok(TfData {
        input_source: result.input,
        output_label: result.output,
        gain,
        input_resistance,
        output_resistance,
        normalization: config.normalization,
        gain_metadata: TfGainMetadata {
            input_quantity,
            output_quantity,
            unit: gain_unit(input_quantity, output_quantity, config.normalization),
            basis: gain_basis(config.normalization),
        },
        nominal_input,
        nominal_output,
    })
}

fn retain_resistance(
    enabled: bool,
    value: Value,
    quantity: &'static str,
) -> ServiceRunResult<Option<Value>> {
    if !enabled {
        return Ok(None);
    }
    if value.is_nan() {
        return Err(ServiceRunError::Failure(format!(
            "TF {quantity} is not a number"
        )));
    }
    Ok(Some(value))
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
        output_expression: format!("V({output_node})"),
        ..TfRunConfig::default()
    })
}

fn apply_accuracy_policy(mut config: SimulationConfig, accuracy: TfAccuracy) -> SimulationConfig {
    match accuracy {
        TfAccuracy::Fast => {
            config.tolerance = 1.0e-5;
            config.max_iterations = 25;
            config.convergence_config.gmin_stepping = false;
            config.convergence_config.source_stepping = false;
            config.convergence_config.pseudo_transient = false;
            config.convergence_config.arc_length = false;
            config.convergence_config.damping_strategy = DampingStrategy::None;
            config.convergence_config.voltage_reltol = 1.0e-3;
            config.convergence_config.voltage_abstol = 1.0e-5;
            config.convergence_config.current_abstol = 1.0e-10;
            config.convergence_config.residual_reltol = 1.0e-3;
        }
        TfAccuracy::Balanced => {}
        TfAccuracy::Accurate => {
            config.tolerance = config.tolerance.min(1.0e-9);
            config.max_iterations = config.max_iterations.max(100);
            config.convergence_config.voltage_reltol =
                config.convergence_config.voltage_reltol.min(1.0e-5);
            config.convergence_config.voltage_abstol =
                positive_minimum(config.convergence_config.voltage_abstol, 1.0e-9);
            config.convergence_config.current_abstol =
                positive_minimum(config.convergence_config.current_abstol, 1.0e-14);
            config.convergence_config.residual_reltol =
                config.convergence_config.residual_reltol.min(1.0e-5);
        }
        TfAccuracy::Robust => {
            config.max_iterations = config.max_iterations.max(200);
            config.convergence_config.gmin_stepping = true;
            config.convergence_config.source_stepping = true;
            config.convergence_config.pseudo_transient = true;
            config.convergence_config.arc_length = true;
            config.convergence_config.damping_strategy = DampingStrategy::Combined;
        }
    }
    config
}

fn positive_minimum(current: Value, ceiling: Value) -> Value {
    if current > 0.0 {
        current.min(ceiling)
    } else {
        ceiling
    }
}

fn input_quantity(
    netlist: &rspice_core::Netlist,
    input_source: &str,
) -> ServiceRunResult<TfQuantity> {
    let element = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(input_source))
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "TF input source '{input_source}' does not exist in the resolved netlist"
            ))
        })?;
    match &element.kind {
        ElementKind::VoltageSource(_) => Ok(TfQuantity::Voltage),
        ElementKind::CurrentSource(_) => Ok(TfQuantity::Current),
        _ => Err(ServiceRunError::Failure(format!(
            "TF input '{input_source}' is not an independent voltage or current source"
        ))),
    }
}

fn gain_basis(normalization: TfNormalization) -> TfGainBasis {
    match normalization {
        TfNormalization::None => TfGainBasis::AbsoluteDerivative,
        TfNormalization::RelativeToNominal => TfGainBasis::NominalRelative,
        TfNormalization::PerSourceUnit => TfGainBasis::PerSourceUnit,
    }
}

fn gain_unit(input: TfQuantity, output: TfQuantity, normalization: TfNormalization) -> TfGainUnit {
    if normalization == TfNormalization::RelativeToNominal {
        return TfGainUnit::Dimensionless;
    }
    match (input, output) {
        (TfQuantity::Voltage, TfQuantity::Voltage) => TfGainUnit::VoltsPerVolt,
        (TfQuantity::Voltage, TfQuantity::Current) => TfGainUnit::AmpsPerVolt,
        (TfQuantity::Current, TfQuantity::Voltage) => TfGainUnit::VoltsPerAmpere,
        (TfQuantity::Current, TfQuantity::Current) => TfGainUnit::AmpsPerAmpere,
    }
}

fn normalize_gain(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    probe: &TfOutputProbe,
    input_source: &str,
    absolute_gain: Value,
    normalization: TfNormalization,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<(Option<Value>, Option<Value>, Option<Value>)> {
    if normalization != TfNormalization::RelativeToNominal {
        return Ok((Some(absolute_gain), None, None));
    }

    ensure_not_aborted(abort)?;
    let operating_point = engine
        .run_dc_op_with_abort(netlist, abort)
        .map_err(|error| ServiceRunError::from_core("TF nominal operating-point error", error))?;
    let nominal_output = probe.nominal_value(&operating_point)?;
    let nominal_input = nominal_source_value(engine, netlist, input_source, abort)?;

    validate_nonzero_finite_nominal(nominal_input, "input source", input_source)?;
    validate_nonzero_finite_nominal(nominal_output, "output", &probe.canonical_expression())?;
    let normalized = absolute_gain * nominal_input / nominal_output;
    if !normalized.is_finite() {
        return Err(ServiceRunError::Failure(format!(
            "TF relative gain from {input_source} to {} is non-finite",
            probe.canonical_expression()
        )));
    }
    Ok((Some(normalized), Some(nominal_input), Some(nominal_output)))
}

fn validate_nonzero_finite_nominal(value: Value, role: &str, label: &str) -> ServiceRunResult<()> {
    if !value.is_finite() {
        return Err(ServiceRunError::Failure(format!(
            "TF relative normalization requires a finite nominal {role} '{label}', got {value}"
        )));
    }
    if value == 0.0 {
        return Err(ServiceRunError::Failure(format!(
            "TF relative normalization is undefined because nominal {role} '{label}' is zero"
        )));
    }
    Ok(())
}

/// Evaluate the authored DC value through rspice-core itself. Converting the
/// selected source to an isolated voltage source makes every supported source
/// form (including file-backed PWL and PAT) observable without duplicating the
/// core's waveform-at-t=0 rules in this service.
fn nominal_source_value(
    engine: &Engine,
    netlist: &rspice_core::Netlist,
    input_source: &str,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Value> {
    const NOMINAL_NODE: &str = "TF_NOMINAL_SOURCE_INTERNAL";
    let mut source = netlist
        .elements
        .iter()
        .find(|element| element.name.eq_ignore_ascii_case(input_source))
        .cloned()
        .ok_or_else(|| {
            ServiceRunError::Failure(format!(
                "TF input source '{input_source}' does not exist in the resolved netlist"
            ))
        })?;
    source.kind = match source.kind {
        ElementKind::VoltageSource(spec) | ElementKind::CurrentSource(spec) => {
            ElementKind::VoltageSource(spec)
        }
        _ => {
            return Err(ServiceRunError::Failure(format!(
                "TF input '{input_source}' is not an independent voltage or current source"
            )));
        }
    };
    source.nodes = vec![NOMINAL_NODE.to_string(), "0".to_string()];

    let mut source_deck = netlist.clone();
    source_deck.elements.clear();
    source_deck.elements.push(source);
    source_deck.analyses.clear();
    source_deck.fft_analyses.clear();
    let result = engine
        .run_dc_op_with_abort(&source_deck, abort)
        .map_err(|error| {
            ServiceRunError::from_core("TF nominal input-source evaluation error", error)
        })?;
    result.try_voltage_named(NOMINAL_NODE).ok_or_else(|| {
        ServiceRunError::Failure(
            "TF nominal input-source evaluation did not retain its internal probe".to_string(),
        )
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum TfOutputProbe {
    Voltage {
        positive: String,
        reference: Option<String>,
    },
    Current {
        element: String,
    },
}

impl TfOutputProbe {
    fn parse(output: &str) -> ServiceRunResult<Self> {
        let output = output.trim();
        if output.is_empty() {
            return Err(ServiceRunError::Failure(
                "TF output expression must be specified".to_string(),
            ));
        }
        let Some(open) = output.find('(') else {
            return Err(ServiceRunError::Failure(format!(
                "Invalid TF output expression '{output}'; expected V(node), V(node,ref), or I(element)"
            )));
        };
        if !output.ends_with(')') || output[open + 1..output.len() - 1].contains(['(', ')']) {
            return Err(ServiceRunError::Failure(format!(
                "Invalid TF output expression '{output}'; expected V(node), V(node,ref), or I(element)"
            )));
        }
        let function = output[..open].trim();
        let arguments = output[open + 1..output.len() - 1]
            .split(',')
            .map(str::trim)
            .collect::<Vec<_>>();
        if arguments.iter().any(|argument| argument.is_empty()) {
            return Err(ServiceRunError::Failure(format!(
                "Invalid TF output expression '{output}'; probe arguments must not be empty"
            )));
        }

        if function.eq_ignore_ascii_case("V") {
            let (positive, embedded_reference) = match arguments.as_slice() {
                [positive] => (*positive, None),
                [positive, reference] => (*positive, Some(*reference)),
                _ => {
                    return Err(ServiceRunError::Failure(format!(
                        "Invalid TF voltage output '{output}'; expected V(node) or V(node,ref)"
                    )));
                }
            };
            return Ok(Self::Voltage {
                positive: positive.to_string(),
                reference: embedded_reference.map(str::to_string),
            });
        }

        if function.eq_ignore_ascii_case("I") {
            let [element] = arguments.as_slice() else {
                return Err(ServiceRunError::Failure(format!(
                    "Invalid TF current output '{output}'; expected I(element)"
                )));
            };
            return Ok(Self::Current {
                element: (*element).to_string(),
            });
        }

        Err(ServiceRunError::Failure(format!(
            "Unsupported TF output expression '{output}'; expected V(node), V(node,ref), or I(element)"
        )))
    }

    fn engine_target(&self) -> &str {
        match self {
            Self::Voltage { positive, .. } => positive,
            Self::Current { element } => element,
        }
    }

    fn reference_node(&self) -> Option<&str> {
        match self {
            Self::Voltage { reference, .. } => reference.as_deref(),
            Self::Current { .. } => None,
        }
    }

    fn is_current(&self) -> bool {
        matches!(self, Self::Current { .. })
    }

    fn quantity(&self) -> TfQuantity {
        if self.is_current() {
            TfQuantity::Current
        } else {
            TfQuantity::Voltage
        }
    }

    fn canonical_expression(&self) -> String {
        match self {
            Self::Voltage {
                positive,
                reference: Some(reference),
            } => format!("V({positive},{reference})"),
            Self::Voltage {
                positive,
                reference: None,
            } => format!("V({positive})"),
            Self::Current { element } => format!("I({element})"),
        }
    }

    fn nominal_value(
        &self,
        operating_point: &rspice_core::SimulationResult,
    ) -> ServiceRunResult<Value> {
        match self {
            Self::Voltage {
                positive,
                reference,
            } => {
                let positive_voltage =
                    operating_point.try_voltage_named(positive).ok_or_else(|| {
                        ServiceRunError::Failure(format!(
                            "TF output node '{positive}' is absent from the nominal operating point"
                        ))
                    })?;
                let reference_voltage = match reference {
                    Some(reference) => {
                        operating_point
                            .try_voltage_named(reference)
                            .ok_or_else(|| {
                                ServiceRunError::Failure(format!(
                                    "TF output reference node '{reference}' is absent from the nominal operating point"
                                ))
                            })?
                    }
                    None => 0.0,
                };
                Ok(positive_voltage - reference_voltage)
            }
            Self::Current { element } => {
                operating_point
                    .branch_current_named(element)
                    .ok_or_else(|| {
                        ServiceRunError::Failure(format!(
                            "TF output element '{element}' has no nominal branch current"
                        ))
                    })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::abort_signal::{CountingAbort, ImmediateAbort};

    const DIVIDER: &str = "\
DC transfer divider
VIN in 0 1
R1 in out 1k
R2 out 0 2k
.end
";

    fn config(output: &str, input: &str) -> TfRunConfig {
        TfRunConfig {
            output_expression: output.to_string(),
            input_source: input.to_string(),
            ..TfRunConfig::default()
        }
    }

    fn assert_close(actual: Value, expected: Value, label: &str) {
        let tolerance = 1.0e-9 * expected.abs().max(1.0);
        assert!(
            (actual - expected).abs() <= tolerance,
            "{label}: expected {expected:.16e}, got {actual:.16e} (tolerance {tolerance:.3e})"
        );
    }

    #[test]
    fn tf_service_preserves_typed_entry_abort() {
        let result = run_tf_analysis_with_config_and_abort(
            "not a netlist",
            &config("V(out)", "VIN"),
            &ImmediateAbort,
        );

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn tf_service_honors_abort_after_entry() {
        let abort = CountingAbort::new(4);
        let result =
            run_tf_analysis_with_config_and_abort(DIVIDER, &config("V(out)", "VIN"), &abort);
        assert!(matches!(result, Err(ServiceRunError::Aborted)));
        assert!(abort.count() > 4);
    }

    #[test]
    fn tf_service_returns_exact_dc_gain_and_resistances() {
        let cfg = config("V(out)", "VIN");
        let data = run_tf_analysis_with_config(DIVIDER, &cfg).expect("TF succeeds");

        assert_close(data.gain.expect("gain"), 2.0 / 3.0, "gain");
        assert_close(data.input_resistance.expect("Rin"), 3000.0, "Rin");
        assert_close(data.output_resistance.expect("Rout"), 2000.0 / 3.0, "Rout");
        assert_eq!(data.gain_metadata.input_quantity, TfQuantity::Voltage);
        assert_eq!(data.gain_metadata.output_quantity, TfQuantity::Voltage);
        assert_eq!(data.gain_metadata.unit, TfGainUnit::VoltsPerVolt);
        assert_eq!(data.gain_metadata.basis, TfGainBasis::AbsoluteDerivative);
        assert_eq!(data.nominal_input, None);
        assert_eq!(data.nominal_output, None);
    }

    #[test]
    fn tf_service_retention_flags_gate_each_scalar() {
        let mut cfg = config("V(out)", "VIN");
        cfg.transfer_gain = false;
        cfg.output_resistance = false;

        let data = run_tf_analysis_with_config(DIVIDER, &cfg).expect("TF succeeds");

        assert_eq!(data.gain, None);
        assert!(data.input_resistance.is_some());
        assert_eq!(data.output_resistance, None);
        assert_eq!(data.nominal_input, None);
        assert_eq!(data.nominal_output, None);
    }

    #[test]
    fn resistance_retention_accepts_infinity_but_rejects_nan() {
        assert_eq!(
            retain_resistance(true, Value::INFINITY, "input resistance").unwrap(),
            Some(Value::INFINITY)
        );
        assert!(retain_resistance(true, Value::NAN, "input resistance").is_err());
        assert_eq!(
            retain_resistance(false, Value::NAN, "input resistance").unwrap(),
            None
        );
    }

    #[test]
    fn tf_service_rejects_a_request_that_retains_nothing() {
        let mut cfg = config("V(out)", "VIN");
        cfg.transfer_gain = false;
        cfg.input_resistance = false;
        cfg.output_resistance = false;

        let error = run_tf_analysis_with_config(DIVIDER, &cfg).expect_err("empty result rejected");
        assert!(error.contains("must retain"));
    }

    #[test]
    fn tf_service_supports_current_input_and_differential_voltage_output() {
        let deck = "\
current input and differential output
IIN 0 in 0
R1 in out 1k
R2 out 0 2k
.end
";
        let data = run_tf_analysis_with_config(deck, &config("V(in,out)", "IIN"))
            .expect("differential TF succeeds");

        assert_close(data.gain.expect("gain"), 1000.0, "gain");
        assert_close(data.input_resistance.expect("Rin"), 3000.0, "Rin");
        assert_close(data.output_resistance.expect("Rout"), 1000.0, "Rout");
        assert_eq!(data.output_label, "V(in,out)");
        assert_eq!(data.gain_metadata.input_quantity, TfQuantity::Current);
        assert_eq!(data.gain_metadata.unit, TfGainUnit::VoltsPerAmpere);
    }

    #[test]
    fn tf_service_supports_authenticated_branch_current_output() {
        let deck = "\
branch current output
VIN in 0 1
R1 in mid 1k
VMEAS mid out 0
R2 out 0 2k
.end
";
        let data = run_tf_analysis_with_config(deck, &config("I(VMEAS)", "VIN"))
            .expect("branch-current TF succeeds");

        assert_close(data.gain.expect("gain"), 1.0 / 3000.0, "gain");
        assert_close(data.input_resistance.expect("Rin"), 3000.0, "Rin");
        assert_eq!(data.output_resistance, Some(1.0e20));
        assert_eq!(data.output_label, "I(VMEAS)");
        assert_eq!(data.gain_metadata.output_quantity, TfQuantity::Current);
        assert_eq!(data.gain_metadata.unit, TfGainUnit::AmpsPerVolt);
    }

    #[test]
    fn tf_output_expression_parser_rejects_ambiguous_or_invented_semantics() {
        assert!(TfOutputProbe::parse("V(out,in,other)").is_err());
        assert!(TfOutputProbe::parse("I(V1,V2)").is_err());
        assert!(TfOutputProbe::parse("P(R1)").is_err());
        let mut padded = config("V(out)", " VIN");
        assert!(run_tf_analysis_with_config(DIVIDER, &padded).is_err());
        padded.input_source = "VIN".to_owned();
        padded.output_expression = " V(out)".to_owned();
        assert!(run_tf_analysis_with_config(DIVIDER, &padded).is_err());
    }

    #[test]
    fn relative_normalization_retains_the_exact_nominal_values_used() {
        let mut cfg = config("V(out)", "VIN");
        cfg.normalization = TfNormalization::RelativeToNominal;

        let data = run_tf_analysis_with_config(DIVIDER, &cfg).expect("relative TF succeeds");

        assert_close(data.gain.expect("gain"), 1.0, "relative gain");
        assert_eq!(data.nominal_input, Some(1.0));
        assert_close(
            data.nominal_output.expect("Ynom"),
            2.0 / 3.0,
            "nominal output",
        );
        assert_eq!(data.gain_metadata.unit, TfGainUnit::Dimensionless);
        assert_eq!(data.gain_metadata.basis, TfGainBasis::NominalRelative);
    }

    #[test]
    fn relative_normalization_rejects_a_zero_nominal_source() {
        let deck = "\
zero nominal source
VIN in 0 0
R1 in out 1k
R2 out 0 2k
.end
";
        let mut cfg = config("V(out)", "VIN");
        cfg.normalization = TfNormalization::RelativeToNominal;

        let error = run_tf_analysis_with_config(deck, &cfg).expect_err("zero Xnom rejected");
        assert!(error.contains("nominal input source 'VIN' is zero"));
    }

    #[test]
    fn relative_normalization_uses_the_authored_current_source_nominal() {
        let deck = "\
current source normalization
IIN 0 in 1m
R1 in 0 1k
.end
";
        let mut cfg = config("V(in)", "IIN");
        cfg.normalization = TfNormalization::RelativeToNominal;

        let data = run_tf_analysis_with_config(deck, &cfg).expect("relative TF succeeds");

        assert_close(data.gain.expect("gain"), 1.0, "relative gain");
        assert!((data.nominal_input.expect("Xnom") - 1.0e-3).abs() < 1e-15);
        assert_close(data.nominal_output.expect("Ynom"), 1.0, "nominal output");
        assert_eq!(data.gain_metadata.input_quantity, TfQuantity::Current);
    }

    #[test]
    fn relative_normalization_rejects_a_zero_nominal_output() {
        let deck = "\
zero nominal output
VIN in 0 1
R1 in 0 1k
VZERO out 0 0
.end
";
        let mut cfg = config("V(out)", "VIN");
        cfg.normalization = TfNormalization::RelativeToNominal;

        let error = run_tf_analysis_with_config(deck, &cfg).expect_err("zero Ynom rejected");
        assert!(error.contains("nominal output 'V(out)' is zero"));
    }

    #[test]
    fn per_source_unit_retains_the_signed_derivative_with_explicit_basis() {
        let mut cfg = config("V(0,out)", "VIN");
        cfg.normalization = TfNormalization::PerSourceUnit;

        let data = run_tf_analysis_with_config(DIVIDER, &cfg).expect("per-unit TF succeeds");

        assert_close(data.gain.expect("gain"), -2.0 / 3.0, "per-unit gain");
        assert_eq!(data.gain_metadata.unit, TfGainUnit::VoltsPerVolt);
        assert_eq!(data.gain_metadata.basis, TfGainBasis::PerSourceUnit);
    }

    #[test]
    fn accuracy_policies_have_stable_documented_solver_contracts() {
        let mut base = SimulationConfig::default();
        base.max_iterations = 63;

        let fast = apply_accuracy_policy(base.clone(), TfAccuracy::Fast);
        assert_eq!(fast.max_iterations, 25);
        assert_eq!(fast.tolerance, 1.0e-5);
        assert!(!fast.convergence_config.gmin_stepping);
        assert_eq!(
            fast.convergence_config.damping_strategy,
            DampingStrategy::None
        );

        let balanced = apply_accuracy_policy(base.clone(), TfAccuracy::Balanced);
        assert_eq!(balanced.max_iterations, 63);
        assert_eq!(balanced.tolerance, base.tolerance);

        let accurate = apply_accuracy_policy(base.clone(), TfAccuracy::Accurate);
        assert_eq!(accurate.max_iterations, 100);
        assert!(accurate.tolerance <= 1.0e-9);
        assert!(accurate.convergence_config.voltage_reltol <= 1.0e-5);

        let robust = apply_accuracy_policy(base, TfAccuracy::Robust);
        assert_eq!(robust.max_iterations, 200);
        assert!(robust.convergence_config.arc_length);
        assert_eq!(
            robust.convergence_config.damping_strategy,
            DampingStrategy::Combined
        );
    }
}
