//! Harmonic-balance envelope-following results (`.ENVELOPE`).
//!
//! An envelope run is two authenticated engine steps: a carrier periodic solve
//! with the slow sources frozen, then transient integration continued from that
//! carrier at slow-time origin zero. This type publishes exactly those two
//! halves and the artifact that binds them.
//!
//! It deliberately exposes no synthesized harmonic envelope series. The engine
//! does not compute them, and inventing them here would publish physics the
//! solver never solved.

use super::*;

/// One completed envelope-following run.
#[pyclass(name = "EnvelopeResult", module = "rspice")]
pub struct PyEnvelopeResult {
    carrier: PyHbResult,
    guarantee: &'static str,
    frozen_sources: Vec<String>,
    fundamental_frequency: f64,
    num_harmonics: usize,
    time_origin: f64,
    slow_time_duration: f64,
    slow_time_max_step: f64,
    continued_transient: Py<PyTransientResult>,
    final_checkpoint: Py<PyTransientCheckpoint>,
    /// The core continuation, kept because this projection splits it into a
    /// carrier, a transient and a checkpoint that cannot be recombined.
    evidence: Option<DocumentEvidence<rspice_core::engine::EnvelopeResult>>,
}

impl CarriesDocumentEvidence for PyEnvelopeResult {
    fn bind_execution(
        &mut self,
        analysis: rspice_core::execution::AnalysisInstanceId,
        coordinate: Option<&rspice_core::execution::ResultCoordinate>,
    ) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_execution(analysis, coordinate));
    }
}

/// Stable label for the completeness contract the continuation was solved to.
fn guarantee_label(guarantee: rspice_core::engine::HbEnvelopeStateGuarantee) -> &'static str {
    match guarantee {
        rspice_core::engine::HbEnvelopeStateGuarantee::ExactLinearRcMnaV1 => {
            "exact-linear-rc-mna-v1"
        }
    }
}

impl PyEnvelopeResult {
    pub(crate) fn from_core(
        py: Python<'_>,
        result: &rspice_core::engine::EnvelopeResult,
    ) -> PyResult<Self> {
        let state = result.state();
        Ok(Self {
            carrier: PyHbResult::from_spectra(result.carrier()),
            guarantee: guarantee_label(result.guarantee()),
            frozen_sources: state.canonical_frozen_sources().to_vec(),
            fundamental_frequency: state.fundamental_freq(),
            num_harmonics: state.num_harmonics(),
            time_origin: result.time_origin(),
            slow_time_duration: result.slow_time_duration(),
            slow_time_max_step: result.slow_time_max_step(),
            continued_transient: Py::new(
                py,
                PyTransientResult::new(result.continued_transient().clone()),
            )?,
            final_checkpoint: Py::new(
                py,
                PyTransientCheckpoint::new(result.final_checkpoint().clone()),
            )?,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Envelope,
                result.clone(),
            )),
        })
    }

    /// The shared result document, projected from the retained continuation.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "envelope")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_envelope(analysis, result)
        })
    }
}

#[pymethods]
impl PyEnvelopeResult {
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

    /// Converged carrier spectra the continuation started from.
    #[getter]
    fn carrier(&self) -> PyHbResult {
        self.carrier.clone()
    }

    /// Transient continued from the carrier at slow-time origin zero.
    #[getter]
    fn continued_transient(&self, py: Python<'_>) -> Py<PyTransientResult> {
        self.continued_transient.clone_ref(py)
    }

    /// Checkpoint at the end of the continued transient.
    #[getter]
    fn final_checkpoint(&self, py: Python<'_>) -> Py<PyTransientCheckpoint> {
        self.final_checkpoint.clone_ref(py)
    }

    /// Completeness contract the carrier-to-transient projection was solved to.
    ///
    /// This is not a quality score: it names the circuit subset for which the
    /// projection is exact, so a caller can tell an authenticated continuation
    /// from an approximation.
    #[getter]
    fn guarantee(&self) -> &'static str {
        self.guarantee
    }

    /// Independent sources held at their time-zero values during the carrier
    /// solve, canonicalized as the engine bound them.
    #[getter]
    fn frozen_sources(&self) -> Vec<String> {
        self.frozen_sources.clone()
    }

    /// Carrier fundamental frequency in Hz.
    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.fundamental_frequency
    }

    /// Harmonics retained by the carrier solve.
    #[getter]
    fn num_harmonics(&self) -> usize {
        self.num_harmonics
    }

    /// Slow-time origin the continuation restarted from, in seconds.
    #[getter]
    fn time_origin(&self) -> f64 {
        self.time_origin
    }

    /// Slow-time interval the continuation integrated, in seconds.
    #[getter]
    fn duration(&self) -> f64 {
        self.slow_time_duration
    }

    /// Maximum slow-time step the continuation was allowed, in seconds.
    #[getter]
    fn max_step(&self) -> f64 {
        self.slow_time_max_step
    }

    fn __repr__(&self) -> String {
        format!(
            "EnvelopeResult(fundamental={:.6e} Hz, harmonics={}, duration={:.6e} s, frozen_sources={:?})",
            self.fundamental_frequency,
            self.num_harmonics,
            self.slow_time_duration,
            self.frozen_sources
        )
    }
}
