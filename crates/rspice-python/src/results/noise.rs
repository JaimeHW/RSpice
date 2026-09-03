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
    /// The whole `.NOISE` sweep this point belongs to.
    ///
    /// The shared result document describes an analysis, and a `.NOISE`
    /// analysis is the sweep — this class is one of its rows. Every row of one
    /// sweep therefore shares one document, held once behind an `Arc` rather
    /// than copied per point.
    evidence: Option<DocumentEvidence<std::sync::Arc<Vec<rspice_core::analysis::NoiseResult>>>>,
}

impl CarriesDocumentEvidence for PyNoiseResult {
    fn bind_analysis(&mut self, analysis: rspice_core::execution::AnalysisInstanceId) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_analysis(analysis));
    }
}

impl PyNoiseResult {
    /// Project one complete `.NOISE` sweep, one row per solved frequency.
    ///
    /// Every row keeps a reference to the sweep it came from, so `signals()`,
    /// `scalars()` and `document()` describe the analysis rather than a
    /// one-point sweep that never ran.
    pub fn sweep_from_core(results: &[rspice_core::analysis::NoiseResult]) -> Vec<Self> {
        let sweep = std::sync::Arc::new(results.to_vec());
        results
            .iter()
            .map(|result| {
                let mut point = Self::from_core(result);
                point.evidence = Some(DocumentEvidence::sole(
                    rspice_core::execution::AnalysisKind::Noise,
                    std::sync::Arc::clone(&sweep),
                ));
                point
            })
            .collect()
    }

    /// The shared result document of the sweep this row belongs to.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "noise")?;
        let analysis = evidence.analysis;
        let sweep = evidence.core.as_slice();
        document::build(py, |abort| {
            AnalysisResultDocument::from_noise(analysis, sweep)?.build_with_abort(abort)
        })
    }

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
            // A single row on its own is not an analysis; only
            // `sweep_from_core` knows the sweep this row belongs to.
            evidence: None,
        }
    }
}

#[pymethods]
impl PyNoiseResult {
    /// Typed inventory of every signal in this sweep's shared document.
    ///
    /// A `.NOISE` analysis is the whole frequency sweep, and this row is one
    /// of its points, so the inventory describes the sweep. The descriptors
    /// are the ones the CLI, the WASM build and the engine adapter publish.
    fn signals(&self, py: Python<'_>) -> PyResult<Vec<PySignalDescriptor>> {
        Ok(document::signals(&self.shared_document(py)?))
    }

    /// Every analysis-owned scalar this sweep publishes, with its unit.
    fn scalars(&self, py: Python<'_>) -> PyResult<Vec<PyResultScalar>> {
        Ok(document::scalars(&self.shared_document(py)?))
    }

    /// Every per-device observable history this sweep captured.
    fn device_observables(&self, py: Python<'_>) -> PyResult<Vec<PyDeviceObservable>> {
        Ok(document::device_observables(&self.shared_document(py)?))
    }

    /// The whole shared result document as JSON-serializable Python data.
    fn document<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        document::json_view(py, &self.shared_document(py)?)
    }

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
            evidence: None,
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
    /// The core run, kept because the shared document's per-contributor
    /// offset grids and jitter band are not part of this projection.
    evidence: Option<DocumentEvidence<rspice_core::engine::PeriodicNoiseResult>>,
}

impl CarriesDocumentEvidence for PyPeriodicNoiseResult {
    fn bind_analysis(&mut self, analysis: rspice_core::execution::AnalysisInstanceId) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_analysis(analysis));
    }
}

impl CarriesDocumentEvidence for PyOscillatorNoiseResult {
    fn bind_analysis(&mut self, analysis: rspice_core::execution::AnalysisInstanceId) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_analysis(analysis));
    }
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
    /// The core run and the probe it measured, which the shared document
    /// records as the phase-diffusion evidence of a `pnoise` result.
    evidence: Option<DocumentEvidence<rspice_core::engine::PeriodicNoiseResult>>,
}

impl PyOscillatorNoiseResult {
    /// Project one autonomous run, naming the probe it measured.
    pub(crate) fn from_run(output: &str, result: &rspice_core::engine::OscPnoiseResult) -> Self {
        Self {
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::PNoise,
                rspice_core::engine::PeriodicNoiseResult::Oscillator {
                    output: output.to_owned(),
                    result: result.clone(),
                },
            )),
            ..Self::from_core(result)
        }
    }

    pub fn from_core(result: &rspice_core::engine::OscPnoiseResult) -> Self {
        Self {
            frequencies: result.frequencies.clone(),
            phase_noise_dbc: result.phase_noise_dbc.clone(),
            diffusion_constant: result.diffusion_constant,
            period: result.period,
            corner_frequency: result.corner_hz,
            // Only a route that knows the authored output probe can publish
            // this run as a `pnoise` document; the probe is not derivable
            // from the spectrum.
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained run.
    ///
    /// The `pnoise` document names the probe the spectrum is referred to, so a
    /// run that reached this surface without one — a direct
    /// `run_oscillator_noise` call, which analyzes the orbit itself and takes
    /// no probe, or a result restored from pickled state — says so rather than
    /// publishing a document under an invented output name.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = self.evidence.as_ref().ok_or_else(|| {
            crate::errors::not_implemented_error(
                "this oscillator phase-noise result names no output probe, so it has no shared \
                 pnoise document; Engine.run publishes one for an authored .PNOISE card around \
                 an autonomous .PSS carrier",
            )
        })?;
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, |abort| {
            AnalysisResultDocument::from_pnoise(analysis, result)?.build_with_abort(abort)
        })
    }
}

#[pymethods]
impl PyOscillatorNoiseResult {
    /// Typed inventory of every signal in this result's shared document.
    ///
    /// The descriptors are the ones the CLI, the WASM build and the engine
    /// adapter publish, so a canonical name, unit, owner, or availability
    /// means the same thing on every surface.
    fn signals(&self, py: Python<'_>) -> PyResult<Vec<PySignalDescriptor>> {
        Ok(document::signals(&self.shared_document(py)?))
    }

    /// Every analysis-owned scalar this result publishes, with its unit.
    fn scalars(&self, py: Python<'_>) -> PyResult<Vec<PyResultScalar>> {
        Ok(document::scalars(&self.shared_document(py)?))
    }

    /// Every per-device observable history this result captured.
    fn device_observables(&self, py: Python<'_>) -> PyResult<Vec<PyDeviceObservable>> {
        Ok(document::device_observables(&self.shared_document(py)?))
    }

    /// The whole shared result document as JSON-serializable Python data.
    ///
    /// The document's `pnoise` payload carries the Demir phase-diffusion
    /// evidence — the diffusion constant, the solved period and the Lorentzian
    /// corner — beside the published dBc/Hz spectrum.
    fn document<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        document::json_view(py, &self.shared_document(py)?)
    }

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
            evidence: None,
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
    /// Project one driven run, naming the probe it measured.
    pub(crate) fn from_run(
        output: &str,
        result: &rspice_core::engine::PnoiseAnalysisResult,
    ) -> Self {
        Self {
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::PNoise,
                rspice_core::engine::PeriodicNoiseResult::Driven {
                    output: output.to_owned(),
                    result: result.clone(),
                },
            )),
            ..Self::from_core(result)
        }
    }

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
            // Only a route that knows the authored output probe can publish
            // this run as a `pnoise` document.
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained run.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "periodic-noise")?;
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, |abort| {
            AnalysisResultDocument::from_pnoise(analysis, result)?.build_with_abort(abort)
        })
    }
}

#[pymethods]
impl PyPeriodicNoiseResult {
    /// Typed inventory of every signal in this result's shared document.
    ///
    /// The descriptors are the ones the CLI, the WASM build and the engine
    /// adapter publish, so a canonical name, unit, owner, or availability
    /// means the same thing on every surface.
    fn signals(&self, py: Python<'_>) -> PyResult<Vec<PySignalDescriptor>> {
        Ok(document::signals(&self.shared_document(py)?))
    }

    /// Every analysis-owned scalar this result publishes, with its unit.
    fn scalars(&self, py: Python<'_>) -> PyResult<Vec<PyResultScalar>> {
        Ok(document::scalars(&self.shared_document(py)?))
    }

    /// Every per-device observable history this result captured.
    fn device_observables(&self, py: Python<'_>) -> PyResult<Vec<PyDeviceObservable>> {
        Ok(document::device_observables(&self.shared_document(py)?))
    }

    /// The whole shared result document as JSON-serializable Python data.
    fn document<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        document::json_view(py, &self.shared_document(py)?)
    }

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
            evidence: None,
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
