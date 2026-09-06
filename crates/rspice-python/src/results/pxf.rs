//! Periodic transfer-function results (`.PXF`).
//!
//! One conversion path out of the matrix a periodic AC solve fills: the
//! complex transfer from a named source at one sideband to a named probe at
//! another, over the offset-frequency sweep the card asked for, with the
//! absolute frequency the converted response appears at beside it.
//!
//! The four curve metrics are `Option`s rather than sentinels. A transfer that
//! never falls 3 dB below its peak has no -3 dB bandwidth, and reporting zero
//! there would name DC as the edge of a band the circuit does not have.

use super::*;

/// Periodic transfer function between one input and one output sideband.
#[pyclass(name = "PxfResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPxfResult {
    inner: rspice_core::analysis::pxf::PxfResult,
    /// The authored card, kept because it states what was measured — the
    /// driving source and the probe — which the numbers alone do not say.
    card: rspice_core::netlist::PxfCard,
    evidence: Option<DocumentEvidence<()>>,
}

impl CarriesDocumentEvidence for PyPxfResult {
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

impl PyPxfResult {
    pub(crate) fn from_core(
        card: &rspice_core::netlist::PxfCard,
        result: &rspice_core::analysis::pxf::PxfResult,
    ) -> Self {
        Self {
            inner: result.clone(),
            card: card.clone(),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Pxf,
                (),
            )),
        }
    }

    /// The shared result document, projected from the retained transfer.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let (analysis, coordinate) = document::execution(&self.evidence, "PXF")?;
        let card = &self.card;
        let inner = &self.inner;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_pxf(analysis, card, inner)
        })
    }
}

#[pymethods]
impl PyPxfResult {
    /// Swept offset frequencies in Hz: the abscissa the card authored.
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .points
            .iter()
            .map(|point| point.freq_in)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Absolute frequency the converted response appears at, in Hz.
    #[getter]
    fn output_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .points
            .iter()
            .map(|point| point.freq_out)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Complex transfer at every swept offset.
    #[getter]
    fn transfer<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<Complex64>> {
        self.inner
            .points
            .iter()
            .map(|point| point.transfer)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Transfer magnitude in dB at every swept offset.
    #[getter]
    fn magnitude_db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .points
            .iter()
            .map(|point| point.magnitude_db())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Transfer phase in degrees at every swept offset.
    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .points
            .iter()
            .map(|point| point.phase_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Midpoint group-delay curve as `(frequency_hz, delay_seconds)` pairs.
    ///
    /// One shorter than the sweep and on a grid of its own, because a
    /// difference-derived delay belongs between two samples rather than at one.
    #[getter]
    fn group_delay(&self) -> Vec<(f64, f64)> {
        self.inner.group_delay_curve()
    }

    /// Large-signal fundamental of the carrier, in Hz.
    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.fundamental_freq
    }

    /// Sideband the drive was applied at.
    #[getter]
    fn input_sideband(&self) -> i32 {
        self.inner.input_sideband
    }

    /// Sideband the response was read at.
    #[getter]
    fn output_sideband(&self) -> i32 {
        self.inner.output_sideband
    }

    /// Conversion depth the solve spanned: sidebands `-n..=n` participated.
    #[getter]
    fn max_sideband(&self) -> i32 {
        self.card.max_sideband
    }

    /// Independent source the transfer was driven from.
    #[getter]
    fn input_source(&self) -> String {
        self.card.input_source.clone()
    }

    /// Probe node the transfer was read at.
    #[getter]
    fn output_node(&self) -> String {
        self.card.output_node.clone()
    }

    /// Reference node of a differential probe, `None` for a single-ended one.
    #[getter]
    fn reference_node(&self) -> Option<String> {
        self.card.output_ref.clone()
    }

    /// Peak transfer magnitude in dB, absent when no sample has a finite one.
    #[getter]
    fn peak_gain_db(&self) -> Option<f64> {
        self.inner.peak_gain.map(|(_, gain_db)| gain_db)
    }

    /// Offset frequency the peak occurs at, in Hz.
    #[getter]
    fn peak_gain_frequency(&self) -> Option<f64> {
        self.inner.peak_gain.map(|(frequency, _)| frequency)
    }

    /// Width of the -3 dB band around the peak, in Hz, absent when the curve
    /// never falls 3 dB below it.
    #[getter]
    fn bandwidth_3db(&self) -> Option<f64> {
        self.inner.bandwidth_3db
    }

    /// Offset frequency of the 0 dB crossing, absent when there is none.
    #[getter]
    fn unity_gain_frequency(&self) -> Option<f64> {
        self.inner.unity_gain_freq
    }

    /// Transfer at the lowest swept offset, when the sweep reaches low enough
    /// for the curve to have a DC end.
    #[getter]
    fn dc_gain(&self) -> Option<PyComplexValue> {
        self.inner.dc_gain.as_ref().map(PyComplexValue::from_core)
    }

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

    fn __repr__(&self) -> String {
        format!(
            "PxfResult(fundamental={:.6e}Hz, points={}, sideband {}->{}, out={})",
            self.inner.fundamental_freq,
            self.inner.points.len(),
            self.inner.input_sideband,
            self.inner.output_sideband,
            self.card.output_node
        )
    }
}
