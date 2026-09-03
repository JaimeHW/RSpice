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
    ///
    /// Qualifying and transforming a long waveform is unbounded work, so it
    /// runs on the interruptible worker: `KeyboardInterrupt` stops it and the
    /// GIL is released while it runs. A result object owns no engine, so the
    /// run is not registered with one — `Engine.cancel_all()` does not reach
    /// post-processing of an already-returned result.
    ///
    /// The sample grid is borrowed rather than copied across that release,
    /// which is sound because this class exposes no mutating method: there is
    /// no `&mut self` entry point and no `__setstate__`, so nothing Python
    /// can call while the worker runs can move the values out from under it.
    fn fourier_of_waveform(
        &self,
        py: Python<'_>,
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
        let time = self.inner.time.as_slice();
        let qualified = crate::abort::run_interruptible_unregistered(py, |abort| {
            match analysis.analyze_with_abort(time, waveform, abort) {
                // Cancellation is the worker's business; every other outcome
                // is this waveform's own and stays a value error below.
                Err(FourierError::Aborted) => Err(rspice_core::SimulationError::Aborted),
                outcome => Ok(outcome),
            }
        })?;
        let result = qualified.map_err(|error| {
            crate::errors::value_error(format!("Fourier waveform could not be analyzed: {error}"))
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

    /// Typed source-authored `.FFT` products in netlist order.
    #[getter]
    fn fft_results(&self) -> Vec<PyTransientFftResult> {
        self.inner
            .fft_results
            .iter()
            .map(PyTransientFftResult::from)
            .collect()
    }

    /// Return one source-authored `.FFT` product by netlist order.
    fn fft(&self, index: usize) -> PyResult<PyTransientFftResult> {
        self.inner
            .fft_results
            .get(index)
            .map(PyTransientFftResult::from)
            .ok_or_else(|| {
                crate::errors::index_error(format!(
                    "FFT result index {index} out of range (0..{})",
                    self.inner.fft_results.len()
                ))
            })
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
        py: Python<'_>,
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
        self.fourier_of_waveform(py, &waveform, fundamental, num_harmonics)
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
        py: Python<'_>,
        element: &str,
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        let waveform = self.signal_waveform(&SignalSpec::Current {
            element: element.to_string(),
        })?;
        self.fourier_of_waveform(py, &waveform, fundamental, num_harmonics)
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
    /// `event_state` carries the pickle's version tag and the XSPICE digital
    /// and real event histories. It is the last parameter and defaults to
    /// `None` so a state written before it existed still reaches this method
    /// and is refused with a message that says what to do, instead of failing
    /// as an arity mismatch. Nothing this method builds is zero-filled: a
    /// state that does not describe a complete, aligned result is rejected.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (time, step_sizes, voltages, branch_currents, num_nodes, names, device_op_traces, store_traces, fft_state=None, event_state=None))]
    fn _unpickle(
        time: Vec<f64>,
        step_sizes: Vec<f64>,
        voltages: Vec<Vec<f64>>,
        branch_currents: Vec<Vec<f64>>,
        num_nodes: usize,
        names: (Vec<String>, Vec<String>),
        device_op_traces: Vec<(String, String, Vec<f64>)>,
        store_traces: Vec<(String, Vec<f64>)>,
        fft_state: Option<TransientFftPersistenceState>,
        event_state: Option<TransientEventPersistenceState>,
    ) -> PyResult<Self> {
        let (digital_traces, real_traces) =
            rebuild_transient_event_traces(event_state).map_err(crate::errors::value_error)?;
        let (node_names, branch_names) = names;
        let restored = TransientResult {
            time,
            step_sizes,
            voltages,
            branch_currents,
            num_nodes,
            node_names,
            branch_names,
            digital_traces,
            real_traces,
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
            fft_results: rebuild_transient_fft_results(fft_state)?,
        };
        validate_transient_state(&restored).map_err(crate::errors::value_error)?;
        Ok(Self::new(restored))
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
            TransientFftPersistenceState,
            TransientEventPersistenceState,
        ),
    )> {
        // Refuse to publish a state the unpickler would refuse to read: a
        // pickle is evidence, and a file that cannot be loaded back is worse
        // than a failed call.
        validate_transient_state(&self.inner).map_err(crate::errors::value_error)?;
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
                transient_fft_persistence_state(&self.inner.fft_results)?,
                transient_event_persistence_state(
                    &self.inner.digital_traces,
                    &self.inner.real_traces,
                ),
            ),
        ))
    }
}

/// Prove a transient result is a complete, internally aligned waveform set.
///
/// Every check names the channel it failed on. The point of the pass is that a
/// restored result can never be shorter, wider, or less aligned than the one
/// that was persisted: a caller reading `voltage_waveform("out")` must get the
/// samples the solver produced or an error, never a truncated array.
pub(crate) fn validate_transient_state(result: &TransientResult) -> Result<(), String> {
    let points = result.time.len();
    if result.step_sizes.len() != points {
        return Err(format!(
            "transient result has {} step sizes for {points} time points",
            result.step_sizes.len()
        ));
    }
    if result.time.iter().any(|time| !time.is_finite()) {
        return Err("transient result time points must all be finite".to_string());
    }
    if result.time.windows(2).any(|window| window[1] <= window[0]) {
        return Err("transient result time points must be strictly increasing".to_string());
    }
    if result.voltages.len() != result.num_nodes || result.node_names.len() != result.num_nodes {
        return Err(format!(
            "transient result declares {} nodes but has {} voltage channels and {} node names",
            result.num_nodes,
            result.voltages.len(),
            result.node_names.len()
        ));
    }
    if result.branch_currents.len() != result.branch_names.len() {
        return Err(format!(
            "transient result has {} branch-current channels but {} branch names",
            result.branch_currents.len(),
            result.branch_names.len()
        ));
    }
    // A deliberately unretained channel is empty; a retained one is aligned
    // with the time axis. Anything between the two is a truncated waveform.
    for (name, series) in result.node_names.iter().zip(&result.voltages) {
        if !series.is_empty() && series.len() != points {
            return Err(format!(
                "transient voltage waveform V({name}) has {} samples but time has {points}",
                series.len()
            ));
        }
    }
    for (name, series) in result.branch_names.iter().zip(&result.branch_currents) {
        if !series.is_empty() && series.len() != points {
            return Err(format!(
                "transient branch waveform I({name}) has {} samples but time has {points}",
                series.len()
            ));
        }
    }
    for trace in &result.device_op_traces {
        if trace.values.len() != points {
            return Err(format!(
                "transient device operating-point trace @{}[{}] has {} samples but time has {points}",
                trace.device_name,
                trace.parameter,
                trace.values.len()
            ));
        }
    }
    for trace in &result.store_traces {
        if trace.values.len() != points {
            return Err(format!(
                "transient store trace '{}' has {} samples but time has {points}",
                trace.name,
                trace.values.len()
            ));
        }
    }
    // Event traces record changes rather than one sample per accepted point,
    // so they are checked for monotone, in-window event times instead.
    let window = result.time.first().zip(result.time.last());
    for trace in &result.digital_traces {
        let times = trace.points.iter().map(|point| point.time);
        validate_event_times("digital", &trace.node_name, times, window)?;
    }
    for trace in &result.real_traces {
        let times = trace.points.iter().map(|point| point.time);
        validate_event_times("real", &trace.node_name, times, window)?;
    }
    Ok(())
}

fn validate_event_times(
    kind: &str,
    node: &str,
    times: impl Iterator<Item = f64>,
    window: Option<(&f64, &f64)>,
) -> Result<(), String> {
    let mut previous: Option<f64> = None;
    for time in times {
        if !time.is_finite() {
            return Err(format!(
                "transient {kind} event trace '{node}' has a non-finite event time"
            ));
        }
        if previous.is_some_and(|previous| time < previous) {
            return Err(format!(
                "transient {kind} event trace '{node}' has out-of-order event times"
            ));
        }
        if let Some((start, stop)) = window
            && (time < *start || time > *stop)
        {
            return Err(format!(
                "transient {kind} event trace '{node}' has an event at {time:.16e} s outside the \
                 result window [{start:.16e}, {stop:.16e}] s"
            ));
        }
        previous = Some(time);
    }
    Ok(())
}

#[cfg(test)]
mod structural_tests {
    use super::*;
    use rspice_core::engine::{DigitalTrace, DigitalTracePoint};
    use rspice_core::xspice::{DigitalState, DigitalStrength, DigitalValue};

    fn two_point_result() -> TransientResult {
        TransientResult {
            time: vec![0.0, 1.0e-9],
            step_sizes: vec![0.0, 1.0e-9],
            voltages: vec![vec![0.0, 1.0]],
            branch_currents: vec![vec![0.0, -1.0e-3]],
            num_nodes: 1,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            digital_traces: Vec::new(),
            real_traces: Vec::new(),
            device_op_traces: Vec::new(),
            store_traces: Vec::new(),
            fft_results: Vec::new(),
        }
    }

    #[test]
    fn a_complete_aligned_result_validates() {
        assert_eq!(validate_transient_state(&two_point_result()), Ok(()));
    }

    #[test]
    fn an_unretained_channel_stays_legal_but_a_truncated_one_does_not() {
        let mut unretained = two_point_result();
        unretained.voltages[0].clear();
        assert_eq!(validate_transient_state(&unretained), Ok(()));

        let mut truncated = two_point_result();
        truncated.voltages[0].pop();
        let message = validate_transient_state(&truncated)
            .expect_err("a half-length waveform is not a result");
        assert!(message.contains("V(out) has 1 samples"), "{message}");
    }

    #[test]
    fn a_channel_count_that_contradicts_the_node_count_is_refused() {
        let mut result = two_point_result();
        result.num_nodes = 2;
        let message = validate_transient_state(&result)
            .expect_err("a declared node without a channel is not a result");
        assert!(message.contains("declares 2 nodes"), "{message}");
    }

    #[test]
    fn a_non_monotone_time_axis_is_refused() {
        let mut result = two_point_result();
        result.time = vec![1.0e-9, 0.0];
        let message = validate_transient_state(&result).expect_err("time must advance");
        assert!(message.contains("strictly increasing"), "{message}");
    }

    #[test]
    fn an_event_outside_the_result_window_is_refused() {
        let mut result = two_point_result();
        result.digital_traces = vec![DigitalTrace {
            node_name: "clk".to_string(),
            points: vec![DigitalTracePoint {
                time: 5.0e-9,
                value: DigitalValue {
                    state: DigitalState::One,
                    strength: DigitalStrength::Strong,
                },
            }],
        }];
        let message = validate_transient_state(&result)
            .expect_err("an event after the last accepted sample is not in this result");
        assert!(message.contains("outside the result window"), "{message}");
    }

    #[test]
    fn a_store_trace_that_does_not_cover_the_run_is_refused() {
        let mut result = two_point_result();
        result.store_traces = vec![rspice_core::engine::TransientStoreTrace {
            name: "R1:power".to_string(),
            values: vec![1.0],
        }];
        let message = validate_transient_state(&result)
            .expect_err("a store trace is sampled at every accepted point");
        assert!(message.contains("'R1:power' has 1 samples"), "{message}");
    }
}
