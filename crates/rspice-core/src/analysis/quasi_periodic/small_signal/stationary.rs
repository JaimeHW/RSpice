//! Keep phase-invariant stamps out of Fourier round trips. In particular an
//! exact algebraic constraint must not acquire spurious off-tone coefficients.
use super::*;

fn terms(sample: &Derivatives, charge: bool) -> &[JacobianEntry] {
    if charge {
        &sample.capacitance
    } else {
        &sample.conductance
    }
}

pub(super) fn separate(
    samples: &mut [Derivatives],
    abort: &dyn AbortSignal,
) -> Result<Derivatives, Error> {
    let mut result = Derivatives {
        conductance: Vec::new(),
        capacitance: Vec::new(),
    };
    for charge in [false, true] {
        let first = terms(&samples[0], charge);
        let mut same = vec![true; first.len()];
        for sample in &*samples {
            check_abort(abort)?;
            let sample = terms(sample, charge);
            for (i, term) in first.iter().enumerate() {
                same[i] &= sample.get(i) == Some(term);
            }
        }
        let constant = first
            .iter()
            .zip(&same)
            .filter_map(|(term, same)| same.then_some(*term))
            .collect();
        for sample in &mut *samples {
            check_abort(abort)?;
            let terms = if charge {
                &mut sample.capacitance
            } else {
                &mut sample.conductance
            };
            let mut index = 0;
            terms.retain(|_| {
                let keep = !same.get(index).copied().unwrap_or(false);
                index += 1;
                keep
            });
        }
        if charge {
            result.capacitance = constant;
        } else {
            result.conductance = constant;
        }
    }
    Ok(result)
}

impl Linearization {
    pub(super) fn add_stationary(
        &self,
        frequencies: &[Value],
        direction: &[Complex64],
        output: &mut [Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<(), Error> {
        let count = self.grid.len();
        for (k, f) in frequencies.iter().enumerate() {
            if k.is_multiple_of(256) {
                check_abort(abort)?;
            }
            for (terms, weight) in [
                (&self.stationary.conductance, Complex64::ONE),
                (
                    &self.stationary.capacitance,
                    Complex64::new(0.0, std::f64::consts::TAU * f),
                ),
            ] {
                for &(r, c, v) in terms {
                    if self.orientation == Orientation::Forward {
                        output[r * count + k] += weight * v * direction[c * count + k];
                    } else {
                        output[c * count + k] += weight.conj() * v * direction[r * count + k];
                    }
                }
            }
        }
        Ok(())
    }
}
