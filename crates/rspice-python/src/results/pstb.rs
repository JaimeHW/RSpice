//! Periodic stability results (`.PSTB`).
//!
//! The Floquet multiplier spectrum of one periodic orbit's monodromy matrix,
//! read at one loop probe: a mode-indexed spectrum, not a frequency sweep and
//! not a loop gain. Gain and phase margins belong to `.STB`, which is a
//! different analysis with a different card.
//!
//! The spectrum is always complete. A truncated spectrum cannot prove
//! stability, so the card's `NMULTS` is carried here as the display limit it
//! is rather than applied to the arrays below.

use super::*;

/// Floquet stability of a periodic orbit at one loop probe.
#[pyclass(name = "PstbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPstbResult {
    inner: rspice_core::engine::PeriodicStabilityResult,
    /// The authored card, kept because it states the display limit the deck
    /// asked for, which the spectrum itself does not carry.
    card: rspice_core::netlist::PstbCard,
    evidence: Option<DocumentEvidence<()>>,
}

impl CarriesDocumentEvidence for PyPstbResult {
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

impl PyPstbResult {
    pub(crate) fn from_core(
        card: &rspice_core::netlist::PstbCard,
        stability: &rspice_core::engine::PeriodicStabilityResult,
    ) -> Self {
        Self {
            inner: stability.clone(),
            card: card.clone(),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Pstb,
                (),
            )),
        }
    }

    /// The shared result document, projected from the retained spectrum.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let (analysis, coordinate) = document::execution(&self.evidence, "PSTB")?;
        let card = &self.card;
        let inner = &self.inner;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_pstb(analysis, card, inner)
        })
    }

    fn per_mode<'py, T>(
        &self,
        py: Python<'py>,
        extract: impl Fn(&rspice_core::analysis::pstb::FloquetMultiplier) -> T,
    ) -> Bound<'py, PyArray1<T>>
    where
        T: numpy::Element,
    {
        self.inner
            .result
            .multipliers
            .iter()
            .map(extract)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }
}

#[pymethods]
impl PyPstbResult {
    /// Period of the analyzed orbit, in seconds.
    #[getter]
    fn period(&self) -> f64 {
        self.inner.result.period
    }

    /// Fundamental frequency of the analyzed orbit, in Hz.
    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.result.fundamental_frequency
    }

    /// Canonical circuit spelling of the loop probe.
    #[getter]
    fn probe_instance(&self) -> String {
        self.inner.probe_instance.clone()
    }

    /// The probe's coordinate in the carrier's shooting-state basis, which is
    /// what the monodromy and every mode shape are indexed by.
    #[getter]
    fn probe_state_index(&self) -> usize {
        self.inner.probe_state_index
    }

    /// Every Floquet multiplier, in canonical order: magnitude descending.
    #[getter]
    fn multipliers<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<Complex64>> {
        self.per_mode(py, |mode| mode.value)
    }

    /// Every Floquet exponent, `ln(lambda) / T`, in the same order.
    #[getter]
    fn exponents<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<Complex64>> {
        self.per_mode(py, |mode| mode.exponent)
    }

    /// Multiplier magnitudes.
    #[getter]
    fn magnitudes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.per_mode(
            py,
            rspice_core::analysis::pstb::FloquetMultiplier::magnitude,
        )
    }

    /// Multiplier phases in degrees.
    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.per_mode(
            py,
            rspice_core::analysis::pstb::FloquetMultiplier::phase_degrees,
        )
    }

    /// Per-mode stability margin in dB, `20 log10(1 / |lambda|)`.
    #[getter]
    fn stability_margins_db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.per_mode(
            py,
            rspice_core::analysis::pstb::FloquetMultiplier::stability_margin_db,
        )
    }

    /// Per-mode damping in 1/s.
    #[getter]
    fn mode_damping<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.per_mode(py, rspice_core::analysis::pstb::FloquetMultiplier::damping)
    }

    /// Per-mode natural frequency in Hz.
    #[getter]
    fn mode_frequencies_hz<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.per_mode(
            py,
            rspice_core::analysis::pstb::FloquetMultiplier::natural_frequency,
        )
    }

    /// The loop probe's normalized share of each mode shape.
    #[getter]
    fn probe_participation<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.probe_participation.to_pyarray(py)
    }

    /// Which modes sit outside the outer stability boundary.
    #[getter]
    fn unstable_modes(&self) -> Vec<bool> {
        self.inner
            .result
            .multipliers
            .iter()
            .map(|mode| mode.is_unstable)
            .collect()
    }

    /// Number of modes outside the boundary, excluding any exempted phase mode.
    #[getter]
    fn num_unstable(&self) -> usize {
        self.inner.result.num_unstable
    }

    /// Largest multiplier magnitude over the complete spectrum.
    #[getter]
    fn max_multiplier_magnitude(&self) -> f64 {
        self.inner.result.max_multiplier_magnitude
    }

    /// Smallest margin over the applicable modes, absent when there is none —
    /// an autonomous spectrum holding only its phase mode.
    #[getter]
    fn min_stability_margin_db(&self) -> Option<f64> {
        self.inner.result.min_stability_margin_db
    }

    /// Detected subharmonic orders over the complete spectrum.
    #[getter]
    fn subharmonics(&self) -> Vec<usize> {
        self.inner.result.subharmonics.clone()
    }

    /// The one exempted autonomous phase mode, when one was qualified.
    #[getter]
    fn trivial_multiplier_index(&self) -> Option<usize> {
        self.inner.result.trivial_multiplier_index
    }

    /// Outer magnitude boundary the modes were classified against.
    #[getter]
    fn stability_threshold(&self) -> f64 {
        self.inner.result.stability_threshold
    }

    /// Whether subharmonic orders were looked for.
    #[getter]
    fn detect_subharmonics(&self) -> bool {
        self.inner.result.detect_subharmonics
    }

    /// Multipliers the card asked a viewer to show. The arrays above are the
    /// complete spectrum regardless.
    #[getter]
    fn num_multipliers(&self) -> usize {
        self.card.num_multipliers
    }

    /// The refined stability determination.
    #[getter]
    fn stability(&self) -> &'static str {
        use rspice_core::analysis::pstb::StabilityType;

        match self.inner.result.stability {
            StabilityType::Stable => "stable",
            StabilityType::UnstableReal => "unstable-real",
            StabilityType::UnstableComplex => "unstable-complex",
            StabilityType::PeriodDoubling => "period-doubling",
            StabilityType::NeimarkSacker => "neimark-sacker",
            StabilityType::SaddleNode => "saddle-node",
            StabilityType::Marginal => "marginal",
            // `StabilityType` is `#[non_exhaustive]`: a label this build has no
            // word for is reported as an unestablished determination rather
            // than as one of the labels it is not.
            _ => "indeterminate",
        }
    }

    /// Whether the shared four-state verdict is stable.
    #[getter]
    fn is_stable(&self) -> bool {
        self.inner.result.is_stable()
    }

    /// Whether the qualified eigensolve completed.
    #[getter]
    fn converged(&self) -> bool {
        self.inner.result.converged
    }

    /// Typed inventory of every signal in this result's shared document.
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
            "PstbResult(probe={}, modes={}, {}, unstable={})",
            self.inner.probe_instance,
            self.inner.result.multipliers.len(),
            self.stability(),
            self.inner.result.num_unstable
        )
    }
}
