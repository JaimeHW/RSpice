//! Harmonic-balance results.
//!
//! The frequency-domain periodic solution, for single- and multi-tone
//! excitation. Retains the branch-current and reactive-element spectra that
//! `is_valid` tests, and records any continuation limitations that applied, so
//! a caller can tell a converged solution from one that leaned on a surrogate.

use super::*;

/// Harmonic-balance spectra and convergence diagnostics.
#[pyclass(name = "HbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyHbResult {
    inner: rspice_core::analysis::HbResult,
}

impl PyHbResult {
    pub fn from_core(result: &rspice_core::engine::HbAnalysisResult) -> Self {
        Self {
            inner: result.result.clone(),
        }
    }

    fn spectral_voltage(
        &self,
        node: &str,
    ) -> PyResult<&rspice_core::analysis::advanced::harmonic_balance::SpectralVoltage> {
        self.inner
            .spectral_voltages
            .iter()
            .find(|value| value.node_name.eq_ignore_ascii_case(node))
            .ok_or_else(|| crate::errors::key_error(format!("unknown node '{node}'")))
    }
}

#[pymethods]
impl PyHbResult {
    #[getter]
    fn converged(&self) -> bool {
        self.inner.converged
    }

    #[getter]
    fn iterations(&self) -> usize {
        self.inner.iterations
    }

    #[getter]
    fn residual_norm(&self) -> f64 {
        self.inner.residual_norm
    }

    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.fundamental_freq
    }

    #[getter]
    fn num_harmonics(&self) -> usize {
        self.inner.num_harmonics
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn harmonic_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.harmonic_frequencies.to_pyarray(py)
    }

    #[getter]
    fn solve_time_seconds(&self) -> f64 {
        self.inner.solve_time_seconds
    }

    #[getter]
    fn is_valid(&self) -> bool {
        self.inner.is_valid()
    }

    fn coefficients<'py>(
        &self,
        py: Python<'py>,
        node: &str,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        Ok(self.spectral_voltage(node)?.coefficients.to_pyarray(py))
    }

    fn magnitude<'py>(&self, py: Python<'py>, node: &str) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .spectral_voltage(node)?
            .coefficients
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .spectral_voltage(node)?
            .coefficients
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn dc(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.dc())
    }

    fn rms(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.rms())
    }

    fn thd_percent(&self, node: &str) -> PyResult<f64> {
        Ok(self.spectral_voltage(node)?.thd_percent())
    }

    fn __repr__(&self) -> String {
        format!(
            "HbResult(fundamental={:.6e}Hz, harmonics={}, nodes={}, converged={})",
            self.inner.fundamental_freq,
            self.inner.num_harmonics,
            self.inner.node_names.len(),
            self.inner.converged
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// Branch-current and reactive spectra are carried even though this class
    /// exposes no accessor for them, because `is_valid` is a finiteness test
    /// over all three spectra and would otherwise change across a round-trip.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        convergence: (bool, usize, f64, f64, usize, f64),
        spectral_voltages: Vec<SpectralSeriesState>,
        node_names: Vec<String>,
        harmonic_frequencies: Vec<f64>,
        tones: Vec<String>,
        mna_branch_currents: Vec<SpectralSeriesState>,
        reactive_spectra: Vec<HbReactiveState>,
        continuation_limitations: Vec<String>,
    ) -> PyResult<Self> {
        let (
            converged,
            iterations,
            residual_norm,
            fundamental_freq,
            num_harmonics,
            solve_time_seconds,
        ) = convergence;
        Ok(Self {
            inner: rspice_core::analysis::HbResult {
                converged,
                iterations,
                residual_norm,
                fundamental_freq,
                spectral_voltages: spectral_voltages
                    .into_iter()
                    .map(|(node_name, coefficients, frequencies)| {
                        rspice_core::analysis::SpectralVoltage {
                            node_name,
                            coefficients: complex_from_state(coefficients),
                            frequencies,
                        }
                    })
                    .collect(),
                node_names,
                num_harmonics,
                harmonic_frequencies,
                solve_time_seconds,
                tones,
                mna_branch_currents: mna_branch_currents
                    .into_iter()
                    .map(|(device_name, coefficients, frequencies)| {
                        rspice_core::analysis::SpectralBranchCurrent {
                            device_name,
                            coefficients: complex_from_state(coefficients),
                            frequencies,
                        }
                    })
                    .collect(),
                reactive_spectra: reactive_spectra
                    .into_iter()
                    .map(rebuild_hb_reactive)
                    .collect::<PyResult<Vec<_>>>()?,
                continuation_limitations: continuation_limitations
                    .iter()
                    .map(|label| hb_limitation_from_label(label))
                    .collect::<PyResult<Vec<_>>>()?,
            },
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (bool, usize, f64, f64, usize, f64),
            Vec<SpectralSeriesState>,
            Vec<String>,
            Vec<f64>,
            Vec<String>,
            Vec<SpectralSeriesState>,
            Vec<HbReactiveState>,
            Vec<String>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.inner.converged,
                    self.inner.iterations,
                    self.inner.residual_norm,
                    self.inner.fundamental_freq,
                    self.inner.num_harmonics,
                    self.inner.solve_time_seconds,
                ),
                self.inner
                    .spectral_voltages
                    .iter()
                    .map(|value| {
                        spectral_series_state(
                            &value.node_name,
                            &value.coefficients,
                            &value.frequencies,
                        )
                    })
                    .collect(),
                self.inner.node_names.clone(),
                self.inner.harmonic_frequencies.clone(),
                self.inner.tones.clone(),
                self.inner
                    .mna_branch_currents
                    .iter()
                    .map(|branch| {
                        spectral_series_state(
                            &branch.device_name,
                            &branch.coefficients,
                            &branch.frequencies,
                        )
                    })
                    .collect(),
                self.inner
                    .reactive_spectra
                    .iter()
                    .map(|reactive| {
                        (
                            reactive.device_name.clone(),
                            hb_reactive_kind_label(reactive.kind).to_string(),
                            complex_state(&reactive.voltage_coefficients),
                            complex_state(&reactive.current_coefficients),
                            reactive.dc_current_is_exact,
                        )
                    })
                    .collect(),
                self.inner
                    .continuation_limitations
                    .iter()
                    .map(|limitation| hb_limitation_label(limitation).to_string())
                    .collect(),
            ),
        ))
    }
}
