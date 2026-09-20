//! Joint periodic voltage-noise covariance with certified adjoint solves.
//!
//! C[r,c] = E[V_r conj(V_c)] retains complex port and sideband correlations.
//! Independent device mechanisms are streamed separately so callers can
//! exclude physical port terminations without subtracting large noise totals.

use super::*;

/// One differential voltage observed at a signed output sideband.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PeriodicNoiseOutput {
    pub node_pos: Option<usize>,
    pub node_neg: Option<usize>,
    pub sideband: i32,
}

/// Colored covariance without materializing either source density or transfer
/// product before their exponents cancel. The phase is left intact.
pub(super) fn scaled_flicker_cross_term(
    left: ScaledComplex,
    right: ScaledComplex,
    coefficient: Value,
    coefficient_binary_exponent: i32,
    frequency: Value,
    exponent: Value,
) -> Result<ScaledComplex, &'static str> {
    validate_scaled_complex(left)?;
    validate_scaled_complex(right)?;
    if !coefficient.is_finite()
        || coefficient < 0.0
        || !frequency.is_finite()
        || frequency < 0.0
        || !exponent.is_finite()
    {
        return Err("a flicker-covariance factor is invalid");
    }
    if left.is_zero() || right.is_zero() || coefficient == 0.0 {
        return Ok(ScaledComplex::ZERO);
    }
    if frequency == 0.0 {
        return if exponent < 0.0 {
            Ok(ScaledComplex::ZERO)
        } else if exponent == 0.0 {
            scaled_flicker_cross_term(
                left,
                right,
                coefficient,
                coefficient_binary_exponent,
                1.0,
                exponent,
            )
        } else {
            Err("positive-exponent flicker density is singular at zero frequency")
        };
    }
    let cross = scaled_complex_product3(
        left.mantissa,
        right.mantissa.conj(),
        Complex64::new(1.0, 0.0),
        0,
    )?;
    let gain_power =
        i64::from(left.exponent) + i64::from(right.exponent) + i64::from(cross.exponent);
    let frequency_power = frequency.powf(exponent);
    let (mantissa, power) = if frequency_power.is_normal() {
        let (mantissa, power) =
            crate::numerics::product_binary_normalization(&[coefficient], &[frequency_power]);
        let power = i64::from(power) + i64::from(coefficient_binary_exponent) + gain_power;
        (
            mantissa,
            i32::try_from(power)
                .map_err(|_| "the flicker covariance exceeds the retained binary exponent range")?,
        )
    } else {
        let coefficient_exponent = libm::ilogb(coefficient);
        crate::numerics::power_product_binary_normalization(
            libm::scalbn(coefficient, -coefficient_exponent),
            coefficient_binary_exponent,
            &[
                (2.0, Value::from(coefficient_exponent) + gain_power as Value),
                (frequency, -exponent),
            ],
        )
    };
    if !mantissa.is_finite() || mantissa <= 0.0 {
        return Err("the nonzero flicker covariance exceeds the retained binary exponent range");
    }
    scaled_complex_product3(
        cross.mantissa,
        Complex64::new(mantissa, 0.0),
        Complex64::new(1.0, 0.0),
        power,
    )
}

/// The same two-pass, compensated accumulation is used for PSD and complex
/// covariance; only diagonal entries must be real and nonnegative.
fn source_covariance(
    left: &[Complex64],
    right: &[Complex64],
    source: &PeriodicNoiseSource,
    window: PeriodicSidebandWindow,
    fundamental_hz: Value,
) -> Result<(Complex64, Value), HbError> {
    let visit = |visitor: &mut dyn FnMut(ScaledComplex) -> Result<(), &'static str>| {
        visit_periodic_noise_terms(
            left,
            right,
            source,
            window.sideband_min,
            window.offset_hz,
            fundamental_hz,
            visitor,
        )
    };
    let mut common_exponent = None;
    let term_count = visit(&mut |term| {
        validate_scaled_complex(term)?;
        if !term.is_zero() {
            common_exponent = Some(
                common_exponent.map_or(term.exponent, |current: i32| current.max(term.exponent)),
            );
        }
        Ok(())
    })?;
    let (mut value, absolute_sum) = if let Some(exponent) = common_exponent {
        let mut accumulator = ScaledComplexAccumulator::new(exponent);
        visit(&mut |term| accumulator.add(term))?;
        accumulator.finish().map_err(|reason| {
            HbError::InvalidCircuit(format!(
                "pnoise source '{}' noise accumulation is invalid: {reason}",
                source.name
            ))
        })?
    } else {
        (Complex64::ZERO, 0.0)
    };
    let roundoff = absolute_sum * Value::EPSILON * 32.0 * term_count.max(1) as Value;
    if !roundoff.is_finite() {
        return Err(HbError::InvalidCircuit(format!(
            "pnoise source '{}' roundoff bound is non-finite",
            source.name
        )));
    }
    if std::ptr::eq(left, right) {
        if value.im.abs() > roundoff {
            return Err(HbError::InvalidCircuit(format!(
                "pnoise source '{}' produced a non-Hermitian density ({:+.6e}{:+.6e}j)",
                source.name, value.re, value.im
            )));
        }
        if value.re < -roundoff {
            return Err(HbError::InvalidCircuit(format!(
                "pnoise source '{}' produced a negative output-noise density {:.6e}",
                source.name, value.re
            )));
        }
        value = Complex64::new(value.re.max(0.0), 0.0);
    }
    Ok((value, roundoff))
}

impl HbSolver {
    /// Stream each source's row-major complex voltage covariance (V²/Hz).
    /// Output order is caller-owned; source indices match `sources`. Shares
    /// the conversion operator and preconditioner across all output adjoints.
    /// The callback can cancel and must not publish a partial sweep as complete.
    pub(crate) fn solve_periodic_noise_correlations_each(
        &mut self,
        state: &HbSolverState,
        window: PeriodicSidebandWindow,
        outputs: &[PeriodicNoiseOutput],
        sources: &[PeriodicNoiseSource],
        abort: &dyn AbortSignal,
        mut consume: impl FnMut(usize, &[Complex64]) -> Result<(), HbError>,
    ) -> Result<(), HbError> {
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        let PeriodicSidebandWindow {
            offset_hz,
            sideband_min,
            sideband_max,
        } = window;
        let n = self.num_nodes;
        if !offset_hz.is_finite() || offset_hz < 0.0 {
            return Err(HbError::InvalidCircuit(format!(
                "pnoise offset frequency must be finite and non-negative, got {offset_hz}"
            )));
        }
        if outputs.is_empty() {
            return Err(HbError::InvalidCircuit(
                "pnoise requires at least one output".into(),
            ));
        }
        validate_periodic_state(state, n, "pnoise")?;
        for output in outputs {
            if output.node_pos.is_some_and(|node| node >= n) {
                return Err(HbError::InvalidCircuit(
                    "pnoise output node out of range".into(),
                ));
            }
            if output.node_neg.is_some_and(|node| node >= n) {
                return Err(HbError::InvalidCircuit(
                    "pnoise output reference node out of range".into(),
                ));
            }
            if output.sideband < sideband_min || output.sideband > sideband_max {
                return Err(HbError::InvalidCircuit(
                    "pnoise sideband range must include the selected output sideband".into(),
                ));
            }
        }
        for source in sources {
            let normalized_pos = (source.node_pos < n).then_some(source.node_pos);
            let normalized_neg = (source.node_neg < n).then_some(source.node_neg);
            if normalized_pos == normalized_neg {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has identical terminals and no effective injection",
                    source.name
                )));
            }
            if source.psd.is_empty() {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has no periodic PSD coefficients",
                    source.name
                )));
            }
            if source
                .psd
                .iter()
                .any(|coefficient| !coefficient.re.is_finite() || !coefficient.im.is_finite())
            {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' contains a non-finite periodic PSD coefficient",
                    source.name
                )));
            }
            let coefficient_scale = source
                .psd
                .iter()
                .map(|coefficient| coefficient.norm())
                .fold(0.0, Value::max);
            let dc_tolerance =
                coefficient_scale * Value::EPSILON * 32.0 * source.psd.len() as Value;
            let dc = source.psd[0];
            if !dc_tolerance.is_finite() || dc.re < -dc_tolerance || dc.im.abs() > dc_tolerance {
                return Err(HbError::InvalidCircuit(format!(
                    "pnoise source '{}' has an invalid DC PSD coefficient ({:+.6e}{:+.6e}j)",
                    source.name, dc.re, dc.im
                )));
            }
            if let Some(flicker) = &source.flicker {
                if !flicker.coefficient.is_finite()
                    || flicker.coefficient < 0.0
                    || !flicker.exponent.is_finite()
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "pnoise source '{}' has invalid flicker parameters ({}, {})",
                        source.name, flicker.coefficient, flicker.exponent
                    )));
                }
                if flicker.modulation.is_empty()
                    || flicker.modulation.len() > i32::MAX as usize
                    || flicker.modulation.iter().any(|&v| !complex_is_finite(v))
                    || flicker.modulation[0].im != 0.0
                {
                    return Err(HbError::InvalidCircuit(format!(
                        "pnoise source '{}' has an invalid flicker modulation spectrum (requires finite coefficients and real DC)",
                        source.name
                    )));
                }
            }
        }
        let num_unknowns = n
            .checked_add(self.periodic_mna_branches.len())
            .ok_or_else(|| {
                HbError::InvalidCircuit("pnoise node and branch count overflows usize".to_string())
            })?;
        let (s, size, span) =
            periodic_sideband_geometry("pnoise", num_unknowns, sideband_min, sideband_max)?;
        if size == 0 {
            return Err(HbError::InvalidCircuit(
                "pnoise requires at least one circuit unknown".to_string(),
            ));
        }

        let try_krylov =
            self.config.use_krylov || size >= super::super::krylov::KRYLOV_AUTO_THRESHOLD;
        let (spectra, cap_spectra) = if self.has_nonlinear_devices() {
            (
                self.conductance_spectra(state, span.max(self.num_harmonics))?,
                self.capacitance_spectra(state, span.max(self.num_harmonics))?,
            )
        } else {
            (Vec::new(), Vec::new())
        };
        let operator = PeriodicConversionOperator {
            num_nodes: n,
            num_sidebands: s,
            sideband_min,
            offset_hz,
            fundamental_hz: self.config.fundamental_freq,
            g_matrix: &self.periodic_g_matrix,
            c_matrix: &self.c_matrix,
            l_matrix: &self.l_matrix,
            mna_branches: &self.periodic_mna_branches,
            mna_static_entries: &self.exact_mna_static_entries,
            mna_inductance_entries: &self.exact_mna_inductance_entries,
            periodic_networks: &self.exact_periodic_networks,
            g_spectra: &spectra,
            c_spectra: &cap_spectra,
        };
        operator.validate("pnoise")?;

        let channels = outputs.len();
        let covariance_len = channels.checked_mul(channels).ok_or_else(|| {
            HbError::InvalidCircuit("pnoise covariance dimensions overflow this platform".into())
        })?;
        let adjoint_len = channels.checked_mul(size).ok_or_else(|| {
            HbError::InvalidCircuit("pnoise adjoint dimensions overflow this platform".into())
        })?;
        let mut adjoints = try_zeroed_complex_values(adjoint_len, "pnoise adjoints")?;
        let preconditioner = if try_krylov {
            Some(PeriodicPreconditioner::build(&operator, true)?)
        } else {
            None
        };
        let transpose = if try_krylov {
            None
        } else {
            Some(operator.to_dense_transpose())
        };
        for (channel, output) in outputs.iter().enumerate() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            let band = usize::try_from(i64::from(output.sideband) - i64::from(sideband_min))
                .map_err(|_| {
                    HbError::InvalidCircuit(
                        "pnoise output-sideband index exceeds this platform".into(),
                    )
                })?;
            let mut rhs = try_zeroed_complex_values(size, "pnoise adjoint RHS")?;
            if let Some(node) = output.node_pos {
                rhs[node * s + band] += Complex64::new(1.0, 0.0);
            }
            if let Some(node) = output.node_neg {
                rhs[node * s + band] -= Complex64::new(1.0, 0.0);
            }
            let solution = if let Some(preconditioner) = &preconditioner {
                let restart =
                    super::super::krylov::bounded_gmres_restart(self.config.gmres_restart, size);
                let outcome = super::super::krylov::gmres(
                    &|input| operator.apply_transpose(input),
                    preconditioner,
                    &rhs,
                    restart,
                    6,
                );
                self.qualify_periodic_noise_adjoint(&operator, &rhs, outcome)?
            } else {
                self.solve_complex_linear_system(
                    transpose.as_ref().ok_or_else(|| {
                        HbError::InvalidCircuit("pnoise direct adjoint matrix is missing".into())
                    })?,
                    &rhs,
                )?
            };
            adjoints[channel * size..(channel + 1) * size].copy_from_slice(&solution);
        }
        let mut covariance = try_zeroed_complex_values(covariance_len, "pnoise covariance")?;
        let gain_len = channels.checked_mul(s).ok_or_else(|| {
            HbError::InvalidCircuit("pnoise gain dimensions overflow this platform".into())
        })?;
        let mut gains = try_zeroed_complex_values(gain_len, "pnoise source gains")?;
        let mut diagonal_roundoff = Vec::new();
        diagonal_roundoff
            .try_reserve_exact(channels)
            .map_err(|error| {
                HbError::InvalidCircuit(format!("pnoise roundoff allocation failed: {error}"))
            })?;
        diagonal_roundoff.resize(channels, 0.0);
        for (source_index, source) in sources.iter().enumerate() {
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            for channel in 0..channels {
                for band in 0..s {
                    let adjoint = &adjoints[channel * size..(channel + 1) * size];
                    let positive = if source.node_pos < n {
                        adjoint[source.node_pos * s + band]
                    } else {
                        Complex64::ZERO
                    };
                    let negative = if source.node_neg < n {
                        adjoint[source.node_neg * s + band]
                    } else {
                        Complex64::ZERO
                    };
                    let gain = positive - negative;
                    if !complex_is_finite(gain) {
                        return Err(HbError::InvalidCircuit(format!(
                            "pnoise source '{}' has a non-finite adjoint gain at sideband {}",
                            source.name,
                            i64::from(sideband_min) + band as i64
                        )));
                    }
                    gains[channel * s + band] = gain;
                }
                let row = &gains[channel * s..(channel + 1) * s];
                let (density, roundoff) =
                    source_covariance(row, row, source, window, self.config.fundamental_freq)?;
                covariance[channel * channels + channel] = density;
                diagonal_roundoff[channel] = roundoff;
            }
            for row in 0..channels {
                if abort.is_aborted() {
                    return Err(HbError::Aborted);
                }
                for column in row + 1..channels {
                    let (cross, roundoff) = source_covariance(
                        &gains[row * s..(row + 1) * s],
                        &gains[column * s..(column + 1) * s],
                        source,
                        window,
                        self.config.fundamental_freq,
                    )?;
                    // The pairwise PSD bound catches non-physical source
                    // spectra. Normalize before measuring the complex magnitude.
                    let limit = covariance[row * channels + row].re.sqrt()
                        * covariance[column * channels + column].re.sqrt();
                    let scale = limit.max(cross.re.abs()).max(cross.im.abs());
                    if scale > 0.0 {
                        let tolerance = roundoff / scale
                            + (diagonal_roundoff[row] / scale).sqrt()
                                * (diagonal_roundoff[column] / scale).sqrt()
                            + 64.0 * Value::EPSILON;
                        if (cross / scale).norm() > limit / scale + tolerance {
                            return Err(HbError::InvalidCircuit(format!(
                                "pnoise source '{}' covariance exceeds its diagonal noise-power bound",
                                source.name
                            )));
                        }
                    }
                    covariance[row * channels + column] = cross;
                    covariance[column * channels + row] = cross.conj();
                }
            }
            if abort.is_aborted() {
                return Err(HbError::Aborted);
            }
            consume(source_index, &covariance)?;
        }
        if abort.is_aborted() {
            return Err(HbError::Aborted);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;
