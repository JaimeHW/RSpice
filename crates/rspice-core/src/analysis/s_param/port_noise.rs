//! Assembly of a `.SP donoise` port-noise sweep from the per-frequency
//! correlation solve.
//!
//! Folding the per-frequency covariance points into a `[port][port][point]`
//! cube, checking that the returned sweep is the sweep that was requested, and
//! deriving the standard two-port noise parameters from the measured S-matrix
//! is one semantic operation. It used to be written twice — once in the CLI
//! and once in the Python bindings — and the two copies disagreed about the
//! one thing that matters: what to do when the two-port derivation has no
//! physical solution. One surface refused the sweep, the other published NaNs
//! next to a validity flag a caller could ignore.
//!
//! This module is that operation, with one validity policy: an undefined
//! two-port parameter set is a typed failure naming the frequency it occurred
//! at. A noise figure that is quietly wrong is worse than one that is visibly
//! absent, because it will be believed.

use thiserror::Error;

use super::{TwoPortNoise, derive_two_port_noise, y_from_s};
use crate::abort_signal::AbortSignal;
use crate::analysis::noise::PortNoiseCorrelationResult;
use crate::{Complex64, NoAbort, Value};

/// Number of ports for which the standard noise parameters are defined.
const TWO_PORT: usize = 2;

/// A fully assembled `.SP donoise` result for one frequency sweep.
#[derive(Debug, Clone, PartialEq)]
pub struct PortNoiseSweep {
    /// Circuit temperature the device noise was evaluated at, in kelvin.
    pub reference_temperature_kelvin: Value,
    /// Complex Norton port-current covariance in A²/Hz, indexed
    /// `[output_port][input_port][frequency_point]`.
    pub current_correlation: Vec<Vec<Vec<Complex64>>>,
    /// Standard two-port noise parameters, one entry per frequency point.
    ///
    /// `None` for any port count other than two: `Rn`, `F`, `Fmin` and `Sopt`
    /// are two-port quantities, and deriving them from a sub-matrix of a
    /// larger network would describe a different device.
    pub two_port_parameters: Option<Vec<TwoPortNoise>>,
}

/// Why a port-noise sweep could not be assembled.
///
/// Every variant identifies the offending point or port count, so a frontend
/// can translate it into its native diagnostic without parsing a message.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum PortNoiseAssemblyError {
    #[error("port-noise assembly was cancelled")]
    Aborted,
    #[error("port-noise sweep requires at least one port")]
    NoPorts,
    #[error("port-noise sweep requires at least one frequency")]
    NoFrequencies,
    #[error("port-noise solve returned {returned} points for {requested} requested frequencies")]
    PointCount { returned: usize, requested: usize },
    #[error("port-noise point {point} is at {actual:.16e} Hz, expected {expected:.16e} Hz")]
    FrequencyMismatch {
        point: usize,
        actual: Value,
        expected: Value,
    },
    #[error("port-noise point {point} returned a malformed covariance matrix for {ports} ports")]
    MalformedCovariance { point: usize, ports: usize },
    #[error("port-noise sweep has {ports} ports but {impedances} reference impedances")]
    ImpedanceCount { ports: usize, impedances: usize },
    #[error(
        "port-noise sweep needs a {ports}x{ports} S-matrix over {points} points, got {rows} rows"
    )]
    ScatteringRows {
        ports: usize,
        points: usize,
        rows: usize,
    },
    #[error(
        "port-noise S-matrix row {row} has {columns} columns over {points} points; expected {ports} columns of {points} values"
    )]
    ScatteringRow {
        row: usize,
        columns: usize,
        ports: usize,
        points: usize,
    },
    #[error("port-noise sweep needs a positive finite temperature, got {temperature}")]
    Temperature { temperature: Value },
    #[error("port-noise admittance conversion failed at {frequency:.16e} Hz: {reason}")]
    Admittance { frequency: Value, reason: String },
    #[error(
        "two-port noise parameters are undefined at {frequency:.16e} Hz; the admittance/noise data do not support a physical finite solution"
    )]
    UndefinedTwoPort { frequency: Value },
}

/// Assemble a port-noise sweep, without cancellation.
pub fn assemble_port_noise(
    points: &[PortNoiseCorrelationResult],
    frequencies: &[Value],
    scattering: &[Vec<Vec<Complex64>>],
    reference_impedances: &[Value],
    temperature: Value,
) -> Result<PortNoiseSweep, PortNoiseAssemblyError> {
    assemble_port_noise_with_abort(
        points,
        frequencies,
        scattering,
        reference_impedances,
        temperature,
        &NoAbort,
    )
}

/// Assemble a port-noise sweep with cooperative cancellation.
///
/// `scattering` is the measured S-matrix indexed `[row][column][point]`, the
/// shape [`super::extract_s_matrix`] produces. `reference_impedances` is one
/// impedance per port. The returned cube is indexed
/// `[output_port][input_port][point]`.
pub fn assemble_port_noise_with_abort(
    points: &[PortNoiseCorrelationResult],
    frequencies: &[Value],
    scattering: &[Vec<Vec<Complex64>>],
    reference_impedances: &[Value],
    temperature: Value,
    abort: &dyn AbortSignal,
) -> Result<PortNoiseSweep, PortNoiseAssemblyError> {
    if abort.is_aborted() {
        return Err(PortNoiseAssemblyError::Aborted);
    }
    let port_count = reference_impedances.len();
    if port_count == 0 {
        return Err(PortNoiseAssemblyError::NoPorts);
    }
    if frequencies.is_empty() {
        return Err(PortNoiseAssemblyError::NoFrequencies);
    }
    if !temperature.is_finite() || temperature <= 0.0 {
        return Err(PortNoiseAssemblyError::Temperature { temperature });
    }
    let point_count = frequencies.len();
    if points.len() != point_count {
        return Err(PortNoiseAssemblyError::PointCount {
            returned: points.len(),
            requested: point_count,
        });
    }
    if scattering.len() != port_count {
        return Err(PortNoiseAssemblyError::ScatteringRows {
            ports: port_count,
            points: point_count,
            rows: scattering.len(),
        });
    }
    for (row_index, row) in scattering.iter().enumerate() {
        if row.len() != port_count || row.iter().any(|column| column.len() != point_count) {
            return Err(PortNoiseAssemblyError::ScatteringRow {
                row: row_index,
                columns: row.len(),
                ports: port_count,
                points: point_count,
            });
        }
    }

    let mut current_correlation =
        vec![vec![vec![Complex64::ZERO; point_count]; port_count]; port_count];
    for (point_index, (expected_frequency, point)) in frequencies.iter().zip(points).enumerate() {
        if point_index.is_multiple_of(64) && abort.is_aborted() {
            return Err(PortNoiseAssemblyError::Aborted);
        }
        // The solve is asked for exactly the requested grid; the tolerance
        // only absorbs a round trip through the solver's own bookkeeping, not
        // a genuinely different frequency.
        let tolerance = expected_frequency.abs().max(1.0) * Value::EPSILON * 64.0;
        if (point.frequency - expected_frequency).abs() > tolerance {
            return Err(PortNoiseAssemblyError::FrequencyMismatch {
                point: point_index + 1,
                actual: point.frequency,
                expected: *expected_frequency,
            });
        }
        if point.current_correlation.len() != port_count
            || point
                .current_correlation
                .iter()
                .any(|row| row.len() != port_count)
        {
            return Err(PortNoiseAssemblyError::MalformedCovariance {
                point: point_index + 1,
                ports: port_count,
            });
        }
        for (row, correlations) in current_correlation.iter_mut().enumerate() {
            for (column, values) in correlations.iter_mut().enumerate() {
                values[point_index] = point.current_correlation[row][column];
            }
        }
    }

    let two_port_parameters = if port_count == TWO_PORT {
        Some(derive_two_port_sweep(
            frequencies,
            scattering,
            &current_correlation,
            reference_impedances,
            temperature,
            abort,
        )?)
    } else {
        None
    };

    Ok(PortNoiseSweep {
        reference_temperature_kelvin: temperature,
        current_correlation,
        two_port_parameters,
    })
}

fn derive_two_port_sweep(
    frequencies: &[Value],
    scattering: &[Vec<Vec<Complex64>>],
    current_correlation: &[Vec<Vec<Complex64>>],
    reference_impedances: &[Value],
    temperature: Value,
    abort: &dyn AbortSignal,
) -> Result<Vec<TwoPortNoise>, PortNoiseAssemblyError> {
    if reference_impedances.len() != TWO_PORT {
        return Err(PortNoiseAssemblyError::ImpedanceCount {
            ports: TWO_PORT,
            impedances: reference_impedances.len(),
        });
    }
    let mut derived = Vec::with_capacity(frequencies.len());
    for (point_index, frequency) in frequencies.iter().enumerate() {
        if point_index.is_multiple_of(16) && abort.is_aborted() {
            return Err(PortNoiseAssemblyError::Aborted);
        }
        // Derived from the S-matrix just measured rather than read off the
        // port branch currents: behind a Thevenin port that current flows
        // through the reference resistor and is not the admittance term.
        let s = point_matrix(scattering, point_index);
        let admittance = y_from_s(&s, reference_impedances).map_err(|error| {
            PortNoiseAssemblyError::Admittance {
                frequency: *frequency,
                reason: error.to_string(),
            }
        })?;
        let covariance = point_matrix(current_correlation, point_index);
        let parameters = derive_two_port_noise(
            &admittance,
            &covariance,
            reference_impedances[0],
            temperature,
        );
        if !parameters.valid {
            return Err(PortNoiseAssemblyError::UndefinedTwoPort {
                frequency: *frequency,
            });
        }
        derived.push(parameters);
    }
    Ok(derived)
}

/// Slice one frequency point out of a `[row][column][point]` cube.
///
/// Shapes are validated by the caller before this runs, so the indexing is a
/// proven invariant rather than an assumption about solver output.
fn point_matrix(cube: &[Vec<Vec<Complex64>>], point_index: usize) -> Vec<Vec<Complex64>> {
    cube.iter()
        .map(|row| {
            row.iter()
                .map(|values| values[point_index])
                .collect::<Vec<_>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::ImmediateAbort;

    fn point(frequency: Value, values: [[Complex64; 2]; 2]) -> PortNoiseCorrelationResult {
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
        Vec<Value>,
        Vec<Vec<Vec<Complex64>>>,
        Vec<PortNoiseCorrelationResult>,
        Vec<Value>,
        Value,
    ) {
        let frequencies = vec![1.0e9];
        let impedances = vec![50.0, 50.0];
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
        let matrix = super::super::s_from_y(&admittance, &impedances)
            .expect("a series resistor has a scattering representation");
        let scattering = (0..2)
            .map(|row| (0..2).map(|column| vec![matrix[row][column]]).collect())
            .collect::<Vec<Vec<Vec<Complex64>>>>();
        let power = 4.0 * crate::constants::K_BOLTZMANN * temperature * conductance;
        let points = vec![point(
            1.0e9,
            [
                [Complex64::new(power, 0.0), Complex64::new(-power, 0.0)],
                [Complex64::new(-power, 0.0), Complex64::new(power, 0.0)],
            ],
        )];
        (frequencies, scattering, points, impedances, temperature)
    }

    #[test]
    fn a_two_port_sweep_folds_into_a_port_major_cube() {
        let (frequencies, scattering, points, impedances, temperature) = resistive_two_port();
        let sweep =
            assemble_port_noise(&points, &frequencies, &scattering, &impedances, temperature)
                .expect("physical two-port noise data assembles");
        assert_eq!(sweep.reference_temperature_kelvin, temperature);
        assert_eq!(sweep.current_correlation.len(), 2);
        assert_eq!(sweep.current_correlation[0][0].len(), 1);
        assert_eq!(
            sweep.current_correlation[0][0][0],
            points[0].current_correlation[0][0]
        );
        let derived = sweep
            .two_port_parameters
            .expect("a two-port sweep derives standard noise parameters");
        assert_eq!(derived.len(), 1);
        assert!(derived[0].valid);
        // A 50 Ω series resistor in a 50 Ω system has Rn = R and NF = 3 dB.
        assert!((derived[0].noise_resistance - 50.0).abs() < 50.0 * 1.0e-9);
        assert!((derived[0].noise_factor - 2.0).abs() < 1.0e-9);
    }

    #[test]
    fn a_three_port_sweep_reports_no_two_port_parameters() {
        let frequencies = vec![1.0e9];
        let impedances = vec![50.0, 50.0, 50.0];
        let scattering = vec![vec![vec![Complex64::ZERO]; 3]; 3];
        let points = vec![PortNoiseCorrelationResult {
            frequency: 1.0e9,
            current_correlation: vec![vec![Complex64::ZERO; 3]; 3],
        }];
        let sweep =
            assemble_port_noise(&points, &frequencies, &scattering, &impedances, 300.15).unwrap();
        assert!(sweep.two_port_parameters.is_none());
    }

    #[test]
    fn undefined_two_port_parameters_fail_the_sweep_instead_of_publishing_nan() {
        let (frequencies, scattering, mut points, impedances, temperature) = resistive_two_port();
        // A covariance matrix that is not positive semidefinite has no
        // physical noise-parameter solution.
        points[0].current_correlation[1][1] = Complex64::new(-1.0, 0.0);
        let error =
            assemble_port_noise(&points, &frequencies, &scattering, &impedances, temperature)
                .expect_err("an unphysical covariance must not publish a noise figure");
        assert!(matches!(
            error,
            PortNoiseAssemblyError::UndefinedTwoPort { .. }
        ));
    }

    #[test]
    fn a_short_or_misaligned_solve_is_refused() {
        let (frequencies, scattering, points, impedances, temperature) = resistive_two_port();
        assert_eq!(
            assemble_port_noise(&[], &frequencies, &scattering, &impedances, temperature),
            Err(PortNoiseAssemblyError::PointCount {
                returned: 0,
                requested: 1
            })
        );

        let mut moved = points.clone();
        moved[0].frequency = 2.0e9;
        assert!(matches!(
            assemble_port_noise(&moved, &frequencies, &scattering, &impedances, temperature),
            Err(PortNoiseAssemblyError::FrequencyMismatch { point: 1, .. })
        ));

        let mut malformed = points.clone();
        malformed[0].current_correlation[1].pop();
        assert!(matches!(
            assemble_port_noise(
                &malformed,
                &frequencies,
                &scattering,
                &impedances,
                temperature
            ),
            Err(PortNoiseAssemblyError::MalformedCovariance { point: 1, ports: 2 })
        ));

        let truncated = vec![scattering[0].clone()];
        assert!(matches!(
            assemble_port_noise(&points, &frequencies, &truncated, &impedances, temperature),
            Err(PortNoiseAssemblyError::ScatteringRows { rows: 1, .. })
        ));

        assert!(matches!(
            assemble_port_noise(&points, &frequencies, &scattering, &impedances, 0.0),
            Err(PortNoiseAssemblyError::Temperature { .. })
        ));
    }

    #[test]
    fn assembly_observes_the_abort_signal() {
        let (frequencies, scattering, points, impedances, temperature) = resistive_two_port();
        assert_eq!(
            assemble_port_noise_with_abort(
                &points,
                &frequencies,
                &scattering,
                &impedances,
                temperature,
                &ImmediateAbort,
            ),
            Err(PortNoiseAssemblyError::Aborted)
        );
    }
}
