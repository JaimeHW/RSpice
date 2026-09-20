//! One real DC coordinate plus real/imaginary parts of each conjugate pair.
use super::*;
use crate::analysis::quasi_periodic::{check_abort, finite};

pub(super) fn validate(
    spectra: &[Vec<Complex64>],
    unknowns: usize,
    grid: &QuasiPeriodicGrid,
    context: &str,
    abort: &dyn AbortSignal,
) -> Result<(), Error> {
    if spectra.len() != unknowns || spectra.iter().any(|s| s.len() != grid.len()) {
        return Err(Error::InvalidConfig(format!(
            "{context} spectra do not match the complete MNA tone lattice"
        )));
    }
    for spectrum in spectra {
        check_abort(abort)?;
        for (i, &value) in spectrum.iter().enumerate() {
            if !finite(value) {
                return Err(Error::InvalidConfig(format!(
                    "{context} has non-finite Fourier coefficients"
                )));
            }
            let other = spectrum[grid.len() - 1 - i].conj();
            // Compare locally: a large DC coefficient must not hide a
            // malformed small AC channel. Scale first to avoid overflow.
            let scale = value
                .re
                .abs()
                .max(value.im.abs())
                .max(other.re.abs())
                .max(other.im.abs());
            if scale > 0.0 && (value / scale - other / scale).norm() > 128.0 * Value::EPSILON {
                return Err(Error::InvalidConfig(format!(
                    "{context} is not conjugate symmetric at tuple {:?}",
                    grid.indices()[i]
                )));
            }
        }
    }
    Ok(())
}

pub(super) fn encode(spectra: &[Vec<Complex64>]) -> Vec<Value> {
    let mut real = Vec::with_capacity(spectra.iter().map(Vec::len).sum());
    for spectrum in spectra {
        let dc = spectrum.len() / 2;
        real.push(spectrum[dc].re);
        for value in &spectrum[dc + 1..] {
            real.extend([value.re, value.im]);
        }
    }
    real
}

pub(super) fn decode(real: &[Value], entries: usize) -> Vec<Vec<Complex64>> {
    real.chunks_exact(entries)
        .map(|row| {
            let dc = entries / 2;
            let mut spectrum = vec![Complex64::ZERO; entries];
            spectrum[dc] = Complex64::new(row[0], 0.0);
            for (index, pair) in row[1..].chunks_exact(2).enumerate() {
                let k = dc + 1 + index;
                spectrum[k] = Complex64::new(pair[0], pair[1]);
                spectrum[entries - 1 - k] = spectrum[k].conj();
            }
            spectrum
        })
        .collect()
}

pub(super) fn basis(entries: usize, coordinate: usize) -> Vec<Complex64> {
    let mut real = vec![0.0; entries];
    real[coordinate] = 1.0;
    decode(&real, entries).pop().unwrap()
}
