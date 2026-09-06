//! The decimated transient container and its accessors.
//!
//! `CompressedTransientResult` carries an error-bounded reduction of the full
//! analog waveform inventory, so a multi-hour run stays addressable without
//! holding every timepoint. Every accessor that could hand back a number the
//! producing run never recorded refuses instead, so a decimated waveform is
//! still evidence rather than an interpolation of a placeholder.

use super::*;

/// Memory-decimated transient analog waveforms with bounded interpolation error.
#[pyclass(name = "CompressedTransientResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCompressedTransientResult {
    inner: rspice_core::engine::TransientResultCompressed,
    evidence: Option<DocumentEvidence<()>>,
}

impl CarriesDocumentEvidence for PyCompressedTransientResult {
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

impl PyCompressedTransientResult {
    pub fn new(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self {
            inner,
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Tran,
                (),
            )),
        }
    }

    /// A compressed trajectory rebuilt from pickled state, which carries no
    /// analysis identity.
    fn restored(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self {
            inner,
            evidence: None,
        }
    }

    /// The shared result document, projected from the retained samples.
    ///
    /// The document carries the compression certificate the run produced, so
    /// a reader always knows which grid the published history is on.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let (analysis, coordinate) = document::execution(&self.evidence, "compressed transient")?;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_compressed_transient(analysis, &self.inner, Vec::new())
        })
    }

    fn node_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        let num_nodes = self.inner.num_nodes();
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) if *index <= num_nodes => Ok(Some(index - 1)),
            NodeIdentifier::Index(index) => Err(invalid_node_index_error(*index, num_nodes).into()),
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names()
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }

    /// Dense retained samples of one channel, refusing to invent a number for
    /// a sample the producing run recorded as absent.
    fn dense_channel_values(
        &self,
        channel: &rspice_core::engine::TransientCompressedChannel,
        label: &str,
    ) -> PyResult<Vec<f64>> {
        if channel.availability != rspice_core::engine::TransientChannelAvailability::Available {
            return Err(crate::errors::key_error(format!(
                "{label} was not recorded; add it to .SAVE"
            )));
        }
        channel.dense_values().ok_or_else(|| {
            crate::errors::value_error(format!(
                "{label} has samples the producing run did not record as numbers; read `channel_absence` for the reason at each retained point"
            ))
        })
    }

    fn channel(
        &self,
        canonical_name: &str,
    ) -> PyResult<&rspice_core::engine::TransientCompressedChannel> {
        self.inner.channel_named(canonical_name).ok_or_else(|| {
            crate::errors::key_error(format!(
                "unknown compressed transient channel '{canonical_name}'"
            ))
        })
    }

    fn branch_current_values(&self, name: &str) -> PyResult<Vec<f64>> {
        let channel = self
            .inner
            .branch_current_channel(name)
            .ok_or_else(|| PyErr::from(unknown_branch_name_error(name)))?;
        self.dense_channel_values(channel, &format!("branch-current waveform '{name}'"))
    }
}

#[pymethods]
impl PyCompressedTransientResult {
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

    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    /// Exact accepted integration intervals at the retained points.
    #[getter]
    fn step_sizes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.step_sizes.to_pyarray(py)
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names()
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    #[getter]
    fn input_points(&self) -> usize {
        self.inner.input_points
    }

    #[getter]
    fn compression_ratio(&self) -> f64 {
        self.inner.compression_ratio
    }

    /// Version of the persisted compression evidence contract.
    #[getter]
    fn compression_report_version(&self) -> u32 {
        self.inner.compression_report.schema_version
    }

    /// Stable identifier of the compression algorithm.
    #[getter]
    fn compression_algorithm(&self) -> &'static str {
        self.inner.compression_report.algorithm.as_str()
    }

    /// Stable identifier of the input-sample domain used for verification.
    #[getter]
    fn compression_sample_domain(&self) -> &'static str {
        self.inner.compression_report.sample_domain.as_str()
    }

    /// Whether waveform decimation was enabled for this result.
    #[getter]
    fn compression_enabled(&self) -> bool {
        self.inner.compression_report.applied_policy.enabled
    }

    /// Applied absolute interpolation tolerance in each signal's native unit.
    #[getter]
    fn compression_absolute_tolerance(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .absolute_tolerance
    }

    /// Applied relative interpolation tolerance.
    #[getter]
    fn compression_relative_tolerance(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .relative_tolerance
    }

    /// Applied maximum interval between retained points in seconds.
    #[getter]
    fn compression_maximum_interval(&self) -> f64 {
        self.inner
            .compression_report
            .applied_policy
            .maximum_retained_interval
    }

    /// Stable kind of the signal with the highest tolerance utilization.
    #[getter]
    fn worst_compression_error_signal_kind(&self) -> Option<&'static str> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.signal.kind.as_str())
    }

    /// Canonical identity of the signal with the highest tolerance utilization.
    #[getter]
    fn worst_compression_error_signal(&self) -> Option<String> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.signal.canonical_name.clone())
    }

    /// Original accepted-grid index of the worst reconstructed sample.
    #[getter]
    fn worst_compression_error_input_sample_index(&self) -> Option<usize> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.input_sample_index)
    }

    /// Source-sample time of the worst final-grid reconstruction error.
    #[getter]
    fn worst_compression_error_time(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.time)
    }

    /// Original value at the worst-utilization observation.
    #[getter]
    fn worst_compression_actual_value(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.actual_value)
    }

    /// Absolute reconstruction error at the worst-utilization observation.
    #[getter]
    fn worst_compression_absolute_error(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.absolute_error)
    }

    /// Relative error at the worst observation; absent when the actual value
    /// is zero or the ratio cannot be represented finitely.
    #[getter]
    fn worst_compression_relative_error(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .and_then(|observation| observation.relative_error)
    }

    /// Allowed absolute-plus-relative tolerance at the worst observation.
    #[getter]
    fn worst_compression_allowed_tolerance(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.allowed_tolerance)
    }

    /// Fraction of the declared tolerance consumed at the worst observation.
    #[getter]
    fn worst_compression_tolerance_utilization(&self) -> Option<f64> {
        self.inner
            .compression_report
            .worst_observed
            .as_ref()
            .map(|observation| observation.tolerance_utilization)
    }

    /// Canonical branch names aligned with retained branch-current waveforms.
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names()
    }

    /// Canonical names of every descriptor-keyed channel in this container.
    #[getter]
    fn channel_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .map(|channel| channel.descriptor.canonical_name().to_string())
            .collect()
    }

    /// Physical unit of one channel, by canonical name.
    fn channel_unit(&self, name: &str) -> PyResult<String> {
        self.channel(name)
            .map(|channel| channel.descriptor.unit().as_str().to_string())
    }

    /// Whether a channel was retained, and if not, why.
    fn channel_availability(&self, name: &str) -> PyResult<&'static str> {
        self.channel(name)
            .map(|channel| channel.availability.as_str())
    }

    /// Per-retained-point absence reasons for one channel. An entry is `None`
    /// where the channel has a number and a reason string where it does not,
    /// so a caller never has to read a placeholder as data.
    fn channel_absence(&self, name: &str) -> PyResult<Vec<Option<&'static str>>> {
        self.channel(name).map(|channel| {
            channel
                .samples
                .iter()
                .map(|sample| sample.absence().map(|reason| reason.as_str()))
                .collect()
        })
    }

    /// XSPICE digital event node names carried exactly through compression.
    #[getter]
    fn digital_trace_names(&self) -> Vec<String> {
        self.inner
            .digital_traces
            .iter()
            .map(|trace| trace.node_name.clone())
            .collect()
    }

    /// Committed `(time, state, strength)` events for one digital node.
    fn digital_trace(&self, name: &str) -> PyResult<Vec<(f64, &'static str, &'static str)>> {
        self.inner
            .digital_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| {
                trace
                    .points
                    .iter()
                    .map(|point| {
                        (
                            point.time,
                            rspice_core::engine::digital_state_tag(point.value.state),
                            rspice_core::engine::digital_strength_tag(point.value.strength),
                        )
                    })
                    .collect()
            })
            .ok_or_else(|| {
                crate::errors::key_error(format!("unknown XSPICE digital event node '{name}'"))
            })
    }

    /// The digital buses this result declares, in declaration order.
    ///
    /// Compression never touches a declaration or an event history, so this is
    /// the table the producing run published, and the members it names are the
    /// same conductors `digital_trace_names` lists.
    fn digital_buses(&self) -> Vec<PyDigitalBus> {
        digital_bus_list(&self.inner.digital_buses)
    }

    /// Every event of one declared bus, as the whole word at each of them.
    ///
    /// The rows are `TransientResult.bus_events`'s exactly: `bits` carries the
    /// `0..=12` event code of each member declared MSB first, `value` the same
    /// word in VCD's four states, and both come from the one reassembly
    /// `rspice_core` performs for every route.
    ///
    /// Raises:
    ///     KeyError: If this result declares no bus by that name
    ///     ValueError: If the declaration names a member this result recorded
    ///         no history for, or the history is past the row limit this
    ///         accessor materializes
    fn bus_events(&self, py: Python<'_>, name: &str) -> PyResult<Vec<PyBusEvent>> {
        bus_event_rows(
            py,
            &self.inner.digital_buses,
            &self.inner.digital_traces,
            name,
        )
    }

    /// XSPICE real event node names carried exactly through compression.
    #[getter]
    fn real_trace_names(&self) -> Vec<String> {
        self.inner
            .real_traces
            .iter()
            .map(|trace| trace.node_name.clone())
            .collect()
    }

    /// Committed `(time, value)` events for one real event node.
    fn real_trace(&self, name: &str) -> PyResult<Vec<(f64, f64)>> {
        self.inner
            .real_traces
            .iter()
            .find(|trace| trace.node_name.eq_ignore_ascii_case(name))
            .map(|trace| {
                trace
                    .points
                    .iter()
                    .map(|point| (point.time, point.value))
                    .collect()
            })
            .ok_or_else(|| {
                crate::errors::key_error(format!("unknown XSPICE real event node '{name}'"))
            })
    }

    /// `.FOUR` spectra computed on the exact accepted trajectory.
    #[getter]
    fn fourier_results(&self) -> Vec<PyFourierResult> {
        self.inner
            .post_results
            .fourier
            .iter()
            .map(|entry| {
                PyFourierResult::from_core_with_provenance(
                    &entry.spectrum,
                    entry.output.clone(),
                    format!("four-{:03}", entry.card_index + 1),
                    None,
                    None,
                )
            })
            .collect()
    }

    /// Transient `.MEASURE` results computed on the exact accepted trajectory.
    #[getter]
    fn measurements(&self) -> Vec<PyMeasurement> {
        self.inner
            .post_results
            .measurements
            .iter()
            .map(|result| PyMeasurement::from_core(result, "TRAN"))
            .collect()
    }

    /// Stable tag of the authored analysis card this result came from.
    #[getter]
    fn analysis_id(&self) -> Option<String> {
        self.inner
            .identity
            .analysis
            .as_ref()
            .map(|analysis| analysis.tag())
    }

    /// Label of the shared-deck coordinate this result was produced at.
    #[getter]
    fn coordinate_label(&self) -> Option<String> {
        self.inner
            .identity
            .coordinate
            .as_ref()
            .map(|coordinate| coordinate.label.clone())
    }

    /// Lower-case hexadecimal topology fingerprint of the solved circuit.
    #[getter]
    fn topology_fingerprint(&self) -> Option<String> {
        self.inner.identity.topology_fingerprint.map(|bytes| {
            bytes
                .iter()
                .map(|byte| format!("{byte:02x}"))
                .collect::<String>()
        })
    }

    fn branch_current_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        Ok(self.branch_current_values(name)?.to_pyarray(py))
    }

    fn branch_current_at(&self, name: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.branch_current_values(name)?;
        self.inner
            .interpolate_branch_current_named(name, time)
            .ok_or_else(|| {
                crate::errors::value_error(format!(
                    "compressed branch-current waveform '{name}' cannot be interpolated"
                ))
            })
    }

    /// Device operating-point traces requested with `.SAVE @device[param]`.
    #[getter]
    fn device_parameter_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                rspice_core::engine::TransientChannelRole::DeviceObservable {
                    device,
                    parameter,
                } => Some(format!("@{device}[{parameter}]")),
                _ => None,
            })
            .collect()
    }

    fn device_parameter_waveform<'py>(
        &self,
        py: Python<'py>,
        device: &str,
        parameter: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let channel = self.inner.device_op_channel(device, parameter).ok_or_else(|| {
            crate::errors::key_error(format!(
                "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
            ))
        })?;
        Ok(self
            .dense_channel_values(
                channel,
                &format!("device operating-point trace '@{device}[{parameter}]'"),
            )?
            .to_pyarray(py))
    }

    fn device_parameter_at(&self, device: &str, parameter: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.inner
            .interpolate_device_op_named(device, parameter, time)
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
    }

    /// Canonical typed device-store trace names.
    #[getter]
    fn store_names(&self) -> Vec<String> {
        self.inner
            .channels
            .iter()
            .filter_map(|channel| match channel.descriptor.role() {
                rspice_core::engine::TransientChannelRole::DeviceStore { store } => {
                    Some(store.clone())
                }
                _ => None,
            })
            .collect()
    }

    fn store_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let channel = self.inner.store_channel(name).ok_or_else(|| {
            crate::errors::key_error(format!("unknown device-store trace '{name}'"))
        })?;
        Ok(self
            .dense_channel_values(channel, &format!("device-store trace '{name}'"))?
            .to_pyarray(py))
    }

    fn store_at(&self, name: &str, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        self.inner
            .interpolate_store_named(name, time)
            .ok_or_else(|| crate::errors::key_error(format!("unknown device-store trace '{name}'")))
    }

    /// Typed `.FFT` products computed before waveform decimation.
    #[getter]
    fn fft_results(&self) -> Vec<PyTransientFftResult> {
        self.inner
            .post_results
            .fft
            .iter()
            .map(PyTransientFftResult::from)
            .collect()
    }

    fn fft(&self, index: usize) -> PyResult<PyTransientFftResult> {
        self.inner
            .post_results
            .fft
            .get(index)
            .map(PyTransientFftResult::from)
            .ok_or_else(|| {
                crate::errors::index_error(format!(
                    "FFT result index {index} out of range (0..{})",
                    self.inner.post_results.fft.len()
                ))
            })
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.node_index(&node)? {
            Some(index) => {
                let channel = self.inner.node_voltage_channel(index).ok_or_else(|| {
                    crate::errors::value_error("malformed compressed transient voltage inventory")
                })?;
                self.dense_channel_values(channel, "requested node voltage")?
            }
            None => vec![0.0; self.inner.time.len()],
        };
        if values.len() != self.inner.time.len() {
            return Err(crate::errors::value_error(
                "malformed compressed transient waveform length",
            ));
        }
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        match self.node_index(&node)? {
            Some(index) => {
                let channel = self.inner.node_voltage_channel(index).ok_or_else(|| {
                    crate::errors::value_error("malformed compressed transient voltage inventory")
                })?;
                if channel.availability
                    != rspice_core::engine::TransientChannelAvailability::Available
                {
                    return Err(crate::errors::key_error(
                        "requested node voltage was not recorded; add it to .SAVE",
                    ));
                }
                self.inner.interpolate(index, time).ok_or_else(|| {
                    crate::errors::value_error(
                        "compressed transient waveform cannot be interpolated",
                    )
                })
            }
            None => Ok(0.0),
        }
    }

    #[allow(clippy::type_complexity)]
    fn resample<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
        num_points: usize,
    ) -> PyResult<(Bound<'py, PyArray1<f64>>, Bound<'py, PyArray1<f64>>)> {
        if num_points < 2 {
            return Err(crate::errors::value_error("num_points must be at least 2"));
        }
        match self.node_index(&node)? {
            Some(index) => self
                .inner
                .node_voltage_channel(index)
                .filter(|channel| {
                    channel.availability
                        == rspice_core::engine::TransientChannelAvailability::Available
                        || self.inner.time.is_empty()
                })
                .ok_or_else(|| {
                    crate::errors::key_error(
                        "requested node voltage was not recorded; add it to .SAVE",
                    )
                })
                .and_then(|_| {
                    self.inner.resample(index, num_points).ok_or_else(|| {
                        crate::errors::value_error("compressed waveform cannot be resampled")
                    })
                })
                .map(|(time, values)| (time.to_pyarray(py), values.to_pyarray(py))),
            None => {
                let Some((&start, &stop)) = self.inner.time.first().zip(self.inner.time.last())
                else {
                    return Err(crate::errors::value_error(
                        "empty compressed transient has no time domain to resample",
                    ));
                };
                let step = (stop - start) / (num_points - 1) as f64;
                let time = (0..num_points)
                    .map(|index| start + index as f64 * step)
                    .collect::<Vec<_>>();
                Ok((time.to_pyarray(py), vec![0.0; num_points].to_pyarray(py)))
            }
        }
    }

    fn __repr__(&self) -> String {
        format!(
            "CompressedTransientResult(nodes={}, stored_points={}, input_points={}, ratio={:.2}x)",
            self.inner.num_nodes(),
            self.inner.time.len(),
            self.inner.input_points,
            self.inner.compression_ratio
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    #[staticmethod]
    #[pyo3(signature = (time, compression_ratio, input_points, fft_state=None, analog_state=None, compression_state=None))]
    fn _unpickle(
        time: Vec<f64>,
        compression_ratio: f64,
        input_points: usize,
        fft_state: Option<TransientFftPersistenceState>,
        analog_state: Option<VersionedCompressedTransientAnalogState>,
        compression_state: Option<CompressionReportPersistenceState>,
    ) -> PyResult<Self> {
        rebuild_compressed_transient(
            time,
            compression_ratio,
            input_points,
            fft_state,
            analog_state,
            compression_state,
        )
        .map(Self::restored)
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<f64>,
            f64,
            usize,
            TransientFftPersistenceState,
            CompressedTransientAnalogState,
            CompressionReportPersistenceState,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.time.clone(),
                self.inner.compression_ratio,
                self.inner.input_points,
                transient_fft_persistence_state(&self.inner.post_results.fft)?,
                compressed_transient_analog_state(&self.inner),
                compression_report_persistence_state(&self.inner.compression_report),
            ),
        ))
    }
}
