//! Value types the engine accepts and returns.
//!
//! `DcSweep` is the general `.DC` specification, covering linear, explicit
//! list, and logarithmic axes plus nested two-source sweeps. The PSS and HB
//! state objects carry a solved periodic operating point between calls, so a
//! long continuation run can be resumed instead of restarted. `HealthReport`
//! is the readiness probe's output.

use super::*;

/// One `.DC` sweep axis
///
/// Describes a single swept source in any of the forms `.DC` accepts:
/// linear, an explicit value list, or a logarithmic decade/octave sweep.
/// Pass one to `Engine.run_dc_sweep_spec` as the inner axis, and optionally
/// a second as the outer axis of a nested sweep.
///
/// Example:
///     >>> DcSweep("V1", start=0, stop=5, step=0.1)
///     >>> DcSweep("V1", values=[0, 1.8, 3.3, 5.0])
///     >>> DcSweep("V1", start=1, stop=1e6, mode="dec", points=10)
#[pyclass(name = "DcSweep", module = "rspice", frozen, from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub struct PyDcSweep {
    /// Name of the swept source or parameter.
    #[pyo3(get)]
    pub source: String,
    pub(super) spec: DcSweepSpec,
}

#[pymethods]
impl PyDcSweep {
    /// Describe one swept axis
    ///
    /// Args:
    ///     source: Source name to sweep (e.g. "V1")
    ///     start: First value; required for every mode except "list"
    ///     stop: Last value; required for every mode except "list"
    ///     step: Increment; required and non-zero for "linear"
    ///     mode: "linear", "list", "dec", or "oct". Inferred from the other
    ///           arguments when omitted: "list" if `values` is given,
    ///           otherwise "linear".
    ///     values: Explicit values; implies and is exclusive to "list"
    ///     points: Points per decade/octave; required for "dec" and "oct"
    ///
    /// Raises:
    ///     ValueError: If the arguments do not describe a usable sweep
    #[new]
    #[pyo3(signature = (source, start=None, stop=None, step=None, *, mode=None, values=None, points=None))]
    fn new(
        source: &str,
        start: Option<f64>,
        stop: Option<f64>,
        step: Option<f64>,
        mode: Option<&str>,
        values: Option<Vec<f64>>,
        points: Option<usize>,
    ) -> PyResult<Self> {
        if source.trim().is_empty() {
            return Err(crate::errors::value_error("source must not be empty"));
        }
        // A value list is self-describing, so `mode` is only needed to pick
        // between linear and the logarithmic axes.
        let normalized = match mode {
            Some(mode) => mode.to_ascii_lowercase(),
            None if values.is_some() => "list".to_string(),
            None => "linear".to_string(),
        };

        // Reject contradictory arguments rather than silently dropping one:
        // an ignored bound or value list is a wrong sweep, not a warning.
        if values.is_some() {
            if normalized != "list" {
                return Err(crate::errors::value_error(format!(
                    "values describes a list sweep and cannot be combined with mode='{normalized}'"
                )));
            }
            if start.is_some() || stop.is_some() || step.is_some() {
                return Err(crate::errors::value_error(
                    "values cannot be combined with start, stop, or step",
                ));
            }
        }
        if points.is_some() && !matches!(normalized.as_str(), "dec" | "decade" | "oct" | "octave") {
            return Err(crate::errors::value_error(
                "points is only valid with mode='dec' or mode='oct'",
            ));
        }

        let spec = match normalized.as_str() {
            "linear" | "lin" => {
                let (start, stop, step) = require_linear_bounds(start, stop, step)?;
                DcSweepSpec {
                    start,
                    stop,
                    step,
                    mode: DcSweepMode::Linear,
                }
            }
            "list" => {
                let values = values
                    .ok_or_else(|| crate::errors::value_error("mode='list' requires values"))?;
                if values.is_empty() {
                    return Err(crate::errors::value_error("values must not be empty"));
                }
                if let Some((index, value)) = values
                    .iter()
                    .enumerate()
                    .find(|(_, value)| !value.is_finite())
                {
                    return Err(crate::errors::value_error(format!(
                        "sweep value at index {index} must be finite, got {value}"
                    )));
                }
                DcSweepSpec {
                    start: values[0],
                    stop: *values.last().expect("values is non-empty"),
                    step: 0.0,
                    mode: DcSweepMode::List(values),
                }
            }
            "dec" | "decade" | "oct" | "octave" => {
                let (start, stop) = require_log_bounds(start, stop)?;
                let points = points.ok_or_else(|| {
                    crate::errors::value_error("mode='dec' and mode='oct' require points")
                })?;
                if points == 0 {
                    return Err(crate::errors::value_error("points must be at least 1"));
                }
                let mode = if normalized.starts_with("dec") {
                    DcSweepMode::Decade {
                        points_per_decade: points,
                    }
                } else {
                    DcSweepMode::Octave {
                        points_per_octave: points,
                    }
                };
                DcSweepSpec {
                    start,
                    stop,
                    step: 0.0,
                    mode,
                }
            }
            other => {
                return Err(crate::errors::value_error(format!(
                    "mode must be 'linear', 'list', 'dec', or 'oct', got '{other}'"
                )));
            }
        };

        if spec.points().is_empty() {
            return Err(crate::errors::value_error(format!(
                "sweep of '{source}' produces no points"
            )));
        }
        Ok(Self {
            source: source.to_string(),
            spec,
        })
    }

    /// The values this axis will visit.
    #[getter]
    fn values<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.spec.points().to_pyarray(py)
    }

    /// Number of points on this axis.
    #[getter]
    fn num_points(&self) -> usize {
        self.spec.points().len()
    }

    /// Sweep mode: "linear", "list", "dec", or "oct".
    #[getter]
    fn mode(&self) -> &'static str {
        match self.spec.mode {
            DcSweepMode::Linear => "linear",
            DcSweepMode::List(_) => "list",
            DcSweepMode::Decade { .. } => "dec",
            DcSweepMode::Octave { .. } => "oct",
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "DcSweep(source='{}', mode='{}', points={})",
            self.source,
            self.mode(),
            self.num_points()
        )
    }
}

/// A converged periodic operating point, reusable by PAC and PNoise.
///
/// Small-signal periodic analyses linearize around a PSS solution. Solving
/// that shooting problem once and passing this object to `run_pac` and
/// `run_pnoise` replaces one full PSS solve per call, which dominates the
/// cost of an RF sweep.
///
/// Opaque by design: it carries the exact shooting state and configuration
/// that produced it, and reconstructing either from Python values would let a
/// caller silently pair an operating point with a different circuit.
#[pyclass(name = "PssOperatingPoint", module = "rspice", frozen)]
pub struct PyPssOperatingPoint {
    pub(super) inner: rspice_core::engine::PssOperatingPoint,
}

#[pymethods]
impl PyPssOperatingPoint {
    /// Converged fundamental period in seconds.
    #[getter]
    fn period(&self) -> f64 {
        self.inner.analysis().result.period
    }

    /// Converged fundamental frequency in Hz.
    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.analysis().result.frequency
    }

    /// Shooting iterations the solve required.
    #[getter]
    fn iterations(&self) -> usize {
        self.inner.analysis().result.iterations
    }

    /// Whether a periodic orbit was detected.
    #[getter]
    fn period_detected(&self) -> bool {
        self.inner.analysis().result.period_detected
    }

    fn __repr__(&self) -> String {
        format!(
            "PssOperatingPoint(frequency={:.6e} Hz, iterations={})",
            self.frequency(),
            self.iterations()
        )
    }
}

/// Phase-equivalent PSS state a transient can continue from.
///
/// Starting a transient from a converged orbit skips the settling interval
/// that a cold start has to integrate through, which is what makes long
/// post-PSS envelope and modulation runs affordable.
#[pyclass(name = "PssContinuationState", module = "rspice", frozen)]
pub struct PyPssContinuationState {
    pub(super) inner: rspice_core::engine::PssContinuationState,
}

#[pymethods]
impl PyPssContinuationState {
    /// Converged fundamental period in seconds.
    #[getter]
    fn period(&self) -> f64 {
        self.inner.period()
    }

    /// Absolute simulation time this phase-equivalent state represents.
    #[getter]
    fn time_origin(&self) -> f64 {
        self.inner.time_origin()
    }

    fn __repr__(&self) -> String {
        format!(
            "PssContinuationState(period={:.6e} s, time_origin={:.6e} s)",
            self.period(),
            self.time_origin()
        )
    }
}

/// Harmonic-balance envelope state a transient can continue from.
///
/// Carries the harmonic-balance configuration and frozen-source list that
/// produced it, so continuing cannot be paired with a different HB setup by
/// accident; core validates the pairing and rejects a mismatch.
#[pyclass(name = "HbEnvelopeState", module = "rspice", frozen)]
pub struct PyHbEnvelopeState {
    pub(super) inner: rspice_core::engine::HbEnvelopeContinuationState,
    pub(super) config: HbConfig,
    pub(super) frozen_sources: Vec<String>,
}

#[pymethods]
impl PyHbEnvelopeState {
    /// Independent sources frozen at their time-zero values for the solve.
    #[getter]
    fn frozen_sources(&self) -> Vec<String> {
        self.frozen_sources.clone()
    }

    /// Fundamental frequency of the harmonic-balance solution, in Hz.
    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.config.fundamental_freq
    }

    fn __repr__(&self) -> String {
        format!(
            "HbEnvelopeState(fundamental={:.6e} Hz, frozen_sources={:?})",
            self.fundamental_frequency(),
            self.frozen_sources
        )
    }
}

/// Outcome of `Engine.health_check()`
///
/// Reports the wall-clock duration of the probe and the size of the circuit
/// it built. The report is only produced when the probe succeeded, so
/// `status` is always `"ready"` and `ready` is always `True`; a failed probe
/// raises instead.
#[pyclass(name = "HealthReport", module = "rspice", frozen, from_py_object)]
#[derive(Debug, Clone, PartialEq)]
pub struct PyHealthReport {
    #[pyo3(get)]
    pub status: &'static str,
    #[pyo3(get)]
    pub ready: bool,
    #[pyo3(get)]
    pub duration_seconds: f64,
    #[pyo3(get)]
    pub element_count: usize,
    #[pyo3(get)]
    pub node_count: usize,
    #[pyo3(get)]
    pub branch_count: usize,
    #[pyo3(get)]
    pub output_voltage: f64,
}

impl From<rspice_core::EngineHealthReport> for PyHealthReport {
    fn from(report: rspice_core::EngineHealthReport) -> Self {
        Self {
            status: "ready",
            ready: true,
            duration_seconds: report.elapsed.as_secs_f64(),
            element_count: report.element_count,
            node_count: report.node_count,
            branch_count: report.branch_count,
            output_voltage: report.output_voltage,
        }
    }
}
