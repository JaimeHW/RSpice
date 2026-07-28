//! Sensitivity results, DC and AC.
//!
//! `SensitivityResult` carries real per-element derivatives of a DC output.
//! `AcSensitivityResult` carries the complex, frequency-dependent case, where
//! each trace has both an absolute and a normalized form: absolute answers
//! "how much does the output move per unit of this parameter", normalized
//! answers "per percent", and confusing the two is a common source of wrong
//! tolerance budgets.

use super::*;

/// Frequency-dependent complex sensitivity to one circuit parameter.
#[pyclass(name = "AcSensitivity", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyAcSensitivity {
    #[pyo3(get)]
    pub vector_name: String,
    #[pyo3(get)]
    pub element: String,
    #[pyo3(get)]
    pub element_type: String,
    #[pyo3(get)]
    pub parameter: String,
    #[pyo3(get)]
    pub nominal_value: f64,
    absolute: Vec<rspice_core::Complex64>,
    normalized: Vec<rspice_core::Complex64>,
    magnitude: Vec<f64>,
    phase: Vec<f64>,
    db: Vec<f64>,
}

#[pymethods]
impl PyAcSensitivity {
    /// Complex unnormalized derivative of the selected output.
    #[getter]
    fn absolute<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.absolute.to_pyarray(py)
    }

    /// Complex normalized derivative `(parameter/output) * d(output)/dp`.
    #[getter]
    fn normalized<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.normalized.to_pyarray(py)
    }

    /// Derivative of output magnitude per unit parameter.
    #[getter]
    fn magnitude<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.magnitude.to_pyarray(py)
    }

    /// Derivative of output phase in radians per unit parameter.
    #[getter]
    fn phase<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.phase.to_pyarray(py)
    }

    /// Derivative of output phase in degrees per unit parameter.
    #[getter]
    fn phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.phase
            .iter()
            .map(|value| value.to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Derivative of `20*log10(|output|)` per unit parameter.
    #[getter]
    fn db<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.db.to_pyarray(py)
    }

    /// Complex percent output change for a one-percent parameter change.
    /// Numerically this equals normalized sensitivity.
    #[getter]
    fn percent_per_percent<'py>(
        &self,
        py: Python<'py>,
    ) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.normalized.to_pyarray(py)
    }

    fn __len__(&self) -> usize {
        self.absolute.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "AcSensitivity(vector_name='{}', parameter='{}', points={})",
            self.vector_name,
            self.parameter,
            self.absolute.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        names: (String, String, String, String),
        nominal_value: f64,
        complex_series: ComplexSeriesPair,
        real_series: (Vec<f64>, Vec<f64>, Vec<f64>),
    ) -> Self {
        let (vector_name, element, element_type, parameter) = names;
        let (absolute, normalized) = complex_series;
        let (absolute, normalized) = (complex_from_state(absolute), complex_from_state(normalized));
        let (magnitude, phase, db) = real_series;
        Self {
            vector_name,
            element,
            element_type,
            parameter,
            nominal_value,
            absolute,
            normalized,
            magnitude,
            phase,
            db,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (String, String, String, String),
            f64,
            ComplexSeriesPair,
            (Vec<f64>, Vec<f64>, Vec<f64>),
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.vector_name.clone(),
                    self.element.clone(),
                    self.element_type.clone(),
                    self.parameter.clone(),
                ),
                self.nominal_value,
                (
                    complex_state(&self.absolute),
                    complex_state(&self.normalized),
                ),
                (self.magnitude.clone(), self.phase.clone(), self.db.clone()),
            ),
        ))
    }
}

/// Complete netlist-wide AC sensitivity result.
#[pyclass(name = "AcSensitivityResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyAcSensitivityResult {
    #[pyo3(get)]
    pub output: String,
    frequencies: Vec<f64>,
    output_values: Vec<rspice_core::Complex64>,
    sensitivities: Vec<PyAcSensitivity>,
}

impl PyAcSensitivityResult {
    pub fn from_core(result: &AcSensitivityResult) -> Self {
        let sensitivities = result
            .sensitivities
            .iter()
            .map(|trace| {
                let db = trace
                    .magnitude
                    .iter()
                    .zip(&result.output_values)
                    .map(|(derivative, output)| {
                        let magnitude = output.norm();
                        if magnitude > 1.0e-300 {
                            20.0 / std::f64::consts::LN_10 * derivative / magnitude
                        } else {
                            0.0
                        }
                    })
                    .collect();
                PyAcSensitivity {
                    vector_name: trace.vector_name.clone(),
                    element: trace.element.clone(),
                    element_type: format!("{:?}", trace.element_type),
                    parameter: trace.parameter.clone(),
                    nominal_value: trace.nominal_value,
                    absolute: trace.absolute.clone(),
                    normalized: trace.normalized.clone(),
                    magnitude: trace.magnitude.clone(),
                    phase: trace.phase.clone(),
                    db,
                }
            })
            .collect();
        Self {
            output: result.output.clone(),
            frequencies: result.frequencies.clone(),
            output_values: result.output_values.clone(),
            sensitivities,
        }
    }
}

#[pymethods]
impl PyAcSensitivityResult {
    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.frequencies.to_pyarray(py)
    }

    #[getter]
    fn output_complex<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.output_values.to_pyarray(py)
    }

    #[getter]
    fn output_magnitude<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_values
            .iter()
            .map(|value| value.norm())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn output_phase<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_values
            .iter()
            .map(|value| value.arg())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn output_phase_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.output_values
            .iter()
            .map(|value| value.arg().to_degrees())
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn sensitivities(&self) -> Vec<PyAcSensitivity> {
        self.sensitivities.clone()
    }

    #[getter]
    fn vector_names(&self) -> Vec<String> {
        self.sensitivities
            .iter()
            .map(|trace| trace.vector_name.clone())
            .collect()
    }

    fn get(&self, vector_name: &str) -> PyResult<PyAcSensitivity> {
        self.sensitivities
            .iter()
            .find(|trace| trace.vector_name.eq_ignore_ascii_case(vector_name))
            .cloned()
            .ok_or_else(|| {
                crate::errors::key_error(format!("unknown AC sensitivity vector '{vector_name}'"))
            })
    }

    /// Most influential traces at one frequency by normalized magnitude.
    #[pyo3(signature = (frequency_index, count=10))]
    fn top(&self, frequency_index: usize, count: usize) -> PyResult<Vec<PyAcSensitivity>> {
        if frequency_index >= self.frequencies.len() {
            return Err(crate::errors::index_error(format!(
                "frequency index {frequency_index} is out of range for result with {} points",
                self.frequencies.len()
            )));
        }
        let mut traces = self.sensitivities.clone();
        traces.sort_by(|left, right| {
            let left_norm = left
                .normalized
                .get(frequency_index)
                .map_or(0.0, |value| value.norm());
            let right_norm = right
                .normalized
                .get(frequency_index)
                .map_or(0.0, |value| value.norm());
            right_norm
                .total_cmp(&left_norm)
                .then_with(|| left.vector_name.cmp(&right.vector_name))
        });
        traces.truncate(count);
        Ok(traces)
    }

    fn __len__(&self) -> usize {
        self.sensitivities.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "AcSensitivityResult(output='{}', frequencies={}, parameters={})",
            self.output,
            self.frequencies.len(),
            self.sensitivities.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        output: String,
        frequencies: Vec<f64>,
        output_values: Vec<(f64, f64)>,
        sensitivities: Vec<PyAcSensitivity>,
    ) -> Self {
        Self {
            output,
            frequencies,
            output_values: complex_from_state(output_values),
            sensitivities,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (String, Vec<f64>, Vec<(f64, f64)>, Vec<PyAcSensitivity>),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.output.clone(),
                self.frequencies.clone(),
                complex_state(&self.output_values),
                self.sensitivities.clone(),
            ),
        ))
    }
}

/// Sensitivity of one output to one device/source parameter.
#[pyclass(name = "ElementSensitivity", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyElementSensitivity {
    #[pyo3(get)]
    pub vector_name: String,
    #[pyo3(get)]
    pub element: String,
    #[pyo3(get)]
    pub element_type: String,
    #[pyo3(get)]
    pub parameter: String,
    #[pyo3(get)]
    pub nominal_value: f64,
    #[pyo3(get)]
    pub absolute: f64,
    #[pyo3(get)]
    pub normalized: f64,
}

impl PyElementSensitivity {
    fn from_core(value: &rspice_core::analysis::Sensitivity) -> Self {
        Self {
            vector_name: value.vector_name.clone(),
            element: value.element.clone(),
            element_type: format!("{:?}", value.element_type),
            parameter: value.parameter.clone(),
            nominal_value: value.nominal_value,
            absolute: value.absolute,
            normalized: value.normalized,
        }
    }
}

#[pymethods]
impl PyElementSensitivity {
    #[getter]
    fn percent_per_percent(&self) -> f64 {
        self.normalized
    }

    fn __repr__(&self) -> String {
        format!(
            "ElementSensitivity(vector_name='{}', element='{}', parameter='{}', absolute={:.6e}, normalized={:.6e})",
            self.vector_name, self.element, self.parameter, self.absolute, self.normalized
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        names: (String, String, String, String),
        nominal_value: f64,
        absolute: f64,
        normalized: f64,
    ) -> Self {
        let (vector_name, element, element_type, parameter) = names;
        Self {
            vector_name,
            element,
            element_type,
            parameter,
            nominal_value,
            absolute,
            normalized,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        ((String, String, String, String), f64, f64, f64),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.vector_name.clone(),
                    self.element.clone(),
                    self.element_type.clone(),
                    self.parameter.clone(),
                ),
                self.nominal_value,
                self.absolute,
                self.normalized,
            ),
        ))
    }
}

/// Complete adjoint DC sensitivity result for an output voltage.
#[pyclass(name = "SensitivityResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySensitivityResult {
    #[pyo3(get)]
    pub output: String,
    #[pyo3(get)]
    pub output_value: f64,
    sensitivities: Vec<PyElementSensitivity>,
}

impl PySensitivityResult {
    pub fn from_core(result: &rspice_core::analysis::SensitivityResult) -> Self {
        Self {
            output: result.output.clone(),
            output_value: result.output_value,
            sensitivities: result
                .sensitivities
                .iter()
                .map(PyElementSensitivity::from_core)
                .collect(),
        }
    }
}

#[pymethods]
impl PySensitivityResult {
    #[getter]
    fn sensitivities(&self) -> Vec<PyElementSensitivity> {
        self.sensitivities.clone()
    }

    #[getter]
    fn vector_names(&self) -> Vec<String> {
        self.sensitivities
            .iter()
            .map(|value| value.vector_name.clone())
            .collect()
    }

    fn __len__(&self) -> usize {
        self.sensitivities.len()
    }

    /// Look up a sensitivity by element and optional parameter name.
    #[pyo3(signature = (element, parameter=None))]
    fn get(&self, element: &str, parameter: Option<&str>) -> PyResult<PyElementSensitivity> {
        self.sensitivities
            .iter()
            .find(|value| {
                (value.vector_name.eq_ignore_ascii_case(element)
                    || value.element.eq_ignore_ascii_case(element))
                    && parameter
                        .is_none_or(|parameter| value.parameter.eq_ignore_ascii_case(parameter))
            })
            .cloned()
            .ok_or_else(|| {
                let suffix = parameter.map_or(String::new(), |name| format!("/{name}"));
                crate::errors::key_error(format!("unknown sensitivity '{element}{suffix}'"))
            })
    }

    /// Most influential entries by absolute normalized sensitivity.
    #[pyo3(signature = (count=10))]
    fn top(&self, count: usize) -> Vec<PyElementSensitivity> {
        let mut values = self.sensitivities.clone();
        values.sort_by(|a, b| {
            b.normalized
                .abs()
                .total_cmp(&a.normalized.abs())
                .then_with(|| a.vector_name.cmp(&b.vector_name))
        });
        values.truncate(count);
        values
    }

    fn __repr__(&self) -> String {
        format!(
            "SensitivityResult(output='{}', output_value={:.6e}, entries={})",
            self.output,
            self.output_value,
            self.sensitivities.len()
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        output: String,
        output_value: f64,
        sensitivities: Vec<PyElementSensitivity>,
    ) -> Self {
        Self {
            output,
            output_value,
            sensitivities,
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (String, f64, Vec<PyElementSensitivity>))> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.output.clone(),
                self.output_value,
                self.sensitivities.clone(),
            ),
        ))
    }
}
