//! Periodic AC results.
//!
//! The small-signal response about a periodic operating point. Values are
//! indexed by sideband as well as by node, and the conversion matrix between
//! input and output sidebands is retained separately, because frequency
//! translation is the whole point of the analysis.

use super::*;

/// Periodic small-signal AC sideband conversion result.
#[pyclass(name = "PacResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPacResult {
    inner: rspice_core::analysis::advanced::PacResult,
    #[pyo3(get)]
    pub converged: bool,
}

impl PyPacResult {
    pub fn from_core(result: &rspice_core::engine::PacAnalysisResult) -> Self {
        Self {
            inner: result.result.clone(),
            converged: result.converged,
        }
    }

    fn validate_sideband(&self, sideband: i32) -> PyResult<()> {
        if sideband < self.inner.sideband_min || sideband > self.inner.sideband_max {
            return Err(crate::errors::index_error(format!(
                "sideband {sideband} is outside [{}, {}]",
                self.inner.sideband_min, self.inner.sideband_max
            )));
        }
        Ok(())
    }
}

#[pymethods]
impl PyPacResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.frequencies.to_pyarray(py)
    }

    #[getter]
    fn fundamental_frequency(&self) -> f64 {
        self.inner.fundamental_frequency
    }

    #[getter]
    fn sideband_min(&self) -> i32 {
        self.inner.sideband_min
    }

    #[getter]
    fn sideband_max(&self) -> i32 {
        self.inner.sideband_max
    }

    #[getter]
    fn sidebands(&self) -> Vec<i32> {
        self.inner.sideband_indices()
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn input_source(&self) -> Option<String> {
        self.inner.input_source.clone()
    }

    #[getter]
    fn output_node(&self) -> Option<String> {
        self.inner.output_node.clone()
    }

    fn voltage<'py>(
        &self,
        py: Python<'py>,
        node: &str,
        sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.validate_sideband(sideband)?;
        let node_index = self
            .inner
            .node_index(node)
            .ok_or_else(|| crate::errors::key_error(format!("unknown node '{node}'")))?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| self.inner.voltage(node_index, frequency_index, sideband))
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn conversion_gain<'py>(
        &self,
        py: Python<'py>,
        input_sideband: i32,
        output_sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        self.validate_sideband(input_sideband)?;
        self.validate_sideband(output_sideband)?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| {
                self.inner
                    .conversion_gain(input_sideband, output_sideband, frequency_index)
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn conversion_gain_db<'py>(
        &self,
        py: Python<'py>,
        input_sideband: i32,
        output_sideband: i32,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        self.validate_sideband(input_sideband)?;
        self.validate_sideband(output_sideband)?;
        let values = (0..self.inner.frequencies.len())
            .map(|frequency_index| {
                self.inner
                    .conversion_gain_db(input_sideband, output_sideband, frequency_index)
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn __repr__(&self) -> String {
        format!(
            "PacResult(fundamental={:.6e}Hz, points={}, sidebands={}..={}, converged={})",
            self.inner.fundamental_frequency,
            self.inner.frequencies.len(),
            self.inner.sideband_min,
            self.inner.sideband_max,
            self.converged
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// Branch currents have no accessor on this class and are not carried.
    /// Each sideband's absolute frequency is recomputed by `PacResult::new`
    /// from the same `sideband * f0 + offset` relation that produced it.
    #[staticmethod]
    fn _unpickle(
        sweep: (f64, i32, i32, usize, f64, bool),
        frequencies: Vec<f64>,
        names: (Vec<String>, Vec<String>),
        node_voltages: ComplexGridState,
        conversion_matrix: ComplexGridState,
        sources: (Option<String>, Option<String>),
    ) -> Self {
        let (fundamental_frequency, sideband_min, sideband_max, iterations, residual, converged) =
            sweep;
        let (node_names, branch_names) = names;
        let mut inner = rspice_core::analysis::advanced::PacResult::new(
            fundamental_frequency,
            frequencies.clone(),
            sideband_min,
            sideband_max,
            node_names,
            branch_names,
        );
        for (freq_idx, per_frequency) in node_voltages.into_iter().enumerate() {
            for (offset, voltages) in per_frequency.into_iter().enumerate() {
                let sideband = sideband_min + offset as i32;
                if let Some(data) = inner.get_sideband_data_mut(freq_idx, sideband) {
                    data.node_voltages = complex_from_state(voltages);
                }
            }
        }
        let mut matrix = rspice_core::analysis::advanced::ConversionMatrix::new(
            fundamental_frequency,
            sideband_min,
            sideband_max,
            frequencies,
        );
        for (freq_idx, per_frequency) in conversion_matrix.into_iter().enumerate() {
            for (output_offset, row) in per_frequency.into_iter().enumerate() {
                for (input_offset, (re, im)) in row.into_iter().enumerate() {
                    matrix.set(
                        freq_idx,
                        sideband_min + output_offset as i32,
                        sideband_min + input_offset as i32,
                        Complex64::new(re, im),
                    );
                }
            }
        }
        inner.conversion_matrix = matrix;
        inner.iterations = iterations;
        inner.residual = residual;
        let (input_source, output_node) = sources;
        inner.input_source = input_source;
        inner.output_node = output_node;
        Self { inner, converged }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (f64, i32, i32, usize, f64, bool),
            Vec<f64>,
            (Vec<String>, Vec<String>),
            ComplexGridState,
            ComplexGridState,
            (Option<String>, Option<String>),
        ),
    )> {
        let sidebands: Vec<i32> = (self.inner.sideband_min..=self.inner.sideband_max).collect();
        let node_voltages = (0..self.inner.frequencies.len())
            .map(|freq_idx| {
                sidebands
                    .iter()
                    .map(|&sideband| {
                        self.inner
                            .get_sideband_data(freq_idx, sideband)
                            .map(|data| complex_state(&data.node_voltages))
                            .unwrap_or_default()
                    })
                    .collect()
            })
            .collect();
        let conversion_matrix = (0..self.inner.frequencies.len())
            .map(|freq_idx| {
                sidebands
                    .iter()
                    .map(|&output| {
                        sidebands
                            .iter()
                            .map(|&input| {
                                let value =
                                    self.inner.conversion_matrix.get(freq_idx, output, input);
                                (value.re, value.im)
                            })
                            .collect()
                    })
                    .collect()
            })
            .collect();
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.inner.fundamental_frequency,
                    self.inner.sideband_min,
                    self.inner.sideband_max,
                    self.inner.iterations,
                    self.inner.residual,
                    self.converged,
                ),
                self.inner.frequencies.clone(),
                (
                    self.inner.node_names.clone(),
                    self.inner.branch_names.clone(),
                ),
                node_voltages,
                conversion_matrix,
                (
                    self.inner.input_source.clone(),
                    self.inner.output_node.clone(),
                ),
            ),
        ))
    }
}
