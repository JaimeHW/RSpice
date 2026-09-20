//! Complex translated Jacobian action; no real-coordinate folding.
use super::*;

impl Linearization {
    pub(super) fn apply(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        direction: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        check_abort(abort)?;
        let entries = self.grid.len();
        let count = self.grid.sample_count();
        if direction.len() != self.unknowns * entries || direction.iter().any(|v| !finite(*v)) {
            return Err(Error::InvalidConfig(
                "QPAC direction differs from its finite complex MNA tone coordinates".into(),
            ));
        }
        let mut waves = Vec::with_capacity(self.unknowns);
        for spectrum in direction.chunks_exact(entries) {
            check_abort(abort)?;
            waves.push(if spectrum.iter().any(|v| *v != Complex64::ZERO) {
                Some(self.transform.to_samples_with_abort(spectrum, abort)?)
            } else {
                None
            });
        }
        let mut conductance = vec![vec![Complex64::ZERO; count]; self.unknowns];
        let mut capacitance = vec![vec![Complex64::ZERO; count]; self.unknowns];
        for (time, sample) in self.derivatives.iter().enumerate() {
            if time.is_multiple_of(256) {
                check_abort(abort)?;
            }
            for (terms, values) in [
                (&sample.conductance, &mut conductance),
                (&sample.capacitance, &mut capacitance),
            ] {
                for &(row, col, derivative) in terms {
                    if let Some(wave) = &waves[col] {
                        values[row][time] += derivative * wave[time];
                    }
                }
            }
        }
        let mut output = vec![Complex64::ZERO; direction.len()];
        for row in 0..self.unknowns {
            let g = self
                .transform
                .to_spectrum_with_abort(&conductance[row], abort)?;
            let c = self
                .transform
                .to_spectrum_with_abort(&capacitance[row], abort)?;
            for k in 0..entries {
                output[row * entries + k] =
                    g[k] + Complex64::new(0.0, std::f64::consts::TAU * frequencies[k]) * c[k];
            }
        }
        for (k, matrix) in linear.iter().enumerate() {
            check_abort(abort)?;
            for &(row, col, value) in matrix {
                output[row * entries + k] += value * direction[col * entries + k];
            }
        }
        if output.iter().any(|v| !finite(*v)) {
            return Err(Error::Numerical(
                "QPAC translated derivative projection overflowed".into(),
            ));
        }
        Ok(output)
    }
}
