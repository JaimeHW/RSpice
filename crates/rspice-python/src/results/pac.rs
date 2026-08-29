//! Periodic AC results.
//!
//! The small-signal response about a periodic operating point. Values are
//! indexed by sideband as well as by node, and the conversion matrix between
//! input and output sidebands is retained separately, because frequency
//! translation is the whole point of the analysis.

use super::*;

fn pac_value_error(error: impl std::fmt::Display) -> PyErr {
    crate::errors::value_error(error.to_string())
}

fn checked_pickle_sideband_count(sideband_min: i32, sideband_max: i32) -> PyResult<usize> {
    let span = i64::from(sideband_max) - i64::from(sideband_min);
    if span < 0 {
        return Err(pac_value_error(format!(
            "PAC pickle sideband range [{sideband_min}, {sideband_max}] is empty"
        )));
    }
    usize::try_from(span + 1)
        .map_err(|_| pac_value_error("PAC pickle sideband count exceeds this platform"))
}

fn checked_pickle_sideband(sideband_min: i32, offset: usize) -> PyResult<i32> {
    i64::from(sideband_min)
        .checked_add(
            i64::try_from(offset)
                .map_err(|_| pac_value_error("PAC pickle sideband offset exceeds i64"))?,
        )
        .and_then(|value| i32::try_from(value).ok())
        .ok_or_else(|| pac_value_error("PAC pickle sideband index exceeds i32"))
}

fn validate_pickle_grid(
    grid: &ComplexGridState,
    expected_frequencies: usize,
    expected_rows: usize,
    expected_values: usize,
    label: &str,
) -> PyResult<()> {
    if grid.len() != expected_frequencies {
        return Err(pac_value_error(format!(
            "PAC pickle {label} has {} frequency rows; expected {expected_frequencies}",
            grid.len()
        )));
    }
    for (frequency_index, rows) in grid.iter().enumerate() {
        if rows.len() != expected_rows {
            return Err(pac_value_error(format!(
                "PAC pickle {label} frequency row {frequency_index} has {} sideband rows; expected {expected_rows}",
                rows.len()
            )));
        }
        for (row_index, values) in rows.iter().enumerate() {
            if values.len() != expected_values {
                return Err(pac_value_error(format!(
                    "PAC pickle {label} frequency row {frequency_index}, sideband row {row_index} has {} values; expected {expected_values}",
                    values.len()
                )));
            }
            if let Some((value_index, (re, im))) = values
                .iter()
                .copied()
                .enumerate()
                .find(|(_, (re, im))| !re.is_finite() || !im.is_finite())
            {
                return Err(pac_value_error(format!(
                    "PAC pickle {label} frequency row {frequency_index}, sideband row {row_index}, value {value_index} is non-finite ({re:+.6e}{im:+.6e}j)"
                )));
            }
        }
    }
    Ok(())
}

/// Periodic small-signal AC sideband conversion result.
#[pyclass(name = "PacResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPacResult {
    inner: rspice_core::analysis::PacResult,
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(pac_value_error)?;
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(pac_value_error)?;
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
            .collect::<Result<Vec<_>, _>>()
            .map_err(pac_value_error)?;
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
    ) -> PyResult<Self> {
        let (fundamental_frequency, sideband_min, sideband_max, iterations, residual, converged) =
            sweep;
        let (node_names, branch_names) = names;
        let (input_source, output_node) = sources;
        if !residual.is_finite() || residual < 0.0 {
            return Err(pac_value_error(format!(
                "PAC pickle residual must be finite and non-negative, got {residual}"
            )));
        }
        let sideband_count = checked_pickle_sideband_count(sideband_min, sideband_max)?;
        validate_pickle_grid(
            &node_voltages,
            frequencies.len(),
            sideband_count,
            node_names.len(),
            "node-voltage grid",
        )?;
        if input_source
            .as_deref()
            .is_none_or(|name| name.trim().is_empty())
        {
            return Err(pac_value_error(
                "PAC pickle is missing its input-source identity",
            ));
        }
        let output_name = output_node
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .ok_or_else(|| pac_value_error("PAC pickle is missing its output-node identity"))?;
        if !node_names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(output_name))
        {
            return Err(pac_value_error(format!(
                "PAC pickle output node '{output_name}' is not present in its node identities"
            )));
        }
        if conversion_matrix.is_empty() {
            return Err(pac_value_error(
                "PAC pickle with an output node is missing its conversion grid",
            ));
        }
        validate_pickle_grid(
            &conversion_matrix,
            frequencies.len(),
            sideband_count,
            sideband_count,
            "conversion grid",
        )?;
        let mut inner = rspice_core::analysis::PacResult::new(
            fundamental_frequency,
            frequencies,
            sideband_min,
            sideband_max,
            node_names,
            branch_names,
        )
        .map_err(pac_value_error)?;
        for (freq_idx, per_frequency) in node_voltages.into_iter().enumerate() {
            for (offset, voltages) in per_frequency.into_iter().enumerate() {
                let sideband = checked_pickle_sideband(sideband_min, offset)?;
                let data = inner
                    .get_sideband_data_mut(freq_idx, sideband)
                    .ok_or_else(|| {
                        pac_value_error(format!(
                            "PAC pickle node-voltage coordinate ({freq_idx}, {sideband}) is outside the constructed result"
                        ))
                    })?;
                for (slot, (re, im)) in data.node_voltages.iter_mut().zip(voltages) {
                    *slot = Complex64::new(re, im);
                }
            }
        }
        for (freq_idx, per_frequency) in conversion_matrix.into_iter().enumerate() {
            for (output_offset, row) in per_frequency.into_iter().enumerate() {
                let output_sideband = checked_pickle_sideband(sideband_min, output_offset)?;
                for (input_offset, (re, im)) in row.into_iter().enumerate() {
                    let input_sideband = checked_pickle_sideband(sideband_min, input_offset)?;
                    inner
                        .conversion_matrix
                        .set(
                            freq_idx,
                            output_sideband,
                            input_sideband,
                            Complex64::new(re, im),
                        )
                        .map_err(pac_value_error)?;
                }
            }
        }
        inner.iterations = iterations;
        inner.residual = residual;
        inner.input_source = input_source;
        inner.output_node = output_node;
        Ok(Self { inner, converged })
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
                                self.inner
                                    .conversion_matrix
                                    .get(freq_idx, output, input)
                                    .map(|value| (value.re, value.im))
                            })
                            .collect::<Result<Vec<_>, _>>()
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(pac_value_error)?;
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
