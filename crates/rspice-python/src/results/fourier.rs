//! `.FOUR` spectra derived from transient waveforms.
//!
//! These live apart from the small-signal frequency-domain results because they
//! are a post-processing step over a time-domain run, not an analysis in their
//! own right: the harmonics come from a DFT of a sampled waveform, so their
//! accuracy is governed by the transient timestep rather than by a solver
//! tolerance.

use super::*;

type PyFourierProvenance = (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<PyRunCoordinate>,
);

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

/// Fourier analysis result (harmonic decomposition + optional THD)
///
/// Example:
///     >>> four = tran.fourier("out", fundamental=1e3)
///     >>> print("undefined" if four.thd_percent is None else f"{four.thd_percent:.3f}%")
///     >>> for h in four.harmonics:
///     ...     print(h)
#[pyclass(name = "FourierResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFourierResult {
    /// DC component of the waveform
    #[pyo3(get)]
    pub dc_component: f64,
    /// Total harmonic distortion as a ratio (0-1), or `None` when the
    /// fundamental magnitude is exactly zero.
    #[pyo3(get)]
    pub thd: Option<f64>,
    /// Authored output expression analyzed by a `.FOUR` directive.
    #[pyo3(get)]
    pub source_signal: Option<String>,
    /// Stable identity of the authored `.FOUR` post-process request.
    #[pyo3(get)]
    pub analysis_id: Option<String>,
    /// Stable identity of the transient trajectory consumed by `.FOUR`.
    #[pyo3(get)]
    pub parent_analysis_id: Option<String>,
    /// Materialized run coordinate of the parent transient trajectory.
    #[pyo3(get)]
    pub coordinate: Option<PyRunCoordinate>,
    harmonics: Vec<PyHarmonic>,
    /// The core spectrum plus the parent transient and probed column the
    /// shared document names; the Python projection drops core's DC harmonic
    /// row and rescales THD, so neither can be read back from it.
    evidence: Option<DocumentEvidence<FourierEvidence>>,
}

/// What a `.FOUR` document needs beyond the spectrum's own identity.
#[derive(Debug, Clone)]
pub(crate) struct FourierEvidence {
    parent: rspice_core::execution::AnalysisInstanceId,
    output: String,
    output_unit: rspice_core::execution::SignalUnit,
    result: rspice_core::analysis::FourierResult,
}

impl CarriesDocumentEvidence for PyFourierResult {
    fn bind_analysis(&mut self, analysis: rspice_core::execution::AnalysisInstanceId) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_analysis(analysis));
    }
}

/// Declared unit of one authored `.FOUR` operand.
///
/// `V(...)` and `I(...)` are the parser's own probe grammar. Anything else is
/// a device observable or a braced parameter expression, whose unit the deck
/// never declared — which is `Unspecified` and not dimensionless, exactly as
/// core's own transient-output rule states it.
/// Zero-based ordinal of a canonical `four-NNN` tag, when it is one.
///
/// The directive runner numbers `.FOUR` operands in authored order and
/// formats the tag; reading the ordinal back is how the same numbering reaches
/// the shared document without a second counter.
fn analysis_ordinal(tag: &str) -> Option<u32> {
    tag.rsplit_once('-')
        .and_then(|(_, ordinal)| ordinal.parse::<u32>().ok())
        .and_then(|ordinal| ordinal.checked_sub(1))
}

fn fourier_output_unit(output: &str) -> rspice_core::execution::SignalUnit {
    let trimmed = output.trim();
    if trimmed.len() >= 2 && trimmed[1..].starts_with('(') {
        match trimmed.as_bytes()[0].to_ascii_uppercase() {
            b'V' => return rspice_core::execution::SignalUnit::Volt,
            b'I' => return rspice_core::execution::SignalUnit::Ampere,
            _ => {}
        }
    }
    rspice_core::execution::SignalUnit::Unspecified
}

impl PyFourierResult {
    /// Name the transient column this spectrum was taken from.
    ///
    /// The shared document names the operand a `.FOUR` card analyzed, so a
    /// spectrum that has not been told which column it came from has no
    /// document to publish.
    pub(crate) fn with_output(mut self, output: &str) -> Self {
        self.source_signal = Some(output.to_owned());
        if let Some(evidence) = self.evidence.as_mut() {
            evidence.core.output = output.to_owned();
            evidence.core.output_unit = fourier_output_unit(output);
        }
        self
    }

    /// The shared result document, projected from the retained spectrum.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "Fourier")?;
        let analysis = evidence.analysis;
        let core = &evidence.core;
        if core.output.trim().is_empty() {
            return Err(crate::errors::SimulationError::new_err(
                "this Fourier result names no analyzed output, which the shared result document \
                 requires",
            ));
        }
        document::build(py, |abort| {
            AnalysisResultDocument::from_fourier(
                analysis,
                core.parent,
                &core.output,
                core.output_unit.clone(),
                &core.result,
            )?
            .build_with_abort(abort)
        })
    }

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
            thd: result.thd.map(|value| value / 100.0),
            source_signal: None,
            analysis_id: None,
            parent_analysis_id: None,
            coordinate: None,
            harmonics,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Fourier,
                FourierEvidence {
                    parent: rspice_core::execution::sole_analysis_identity(
                        rspice_core::execution::AnalysisKind::Tran,
                    ),
                    output: String::new(),
                    output_unit: rspice_core::execution::SignalUnit::Unspecified,
                    result: result.clone(),
                },
            )),
        }
    }

    pub fn from_core_with_provenance(
        result: &rspice_core::analysis::FourierResult,
        source_signal: String,
        analysis_id: String,
        parent_analysis_id: Option<String>,
        coordinate: Option<PyRunCoordinate>,
    ) -> Self {
        let ordinal = analysis_ordinal(&analysis_id);
        let mut projected = Self::from_core(result).with_output(&source_signal);
        projected.analysis_id = Some(analysis_id);
        projected.parent_analysis_id = parent_analysis_id;
        projected.coordinate = coordinate;
        if let Some(ordinal) = ordinal {
            projected.bind_analysis(rspice_core::execution::analysis_instance_identity(
                rspice_core::execution::AnalysisKind::Fourier,
                ordinal,
            ));
        }
        projected
    }
}

#[pymethods]
impl PyFourierResult {
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

    /// Total harmonic distortion in percent
    #[getter]
    fn thd_percent(&self) -> Option<f64> {
        self.thd.map(|value| value * 100.0)
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
        let thd = self
            .thd
            .map(|value| format!("{:.4}%", value * 100.0))
            .unwrap_or_else(|| "undefined".to_owned());
        format!(
            "FourierResult(harmonics={}, dc={:.4e}, thd={})",
            self.harmonics.len(),
            self.dc_component,
            thd
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[pyo3(signature = (dc_component, thd, harmonics, provenance=None))]
    fn _unpickle(
        dc_component: f64,
        thd: Option<f64>,
        harmonics: Vec<PyHarmonic>,
        provenance: Option<PyFourierProvenance>,
    ) -> Self {
        let (source_signal, analysis_id, parent_analysis_id, coordinate) =
            provenance.unwrap_or((None, None, None, None));
        Self {
            dc_component,
            thd,
            source_signal,
            analysis_id,
            parent_analysis_id,
            coordinate,
            harmonics,
            evidence: None,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (f64, Option<f64>, Vec<PyHarmonic>, PyFourierProvenance),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.dc_component,
                self.thd,
                self.harmonics.clone(),
                (
                    self.source_signal.clone(),
                    self.analysis_id.clone(),
                    self.parent_analysis_id.clone(),
                    self.coordinate.clone(),
                ),
            ),
        ))
    }
}
