//! Periodic steady-state results.
//!
//! The time-domain periodic solution: one period sampled on the shooting grid,
//! plus the spectra derived from it. Data is addressable by harmonic as well as
//! by node, which is what separates it from a plain transient result.

use super::*;

/// Periodic steady-state waveform and convergence diagnostics.
#[pyclass(name = "PssResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPssResult {
    inner: rspice_core::analysis::PssResult,
    #[pyo3(get)]
    pub num_harmonics: usize,
    #[pyo3(get)]
    pub iterations: usize,
    #[pyo3(get)]
    pub residual_norm: f64,
    #[pyo3(get)]
    pub period: f64,
    #[pyo3(get)]
    pub is_stable: bool,
}

impl PyPssResult {
    pub fn from_core(
        result: &rspice_core::engine::PssAnalysisResult,
        num_harmonics: usize,
    ) -> Self {
        Self {
            inner: result.result.clone(),
            num_harmonics,
            iterations: result.iterations,
            residual_norm: result.final_residual,
            period: result.period,
            is_stable: result.is_stable,
        }
    }

    fn waveform_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) => {
                let waveform_index = index - 1;
                if waveform_index < self.inner.waveforms.len() {
                    Ok(Some(waveform_index))
                } else {
                    Err(invalid_node_index_error(*index, self.inner.waveforms.len()).into())
                }
            }
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }

    fn harmonic_components(
        &self,
        node: &NodeIdentifier,
    ) -> PyResult<Vec<rspice_core::analysis::HarmonicComponent>> {
        match self.waveform_index(node)? {
            Some(index) => Ok(self.inner.harmonics(index + 1, self.num_harmonics)),
            None => Ok((0..=self.num_harmonics)
                .map(|harmonic_number| rspice_core::analysis::HarmonicComponent {
                    harmonic_number,
                    frequency: harmonic_number as f64 * self.inner.frequency,
                    magnitude: 0.0,
                    phase: 0.0,
                })
                .collect()),
        }
    }
}

#[pymethods]
impl PyPssResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.frequency
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn period_detected(&self) -> bool {
        self.inner.period_detected
    }

    #[getter]
    fn floquet_multipliers<'py>(
        &self,
        py: Python<'py>,
    ) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.inner.floquet_multipliers.to_pyarray(py)
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.waveforms.len()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.waveform_index(&node)? {
            Some(index) => self.inner.waveforms[index].values.clone(),
            None => vec![0.0; self.inner.time.len()],
        };
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].interpolate(
                &self.inner.time,
                time,
                self.inner.period,
            )),
            None => Ok(0.0),
        }
    }

    fn dc(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].dc(&self.inner.time, self.inner.period)),
            None => Ok(0.0),
        }
    }

    fn peak_to_peak(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].peak_to_peak()),
            None => Ok(0.0),
        }
    }

    /// Frequencies of the configured DC-through-N harmonic spectrum.
    #[getter]
    fn harmonic_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        (0..=self.num_harmonics)
            .map(|harmonic| harmonic as f64 * self.inner.frequency)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Complex peak phasors for a node's configured harmonic spectrum.
    fn harmonic_coefficients<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| {
                rspice_core::Complex64::from_polar(
                    component.magnitude,
                    component.phase.to_radians(),
                )
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn harmonic_magnitude<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| component.magnitude.abs())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn harmonic_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| component.phase)
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    /// Non-DC harmonic records, beginning with the fundamental.
    fn harmonics(&self, node: NodeIdentifier) -> PyResult<Vec<PyHarmonic>> {
        Ok(self
            .harmonic_components(&node)?
            .into_iter()
            .filter(|component| component.harmonic_number > 0)
            .map(|component| PyHarmonic {
                n: component.harmonic_number,
                frequency: component.frequency,
                magnitude: component.magnitude,
                phase: component.phase.to_radians(),
            })
            .collect())
    }

    fn thd_percent(&self, node: NodeIdentifier) -> PyResult<f64> {
        let components = self.harmonic_components(&node)?;
        let Some(fundamental) = components.get(1).map(|component| component.magnitude.abs()) else {
            return Ok(0.0);
        };
        if fundamental <= f64::EPSILON {
            return Ok(0.0);
        }
        let distortion_power = components
            .iter()
            .skip(2)
            .map(|component| component.magnitude * component.magnitude)
            .sum::<f64>();
        Ok(100.0 * distortion_power.sqrt() / fundamental)
    }

    fn __repr__(&self) -> String {
        format!(
            "PssResult(frequency={:.6e}Hz, harmonics={}, nodes={}, points={}, iterations={}, residual={:.3e})",
            self.inner.frequency,
            self.num_harmonics,
            self.inner.waveforms.len(),
            self.inner.time.len(),
            self.iterations,
            self.residual_norm
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// The orbit group is the converged periodic solution; the diagnostics
    /// group is what the shooting run reported around it.
    #[staticmethod]
    fn _unpickle(
        orbit: (f64, f64, usize, f64, bool),
        time: Vec<f64>,
        waveforms: Vec<Vec<f64>>,
        node_names: Vec<String>,
        floquet_multipliers: Vec<(f64, f64)>,
        diagnostics: (usize, usize, f64, f64, bool),
    ) -> Self {
        let (period, frequency, iterations, residual_norm, period_detected) = orbit;
        let (num_harmonics, run_iterations, run_residual, run_period, is_stable) = diagnostics;
        Self {
            inner: rspice_core::analysis::PssResult {
                period,
                frequency,
                iterations,
                residual_norm,
                time,
                waveforms: waveforms
                    .into_iter()
                    .map(rspice_core::analysis::PeriodicWaveform::from_values)
                    .collect(),
                node_names,
                period_detected,
                floquet_multipliers: complex_from_state(floquet_multipliers),
            },
            num_harmonics,
            iterations: run_iterations,
            residual_norm: run_residual,
            period: run_period,
            is_stable,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (f64, f64, usize, f64, bool),
            Vec<f64>,
            Vec<Vec<f64>>,
            Vec<String>,
            Vec<(f64, f64)>,
            (usize, usize, f64, f64, bool),
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.inner.period,
                    self.inner.frequency,
                    self.inner.iterations,
                    self.inner.residual_norm,
                    self.inner.period_detected,
                ),
                self.inner.time.clone(),
                self.inner
                    .waveforms
                    .iter()
                    .map(|waveform| waveform.values.clone())
                    .collect(),
                self.inner.node_names.clone(),
                complex_state(&self.inner.floquet_multipliers),
                (
                    self.num_harmonics,
                    self.iterations,
                    self.residual_norm,
                    self.period,
                    self.is_stable,
                ),
            ),
        ))
    }
}
