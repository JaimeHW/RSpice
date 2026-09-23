//! Intrinsic periodic noise waves at the authored power-wave references.

use super::*;
use crate::analysis::harmonic_balance::{PeriodicNoiseOutput, PeriodicNoiseProjection};

/// E[c_r conj(c_c)] in W/Hz, excluding the external port terminations.
#[derive(Debug, Clone)]
pub struct PspNoiseCorrelation {
    /// The swept offset in hertz. Channel k is at offset + k * fundamental.
    pub frequency: Value,
    /// Square row-major covariance, with the same physical-port-major,
    /// ascending-sideband channel order as the associated scattering matrix.
    pub wave_correlation: Vec<Vec<Complex64>>,
}

// All arguments belong to the already-authenticated PreparedPsp. Keeping the
// projection here lets the lower solver certify each complete observation.
#[allow(clippy::too_many_arguments)]
pub(super) fn solve_noise_waves(
    solver: &mut HbSolver,
    state: &HbSolverState,
    config: &PacConfig,
    scattering: &SMatrix,
    nodes: &[(Option<usize>, Option<usize>)],
    physical: &[Value],
    sources: &[PeriodicNoiseSource],
    limits: &crate::ResourceLimits,
    abort: &dyn AbortSignal,
    authored: &[Value],
) -> Result<PspNoiseCorrelation, SimulationError> {
    let channels = physical.len();
    let sidebands = config.num_sidebands();
    let mut observations = Vec::with_capacity(channels);
    for row in 0..channels {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut terms = Vec::new();
        for column in 0..channels {
            // b = S a + c; a'=A a+B b and b'=B a+A b imply
            // c'=(A-S'B)c. At the physical matched loads c=V/sqrt(R).
            // Combine the diagonal as (1-S') and (1+S') to avoid
            // subtracting two large terms for an ideal open/short.
            let s = scattering.get(row + 1, column + 1);
            let root_z = authored[column].sqrt();
            let weight = if physical[column] == authored[column] {
                if row == column {
                    Complex64::new(1.0 / root_z, 0.0)
                } else {
                    Complex64::ZERO
                }
            } else if row == column {
                (Complex64::ONE - s) * (0.5 / root_z)
                    + (Complex64::ONE + s) * (0.5 * root_z / physical[column])
            } else {
                s * (0.5 * root_z / physical[column] - 0.5 / root_z)
            };
            if !weight.re.is_finite() || !weight.im.is_finite() {
                return Err(SimulationError::Circuit(format!(
                    "PSP noise-wave projection overflowed at channel ({}, {})",
                    row + 1,
                    column + 1
                )));
            }
            if weight == Complex64::ZERO {
                continue;
            }
            let (node_pos, node_neg) = nodes[column / sidebands];
            let sideband =
                i32::try_from(i64::from(config.sideband_min) + (column % sidebands) as i64)
                    .map_err(|_| {
                        SimulationError::Circuit("PSP noise output sideband overflowed".into())
                    })?;
            terms.push((
                PeriodicNoiseOutput {
                    node_pos,
                    node_neg,
                    sideband,
                },
                weight,
            ));
        }
        observations.push(PeriodicNoiseProjection {
            terms,
            phase_response: None,
        });
    }
    let entries = channels.checked_mul(channels).ok_or_else(|| {
        SimulationError::Circuit("PSP covariance dimensions overflow this platform".into())
    })?;
    let mut sums = vec![Complex64::ZERO; entries];
    let mut corrections = vec![Complex64::ZERO; entries];
    solver
        .solve_periodic_noise_projected_correlations_with_adjoints_each(
            state,
            PeriodicSidebandWindow {
                offset_hz: scattering.frequency,
                sideband_min: config.sideband_min,
                sideband_max: config.sideband_max,
            },
            &observations,
            sources,
            None,
            limits,
            abort,
            |_, covariance| {
                for (index, &value) in covariance.iter().enumerate() {
                    if index.is_multiple_of(256) && abort.is_aborted() {
                        return Err(HbError::Aborted);
                    }
                    crate::numerics::compensated_add(
                        &mut sums[index].re,
                        &mut corrections[index].re,
                        value.re,
                    );
                    crate::numerics::compensated_add(
                        &mut sums[index].im,
                        &mut corrections[index].im,
                        value.im,
                    );
                    if !sums[index].re.is_finite()
                        || !sums[index].im.is_finite()
                        || !corrections[index].re.is_finite()
                        || !corrections[index].im.is_finite()
                    {
                        return Err(HbError::InvalidCircuit(
                            "PSP noise covariance accumulation overflowed".into(),
                        ));
                    }
                }
                Ok(())
            },
        )
        .map_err(|error| match error {
            HbError::Aborted => SimulationError::Aborted,
            error => SimulationError::Circuit(format!("PSP correlated noise failed: {error}")),
        })?;
    for (sum, correction) in sums.iter_mut().zip(corrections) {
        *sum += correction;
        if !sum.re.is_finite() || !sum.im.is_finite() {
            return Err(SimulationError::Circuit(
                "PSP completed noise covariance overflowed".into(),
            ));
        }
    }
    let wave_correlation = sums
        .chunks_exact(channels)
        .map(<[Complex64]>::to_vec)
        .collect();
    Ok(PspNoiseCorrelation {
        frequency: scattering.frequency,
        wave_correlation,
    })
}
