//! Loop-gain result projection, stability evidence, and persistence.

use super::*;

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
    restored_circuit_poles: rspice_core::analysis::stb::CircuitPoleEvidence,
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
    fn circuit_pole_state(&self) -> &rspice_core::analysis::stb::CircuitPoleEvidence {
        self.evidence
            .as_ref()
            .map_or(&self.restored_circuit_poles, |e| &e.core.circuit_poles)
    }

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
            restored_circuit_poles: Default::default(),
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
    /// Complete finite circuit poles in rad/s, or None when unavailable.
    #[getter]
    fn circuit_poles<'py>(
        &self,
        py: Python<'py>,
    ) -> Option<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.circuit_pole_state()
            .spectrum()
            .map(|s| s.poles.to_pyarray(py))
    }

    #[getter]
    fn circuit_pole_evidence(&self) -> PyResult<Option<PyRootSetEvidence>> {
        self.circuit_pole_state()
            .spectrum()
            .map(|s| PyRootSetEvidence::from_core(&s.evidence))
            .transpose()
    }

    /// Only qualified complete circuit modes establish stability; margins do not.
    #[getter]
    fn is_stable(&self) -> Option<bool> {
        use rspice_core::analysis::pole_zero::StabilityVerdict;
        if !self.success {
            return None;
        }
        match self.circuit_pole_state().stability_verdict() {
            StabilityVerdict::Stable => Some(true),
            StabilityVerdict::Unstable => Some(false),
            StabilityVerdict::Indeterminate => None,
        }
    }

    #[getter]
    fn circuit_pole_status(&self) -> &'static str {
        use rspice_core::analysis::stb::CircuitPoleEvidence as E;
        match self.circuit_pole_state() {
            E::NotComputed => "not_computed",
            E::Available { .. } => "available",
            E::Unavailable { .. } => "unavailable",
        }
    }

    #[getter]
    fn circuit_pole_failure<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<Option<Bound<'py, pyo3::types::PyDict>>> {
        use rspice_core::analysis::stb::{CircuitPoleEvidence as E, CircuitPoleFailure as F};
        let E::Unavailable { cause } = self.circuit_pole_state() else {
            return Ok(None);
        };
        let result = pyo3::types::PyDict::new(py);
        match cause {
            F::Unsupported { capability, detail } => {
                result.set_item("kind", "unsupported")?;
                result.set_item("capability", capability)?;
                result.set_item("detail", detail)?;
            }
            F::Numerical { detail } => {
                result.set_item("kind", "numerical")?;
                result.set_item("detail", detail)?;
            }
            F::ResourceLimit {
                resource,
                requested,
                limit,
            } => {
                result.set_item("kind", "resource_limit")?;
                result.set_item("resource", resource)?;
                result.set_item("requested", requested)?;
                result.set_item("limit", limit)?;
            }
        }
        Ok(Some(result))
    }

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
    #[pyo3(signature = (frequencies, loop_gains, probe_name, margins, flags, warnings, assessment, dc_state=None, margin_state=None, circuit_state=None))]
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
        circuit_state: Option<(u8, String)>,
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
        let projected_retained_values = projected.retained_value_count();
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
        let restored_circuit_poles = match circuit_state {
            None => Default::default(),
            Some((1, text)) => {
                let limits = rspice_core::ResourceLimits::default();
                if text.len() > limits.max_external_data_bytes {
                    return Err(crate::errors::value_error(
                        "STB circuit-pole pickle exceeds byte limit",
                    ));
                }
                let evidence: rspice_core::analysis::stb::CircuitPoleEvidence =
                    serde_json::from_str(&text).map_err(|e| {
                        crate::errors::value_error(format!("invalid STB circuit-pole pickle: {e}"))
                    })?;
                evidence
                    .validate_with_abort(&limits, &rspice_core::NoAbort)
                    .map_err(|e| crate::errors::value_error(e.to_string()))?;
                evidence
            }
            Some((version, _)) => {
                return Err(crate::errors::value_error(format!(
                    "unsupported STB circuit-pole pickle version {version}"
                )));
            }
        };
        let limits = rspice_core::ResourceLimits::default();
        let retained_values = projected_retained_values
            .saturating_sub(4)
            .saturating_add(restored_circuit_poles.retained_value_count());
        let retained_bytes = warnings
            .iter()
            .fold(restored_circuit_poles.diagnostic_bytes(), |n, s| {
                n.saturating_add(s.len())
            });
        if retained_values > limits.max_result_values
            || retained_bytes > limits.max_external_data_bytes
        {
            return Err(crate::errors::value_error(
                "STB pickle exceeds retained-evidence limits",
            ));
        }
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
            restored_circuit_poles,
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
            Option<(u8, String)>,
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
                Some((
                    1,
                    serde_json::to_string(self.circuit_pole_state())
                        .map_err(|e| crate::errors::value_error(e.to_string()))?,
                )),
            ),
        ))
    }
}
