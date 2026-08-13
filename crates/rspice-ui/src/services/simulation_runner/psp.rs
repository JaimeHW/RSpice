//! Periodic S-parameter analysis around an authenticated shooting-PSS state.
//!
//! Each port is a physical, matched Thevenin termination present in the deck
//! that produced the PSS orbit. PAC then supplies the complete input/output
//! sideband conversion matrices, which are converted from port-plane voltages
//! to power-wave scattering parameters.

use std::path::Path;

use num_complex::Complex64;
use rspice_core::Value;
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::s_param::{self, PortRealization};

use super::error::{ensure_not_aborted, poll_periodically};
use super::pac_pxf::{run_pac_internal_from_hb_with_abort, run_pac_internal_from_pss_with_abort};
use super::{
    PacFrequencySweep, PacRunConfig, SParameterPort, ServiceRunError, ServiceRunResult,
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
        if self.ports.len() < 2 {
            return Err(format!("{analysis} requires at least two configured ports"));
        }
        if self.max_sideband == 0 {
            return Err(format!(
                "{analysis} maximum sideband must be greater than zero"
            ));
        }
        if self.max_sideband > i32::MAX as usize {
            return Err(format!(
                "{analysis} maximum sideband exceeds the engine index range"
            ));
        }
        if self.mixed_mode {
            return Err(format!(
                "{analysis} mixed-mode conversion is not implemented; disable mixed mode"
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

    fn frequency_count(&self) -> usize {
        match self.sweep {
            PspSweep::Linear => self.points_per_unit,
            PspSweep::Decade => ((self.stop_freq.log10() - self.start_freq.log10())
                * self.points_per_unit as Value)
                .ceil() as usize,
            PspSweep::Octave => ((self.stop_freq.log2() - self.start_freq.log2())
                * self.points_per_unit as Value)
                .ceil() as usize,
        }
        .max(1)
    }
}

/// One power-wave conversion path in the periodic multiport matrix.
#[derive(Debug, Clone)]
pub(crate) struct PspPath {
    pub output_port: usize,
    pub input_port: usize,
    pub output_sideband: i32,
    pub input_sideband: i32,
    pub values: Vec<Complex64>,
}

/// Periodic S-parameter output indexed by port pair and sideband pair.
#[derive(Debug, Clone)]
pub(crate) struct PspData {
    pub frequencies: Vec<Value>,
    pub paths: Vec<PspPath>,
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

    fn basis(self) -> (Value, usize, Value) {
        match self {
            Self::Pss(point) => {
                let config = point.config();
                (
                    config.fundamental_freq,
                    config.num_harmonics,
                    config.tolerance,
                )
            }
            Self::Hb(point) => {
                let config = point.config();
                (
                    config.fundamental_freq,
                    config.num_harmonics,
                    config.tolerance,
                )
            }
        }
    }

    fn harmonic_capacity(self) -> usize {
        match self {
            Self::Pss(point) => point.spectral_harmonic_capacity(),
            Self::Hb(point) => point.spectral_harmonic_capacity(),
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
    let ports = s_param::collect_ports(&netlist).map_err(|error| {
        ServiceRunError::Failure(format!(
            "{analysis} requires RF Port components in the {producer} producer deck: {error}"
        ))
    })?;
    if ports.len() < 2 {
        return Err(ServiceRunError::Failure(format!(
            "{analysis} requires at least two RF Port components in the {producer} producer deck"
        )));
    }
    if ports
        .iter()
        .any(|port| port.realization != PortRealization::Thevenin)
    {
        return Err(ServiceRunError::Failure(format!(
            "{analysis} ports must be physical RF Port components with their reference impedances present during {producer}"
        )));
    }
    validate_declared_ports(config, &ports, analysis, producer)?;

    let required_periodic_harmonics = config.max_sideband.saturating_mul(2).max(8);
    if required_periodic_harmonics > operating_point.harmonic_capacity() {
        return Err(ServiceRunError::Failure(format!(
            "{analysis} requires {required_periodic_harmonics} periodic harmonics for its sideband span, but the retained {producer} state has capacity {}",
            operating_point.harmonic_capacity()
        )));
    }

    let sideband_count = config
        .max_sideband
        .checked_mul(2)
        .and_then(|value| value.checked_add(1))
        .unwrap_or(usize::MAX);
    let path_values = ports
        .len()
        .checked_mul(ports.len())
        .and_then(|value| value.checked_mul(sideband_count))
        .and_then(|value| value.checked_mul(sideband_count))
        .and_then(|value| value.checked_mul(config.frequency_count()))
        // Each emitted complex waveform retains x, real, and imaginary.
        .and_then(|value| value.checked_mul(3))
        .unwrap_or(usize::MAX);
    let alias_values = ports
        .len()
        .checked_mul(ports.len())
        .and_then(|value| value.checked_mul(config.frequency_count()))
        .and_then(|value| value.checked_mul(3))
        .unwrap_or(usize::MAX);
    let retained_values = path_values.saturating_add(alias_values);
    let result_limit = rspice_core::ResourceLimits::default().max_result_values;
    if retained_values > result_limit {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::ResultValues,
            retained_values,
            result_limit,
        ));
    }
    let pac_solve_count = ports.len().saturating_mul(ports.len());
    let solve_limit = rspice_core::ResourceLimits::default().max_batch_runs;
    if pac_solve_count > solve_limit {
        return Err(ServiceRunError::resource_limit(
            rspice_core::ResourceKind::BatchRuns,
            pac_solve_count,
            solve_limit,
        ));
    }

    let (fundamental_freq, num_harmonics, tolerance) = operating_point.basis();
    let mut frequencies: Option<Vec<Value>> = None;
    let mut paths = Vec::with_capacity(
        ports
            .len()
            .saturating_mul(ports.len())
            .saturating_mul(sideband_count)
            .saturating_mul(sideband_count),
    );
    let max_sideband = config.max_sideband as i32;

    for (input_index, input_port) in ports.iter().enumerate() {
        poll_periodically(abort, input_index)?;
        for (output_index, output_port) in ports.iter().enumerate() {
            poll_periodically(abort, output_index)?;
            let pac_config = PacRunConfig {
                pss_fundamental_freq: fundamental_freq,
                pss_num_harmonics: num_harmonics,
                pss_tolerance: tolerance,
                start_freq: config.start_freq,
                stop_freq: config.stop_freq,
                points_per_unit: config.points_per_unit,
                sweep: match config.sweep {
                    PspSweep::Decade => PacFrequencySweep::Decade,
                    PspSweep::Octave => PacFrequencySweep::Octave,
                    PspSweep::Linear => PacFrequencySweep::Linear,
                },
                max_sideband,
                input_source: input_port.source_name.clone(),
                output_node: output_port.node_pos.clone(),
                output_ref: Some(output_port.node_neg.clone()),
                pac_magnitude: 1.0,
                include_dc: true,
                reltol: config.reltol,
                abstol: config.abstol,
            };
            let pac = match operating_point {
                PeriodicOperatingPoint::Pss(point) => {
                    run_pac_internal_from_pss_with_abort(&netlist, &pac_config, point, abort)?
                }
                PeriodicOperatingPoint::Hb(point) => {
                    run_pac_internal_from_hb_with_abort(&netlist, &pac_config, point, abort)?
                }
            }
            .pac_result;

            if let Some(expected) = &frequencies {
                if expected != &pac.frequencies {
                    return Err(ServiceRunError::Failure(
                        "PSP port solves produced inconsistent frequency grids".to_owned(),
                    ));
                }
            } else {
                frequencies = Some(pac.frequencies.clone());
            }

            let wave_scale = 2.0 * (input_port.z0 / output_port.z0).sqrt();
            for output_sideband in -max_sideband..=max_sideband {
                for input_sideband in -max_sideband..=max_sideband {
                    let mut values = Vec::with_capacity(pac.frequencies.len());
                    for frequency_index in 0..pac.frequencies.len() {
                        poll_periodically(abort, frequency_index)?;
                        let mut value = pac.conversion_matrix.get(
                            frequency_index,
                            output_sideband,
                            input_sideband,
                        ) * wave_scale;
                        if input_index == output_index && input_sideband == output_sideband {
                            value -= Complex64::new(1.0, 0.0);
                        }
                        values.push(value);
                    }
                    paths.push(PspPath {
                        output_port: output_index + 1,
                        input_port: input_index + 1,
                        output_sideband,
                        input_sideband,
                        values,
                    });
                }
            }
        }
    }

    ensure_not_aborted(abort)?;
    Ok(PspData {
        frequencies: frequencies.unwrap_or_default(),
        paths,
    })
}

fn validate_declared_ports(
    config: &PspRunConfig,
    declared: &[s_param::SParameterPort],
    analysis: &str,
    producer: &str,
) -> ServiceRunResult<()> {
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
    use rspice_core::abort_signal::NoAbort;
    use rspice_core::analysis::PssConfig;
    use rspice_core::engine::{Engine, SimulationConfig};

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
    fn optional_outputs_fail_closed_until_their_numerics_exist() {
        let mut request = config();
        request.mixed_mode = true;
        assert!(
            request
                .validate_for("PSP")
                .unwrap_err()
                .contains("mixed-mode")
        );
        request.mixed_mode = false;
        request.noise_parameters = true;
        assert!(
            request
                .validate_for("PSP")
                .unwrap_err()
                .contains("noise parameters")
        );
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

        let mut mismatch = config();
        mismatch.ports[1].z0 = Some(75.0);
        assert!(
            validate_declared_ports(&mismatch, &declared, "PSP", "PSS")
                .unwrap_err()
                .to_string()
                .contains("does not exactly match")
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
        let netlist = rspice_core::Netlist::parse(deck).expect("deck parses");
        let operating_point = Engine::new(SimulationConfig::default())
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
    }

    #[test]
    fn static_matched_two_port_produces_closed_form_hb_s_parameters() {
        let deck = "* matched ports around a 50 ohm series network\n\
                    P1 p1 0 PORT=1 Z0=50\n\
                    R1 p1 p2 50\n\
                    C1 p1 0 1e-18\n\
                    P2 p2 0 PORT=2 Z0=50\n\
                    .end\n";
        let netlist = rspice_core::Netlist::parse(deck).expect("deck parses");
        let operating_point = Engine::new(SimulationConfig::default())
            .run_hb(
                &netlist,
                rspice_core::analysis::HbConfig::new(1.0e6).with_harmonics(8),
            )
            .expect("HB converges")
            .operating_point;
        let mut request = config();
        request.start_freq = 1.0e4;
        request.stop_freq = 1.0e4;
        request.points_per_unit = 1;
        request.sweep = PspSweep::Linear;

        let result = run_hbsp_analysis_from_hb_with_source_path_and_abort(
            deck,
            &request,
            &operating_point,
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
    }
}
