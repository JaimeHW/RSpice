//! Loop stability, pole-zero, and transfer-function results.
//!
//! `StbResult` carries the Tian loop-gain probe (`.STB`), which measures a
//! feedback loop without breaking it. `PoleZeroResult` carries the roots of the
//! small-signal network, and `TransferFunctionResult` the DC transfer gain with
//! its input and output resistances. Measured loop margins alone do not
//! establish closed-loop stability.

use super::*;

/// Numerical qualification certificate for a complete eigenspectrum.
#[pyclass(name = "SpectrumCertificate", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySpectrumCertificate {
    #[pyo3(get)]
    pub problem_order: usize,
    #[pyo3(get)]
    pub infinite_count: usize,
    #[pyo3(get)]
    pub max_backward_error: f64,
    #[pyo3(get)]
    pub qualification_tolerance: f64,
}

impl PySpectrumCertificate {
    fn from_core(certificate: &rspice_core::analysis::SpectrumCertificate) -> Self {
        Self {
            problem_order: certificate.problem_order,
            infinite_count: certificate.infinite_count,
            max_backward_error: certificate.max_backward_error,
            qualification_tolerance: certificate.qualification_tolerance,
        }
    }

    fn to_state(&self) -> SpectrumCertificateState {
        (
            self.problem_order,
            self.infinite_count,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }
}

#[pymethods]
impl PySpectrumCertificate {
    /// Number of finite roots certified by the finite/infinite accounting.
    #[getter]
    fn finite_count(&self) -> usize {
        self.problem_order.saturating_sub(self.infinite_count)
    }

    /// Whether the worst backward error satisfies the strict threshold.
    #[getter]
    fn is_strictly_qualified(&self) -> bool {
        self.max_backward_error <= self.qualification_tolerance
    }

    fn __repr__(&self) -> String {
        format!(
            "SpectrumCertificate(order={}, finite={}, infinite={}, max_backward_error={:.3e}, tolerance={:.3e})",
            self.problem_order,
            self.finite_count(),
            self.infinite_count,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        problem_order: usize,
        infinite_count: usize,
        max_backward_error: f64,
        qualification_tolerance: f64,
    ) -> PyResult<Self> {
        let certificate = spectrum_certificate_from_state((
            problem_order,
            infinite_count,
            max_backward_error,
            qualification_tolerance,
        ))?;
        Ok(Self::from_core(&certificate))
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, SpectrumCertificateState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}

/// Completeness and numerical evidence for one returned pole or zero set.
#[pyclass(name = "RootSetEvidence", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyRootSetEvidence {
    #[pyo3(get)]
    pub kind: String,
    certificate: Option<PySpectrumCertificate>,
}

impl PyRootSetEvidence {
    fn from_core(evidence: &rspice_core::analysis::RootSetEvidence) -> PyResult<Self> {
        let (kind, certificate) = root_set_evidence_state(evidence)?;
        Ok(Self {
            kind,
            certificate: certificate.map(
                |(problem_order, infinite_count, max_backward_error, qualification_tolerance)| {
                    PySpectrumCertificate {
                        problem_order,
                        infinite_count,
                        max_backward_error,
                        qualification_tolerance,
                    }
                },
            ),
        })
    }

    fn to_state(&self) -> RootSetEvidenceState {
        (
            self.kind.clone(),
            self.certificate
                .as_ref()
                .map(PySpectrumCertificate::to_state),
        )
    }
}

#[pymethods]
impl PyRootSetEvidence {
    #[getter]
    fn certificate(&self) -> Option<PySpectrumCertificate> {
        self.certificate.clone()
    }

    /// True only for a strictly qualified complete root set.
    #[getter]
    fn is_qualified(&self) -> bool {
        matches!(self.kind.as_str(), "qualified" | "qualified_empty")
    }

    fn __repr__(&self) -> String {
        format!(
            "RootSetEvidence(kind='{}', certificate={})",
            self.kind,
            if self.certificate.is_some() {
                "present"
            } else {
                "None"
            }
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(kind: String, certificate: Option<SpectrumCertificateState>) -> PyResult<Self> {
        let evidence = root_set_evidence_from_state((kind, certificate))?;
        Self::from_core(&evidence)
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, RootSetEvidenceState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}

/// Pole-zero analysis result
///
/// Contains poles and zeros of a circuit's transfer function.
///
/// Note: `run_pz` defaults to injecting a unit *current* at the input node,
/// so `dc_gain` is a transimpedance (V/A) rather than a voltage ratio unless
/// the call passed `input_type="voltage"`. Pole/zero locations are
/// input-independent either way.
///
/// Example:
///     >>> result = engine.run_pz(netlist, input_node="in", output_node="out")
///     >>> print(f"Stable: {result.is_stable}")
///     >>> for pole in result.poles:
///     ...     print(f"Pole: {pole}")
#[pyclass(name = "PoleZeroResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPoleZeroResult {
    /// System poles (natural frequencies)
    poles: Vec<PyComplexValue>,
    /// System zeros
    zeros: Vec<PyComplexValue>,
    /// Completeness and numerical evidence for the pole vector.
    pole_evidence: PyRootSetEvidence,
    /// Completeness and numerical evidence for the zero vector.
    zero_evidence: PyRootSetEvidence,
    /// Finite DC gain H(0), when available: a transimpedance in V/A for the
    /// default unit-current input, or a dimensionless voltage ratio for a
    /// unit-voltage input.
    #[pyo3(get)]
    pub dc_gain: Option<f64>,
    /// High-frequency gain H(∞) if finite
    #[pyo3(get)]
    pub hf_gain: Option<f64>,
    /// Unit symbol for both gains; absent in old pickles.
    #[pyo3(get)]
    pub gain_unit: Option<String>,
    /// Input specification
    #[pyo3(get)]
    pub input: String,
    /// Output specification
    #[pyo3(get)]
    pub output: String,
    /// The core result, kept because this projection splits complex roots
    /// into its own value type and labels evidence kinds as strings.
    evidence: Option<DocumentEvidence<rspice_core::analysis::PoleZeroResult>>,
}

impl CarriesDocumentEvidence for PyPoleZeroResult {
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

impl PyPoleZeroResult {
    pub fn from_core(result: &rspice_core::analysis::PoleZeroResult) -> PyResult<Self> {
        Ok(Self {
            poles: result.poles.iter().map(PyComplexValue::from_core).collect(),
            zeros: result.zeros.iter().map(PyComplexValue::from_core).collect(),
            pole_evidence: PyRootSetEvidence::from_core(&result.pole_evidence)?,
            zero_evidence: PyRootSetEvidence::from_core(&result.zero_evidence)?,
            dc_gain: result.dc_gain,
            hf_gain: result.hf_gain,
            gain_unit: Some(result.gain_unit.symbol()),
            input: result.input.clone(),
            output: result.output.clone(),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::PoleZero,
                result.clone(),
            )),
        })
    }

    /// The shared result document, projected from the retained root sets.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "pole-zero")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_pole_zero(analysis, result)
        })
    }
}

#[pymethods]
impl PyPoleZeroResult {
    /// Unit of every pole and zero, without conversion to Hz.
    #[getter]
    fn root_unit(&self) -> String {
        rspice_core::execution::SignalUnit::RadianPerSecond.symbol()
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

    /// Get all poles
    #[getter]
    fn poles(&self) -> Vec<PyComplexValue> {
        self.poles.clone()
    }

    /// Get all zeros
    #[getter]
    fn zeros(&self) -> Vec<PyComplexValue> {
        self.zeros.clone()
    }

    /// Completeness and numerical evidence for the pole vector.
    #[getter]
    fn pole_evidence(&self) -> PyRootSetEvidence {
        self.pole_evidence.clone()
    }

    /// Completeness and numerical evidence for the zero vector.
    #[getter]
    fn zero_evidence(&self) -> PyRootSetEvidence {
        self.zero_evidence.clone()
    }

    /// Get all poles as a complex128 NumPy array
    #[getter]
    fn poles_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        let values: Vec<rspice_core::Complex64> = self
            .poles
            .iter()
            .map(|p| rspice_core::Complex64::new(p.real, p.imag))
            .collect();
        values.to_pyarray(py)
    }

    /// Get all zeros as a complex128 NumPy array
    #[getter]
    fn zeros_array<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        let values: Vec<rspice_core::Complex64> = self
            .zeros
            .iter()
            .map(|z| rspice_core::Complex64::new(z.real, z.imag))
            .collect();
        values.to_pyarray(py)
    }

    /// Get real poles only (as list)
    fn real_poles(&self) -> Vec<PyComplexValue> {
        self.poles.iter().filter(|p| p.is_real()).copied().collect()
    }

    /// Get complex poles only (as list)
    fn complex_poles(&self) -> Vec<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| !p.is_real())
            .copied()
            .collect()
    }

    /// Check asymptotic stability: every pole is finite and strictly in the
    /// open left half-plane. Marginal poles are not reported as stable.
    #[getter]
    fn is_stable(&self) -> Option<bool> {
        if !self.pole_evidence.is_qualified() {
            return None;
        }
        if self
            .poles
            .iter()
            .any(|pole| !pole.real.is_finite() || !pole.imag.is_finite())
        {
            return None;
        }
        Some(self.poles.iter().all(|pole| pole.real < 0.0))
    }

    /// Get the dominant pole (closest to imaginary axis with Re < 0)
    fn dominant_pole(&self) -> Option<PyComplexValue> {
        self.poles
            .iter()
            .filter(|p| p.real < 0.0 && p.real.is_finite())
            .min_by(|a, b| a.real.abs().total_cmp(&b.real.abs()))
            .copied()
    }

    /// Decay frequency `|Re(p_dominant)| / 2π` in Hz.
    ///
    /// This is a pole metric, not a general 3 dB bandwidth.
    #[getter]
    fn dominant_pole_decay_hz(&self) -> Option<f64> {
        self.dominant_pole()
            .map(|p| p.real.abs() / (2.0 * std::f64::consts::PI))
    }

    /// Exact 3 dB bandwidth for the special one-real-pole/no-zero case.
    ///
    /// Returns None for higher-order or zero-containing transfer functions;
    /// use an AC sweep to compute their actual bandwidth.
    #[getter]
    fn bandwidth_hz(&self) -> Option<f64> {
        if self.zeros.is_empty() && matches!(self.poles.as_slice(), [only] if only.is_real()) {
            self.dominant_pole_decay_hz()
        } else {
            None
        }
    }

    /// Get number of poles
    #[getter]
    fn num_poles(&self) -> usize {
        self.poles.len()
    }

    /// Get number of zeros
    #[getter]
    fn num_zeros(&self) -> usize {
        self.zeros.len()
    }

    fn __repr__(&self) -> String {
        let dc_gain = self
            .dc_gain
            .map(|gain| format!("{gain:.3e}"))
            .unwrap_or_else(|| "None".to_owned());
        format!(
            "PoleZeroResult(poles={}, zeros={}, dc_gain={}, stable={}, pole_evidence='{}', zero_evidence='{}')",
            self.poles.len(),
            self.zeros.len(),
            dc_gain,
            self.is_stable()
                .map(|stable| stable.to_string())
                .unwrap_or_else(|| "None".to_string()),
            self.pole_evidence.kind,
            self.zero_evidence.kind,
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[pyo3(signature = (poles, zeros, gains, ports, evidence=None, gain_unit=None))]
    fn _unpickle(
        poles: Vec<PyComplexValue>,
        zeros: Vec<PyComplexValue>,
        gains: (Option<f64>, Option<f64>),
        ports: (String, String),
        evidence: Option<(RootSetEvidenceState, RootSetEvidenceState)>,
        gain_unit: Option<String>,
    ) -> PyResult<Self> {
        if gain_unit
            .as_deref()
            .is_some_and(|unit| !matches!(unit, "1" | "ohm" | "unspecified"))
        {
            return Err(crate::errors::value_error("invalid pole-zero gain unit"));
        }
        let (dc_gain, hf_gain) = gains;
        let (input, output) = ports;
        let (pole_evidence, zero_evidence) = if let Some((poles, zeros)) = evidence {
            (
                root_set_evidence_from_state(poles)?,
                root_set_evidence_from_state(zeros)?,
            )
        } else {
            (
                rspice_core::analysis::RootSetEvidence::LegacyUnknown,
                rspice_core::analysis::RootSetEvidence::LegacyUnknown,
            )
        };
        let core_poles = poles
            .iter()
            .map(|pole| rspice_core::Complex64::new(pole.real, pole.imag))
            .collect::<Vec<_>>();
        let core_zeros = zeros
            .iter()
            .map(|zero| rspice_core::Complex64::new(zero.real, zero.imag))
            .collect::<Vec<_>>();
        if !pole_evidence.is_consistent_with(&core_poles)
            || !zero_evidence.is_consistent_with(&core_zeros)
        {
            return Err(crate::errors::value_error(
                "root-set evidence is inconsistent with the pickled pole-zero vectors".to_string(),
            ));
        }
        Ok(Self {
            poles,
            zeros,
            pole_evidence: PyRootSetEvidence::from_core(&pole_evidence)?,
            zero_evidence: PyRootSetEvidence::from_core(&zero_evidence)?,
            dc_gain,
            hf_gain,
            gain_unit,
            input,
            output,
            evidence: None,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<PyComplexValue>,
            Vec<PyComplexValue>,
            (Option<f64>, Option<f64>),
            (String, String),
            (RootSetEvidenceState, RootSetEvidenceState),
            Option<String>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.poles.clone(),
                self.zeros.clone(),
                (self.dc_gain, self.hf_gain),
                (self.input.clone(), self.output.clone()),
                (self.pole_evidence.to_state(), self.zero_evidence.to_state()),
                self.gain_unit.clone(),
            ),
        ))
    }
}

type StbMarginState = (u8, Option<(f64, f64)>, Option<(f64, f64)>);

/// Loop-gain sweep and measured margins from Tian double-injection STB.
#[pyclass(name = "StbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyStbResult {
    frequencies: Vec<f64>,
    loop_gains: Vec<rspice_core::Complex64>,
    /// Pickle restoration retains the core's phase projection without
    /// inventing authored document evidence. Direct results use `evidence`.
    restored_bode_points: Vec<rspice_core::analysis::stb::BodePoint>,
    #[pyo3(get)]
    pub probe_name: String,
    #[pyo3(get)]
    pub gain_margin_db: Option<f64>,
    #[pyo3(get)]
    pub gain_margin_frequency: Option<f64>,
    #[pyo3(get)]
    pub phase_margin_degrees: Option<f64>,
    #[pyo3(get)]
    pub phase_margin_frequency: Option<f64>,
    dc_loop_gain: Option<rspice_core::Complex64>,
    #[pyo3(get)]
    pub unity_gain_bandwidth: Option<f64>,
    #[pyo3(get)]
    pub multiple_crossovers: bool,
    #[pyo3(get)]
    pub num_crossovers: usize,
    #[pyo3(get)]
    pub success: bool,
    #[pyo3(get)]
    pub warnings: Vec<String>,
    assessment: String,
    /// The core loop analysis, retained for typed scalar availability,
    /// signal descriptors, units and authored execution identity.
    evidence: Option<DocumentEvidence<rspice_core::analysis::stb::StbResult>>,
}

impl CarriesDocumentEvidence for PyStbResult {
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

impl PyStbResult {
    fn bode_points(&self) -> &[rspice_core::analysis::stb::BodePoint] {
        self.evidence
            .as_ref()
            .map_or(&self.restored_bode_points, |e| &e.core.bode_points)
    }

    fn bode_values<'py>(
        &self,
        py: Python<'py>,
        field: impl Fn(&rspice_core::analysis::stb::BodePoint) -> Option<f64>,
    ) -> Bound<'py, PyArray1<f64>> {
        // NumPy uses NaN at the boundary; the paired validity array and the
        // retained core Option preserve the actual availability contract.
        self.bode_points()
            .iter()
            .map(|point| field(point).unwrap_or(f64::NAN))
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    fn bode_validity<'py>(
        &self,
        py: Python<'py>,
        field: impl Fn(&rspice_core::analysis::stb::BodePoint) -> Option<f64>,
    ) -> Bound<'py, PyArray1<bool>> {
        self.bode_points()
            .iter()
            .map(|point| field(point).is_some())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// The shared result document, projected from the retained loop gain.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "stability")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_stability(analysis, result)
        })
    }

    pub fn from_core(result: &rspice_core::engine::StbAnalysisResult) -> Self {
        let margins = &result.result.margins;
        Self {
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Stb,
                result.result.clone(),
            )),
            frequencies: result.frequencies.clone(),
            loop_gains: result.loop_gains.clone(),
            restored_bode_points: Vec::new(),
            probe_name: result.probe_name.clone(),
            gain_margin_db: margins.gain_margin.map(|m| m.value),
            gain_margin_frequency: margins.gain_margin.map(|m| m.frequency),
            phase_margin_degrees: margins.phase_margin.map(|m| m.value),
            phase_margin_frequency: margins.phase_margin.map(|m| m.frequency),
            dc_loop_gain: margins.dc_loop_gain,
            unity_gain_bandwidth: margins.unity_gain_bandwidth(),
            multiple_crossovers: margins.num_crossovers > 1,
            num_crossovers: margins.num_crossovers,
            success: result.result.success,
            warnings: result.result.warnings.clone(),
            assessment: result.result.margin_assessment().into(),
        }
    }
}

#[pymethods]
impl PyStbResult {
    /// Independently measured zero-frequency return ratio, or None.
    #[getter]
    fn dc_loop_gain(&self) -> Option<PyComplexValue> {
        self.dc_loop_gain.as_ref().map(PyComplexValue::from_core)
    }

    /// DC magnitude in dB; None if unmeasured, -inf for a measured zero.
    #[getter]
    fn dc_gain_db(&self) -> Option<f64> {
        rspice_core::analysis::stb::StabilityMargins {
            dc_loop_gain: self.dc_loop_gain,
            ..Default::default()
        }
        .dc_gain_db()
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
    ///
    /// A crossover not resolved in the sampled band leaves both its margin
    /// and frequency unavailable, with the typed reason `no_crossover`.
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
    fn loop_gain<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.loop_gains.to_pyarray(py)
    }

    #[getter]
    fn magnitude<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.bode_values(py, |point| point.magnitude)
    }

    #[getter]
    fn magnitude_db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.bode_values(py, |point| point.magnitude_db)
    }

    /// Continuous Bode phase in degrees, using the core's unwrap convention.
    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.bode_values(py, |point| point.phase_deg)
    }

    #[getter]
    fn magnitude_validity<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<bool>> {
        self.bode_validity(py, |point| point.magnitude)
    }

    #[getter]
    fn magnitude_db_validity<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<bool>> {
        self.bode_validity(py, |point| point.magnitude_db)
    }

    #[getter]
    fn phase_validity<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<bool>> {
        self.bode_validity(py, |point| point.phase_deg)
    }

    #[getter]
    fn assessment(&self) -> String {
        self.assessment.clone()
    }

    fn __repr__(&self) -> String {
        let margin = |value: Option<f64>| value.map_or_else(|| "None".into(), |v| v.to_string());
        format!(
            "StbResult(probe='{}', points={}, gain_margin_db={}, phase_margin_degrees={}, assessment='{}')",
            self.probe_name,
            self.frequencies.len(),
            margin(self.gain_margin_db),
            margin(self.phase_margin_degrees),
            self.assessment
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (frequencies, loop_gains, probe_name, margins, flags, warnings, assessment, dc_state=None, margin_state=None))]
    fn _unpickle(
        frequencies: Vec<f64>,
        loop_gains: Vec<(f64, f64)>,
        probe_name: String,
        margins: [f64; 6],
        flags: (bool, usize, bool),
        mut warnings: Vec<String>,
        assessment: String,
        dc_state: Option<(u8, Option<(f64, f64)>)>,
        margin_state: Option<StbMarginState>,
    ) -> PyResult<Self> {
        let dc_loop_gain = match dc_state {
            Some((1, value)) => value.map(|(re, im)| rspice_core::Complex64::new(re, im)),
            Some((version, _)) => {
                return Err(crate::errors::value_error(format!(
                    "unsupported STB DC pickle version {version}"
                )));
            }
            None => {
                warnings
                    .push("Legacy STB pickle did not record an independent DC measurement".into());
                None
            }
        };
        if dc_loop_gain.is_some_and(|gain| !gain.re.is_finite() || !gain.im.is_finite()) {
            return Err(crate::errors::value_error(
                "STB DC return ratio must be finite",
            ));
        }
        use rspice_core::analysis::stb::{CrossoverMargin, StbAnalyzer, StbConfig};
        let gains = complex_from_state(loop_gains);
        let projected = StbAnalyzer::new(StbConfig::default())
            .analyze(&frequencies, &gains)
            .map_err(|error| {
                crate::errors::value_error(format!("invalid STB pickle samples: {error}"))
            })?;
        let restored = match margin_state {
            Some((1, gain, phase)) => {
                let decode =
                    |pair: Option<(f64, f64)>| -> PyResult<Option<CrossoverMargin>> {
                        pair.map(|(value, frequency)| {
                        if !value.is_finite() || !frequency.is_finite() || frequency <= 0.0 {
                            return Err(crate::errors::value_error(
                                "STB pickle margin requires a finite value and positive frequency",
                            ));
                        }
                        if !frequencies.first().zip(frequencies.last()).is_some_and(
                            |(&start, &stop)| (start..=stop).contains(&frequency),
                        ) {
                            return Err(crate::errors::value_error(
                                "STB pickle crossover frequency is outside the sampled band",
                            ));
                        }
                        Ok(CrossoverMargin { value, frequency })
                    })
                    .transpose()
                    };
                let gain_margin = decode(gain)?;
                let phase_margin = decode(phase)?;
                if phase_margin.is_some() != projected.margins.phase_margin.is_some()
                    || gain_margin.is_some() != projected.margins.gain_margin.is_some()
                    || flags.1 != projected.margins.num_crossovers
                    || flags.0 != (flags.1 > 1)
                {
                    return Err(crate::errors::value_error(
                        "STB pickle crossover observations disagree with margin availability",
                    ));
                }
                rspice_core::analysis::stb::StabilityMargins {
                    gain_margin,
                    phase_margin,
                    dc_loop_gain,
                    num_crossovers: flags.1,
                }
            }
            Some((version, _, _)) => {
                return Err(crate::errors::value_error(format!(
                    "unsupported STB margin pickle version {version}"
                )));
            }
            None => {
                warnings.push("Legacy STB margins were recomputed from retained loop samples; old stability claims were not retained".into());
                projected.margins
            }
        };
        let _ = (margins, assessment); // Legacy scalar placeholders and inferred verdict.
        let success = flags.2 && projected.success;
        let assessment = if success {
            restored.assessment()
        } else {
            "ANALYSIS FAILED"
        }
        .to_owned();
        Ok(Self {
            frequencies,
            loop_gains: gains,
            restored_bode_points: projected.bode_points,
            probe_name,
            gain_margin_db: restored.gain_margin.map(|m| m.value),
            gain_margin_frequency: restored.gain_margin.map(|m| m.frequency),
            phase_margin_degrees: restored.phase_margin.map(|m| m.value),
            phase_margin_frequency: restored.phase_margin.map(|m| m.frequency),
            dc_loop_gain,
            unity_gain_bandwidth: restored.unity_gain_bandwidth(),
            multiple_crossovers: restored.num_crossovers > 1,
            num_crossovers: restored.num_crossovers,
            success,
            warnings,
            assessment,
            evidence: None,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<f64>,
            Vec<(f64, f64)>,
            String,
            [f64; 6],
            (bool, usize, bool),
            Vec<String>,
            String,
            Option<(u8, Option<(f64, f64)>)>,
            Option<StbMarginState>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequencies.clone(),
                complex_state(&self.loop_gains),
                self.probe_name.clone(),
                [
                    self.gain_margin_db.unwrap_or(0.0),
                    self.gain_margin_frequency.unwrap_or(0.0),
                    self.phase_margin_degrees.unwrap_or(0.0),
                    self.phase_margin_frequency.unwrap_or(0.0),
                    0.0, // Legacy DC slot; only dc_state carries qualified evidence.
                    self.unity_gain_bandwidth.unwrap_or(0.0),
                ],
                (self.multiple_crossovers, self.num_crossovers, self.success),
                self.warnings.clone(),
                self.assessment.clone(),
                Some((1, self.dc_loop_gain.map(|gain| (gain.re, gain.im)))),
                Some((
                    1,
                    self.gain_margin_db.zip(self.gain_margin_frequency),
                    self.phase_margin_degrees.zip(self.phase_margin_frequency),
                )),
            ),
        ))
    }
}

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
