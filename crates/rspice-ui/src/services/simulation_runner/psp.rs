//! Periodic S-parameter analysis around an authenticated PSS or HB state.
//!
//! Ports are resolved from the elaborated producer circuit. The engine solves
//! all port/sideband inputs together and returns the scattering matrix at the
//! authored wave references, independently of the realized terminations.

use std::collections::HashMap;
use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::s_param;

use super::error::{ensure_not_aborted, poll_periodically};
use super::{
    SParameterPort, ServiceRunError, ServiceRunResult, build_resolved_periodic_engine,
    parse_runner_netlist_with_abort,
};

/// Sweep type for periodic S-parameter analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PspSweep {
    Decade,
    Octave,
    Linear,
}

/// Exact periodic-network request consumed by the service layer.
#[derive(Debug, Clone)]
pub struct PspRunConfig {
    pub start_freq: Value,
    pub stop_freq: Value,
    pub points_per_unit: usize,
    pub sweep: PspSweep,
    /// Optional port-plane assertions. An empty list discovers producer ports.
    pub ports: Vec<SParameterPort>,
    pub max_sideband: usize,
    pub mixed_mode: bool,
    pub noise_parameters: bool,
    pub reltol: Value,
    pub abstol: Value,
}

/// HBSP uses the same sweep and port-plane contract as PSP; only the exact
/// retained large-signal operating point differs.
pub type HbspRunConfig = PspRunConfig;

impl PspRunConfig {
    fn validate_for(&self, analysis: &str) -> Result<(), String> {
        if !self.start_freq.is_finite() || self.start_freq <= 0.0 {
            return Err(format!("{analysis} start frequency must be positive"));
        }
        if !self.stop_freq.is_finite() || self.stop_freq < self.start_freq {
            return Err(format!(
                "{analysis} stop frequency must be >= start frequency"
            ));
        }
        if self.points_per_unit == 0 {
            return Err(format!(
                "{analysis} points per unit must be greater than zero"
            ));
        }
        if self.max_sideband > i32::MAX as usize {
            return Err(format!(
                "{analysis} maximum sideband exceeds the engine index range"
            ));
        }
        if self.mixed_mode && !self.ports.len().is_multiple_of(2) {
            return Err(format!(
                "{analysis} mixed-mode conversion requires an even number of ports paired in declaration order"
            ));
        }
        if self.noise_parameters {
            return Err(format!(
                "{analysis} noise parameters require a correlated periodic-noise solve and are not implemented"
            ));
        }
        if !self.reltol.is_finite() || self.reltol <= 0.0 {
            return Err(format!("{analysis} relative tolerance must be positive"));
        }
        if !self.abstol.is_finite() || self.abstol <= 0.0 {
            return Err(format!("{analysis} absolute tolerance must be positive"));
        }
        for (index, port) in self.ports.iter().enumerate() {
            if port.node_pos.trim().is_empty() || port.node_neg.trim().is_empty() {
                return Err(format!("{analysis} port {} requires both nodes", index + 1));
            }
            if port.z0.is_some_and(|z0| !z0.is_finite() || z0 <= 0.0) {
                return Err(format!(
                    "{analysis} port {} reference impedance must be positive",
                    index + 1
                ));
            }
        }
        Ok(())
    }
}

/// One power-wave conversion path in the periodic multiport matrix.
#[derive(Debug, Clone)]
pub(crate) struct PspPath {
    pub output_port: usize,
    pub input_port: usize,
    pub base_name: String,
    pub output_sideband: i32,
    pub input_sideband: i32,
    pub values: Vec<Complex64>,
}

/// Periodic S-parameter output indexed by port pair and sideband pair.
#[derive(Debug, Clone)]
pub(crate) struct PspData {
    pub frequencies: Vec<Value>,
    pub paths: Vec<PspPath>,
    /// Authored physical-port references in port-number order. Adjacent equal
    /// pairs define differential (2 Z0) and common (Z0 / 2) power-wave channels.
    pub reference_impedances_ohm: Option<Vec<Value>>,
}

/// Run PSP from the exact retained PSS orbit.
///
/// Ports must already be physical RF `P` elements in the producer deck. It is
/// not numerically valid to add terminations after PSS: doing so changes both
/// the topology and the periodic operating point that PSP is linearizing.
pub fn run_psp_analysis_from_pss_with_source_path_and_abort(
    netlist_text: &str,
    config: &PspRunConfig,
    operating_point: &rspice_core::engine::PssOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PspData> {
    run_periodic_sparameter_analysis(
        netlist_text,
        config,
        PeriodicOperatingPoint::Pss(operating_point),
        source_path,
        abort,
    )
}

/// Run HBSP from the exact retained harmonic-balance state.
pub fn run_hbsp_analysis_from_hb_with_source_path_and_abort(
    netlist_text: &str,
    config: &HbspRunConfig,
    operating_point: &rspice_core::engine::HbOperatingPoint,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PspData> {
    run_periodic_sparameter_analysis(
        netlist_text,
        config,
        PeriodicOperatingPoint::Hb(operating_point),
        source_path,
        abort,
    )
}

#[derive(Clone, Copy)]
enum PeriodicOperatingPoint<'a> {
    Pss(&'a rspice_core::engine::PssOperatingPoint),
    Hb(&'a rspice_core::engine::HbOperatingPoint),
}

impl PeriodicOperatingPoint<'_> {
    fn analysis_name(self) -> &'static str {
        match self {
            Self::Pss(_) => "PSP",
            Self::Hb(_) => "HBSP",
        }
    }

    fn producer_name(self) -> &'static str {
        match self {
            Self::Pss(_) => "PSS",
            Self::Hb(_) => "HB",
        }
    }

    fn tolerance(self) -> Value {
        match self {
            Self::Pss(point) => point.config().tolerance,
            Self::Hb(point) => point.config().tolerance,
        }
    }
}

fn run_periodic_sparameter_analysis(
    netlist_text: &str,
    config: &PspRunConfig,
    operating_point: PeriodicOperatingPoint<'_>,
    source_path: Option<&Path>,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<PspData> {
    ensure_not_aborted(abort)?;
    let analysis = operating_point.analysis_name();
    let producer = operating_point.producer_name();
    config
        .validate_for(analysis)
        .map_err(ServiceRunError::Failure)?;
    let netlist = parse_runner_netlist_with_abort(netlist_text, source_path, abort)?;
    let engine = build_resolved_periodic_engine(
        &netlist,
        operating_point.tolerance(),
        "periodic S-parameter configuration",
    )?;
    let max_sideband = config.max_sideband as i32;
    let pac_config = rspice_core::analysis::pac::PacConfig {
        sweep_start: config.start_freq,
        sweep_stop: config.stop_freq,
        num_points: config.points_per_unit,
        sweep_type: match config.sweep {
            PspSweep::Decade => rspice_core::analysis::pac::PacSweepType::Decade,
            PspSweep::Octave => rspice_core::analysis::pac::PacSweepType::Octave,
            PspSweep::Linear => rspice_core::analysis::pac::PacSweepType::Linear,
        },
        sideband_min: -max_sideband,
        sideband_max: max_sideband,
        reltol: config.reltol,
        abstol: config.abstol,
        ..Default::default()
    };
    let prepared = match operating_point {
        PeriodicOperatingPoint::Pss(point) => {
            engine.prepare_psp_from_pss_with_abort(&netlist, pac_config, point, abort)
        }
        PeriodicOperatingPoint::Hb(point) => {
            engine.prepare_psp_from_hb_with_abort(&netlist, pac_config, point, abort)
        }
    }
    .map_err(|error| ServiceRunError::from_core(analysis, error))?;
    let ports = prepared.ports();
    validate_declared_ports(config, ports, analysis, producer)?;
    if config.mixed_mode {
        validate_mixed_mode_port_pairs(ports, analysis)?;
    }

    let sideband_count = config.max_sideband * 2 + 1;
    let path_count = ports
        .len()
        .checked_mul(ports.len())
        .and_then(|n| n.checked_mul(sideband_count))
        .and_then(|n| n.checked_mul(sideband_count))
        .unwrap_or(usize::MAX);
    // The publisher retains x, real and imaginary values for each path and
    // the direct (k=m=0) aliases. Count the actual, endpoint-inclusive grid.
    let retained_values = path_count
        .checked_add(ports.len().saturating_mul(ports.len()))
        .and_then(|n| n.checked_mul(prepared.frequencies().len()))
        .and_then(|n| n.checked_mul(3))
        .unwrap_or(usize::MAX);
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if retained_values > result_limit {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            retained_values,
            result_limit,
        ));
    }
    // The metadata retains physical-port references, including when the
    // coefficients use paired power waves. Consumers derive differential
    // (2 Z0) and common (Z0 / 2) references from the validated equal-Z0 pairs.
    // Dropping this authority made a successful mixed-mode run unviewable.
    let reference_impedances_ohm = Some(ports.iter().map(|port| port.z0).collect());
    let port_count = ports.len();
    let result = prepared
        .run_with_abort(abort)
        .map_err(|error| ServiceRunError::from_core(analysis, error))?;
    let frequencies = result.data.iter().map(|matrix| matrix.frequency).collect();
    let mut paths = Vec::with_capacity(path_count);
    for input in 0..port_count {
        for output in 0..port_count {
            for (out_band, output_sideband) in
                (result.sideband_min..=result.sideband_max).enumerate()
            {
                for (in_band, input_sideband) in
                    (result.sideband_min..=result.sideband_max).enumerate()
                {
                    ensure_not_aborted(abort)?;
                    let mut values = Vec::with_capacity(result.data.len());
                    for (index, matrix) in result.data.iter().enumerate() {
                        poll_periodically(abort, index)?;
                        values.push(matrix.get(
                            output * sideband_count + out_band + 1,
                            input * sideband_count + in_band + 1,
                        ));
                    }
                    paths.push(PspPath {
                        output_port: output + 1,
                        input_port: input + 1,
                        base_name: sparameter_name(output + 1, input + 1, port_count),
                        output_sideband,
                        input_sideband,
                        values,
                    });
                }
            }
        }
    }
    if config.mixed_mode {
        paths = convert_paths_to_mixed_mode(paths, port_count, abort)?;
    }
    ensure_not_aborted(abort)?;
    Ok(PspData {
        frequencies,
        paths,
        reference_impedances_ohm,
    })
}

fn sparameter_name(output: usize, input: usize, port_count: usize) -> String {
    if port_count <= 9 {
        format!("S{output}{input}")
    } else {
        format!("S{output}_{input}")
    }
}

fn validate_mixed_mode_port_pairs(
    ports: &[s_param::SParameterPort],
    analysis: &str,
) -> ServiceRunResult<()> {
    if !ports.len().is_multiple_of(2) {
        return Err(ServiceRunError::Failure(format!(
            "{analysis} mixed-mode conversion requires an even number of physical ports"
        )));
    }
    for (pair_index, pair) in ports.chunks_exact(2).enumerate() {
        if pair[0].z0.to_bits() != pair[1].z0.to_bits() {
            return Err(ServiceRunError::Failure(format!(
                "{analysis} mixed-mode pair {} has unequal reference impedances ({} and {} ohm)",
                pair_index + 1,
                pair[0].z0,
                pair[1].z0
            )));
        }
    }
    Ok(())
}

fn convert_paths_to_mixed_mode(
    paths: Vec<PspPath>,
    port_count: usize,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<PspPath>> {
    if !port_count.is_multiple_of(2) {
        return Err(ServiceRunError::Failure(
            "periodic mixed-mode conversion requires an even number of ports".to_owned(),
        ));
    }
    type PathKey = (usize, usize, i32, i32);
    let mut single_ended: HashMap<PathKey, Vec<Complex64>> = HashMap::with_capacity(paths.len());
    let mut sideband_pairs = Vec::new();
    for (index, path) in paths.into_iter().enumerate() {
        poll_periodically(abort, index)?;
        let key = (
            path.output_port - 1,
            path.input_port - 1,
            path.output_sideband,
            path.input_sideband,
        );
        if !sideband_pairs.contains(&(path.output_sideband, path.input_sideband)) {
            sideband_pairs.push((path.output_sideband, path.input_sideband));
        }
        if single_ended.insert(key, path.values).is_some() {
            return Err(ServiceRunError::Failure(
                "periodic mixed-mode conversion received a duplicate single-ended path".to_owned(),
            ));
        }
    }
    sideband_pairs.sort_unstable();

    let pair_count = port_count / 2;
    let modes = [
        (
            'd',
            [
                std::f64::consts::FRAC_1_SQRT_2,
                -std::f64::consts::FRAC_1_SQRT_2,
            ],
        ),
        (
            'c',
            [
                std::f64::consts::FRAC_1_SQRT_2,
                std::f64::consts::FRAC_1_SQRT_2,
            ],
        ),
    ];
    let mut mixed = Vec::with_capacity(single_ended.len());
    for &(output_sideband, input_sideband) in &sideband_pairs {
        for output_pair in 0..pair_count {
            for (output_mode, output_coefficients) in modes {
                for input_pair in 0..pair_count {
                    for (input_mode, input_coefficients) in modes {
                        ensure_not_aborted(abort)?;
                        let first = single_ended
                            .get(&(
                                output_pair * 2,
                                input_pair * 2,
                                output_sideband,
                                input_sideband,
                            ))
                            .ok_or_else(|| {
                                ServiceRunError::Failure(
                                    "periodic mixed-mode conversion is missing a single-ended path"
                                        .to_owned(),
                                )
                            })?;
                        let mut values = vec![Complex64::new(0.0, 0.0); first.len()];
                        for (physical_output, output_coefficient) in
                            output_coefficients.iter().enumerate()
                        {
                            for (physical_input, input_coefficient) in
                                input_coefficients.iter().enumerate()
                            {
                                let source = single_ended
                                    .get(&(
                                        output_pair * 2 + physical_output,
                                        input_pair * 2 + physical_input,
                                        output_sideband,
                                        input_sideband,
                                    ))
                                    .ok_or_else(|| {
                                        ServiceRunError::Failure(
                                            "periodic mixed-mode conversion is missing a paired single-ended path"
                                                .to_owned(),
                                        )
                                    })?;
                                if source.len() != values.len() {
                                    return Err(ServiceRunError::Failure(
                                        "periodic mixed-mode paths have inconsistent frequency lengths"
                                            .to_owned(),
                                    ));
                                }
                                let coefficient = output_coefficient * input_coefficient;
                                for (sample_index, (target, value)) in
                                    values.iter_mut().zip(source).enumerate()
                                {
                                    poll_periodically(abort, sample_index)?;
                                    *target += *value * coefficient;
                                }
                            }
                        }
                        mixed.push(PspPath {
                            output_port: output_pair + 1,
                            input_port: input_pair + 1,
                            base_name: format!(
                                "S{output_mode}{input_mode}{}{}",
                                output_pair + 1,
                                input_pair + 1
                            ),
                            output_sideband,
                            input_sideband,
                            values,
                        });
                    }
                }
            }
        }
    }
    ensure_not_aborted(abort)?;
    Ok(mixed)
}

fn validate_declared_ports(
    config: &PspRunConfig,
    declared: &[s_param::SParameterPort],
    analysis: &str,
    producer: &str,
) -> ServiceRunResult<()> {
    if config.ports.is_empty() {
        return Ok(());
    }
    if config.ports.len() != declared.len() {
        return Err(ServiceRunError::Failure(format!(
            "{analysis} setup declares {} port(s), but the {producer} producer deck contains {} RF Port component(s)",
            config.ports.len(),
            declared.len()
        )));
    }
    for (index, (configured, actual)) in config.ports.iter().zip(declared).enumerate() {
        if !configured
            .node_pos
            .trim()
            .eq_ignore_ascii_case(&actual.node_pos)
            || !configured
                .node_neg
                .trim()
                .eq_ignore_ascii_case(&actual.node_neg)
        {
            return Err(ServiceRunError::Failure(format!(
                "{analysis} port {} setup ({}, {}) does not match producer-deck port ({}, {})",
                index + 1,
                configured.node_pos.trim(),
                configured.node_neg.trim(),
                actual.node_pos,
                actual.node_neg
            )));
        }
        if configured
            .z0
            .is_some_and(|z0| z0.to_bits() != actual.z0.to_bits())
        {
            return Err(ServiceRunError::Failure(format!(
                "{analysis} port {} setup impedance does not exactly match producer-deck z0 {} ohm",
                index + 1,
                actual.z0
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::services::simulation_runner::hb::{
        HbRunConfig, HbToneRunConfig, run_hb_analysis_with_abort,
    };
    use crate::services::simulation_runner::{
        build_resolved_periodic_engine, parse_runner_netlist_with_abort,
    };
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::PssConfig;
    use rspice_core::analysis::s_param::PortRealization;

    fn config() -> PspRunConfig {
        PspRunConfig {
            start_freq: 1.0e3,
            stop_freq: 1.0e6,
            points_per_unit: 3,
            sweep: PspSweep::Decade,
            ports: vec![
                SParameterPort {
                    node_pos: "P1".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: Some(50.0),
                },
                SParameterPort {
                    node_pos: "P2".to_owned(),
                    node_neg: "0".to_owned(),
                    z0: Some(50.0),
                },
            ],
            max_sideband: 1,
            mixed_mode: false,
            noise_parameters: false,
            reltol: 1.0e-3,
            abstol: 1.0e-12,
        }
    }

    #[test]
    fn mixed_mode_is_validated_while_unimplemented_noise_parameters_fail_closed() {
        let mut request = config();
        request.mixed_mode = true;
        request
            .validate_for("PSP")
            .expect("an even port list supports mixed-mode conversion");
        request.noise_parameters = true;
        assert!(
            request
                .validate_for("PSP")
                .unwrap_err()
                .contains("noise parameters")
        );
    }

    #[test]
    fn mixed_mode_power_wave_transform_preserves_identity_and_rejects_cross_mode_leakage() {
        let mut single_ended = Vec::new();
        for output in 1..=2 {
            for input in 1..=2 {
                single_ended.push(PspPath {
                    output_port: output,
                    input_port: input,
                    base_name: format!("S{output}{input}"),
                    output_sideband: 0,
                    input_sideband: 0,
                    values: vec![if output == input {
                        Complex64::new(1.0, 0.0)
                    } else {
                        Complex64::new(0.0, 0.0)
                    }],
                });
            }
        }

        let mixed = convert_paths_to_mixed_mode(single_ended, 2, &NoAbort)
            .expect("two equal-reference ports convert");
        assert_eq!(mixed.len(), 4);
        for name in ["Sdd11", "Scc11"] {
            let value = mixed
                .iter()
                .find(|path| path.base_name == name)
                .expect("diagonal mixed-mode path")
                .values[0];
            assert!((value - Complex64::new(1.0, 0.0)).norm() < 1.0e-14);
        }
        for name in ["Sdc11", "Scd11"] {
            let value = mixed
                .iter()
                .find(|path| path.base_name == name)
                .expect("cross-mode path")
                .values[0];
            assert!(value.norm() < 1.0e-14);
        }
    }

    #[test]
    fn producer_ports_are_an_exact_execution_contract() {
        let declared = vec![
            s_param::SParameterPort {
                number: 1,
                source_name: "P1".to_owned(),
                node_pos: "P1".to_owned(),
                node_neg: "0".to_owned(),
                z0: 50.0,
                realization: PortRealization::Thevenin,
            },
            s_param::SParameterPort {
                number: 2,
                source_name: "P2".to_owned(),
                node_pos: "P2".to_owned(),
                node_neg: "0".to_owned(),
                z0: 50.0,
                realization: PortRealization::Thevenin,
            },
        ];
        validate_declared_ports(&config(), &declared, "PSP", "PSS").expect("matching ports bind");
        validate_mixed_mode_port_pairs(&declared, "PSP")
            .expect("equal-impedance adjacent ports form a mixed-mode pair");

        let mut mismatch = config();
        mismatch.ports[1].z0 = Some(75.0);
        assert!(
            validate_declared_ports(&mismatch, &declared, "PSP", "PSS")
                .unwrap_err()
                .to_string()
                .contains("does not exactly match")
        );

        let mut unequal_pair = declared.clone();
        unequal_pair[1].z0 = 75.0;
        assert!(
            validate_mixed_mode_port_pairs(&unequal_pair, "PSP")
                .unwrap_err()
                .to_string()
                .contains("unequal reference impedances")
        );
    }

    #[test]
    fn static_matched_two_port_produces_closed_form_periodic_s_parameters() {
        let deck = "* matched ports around a 50 ohm series network\n\
                    P1 p1 0 PORT=1 Z0=50\n\
                    R1 p1 p2 50\n\
                    C1 p1 0 1e-18\n\
                    P2 p2 0 PORT=2 Z0=50\n\
                    .end\n";
        let netlist = parse_runner_netlist_with_abort(deck, None, &NoAbort)
            .expect("service deck parsing succeeds");
        let operating_point =
            build_resolved_periodic_engine(&netlist, 1.0e-7, "test PSS producer configuration")
                .expect("resolved producer engine")
                .run_pss_operating_point_with_abort(
                    &netlist,
                    PssConfig::new(1.0e6)
                        .with_tstab_periods(1)
                        .with_points_per_period(32)
                        .with_harmonics(4)
                        .with_tolerance(1.0e-7),
                    &NoAbort,
                )
                .expect("zero-state PSS converges");
        let mut request = config();
        request.start_freq = 1.0e4;
        request.stop_freq = 1.0e4;
        request.points_per_unit = 1;
        request.sweep = PspSweep::Linear;

        let result = run_psp_analysis_from_pss_with_source_path_and_abort(
            deck,
            &request,
            &operating_point,
            None,
            &NoAbort,
        )
        .expect("PSP completes");
        let direct = |out, input| {
            result
                .paths
                .iter()
                .find(|path| {
                    path.output_port == out
                        && path.input_port == input
                        && path.output_sideband == 0
                        && path.input_sideband == 0
                })
                .expect("direct path")
                .values[0]
        };

        for (actual, expected) in [
            (direct(1, 1), Complex64::new(1.0 / 3.0, 0.0)),
            (direct(2, 1), Complex64::new(2.0 / 3.0, 0.0)),
            (direct(1, 2), Complex64::new(2.0 / 3.0, 0.0)),
            (direct(2, 2), Complex64::new(1.0 / 3.0, 0.0)),
        ] {
            assert!(
                (actual - expected).norm() < 1.0e-8,
                "PSP direct path = {actual}, expected {expected}"
            );
        }

        assert!(result.paths.iter().all(|path| {
            path.output_sideband == path.input_sideband || path.values[0].norm() < 1.0e-9
        }));

        request.mixed_mode = true;
        let mixed = run_psp_analysis_from_pss_with_source_path_and_abort(
            deck,
            &request,
            &operating_point,
            None,
            &NoAbort,
        )
        .expect("mixed-mode PSP completes");
        assert_mixed_matched_network(&mixed);
    }

    #[test]
    fn static_matched_two_port_produces_closed_form_hb_s_parameters() {
        let deck = "* matched ports around a 50 ohm series network\n\
                    P1 p1 0 PORT=1 Z0=50\n\
                    R1 p1 p2 50\n\
                    C1 p1 0 1e-18\n\
                    P2 p2 0 PORT=2 Z0=50\n\
                    .end\n";
        let operating_point = run_hb_analysis_with_abort(
            deck,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(1.0e6, 8)],
                reltol: 2.5e-7,
                ..HbRunConfig::default()
            },
            &NoAbort,
        )
        .expect("HB service converges")
        .operating_point;
        let mut request = config();
        request.start_freq = 1.0e4;
        request.stop_freq = 1.0e4;
        request.points_per_unit = 1;
        request.sweep = PspSweep::Linear;

        let result = run_hbsp_analysis_from_hb_with_source_path_and_abort(
            deck,
            &request,
            operating_point.as_ref(),
            None,
            &NoAbort,
        )
        .expect("HBSP completes");
        let direct = |out, input| {
            result
                .paths
                .iter()
                .find(|path| {
                    path.output_port == out
                        && path.input_port == input
                        && path.output_sideband == 0
                        && path.input_sideband == 0
                })
                .expect("direct path")
                .values[0]
        };
        for (actual, expected) in [
            (direct(1, 1), Complex64::new(1.0 / 3.0, 0.0)),
            (direct(2, 1), Complex64::new(2.0 / 3.0, 0.0)),
            (direct(1, 2), Complex64::new(2.0 / 3.0, 0.0)),
            (direct(2, 2), Complex64::new(1.0 / 3.0, 0.0)),
        ] {
            assert!((actual - expected).norm() < 1.0e-8);
        }
        assert!(result.paths.iter().all(|path| {
            path.output_sideband == path.input_sideband || path.values[0].norm() < 1.0e-9
        }));

        request.mixed_mode = true;
        let mixed = run_hbsp_analysis_from_hb_with_source_path_and_abort(
            deck,
            &request,
            operating_point.as_ref(),
            None,
            &NoAbort,
        )
        .expect("mixed-mode HBSP completes");
        assert_mixed_matched_network(&mixed);
    }

    fn assert_mixed_matched_network(data: &PspData) {
        assert_eq!(data.reference_impedances_ohm, Some(vec![50.0, 50.0]));
        for (name, expected) in [
            ("Sdd11", -1.0 / 3.0),
            ("Scc11", 1.0),
            ("Sdc11", 0.0),
            ("Scd11", 0.0),
        ] {
            let path = data
                .paths
                .iter()
                .find(|path| {
                    path.base_name == name && path.output_sideband == 0 && path.input_sideband == 0
                })
                .expect("complete modal matrix");
            assert!(
                (path.values[0] - Complex64::new(expected, 0.0)).norm() < 1e-8,
                "{name}: {:?}",
                path.values[0]
            );
        }
    }

    fn hbsp_fixture(deck: &str) -> ServiceRunResult<PspData> {
        let mut request = config();
        request.start_freq = 1e4;
        request.stop_freq = 1e4;
        request.points_per_unit = 1;
        request.sweep = PspSweep::Linear;
        hbsp_fixture_request(deck, &request)
    }

    fn hbsp_fixture_request(deck: &str, request: &PspRunConfig) -> ServiceRunResult<PspData> {
        let point = run_hb_analysis_with_abort(
            deck,
            &HbRunConfig {
                tones: vec![HbToneRunConfig::new(1.0e6, 8)],
                reltol: 2.5e-7,
                ..HbRunConfig::default()
            },
            &NoAbort,
        )?
        .operating_point;
        run_hbsp_analysis_from_hb_with_source_path_and_abort(
            deck,
            request,
            point.as_ref(),
            None,
            &NoAbort,
        )
    }

    fn assert_series_network(data: &PspData) {
        assert_eq!(data.reference_impedances_ohm, Some(vec![50.0, 50.0]));
        for path in &data.paths {
            let expected = if path.output_sideband != path.input_sideband {
                0.0
            } else if path.output_port == path.input_port {
                1.0 / 3.0
            } else {
                2.0 / 3.0
            };
            for value in &path.values {
                assert!(
                    (*value - Complex64::new(expected, 0.0)).norm() < 1e-8,
                    "{}[{},{}]: {value}, expected {expected}",
                    path.base_name,
                    path.output_sideband,
                    path.input_sideband
                );
            }
        }
    }

    #[test]
    fn periodic_ports_resolve_after_hierarchy_elaboration() {
        let data = hbsp_fixture("* nested RF ports\n.subckt ports a b\nP1 a 0 PORT=1 Z0=50\nP2 b 0 PORT=2 Z0=50\n.ends\nXports p1 p2 ports\nR1 p1 p2 50\nC1 p1 0 1e-18\n.end\n")
            .expect("ports must come from the circuit consumed by HBSP");
        assert_series_network(&data);
    }

    #[test]
    fn periodic_ports_use_physical_terminations_and_authored_wave_references() {
        let data = hbsp_fixture("* port multiplicity changes its physical termination\n.subckt generator p\nP1 p 0 PORT=1 Z0=50\n.ends\nX1 p1 generator M=2\nR1 p1 p2 50\nC1 p1 0 1e-18\nP2 p2 0 PORT=2 Z0=50\n.end\n")
            .expect("HBSP retains the physical producer circuit");
        assert_series_network(&data);
    }

    #[test]
    fn periodic_ports_discover_single_port_and_preserve_exact_sweep_and_reference() {
        let deck = "* one annotated hierarchical port\n.subckt source p\nV1 0 p DC 0 portnum=1 z0=75\n.ends\nX1 p source M=2\nR1 p 0 150\n.end\n";
        let mut request = config();
        request.ports.clear();
        request.max_sideband = 0;
        request.start_freq = 1e3;
        request.stop_freq = 1e4;
        request.points_per_unit = 2;
        let data = hbsp_fixture_request(deck, &request).unwrap();
        assert_eq!(data.frequencies.len(), 2);
        assert_eq!(data.frequencies[0], 1e3);
        assert_eq!(data.frequencies[1], 1e4);
        assert_eq!(data.reference_impedances_ohm, Some(vec![75.0]));
        assert_eq!(data.paths.len(), 1);
        assert_eq!(data.paths[0].base_name, "S11");
        assert!(
            data.paths[0]
                .values
                .iter()
                .all(|value| (*value - Complex64::new(1.0 / 3.0, 0.0)).norm() < 1e-12)
        );
    }
}
