//! Pickle state encoding shared by every result type.
//!
//! A result pickles as a plain tuple of primitives rather than through PyO3's
//! derive, because the core types it wraps are not themselves Python objects.
//! Two conventions here are load-bearing across versions:
//!
//! - Complex values travel as `(real, imaginary)` pairs. `Complex64` has no
//!   PyO3 scalar conversion, and a pair is the lossless encoding a NumPy-free
//!   consumer can read too.
//! - Enums travel as stable string labels, never ordinals, so state pickled by
//!   one build still reads correctly under a later one that reorders a variant.
//!
//! Those two conventions, and the `_unpickle` lookup every `__reduce__` goes
//! through, live here. Everything else is one module per result family, so a
//! state contract is read next to the other fields of the same contract rather
//! than interleaved with unrelated ones: [`simulation`] the DC operating
//! point, [`transient_fft`] the `.FFT` products (with [`transient_fft_labels`]
//! for its tag vocabulary), [`ac_row`] AC rows and the distortion products
//! built from them, [`harmonic_balance`] the HB spectra and continuation
//! limitations, [`floquet`] the PSS Floquet evidence, and [`root_set`] the
//! pole-zero root-set evidence.

use super::*;

mod ac_row;
mod floquet;
mod harmonic_balance;
mod root_set;
mod simulation;
mod transient_fft;
mod transient_fft_labels;

pub(crate) use ac_row::*;
pub(crate) use floquet::*;
pub(crate) use harmonic_balance::*;
pub(crate) use root_set::*;
pub(crate) use simulation::*;
pub(crate) use transient_fft::*;

/// Complex values in pickled state travel as `(real, imaginary)` pairs.
///
/// `num_complex::Complex64` has no PyO3 scalar conversion, and a pair is the
/// lossless encoding NumPy-free consumers can read too.
pub(super) fn complex_state(values: &[Complex64]) -> Vec<(f64, f64)> {
    values.iter().map(|value| (value.re, value.im)).collect()
}

pub(super) fn complex_from_state(values: Vec<(f64, f64)>) -> Vec<Complex64> {
    values
        .into_iter()
        .map(|(re, im)| Complex64::new(re, im))
        .collect()
}

/// The `(absolute, normalized)` complex derivative pair of an AC sensitivity
/// trace, in pickled `(real, imaginary)` form.
pub(super) type ComplexSeriesPair = (Vec<(f64, f64)>, Vec<(f64, f64)>);

/// The `_unpickle` callable bound to a pyclass, referenced by `__reduce__`.
///
/// Pickling a bound staticmethod resolves by qualified name, which keeps each
/// helper namespaced on its own class instead of adding private names to the
/// module surface.
pub(super) fn unpickler<'py, T: pyo3::PyTypeInfo>(py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
    py.get_type::<T>().getattr("_unpickle")
}
