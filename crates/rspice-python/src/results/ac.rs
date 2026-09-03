//! Small-signal AC results.
//!
//! `AcResult` holds the complex phasor solution across a frequency sweep.
//! `ComplexValue` is the scalar complex projection defined here and reused by
//! the pole-zero and stability results, which need the same magnitude, phase,
//! and rectangular accessors over a single complex quantity.

use super::*;

/// AC analysis result with complex frequency-domain data
///
/// Contains frequencies and complex voltage phasors for each node at each
/// frequency. Nodes are addressable by index or name; out-of-range nodes
/// raise rather than returning silent zeros.
///
/// Example:
///     >>> result = engine.run_ac(netlist, [10, 100, 1000])
///     >>> mag_db = result.voltage_db("out")
///     >>> phase_deg = result.voltage_phase_degrees("out")
#[pyclass(name = "AcResult", module = "rspice")]
pub struct PyAcResult {
    frequencies: Vec<f64>,
    pub(crate) results: Vec<AcResult>,
    node_names: Vec<String>,
    branch_names: Vec<String>,
}

/// One frequency point's complete signal schema, as `V(name)`/`I(name)`.
fn ac_point_signals(point: &AcResult) -> Vec<String> {
    point
        .node_names
        .iter()
        .map(|name| format!("V({name})"))
        .chain(point.branch_names.iter().map(|name| format!("I({name})")))
        .collect()
}

/// Adopt the first point's signal schema, having proved every point matches it.
///
/// A frequency sweep is one table whose columns are fixed by the circuit, not
/// by the point. Reading the names off `results[0]` and then indexing later
/// rows through a fallible lookup leaves a solver that changed its mind
/// looking like a well-formed table with a few NaNs in it. Divergence is a
/// typed failure naming the point and the signals instead.
pub(crate) fn validated_ac_schema(
    analysis: &str,
    points: &[AcResult],
) -> Result<(Vec<String>, Vec<String>), String> {
    let Some(first) = points.first() else {
        return Ok((Vec::new(), Vec::new()));
    };
    let expected = ac_point_signals(first);
    for (index, point) in points.iter().enumerate() {
        let where_ = format!("point {} ({:.16e} Hz)", index + 1, point.frequency);
        if point.voltages.len() != point.node_names.len()
            || point.currents.len() != point.branch_names.len()
        {
            return Err(format!(
                "malformed {analysis} result at {where_}: {} voltages for {} node names and {} currents for {} branch names",
                point.voltages.len(),
                point.node_names.len(),
                point.currents.len(),
                point.branch_names.len()
            ));
        }
        if index == 0 {
            continue;
        }
        let actual = ac_point_signals(point);
        if actual == expected {
            continue;
        }
        let expected_set = expected.iter().collect::<std::collections::BTreeSet<_>>();
        let actual_set = actual.iter().collect::<std::collections::BTreeSet<_>>();
        let missing = expected_set
            .difference(&actual_set)
            .map(|name| name.as_str())
            .collect::<Vec<_>>();
        let unexpected = actual_set
            .difference(&expected_set)
            .map(|name| name.as_str())
            .collect::<Vec<_>>();
        if missing.is_empty() && unexpected.is_empty() {
            let first_difference = expected
                .iter()
                .zip(&actual)
                .position(|(left, right)| left != right)
                .unwrap_or(0);
            return Err(format!(
                "{analysis} result schema is reordered at {where_}: column {first_difference} is '{}' but the first point published '{}'",
                actual
                    .get(first_difference)
                    .map_or("<missing>", String::as_str),
                expected
                    .get(first_difference)
                    .map_or("<missing>", String::as_str)
            ));
        }
        return Err(format!(
            "{analysis} result schema changes at {where_}: missing [{}]; unexpected [{}]",
            missing.join(", "),
            unexpected.join(", ")
        ));
    }
    Ok((first.node_names.clone(), first.branch_names.clone()))
}

impl PyAcResult {
    pub fn new(frequencies: Vec<f64>, results: Vec<AcResult>) -> PyResult<Self> {
        Self::checked(frequencies, results).map_err(crate::errors::SimulationError::new_err)
    }

    fn checked(frequencies: Vec<f64>, results: Vec<AcResult>) -> Result<Self, String> {
        if frequencies.len() != results.len() {
            return Err(format!(
                "malformed AC result: {} solved points for {} requested frequencies",
                results.len(),
                frequencies.len()
            ));
        }
        let (node_names, branch_names) = validated_ac_schema("AC", &results)?;
        Ok(Self {
            frequencies,
            results,
            node_names,
            branch_names,
        })
    }

    /// Number of non-ground nodes with phasor data.
    ///
    /// Every point carries the same schema (proved on construction), so the
    /// first row's width is the sweep's width.
    fn node_count(&self) -> usize {
        self.results
            .first()
            .map(|r| r.voltages.len())
            .unwrap_or(self.node_names.len())
    }

    /// Resolve a node identifier to a node index (0 = ground).
    ///
    /// `node_names[i]` corresponds to node index `i + 1` (core's
    /// `node_names_sorted` excludes ground).
    fn resolve_node(&self, node: &NodeIdentifier) -> AccessResult<usize> {
        match node {
            NodeIdentifier::Index(idx) => {
                if *idx <= self.node_count() {
                    Ok(*idx)
                } else {
                    Err(invalid_node_index_error(*idx, self.node_count()))
                }
            }
            NodeIdentifier::Name(name) => {
                if is_ground_name(name) {
                    return Ok(0);
                }
                self.node_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(name))
                    .map(|pos| pos + 1)
                    .ok_or_else(|| unknown_node_name_error(name))
            }
        }
    }

    fn resolve_branch(&self, name: &str) -> AccessResult<usize> {
        self.branch_names
            .iter()
            .position(|n| n.eq_ignore_ascii_case(name))
            .ok_or_else(|| unknown_branch_name_error(name))
    }

    fn checked_freq_index(&self, index: usize) -> AccessResult<()> {
        if index < self.results.len() {
            Ok(())
        } else {
            Err(invalid_freq_index_error(index, self.results.len()))
        }
    }

    /// Column layout shared by the CSV and raw exporters.
    ///
    /// Every point carries the schema the first point published — proved when
    /// the result was constructed — so each column is exactly as long as the
    /// frequency axis and no cell has to be invented.
    fn raw_plot(&self, title: &str) -> crate::export::RawPlot {
        use crate::export::{RawVariable, RawVariableKind};

        let mut variables = vec![RawVariable {
            name: "frequency".to_string(),
            kind: RawVariableKind::Frequency,
        }];
        let mut series = vec![
            self.frequencies
                .iter()
                .map(|frequency| rspice_core::Complex64::new(*frequency, 0.0))
                .collect::<Vec<_>>(),
        ];
        for (index, name) in self.node_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("V({name})"),
                kind: RawVariableKind::Voltage,
            });
            series.push(
                self.results
                    .iter()
                    .filter_map(|row| row.voltages.get(index).copied())
                    .collect(),
            );
        }
        for (index, name) in self.branch_names.iter().enumerate() {
            variables.push(RawVariable {
                name: format!("I({name})"),
                kind: RawVariableKind::Current,
            });
            series.push(
                self.results
                    .iter()
                    .filter_map(|row| row.currents.get(index).copied())
                    .collect(),
            );
        }

        crate::export::RawPlot {
            title: title.to_string(),
            plot_name: "AC Analysis".to_string(),
            variables,
            series,
            complex: true,
            timestamp: None,
        }
    }

    fn frequency_label(&self, freq_index: usize, result: &AcResult) -> String {
        let frequency = self
            .frequencies
            .get(freq_index)
            .copied()
            .unwrap_or(result.frequency);
        format!("{frequency:.6e} Hz")
    }

    fn node_label(&self, node: usize) -> String {
        if node == 0 {
            return "ground".to_string();
        }
        match self.node_names.get(node - 1) {
            Some(name) => format!("{node} ('{name}')"),
            None => node.to_string(),
        }
    }

    fn branch_label(&self, branch: usize) -> String {
        match self.branch_names.get(branch) {
            Some(name) => format!("{branch} ('{name}')"),
            None => branch.to_string(),
        }
    }

    fn checked_voltage_phasor(
        &self,
        freq_index: usize,
        node: usize,
    ) -> PyResult<rspice_core::Complex64> {
        let result = self
            .results
            .get(freq_index)
            .ok_or_else(|| PyErr::from(invalid_freq_index_error(freq_index, self.results.len())))?;
        self.voltage_phasor_from_row(freq_index, result, node)
            .map_err(crate::errors::value_error)
    }

    fn voltage_phasor_from_row(
        &self,
        freq_index: usize,
        result: &AcResult,
        node: usize,
    ) -> Result<rspice_core::Complex64, String> {
        if node == 0 {
            return Ok(rspice_core::Complex64::new(0.0, 0.0));
        }
        result.voltages.get(node - 1).copied().ok_or_else(|| {
            format!(
                "malformed AC result row {freq_index} ({}): missing voltage for node {}; \
                 row has {} voltage value(s), expected at least {}",
                self.frequency_label(freq_index, result),
                self.node_label(node),
                result.voltages.len(),
                node
            )
        })
    }

    fn checked_branch_current(
        &self,
        freq_index: usize,
        branch: usize,
    ) -> PyResult<rspice_core::Complex64> {
        let result = self
            .results
            .get(freq_index)
            .ok_or_else(|| PyErr::from(invalid_freq_index_error(freq_index, self.results.len())))?;
        self.branch_current_from_row(freq_index, result, branch)
            .map_err(crate::errors::value_error)
    }

    fn branch_current_from_row(
        &self,
        freq_index: usize,
        result: &AcResult,
        branch: usize,
    ) -> Result<rspice_core::Complex64, String> {
        result.currents.get(branch).copied().ok_or_else(|| {
            format!(
                "malformed AC result row {freq_index} ({}): missing current for branch {}; \
                 row has {} current value(s), expected at least {}",
                self.frequency_label(freq_index, result),
                self.branch_label(branch),
                result.currents.len(),
                branch + 1
            )
        })
    }
}

#[pymethods]
impl PyAcResult {
    /// Get the frequency array
    ///
    /// Returns:
    ///     numpy.ndarray: Array of frequencies in Hz
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    /// Get the number of frequency points
    #[getter]
    pub fn num_frequencies(&self) -> usize {
        self.frequencies.len()
    }

    /// Node names addressable in this result (excluding ground)
    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.node_names.clone()
    }

    /// Branch names with complex current phasors
    #[getter]
    fn branch_names(&self) -> Vec<String> {
        self.branch_names.clone()
    }

    /// Get voltage magnitude at a node across all frequencies
    ///
    /// Args:
    ///     node: Node index or name
    ///
    /// Returns:
    ///     numpy.ndarray: Magnitude values at each frequency
    ///
    /// Raises:
    ///     IndexError / KeyError: For invalid nodes
    fn voltage_magnitude<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let magnitudes: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.norm())
            })
            .collect();
        Ok(magnitudes?.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (radians)
    fn voltage_phase<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.arg())
            })
            .collect();
        Ok(phases?.to_pyarray(py))
    }

    /// Get voltage phase at a node across all frequencies (degrees)
    fn voltage_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let phases: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| v.arg().to_degrees())
            })
            .collect();
        Ok(phases?.to_pyarray(py))
    }

    /// Get complex voltage at a node across all frequencies
    ///
    /// Returns:
    ///     numpy.ndarray: complex128 phasor values at each frequency
    ///
    /// Example:
    ///     >>> h = ac.voltage_complex("out") / ac.voltage_complex("in")
    fn voltage_complex<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let values: PyResult<Vec<rspice_core::Complex64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| self.checked_voltage_phasor(freq_index, idx))
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get voltage magnitude in dB (20·log10 |V|) at a node across all frequencies
    fn voltage_db<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        let db: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_voltage_phasor(freq_index, idx)
                    .map(|v| 20.0 * v.norm().log10())
            })
            .collect();
        Ok(db?.to_pyarray(py))
    }

    /// Get complex branch current through an element across all frequencies
    ///
    /// Branch currents exist for voltage sources and inductors.
    ///
    /// Args:
    ///     name: Element name (e.g. "V1", "L2")
    ///
    /// Raises:
    ///     KeyError: If no branch carries that name
    fn branch_current_complex<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let idx = self.resolve_branch(name).map_err(PyErr::from)?;
        let values: PyResult<Vec<rspice_core::Complex64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| self.checked_branch_current(freq_index, idx))
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get branch current magnitude through an element across all frequencies
    fn branch_current_magnitude<'py>(
        &self,
        py: Python<'py>,
        name: &str,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let idx = self.resolve_branch(name).map_err(PyErr::from)?;
        let values: PyResult<Vec<f64>> = self
            .results
            .iter()
            .enumerate()
            .map(|(freq_index, _)| {
                self.checked_branch_current(freq_index, idx)
                    .map(|c| c.norm())
            })
            .collect();
        Ok(values?.to_pyarray(py))
    }

    /// Get voltage magnitude at a specific frequency index and node
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    pub fn magnitude_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.checked_voltage_phasor(freq_index, idx)?.norm())
    }

    /// Get phase at a specific frequency index and node (radians)
    ///
    /// Raises:
    ///     IndexError: If the frequency index is out of range
    fn phase_at(&self, freq_index: usize, node: NodeIdentifier) -> PyResult<f64> {
        self.checked_freq_index(freq_index).map_err(PyErr::from)?;
        let idx = self.resolve_node(&node).map_err(PyErr::from)?;
        Ok(self.checked_voltage_phasor(freq_index, idx)?.arg())
    }

    /// Column headers used by the raw exporter, in order.
    #[getter]
    fn export_columns(&self) -> Vec<String> {
        self.raw_plot("")
            .variables
            .into_iter()
            .map(|v| v.name)
            .collect()
    }

    /// Render this sweep as RFC 4180 CSV.
    ///
    /// The first column is `frequency`; every phasor becomes an adjacent
    /// `<name>_real` / `<name>_imag` pair, so the table is lossless rather
    /// than committing to a magnitude/phase convention.
    fn to_csv(&self) -> PyResult<String> {
        let plot = self.raw_plot("");
        // The sweep axis is real; only the phasor columns are split, so the
        // table has no all-zero imaginary frequency column.
        let (axis, phasors) = plot
            .variables
            .split_first()
            .ok_or_else(|| crate::errors::value_error("AC result has no columns"))?;
        let mut headers = vec![axis.name.clone()];
        headers.extend(complex_csv_headers(
            &phasors
                .iter()
                .map(|variable| variable.name.clone())
                .collect::<Vec<_>>(),
        ));

        let (axis_series, phasor_series) = plot
            .series
            .split_first()
            .ok_or_else(|| crate::errors::value_error("AC result has no data"))?;
        let interleaved = transpose_complex(phasor_series);
        let rows = axis_series
            .iter()
            .zip(interleaved)
            .map(|(frequency, mut row)| {
                row.insert(0, frequency.re);
                row
            })
            .collect::<Vec<_>>();

        crate::export::csv(&headers, &rows).map_err(crate::errors::value_error)
    }

    /// Write `to_csv()` to a file.
    fn write_csv(&self, path: PathBuf) -> PyResult<()> {
        write_export_file(&path, self.to_csv()?.as_bytes())
    }

    /// Serialize as an ngspice-compatible SPICE raw file.
    ///
    /// AC data is written with `Flags: complex`, so every value carries both
    /// its real and imaginary part.
    ///
    /// Args:
    ///     format: "ascii" (default) or "binary"
    ///     title: Header title; defaults to "RSpice AC analysis"
    #[pyo3(signature = (*, format="ascii", title=None, timestamp=None))]
    fn to_raw<'py>(
        &self,
        py: Python<'py>,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<Bound<'py, pyo3::types::PyBytes>> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice AC analysis"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        Ok(pyo3::types::PyBytes::new(py, &bytes))
    }

    /// Write a SPICE raw file. Same options as `to_raw`.
    #[pyo3(signature = (path, *, format="ascii", title=None, timestamp=None))]
    fn write_raw(
        &self,
        path: PathBuf,
        format: &str,
        title: Option<&str>,
        timestamp: Option<&str>,
    ) -> PyResult<()> {
        let mut plot = self.raw_plot(title.unwrap_or("RSpice AC analysis"));
        plot.timestamp = timestamp.map(str::to_string);
        let bytes = raw_export_bytes(
            &plot,
            crate::export::RawFormat::parse(format).map_err(crate::errors::value_error)?,
        )?;
        write_export_file(&path, &bytes)
    }

    fn __repr__(&self) -> String {
        let freq_range = match (self.frequencies.first(), self.frequencies.last()) {
            (Some(first), Some(last)) => format!("{first:.1e}-{last:.1e} Hz"),
            _ => "no frequencies".to_string(),
        };
        format!(
            "AcResult(frequencies={}, range={}, nodes={})",
            self.frequencies.len(),
            freq_range,
            self.node_count()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(frequencies: Vec<f64>, rows: Vec<AcRowState>) -> PyResult<Self> {
        Self::new(frequencies, rows.into_iter().map(rebuild_ac_row).collect())
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (Vec<f64>, Vec<AcRowState>))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.frequencies.clone(),
                self.results.iter().map(ac_row_state).collect(),
            ),
        ))
    }
}

/// Complex number for poles and zeros
///
/// Represents a pole or zero in the s-domain (Laplace domain). Convertible
/// to a built-in complex with `complex(value)`.
///
/// Example:
///     >>> for pole in result.poles:
///     ...     print(complex(pole))
///     ...     if pole.is_real:
///     ...         print(f"  Time constant: {pole.time_constant:.3e}s")
#[pyclass(name = "ComplexValue", module = "rspice", from_py_object)]
#[derive(Debug, Clone, Copy)]
pub struct PyComplexValue {
    /// Real part
    #[pyo3(get)]
    pub real: f64,
    /// Imaginary part
    #[pyo3(get)]
    pub imag: f64,
}

impl PyComplexValue {
    pub fn from_core(c: &rspice_core::Complex64) -> Self {
        Self {
            real: c.re,
            imag: c.im,
        }
    }
}

#[pymethods]
impl PyComplexValue {
    /// Convert to a built-in Python complex number
    fn __complex__<'py>(&self, py: Python<'py>) -> Bound<'py, pyo3::types::PyComplex> {
        pyo3::types::PyComplex::from_doubles(py, self.real, self.imag)
    }

    /// Get magnitude |z|
    #[getter]
    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Get phase in radians
    #[getter]
    fn phase(&self) -> f64 {
        self.imag.atan2(self.real)
    }

    /// Get phase in degrees
    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.phase().to_degrees()
    }

    /// Check if this is a real value (imaginary part near zero)
    ///
    /// Visible across the results tree because `PoleZeroResult` classifies
    /// roots with the same tolerance rather than inventing a second one.
    #[getter]
    pub(super) fn is_real(&self) -> bool {
        let scale = self.real.abs().max(self.imag.abs()).max(1.0);
        self.imag.abs() <= 64.0 * f64::EPSILON * scale
    }

    /// Get frequency in Hz (|Im| / 2π)
    #[getter]
    fn frequency_hz(&self) -> f64 {
        self.imag.abs() / (2.0 * std::f64::consts::PI)
    }

    /// Get damping factor ζ = -Re(p)/|p| for complex pole
    #[getter]
    fn damping_factor(&self) -> f64 {
        let mag = self.magnitude();
        if mag > 1e-15 { -self.real / mag } else { 0.0 }
    }

    /// Get decay time constant τ = -1/Re(p) for a stable real pole.
    #[getter]
    fn time_constant(&self) -> Option<f64> {
        if self.is_real() && self.real < 0.0 {
            Some(-1.0 / self.real)
        } else {
            None
        }
    }

    fn __repr__(&self) -> String {
        if self.imag >= 0.0 {
            format!("{:.6e}+{:.6e}j", self.real, self.imag)
        } else {
            format!("{:.6e}{:.6e}j", self.real, self.imag)
        }
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(real: f64, imag: f64) -> Self {
        Self { real, imag }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(&self, py: Python<'py>) -> PyResult<(Bound<'py, PyAny>, (f64, f64))> {
        Ok((unpickler::<Self>(py)?, (self.real, self.imag)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::Complex64;

    fn ac_row(frequency: f64, voltages: Vec<Complex64>, currents: Vec<Complex64>) -> AcResult {
        AcResult {
            frequency,
            node_names: vec!["out".to_string()],
            branch_names: vec!["V1".to_string()],
            voltages,
            currents,
        }
    }

    /// Assemble the struct without the schema proof, to exercise the
    /// per-row guards that construction now makes unreachable from Python.
    fn unchecked(frequencies: Vec<f64>, results: Vec<AcResult>) -> PyAcResult {
        let node_names = results[0].node_names.clone();
        let branch_names = results[0].branch_names.clone();
        PyAcResult {
            frequencies,
            results,
            node_names,
            branch_names,
        }
    }

    #[test]
    fn construction_refuses_a_row_whose_width_contradicts_its_own_names() {
        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                ac_row(2.0, Vec::new(), Vec::new()),
            ],
        )
        .err()
        .expect("a row that does not fill its own schema is malformed");

        assert!(
            message.contains("point 2 (2.0000000000000000e0"),
            "{message}"
        );
        assert!(message.contains("0 voltages for 1 node names"), "{message}");
    }

    #[test]
    fn construction_names_the_signals_a_diverging_point_added_and_dropped() {
        let mut diverged = ac_row(
            2.0,
            vec![Complex64::new(1.0, 0.0)],
            vec![Complex64::new(0.0, 1.0)],
        );
        diverged.node_names = vec!["elsewhere".to_string()];

        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                diverged,
            ],
        )
        .err()
        .expect("a point that publishes different signals is malformed");

        assert!(
            message.contains("AC result schema changes at point 2"),
            "{message}"
        );
        assert!(message.contains("missing [V(out)]"), "{message}");
        assert!(message.contains("unexpected [V(elsewhere)]"), "{message}");
    }

    #[test]
    fn construction_refuses_a_frequency_axis_the_solve_did_not_fill() {
        let message = PyAcResult::checked(
            vec![1.0, 2.0],
            vec![ac_row(
                1.0,
                vec![Complex64::new(1.0, 0.0)],
                vec![Complex64::new(0.0, 1.0)],
            )],
        )
        .err()
        .expect("a short solve must not be padded out");

        assert!(
            message.contains("1 solved points for 2 requested frequencies"),
            "{message}"
        );
    }

    #[test]
    fn ac_voltage_access_rejects_short_later_rows() {
        let ac = unchecked(
            vec![1.0, 2.0],
            vec![
                ac_row(1.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
                ac_row(2.0, Vec::new(), Vec::new()),
            ],
        );

        let message = ac
            .voltage_phasor_from_row(1, &ac.results[1], 1)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing voltage"), "{message}");
    }

    #[test]
    fn ac_branch_access_rejects_short_later_rows() {
        let ac = unchecked(
            vec![1.0, 2.0],
            vec![
                ac_row(
                    1.0,
                    vec![Complex64::new(1.0, 0.0)],
                    vec![Complex64::new(0.0, 1.0)],
                ),
                ac_row(2.0, vec![Complex64::new(1.0, 0.0)], Vec::new()),
            ],
        );

        let message = ac
            .branch_current_from_row(1, &ac.results[1], 0)
            .unwrap_err();
        assert!(message.contains("malformed AC result row 1"), "{message}");
        assert!(message.contains("missing current"), "{message}");
    }
}
