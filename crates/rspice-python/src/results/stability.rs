//! Loop stability, pole-zero, and transfer-function results.
//!
//! `StbResult` carries the Tian loop-gain probe (`.STB`), which measures a
//! feedback loop without breaking it. `PoleZeroResult` carries the roots of the
//! small-signal network, and `TransferFunctionResult` the DC transfer gain with
//! its input and output resistances. All three answer the same question from
//! different directions: whether, and how, a circuit is stable.

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
    /// Input specification
    #[pyo3(get)]
    pub input: String,
    /// Output specification
    #[pyo3(get)]
    pub output: String,
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
            input: result.input.clone(),
            output: result.output.clone(),
        })
    }
}

#[pymethods]
impl PyPoleZeroResult {
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
    #[pyo3(signature = (poles, zeros, gains, ports, evidence=None))]
    fn _unpickle(
        poles: Vec<PyComplexValue>,
        zeros: Vec<PyComplexValue>,
        gains: (Option<f64>, Option<f64>),
        ports: (String, String),
        evidence: Option<(RootSetEvidenceState, RootSetEvidenceState)>,
    ) -> PyResult<Self> {
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
            input,
            output,
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
            ),
        ))
    }
}

/// Loop-gain sweep and stability margins from Tian double-injection STB.
#[pyclass(name = "StbResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyStbResult {
    frequencies: Vec<f64>,
    loop_gains: Vec<rspice_core::Complex64>,
    #[pyo3(get)]
    pub probe_name: String,
    #[pyo3(get)]
    pub gain_margin_db: f64,
    #[pyo3(get)]
    pub gain_margin_frequency: f64,
    #[pyo3(get)]
    pub phase_margin_degrees: f64,
    #[pyo3(get)]
    pub phase_margin_frequency: f64,
    #[pyo3(get)]
    pub dc_gain_db: f64,
    #[pyo3(get)]
    pub unity_gain_bandwidth: f64,
    #[pyo3(get)]
    pub conditionally_stable: bool,
    #[pyo3(get)]
    pub num_crossovers: usize,
    #[pyo3(get)]
    pub success: bool,
    #[pyo3(get)]
    pub warnings: Vec<String>,
    assessment: String,
}

impl PyStbResult {
    pub fn from_core(result: &rspice_core::engine::StbAnalysisResult) -> Self {
        let margins = &result.result.margins;
        Self {
            frequencies: result.frequencies.clone(),
            loop_gains: result.loop_gains.clone(),
            probe_name: result.probe_name.clone(),
            gain_margin_db: margins.gain_margin_db,
            gain_margin_frequency: margins.gain_margin_freq,
            phase_margin_degrees: margins.phase_margin_deg,
            phase_margin_frequency: margins.phase_margin_freq,
            dc_gain_db: margins.dc_gain_db,
            unity_gain_bandwidth: margins.unity_gain_bandwidth,
            conditionally_stable: margins.conditionally_stable,
            num_crossovers: margins.num_crossovers,
            success: result.result.success,
            warnings: result.result.warnings.clone(),
            assessment: result.result.assessment(),
        }
    }
}

#[pymethods]
impl PyStbResult {
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
        self.loop_gains
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn magnitude_db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.loop_gains
            .iter()
            .map(|value| 20.0 * value.norm().log10())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.loop_gains
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn is_stable(&self) -> bool {
        self.gain_margin_db > 0.0 && self.phase_margin_degrees > 0.0
    }

    #[getter]
    fn assessment(&self) -> String {
        self.assessment.clone()
    }

    fn __repr__(&self) -> String {
        format!(
            "StbResult(probe='{}', points={}, gain_margin={:.2}dB, phase_margin={:.2}deg, assessment='{}')",
            self.probe_name,
            self.frequencies.len(),
            self.gain_margin_db,
            self.phase_margin_degrees,
            self.assessment
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        frequencies: Vec<f64>,
        loop_gains: Vec<(f64, f64)>,
        probe_name: String,
        margins: [f64; 6],
        flags: (bool, usize, bool),
        warnings: Vec<String>,
        assessment: String,
    ) -> Self {
        let (conditionally_stable, num_crossovers, success) = flags;
        Self {
            frequencies,
            loop_gains: complex_from_state(loop_gains),
            probe_name,
            gain_margin_db: margins[0],
            gain_margin_frequency: margins[1],
            phase_margin_degrees: margins[2],
            phase_margin_frequency: margins[3],
            dc_gain_db: margins[4],
            unity_gain_bandwidth: margins[5],
            conditionally_stable,
            num_crossovers,
            success,
            warnings,
            assessment,
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
            Vec<(f64, f64)>,
            String,
            [f64; 6],
            (bool, usize, bool),
            Vec<String>,
            String,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequencies.clone(),
                complex_state(&self.loop_gains),
                self.probe_name.clone(),
                [
                    self.gain_margin_db,
                    self.gain_margin_frequency,
                    self.phase_margin_degrees,
                    self.phase_margin_frequency,
                    self.dc_gain_db,
                    self.unity_gain_bandwidth,
                ],
                (self.conditionally_stable, self.num_crossovers, self.success),
                self.warnings.clone(),
                self.assessment.clone(),
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
}

impl PyTransferFunctionResult {
    pub fn from_core(result: &rspice_core::analysis::TransferFunctionResult) -> Self {
        Self {
            output: result.output.clone(),
            input: result.input.clone(),
            gain: result.gain,
            input_impedance: result.input_impedance,
            output_impedance: result.output_impedance,
        }
    }
}

#[pymethods]
impl PyTransferFunctionResult {
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
