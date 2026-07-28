//! Noise results, both stationary and periodic.
//!
//! `NoiseResult` is the small-signal `.NOISE` spectrum. `PeriodicNoiseResult`
//! carries driven cyclostationary noise about a periodic operating point, and
//! `OscillatorNoiseResult` carries autonomous phase noise, which is a different
//! quantity: it comes from the perturbation projection vector rather than from
//! a transfer function, because an oscillator has no forced reference phase.

use super::*;

/// Noise contribution from a single device
///
/// Provides information about one noise source's contribution to total output noise.
///
/// Example:
///     >>> for contrib in result.contributions:
///     ...     print(f"{contrib.device_name}: {contrib.percentage:.1f}%")
#[pyclass(name = "NoiseContribution", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyNoiseContribution {
    /// Device name generating this noise
    #[pyo3(get)]
    pub device_name: String,
    /// Noise type as string (Thermal, Shot, Flicker, Burst)
    #[pyo3(get)]
    pub noise_type: String,
    /// Output contribution in V²/Hz
    #[pyo3(get)]
    pub output_contribution: f64,
    /// Percentage of total noise
    #[pyo3(get)]
    pub percentage: f64,
}

#[pymethods]
impl PyNoiseContribution {
    fn __repr__(&self) -> String {
        format!(
            "NoiseContribution({}: {:.1}%, type={})",
            self.device_name, self.percentage, self.noise_type
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        device_name: String,
        noise_type: String,
        output_contribution: f64,
        percentage: f64,
    ) -> Self {
        Self {
            device_name,
            noise_type,
            output_contribution,
            percentage,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (String, String, f64, f64))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.device_name.clone(),
                self.noise_type.clone(),
                self.output_contribution,
                self.percentage,
            ),
        ))
    }
}

/// Noise analysis result at a single frequency
///
/// Contains output noise spectral density and contribution breakdown.
///
/// Example:
///     >>> result = engine.run_noise(netlist, output_node="out", frequencies=[1e3, 10e3])
///     >>> for r in result:
///     ...     print(f"f={r.frequency:.0f}Hz: {r.output_noise_rms*1e9:.2f}nV/√Hz")
#[pyclass(name = "NoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyNoiseResult {
    /// Frequency in Hz
    #[pyo3(get)]
    pub frequency: f64,
    /// Total output voltage noise spectral density (V²/Hz)
    #[pyo3(get)]
    pub output_noise_density: f64,
    /// Input-referred noise spectral density (V²/Hz)
    #[pyo3(get)]
    pub input_referred_density: f64,
    /// Individual noise contributions
    contributions: Vec<PyNoiseContribution>,
}

impl PyNoiseResult {
    /// Create from core NoiseResult
    pub fn from_core(result: &rspice_core::analysis::NoiseResult) -> Self {
        let contributions = result
            .contributions
            .iter()
            .map(|c| PyNoiseContribution {
                device_name: c.identity.device.clone(),
                noise_type: format!("{:?}", c.noise_type),
                output_contribution: c.output_contribution,
                percentage: c.percentage,
            })
            .collect();

        Self {
            frequency: result.frequency,
            output_noise_density: result.output_noise_density,
            input_referred_density: result.input_referred_density,
            contributions,
        }
    }
}

#[pymethods]
impl PyNoiseResult {
    /// Get output noise in V/√Hz (RMS voltage noise density)
    #[getter]
    fn output_noise_rms(&self) -> f64 {
        self.output_noise_density.sqrt()
    }

    /// Get input-referred noise in V/√Hz
    #[getter]
    fn input_referred_rms(&self) -> f64 {
        self.input_referred_density.sqrt()
    }

    /// Get output noise in dBV/sqrt(Hz).
    ///
    /// This is `20*log10(sqrt(Svo) / 1 V/sqrt(Hz))`, equivalently
    /// `10*log10(Svo / 1 V^2/Hz)`. Zero density is represented by negative
    /// infinity; invalid negative density propagates as NaN.
    #[getter]
    fn output_noise_dbv(&self) -> f64 {
        10.0 * self.output_noise_density.log10()
    }

    /// Get all noise contributions
    #[getter]
    fn contributions(&self) -> Vec<PyNoiseContribution> {
        self.contributions.clone()
    }

    /// Get the dominant noise source
    fn dominant_source(&self) -> Option<PyNoiseContribution> {
        self.contributions
            .iter()
            .max_by(|a, b| {
                a.output_contribution
                    .partial_cmp(&b.output_contribution)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .cloned()
    }

    fn __repr__(&self) -> String {
        format!(
            "NoiseResult(f={:.2e}Hz, Svo={:.2e}V²/Hz, Svo_rms={:.2e}V/√Hz)",
            self.frequency,
            self.output_noise_density,
            self.output_noise_rms()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        frequency: f64,
        output_noise_density: f64,
        input_referred_density: f64,
        contributions: Vec<PyNoiseContribution>,
    ) -> Self {
        Self {
            frequency,
            output_noise_density,
            input_referred_density,
            contributions,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (f64, f64, f64, Vec<PyNoiseContribution>))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequency,
                self.output_noise_density,
                self.input_referred_density,
                self.contributions.clone(),
            ),
        ))
    }
}

/// One source's folded periodic-noise power spectral density.
#[pyclass(name = "PeriodicNoiseContribution", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPeriodicNoiseContribution {
    #[pyo3(get)]
    pub name: String,
    values: Vec<f64>,
}

#[pymethods]
impl PyPeriodicNoiseContribution {
    #[getter]
    fn power_spectral_density<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.values.to_pyarray(py)
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(name: String, values: Vec<f64>) -> Self {
        Self { name, values }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (String, Vec<f64>))> {
        Ok((
            unpickler::<Self>(py)?,
            (self.name.clone(), self.values.clone()),
        ))
    }
}

/// Periodic-noise result, in power spectral density units (V^2/Hz).
#[pyclass(name = "PeriodicNoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPeriodicNoiseResult {
    frequencies: Vec<f64>,
    output_noise: Vec<f64>,
    input_noise: Option<Vec<f64>>,
    contributors: Vec<PyPeriodicNoiseContribution>,
    #[pyo3(get)]
    pub fundamental_frequency: f64,
    #[pyo3(get)]
    pub converged: bool,
}

/// Autonomous-oscillator single-sideband phase noise from PPV projection.
#[pyclass(name = "OscillatorNoiseResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyOscillatorNoiseResult {
    frequencies: Vec<f64>,
    phase_noise_dbc: Vec<f64>,
    #[pyo3(get)]
    pub diffusion_constant: f64,
    #[pyo3(get)]
    pub period: f64,
    #[pyo3(get)]
    pub corner_frequency: f64,
}

impl PyOscillatorNoiseResult {
    pub fn from_core(result: &rspice_core::engine::OscPnoiseResult) -> Self {
        Self {
            frequencies: result.frequencies.clone(),
            phase_noise_dbc: result.phase_noise_dbc.clone(),
            diffusion_constant: result.diffusion_constant,
            period: result.period,
            corner_frequency: result.corner_hz,
        }
    }
}

#[pymethods]
impl PyOscillatorNoiseResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn phase_noise_dbc<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.phase_noise_dbc.to_pyarray(py)
    }

    #[getter]
    fn carrier_frequency(&self) -> f64 {
        1.0 / self.period
    }

    fn __repr__(&self) -> String {
        format!(
            "OscillatorNoiseResult(carrier={:.6e}Hz, points={}, corner={:.6e}Hz)",
            1.0 / self.period,
            self.frequencies.len(),
            self.corner_frequency
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        frequencies: Vec<f64>,
        phase_noise_dbc: Vec<f64>,
        diffusion_constant: f64,
        period: f64,
        corner_frequency: f64,
    ) -> Self {
        Self {
            frequencies,
            phase_noise_dbc,
            diffusion_constant,
            period,
            corner_frequency,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (Vec<f64>, Vec<f64>, f64, f64, f64))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequencies.clone(),
                self.phase_noise_dbc.clone(),
                self.diffusion_constant,
                self.period,
                self.corner_frequency,
            ),
        ))
    }
}

impl PyPeriodicNoiseResult {
    pub fn from_core(result: &rspice_core::engine::PnoiseAnalysisResult) -> Self {
        Self {
            frequencies: result.frequencies.clone(),
            output_noise: result.output_noise.clone(),
            input_noise: result.input_noise.clone(),
            contributors: result
                .contributors
                .iter()
                .map(|(name, values)| PyPeriodicNoiseContribution {
                    name: name.clone(),
                    values: values.clone(),
                })
                .collect(),
            fundamental_frequency: result.fundamental_freq,
            converged: result.converged,
        }
    }
}

#[pymethods]
impl PyPeriodicNoiseResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn output_noise<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_noise.to_pyarray(py)
    }

    #[getter]
    fn output_noise_density<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_noise
            .iter()
            .map(|value| value.sqrt())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn input_noise<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.input_noise
            .as_ref()
            .map(|values| values.to_pyarray(py))
    }

    #[getter]
    fn contributors(&self) -> Vec<PyPeriodicNoiseContribution> {
        self.contributors.clone()
    }

    fn contribution(&self, name: &str) -> PyResult<PyPeriodicNoiseContribution> {
        self.contributors
            .iter()
            .find(|value| value.name.eq_ignore_ascii_case(name))
            .cloned()
            .ok_or_else(|| crate::errors::key_error(format!("unknown noise contributor '{name}'")))
    }

    fn __repr__(&self) -> String {
        format!(
            "PeriodicNoiseResult(fundamental={:.6e}Hz, points={}, contributors={}, converged={})",
            self.fundamental_frequency,
            self.frequencies.len(),
            self.contributors.len(),
            self.converged
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        frequencies: Vec<f64>,
        output_noise: Vec<f64>,
        input_noise: Option<Vec<f64>>,
        contributors: Vec<PyPeriodicNoiseContribution>,
        fundamental_frequency: f64,
        converged: bool,
    ) -> Self {
        Self {
            frequencies,
            output_noise,
            input_noise,
            contributors,
            fundamental_frequency,
            converged,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<f64>,
            Vec<f64>,
            Option<Vec<f64>>,
            Vec<PyPeriodicNoiseContribution>,
            f64,
            bool,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequencies.clone(),
                self.output_noise.clone(),
                self.input_noise.clone(),
                self.contributors.clone(),
                self.fundamental_frequency,
                self.converged,
            ),
        ))
    }
}
