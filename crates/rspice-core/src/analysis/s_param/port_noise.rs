//! Assembly of one `.SP DONOISE` run's port-noise evidence.
//!
//! The port-noise solver returns a per-frequency current-noise covariance
//! matrix. Turning that into publishable evidence means proving it lines up
//! with the scattering sweep it belongs to, and — for a two-port network —
//! converting scattering to admittance and deriving the standard noise
//! parameters at every frequency.
//!
//! That assembly used to live in the CLI and the Python bindings, in two
//! copies that agreed by inspection but disagreed about the one thing that
//! matters: what to do when the two-port derivation has no physical solution.
//! One surface refused the sweep, the other published NaNs next to a validity
//! flag a caller could ignore. This module is that operation, with one
//! validity policy: an undefined two-port parameter set is a typed failure
//! naming the frequency it occurred at. A noise figure that is quietly wrong
//! is worse than one that is visibly absent, because it will be believed.

use thiserror::Error;

use crate::abort_signal::AbortSignal;
use crate::analysis::noise::PortNoiseCorrelationResult;
use crate::{Complex64, NoAbort, Value};

use super::matrix::SParameterResult;
use super::network::y_from_s;
use super::noise_params::{TwoPortNoise, derive_two_port_noise};
use super::ports::SParameterPort;

/// Number of ports for which the standard noise parameters are defined.
const TWO_PORT: usize = 2;

/// Why a port-noise sweep could not be assembled into publishable evidence.
///
/// Every variant identifies the offending point, port count, or frequency, so
/// a frontend can translate it into its native diagnostic without parsing a
/// message.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum PortNoiseAssemblyError {
    /// The assembly was cancelled by its abort source.
    #[error("port-noise assembly was cancelled")]
    Aborted,
    /// The sweep declared no ports, so there is nothing to correlate.
    #[error("port-noise sweep requires at least one port")]
    NoPorts,
    /// The sweep covered no frequencies, so there is nothing to publish.
    #[error("port-noise sweep requires at least one frequency")]
    NoFrequencies,
    /// The scattering sweep and the noise sweep describe different grids.
    #[error("the scattering sweep has {scattering} frequencies and the noise sweep {noise}")]
    ScatteringMismatch { scattering: usize, noise: usize },
    /// The scattering sweep was measured for a different number of ports than
    /// the noise sweep declares.
    #[error("port-noise sweep has {ports} ports but the scattering sweep has {scattering_ports}")]
    PortCountMismatch {
        ports: usize,
        scattering_ports: usize,
    },
    /// A covariance point is not at the frequency it was requested for.
    #[error("port-noise point {} is at {actual:.16e} Hz, expected {expected:.16e} Hz", index + 1)]
    FrequencyMismatch {
        index: usize,
        expected: Value,
        actual: Value,
    },
    /// A covariance matrix is not square and `N x N` for the declared ports.
    #[error(
        "port-noise point {} returned a malformed covariance matrix for {ports} ports",
        index + 1
    )]
    MalformedCovariance { index: usize, ports: usize },
    /// The noise was evaluated at a temperature no physical derivation accepts.
    #[error("port-noise sweep needs a positive finite temperature, got {temperature}")]
    Temperature { temperature: Value },
    /// Scattering could not be converted to admittance at one frequency.
    #[error("scattering could not be converted to admittance at {frequency:.16e} Hz: {detail}")]
    Admittance { frequency: Value, detail: String },
    /// The admittance and covariance at one frequency do not support a
    /// physical two-port noise solution.
    ///
    /// Reported rather than published with `valid = false`: a noise figure
    /// that is present but meaningless will be believed.
    #[error(
        "two-port noise parameters are undefined at {frequency:.16e} Hz; the admittance/noise data do not support a physical finite solution"
    )]
    UndefinedTwoPortParameters { frequency: Value },
}

/// One `.SP DONOISE` run's assembled port-noise evidence.
#[derive(Debug, Clone)]
pub struct PortNoiseAssembly {
    /// Circuit temperature the noise parameters were derived at, in kelvin.
    pub reference_temperature_kelvin: Value,
    /// Validated correlation sweep, one point per swept frequency, in the
    /// exact shape the shared port-noise result document accepts.
    pub points: Vec<PortNoiseCorrelationResult>,
    /// Standard two-port noise parameters, present only for a two-port
    /// network. Every entry is a physical solution: an unphysical one is a
    /// typed error rather than a `valid = false` placeholder.
    ///
    /// `None` for any other port count: `Rn`, `F`, `Fmin` and `Sopt` are
    /// two-port quantities, and deriving them from a sub-matrix of a larger
    /// network would describe a different device.
    pub two_port: Option<Vec<TwoPortNoise>>,
}

/// Validate one port-noise sweep against its scattering sweep and derive the
/// two-port noise parameters when the network has exactly two ports, without
/// cancellation.
pub fn assemble_port_noise(
    ports: &[SParameterPort],
    scattering: &SParameterResult,
    points: Vec<PortNoiseCorrelationResult>,
    temperature: Value,
) -> Result<PortNoiseAssembly, PortNoiseAssemblyError> {
    assemble_port_noise_with_abort(ports, scattering, points, temperature, &NoAbort)
}

/// Validate one port-noise sweep against its scattering sweep and derive the
/// two-port noise parameters when the network has exactly two ports.
///
/// `points` must already be in swept-frequency order, which is the order the
/// port-noise solver returns them in. The abort source is polled per frequency
/// so a cancelled run stops inside the derivation rather than only between
/// solver calls.
pub fn assemble_port_noise_with_abort(
    ports: &[SParameterPort],
    scattering: &SParameterResult,
    points: Vec<PortNoiseCorrelationResult>,
    temperature: Value,
    abort: &dyn AbortSignal,
) -> Result<PortNoiseAssembly, PortNoiseAssemblyError> {
    if abort.is_aborted() {
        return Err(PortNoiseAssemblyError::Aborted);
    }
    let count = ports.len();
    if count == 0 {
        return Err(PortNoiseAssemblyError::NoPorts);
    }
    if scattering.data.is_empty() || points.is_empty() {
        return Err(PortNoiseAssemblyError::NoFrequencies);
    }
    if !temperature.is_finite() || temperature <= 0.0 {
        return Err(PortNoiseAssemblyError::Temperature { temperature });
    }
    if scattering.data.len() != points.len() {
        return Err(PortNoiseAssemblyError::ScatteringMismatch {
            scattering: scattering.data.len(),
            noise: points.len(),
        });
    }
    if scattering.num_ports != count {
        return Err(PortNoiseAssemblyError::PortCountMismatch {
            ports: count,
            scattering_ports: scattering.num_ports,
        });
    }
    for (index, (matrix, point)) in scattering.data.iter().zip(&points).enumerate() {
        if index.is_multiple_of(64) && abort.is_aborted() {
            return Err(PortNoiseAssemblyError::Aborted);
        }
        // The solve is asked for exactly the swept grid; the tolerance only
        // absorbs a round trip through the solver's own bookkeeping, not a
        // genuinely different frequency.
        let tolerance = matrix.frequency.abs().max(1.0) * Value::EPSILON * 64.0;
        if (point.frequency - matrix.frequency).abs() > tolerance {
            return Err(PortNoiseAssemblyError::FrequencyMismatch {
                index,
                expected: matrix.frequency,
                actual: point.frequency,
            });
        }
        if point.current_correlation.len() != count
            || point
                .current_correlation
                .iter()
                .any(|row| row.len() != count)
        {
            return Err(PortNoiseAssemblyError::MalformedCovariance {
                index,
                ports: count,
            });
        }
    }

    let two_port = if count == TWO_PORT {
        Some(derive_two_port_sweep(
            ports,
            scattering,
            &points,
            temperature,
            abort,
        )?)
    } else {
        None
    };

    Ok(PortNoiseAssembly {
        reference_temperature_kelvin: temperature,
        points,
        two_port,
    })
}

/// Derive the standard two-port noise parameters at every swept frequency.
///
/// Shapes were validated by the caller, so the only failures here are physical
/// ones: an admittance conversion that has no solution, and a parameter set
/// that is not finite and physical.
fn derive_two_port_sweep(
    ports: &[SParameterPort],
    scattering: &SParameterResult,
    points: &[PortNoiseCorrelationResult],
    temperature: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<TwoPortNoise>, PortNoiseAssemblyError> {
    let reference_impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();
    // Two ports were counted by the caller, so port 1 is present; its
    // reference impedance is the one the noise figure is defined against.
    let Some(input_reference) = ports.first().map(|port| port.z0) else {
        return Err(PortNoiseAssemblyError::NoPorts);
    };
    let mut derived = Vec::with_capacity(points.len());
    for (index, (matrix, point)) in scattering.data.iter().zip(points).enumerate() {
        if index.is_multiple_of(16) && abort.is_aborted() {
            return Err(PortNoiseAssemblyError::Aborted);
        }
        // Derived from the S-matrix just measured rather than read off the
        // port branch currents: behind a Thevenin port that current flows
        // through the reference resistor and is not the admittance term.
        let square = (1..=TWO_PORT)
            .map(|row| {
                (1..=TWO_PORT)
                    .map(|column| matrix.get(row, column))
                    .collect()
            })
            .collect::<Vec<Vec<Complex64>>>();
        let admittance = y_from_s(&square, &reference_impedances).map_err(|error| {
            PortNoiseAssemblyError::Admittance {
                frequency: matrix.frequency,
                detail: error.to_string(),
            }
        })?;
        let parameters = derive_two_port_noise(
            &admittance,
            &point.current_correlation,
            input_reference,
            temperature,
        );
        if !parameters.valid {
            return Err(PortNoiseAssemblyError::UndefinedTwoPortParameters {
                frequency: matrix.frequency,
            });
        }
        derived.push(parameters);
    }
    Ok(derived)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;
    use crate::analysis::s_param::{Port, PortRealization, SMatrix};

    /// The typed refusal an assembly that must not succeed produced.
    fn refused(
        outcome: Result<PortNoiseAssembly, PortNoiseAssemblyError>,
    ) -> PortNoiseAssemblyError {
        outcome.expect_err("this input has no publishable port-noise evidence")
    }

    fn port(number: usize, z0: Value) -> SParameterPort {
        SParameterPort {
            number,
            source_name: format!("V{number}"),
            node_pos: format!("p{number}"),
            node_neg: "0".to_owned(),
            z0,
            realization: PortRealization::Ideal,
        }
    }

    fn sweep(impedances: &[Value], matrices: Vec<SMatrix>) -> SParameterResult {
        let reference = impedances.first().copied().unwrap_or(50.0);
        let mut result = SParameterResult::new(
            reference,
            impedances
                .iter()
                .enumerate()
                .map(|(index, z0)| Port::single_ended(index + 1, &format!("p{}", index + 1), *z0))
                .collect(),
        );
        for matrix in matrices {
            result.add(matrix);
        }
        result
    }

    fn correlation(frequency: Value, values: [[Complex64; 2]; 2]) -> PortNoiseCorrelationResult {
        PortNoiseCorrelationResult {
            frequency,
            current_correlation: values.map(|row| row.to_vec()).to_vec(),
        }
    }

    /// A 50 Ω series resistor measured in a 50 Ω system: the Norton covariance
    /// of a single conductance, and the S-matrix that admittance produces. The
    /// scattering data therefore round-trips back to the admittance the noise
    /// data belongs to, which is exactly what the assembly relies on.
    fn resistive_two_port() -> (
        Vec<SParameterPort>,
        SParameterResult,
        Vec<PortNoiseCorrelationResult>,
        Value,
    ) {
        let impedances = [50.0, 50.0];
        let ports = vec![port(1, 50.0), port(2, 50.0)];
        let temperature = 300.15;
        let conductance = 1.0 / 50.0;
        let admittance = vec![
            vec![
                Complex64::new(conductance, 0.0),
                Complex64::new(-conductance, 0.0),
            ],
            vec![
                Complex64::new(-conductance, 0.0),
                Complex64::new(conductance, 0.0),
            ],
        ];
        let square = super::super::s_from_y(&admittance, &impedances)
            .expect("a series resistor has a scattering representation");
        let mut matrix = SMatrix::new(1.0e9, 2);
        for (row, entries) in square.iter().enumerate() {
            for (column, &value) in entries.iter().enumerate() {
                matrix.set(row + 1, column + 1, value);
            }
        }
        let power = 4.0 * crate::constants::K_BOLTZMANN * temperature * conductance;
        let points = vec![correlation(
            1.0e9,
            [
                [Complex64::new(power, 0.0), Complex64::new(-power, 0.0)],
                [Complex64::new(-power, 0.0), Complex64::new(power, 0.0)],
            ],
        )];
        (ports, sweep(&impedances, vec![matrix]), points, temperature)
    }

    #[test]
    fn a_two_port_sweep_keeps_its_points_and_derives_noise_parameters() {
        let (ports, scattering, points, temperature) = resistive_two_port();
        let expected = points[0].current_correlation[0][0];
        let assembly = assemble_port_noise(&ports, &scattering, points, temperature)
            .expect("physical two-port noise data assembles");
        assert_eq!(assembly.reference_temperature_kelvin, temperature);
        assert_eq!(assembly.points.len(), 1);
        assert_eq!(assembly.points[0].current_correlation[0][0], expected);
        let derived = assembly
            .two_port
            .expect("a two-port sweep derives standard noise parameters");
        assert_eq!(derived.len(), 1);
        assert!(derived[0].valid);
        // A 50 Ω series resistor in a 50 Ω system has Rn = R and NF = 3 dB.
        assert!((derived[0].noise_resistance - 50.0).abs() < 50.0 * 1.0e-9);
        assert!((derived[0].noise_factor - 2.0).abs() < 1.0e-9);
    }

    #[test]
    fn a_three_port_sweep_reports_no_two_port_parameters() {
        let impedances = [50.0, 50.0, 50.0];
        let ports = vec![port(1, 50.0), port(2, 50.0), port(3, 50.0)];
        let scattering = sweep(&impedances, vec![SMatrix::new(1.0e9, 3)]);
        let points = vec![PortNoiseCorrelationResult {
            frequency: 1.0e9,
            current_correlation: vec![vec![Complex64::ZERO; 3]; 3],
        }];
        let assembly = assemble_port_noise(&ports, &scattering, points, 300.15)
            .expect("a three-port correlation sweep is still evidence");
        assert!(assembly.two_port.is_none());
        assert_eq!(assembly.points.len(), 1);
    }

    #[test]
    fn undefined_two_port_parameters_fail_the_sweep_instead_of_publishing_nan() {
        let (ports, scattering, mut points, temperature) = resistive_two_port();
        // A covariance matrix that is not positive semidefinite has no
        // physical noise-parameter solution.
        points[0].current_correlation[1][1] = Complex64::new(-1.0, 0.0);
        let error = assemble_port_noise(&ports, &scattering, points, temperature)
            .expect_err("an unphysical covariance must not publish a noise figure");
        assert!(matches!(
            error,
            PortNoiseAssemblyError::UndefinedTwoPortParameters { .. }
        ));
    }

    #[test]
    fn a_short_or_misaligned_solve_is_refused() {
        let (ports, scattering, points, temperature) = resistive_two_port();

        assert_eq!(
            refused(assemble_port_noise(
                &[],
                &scattering,
                points.clone(),
                temperature
            )),
            PortNoiseAssemblyError::NoPorts
        );
        assert_eq!(
            refused(assemble_port_noise(
                &ports,
                &scattering,
                Vec::new(),
                temperature
            )),
            PortNoiseAssemblyError::NoFrequencies
        );
        assert_eq!(
            refused(assemble_port_noise(
                &ports,
                &sweep(&[50.0, 50.0], Vec::new()),
                points.clone(),
                temperature
            )),
            PortNoiseAssemblyError::NoFrequencies
        );
        assert!(matches!(
            refused(assemble_port_noise(
                &ports,
                &scattering,
                points.clone(),
                0.0
            )),
            PortNoiseAssemblyError::Temperature { .. }
        ));

        let mut doubled = points.clone();
        doubled.push(points[0].clone());
        assert_eq!(
            refused(assemble_port_noise(
                &ports,
                &scattering,
                doubled,
                temperature
            )),
            PortNoiseAssemblyError::ScatteringMismatch {
                scattering: 1,
                noise: 2
            }
        );

        let three_ports = vec![port(1, 50.0), port(2, 50.0), port(3, 50.0)];
        assert_eq!(
            refused(assemble_port_noise(
                &three_ports,
                &scattering,
                points.clone(),
                temperature
            )),
            PortNoiseAssemblyError::PortCountMismatch {
                ports: 3,
                scattering_ports: 2
            }
        );

        let mut moved = points.clone();
        moved[0].frequency = 2.0e9;
        assert!(matches!(
            refused(assemble_port_noise(&ports, &scattering, moved, temperature)),
            PortNoiseAssemblyError::FrequencyMismatch { index: 0, .. }
        ));

        let mut malformed = points;
        malformed[0].current_correlation[1].pop();
        assert!(matches!(
            refused(assemble_port_noise(
                &ports,
                &scattering,
                malformed,
                temperature
            )),
            PortNoiseAssemblyError::MalformedCovariance { index: 0, ports: 2 }
        ));
    }

    #[test]
    fn assembly_observes_the_abort_signal() {
        let (ports, scattering, points, temperature) = resistive_two_port();
        assert_eq!(
            refused(assemble_port_noise_with_abort(
                &ports,
                &scattering,
                points,
                temperature,
                &ImmediateAbort,
            )),
            PortNoiseAssemblyError::Aborted
        );
    }
}
