//! `.FOUR` spectra derived from transient waveforms.
//!
//! These live apart from the small-signal frequency-domain results because they
//! are a post-processing step over a time-domain run, not an analysis in their
//! own right: the harmonics come from a DFT of a sampled waveform, so their
//! accuracy is governed by the transient timestep rather than by a solver
//! tolerance.

use super::*;

/// A single harmonic component from Fourier analysis
#[pyclass(name = "Harmonic", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyHarmonic {
    /// Harmonic number (1 = fundamental)
    #[pyo3(get)]
    pub n: usize,
    /// Frequency in Hz
    #[pyo3(get)]
    pub frequency: f64,
    /// Magnitude
    #[pyo3(get)]
    pub magnitude: f64,
    /// Phase in radians
    #[pyo3(get)]
    pub phase: f64,
}

#[pymethods]
impl PyHarmonic {
    /// Phase in degrees
    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.phase.to_degrees()
    }

    fn __repr__(&self) -> String {
        format!(
            "Harmonic(n={}, f={:.4e}Hz, mag={:.6e}, phase={:.2}°)",
            self.n,
            self.frequency,
            self.magnitude,
            self.phase.to_degrees()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(n: usize, frequency: f64, magnitude: f64, phase: f64) -> Self {
        Self {
            n,
            frequency,
            magnitude,
            phase,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (usize, f64, f64, f64))> {
        Ok((
            unpickler::<Self>(py)?,
            (self.n, self.frequency, self.magnitude, self.phase),
        ))
    }
}

/// Fourier analysis result (harmonic decomposition + THD)
///
/// Example:
///     >>> four = tran.fourier("out", fundamental=1e3)
///     >>> print(f"THD = {four.thd_percent:.3f}%")
///     >>> for h in four.harmonics:
///     ...     print(h)
#[pyclass(name = "FourierResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFourierResult {
    /// DC component of the waveform
    #[pyo3(get)]
    pub dc_component: f64,
    /// Total harmonic distortion as a ratio (0-1)
    #[pyo3(get)]
    pub thd: f64,
    harmonics: Vec<PyHarmonic>,
}

impl PyFourierResult {
    pub fn from_core(result: &rspice_core::analysis::FourierResult) -> Self {
        // Core's harmonic 0 is the DC term; expose it via `dc_component`
        // and keep the list at n >= 1 so harmonics[0] is the fundamental.
        // Core reports phase in degrees; this API uses radians plus
        // *_degrees helpers everywhere.
        let harmonics = result
            .harmonics
            .iter()
            .filter(|h| h.harmonic_number >= 1)
            .map(|h| PyHarmonic {
                n: h.harmonic_number,
                frequency: h.frequency,
                magnitude: h.magnitude,
                phase: h.phase.to_radians(),
            })
            .collect();
        Self {
            dc_component: result.dc_component,
            // Core reports THD in percent already.
            thd: result.thd / 100.0,
            harmonics,
        }
    }
}

#[pymethods]
impl PyFourierResult {
    /// Total harmonic distortion in percent
    #[getter]
    fn thd_percent(&self) -> f64 {
        self.thd * 100.0
    }

    /// All harmonic components (index 0 = fundamental)
    #[getter]
    fn harmonics(&self) -> Vec<PyHarmonic> {
        self.harmonics.clone()
    }

    /// Harmonic magnitudes as a NumPy array (index 0 = fundamental)
    #[getter]
    fn magnitudes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let mags: Vec<f64> = self.harmonics.iter().map(|h| h.magnitude).collect();
        mags.to_pyarray(py)
    }

    /// Magnitude of the fundamental
    #[getter]
    fn fundamental_magnitude(&self) -> Option<f64> {
        self.harmonics.first().map(|h| h.magnitude)
    }

    fn __repr__(&self) -> String {
        format!(
            "FourierResult(harmonics={}, dc={:.4e}, thd={:.4}%)",
            self.harmonics.len(),
            self.dc_component,
            self.thd * 100.0
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(dc_component: f64, thd: f64, harmonics: Vec<PyHarmonic>) -> Self {
        Self {
            dc_component,
            thd,
            harmonics,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (f64, f64, Vec<PyHarmonic>))> {
        Ok((
            unpickler::<Self>(py)?,
            (self.dc_component, self.thd, self.harmonics.clone()),
        ))
    }
}
