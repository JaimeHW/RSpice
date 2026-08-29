//! Time-domain waveform results.
//!
//! `TransientResult` holds the full waveform set and the signal-spec resolution
//! that lets a caller address a node voltage, a differential pair, or a branch
//! current with the same probe grammar the deck uses.

use super::*;

/// Transient simulation result with time-domain waveforms
///
/// Contains time points, node voltage waveforms, and branch current
/// waveforms. Arrays are returned as NumPy ndarrays.
///
/// Example:
///     >>> result = engine.run_tran(netlist, 1e-3, 1e-6)
///     >>> import matplotlib.pyplot as plt
///     >>> plt.plot(result.time, result.voltage_waveform("out"))
#[pyclass(name = "TransientResult", module = "rspice")]
pub struct PyTransientResult {
    pub(crate) inner: TransientResult,
}

impl PyTransientResult {
    pub fn new(inner: TransientResult) -> Self {
        Self { inner }
    }

    fn device_op_values(&self, device: &str, parameter: &str) -> PyResult<&[f64]> {
        self.inner
            .device_op_traces
            .iter()
            .find(|trace| {
                trace.device_name.eq_ignore_ascii_case(device)
                    && trace.parameter.eq_ignore_ascii_case(parameter)
            })
            .map(|trace| trace.values.as_slice())
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
    }

    /// Discard samples before a requested SPICE `TSTART` output boundary.
    ///
    /// The solver must still integrate from zero so dynamic state at TSTART
    /// is correct; only the returned/output data is clipped. All time-aligned
    /// vectors are validated before mutation so malformed core results cannot
    /// become silently misaligned Python arrays.
    pub(crate) fn new_with_start(mut inner: TransientResult, start_time: f64) -> PyResult<Self> {
        if start_time <= 0.0 {
            return Ok(Self { inner });
        }

        let original_len = inner.time.len();
        let start_index = inner.time.partition_point(|time| *time < start_time);
        if start_index >= original_len {
            return Err(crate::errors::SimulationError::new_err(format!(
                "transient result contains no sample at or after requested start_time {start_time}"
            )));
        }

        for (kind, series) in inner
            .voltages
            .iter()
            .map(|series| ("voltage", series))
            .chain(
                inner
                    .branch_currents
                    .iter()
                    .map(|series| ("branch-current", series)),
            )
            .chain(
                inner
                    .device_op_traces
                    .iter()
                    .map(|trace| ("device operating-point", &trace.values)),
            )
        {
            if series.len() != original_len {
                return Err(crate::errors::SimulationError::new_err(format!(
                    "malformed transient result: {kind} series has {} samples but time has {original_len}",
                    series.len()
                )));
            }
        }

        inner.time.drain(..start_index);
        for series in &mut inner.voltages {
            series.drain(..start_index);
        }
        for series in &mut inner.branch_currents {
            series.drain(..start_index);
        }
        for trace in &mut inner.device_op_traces {
            trace.values.drain(..start_index);
        }

        // Event traces store changes rather than one value per accepted analog
        // point. Preserve the state in force at TSTART, then subsequent events.
        for trace in &mut inner.digital_traces {
            let prior = trace
                .points
                .iter()
                .rev()
                .find(|point| point.time < start_time)
                .copied();
            trace.points.retain(|point| point.time >= start_time);
            if trace
                .points
                .first()
                .is_none_or(|point| point.time > start_time)
                && let Some(mut point) = prior
            {
                point.time = start_time;
                trace.points.insert(0, point);
            }
        }
        for trace in &mut inner.real_traces {
            let prior = trace
                .points
                .iter()
                .rev()
                .find(|point| point.time < start_time)
                .copied();
            trace.points.retain(|point| point.time >= start_time);
            if trace
                .points
                .first()
                .is_none_or(|point| point.time > start_time)
                && let Some(mut point) = prior
            {
                point.time = start_time;
                trace.points.insert(0, point);
            }
        }

        Ok(Self { inner })
    }

    fn checked_time_index(&self, time_index: usize) -> AccessResult<()> {
        if time_index < self.inner.time.len() {
            Ok(())
        } else {
            Err(invalid_time_index_error(time_index, self.inner.time.len()))
        }
    }

    fn checked_waveform(&self, node: usize) -> AccessResult<Vec<f64>> {
        if node == 0 {
            return Ok(vec![0.0; self.inner.num_points()]);
        }

        self.inner
            .try_voltage_waveform(node)
            .map(|waveform| waveform.to_vec())
            .ok_or_else(|| invalid_node_index_error(node, self.inner.num_nodes))
    }

    fn checked_waveform_named(&self, name: &str) -> AccessResult<Vec<f64>> {
        if is_ground_name(name) {
            return self.checked_waveform(0);
        }

        let node = self
            .inner
            .node_index_named(name)
            .ok_or_else(|| unknown_node_name_error(name))?;
        self.checked_waveform(node)
    }

    fn waveform_for(&self, node: &NodeIdentifier) -> PyResult<Vec<f64>> {
        match node {
            NodeIdentifier::Index(idx) => self.checked_waveform(*idx),
            NodeIdentifier::Name(name) => self.checked_waveform_named(name),
        }
        .map_err(PyErr::from)
    }

    /// Resolve any SPICE output specification to a time series.
    ///
    /// Differential voltages are evaluated sample by sample rather than being
    /// approximated from a single node, and branch currents come from the MNA
    /// branch equations rather than the node map.
    pub(crate) fn signal_waveform(&self, spec: &SignalSpec) -> PyResult<Vec<f64>> {
        match spec {
            SignalSpec::Voltage {
                node,
                reference: None,
            } => self
                .checked_waveform_named(node)
                .map_err(|_| unknown_signal_error(spec, "node")),
            SignalSpec::Voltage {
                node,
                reference: Some(reference),
            } => {
                let positive = self
                    .checked_waveform_named(node)
                    .map_err(|_| unknown_signal_error(spec, "node"))?;
                let negative = self
                    .checked_waveform_named(reference)
                    .map_err(|_| unknown_signal_error(spec, "reference node"))?;
                if positive.len() != negative.len() {
                    return Err(crate::errors::value_error(format!(
                        "malformed transient result: '{}' and its reference have {} and {} samples",
                        spec.label(),
                        positive.len(),
                        negative.len()
                    )));
                }
                Ok(positive
                    .iter()
                    .zip(&negative)
                    .map(|(high, low)| high - low)
                    .collect())
            }
            SignalSpec::Current { element } => self
                .inner
                .try_branch_current_waveform_named(element)
                .map(<[f64]>::to_vec)
                .ok_or_else(|| unknown_signal_error(spec, "branch")),
        }
    }
}

/// Error for a probe whose circuit quantity does not exist, naming the
/// original specification rather than an extracted fragment.
fn unknown_signal_error(spec: &SignalSpec, what: &str) -> PyErr {
    crate::errors::key_error(format!(
        "unknown {what} in output specification '{}'",
        spec.label()
    ))
}

impl PyTransientResult {
    /// Column layout shared by the CSV and raw exporters.
    fn raw_plot(&self, title: &str) -> crate::export::RawPlot {
        use crate::export::{RawVariable, RawVariableKind};
        let real = |values: &[f64]| -> Vec<rspice_core::Complex64> {
            values
                .iter()
                .map(|value| rspice_core::Complex64::new(*value, 0.0))
                .collect()
        };

        let mut variables = vec![RawVariable {
            name: "time".to_string(),
            kind: RawVariableKind::Time,
        }];
        let mut series = vec![real(&self.inner.time)];
        for (index, name) in self.inner.node_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(real(
                self.inner
                    .voltages
                    .get(index)
                    .map_or(&[][..], Vec::as_slice),
            ));
        }
        for (index, name) in self.inner.branch_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(real(
                self.inner
                    .branch_currents
                    .get(index)
                    .map_or(&[][..], Vec::as_slice),
            ));
        }

        crate::export::RawPlot {
            title: title.to_string(),
            plot_name: "Transient Analysis".to_string(),
            variables,
            series,
            complex: false,
            timestamp: None,
        }
    }

    /// Shared `.FOUR` evaluation for every waveform source.
    fn fourier_of_waveform(
        &self,
        waveform: &[f64],
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        if !fundamental.is_finite() || fundamental <= 0.0 {
            return Err(crate::errors::value_error(format!(
                "fundamental must be a positive finite frequency in Hz, got {fundamental}"
            )));
        }
        if num_harmonics == 0 {
            return Err(crate::errors::value_error(
                "num_harmonics must be at least 1",
            ));
        }
        let analysis =
            FourierAnalysis::new(FourierConfig::new(fundamental).with_harmonics(num_harmonics));
        let result = analysis
            .analyze(&self.inner.time, waveform)
            .map_err(|error| {
                crate::errors::value_error(format!(
                    "Fourier waveform could not be analyzed: {error}"
                ))
            })?;
        Ok(PyFourierResult::from_core(&result))
    }
}

#[pymethods]
impl PyTransientResult {
    /// Get the time points array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of time points in seconds
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    /// Get the voltage waveform for a node
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Returns:
    ///     numpy.ndarray: Voltage values at each time point
    ///
    /// Raises:
    ///     IndexError: If the node index is out of range
    ///     KeyError: If the node name does not exist
    ///
    /// Example:
    ///     >>> v_out = result.voltage_waveform(2)
    ///     >>> v_out = result.voltage_waveform("out")
    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.waveform_for(&node)?.to_pyarray(py))
    }

    /// Get the current waveform through a branch element
    ///
    /// Branch currents exist for voltage sources and inductors (MNA branch
    /// equations).
    ///
    /// Args:
    ///     name: Element name (e.g. "V1", "L2")
    ///
    /// Returns:
    ///     numpy.ndarray: Current values at each time point
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    ///
    /// Example:
    ///     >>> i_supply = result.branch_current_waveform("V1")
    fn branch_current_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        self.inner
            .try_branch_current_waveform_named(name)
            .map(|waveform| waveform.to_pyarray(py))
            .ok_or_else(|| unknown_branch_name_error(name))
            .map_err(PyErr::from)
    }

    /// Device operating-point traces requested with `.SAVE @device[param]`.
    #[getter]
    fn device_parameter_names(&self) -> Vec<String> {
        self.inner
            .device_op_traces
            .iter()
            .map(|trace| format!("@{}[{}]", trace.device_name, trace.parameter))
            .collect()
    }

    fn device_parameter_waveform<'py>(
        &self,
        py: Python<'py>,
        device: &str,
        parameter: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self.device_op_values(device, parameter)?;
        if values.len() != self.inner.time.len() {
            return Err(crate::errors::value_error(format!(
                "malformed device operating-point trace '@{device}[{parameter}]': {} samples for {} time points",
                values.len(),
                self.inner.time.len()
            )));
        }
        Ok(values.to_pyarray(py))
    }

    /// Get voltage at a specific node and time index
    ///
    /// Args:
    ///     node: Node index
    ///     time_index: Index into the time array
    ///
    /// Returns:
    ///     float: Voltage at the specified node and time
    ///
    /// Raises:
    ///     IndexError: If the node or time index is out of range
    pub fn voltage_at(&self, node: usize, time_index: usize) -> PyResult<f64> {
        self.checked_time_index(time_index).map_err(PyErr::from)?;
        if node == 0 {
            return Ok(0.0);
        }

        self.inner
            .try_voltage_at(node, time_index)
            .ok_or_else(|| invalid_node_index_error(node, self.inner.num_nodes))
            .map_err(PyErr::from)
    }

    /// Fourier-analyze a node waveform
    ///
    /// Decomposes the waveform into harmonics of `fundamental` and computes
    /// total harmonic distortion. Equivalent to the `.FOUR` SPICE analysis.
    ///
    /// Args:
    ///     node: Node index or name
    ///     fundamental: Fundamental frequency in Hz
    ///     num_harmonics: Number of harmonics to compute (default 9)
    ///
    /// Returns:
    ///     FourierResult: DC component, harmonics, and optional THD. THD is
    ///     None when the fundamental magnitude is exactly zero.
    ///
    /// Raises:
    ///     ValueError: If the Fourier configuration or waveform evidence is invalid
    ///     IndexError / KeyError: For invalid nodes
    ///
    /// Example:
    ///     >>> four = tran.fourier("out", fundamental=1e3)
    ///     >>> print("undefined" if four.thd_percent is None else f"{four.thd_percent:.2f}%")
    ///     >>> diff = tran.fourier("outp", 1e3, reference="outn")
    #[pyo3(signature = (node, fundamental, num_harmonics=9, *, reference=None))]
    fn fourier(
        &self,
        node: NodeIdentifier,
        fundamental: f64,
        num_harmonics: usize,
        reference: Option<NodeIdentifier>,
    ) -> PyResult<PyFourierResult> {
        let waveform = match reference {
            None => self.waveform_for(&node)?,
            Some(reference) => {
                let positive = self.waveform_for(&node)?;
                let negative = self.waveform_for(&reference)?;
                positive
                    .iter()
                    .zip(&negative)
                    .map(|(high, low)| high - low)
                    .collect()
            }
        };
        self.fourier_of_waveform(&waveform, fundamental, num_harmonics)
    }

    /// Fourier-analyze a branch-current waveform
    ///
    /// The `.FOUR` counterpart of `fourier` for `I(element)` outputs. Branch
    /// currents exist for voltage sources and inductors.
    ///
    /// Args:
    ///     element: Element name (e.g. "V1", "L2")
    ///     fundamental: Fundamental frequency in Hz
    ///     num_harmonics: Number of harmonics to compute (default 9)
    ///
    /// Raises:
    ///     ValueError: If fundamental is not a positive finite number
    ///     KeyError: If no branch carries that name
    ///
    /// Example:
    ///     >>> four = tran.fourier_current("V1", fundamental=1e3)
    #[pyo3(signature = (element, fundamental, num_harmonics=9))]
    fn fourier_current(
        &self,
        element: &str,
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        let waveform = self.signal_waveform(&SignalSpec::Current {
            element: element.to_string(),
        })?;
        self.fourier_of_waveform(&waveform, fundamental, num_harmonics)
    }

    /// Evaluate any SPICE output specification against this result
    ///
    /// Accepts `V(out)`, `V(outp,outn)`, `I(V1)`, or a bare node name — the
    /// same probe grammar `.FOUR` and `.PRINT` use.
    ///
    /// Args:
    ///     spec: Output specification string
    ///
    /// Returns:
    ///     numpy.ndarray: Signal values at each time point
    ///
    /// Raises:
    ///     ValueError: If the specification is malformed
    ///     KeyError: If the node, reference node, or branch does not exist
    ///
    /// Example:
    ///     >>> vdiff = tran.signal("V(outp,outn)")
    ///     >>> isupply = tran.signal("I(V1)")
    fn signal<'py>(&self, py: Python<'py>, spec: &str) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let parsed = crate::signal::parse_signal_spec(spec).map_err(crate::errors::value_error)?;
        Ok(self.signal_waveform(&parsed)?.to_pyarray(py))
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

    /// Render every node voltage and branch current as RFC 4180 CSV.
    ///
    /// The first column is `time`; the rest are `V(node)` and `I(element)`
    /// in `node_names` then `branch_names` order. Values carry full
    /// double precision so the table round-trips exactly.
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
    ///     title: Header title; defaults to "RSpice transient analysis"
    ///
    /// Returns:
    ///     bytes: The complete raw file
    #[pyo3(signature = (*, format="ascii", title=None, timestamp=None))]
    fn to_raw<'py>(
        &self,
        py: Python<'py>,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<Bound<'py, pyo3::types::PyBytes>> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice transient analysis"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        Ok(pyo3::types::PyBytes::new(py, &bytes))
    }

    /// Write a SPICE raw file. Same options as `to_raw`.
    ///
    /// Example:
    ///     >>> tran.write_raw("run.raw", format="binary")
    #[pyo3(signature = (path, *, format="ascii", title=None, timestamp=None))]
    fn write_raw(
        &self,
        path: PathBuf,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<()> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice transient analysis"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        write_export_file(&path, &bytes)
    }

    /// Get the number of time points
    ///
    /// Returns:
    ///     int: Number of time points in the simulation
    #[getter]
    pub fn num_points(&self) -> usize {
        self.inner.num_points()
    }

    /// Get the number of nodes
    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes
    }

    /// Get node names
    ///
    /// Returns:
    ///     list[str]: List of node names
    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    /// Get branch names aligned with branch current waveforms
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names.clone()
    }

    /// Get the simulation stop time
    #[getter]
    pub fn stop_time(&self) -> f64 {
        self.inner.time.last().copied().unwrap_or(0.0)
    }

    fn __repr__(&self) -> String {
        format!(
            "TransientResult(nodes={}, points={}, stop_time={:.3e}s)",
            self.inner.num_nodes,
            self.inner.num_points(),
            self.stop_time()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// XSPICE digital and real event traces are not part of this type's
    /// Python surface and are therefore not carried; every quantity a caller
    /// can read back is.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        time: Vec<f64>,
        step_sizes: Vec<f64>,
        voltages: Vec<Vec<f64>>,
        branch_currents: Vec<Vec<f64>>,
        num_nodes: usize,
        names: (Vec<String>, Vec<String>),
        device_op_traces: Vec<(String, String, Vec<f64>)>,
        store_traces: Vec<(String, Vec<f64>)>,
    ) -> Self {
        let (node_names, branch_names) = names;
        Self::new(TransientResult {
            time,
            step_sizes,
            voltages,
            branch_currents,
            num_nodes,
            node_names,
            branch_names,
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: device_op_traces
                .into_iter()
                .map(|(device_name, parameter, values)| {
                    rspice_core::engine::TransientDeviceOpTrace {
                        device_name,
                        parameter,
                        values,
                    }
                })
                .collect(),
            store_traces: store_traces
                .into_iter()
                .map(|(name, values)| rspice_core::engine::TransientStoreTrace { name, values })
                .collect(),
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
            Vec<f64>,
            Vec<Vec<f64>>,
            Vec<Vec<f64>>,
            usize,
            (Vec<String>, Vec<String>),
            Vec<(String, String, Vec<f64>)>,
            Vec<(String, Vec<f64>)>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.time.clone(),
                self.inner.step_sizes.clone(),
                self.inner.voltages.clone(),
                self.inner.branch_currents.clone(),
                self.inner.num_nodes,
                (
                    self.inner.node_names.clone(),
                    self.inner.branch_names.clone(),
                ),
                self.inner
                    .device_op_traces
                    .iter()
                    .map(|trace| {
                        (
                            trace.device_name.clone(),
                            trace.parameter.clone(),
                            trace.values.clone(),
                        )
                    })
                    .collect(),
                self.inner
                    .store_traces
                    .iter()
                    .map(|trace| (trace.name.clone(), trace.values.clone()))
                    .collect(),
            ),
        ))
    }
}
