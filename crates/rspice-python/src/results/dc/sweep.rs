//! A swept sequence of DC operating points.
//!
//! One or two swept sources produce a flattened grid whose inner axis varies
//! fastest, and `.STEP`/`.TEMP` runs reuse the same shape rather than
//! introducing a parallel one.

use super::*;

/// Split reported sweep points into the retained solutions and their reports.
fn split_reported_points(
    results: Vec<rspice_core::engine::DcSweepPointResult>,
) -> (
    Vec<(f64, SimulationResult)>,
    Vec<rspice_core::circuit::DeviceOpReport>,
) {
    results
        .into_iter()
        .map(|point| ((point.sweep_value, point.result), point.device_op_report))
        .unzip()
}

/// One sweep point's device report in its Python projection.
fn projected_device_report(
    report: &rspice_core::circuit::DeviceOpReport,
) -> Vec<PyDeviceOperatingPoint> {
    report
        .entries
        .iter()
        .cloned()
        .map(PyDeviceOperatingPoint::from_core)
        .collect()
}

/// DC sweep analysis result
///
/// A sequence of (sweep_value, SimulationResult) pairs. Supports `len()`,
/// indexing (including negative indices), and iteration:
///
///     >>> for v_in, sol in engine.run_dc_sweep(netlist, "V1", 0, 5, 0.1):
///     ...     print(v_in, sol.voltage("out"))
#[pyclass(name = "DcSweepResult", module = "rspice", from_py_object)]
#[derive(Clone)]
pub struct PyDcSweepResult {
    pub(crate) results: Vec<(f64, SimulationResult)>,
    /// `None` when the producing run captured no device operating-point
    /// reports. When present it holds exactly one entry per sweep point, so a
    /// point can never silently borrow another point's devices or be handed an
    /// empty list that reads as "this point has no devices".
    device_operating_points: Option<Vec<Vec<PyDeviceOperatingPoint>>>,
    primary_source: Option<String>,
    secondary_source: Option<String>,
    secondary_sweep_values: Option<Vec<f64>>,
    inner_points: usize,
    /// Per-point core device reports, kept beside their Python projection
    /// because the projection owns its parameter names and cannot be turned
    /// back into a report. `None` when the producing run captured none.
    evidence: Option<DocumentEvidence<Option<Vec<rspice_core::circuit::DeviceOpReport>>>>,
}

impl CarriesDocumentEvidence for PyDcSweepResult {
    fn bind_analysis(&mut self, analysis: rspice_core::execution::AnalysisInstanceId) {
        self.evidence = self
            .evidence
            .take()
            .map(|evidence| evidence.with_analysis(analysis));
    }
}

impl PyDcSweepResult {
    pub fn new(results: Vec<(f64, SimulationResult)>) -> Self {
        let inner_points = results.len();
        Self {
            results,
            device_operating_points: None,
            primary_source: None,
            secondary_source: None,
            secondary_sweep_values: None,
            inner_points,
            // A sweep with no named axis cannot be published as a `dc`
            // document: the shared schema names the swept variable, and this
            // constructor was told none.
            evidence: None,
        }
    }

    pub fn new_named(results: Vec<(f64, SimulationResult)>, primary_source: &str) -> Self {
        let mut result = Self::new(results);
        result.primary_source = Some(primary_source.to_string());
        result.evidence = Some(DocumentEvidence::sole(
            rspice_core::execution::AnalysisKind::Dc,
            None,
        ));
        result
    }

    /// The shared result document, projected from the retained sweep points.
    ///
    /// The core projection takes complete sweep points, so the retained
    /// solutions and their device reports are re-paired here; the pairing is
    /// a temporary the document build consumes, not a second retained copy.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        use rspice_core::execution::result_document::DcSweepAxisDocument;

        let evidence = document::evidence(&self.evidence, "DC-sweep")?;
        let analysis = evidence.analysis;
        let primary = self.primary_source.clone().ok_or_else(|| {
            crate::errors::SimulationError::new_err(
                "this DC sweep result names no swept variable, which the shared result document \
                 requires",
            )
        })?;
        let reports = evidence.core.as_deref();
        if let Some(reports) = reports
            && reports.len() != self.results.len()
        {
            return Err(crate::errors::SimulationError::new_err(format!(
                "malformed DC sweep result: device operating-point reports cover {} of {} sweep \
                 points",
                reports.len(),
                self.results.len()
            )));
        }
        let points = self
            .results
            .iter()
            .enumerate()
            .map(
                |(index, (sweep_value, result))| rspice_core::engine::DcSweepPointResult {
                    sweep_value: *sweep_value,
                    result: result.clone(),
                    device_op_report: reports
                        .and_then(|reports| reports.get(index))
                        .cloned()
                        .unwrap_or_default(),
                },
            )
            .collect::<Vec<_>>();
        let secondary = self.secondary_source.clone();
        let secondary_values = self.secondary_sweep_values.clone();
        let inner_points = self.inner_points;
        document::build(py, move |abort| {
            let builder = match (secondary, secondary_values) {
                (Some(outer), Some(values)) => AnalysisResultDocument::from_nested_dc_sweep(
                    analysis,
                    &[
                        DcSweepAxisDocument {
                            name: outer.trim().to_ascii_lowercase(),
                            unit: rspice_core::execution::sweep_axis_unit(&outer),
                            value_count: values.len(),
                        },
                        DcSweepAxisDocument {
                            name: primary.trim().to_ascii_lowercase(),
                            unit: rspice_core::execution::sweep_axis_unit(&primary),
                            value_count: inner_points,
                        },
                    ],
                    &points,
                )?,
                _ => AnalysisResultDocument::from_dc_sweep(
                    analysis,
                    &primary,
                    rspice_core::execution::sweep_axis_unit(&primary),
                    &points,
                )?,
            };
            builder.build_with_abort(abort)
        })
    }

    pub fn new_named_with_reports(
        results: Vec<rspice_core::engine::DcSweepPointResult>,
        primary_source: &str,
    ) -> Self {
        let inner_points = results.len();
        let (points, reports) = split_reported_points(results);
        Self {
            results: points,
            device_operating_points: Some(reports.iter().map(projected_device_report).collect()),
            primary_source: Some(primary_source.to_string()),
            secondary_source: None,
            secondary_sweep_values: None,
            inner_points,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Dc,
                Some(reports),
            )),
        }
    }

    pub fn new_nested(
        results: Vec<(f64, SimulationResult)>,
        primary_source: &str,
        secondary_source: &str,
        secondary_sweep_values: Vec<f64>,
    ) -> PyResult<Self> {
        if secondary_sweep_values.is_empty()
            || !results.len().is_multiple_of(secondary_sweep_values.len())
        {
            return Err(crate::errors::SimulationError::new_err(format!(
                "malformed nested DC sweep: {} result points cannot be divided across {} secondary sweep values",
                results.len(),
                secondary_sweep_values.len()
            )));
        }
        let inner_points = results.len() / secondary_sweep_values.len();
        Ok(Self {
            results,
            device_operating_points: None,
            primary_source: Some(primary_source.to_string()),
            secondary_source: Some(secondary_source.to_string()),
            secondary_sweep_values: Some(secondary_sweep_values),
            inner_points,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Dc,
                None,
            )),
        })
    }

    pub fn new_nested_with_reports(
        results: Vec<rspice_core::engine::DcSweepPointResult>,
        primary_source: &str,
        secondary_source: &str,
        secondary_sweep_values: Vec<f64>,
    ) -> PyResult<Self> {
        if secondary_sweep_values.is_empty()
            || !results.len().is_multiple_of(secondary_sweep_values.len())
        {
            return Err(crate::errors::SimulationError::new_err(format!(
                "malformed nested DC sweep: {} result points cannot be divided across {} secondary sweep values",
                results.len(),
                secondary_sweep_values.len()
            )));
        }
        let inner_points = results.len() / secondary_sweep_values.len();
        let (points, reports) = split_reported_points(results);
        Ok(Self {
            results: points,
            device_operating_points: Some(reports.iter().map(projected_device_report).collect()),
            primary_source: Some(primary_source.to_string()),
            secondary_source: Some(secondary_source.to_string()),
            secondary_sweep_values: Some(secondary_sweep_values),
            inner_points,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Dc,
                Some(reports),
            )),
        })
    }

    fn point(&self, index: usize) -> AccessResult<&(f64, SimulationResult)> {
        self.results
            .get(index)
            .ok_or_else(|| invalid_sweep_index_error(index, self.results.len()))
    }

    /// Device operating-point report for one sweep point.
    ///
    /// `Ok(None)` means this sweep captured no reports at all. A point that is
    /// missing from a captured report is a malformed result, not an empty
    /// device list, so it fails instead of publishing one.
    fn device_operating_points_for(
        &self,
        index: usize,
    ) -> PyResult<Option<Vec<PyDeviceOperatingPoint>>> {
        let Some(entries) = &self.device_operating_points else {
            return Ok(None);
        };
        entries.get(index).cloned().map(Some).ok_or_else(|| {
            crate::errors::SimulationError::new_err(format!(
                "malformed DC sweep result: device operating-point reports cover {} of {} sweep points, \
                 so point {index} has none",
                entries.len(),
                self.results.len()
            ))
        })
    }
}

#[pymethods]
impl PyDcSweepResult {
    /// Name of the primary (inner) sweep source, when known.
    #[getter]
    fn primary_source(&self) -> Option<String> {
        self.primary_source.clone()
    }

    /// Name of the secondary (outer) sweep source for a nested `.DC` sweep.
    #[getter]
    fn secondary_source(&self) -> Option<String> {
        self.secondary_source.clone()
    }

    /// Unique secondary sweep values in outer-loop order.
    #[getter]
    fn secondary_sweep_values<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.secondary_sweep_values
            .as_ref()
            .map(|values| values.to_pyarray(py))
    }

    /// `(outer_points, inner_points)` shape of the flattened sweep result.
    #[getter]
    fn shape(&self) -> (usize, usize) {
        (
            self.secondary_sweep_values.as_ref().map_or(1, Vec::len),
            self.inner_points,
        )
    }

    /// Whether this result came from a two-source nested `.DC` sweep.
    #[getter]
    fn is_nested(&self) -> bool {
        self.secondary_sweep_values.is_some()
    }

    /// Secondary sweep coordinate for a flattened result index.
    fn secondary_value_at(&self, index: usize) -> PyResult<Option<f64>> {
        self.point(index).map_err(PyErr::from)?;
        Ok(self.secondary_sweep_values.as_ref().map(|values| {
            let outer_index = index / self.inner_points;
            values[outer_index]
        }))
    }

    /// Get the sweep values array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of swept source values
    #[getter]
    fn sweep_values<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        let values: Vec<f64> = self.results.iter().map(|(v, _)| *v).collect();
        values.to_pyarray(py)
    }

    /// Get the number of sweep points
    fn __len__(&self) -> usize {
        self.results.len()
    }

    /// Get all sweep points as (value, result) pairs
    ///
    /// Returns:
    ///     list[tuple[float, SimulationResult]]: One entry per sweep point
    pub fn points(&self) -> PyResult<Vec<(f64, PySimulationResult)>> {
        self.results
            .iter()
            .enumerate()
            .map(|(index, (value, result))| {
                Ok((
                    *value,
                    PySimulationResult::new_with_device_operating_points(
                        result.clone(),
                        self.device_operating_points_for(index)?,
                    ),
                ))
            })
            .collect()
    }

    /// Device operating-point data at one sweep point.
    ///
    /// `None` when this sweep captured no reports; an empty list when it did
    /// and this point has no device with an operating point.
    fn device_operating_points_at(
        &self,
        index: usize,
    ) -> PyResult<Option<Vec<PyDeviceOperatingPoint>>> {
        self.point(index).map_err(PyErr::from)?;
        self.device_operating_points_for(index)
    }

    /// Whether this sweep carries device operating-point reports.
    #[getter]
    fn has_device_operating_points(&self) -> bool {
        self.device_operating_points.is_some()
    }

    /// Iterate over (value, SimulationResult) pairs
    fn __iter__(slf: PyRef<'_, Self>, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let points = slf.points()?;
        let list = points.into_pyobject(py)?;
        Ok(list.call_method0("__iter__")?.unbind())
    }

    /// Index into the sweep: `result[i]` -> (value, SimulationResult)
    ///
    /// Supports negative indices.
    fn __getitem__(&self, index: isize) -> PyResult<(f64, PySimulationResult)> {
        let len = self.results.len() as isize;
        let idx = if index < 0 { index + len } else { index };
        if idx < 0 || idx >= len {
            return Err(invalid_sweep_index_error(index.unsigned_abs(), self.results.len()).into());
        }
        let index = idx as usize;
        let (value, result) = &self.results[index];
        Ok((
            *value,
            PySimulationResult::new_with_device_operating_points(
                result.clone(),
                self.device_operating_points_for(index)?,
            ),
        ))
    }

    /// Get the result at a specific sweep index
    ///
    /// Carries the same per-device operating-point data as `sweep[index]`
    /// and `points()`.
    ///
    /// Raises:
    ///     IndexError: If the sweep index is out of range
    pub fn result_at(&self, index: usize) -> PyResult<PySimulationResult> {
        let (_, result) = self.point(index).map_err(PyErr::from)?;
        Ok(PySimulationResult::new_with_device_operating_points(
            result.clone(),
            self.device_operating_points_for(index)?,
        ))
    }

    /// Get the sweep value at a specific index
    ///
    /// Raises:
    ///     IndexError: If the sweep index is out of range
    pub fn sweep_value_at(&self, index: usize) -> PyResult<f64> {
        self.point(index)
            .map(|(value, _)| *value)
            .map_err(PyErr::from)
    }

    /// Get voltage at a node for a specific sweep point
    ///
    /// Raises:
    ///     IndexError: If the sweep point or node index is out of range
    ///     KeyError: If the node name does not exist
    fn voltage(&self, index: usize, node: NodeIdentifier) -> PyResult<f64> {
        let (_, result) = self.point(index).map_err(PyErr::from)?;
        match node {
            NodeIdentifier::Index(idx) => checked_simulation_voltage(result, idx),
            NodeIdentifier::Name(name) => checked_simulation_voltage_named(result, &name),
        }
        .map_err(PyErr::from)
    }

    /// Get voltage at a node across all sweep points as a NumPy array
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Raises:
    ///     IndexError / KeyError: For invalid nodes
    fn voltage_array<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let mut voltages = Vec::with_capacity(self.results.len());
        for (_, result) in &self.results {
            let v = match &node {
                NodeIdentifier::Index(idx) => checked_simulation_voltage(result, *idx),
                NodeIdentifier::Name(name) => checked_simulation_voltage_named(result, name),
            }
            .map_err(PyErr::from)?;
            voltages.push(v);
        }
        Ok(voltages.to_pyarray(py))
    }

    /// Column headers used by `to_csv` and the raw exporters, in order.
    #[getter]
    fn export_columns(&self) -> Vec<String> {
        self.raw_plot("")
            .variables
            .into_iter()
            .map(|v| v.name)
            .collect()
    }

    /// Render the sweep as RFC 4180 CSV.
    ///
    /// The first column is the swept source; the rest are `V(node)` and
    /// `I(element)`. A nested sweep is flattened in the same order as
    /// `points()`, with the secondary coordinate in its own column.
    fn to_csv(&self) -> PyResult<String> {
        let plot = self.raw_plot("");
        crate::export::csv(
            &plot
                .variables
                .iter()
                .map(|v| v.name.clone())
                .collect::<Vec<_>>(),
            &transpose_real(&plot.series),
        )
        .map_err(crate::errors::value_error)
    }

    /// Write `to_csv()` to a file.
    fn write_csv(&self, path: PathBuf) -> PyResult<()> {
        write_export_file(&path, self.to_csv()?.as_bytes())
    }

    /// Serialize as an ngspice-compatible SPICE raw file.
    ///
    /// Args:
    ///     format: "ascii" (default) or "binary"
    ///     title: Header title; defaults to "RSpice DC sweep"
    #[pyo3(signature = (*, format="ascii", title=None, timestamp=None))]
    fn to_raw<'py>(
        &self,
        py: Python<'py>,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<Bound<'py, pyo3::types::PyBytes>> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice DC sweep"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        Ok(pyo3::types::PyBytes::new(py, &bytes))
    }

    /// Write a SPICE raw file. Same options as `to_raw`.
    #[pyo3(signature = (path, *, format="ascii", title=None, timestamp=None))]
    fn write_raw(
        &self,
        path: PathBuf,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<()> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice DC sweep"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        write_export_file(&path, &bytes)
    }

    fn __repr__(&self) -> String {
        let sweep_range = match (self.results.first(), self.results.last()) {
            (Some(first), Some(last)) => format!("{:.2}-{:.2}", first.0, last.0),
            _ => "empty".to_string(),
        };
        format!(
            "DcSweepResult(points={}, range={}, shape={:?})",
            self.results.len(),
            sweep_range,
            self.shape()
        )
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

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        points: Vec<(f64, SimulationResultState)>,
        device_operating_points: Option<Vec<Vec<PyDeviceOperatingPoint>>>,
        primary_source: Option<String>,
        secondary_source: Option<String>,
        secondary_sweep_values: Option<Vec<f64>>,
        inner_points: usize,
    ) -> Self {
        Self {
            results: points
                .into_iter()
                .map(|(value, state)| (value, rebuild_simulation_result(state)))
                .collect(),
            device_operating_points,
            primary_source,
            secondary_source,
            secondary_sweep_values,
            inner_points,
            evidence: None,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<(f64, SimulationResultState)>,
            Option<Vec<Vec<PyDeviceOperatingPoint>>>,
            Option<String>,
            Option<String>,
            Option<Vec<f64>>,
            usize,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.results
                    .iter()
                    .map(|(value, result)| (*value, simulation_result_state(result)))
                    .collect(),
                self.device_operating_points.clone(),
                self.primary_source.clone(),
                self.secondary_source.clone(),
                self.secondary_sweep_values.clone(),
                self.inner_points,
            ),
        ))
    }
}

impl PyDcSweepResult {
    /// Column layout shared by the CSV and raw exporters.
    ///
    /// The sweep axis is a voltage rather than time or frequency, so it is
    /// declared as a voltage column, matching how ngspice writes DC plots.
    fn raw_plot(&self, title: &str) -> crate::export::RawPlot {
        use crate::export::{RawVariable, RawVariableKind};
        let complex = |values: Vec<f64>| -> Vec<rspice_core::Complex64> {
            values
                .into_iter()
                .map(|value| rspice_core::Complex64::new(value, 0.0))
                .collect()
        };

        let axis_name = self
            .primary_source
            .clone()
            .unwrap_or_else(|| "sweep".to_string());
        let mut variables = vec![RawVariable {
            name: format!("v-sweep({axis_name})"),
            kind: RawVariableKind::Voltage,
        }];
        let mut series = vec![complex(
            self.results.iter().map(|(value, _)| *value).collect(),
        )];

        if let (Some(secondary), Some(values)) =
            (&self.secondary_source, &self.secondary_sweep_values)
        {
            variables.push(RawVariable {
                name: format!("v-sweep2({secondary})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(complex(
                (0..self.results.len())
                    .map(|index| {
                        values
                            .get(index / self.inner_points.max(1))
                            .copied()
                            .unwrap_or(f64::NAN)
                    })
                    .collect(),
            ));
        }

        // Every point of a sweep shares one circuit, so the first result's
        // name vectors define the columns.
        let (node_names, branch_names) = self.results.first().map_or_else(
            || (Vec::new(), Vec::new()),
            |(_, result)| (result.node_names.clone(), result.branch_names.clone()),
        );
        // node_names[0] is ground; it is a constant zero column and is
        // deliberately omitted, matching the transient and AC exporters.
        for (index, name) in node_names.iter().enumerate().skip(1) {
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(complex(
                self.results
                    .iter()
                    .map(|(_, result)| result.node_voltages.get(index).copied().unwrap_or(f64::NAN))
                    .collect(),
            ));
        }
        for (index, name) in branch_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(complex(
                self.results
                    .iter()
                    .map(|(_, result)| {
                        result
                            .branch_currents
                            .get(index)
                            .copied()
                            .unwrap_or(f64::NAN)
                    })
                    .collect(),
            ));
        }

        crate::export::RawPlot {
            title: title.to_string(),
            plot_name: "DC transfer characteristic".to_string(),
            variables,
            series,
            complex: false,
            timestamp: None,
        }
    }
}
