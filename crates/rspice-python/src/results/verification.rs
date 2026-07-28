//! Automated-verification results.
//!
//! `RunReport` is what `Engine.run()` returns: the analyses a deck actually
//! executed, the `.MEAS` statements evaluated against them, and the verdict
//! `assert_passed()` turns into a CI gate. A measurement whose analysis did not
//! run is recorded explicitly as not-evaluated rather than omitted, so a
//! skipped check fails loudly instead of silently passing.

use super::*;

/// Result of a single .MEAS statement
///
/// `passed` is true when the measurement evaluated to a value and, if the
/// statement declared `GOAL=` (optionally `TOL=`), the value landed within
/// tolerance. A failed measurement carries an `error` message.
///
/// Example:
///     >>> m = report.measurement("trise")
///     >>> assert m.passed and m.value < 1e-6
#[pyclass(name = "Measurement", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyMeasurement {
    /// Measurement name (from the .MEAS statement)
    #[pyo3(get)]
    pub name: String,
    /// Analysis the measurement applies to ("TRAN", "DC", "AC", "NOISE")
    #[pyo3(get)]
    pub analysis: String,
    /// Measured value, or None if evaluation failed
    #[pyo3(get)]
    pub value: Option<f64>,
    /// Failure description when evaluation failed
    #[pyo3(get)]
    pub error: Option<String>,
    /// Declared GOAL, when the statement carried one
    #[pyo3(get)]
    pub expected: Option<f64>,
    /// Effective tolerance applied to the GOAL check
    #[pyo3(get)]
    pub tolerance: Option<f64>,
    pub(crate) ok: bool,
}

impl PyMeasurement {
    pub fn from_core(result: &rspice_core::MeasureResult, analysis: &str) -> Self {
        Self {
            name: result.name.clone(),
            analysis: analysis.to_string(),
            value: result.value,
            error: result.error.clone(),
            expected: result.expected,
            tolerance: result.tolerance,
            ok: result.passed,
        }
    }

    pub fn unevaluated(name: &str, analysis: &str, reason: &str) -> Self {
        Self {
            name: name.to_string(),
            analysis: analysis.to_string(),
            value: None,
            error: Some(reason.to_string()),
            expected: None,
            tolerance: None,
            ok: false,
        }
    }

    fn failure_message(&self) -> String {
        if let Some(error) = &self.error {
            return error.clone();
        }
        match (self.value, self.expected, self.tolerance) {
            (Some(value), Some(expected), Some(tolerance)) => {
                format!("value {value:.6e} is outside goal {expected:.6e} +/- {tolerance:.6e}")
            }
            (Some(value), Some(expected), None) => {
                format!("value {value:.6e} did not meet goal {expected:.6e}")
            }
            (Some(value), None, _) => format!("measurement failed with value {value:.6e}"),
            (None, _, _) => "evaluation failed".to_string(),
        }
    }
}

#[pymethods]
impl PyMeasurement {
    /// True when the measurement produced a value within any declared GOAL
    #[getter]
    fn passed(&self) -> bool {
        self.ok
    }

    /// Convert to float; raises ValueError when the measurement failed
    fn __float__(&self) -> PyResult<f64> {
        match (self.ok, self.value) {
            (true, Some(value)) => Ok(value),
            _ => Err(crate::errors::value_error(format!(
                "measurement '{}' failed: {}",
                self.name,
                self.failure_message()
            ))),
        }
    }

    fn __repr__(&self) -> String {
        if self.ok {
            match self.value {
                Some(v) => format!("Measurement({}={:.6e} [{}])", self.name, v, self.analysis),
                None => format!(
                    "Measurement({} FAILED [{}]: {})",
                    self.name,
                    self.analysis,
                    self.failure_message()
                ),
            }
        } else {
            format!(
                "Measurement({} FAILED [{}]: {})",
                self.name,
                self.analysis,
                self.failure_message()
            )
        }
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        name: String,
        analysis: String,
        value: Option<f64>,
        error: Option<String>,
        goal: (Option<f64>, Option<f64>),
        ok: bool,
    ) -> Self {
        let (expected, tolerance) = goal;
        Self {
            name,
            analysis,
            value,
            error,
            expected,
            tolerance,
            ok,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            String,
            String,
            Option<f64>,
            Option<String>,
            (Option<f64>, Option<f64>),
            bool,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.name.clone(),
                self.analysis.clone(),
                self.value,
                self.error.clone(),
                (self.expected, self.tolerance),
                self.ok,
            ),
        ))
    }
}

/// Record of one analysis directive handled by `Engine.run`
#[pyclass(name = "AnalysisRecord", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyAnalysisRecord {
    /// Analysis kind: "op", "dc", "ac", "tran", "noise", "tf", "four", ...
    #[pyo3(get)]
    pub kind: String,
    /// Human-readable summary of the directive
    #[pyo3(get)]
    pub detail: String,
    /// True when the directive was not executed
    #[pyo3(get)]
    pub skipped: bool,
    /// Why the directive was skipped (when skipped)
    #[pyo3(get)]
    pub reason: Option<String>,
}

impl PyAnalysisRecord {
    pub fn executed(kind: &str, detail: String) -> Self {
        Self {
            kind: kind.to_string(),
            detail,
            skipped: false,
            reason: None,
        }
    }

    pub fn skipped(kind: &str, detail: String, reason: &str) -> Self {
        Self {
            kind: kind.to_string(),
            detail,
            skipped: true,
            reason: Some(reason.to_string()),
        }
    }
}

#[pymethods]
impl PyAnalysisRecord {
    fn __repr__(&self) -> String {
        if self.skipped {
            format!(
                "AnalysisRecord({} SKIPPED: {})",
                self.detail,
                self.reason.as_deref().unwrap_or("")
            )
        } else {
            format!("AnalysisRecord({})", self.detail)
        }
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(kind: String, detail: String, skipped: bool, reason: Option<String>) -> Self {
        Self {
            kind,
            detail,
            skipped,
            reason,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (String, String, bool, Option<String>))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.kind.clone(),
                self.detail.clone(),
                self.skipped,
                self.reason.clone(),
            ),
        ))
    }
}

/// Aggregated outcome of `Engine.run`: every analysis the netlist requested
/// plus all .MEAS verification results.
///
/// Designed for CI: `report.assert_passed()` raises `MeasurementError` if
/// any directive was skipped, any measurement failed (or none were evaluated),
/// with a message listing each failure.
///
/// Example:
///     >>> report = engine.run(netlist)
///     >>> report.assert_passed()
///     >>> tpd = report.measurement("tpd").value
#[pyclass(name = "RunReport", module = "rspice")]
pub struct PyRunReport {
    /// DC operating point result (last .op)
    #[pyo3(get)]
    pub op: Option<Py<PySimulationResult>>,
    /// DC sweep result (last .dc)
    #[pyo3(get)]
    pub dc: Option<Py<PyDcSweepResult>>,
    /// Transient result (last .tran)
    #[pyo3(get)]
    pub tran: Option<Py<PyTransientResult>>,
    /// AC result (last .ac)
    #[pyo3(get)]
    pub ac: Option<Py<PyAcResult>>,
    /// Third-order Volterra distortion result (last .disto)
    #[pyo3(get)]
    pub distortion: Option<Py<PyDistortionResult>>,
    /// Harmonic-balance result (last .hb)
    #[pyo3(get)]
    pub hb: Option<PyHbResult>,
    /// N-port scattering parameters (last .sp)
    #[pyo3(get)]
    pub s_parameters: Option<PySParameterResult>,
    /// Noise results (last .noise)
    #[pyo3(get)]
    pub noise: Option<Vec<PyNoiseResult>>,
    /// Transfer function result (last .tf)
    #[pyo3(get)]
    pub tf: Option<PyTransferFunctionResult>,
    /// Loop-stability result (last .stb)
    #[pyo3(get)]
    pub stb: Option<PyStbResult>,
    /// Pole-zero result (last .pz)
    #[pyo3(get)]
    pub pz: Option<PyPoleZeroResult>,
    /// Monte Carlo result (last .mc)
    #[pyo3(get)]
    pub monte_carlo: Option<PyMonteCarloResult>,
    /// Parametric DC operating-point sweep (last .step)
    #[pyo3(get)]
    pub step: Option<PyDcSweepResult>,
    /// Temperature DC operating-point sweep (last .temp)
    #[pyo3(get)]
    pub temperature: Option<PyDcSweepResult>,
    /// Adjoint DC sensitivity result (last DC .sens)
    #[pyo3(get)]
    pub sensitivity: Option<PySensitivityResult>,
    /// Complete complex AC sensitivity result (last AC .sens)
    #[pyo3(get)]
    pub sensitivity_ac: Option<PyAcSensitivityResult>,
    /// Fourier results (one per .four output)
    #[pyo3(get)]
    pub fourier: Vec<PyFourierResult>,
    /// One or more records per analysis directive in the netlist. A `.four`
    /// contributes one record per output and an `.sp donoise` contributes a
    /// separate noise record.
    #[pyo3(get)]
    pub records: Vec<PyAnalysisRecord>,
    /// All measurement outcomes
    #[pyo3(get)]
    pub measurements: Vec<PyMeasurement>,
    /// Every `.op` result, in deck order. `op` is the last of these.
    #[pyo3(get)]
    pub all_op: Vec<Py<PySimulationResult>>,
    /// Every `.dc` result, in deck order. `dc` is the last of these.
    #[pyo3(get)]
    pub all_dc: Vec<Py<PyDcSweepResult>>,
    /// Every `.tran` result, in deck order. `tran` is the last of these.
    #[pyo3(get)]
    pub all_tran: Vec<Py<PyTransientResult>>,
    /// Every `.ac`/`.ac data` result, in deck order. `ac` is the last of these.
    #[pyo3(get)]
    pub all_ac: Vec<Py<PyAcResult>>,
    /// Every `.noise`/`.noise data` sweep, in deck order. `noise` is the last.
    #[pyo3(get)]
    pub all_noise: Vec<Vec<PyNoiseResult>>,
}

#[pymethods]
impl PyRunReport {
    /// Look up a measurement by name (case-insensitive)
    fn measurement(&self, name: &str) -> Option<PyMeasurement> {
        self.measurements
            .iter()
            .find(|m| m.name.eq_ignore_ascii_case(name))
            .cloned()
    }

    /// Number of measurements evaluated
    #[getter]
    fn num_measurements(&self) -> usize {
        self.measurements.len()
    }

    /// True when no analysis directive was skipped, at least one measurement
    /// was evaluated, and every measurement passed.
    #[getter]
    fn all_passed(&self) -> bool {
        !self.measurements.is_empty()
            && self.records.iter().all(|r| !r.skipped)
            && self.measurements.iter().all(|m| m.ok)
    }

    /// Measurements that failed to evaluate or failed their goal/tolerance check
    #[getter]
    fn failures(&self) -> Vec<PyMeasurement> {
        self.measurements
            .iter()
            .filter(|m| !m.ok)
            .cloned()
            .collect()
    }

    /// Kinds of analyses that actually executed (e.g. ["op", "tran"])
    #[getter]
    fn analyses_run(&self) -> Vec<String> {
        self.records
            .iter()
            .filter(|r| !r.skipped)
            .map(|r| r.kind.clone())
            .collect()
    }

    /// Records for directives that were skipped
    #[getter]
    fn skipped(&self) -> Vec<PyAnalysisRecord> {
        self.records.iter().filter(|r| r.skipped).cloned().collect()
    }

    /// Raise MeasurementError unless every requested analysis ran, at least
    /// one measurement was evaluated, and all of them passed.
    ///
    /// This is the CI primitive: a netlist whose .MEAS statements were
    /// silently skipped fails loudly instead of green-washing a pipeline.
    fn assert_passed(&self) -> PyResult<()> {
        let skipped = self.skipped();
        if !skipped.is_empty() {
            let mut message = format!(
                "{} of {} analysis directives were skipped:",
                skipped.len(),
                self.records.len()
            );
            for record in &skipped {
                message.push_str(&format!(
                    "\n  {}: {}",
                    record.detail,
                    record.reason.as_deref().unwrap_or("skipped")
                ));
            }
            return Err(crate::errors::MeasurementError::new_err(message));
        }
        if self.measurements.is_empty() {
            return Err(crate::errors::MeasurementError::new_err(
                "no measurements were evaluated: the netlist has no .MEAS statements \
                 covered by the analyses that ran",
            ));
        }
        let failures = self.failures();
        if failures.is_empty() {
            return Ok(());
        }
        let mut message = format!(
            "{} of {} measurements failed:",
            failures.len(),
            self.measurements.len()
        );
        for f in &failures {
            message.push_str(&format!(
                "\n  {} [{}]: {}",
                f.name,
                f.analysis,
                f.failure_message()
            ));
        }
        Err(crate::errors::MeasurementError::new_err(message))
    }

    fn __repr__(&self) -> String {
        let executed = self.records.iter().filter(|r| !r.skipped).count();
        let skipped = self.records.len() - executed;
        format!(
            "RunReport(analyses={}, skipped={}, measurements={}, all_passed={})",
            executed,
            skipped,
            self.measurements.len(),
            self.all_passed()
        )
    }
}
