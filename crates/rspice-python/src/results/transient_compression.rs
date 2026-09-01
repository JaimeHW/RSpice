//! Long-run transient controls: compression and resumable checkpoints.
//!
//! `CompressedTransientResult` carries an error-bounded reduction of the full
//! analog waveform inventory, so a multi-hour run stays addressable without
//! holding every timepoint.
//! `TransientCheckpoint` carries the netlist-fingerprinted state a resumed run
//! restarts from; the fingerprint is what stops a checkpoint being replayed
//! against a deck it was not produced from.

use super::*;

/// Memory-decimated transient analog waveforms with bounded interpolation error.
#[pyclass(name = "CompressedTransientResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCompressedTransientResult {
    inner: rspice_core::engine::TransientResultCompressed,
}

const COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION: usize = 1;
type CompressedTransientAnalogState = (
    usize,
    Vec<f64>,
    Vec<Vec<f64>>,
    Vec<String>,
    Vec<(String, String, Vec<f64>)>,
    Vec<(String, Vec<f64>)>,
);

impl PyCompressedTransientResult {
    pub fn new(inner: rspice_core::engine::TransientResultCompressed) -> Self {
        Self { inner }
    }

    fn node_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) if *index <= self.inner.num_nodes => Ok(Some(index - 1)),
            NodeIdentifier::Index(index) => {
                Err(invalid_node_index_error(*index, self.inner.num_nodes).into())
            }
            NodeIdentifier::Name(name) if is_ground_name(name) => Ok(None),
            NodeIdentifier::Name(name) => self
                .inner
                .node_names
                .iter()
                .position(|candidate| candidate.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| unknown_node_name_error(name).into()),
        }
    }

    fn branch_current_values(&self, name: &str) -> PyResult<&[f64]> {
        let index = self
            .inner
            .branch_names
            .iter()
            .position(|candidate| candidate.eq_ignore_ascii_case(name))
            .ok_or_else(|| PyErr::from(unknown_branch_name_error(name)))?;
        let values = self.inner.branch_currents.get(index).ok_or_else(|| {
            crate::errors::value_error("malformed compressed transient branch inventory")
        })?;
        if values.is_empty() && !self.inner.time.is_empty() {
            return Err(crate::errors::key_error(format!(
                "branch-current waveform '{name}' was not recorded; add it to .SAVE"
            )));
        }
        if values.len() != self.inner.time.len() {
            return Err(crate::errors::value_error(format!(
                "malformed compressed transient branch-current waveform '{name}'"
            )));
        }
        Ok(values)
    }
}

#[pymethods]
impl PyCompressedTransientResult {
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
        self.inner.node_names.clone()
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.num_nodes
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

    /// Canonical branch names aligned with retained branch-current waveforms.
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.inner.branch_names.clone()
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
        let values = self.branch_current_values(name)?;
        self.inner
            .interpolate_branch_current_named(name, time)
            .filter(|_| !values.is_empty())
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
        self.inner
            .try_device_op_waveform_named(device, parameter)
            .map(|waveform| waveform.to_pyarray(py))
            .ok_or_else(|| {
                crate::errors::key_error(format!(
                    "device operating-point trace '@{device}[{parameter}]' was not recorded; add it to .SAVE"
                ))
            })
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
            .store_traces
            .iter()
            .map(|trace| trace.name.clone())
            .collect()
    }

    fn store_waveform<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        self.inner
            .try_store_waveform_named(name)
            .map(|waveform| waveform.to_pyarray(py))
            .ok_or_else(|| crate::errors::key_error(format!("unknown device-store trace '{name}'")))
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
            .fft_results
            .iter()
            .map(PyTransientFftResult::from)
            .collect()
    }

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

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.node_index(&node)? {
            Some(index) => {
                let values = self.inner.voltages.get(index).cloned().ok_or_else(|| {
                    crate::errors::value_error("malformed compressed transient voltage matrix")
                })?;
                if values.is_empty() && !self.inner.time.is_empty() {
                    return Err(crate::errors::key_error(
                        "requested node voltage was not recorded; add it to .SAVE",
                    ));
                }
                values
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
                if self.inner.voltages.get(index).is_some_and(Vec::is_empty)
                    && !self.inner.time.is_empty()
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
                .voltages
                .get(index)
                .filter(|values| !values.is_empty() || self.inner.time.is_empty())
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
            self.inner.num_nodes,
            self.inner.time.len(),
            self.inner.input_points,
            self.inner.compression_ratio
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    #[staticmethod]
    #[pyo3(signature = (time, voltages, num_nodes, node_names, compression_ratio, input_points, fft_state=None, analog_state=None))]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(
        time: Vec<f64>,
        voltages: Vec<Vec<f64>>,
        num_nodes: usize,
        node_names: Vec<String>,
        compression_ratio: f64,
        input_points: usize,
        fft_state: Option<TransientFftPersistenceState>,
        analog_state: Option<CompressedTransientAnalogState>,
    ) -> PyResult<Self> {
        let Some((
            version,
            step_sizes,
            branch_currents,
            branch_names,
            device_op_traces,
            store_traces,
        )) = analog_state
        else {
            return Err(crate::errors::value_error(
                "legacy compressed-transient pickle predates lossless analog inventory persistence; rerun the analysis",
            ));
        };
        if version != COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION {
            return Err(crate::errors::value_error(format!(
                "unsupported compressed-transient analog pickle state version {version}"
            )));
        }
        let inner = rspice_core::engine::TransientResultCompressed {
            time,
            step_sizes,
            voltages,
            branch_currents,
            num_nodes,
            node_names,
            branch_names,
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
            compression_ratio,
            input_points,
        };
        inner.validate().map_err(crate::errors::value_error)?;
        Ok(Self::new(inner))
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            Vec<f64>,
            Vec<Vec<f64>>,
            usize,
            Vec<String>,
            f64,
            usize,
            TransientFftPersistenceState,
            CompressedTransientAnalogState,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.time.clone(),
                self.inner.voltages.clone(),
                self.inner.num_nodes,
                self.inner.node_names.clone(),
                self.inner.compression_ratio,
                self.inner.input_points,
                transient_fft_persistence_state(&self.inner.fft_results)?,
                (
                    COMPRESSED_TRANSIENT_ANALOG_STATE_VERSION,
                    self.inner.step_sizes.clone(),
                    self.inner.branch_currents.clone(),
                    self.inner.branch_names.clone(),
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
            ),
        ))
    }
}

/// Versioned transient integrator state for fingerprint-validated continuation.
#[pyclass(name = "TransientCheckpoint", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientCheckpoint {
    pub(crate) inner: rspice_core::engine::TransientCheckpoint,
}

impl PyTransientCheckpoint {
    pub fn new(inner: rspice_core::engine::TransientCheckpoint) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl PyTransientCheckpoint {
    #[staticmethod]
    fn load(path: PathBuf) -> PyResult<Self> {
        rspice_core::engine::TransientCheckpoint::load(&path)
            .map(Self::new)
            .map_err(crate::errors::value_error)
    }

    fn save(&self, path: PathBuf) -> PyResult<()> {
        self.inner.save(&path).map_err(crate::errors::value_error)
    }

    #[getter]
    fn time(&self) -> f64 {
        self.inner.time
    }

    #[getter]
    fn solution<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.solution.to_pyarray(py)
    }

    #[getter]
    fn netlist_fingerprint(&self) -> u64 {
        self.inner.netlist_fingerprint
    }

    fn __repr__(&self) -> String {
        format!(
            "TransientCheckpoint(time={:.6e}, state_size={}, fingerprint={:#018x})",
            self.inner.time,
            self.inner.solution.len(),
            self.inner.netlist_fingerprint
        )
    }
}
