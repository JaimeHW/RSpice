//! Pole-zero result projection and persistence.

use super::*;

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

    /// Exact asymptotic-stability evidence, independent of rounded root signs.
    /// Older results without that evidence return None.
    #[getter]
    fn is_stable(&self) -> Option<bool> {
        use rspice_core::analysis::pole_zero::{PoleSpectrum, StabilityVerdict};
        let spectrum = PoleSpectrum {
            poles: self
                .poles
                .iter()
                .map(|pole| rspice_core::Complex64::new(pole.real, pole.imag))
                .collect(),
            evidence: root_set_evidence_from_state(self.pole_evidence.to_state()).ok()?,
        };
        match spectrum.stability_verdict() {
            StabilityVerdict::Stable => Some(true),
            StabilityVerdict::Unstable => Some(false),
            StabilityVerdict::Indeterminate => None,
        }
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
