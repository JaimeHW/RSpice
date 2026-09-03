//! Resumable transient checkpoints.
//!
//! `TransientCheckpoint` carries the netlist-fingerprinted solver state a
//! resumed run restarts from; the fingerprint is what stops a checkpoint being
//! replayed against a deck it was not produced from.

use super::*;

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
    /// Read a checkpoint from disk.
    ///
    /// Reading and validating a checkpoint is bounded but not fast — a large
    /// one carries the whole solver state — so it runs on the interruptible
    /// worker rather than holding the GIL and ignoring `KeyboardInterrupt`.
    #[staticmethod]
    fn load(py: Python<'_>, path: PathBuf) -> PyResult<Self> {
        let loaded = crate::abort::run_interruptible_unregistered(py, |abort| {
            // Cancellation belongs to the worker; a rejected or unreadable
            // checkpoint stays this call's own `ValueError`.
            match rspice_core::engine::TransientCheckpoint::load_with_abort(&path, abort) {
                Err(rspice_core::SimulationError::Aborted) => {
                    Err(rspice_core::SimulationError::Aborted)
                }
                outcome => Ok(outcome),
            }
        })?;
        loaded
            .map(Self::new)
            .map_err(|error| crate::errors::value_error(error.to_string()))
    }

    /// Write the checkpoint to disk, interruptibly.
    fn save(&self, py: Python<'_>, path: PathBuf) -> PyResult<()> {
        let saved = crate::abort::run_interruptible_unregistered(py, |abort| {
            match self.inner.save_with_abort(&path, abort) {
                Err(rspice_core::SimulationError::Aborted) => {
                    Err(rspice_core::SimulationError::Aborted)
                }
                outcome => Ok(outcome),
            }
        })?;
        saved.map_err(|error| crate::errors::value_error(error.to_string()))
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
