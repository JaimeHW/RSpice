//! Assembly of one `.SP DONOISE` run's port-noise evidence.
//!
//! The port-noise solver returns a per-frequency current-noise covariance
//! matrix. Turning that into publishable evidence means proving it lines up
//! with the scattering sweep it belongs to, and — for a two-port network —
//! converting scattering to admittance and deriving the standard noise
//! parameters at every frequency.
//!
//! That assembly used to live in the CLI and the Python bindings, in two
//! copies that agreed by inspection. It is analysis semantics: which frequency
//! a covariance point belongs to, and whether a derived noise figure is
//! physical, are not presentation decisions.

use crate::analysis::noise::PortNoiseCorrelationResult;
use crate::{Complex64, Value};

use super::matrix::SParameterResult;
use super::network::y_from_s;
use super::noise_params::{TwoPortNoise, derive_two_port_noise};
use super::ports::SParameterPort;

/// Why a port-noise sweep could not be assembled into publishable evidence.
#[derive(Debug, Clone, PartialEq)]
pub enum PortNoiseAssemblyError {
    /// The solver returned a different number of points than were swept.
    PointCount { returned: usize, requested: usize },
    /// A covariance point is not at the frequency it was requested for.
    FrequencyMismatch {
        index: usize,
        expected: Value,
        actual: Value,
    },
    /// A covariance matrix is not square and `N x N` for the declared ports.
    MalformedCovariance { index: usize, ports: usize },
    /// The scattering sweep and the noise sweep describe different grids.
    ScatteringMismatch { scattering: usize, noise: usize },
    /// Scattering could not be converted to admittance at one frequency.
    Admittance { frequency: Value, detail: String },
    /// The admittance and covariance at one frequency do not support a
    /// physical two-port noise solution.
    ///
    /// Reported rather than published with `valid = false`: a noise figure
    /// that is present but meaningless will be believed.
    UndefinedTwoPortParameters { frequency: Value },
}

impl std::fmt::Display for PortNoiseAssemblyError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PointCount {
                returned,
                requested,
            } => write!(
                formatter,
                "port-noise solve returned {returned} points for {requested} requested frequencies"
            ),
            Self::FrequencyMismatch {
                index,
                expected,
                actual,
            } => write!(
                formatter,
                "port-noise point {} is at {actual:.16e} Hz, expected {expected:.16e} Hz",
                index + 1
            ),
            Self::MalformedCovariance { index, ports } => write!(
                formatter,
                "port-noise point {} returned a malformed covariance matrix for {ports} ports",
                index + 1
            ),
            Self::ScatteringMismatch { scattering, noise } => write!(
                formatter,
                "the scattering sweep has {scattering} frequencies and the noise sweep {noise}"
            ),
            Self::Admittance { frequency, detail } => write!(
                formatter,
                "scattering could not be converted to admittance at {frequency:.16e} Hz: {detail}"
            ),
            Self::UndefinedTwoPortParameters { frequency } => write!(
                formatter,
                "two-port noise parameters are undefined at {frequency:.16e} Hz; the \
                 admittance/noise data do not support a physical finite solution"
            ),
        }
    }
}

impl std::error::Error for PortNoiseAssemblyError {}

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
    pub two_port: Option<Vec<TwoPortNoise>>,
}

/// Validate one port-noise sweep against its scattering sweep and derive the
/// two-port noise parameters when the network has exactly two ports.
///
/// `points` must already be in swept-frequency order, which is the order the
/// port-noise solver returns them in.
pub fn assemble_port_noise(
    ports: &[SParameterPort],
    scattering: &SParameterResult,
    points: Vec<PortNoiseCorrelationResult>,
    temperature: Value,
) -> Result<PortNoiseAssembly, PortNoiseAssemblyError> {
    let count = ports.len();
    if scattering.data.len() != points.len() {
        return Err(PortNoiseAssemblyError::ScatteringMismatch {
            scattering: scattering.data.len(),
            noise: points.len(),
        });
    }
    for (index, (matrix, point)) in scattering.data.iter().zip(&points).enumerate() {
        let tolerance = matrix.frequency.abs().max(1.0) * f64::EPSILON * 64.0;
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

    let two_port = if count == 2 {
        let reference_impedances = ports.iter().map(|port| port.z0).collect::<Vec<_>>();
        // Two ports were counted just above, so port 1 is present; its
        // reference impedance is the one the noise figure is defined against.
        let Some(input_reference) = ports.first().map(|port| port.z0) else {
            return Err(PortNoiseAssemblyError::MalformedCovariance {
                index: 0,
                ports: count,
            });
        };
        let mut derived = Vec::with_capacity(points.len());
        for (matrix, point) in scattering.data.iter().zip(&points) {
            let square = (1..=count)
                .map(|row| (1..=count).map(|column| matrix.get(row, column)).collect())
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
        Some(derived)
    } else {
        None
    };

    Ok(PortNoiseAssembly {
        reference_temperature_kelvin: temperature,
        points,
        two_port,
    })
}
