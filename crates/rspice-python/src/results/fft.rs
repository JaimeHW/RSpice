//! Typed Python projection of source-authored transient `.FFT` products.
//!
//! FFT products are immutable evidence computed from the qualified uniform
//! source trajectory before optional waveform compression.  These wrappers
//! expose every core field without recomputing bins from the Python-visible
//! waveform subset.

use super::*;

fn fft_value_unit(
    physical_type: &str,
    format: rspice_core::netlist::FftFormat,
) -> Result<Option<&'static str>, String> {
    let physical_unit = match physical_type {
        "voltage" => Some("V"),
        "current" => Some("A"),
        "parameter" => None,
        other => return Err(format!("unsupported transient FFT physical type '{other}'")),
    };
    Ok(match format {
        rspice_core::netlist::FftFormat::Normalized => Some("1"),
        rspice_core::netlist::FftFormat::Unnormalized => physical_unit,
    })
}

#[pyclass(name = "FftBin", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientFftBin {
    inner: rspice_core::engine::TransientFftBin,
}

impl From<&rspice_core::engine::TransientFftBin> for PyTransientFftBin {
    fn from(inner: &rspice_core::engine::TransientFftBin) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[pymethods]
impl PyTransientFftBin {
    #[getter]
    fn index(&self) -> usize {
        self.inner.index
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.frequency
    }

    #[getter]
    fn real(&self) -> f64 {
        self.inner.real
    }

    #[getter]
    fn imaginary(&self) -> f64 {
        self.inner.imaginary
    }

    #[getter]
    fn value(&self) -> PyComplexValue {
        PyComplexValue::from_core(&rspice_core::Complex64::new(
            self.inner.real,
            self.inner.imaginary,
        ))
    }

    #[getter]
    fn magnitude(&self) -> f64 {
        self.inner.magnitude
    }

    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.inner.phase_degrees
    }

    fn __repr__(&self) -> String {
        format!(
            "FftBin(index={}, frequency={:.6e}Hz, value={:.6e}{:+.6e}j)",
            self.inner.index, self.inner.frequency, self.inner.real, self.inner.imaginary
        )
    }

    #[staticmethod]
    fn _unpickle(
        index: usize,
        frequency: f64,
        real: f64,
        imaginary: f64,
        magnitude: f64,
        phase_degrees: f64,
    ) -> Self {
        Self {
            inner: rspice_core::engine::TransientFftBin {
                index,
                frequency,
                real,
                imaginary,
                magnitude,
                phase_degrees,
            },
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, TransientFftBinState)> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.index,
                self.inner.frequency,
                self.inner.real,
                self.inner.imaginary,
                self.inner.magnitude,
                self.inner.phase_degrees,
            ),
        ))
    }
}

#[pyclass(name = "FftHarmonic", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientFftHarmonic {
    inner: rspice_core::engine::TransientFftHarmonic,
}

impl From<&rspice_core::engine::TransientFftHarmonic> for PyTransientFftHarmonic {
    fn from(inner: &rspice_core::engine::TransientFftHarmonic) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[pymethods]
impl PyTransientFftHarmonic {
    #[getter]
    fn rank(&self) -> usize {
        self.inner.rank
    }

    #[getter]
    fn bin(&self) -> usize {
        self.inner.bin
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.frequency
    }

    #[getter]
    fn magnitude(&self) -> f64 {
        self.inner.magnitude
    }

    #[getter]
    fn magnitude_db(&self) -> f64 {
        self.inner.magnitude_db
    }

    #[getter]
    fn phase_degrees(&self) -> f64 {
        self.inner.phase_degrees
    }

    fn __repr__(&self) -> String {
        format!(
            "FftHarmonic(rank={}, bin={}, frequency={:.6e}Hz, magnitude={:.6e})",
            self.inner.rank, self.inner.bin, self.inner.frequency, self.inner.magnitude
        )
    }

    #[staticmethod]
    fn _unpickle(
        rank: usize,
        bin: usize,
        frequency: f64,
        magnitude: f64,
        magnitude_db: f64,
        phase_degrees: f64,
    ) -> Self {
        Self {
            inner: rspice_core::engine::TransientFftHarmonic {
                rank,
                bin,
                frequency,
                magnitude,
                magnitude_db,
                phase_degrees,
            },
        }
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, TransientFftHarmonicState)> {
        Ok((
            unpickler::<Self>(py)?,
            (
                self.inner.rank,
                self.inner.bin,
                self.inner.frequency,
                self.inner.magnitude,
                self.inner.magnitude_db,
                self.inner.phase_degrees,
            ),
        ))
    }
}

#[pyclass(name = "FftMetrics", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientFftMetrics {
    inner: rspice_core::engine::TransientFftMetrics,
}

impl From<&rspice_core::engine::TransientFftMetrics> for PyTransientFftMetrics {
    fn from(inner: &rspice_core::engine::TransientFftMetrics) -> Self {
        Self {
            inner: inner.clone(),
        }
    }
}

#[pymethods]
impl PyTransientFftMetrics {
    #[getter]
    fn fundamental_magnitude(&self) -> f64 {
        self.inner.fundamental_magnitude
    }

    #[getter]
    fn thd_ratio(&self) -> f64 {
        self.inner.thd_ratio
    }

    #[getter]
    fn thd_db(&self) -> f64 {
        self.inner.thd_db
    }

    #[getter]
    fn sndr_db(&self) -> f64 {
        self.inner.sndr_db
    }

    #[getter]
    fn enob_bits(&self) -> f64 {
        self.inner.enob_bits
    }

    #[getter]
    fn snr_db(&self) -> f64 {
        self.inner.snr_db
    }

    #[getter]
    fn sfdr_db(&self) -> f64 {
        self.inner.sfdr_db
    }

    #[getter]
    fn sfdr_spur_bin(&self) -> Option<usize> {
        self.inner.sfdr_spur_bin
    }

    #[getter]
    fn sfdr_spur_frequency(&self) -> Option<f64> {
        self.inner.sfdr_spur_frequency
    }

    #[getter]
    fn largest_harmonics(&self) -> Vec<PyTransientFftHarmonic> {
        self.inner
            .largest_harmonics
            .iter()
            .map(PyTransientFftHarmonic::from)
            .collect()
    }

    fn __repr__(&self) -> String {
        format!(
            "FftMetrics(thd_ratio={:.6e}, sndr_db={:.3}, enob_bits={:.3}, harmonics={})",
            self.inner.thd_ratio,
            self.inner.sndr_db,
            self.inner.enob_bits,
            self.inner.largest_harmonics.len()
        )
    }

    #[staticmethod]
    fn _unpickle(
        scalars: (f64, f64, f64, f64, f64, f64, f64),
        spur: (Option<usize>, Option<f64>),
        largest_harmonics: Vec<PyTransientFftHarmonic>,
    ) -> PyResult<Self> {
        let (fundamental_magnitude, thd_ratio, thd_db, sndr_db, enob_bits, snr_db, sfdr_db) =
            scalars;
        let (sfdr_spur_bin, sfdr_spur_frequency) = spur;
        if sfdr_spur_bin.is_some() != sfdr_spur_frequency.is_some() {
            return Err(crate::errors::value_error(
                "FFT metric spur bin and frequency must be present together",
            ));
        }
        Ok(Self {
            inner: rspice_core::engine::TransientFftMetrics {
                fundamental_magnitude,
                thd_ratio,
                thd_db,
                sndr_db,
                enob_bits,
                snr_db,
                sfdr_db,
                sfdr_spur_bin,
                sfdr_spur_frequency,
                largest_harmonics: largest_harmonics
                    .into_iter()
                    .map(|harmonic| harmonic.inner)
                    .collect(),
            },
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (f64, f64, f64, f64, f64, f64, f64),
            (Option<usize>, Option<f64>),
            Vec<PyTransientFftHarmonic>,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.inner.fundamental_magnitude,
                    self.inner.thd_ratio,
                    self.inner.thd_db,
                    self.inner.sndr_db,
                    self.inner.enob_bits,
                    self.inner.snr_db,
                    self.inner.sfdr_db,
                ),
                (self.inner.sfdr_spur_bin, self.inner.sfdr_spur_frequency),
                self.largest_harmonics(),
            ),
        ))
    }
}

#[pyclass(name = "FftResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyTransientFftResult {
    pub(crate) inner: rspice_core::engine::TransientFftResult,
    /// The transient this spectrum post-processed, which the shared document
    /// records as the spectrum's parent analysis.
    evidence: Option<DocumentEvidence<rspice_core::execution::AnalysisInstanceId>>,
}

impl CarriesDocumentEvidence for PyTransientFftResult {
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

impl PyTransientFftResult {
    /// The shared result document, projected from the retained spectrum.
    ///
    /// The coefficient unit is core's own `.FFT` rule, so a normalized
    /// spectrum is a ratio here exactly as it is on every other surface.
    fn shared_document(&self, py: Python<'_>) -> PyResult<AnalysisResultDocument> {
        let evidence = document::evidence(&self.evidence, "FFT")?;
        let coordinate = evidence.coordinate.clone();
        let analysis = evidence.analysis;
        let parent = evidence.core;
        let unit = rspice_core::execution::transient_fft_output_unit(
            self.inner.physical_type,
            self.inner.format,
        )
        .map_err(crate::errors::simulation_error_to_pyerr)?;
        document::build(py, coordinate, || {
            AnalysisResultDocument::from_transient_fft(analysis, parent, unit, &self.inner)
        })
    }
}

impl From<&rspice_core::engine::TransientFftResult> for PyTransientFftResult {
    fn from(inner: &rspice_core::engine::TransientFftResult) -> Self {
        Self {
            inner: inner.clone(),
            evidence: Some(DocumentEvidence::sole(
                rspice_core::execution::AnalysisKind::Fft,
                rspice_core::execution::sole_analysis_identity(
                    rspice_core::execution::AnalysisKind::Tran,
                ),
            )),
        }
    }
}

#[pymethods]
impl PyTransientFftResult {
    #[getter]
    fn source_kind(&self) -> &'static str {
        match self.inner.output {
            rspice_core::netlist::FftOutput::Probe(_) => "probe",
            rspice_core::netlist::FftOutput::Expression(_) => "expression",
        }
    }

    #[getter]
    fn source(&self) -> String {
        match &self.inner.output {
            rspice_core::netlist::FftOutput::Probe(value)
            | rspice_core::netlist::FftOutput::Expression(value) => value.clone(),
        }
    }

    #[getter]
    fn output_name(&self) -> String {
        self.inner.output_name.clone()
    }

    #[getter]
    fn physical_type(&self) -> &'static str {
        self.inner.physical_type
    }

    /// Effective unit of the complex coefficients, magnitudes, and
    /// magnitude-like metrics. Normalization makes the values dimensionless
    /// without erasing the source quantity reported by `physical_type`.
    #[getter]
    fn value_unit(&self) -> PyResult<Option<&'static str>> {
        fft_value_unit(self.inner.physical_type, self.inner.format)
            .map_err(crate::errors::value_error)
    }

    #[getter]
    fn start_time(&self) -> f64 {
        self.inner.start_time
    }

    #[getter]
    fn stop_time(&self) -> f64 {
        self.inner.stop_time
    }

    #[getter]
    fn sample_interval(&self) -> f64 {
        self.inner.sample_interval
    }

    #[getter]
    fn point_count(&self) -> usize {
        self.inner.point_count
    }

    #[getter]
    fn accurate_sampling(&self) -> bool {
        self.inner.accurate_sampling
    }

    #[getter]
    fn format(&self) -> &'static str {
        match self.inner.format {
            rspice_core::netlist::FftFormat::Normalized => "normalized",
            rspice_core::netlist::FftFormat::Unnormalized => "unnormalized",
        }
    }

    #[getter]
    fn mode(&self) -> &'static str {
        match self.inner.mode {
            rspice_core::netlist::XyceFftMode::HspiceCompatible => "hspice_compatible",
            rspice_core::netlist::XyceFftMode::SpectreCompatible => "spectre_compatible",
        }
    }

    #[getter]
    fn window(&self) -> &'static str {
        use rspice_core::netlist::FftWindow;
        match self.inner.window {
            FftWindow::Rectangular => "rectangular",
            FftWindow::Bartlett => "bartlett",
            FftWindow::BartlettHann => "bartlett_hann",
            FftWindow::Hamming => "hamming",
            FftWindow::Hann => "hann",
            FftWindow::Blackman67Db => "blackman_67db",
            FftWindow::Blackman => "blackman",
            FftWindow::BlackmanHarris => "blackman_harris",
            FftWindow::Nuttall => "nuttall",
            FftWindow::HalfCycleSine => "half_cycle_sine",
            FftWindow::HalfCycleSine3 => "half_cycle_sine_3",
            FftWindow::HalfCycleSine6 => "half_cycle_sine_6",
            FftWindow::Cosine2 => "cosine_2",
            FftWindow::Cosine4 => "cosine_4",
        }
    }

    #[getter]
    fn window_name(&self) -> String {
        self.inner.window_name.clone()
    }

    #[getter]
    fn alpha(&self) -> f64 {
        self.inner.alpha
    }

    #[getter]
    fn coherent_gain(&self) -> f64 {
        self.inner.coherent_gain
    }

    #[getter]
    fn frequency_resolution(&self) -> f64 {
        self.inner.frequency_resolution
    }

    #[getter]
    fn fundamental_bin(&self) -> usize {
        self.inner.fundamental_bin
    }

    #[getter]
    fn minimum_metric_bin(&self) -> usize {
        self.inner.minimum_metric_bin
    }

    #[getter]
    fn maximum_metric_bin(&self) -> usize {
        self.inner.maximum_metric_bin
    }

    #[getter]
    fn bins(&self) -> Vec<PyTransientFftBin> {
        self.inner
            .bins
            .iter()
            .map(PyTransientFftBin::from)
            .collect()
    }

    fn bin(&self, index: usize) -> PyResult<PyTransientFftBin> {
        self.inner
            .bins
            .get(index)
            .map(PyTransientFftBin::from)
            .ok_or_else(|| {
                crate::errors::index_error(format!(
                    "FFT bin index {index} out of range (0..{})",
                    self.inner.bins.len()
                ))
            })
    }

    #[getter]
    fn frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .bins
            .iter()
            .map(|bin| bin.frequency)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn complex_bins<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.inner
            .bins
            .iter()
            .map(|bin| rspice_core::Complex64::new(bin.real, bin.imaginary))
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn magnitudes<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .bins
            .iter()
            .map(|bin| bin.magnitude)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn phases_degrees<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner
            .bins
            .iter()
            .map(|bin| bin.phase_degrees)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    #[getter]
    fn metrics(&self) -> Option<PyTransientFftMetrics> {
        self.inner.metrics.as_ref().map(PyTransientFftMetrics::from)
    }

    fn __repr__(&self) -> String {
        format!(
            "FftResult(source='{}', points={}, bins={}, window='{}', format='{}')",
            self.inner.output_name,
            self.inner.point_count,
            self.inner.bins.len(),
            self.window(),
            self.format()
        )
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

    #[staticmethod]
    fn _unpickle(state: TransientFftResultState) -> PyResult<Self> {
        rebuild_transient_fft_result(state).map(|inner| Self {
            inner,
            evidence: None,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, (TransientFftResultState,))> {
        Ok((
            unpickler::<Self>(py)?,
            (transient_fft_result_state(&self.inner)?,),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_core::netlist::FftFormat::{Normalized, Unnormalized};

    #[test]
    fn value_units_preserve_quantity_provenance_and_transform_semantics() {
        assert_eq!(fft_value_unit("voltage", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("current", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("parameter", Normalized).unwrap(), Some("1"));
        assert_eq!(fft_value_unit("voltage", Unnormalized).unwrap(), Some("V"));
        assert_eq!(fft_value_unit("current", Unnormalized).unwrap(), Some("A"));
        assert_eq!(fft_value_unit("parameter", Unnormalized).unwrap(), None);
        assert!(fft_value_unit("unsupported", Normalized).is_err());
    }
}
