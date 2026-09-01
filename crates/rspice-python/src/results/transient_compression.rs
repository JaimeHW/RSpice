//! Long-run transient controls: compression and resumable checkpoints.
//!
//! `CompressedTransientResult` carries an error-bounded reduction of a waveform
//! set, so a multi-hour run stays addressable without holding every timepoint.
//! `TransientCheckpoint` carries the netlist-fingerprinted state a resumed run
//! restarts from; the fingerprint is what stops a checkpoint being replayed
//! against a deck it was not produced from.

use super::*;

/// Memory-decimated transient voltage waveforms with bounded interpolation error.
#[pyclass(name = "CompressedTransientResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyCompressedTransientResult {
    inner: rspice_core::engine::TransientResultCompressed,
}

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
}

#[pymethods]
impl PyCompressedTransientResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
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

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.node_index(&node)? {
            Some(index) => self.inner.voltages.get(index).cloned().ok_or_else(|| {
                crate::errors::value_error("malformed compressed transient voltage matrix")
            })?,
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
            Some(index) => self.inner.interpolate(index, time).ok_or_else(|| {
                crate::errors::value_error("compressed transient waveform cannot be interpolated")
            }),
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
                .resample(index, num_points)
                .map(|(time, values)| (time.to_pyarray(py), values.to_pyarray(py)))
                .ok_or_else(|| {
                    crate::errors::value_error("compressed waveform cannot be resampled")
                }),
            None => {
                let start = self.inner.time.first().copied().unwrap_or(0.0);
                let stop = self.inner.time.last().copied().unwrap_or(start);
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
    /// Device store traces have no accessor on this class, so they are not
    /// carried; every quantity a caller can read back is.
    #[staticmethod]
    fn _unpickle(
        time: Vec<f64>,
        voltages: Vec<Vec<f64>>,
        num_nodes: usize,
        node_names: Vec<String>,
        compression_ratio: f64,
        input_points: usize,
    ) -> Self {
        Self::new(rspice_core::engine::TransientResultCompressed {
            time,
            voltages,
            num_nodes,
            node_names,
            store_traces: Vec::new(),
            fft_results: Vec::new(),
            compression_ratio,
            input_points,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (Vec<f64>, Vec<Vec<f64>>, usize, Vec<String>, f64, usize),
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
