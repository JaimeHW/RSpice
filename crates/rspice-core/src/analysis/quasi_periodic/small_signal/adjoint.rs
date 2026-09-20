//! Hermitian adjoint of the full translated F/Q operator, without dense assembly.
use super::*;

#[cfg(test)]
mod tests;

impl Linearization {
    pub(super) fn apply_adjoint(
        &mut self,
        frequencies: &[Value],
        linear: &[Vec<LinearEntry>],
        direction: &[Complex64],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, Error> {
        check_abort(abort)?;
        let entries = self.grid.len();
        let samples = self.grid.sample_count();
        if direction.len() != self.unknowns * entries || direction.iter().any(|v| !finite(*v)) {
            return Err(Error::InvalidConfig(
                "QPXF adjoint direction differs from its finite MNA tone coordinates".into(),
            ));
        }
        let mut waves = Vec::with_capacity(self.unknowns);
        let mut charge_waves = Vec::with_capacity(self.unknowns);
        for spectrum in direction.chunks_exact(entries) {
            check_abort(abort)?;
            if spectrum.iter().any(|v| *v != Complex64::ZERO) {
                waves.push(Some(self.transform.to_samples_with_abort(spectrum, abort)?));
                // (jΩ C)ᴴ = Cᴴ (-jΩ): multiply at the original OUTPUT
                // frequencies before the transposed charge convolution.
                let weighted: Vec<_> = spectrum
                    .iter()
                    .zip(frequencies)
                    .map(|(v, f)| *v * Complex64::new(0.0, -std::f64::consts::TAU * *f))
                    .collect();
                charge_waves.push(Some(
                    self.transform.to_samples_with_abort(&weighted, abort)?,
                ));
            } else {
                waves.push(None);
                charge_waves.push(None);
            }
        }
        let mut products = vec![vec![Complex64::ZERO; samples]; self.unknowns];
        for (time, sample) in self.derivatives.iter().enumerate() {
            if time.is_multiple_of(256) {
                check_abort(abort)?;
            }
            for (terms, values) in [
                (&sample.conductance, &waves),
                (&sample.capacitance, &charge_waves),
            ] {
                for &(row, col, derivative) in terms {
                    if let Some(wave) = &values[row] {
                        products[col][time] += derivative * wave[time];
                    }
                }
            }
        }
        let mut output = Vec::with_capacity(direction.len());
        for row in &products {
            output.extend(self.transform.to_spectrum_with_abort(row, abort)?);
        }
        for (k, matrix) in linear.iter().enumerate() {
            check_abort(abort)?;
            for &(row, col, value) in matrix {
                output[col * entries + k] += value.conj() * direction[row * entries + k];
            }
        }
        if output.iter().any(|v| !finite(*v)) {
            return Err(Error::Numerical(
                "QPXF adjoint derivative projection overflowed".into(),
            ));
        }
        Ok(output)
    }
}
