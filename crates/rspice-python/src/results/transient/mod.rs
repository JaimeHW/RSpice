//! Time-domain waveform results.
//!
//! `TransientResult` holds the full waveform set and the signal-spec resolution
//! that lets a caller address a node voltage, a differential pair, or a branch
//! current with the same probe grammar the deck uses.

use super::*;

/// Structural proof that a transient result is a complete, aligned waveform set.
mod structure;

pub(crate) use structure::{clip_transient_to_start, validate_transient_state};

/// Authored spelling of a node a caller addressed by name or by index.
///
/// A `.FOUR` spectrum records the operand it analyzed, and a caller who probed
/// by index gets that index back rather than a node name the deck never wrote.
fn node_label(node: &NodeIdentifier) -> String {
    match node {
        NodeIdentifier::Index(index) => index.to_string(),
        NodeIdentifier::Name(name) => name.clone(),
    }
}

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
    evidence: Option<DocumentEvidence<()>>,
}

impl CarriesDocumentEvidence for PyTransientResult {
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

impl PyTransientResult {
    pub fn new(inner: TransientResult) -> Self {
        Self {
            inner,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Tran,
                (),
            )),
        }
    }

    /// A trajectory rebuilt from pickled state, which carries no identity.
    fn restored(inner: TransientResult) -> Self {
        Self {
            inner,
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained trajectory.
    ///
    /// The document's `.FFT` child references are deliberately empty: this
    /// binding publishes each authored `.FFT` spectrum as its own
    /// `FftResult`, reachable from `fft_results`, rather than as a separate
    /// document with an identity of its own.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let (analysis, coordinate) = document::execution(&self.evidence, "transient")?;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_transient(analysis, &self.inner, None, Vec::new())
        })
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
        clip_transient_to_start(&mut inner, start_time)
            .map_err(crate::errors::SimulationError::new_err)?;
        Ok(Self::new(inner))
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
    /// The core output resolver owns this grammar, so a differential pair, a
    /// branch current, a device-lead current, an `@device[param]` observable,
    /// and a hierarchy spelling mean exactly what they mean on a `.PRINT` or
    /// `.FOUR` card. The binding layer only maps the typed failure onto an
    /// exception.
    pub(crate) fn probe_waveform(&self, spec: &str) -> PyResult<Vec<f64>> {
        rspice_core::analysis::evaluate_transient_probe_with_abort(
            None,
            &self.inner,
            spec,
            &rspice_core::abort_signal::NoAbort,
        )
        .map_err(|error| match error {
            rspice_core::SimulationError::RequestedSignalUnavailable(_) => {
                crate::errors::key_error(error.to_string())
            }
            other => crate::errors::value_error(other.to_string()),
        })
    }
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
        output: &str,
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
        Ok(PyFourierResult::from_core(&result).with_output(output))
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
        let output = match &reference {
            None => format!("V({})", node_label(&node)),
            Some(reference) => format!("V({},{})", node_label(&node), node_label(reference)),
        };
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
        self.fourier_of_waveform(py, &output, &waveform, fundamental, num_harmonics)
    }

    /// Fourier-analyze any SPICE output specification
    ///
    /// The `.FOUR` card accepts the full probe grammar, so this accessor does
    /// too: `V(out)`, `V(outp,outn)`, `I(V1)`, `@m1[id]`, or a bare node name,
    /// resolved by the same core resolver `.FOUR` uses.
    ///
    /// Args:
    ///     spec: Output specification string
    ///     fundamental: Fundamental frequency in Hz
    ///     num_harmonics: Number of harmonics to compute (default 9)
    ///
    /// Raises:
    ///     ValueError: If the specification or Fourier configuration is invalid
    ///     KeyError: If the result does not supply that signal
    ///
    /// Example:
    ///     >>> four = tran.fourier_of("@d1[id]", fundamental=1e3)
    #[pyo3(signature = (spec, fundamental, num_harmonics=9))]
    fn fourier_of(
        &self,
        py: Python<'_>,
        spec: &str,
        fundamental: f64,
        num_harmonics: usize,
    ) -> PyResult<PyFourierResult> {
        let waveform = self.probe_waveform(spec)?;
        self.fourier_of_waveform(py, spec, &waveform, fundamental, num_harmonics)
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
        let waveform = self.probe_waveform(&format!("I({element})"))?;
        self.fourier_of_waveform(
            py,
            &format!("I({element})"),
            &waveform,
            fundamental,
            num_harmonics,
        )
    }

    /// Evaluate any SPICE output specification against this result
    ///
    /// Accepts `V(out)`, `V(outp,outn)`, `I(V1)`, `@d1[id]`, or a bare node
    /// name — the same probe grammar `.FOUR` and `.PRINT` use, resolved by the
    /// same core resolver.
    ///
    /// Args:
    ///     spec: Output specification string
    ///
    /// Returns:
    ///     numpy.ndarray: Signal values at each time point
    ///
    /// Raises:
    ///     ValueError: If the specification is malformed
    ///     KeyError: If the result does not supply that signal
    ///
    /// Example:
    ///     >>> vdiff = tran.signal("V(outp,outn)")
    ///     >>> isupply = tran.signal("I(V1)")
    ///     >>> idrain = tran.signal("@m1[id]")
    fn signal<'py>(&self, py: Python<'py>, spec: &str) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.probe_waveform(spec)?.to_pyarray(py))
    }

    /// Project this result onto a deck's authored output contract
    ///
    /// Returns the columns the deck's `.SAVE`, `.PROBE`, `.PRINT TRAN` and
    /// `.PLOT TRAN` cards select, in card order, each with its per-sample
    /// validity. Whole-result access is unaffected: this is the authored view,
    /// not a replacement for it.
    ///
    /// A deck with no output directive selects everything.
    ///
    /// Args:
    ///     netlist: The parsed deck whose output cards to apply
    ///
    /// Returns:
    ///     list[ProjectedSignal]: Selected columns in authored order
    ///
    /// Raises:
    ///     RequestedSignalUnavailableError: If an authored symbol is absent
    ///
    /// Example:
    ///     >>> for signal in tran.saved_signals(netlist):
    ///     ...     print(signal.name, signal.values[-1])
    fn saved_signals(
        &self,
        netlist: &crate::netlist::PyNetlist,
    ) -> PyResult<Vec<crate::results::PyProjectedSignal>> {
        let inventory = rspice_core::execution::transient_projection_signals(&self.inner)
            .map_err(|error| crate::errors::value_error(error.to_string()))?;
        let projection = rspice_core::execution::SignalProjection::from_netlist(&netlist.inner)
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        let ordered = projection
            .ordered_transient_columns(
                &netlist.inner,
                &self.inner,
                netlist.resource_limits,
                &rspice_core::abort_signal::NoAbort,
            )
            .map_err(crate::errors::simulation_error_to_pyerr)?;
        crate::results::projection::project_real(
            &netlist.inner,
            rspice_core::execution::AnalysisResultKind::Transient,
            "TRAN",
            &self.inner.time,
            inventory,
            rspice_core::analysis::measure_signals::transient_signal_map(&self.inner),
            ordered,
        )
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
            // The pickled event state is version 1 and carries no bus table,
            // so a restored result declares none rather than inventing one.
            digital_buses: Vec::new(),
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
        Ok(Self::restored(restored))
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
