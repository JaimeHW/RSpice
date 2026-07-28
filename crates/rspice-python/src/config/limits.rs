//! Resource ceilings applied across parsing and execution.
//!
//! One policy bounds netlist and dependency ingestion, hierarchy and circuit
//! growth, analysis and result sizes, batch runs, external data, and shared
//! caches. Having a single object carry all of them is what makes a limit
//! auditable: a caller can read one value and know what the run may consume.

use super::*;

/// Resource ceilings for untrusted, interactive, and batch workloads.
///
/// The same object can be supplied to `Netlist.parse*` and
/// `SimulationConfig`, ensuring parsing and execution use one policy.
#[pyclass(name = "ResourceLimits", module = "rspice", from_py_object, eq)]
#[derive(Clone, PartialEq, Eq)]
pub struct PyResourceLimits {
    #[pyo3(get, set)]
    pub max_netlist_bytes: usize,
    #[pyo3(get, set)]
    pub max_netlist_lines: usize,
    #[pyo3(get, set)]
    pub max_expanded_source_bytes: usize,
    #[pyo3(get, set)]
    pub max_dependency_source_bytes: usize,
    #[pyo3(get, set)]
    pub max_external_data_bytes: usize,
    #[pyo3(get, set)]
    pub max_external_data_values: usize,
    #[pyo3(get, set)]
    pub max_shared_cache_bytes: usize,
    #[pyo3(get, set)]
    pub max_include_depth: usize,
    #[pyo3(get, set)]
    pub max_hierarchy_depth: usize,
    #[pyo3(get, set)]
    pub max_flattened_elements: usize,
    #[pyo3(get, set)]
    pub max_circuit_nodes: usize,
    #[pyo3(get, set)]
    pub max_matrix_unknowns: usize,
    #[pyo3(get, set)]
    pub max_analysis_points: usize,
    #[pyo3(get, set)]
    pub max_result_values: usize,
    #[pyo3(get, set)]
    pub max_parallel_workers: usize,
    #[pyo3(get, set)]
    pub max_batch_runs: usize,
}

impl PyResourceLimits {
    pub(crate) fn from_core(limits: ResourceLimits) -> Self {
        Self {
            max_netlist_bytes: limits.max_netlist_bytes,
            max_netlist_lines: limits.max_netlist_lines,
            max_expanded_source_bytes: limits.max_expanded_source_bytes,
            max_dependency_source_bytes: limits.max_dependency_source_bytes,
            max_external_data_bytes: limits.max_external_data_bytes,
            max_external_data_values: limits.max_external_data_values,
            max_shared_cache_bytes: limits.max_shared_cache_bytes,
            max_include_depth: limits.max_include_depth,
            max_hierarchy_depth: limits.max_hierarchy_depth,
            max_flattened_elements: limits.max_flattened_elements,
            max_circuit_nodes: limits.max_circuit_nodes,
            max_matrix_unknowns: limits.max_matrix_unknowns,
            max_analysis_points: limits.max_analysis_points,
            max_result_values: limits.max_result_values,
            max_parallel_workers: limits.max_parallel_workers,
            max_batch_runs: limits.max_batch_runs,
        }
    }

    pub(crate) fn to_core(&self) -> ResourceLimits {
        let mut limits = ResourceLimits::default();
        limits.max_netlist_bytes = self.max_netlist_bytes;
        limits.max_netlist_lines = self.max_netlist_lines;
        limits.max_expanded_source_bytes = self.max_expanded_source_bytes;
        limits.max_dependency_source_bytes = self.max_dependency_source_bytes;
        limits.max_external_data_bytes = self.max_external_data_bytes;
        limits.max_external_data_values = self.max_external_data_values;
        limits.max_shared_cache_bytes = self.max_shared_cache_bytes;
        limits.max_include_depth = self.max_include_depth;
        limits.max_hierarchy_depth = self.max_hierarchy_depth;
        limits.max_flattened_elements = self.max_flattened_elements;
        limits.max_circuit_nodes = self.max_circuit_nodes;
        limits.max_matrix_unknowns = self.max_matrix_unknowns;
        limits.max_analysis_points = self.max_analysis_points;
        limits.max_result_values = self.max_result_values;
        limits.max_parallel_workers = self.max_parallel_workers;
        limits.max_batch_runs = self.max_batch_runs;
        limits
    }
}

#[pymethods]
impl PyResourceLimits {
    /// Construct a policy from production-safe defaults, overriding any
    /// supplied keyword fields. A zero ceiling intentionally rejects the
    /// first use of that resource.
    #[new]
    #[pyo3(signature = (*, max_netlist_bytes=None, max_netlist_lines=None,
                        max_expanded_source_bytes=None, max_dependency_source_bytes=None,
                        max_external_data_bytes=None, max_external_data_values=None,
                        max_shared_cache_bytes=None, max_include_depth=None,
                        max_hierarchy_depth=None, max_flattened_elements=None,
                        max_circuit_nodes=None, max_matrix_unknowns=None,
                        max_analysis_points=None, max_result_values=None,
                        max_parallel_workers=None, max_batch_runs=None))]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        max_netlist_bytes: Option<usize>,
        max_netlist_lines: Option<usize>,
        max_expanded_source_bytes: Option<usize>,
        max_dependency_source_bytes: Option<usize>,
        max_external_data_bytes: Option<usize>,
        max_external_data_values: Option<usize>,
        max_shared_cache_bytes: Option<usize>,
        max_include_depth: Option<usize>,
        max_hierarchy_depth: Option<usize>,
        max_flattened_elements: Option<usize>,
        max_circuit_nodes: Option<usize>,
        max_matrix_unknowns: Option<usize>,
        max_analysis_points: Option<usize>,
        max_result_values: Option<usize>,
        max_parallel_workers: Option<usize>,
        max_batch_runs: Option<usize>,
    ) -> Self {
        let mut limits = Self::from_core(ResourceLimits::default());
        if let Some(value) = max_netlist_bytes {
            limits.max_netlist_bytes = value;
        }
        if let Some(value) = max_netlist_lines {
            limits.max_netlist_lines = value;
        }
        if let Some(value) = max_expanded_source_bytes {
            limits.max_expanded_source_bytes = value;
        }
        if let Some(value) = max_dependency_source_bytes {
            limits.max_dependency_source_bytes = value;
        }
        if let Some(value) = max_external_data_bytes {
            limits.max_external_data_bytes = value;
        }
        if let Some(value) = max_external_data_values {
            limits.max_external_data_values = value;
        }
        if let Some(value) = max_shared_cache_bytes {
            limits.max_shared_cache_bytes = value;
        }
        if let Some(value) = max_include_depth {
            limits.max_include_depth = value;
        }
        if let Some(value) = max_hierarchy_depth {
            limits.max_hierarchy_depth = value;
        }
        if let Some(value) = max_flattened_elements {
            limits.max_flattened_elements = value;
        }
        if let Some(value) = max_circuit_nodes {
            limits.max_circuit_nodes = value;
        }
        if let Some(value) = max_matrix_unknowns {
            limits.max_matrix_unknowns = value;
        }
        if let Some(value) = max_analysis_points {
            limits.max_analysis_points = value;
        }
        if let Some(value) = max_result_values {
            limits.max_result_values = value;
        }
        if let Some(value) = max_parallel_workers {
            limits.max_parallel_workers = value;
        }
        if let Some(value) = max_batch_runs {
            limits.max_batch_runs = value;
        }
        limits
    }

    /// Construct a policy with every ceiling disabled.
    #[staticmethod]
    pub fn unlimited() -> Self {
        Self::from_core(ResourceLimits::unlimited())
    }

    fn __repr__(&self) -> String {
        format!(
            "ResourceLimits(max_netlist_bytes={}, max_analysis_points={}, max_result_values={}, max_parallel_workers={}, max_batch_runs={})",
            self.max_netlist_bytes,
            self.max_analysis_points,
            self.max_result_values,
            self.max_parallel_workers,
            self.max_batch_runs
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[allow(clippy::too_many_arguments)]
    fn _unpickle(fields: [usize; 16]) -> Self {
        Self {
            max_netlist_bytes: fields[0],
            max_netlist_lines: fields[1],
            max_expanded_source_bytes: fields[2],
            max_dependency_source_bytes: fields[3],
            max_external_data_bytes: fields[4],
            max_external_data_values: fields[5],
            max_shared_cache_bytes: fields[6],
            max_include_depth: fields[7],
            max_hierarchy_depth: fields[8],
            max_flattened_elements: fields[9],
            max_circuit_nodes: fields[10],
            max_matrix_unknowns: fields[11],
            max_analysis_points: fields[12],
            max_result_values: fields[13],
            max_parallel_workers: fields[14],
            max_batch_runs: fields[15],
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, ([usize; 16],))> {
        Ok((
            py.get_type::<Self>().getattr("_unpickle")?,
            ([
                self.max_netlist_bytes,
                self.max_netlist_lines,
                self.max_expanded_source_bytes,
                self.max_dependency_source_bytes,
                self.max_external_data_bytes,
                self.max_external_data_values,
                self.max_shared_cache_bytes,
                self.max_include_depth,
                self.max_hierarchy_depth,
                self.max_flattened_elements,
                self.max_circuit_nodes,
                self.max_matrix_unknowns,
                self.max_analysis_points,
                self.max_result_values,
                self.max_parallel_workers,
                self.max_batch_runs,
            ],),
        ))
    }
}
