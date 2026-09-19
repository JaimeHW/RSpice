//! DC mismatch results (`.DCMATCH`).
//!
//! One output's DC spread at the nominal operating point, and the ranked
//! `(instance, variable)` pairs that own it. Every sigma here came from the
//! design's own `statistics` block: there is no default spread, so a result
//! exists only for a deck that declared one.
//!
//! The three sigmas are kept apart rather than summed into one number,
//! because a spread a per-instance mismatch variable owns and a spread the
//! whole design shares are fixed by different design decisions.

use super::*;

/// One statistical variable's contribution to an output's DC variance.
#[pyclass(name = "DcMatchContributor", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDcMatchContributor {
    /// Instance that drew the variable, or the design itself for a
    /// design-wide process variable.
    #[pyo3(get)]
    pub instance: String,
    /// Canonical (upper-case) statistical parameter name.
    #[pyo3(get)]
    pub parameter: String,
    /// Which statistical scope declared it: `"process"` or `"mismatch"`.
    #[pyo3(get)]
    pub scope: String,
    /// Standard deviation of the parameter itself.
    #[pyo3(get)]
    pub sigma_parameter: f64,
    /// `d(output)/d(parameter)` at the nominal operating point.
    #[pyo3(get)]
    pub sensitivity: f64,
    /// Signed output displacement one standard deviation of this variable
    /// produces, so a reader can tell which way the output moves.
    #[pyo3(get)]
    pub contribution: f64,
    /// This contributor's allocation of the total output variance. Signed: a
    /// variable whose correlated partner cancels it carries a negative share,
    /// and the shares still sum to one.
    #[pyo3(get)]
    pub share: f64,
}

impl PyDcMatchContributor {
    fn from_core(contributor: &rspice_core::analysis::dcmatch::DcMatchContributor) -> Self {
        Self {
            instance: contributor.instance.clone(),
            parameter: contributor.parameter.clone(),
            scope: contributor.scope.tag().to_owned(),
            sigma_parameter: contributor.sigma_parameter,
            sensitivity: contributor.sensitivity,
            contribution: contributor.contribution,
            share: contributor.share,
        }
    }
}

#[pymethods]
impl PyDcMatchContributor {
    fn __repr__(&self) -> String {
        format!(
            "DcMatchContributor(instance='{}', parameter='{}', scope='{}', share={:.6e}, contribution={:.6e})",
            self.instance, self.parameter, self.scope, self.share, self.contribution
        )
    }
}

/// DC mismatch variance of one output, with its ranked contributors.
///
/// Example:
///     >>> report = engine.run(netlist)
///     >>> print(f"sigma={report.dcmatch.sigma_total:.3e} V")
#[pyclass(name = "DcMatchResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyDcMatchResult {
    /// The probe as the deck spelled it, for example `V(out,in)` or `I(V1)`.
    #[pyo3(get)]
    pub output: String,
    /// The output with every statistical variable at its nominal value.
    #[pyo3(get)]
    pub nominal_value: f64,
    /// Total output standard deviation, before the card's multiplier.
    #[pyo3(get)]
    pub sigma_total: f64,
    /// The part of `sigma_total` the per-instance mismatch variables own.
    #[pyo3(get)]
    pub sigma_mismatch: f64,
    /// The part of `sigma_total` the design-wide process variables own.
    #[pyo3(get)]
    pub sigma_process: f64,
    /// The multiple of sigma the card asked the report to quote.
    #[pyo3(get)]
    pub sigma_multiplier: f64,
    /// How many contributors the analysis evaluated, including any the card's
    /// `CONTRIBUTORS` and `THRESHOLD` limits dropped from the list below.
    #[pyo3(get)]
    pub evaluated_contributors: usize,
    contributors: Vec<PyDcMatchContributor>,
    /// The core result, kept because the shared document is projected from it
    /// rather than rebuilt out of this projection's own floats.
    evidence: Option<DocumentEvidence<rspice_core::analysis::dcmatch::DcMatchResult>>,
}

impl CarriesDocumentEvidence for PyDcMatchResult {
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

impl PyDcMatchResult {
    pub(crate) fn from_core(result: &rspice_core::analysis::dcmatch::DcMatchResult) -> Self {
        Self {
            output: result.output.clone(),
            nominal_value: result.nominal_value,
            sigma_total: result.sigma_total,
            sigma_mismatch: result.sigma_mismatch,
            sigma_process: result.sigma_process,
            sigma_multiplier: result.sigma_multiplier,
            evaluated_contributors: result.evaluated_contributors,
            contributors: result
                .contributors
                .iter()
                .map(PyDcMatchContributor::from_core)
                .collect(),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::DcMatch,
                result.clone(),
            )),
        }
    }

    /// The shared result document, projected from the retained variance.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "DC mismatch")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let result = &evidence.core;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_dc_match(analysis, result)
        })
    }
}

#[pymethods]
impl PyDcMatchResult {
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

    /// The total standard deviation the card asked to be quoted:
    /// `sigma_multiplier * sigma_total`.
    #[getter]
    fn quoted_sigma(&self) -> f64 {
        self.sigma_multiplier * self.sigma_total
    }

    /// Contributors the card retained, largest variance share first.
    #[getter]
    fn contributors(&self) -> Vec<PyDcMatchContributor> {
        self.contributors.clone()
    }

    fn __len__(&self) -> usize {
        self.contributors.len()
    }

    /// Look up one contributor by instance and optional parameter name.
    #[pyo3(signature = (instance, parameter=None))]
    fn get(&self, instance: &str, parameter: Option<&str>) -> PyResult<PyDcMatchContributor> {
        self.contributors
            .iter()
            .find(|contributor| {
                contributor.instance.eq_ignore_ascii_case(instance)
                    && parameter.is_none_or(|parameter| {
                        contributor.parameter.eq_ignore_ascii_case(parameter)
                    })
            })
            .cloned()
            .ok_or_else(|| {
                let suffix = parameter.map_or(String::new(), |name| format!("/{name}"));
                crate::errors::key_error(format!(
                    "unknown mismatch contributor '{instance}{suffix}'"
                ))
            })
    }

    /// The largest `count` contributors, which are already the first ones:
    /// the analysis ranks by variance share.
    #[pyo3(signature = (count=10))]
    fn top(&self, count: usize) -> Vec<PyDcMatchContributor> {
        self.contributors.iter().take(count).cloned().collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "DcMatchResult(output='{}', nominal={:.6e}, sigma_total={:.6e}, contributors={})",
            self.output,
            self.nominal_value,
            self.sigma_total,
            self.contributors.len()
        )
    }
}
