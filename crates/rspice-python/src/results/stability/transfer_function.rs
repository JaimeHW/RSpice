//! DC transfer-function result projection and persistence.

use super::*;

/// Small-signal transfer function result (.TF)
///
/// Example:
///     >>> tf = engine.run_transfer_function(netlist, "out", "V1")
///     >>> print(f"gain={tf.gain:.3f}, Zin={tf.input_impedance:.1f}Ω")
#[pyclass(name = "TransferFunctionResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransferFunctionResult {
    /// Output specification
    #[pyo3(get)]
    pub output: String,
    /// Input source name
    #[pyo3(get)]
    pub input: String,
    /// DC small-signal gain (output / input)
    #[pyo3(get)]
    pub gain: f64,
    /// Input impedance in Ohms
    #[pyo3(get)]
    pub input_impedance: f64,
    /// Output impedance (Thevenin) in Ohms
    #[pyo3(get)]
    pub output_impedance: f64,
    /// The core result, kept because this projection reports an infinite
    /// impedance as a plain float rather than as the typed determination the
    /// shared document publishes.
    evidence: Option<DocumentEvidence<rspice_core::analysis::TransferFunctionResult>>,
}

impl CarriesDocumentEvidence for PyTransferFunctionResult {
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

impl PyTransferFunctionResult {
    pub fn from_core(result: &rspice_core::analysis::TransferFunctionResult) -> Self {
        Self {
            output: result.output.clone(),
            input: result.input.clone(),
            gain: result.gain,
            input_impedance: result.input_impedance,
            output_impedance: result.output_impedance,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::TransferFunction,
                result.clone(),
            )),
        }
    }

    /// The shared result document, projected from the retained ratios.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "transfer-function")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_transfer_function(analysis, result)
        })
    }
}

#[pymethods]
impl PyTransferFunctionResult {
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

    /// Gain in dB (20·log10 |gain|)
    #[getter]
    fn gain_db(&self) -> f64 {
        20.0 * self.gain.abs().log10()
    }

    fn __repr__(&self) -> String {
        format!(
            "TransferFunctionResult({}/{}: gain={:.4e}, Zin={:.4e}, Zout={:.4e})",
            self.output, self.input, self.gain, self.input_impedance, self.output_impedance
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        output: String,
        input: String,
        gain: f64,
        input_impedance: f64,
        output_impedance: f64,
    ) -> Self {
        Self {
            output,
            input,
            gain,
            input_impedance,
            output_impedance,
            evidence: None,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (String, String, f64, f64, f64))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.output.clone(),
                self.input.clone(),
                self.gain,
                self.input_impedance,
                self.output_impedance,
            ),
        ))
    }
}
