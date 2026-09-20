//! Non-cyclic signed-tuple convolution, including stationary noise beyond the circuit window.
use super::*;
use std::collections::BTreeMap;

impl QuasiPeriodicNoiseProjector {
    pub(super) fn colored(
        &self,
        gains: &[projection::Spectrum],
        adjoint: &QuasiPeriodicAdjointSolution,
        coefficient: Value,
        exponent: Value,
        modulation: &[Complex64],
        binary_scale: i32,
        abort: &dyn AbortSignal,
    ) -> Result<QuasiPeriodicNoiseCovariance, Error> {
        let modes: Vec<_> = modulation
            .iter()
            .enumerate()
            .filter(|(_, a)| **a != Complex64::ZERO)
            .collect();
        let pairs = self.selected.len().saturating_mul(modes.len());
        // Bound the convolution traversal and its worst-case distinct tuples
        // before allocation, including arbitrary-dimensional sparse lattices.
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            pairs,
            self.limits.max_analysis_points,
        )?;
        self.check_values(
            gains.len(),
            pairs.saturating_mul(self.grid.dimensions().len() + gains.len().saturating_mul(16)),
        )?;
        let mut support: BTreeMap<Vec<i32>, Vec<Option<i32>>> = BTreeMap::new();
        self.visit_modulated_terms(gains, &modes, abort, |tuple, output, term| {
            if term.is_zero() {
                return Ok(());
            }
            let powers = support
                .entry(tuple)
                .or_insert_with(|| vec![None; gains.len()]);
            powers[output] = Some(powers[output].map_or(term.exponent, |e| e.max(term.exponent)));
            Ok(())
        })?;
        let mut sums: BTreeMap<_, _> = support
            .into_iter()
            .map(|(tuple, powers)| {
                (
                    tuple,
                    powers
                        .into_iter()
                        .map(|p| p.map(ScaledComplexAccumulator::new))
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        self.visit_modulated_terms(gains, &modes, abort, |tuple, output, term| {
            if !term.is_zero() {
                sums.get_mut(&tuple).expect("visited support")[output]
                    .as_mut()
                    .expect("nonzero direction")
                    .add(term)
                    .map_err(numerical)?;
            }
            Ok(())
        })?;
        let mut folded = Vec::with_capacity(sums.len());
        for (tuple, sums) in sums {
            check_abort(abort)?;
            let mut channels = Vec::with_capacity(gains.len());
            for sum in sums {
                let Some(sum) = sum else {
                    channels.push(ScaledComplex::ZERO);
                    continue;
                };
                // The accumulator keeps its exponent until the density cancels it.
                let value = sum.into_scaled().map_err(numerical)?;
                channels.push(value);
            }
            let frequency = self
                .grid
                .frequency_relative_to(adjoint.frequency_hz, &adjoint.frequency_lattice, &tuple)?
                .abs();
            folded.push((channels, frequency));
        }
        covariance(
            gains.len(),
            |r, c| {
                sum_terms(
                    |visit| {
                        for (i, (channels, frequency)) in folded.iter().enumerate() {
                            if i.is_multiple_of(256) {
                                check_abort(abort)?;
                            }
                            // h is M(a)ᴴ bᴴλ. Output transfers are conj(h), hence the
                            // first conjugate here; preserve complex cross-output phase.
                            let left = ScaledComplex {
                                mantissa: channels[r].mantissa.conj(),
                                exponent: channels[r].exponent,
                            };
                            let right = ScaledComplex {
                                mantissa: channels[c].mantissa.conj(),
                                exponent: channels[c].exponent,
                            };
                            let term = scaled_flicker_cross_term(
                                left,
                                right,
                                coefficient,
                                binary_scale,
                                *frequency,
                                exponent,
                            )
                            .map_err(numerical)?;
                            visit(term)?;
                        }
                        Ok(())
                    },
                    r == c,
                )
            },
            abort,
        )
    }

    fn visit_modulated_terms(
        &self,
        gains: &[projection::Spectrum],
        modes: &[(usize, &Complex64)],
        abort: &dyn AbortSignal,
        mut visit: impl FnMut(Vec<i32>, usize, ScaledComplex) -> Result<(), Error>,
    ) -> Result<(), Error> {
        let mut count = 0usize;
        for &k in &self.selected {
            for &(m, amplitude) in modes {
                if count.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                count += 1;
                let tuple = self.grid.indices()[k]
                    .iter()
                    .zip(&self.grid.indices()[m])
                    .map(|(k, m)| {
                        k.checked_sub(*m).ok_or_else(|| {
                            invalid("colored stationary tuple exceeds signed coordinate range")
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                for (output, gain) in gains.iter().enumerate() {
                    let term = scaled_complex_product3(
                        gain.values[k],
                        amplitude.conj(),
                        Complex64::ONE,
                        gain.exponent,
                    )
                    .map_err(numerical)?;
                    visit(tuple.clone(), output, term)?;
                }
            }
        }
        Ok(())
    }
}
