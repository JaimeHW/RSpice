//! Pole-zero analysis runner.

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    ServiceRunError, ServiceRunResult, build_engine_config, parse_runner_netlist_with_abort,
};
use rspice_core::Value;
use rspice_core::abort_signal::{AbortSignal, NoAbort};
use rspice_core::analysis::pole_zero::Complex as PzComplex;
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

/// Request parameters for a pole-zero analysis run.
#[derive(Debug, Clone, Copy)]
pub struct PoleZeroRunSpec<'a> {
    pub input_node: &'a str,
    pub input_ref: &'a str,
    pub output_node: &'a str,
    pub output_ref: &'a str,
    pub transfer_type: &'a str,
    pub analysis_type: &'a str,
}

impl<'a> PoleZeroRunSpec<'a> {
    pub const fn new(
        input_node: &'a str,
        input_ref: &'a str,
        output_node: &'a str,
        output_ref: &'a str,
        transfer_type: &'a str,
        analysis_type: &'a str,
    ) -> Self {
        Self {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type,
            analysis_type,
        }
    }

    fn validate(self) -> Result<ValidatedPoleZeroRunSpec<'a>, String> {
        let input_node = self.input_node.trim();
        let input_ref = self.input_ref.trim();
        let output_node = self.output_node.trim();
        let output_ref = self.output_ref.trim();

        if input_node.is_empty() {
            return Err("Pole-zero input_node is required".to_string());
        }
        if output_node.is_empty() {
            return Err("Pole-zero output_node is required".to_string());
        }

        Ok(ValidatedPoleZeroRunSpec {
            input_node,
            input_ref,
            output_node,
            output_ref,
            transfer_type: PoleZeroTransferType::parse(self.transfer_type)?,
            analysis_type: PoleZeroAnalysisType::parse(self.analysis_type)?,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PoleZeroTransferType {
    Voltage,
    Current,
}

impl PoleZeroTransferType {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "VOL" => Ok(Self::Voltage),
            "CUR" => Ok(Self::Current),
            _ => Err("Pole-zero transfer_type must be VOL or CUR".to_string()),
        }
    }

    fn input_is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PoleZeroAnalysisType {
    PoleZero,
    PolesOnly,
    ZerosOnly,
}

impl PoleZeroAnalysisType {
    fn parse(raw: &str) -> Result<Self, String> {
        match raw.trim().to_ascii_uppercase().as_str() {
            "PZ" => Ok(Self::PoleZero),
            "POL" => Ok(Self::PolesOnly),
            "ZER" => Ok(Self::ZerosOnly),
            _ => Err("Pole-zero analysis_type must be PZ, POL, or ZER".to_string()),
        }
    }

    fn compute_flags(self) -> (bool, bool) {
        match self {
            Self::PoleZero => (true, true),
            Self::PolesOnly => (true, false),
            Self::ZerosOnly => (false, true),
        }
    }

    fn filter_results(self, poles: &mut Vec<(Value, Value)>, zeros: &mut Vec<(Value, Value)>) {
        match self {
            Self::PolesOnly => zeros.clear(),
            Self::ZerosOnly => poles.clear(),
            Self::PoleZero => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ValidatedPoleZeroRunSpec<'a> {
    input_node: &'a str,
    input_ref: &'a str,
    output_node: &'a str,
    output_ref: &'a str,
    transfer_type: PoleZeroTransferType,
    analysis_type: PoleZeroAnalysisType,
}

/// Run pole-zero analysis.
pub fn run_pole_zero_analysis(
    netlist_text: &str,
    spec: PoleZeroRunSpec<'_>,
) -> Result<PoleZeroData, String> {
    run_pole_zero_analysis_with_abort(netlist_text, spec, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run pole-zero analysis with cooperative cancellation.
pub fn run_pole_zero_analysis_with_abort(
    netlist_text: &str,
    spec: PoleZeroRunSpec<'_>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PoleZeroData> {
    run_pole_zero_analysis_with_source_path_and_abort(netlist_text, spec, None, abort)
}

/// Run pole-zero analysis with a source path used to resolve relative includes
/// and model file references.
pub fn run_pole_zero_analysis_with_source_path(
    netlist_text: &str,
    spec: PoleZeroRunSpec<'_>,
    source_path: Option<&Path>,
) -> Result<PoleZeroData, String> {
    run_pole_zero_analysis_with_source_path_and_abort(netlist_text, spec, source_path, &NoAbort)
        .map_err(|error| error.to_string())
}

/// Run pole-zero analysis with source-path resolution and cooperative
/// cancellation through parsing, operating-point setup, solving, and result
/// conversion.
pub fn run_pole_zero_analysis_with_source_path_and_abort(
    netlist_text: &str,
    spec: PoleZeroRunSpec<'_>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PoleZeroData> {
    ensure_not_aborted(abort)?;
    let spec = spec.validate().map_err(ServiceRunError::Failure)?;
    ensure_not_aborted(abort)?;

    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    ensure_not_aborted(abort)?;
    let engine = Engine::new(build_engine_config(&netlist, None));

    let dc_result = engine
        .run_dc_op_with_abort(&netlist, abort)
        .map_err(|error| {
            ServiceRunError::from_core("DC OP error (required for pole-zero)", error)
        })?;

    let input_idx =
        resolve_node_or_ground_index_with_abort(spec.input_node, &dc_result.node_names, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Pole-zero input node '{}' not found",
                    spec.input_node
                ))
            })?;
    let input_ref_idx =
        resolve_node_or_ground_index_with_abort(spec.input_ref, &dc_result.node_names, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Pole-zero input reference '{}' not found",
                    spec.input_ref
                ))
            })?;
    let output_idx =
        resolve_node_or_ground_index_with_abort(spec.output_node, &dc_result.node_names, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Pole-zero output node '{}' not found",
                    spec.output_node
                ))
            })?;
    let output_ref_idx =
        resolve_node_or_ground_index_with_abort(spec.output_ref, &dc_result.node_names, abort)?
            .ok_or_else(|| {
                ServiceRunError::Failure(format!(
                    "Pole-zero output reference '{}' not found",
                    spec.output_ref
                ))
            })?;

    if input_idx == input_ref_idx {
        return Err(ServiceRunError::Failure(
            "Pole-zero input_node and input_ref cannot be the same node".to_string(),
        ));
    }
    if output_idx == output_ref_idx {
        return Err(ServiceRunError::Failure(
            "Pole-zero output_node and output_ref cannot be the same node".to_string(),
        ));
    }

    let (input_pos, input_neg, input_sign) = canonicalize_pz_port(input_idx, input_ref_idx)
        .map_err(|error| {
            ServiceRunError::Failure(format!("Invalid pole-zero input port: {error}"))
        })?;
    let (output_pos, output_neg, output_sign) = canonicalize_pz_port(output_idx, output_ref_idx)
        .map_err(|error| {
            ServiceRunError::Failure(format!("Invalid pole-zero output port: {error}"))
        })?;

    let input_is_current = spec.transfer_type.input_is_current();
    let (compute_poles, compute_zeros) = spec.analysis_type.compute_flags();

    let pz_result = engine
        .run_pz_ports_with_abort(
            &netlist,
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            input_is_current,
            compute_poles,
            compute_zeros,
            abort,
        )
        .map_err(|error| {
            ServiceRunError::from_core(
                &format!(
                    "Pole-zero analysis error (input={input_idx:?}->{input_ref_idx:?}, output={output_idx:?}->{output_ref_idx:?})"
                ),
                error,
            )
        })?;

    let mut poles = map_pz_roots_with_abort(&pz_result.poles, abort)?;
    let mut zeros = map_pz_roots_with_abort(&pz_result.zeros, abort)?;

    spec.analysis_type.filter_results(&mut poles, &mut zeros);
    ensure_not_aborted(abort)?;

    Ok(PoleZeroData {
        poles,
        zeros,
        gain: input_sign * output_sign * pz_result.dc_gain,
    })
}

fn resolve_node_or_ground_index_with_abort(
    node_name: &str,
    node_names: &[String],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Option<usize>> {
    ensure_not_aborted(abort)?;
    let trimmed = node_name.trim();
    if trimmed.is_empty() {
        return Ok(None);
    }
    if matches!(
        trimmed.to_ascii_lowercase().as_str(),
        "0" | "gnd" | "ground"
    ) {
        return Ok(Some(0));
    }
    if let Ok(index) = trimmed.parse::<usize>() {
        return Ok((index < node_names.len()).then_some(index));
    }

    for (index, candidate) in node_names.iter().enumerate() {
        poll_periodically(abort, index)?;
        if candidate.eq_ignore_ascii_case(trimmed) {
            return Ok(Some(index));
        }
    }
    ensure_not_aborted(abort)?;
    Ok(None)
}

fn map_pz_roots_with_abort(
    roots: &[PzComplex],
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<(Value, Value)>> {
    ensure_not_aborted(abort)?;
    let mut converted = Vec::with_capacity(roots.len());
    for (root_index, root) in roots.iter().enumerate() {
        poll_periodically(abort, root_index)?;
        converted.push((root.re, root.im));
    }
    ensure_not_aborted(abort)?;
    Ok(converted)
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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use rspice_core::abort_signal::ImmediateAbort;

    use super::*;

    struct AbortOnPoll {
        abort_on: usize,
        polls: AtomicUsize,
    }

    impl AbortOnPoll {
        fn new(abort_on: usize) -> Self {
            Self {
                abort_on,
                polls: AtomicUsize::new(0),
            }
        }
    }

    impl AbortSignal for AbortOnPoll {
        fn is_aborted(&self) -> bool {
            self.polls.fetch_add(1, Ordering::Relaxed) + 1 >= self.abort_on
        }
    }

    #[test]
    fn pole_zero_runner_preserves_typed_abort_before_parse_or_validation() {
        let spec = PoleZeroRunSpec::new("", "", "", "", "invalid", "invalid");
        let result = run_pole_zero_analysis_with_abort("not a netlist", spec, &ImmediateAbort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }

    #[test]
    fn pole_zero_root_conversion_polls_inside_the_result_loop() {
        let roots = vec![PzComplex::new(-1.0, 2.0); 129];
        let abort = AbortOnPoll::new(3);

        let result = map_pz_roots_with_abort(&roots, &abort);

        assert!(matches!(result, Err(ServiceRunError::Aborted)));
    }
}
