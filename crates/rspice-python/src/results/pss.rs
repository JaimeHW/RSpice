//! Periodic steady-state results.
//!
//! The time-domain periodic solution: one period sampled on the shooting grid,
//! plus the spectra derived from it. Data is addressable by harmonic as well as
//! by node, which is what separates it from a plain transient result.

use super::*;

/// Numerical qualification certificate for one complete Floquet spectrum.
#[pyclass(name = "FloquetSpectrumCertificate", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFloquetSpectrumCertificate {
    #[pyo3(get)]
    pub problem_order: usize,
    #[pyo3(get)]
    pub max_backward_error: f64,
    #[pyo3(get)]
    pub qualification_tolerance: f64,
}

impl PyFloquetSpectrumCertificate {
    fn from_core(certificate: &rspice_core::analysis::FloquetSpectrumCertificate) -> Self {
        Self {
            problem_order: certificate.problem_order,
            max_backward_error: certificate.max_backward_error,
            qualification_tolerance: certificate.qualification_tolerance,
        }
    }

    fn to_state(&self) -> FloquetSpectrumCertificateState {
        (
            self.problem_order,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }
}

#[pymethods]
impl PyFloquetSpectrumCertificate {
    /// Whether the certificate satisfies the canonical strict threshold.
    #[getter]
    fn is_strictly_qualified(&self) -> bool {
        rspice_core::analysis::FloquetSpectrumCertificate::new(
            self.problem_order,
            self.max_backward_error,
            self.qualification_tolerance,
        )
        .is_some()
    }

    fn __repr__(&self) -> String {
        format!(
            "FloquetSpectrumCertificate(order={}, max_backward_error={:.3e}, tolerance={:.3e})",
            self.problem_order, self.max_backward_error, self.qualification_tolerance,
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        problem_order: usize,
        max_backward_error: f64,
        qualification_tolerance: f64,
    ) -> PyResult<Self> {
        let certificate = floquet_spectrum_certificate_from_state((
            problem_order,
            max_backward_error,
            qualification_tolerance,
        ))?;
        Ok(Self::from_core(&certificate))
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, FloquetSpectrumCertificateState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}

/// Completeness and numerical provenance for the retained Floquet multipliers.
#[pyclass(name = "FloquetSpectrumEvidence", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyFloquetSpectrumEvidence {
    #[pyo3(get)]
    pub kind: String,
    certificate: Option<PyFloquetSpectrumCertificate>,
}

impl PyFloquetSpectrumEvidence {
    fn from_core(evidence: &rspice_core::analysis::FloquetSpectrumEvidence) -> PyResult<Self> {
        let (kind, certificate) = floquet_spectrum_evidence_state(evidence)?;
        Ok(Self {
            kind,
            certificate: certificate.map(
                |(problem_order, max_backward_error, qualification_tolerance)| {
                    PyFloquetSpectrumCertificate {
                        problem_order,
                        max_backward_error,
                        qualification_tolerance,
                    }
                },
            ),
        })
    }

    fn to_state(&self) -> FloquetSpectrumEvidenceState {
        (
            self.kind.clone(),
            self.certificate
                .as_ref()
                .map(PyFloquetSpectrumCertificate::to_state),
        )
    }
}

#[pymethods]
impl PyFloquetSpectrumEvidence {
    #[getter]
    fn certificate(&self) -> Option<PyFloquetSpectrumCertificate> {
        self.certificate.clone()
    }

    #[getter]
    fn is_qualified(&self) -> bool {
        matches!(self.kind.as_str(), "qualified" | "no_dynamic_modes")
    }

    fn __repr__(&self) -> String {
        format!(
            "FloquetSpectrumEvidence(kind='{}', certificate={})",
            self.kind,
            if self.certificate.is_some() {
                "present"
            } else {
                "None"
            }
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    fn _unpickle(
        kind: String,
        certificate: Option<FloquetSpectrumCertificateState>,
    ) -> PyResult<Self> {
        let evidence = floquet_spectrum_evidence_from_state((kind, certificate))?;
        Self::from_core(&evidence)
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, FloquetSpectrumEvidenceState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}

fn pss_stability_verdict_label(
    verdict: rspice_core::analysis::FloquetStabilityVerdict,
) -> PyResult<&'static str> {
    use rspice_core::analysis::FloquetStabilityVerdict;

    match verdict {
        FloquetStabilityVerdict::Stable => Ok("stable"),
        FloquetStabilityVerdict::Unstable => Ok("unstable"),
        FloquetStabilityVerdict::Marginal => Ok("marginal"),
        FloquetStabilityVerdict::Indeterminate => Ok("indeterminate"),
        _ => Err(crate::errors::value_error(
            "unsupported Floquet stability verdict".to_string(),
        )),
    }
}

fn validate_pickled_pss_result(result: &rspice_core::analysis::PssResult) -> PyResult<()> {
    if !result.period.is_finite()
        || !result.frequency.is_finite()
        || !result.residual_norm.is_finite()
        || result.time.iter().any(|value| !value.is_finite())
        || result
            .floquet_multipliers
            .iter()
            .any(|value| !value.re.is_finite() || !value.im.is_finite())
    {
        return Err(crate::errors::value_error(
            "pickled PSS result contains non-finite values".to_string(),
        ));
    }
    if result.node_names.len() != result.waveforms.len()
        || result
            .waveforms
            .iter()
            .any(|waveform| waveform.values.len() != result.time.len())
    {
        return Err(crate::errors::value_error(
            "pickled PSS result has inconsistent node, waveform, or time cardinality".to_string(),
        ));
    }
    if result
        .waveforms
        .iter()
        .flat_map(|waveform| &waveform.values)
        .any(|value| !value.is_finite())
    {
        return Err(crate::errors::value_error(
            "pickled PSS result contains a non-finite waveform value".to_string(),
        ));
    }
    if !result.has_consistent_floquet_contract() {
        return Err(crate::errors::value_error(
            "pickled PSS result has inconsistent Floquet evidence, cardinality, or orbit policy"
                .to_string(),
        ));
    }
    Ok(())
}

/// Periodic steady-state waveform and convergence diagnostics.
#[pyclass(name = "PssResult", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyPssResult {
    inner: rspice_core::analysis::PssResult,
    #[pyo3(get)]
    pub num_harmonics: usize,
    #[pyo3(get)]
    pub iterations: usize,
    #[pyo3(get)]
    pub residual_norm: f64,
    #[pyo3(get)]
    pub period: f64,
}

impl PyPssResult {
    pub fn from_core(
        result: &rspice_core::engine::PssAnalysisResult,
        num_harmonics: usize,
    ) -> Self {
        Self {
            inner: result.result.clone(),
            num_harmonics,
            iterations: result.iterations,
            residual_norm: result.final_residual,
            period: result.period,
        }
    }

    fn floquet_contract_state(&self) -> PyResult<PssFloquetContractState> {
        Ok((
            PSS_FLOQUET_CONTRACT_STATE_VERSION,
            floquet_spectrum_evidence_state(&self.inner.floquet_evidence)?,
            floquet_orbit_kind_state(self.inner.floquet_orbit_kind)?,
            self.inner.trivial_floquet_multiplier_index,
        ))
    }

    fn waveform_index(&self, node: &NodeIdentifier) -> PyResult<Option<usize>> {
        match node {
            NodeIdentifier::Index(0) => Ok(None),
            NodeIdentifier::Index(index) => {
                let waveform_index = index - 1;
                if waveform_index < self.inner.waveforms.len() {
                    Ok(Some(waveform_index))
                } else {
                    Err(invalid_node_index_error(*index, self.inner.waveforms.len()).into())
                }
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

    fn harmonic_components(
        &self,
        node: &NodeIdentifier,
    ) -> PyResult<Vec<rspice_core::analysis::HarmonicComponent>> {
        match self.waveform_index(node)? {
            Some(index) => Ok(self.inner.harmonics(index + 1, self.num_harmonics)),
            None => Ok((0..=self.num_harmonics)
                .map(|harmonic_number| rspice_core::analysis::HarmonicComponent {
                    harmonic_number,
                    frequency: harmonic_number as f64 * self.inner.frequency,
                    magnitude: 0.0,
                    phase: 0.0,
                })
                .collect()),
        }
    }
}

#[pymethods]
impl PyPssResult {
    #[getter]
    fn time<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        self.inner.time.to_pyarray(py)
    }

    #[getter]
    fn frequency(&self) -> f64 {
        self.inner.frequency
    }

    #[getter]
    fn node_names(&self) -> Vec<String> {
        self.inner.node_names.clone()
    }

    #[getter]
    fn period_detected(&self) -> bool {
        self.inner.period_detected
    }

    #[getter]
    fn floquet_multipliers<'py>(
        &self,
        py: Python<'py>,
    ) -> Bound<'py, PyArray1<rspice_core::Complex64>> {
        self.inner.floquet_multipliers.to_pyarray(py)
    }

    /// Completeness and numerical evidence for the full multiplier vector.
    #[getter]
    fn floquet_evidence(&self) -> PyResult<PyFloquetSpectrumEvidence> {
        PyFloquetSpectrumEvidence::from_core(&self.inner.floquet_evidence)
    }

    /// Periodic-orbit policy used to interpret a unity multiplier.
    #[getter]
    fn floquet_orbit_kind(&self) -> PyResult<String> {
        floquet_orbit_kind_state(self.inner.floquet_orbit_kind)
    }

    /// Qualified autonomous phase-mode index, when one was selected.
    #[getter]
    fn trivial_floquet_multiplier_index(&self) -> Option<usize> {
        self.inner.trivial_floquet_multiplier_index
    }

    /// Evidence-aware four-state Floquet stability verdict.
    #[getter]
    fn stability_verdict(&self) -> PyResult<String> {
        Ok(pss_stability_verdict_label(self.inner.stability_verdict())?.to_string())
    }

    /// Stable/unstable convenience value. Marginal and indeterminate results
    /// deliberately remain unknown rather than collapsing to false.
    #[getter]
    fn is_stable(&self) -> PyResult<Option<bool>> {
        use rspice_core::analysis::FloquetStabilityVerdict;

        match self.inner.stability_verdict() {
            FloquetStabilityVerdict::Stable => Ok(Some(true)),
            FloquetStabilityVerdict::Unstable => Ok(Some(false)),
            FloquetStabilityVerdict::Marginal | FloquetStabilityVerdict::Indeterminate => Ok(None),
            _ => Err(crate::errors::value_error(
                "unsupported Floquet stability verdict".to_string(),
            )),
        }
    }

    #[getter]
    fn num_nodes(&self) -> usize {
        self.inner.waveforms.len()
    }

    #[getter]
    fn num_points(&self) -> usize {
        self.inner.time.len()
    }

    fn voltage_waveform<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = match self.waveform_index(&node)? {
            Some(index) => self.inner.waveforms[index].values.clone(),
            None => vec![0.0; self.inner.time.len()],
        };
        Ok(values.to_pyarray(py))
    }

    fn voltage_at(&self, node: NodeIdentifier, time: f64) -> PyResult<f64> {
        if !time.is_finite() {
            return Err(crate::errors::value_error("time must be finite"));
        }
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].interpolate(
                &self.inner.time,
                time,
                self.inner.period,
            )),
            None => Ok(0.0),
        }
    }

    fn dc(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].dc(&self.inner.time, self.inner.period)),
            None => Ok(0.0),
        }
    }

    fn peak_to_peak(&self, node: NodeIdentifier) -> PyResult<f64> {
        match self.waveform_index(&node)? {
            Some(index) => Ok(self.inner.waveforms[index].peak_to_peak()),
            None => Ok(0.0),
        }
    }

    /// Frequencies of the configured DC-through-N harmonic spectrum.
    #[getter]
    fn harmonic_frequencies<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f64>> {
        (0..=self.num_harmonics)
            .map(|harmonic| harmonic as f64 * self.inner.frequency)
            .collect::<Vec<_>>()
            .to_pyarray(py)
    }

    /// Complex peak phasors for a node's configured harmonic spectrum.
    fn harmonic_coefficients<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<rspice_core::Complex64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| {
                rspice_core::Complex64::from_polar(
                    component.magnitude,
                    component.phase.to_radians(),
                )
            })
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn harmonic_magnitude<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| component.magnitude.abs())
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    fn harmonic_phase_degrees<'py>(
        &self,
        py: Python<'py>,
        node: NodeIdentifier,
    ) -> PyResult<Bound<'py, PyArray1<f64>>> {
        let values = self
            .harmonic_components(&node)?
            .into_iter()
            .map(|component| component.phase)
            .collect::<Vec<_>>();
        Ok(values.to_pyarray(py))
    }

    /// Non-DC harmonic records, beginning with the fundamental.
    fn harmonics(&self, node: NodeIdentifier) -> PyResult<Vec<PyHarmonic>> {
        Ok(self
            .harmonic_components(&node)?
            .into_iter()
            .filter(|component| component.harmonic_number > 0)
            .map(|component| PyHarmonic {
                n: component.harmonic_number,
                frequency: component.frequency,
                magnitude: component.magnitude,
                phase: component.phase.to_radians(),
            })
            .collect())
    }

    fn thd_percent(&self, node: NodeIdentifier) -> PyResult<f64> {
        let components = self.harmonic_components(&node)?;
        let Some(fundamental) = components.get(1).map(|component| component.magnitude.abs()) else {
            return Ok(0.0);
        };
        if fundamental <= f64::EPSILON {
            return Ok(0.0);
        }
        let distortion_power = components
            .iter()
            .skip(2)
            .map(|component| component.magnitude * component.magnitude)
            .sum::<f64>();
        Ok(100.0 * distortion_power.sqrt() / fundamental)
    }

    fn __repr__(&self) -> String {
        format!(
            "PssResult(frequency={:.6e}Hz, harmonics={}, nodes={}, points={}, iterations={}, residual={:.3e})",
            self.inner.frequency,
            self.num_harmonics,
            self.inner.waveforms.len(),
            self.inner.time.len(),
            self.iterations,
            self.residual_norm
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    ///
    /// The orbit group is the converged periodic solution; the diagnostics
    /// group is what the shooting run reported around it.
    #[staticmethod]
    #[pyo3(signature = (orbit, time, waveforms, node_names, floquet_multipliers, diagnostics, floquet_contract=None))]
    fn _unpickle(
        orbit: (f64, f64, usize, f64, bool),
        time: Vec<f64>,
        waveforms: Vec<Vec<f64>>,
        node_names: Vec<String>,
        floquet_multipliers: Vec<(f64, f64)>,
        diagnostics: (usize, usize, f64, f64, bool),
        floquet_contract: Option<PssFloquetContractState>,
    ) -> PyResult<Self> {
        let (period, frequency, iterations, residual_norm, period_detected) = orbit;
        let (num_harmonics, run_iterations, run_residual, run_period, _legacy_is_stable) =
            diagnostics;
        let (floquet_evidence, floquet_orbit_kind, trivial_floquet_multiplier_index) =
            match floquet_contract {
                Some((version, evidence, orbit_kind, trivial_index)) => {
                    if version != PSS_FLOQUET_CONTRACT_STATE_VERSION {
                        return Err(crate::errors::value_error(format!(
                            "unsupported pickled PSS Floquet contract version {version}"
                        )));
                    }
                    (
                        floquet_spectrum_evidence_from_state(evidence)?,
                        floquet_orbit_kind_from_state(&orbit_kind)?,
                        trivial_index,
                    )
                }
                None => (
                    rspice_core::analysis::FloquetSpectrumEvidence::LegacyUnknown,
                    if period_detected {
                        rspice_core::analysis::FloquetOrbitKind::Autonomous
                    } else {
                        rspice_core::analysis::FloquetOrbitKind::Driven
                    },
                    None,
                ),
            };
        let inner = rspice_core::analysis::PssResult {
            period,
            frequency,
            iterations,
            residual_norm,
            time,
            waveforms: waveforms
                .into_iter()
                .map(rspice_core::analysis::PeriodicWaveform::from_values)
                .collect(),
            node_names,
            period_detected,
            floquet_multipliers: complex_from_state(floquet_multipliers),
            floquet_evidence,
            floquet_orbit_kind,
            trivial_floquet_multiplier_index,
        };
        validate_pickled_pss_result(&inner)?;
        Ok(Self {
            inner,
            num_harmonics,
            iterations: run_iterations,
            residual_norm: run_residual,
            period: run_period,
        })
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(
        Bound<'py, PyAny>,
        (
            (f64, f64, usize, f64, bool),
            Vec<f64>,
            Vec<Vec<f64>>,
            Vec<String>,
            Vec<(f64, f64)>,
            (usize, usize, f64, f64, bool),
            PssFloquetContractState,
        ),
    )> {
        Ok((
            unpickler::<Self>(py)?,
            (
                (
                    self.inner.period,
                    self.inner.frequency,
                    self.inner.iterations,
                    self.inner.residual_norm,
                    self.inner.period_detected,
                ),
                self.inner.time.clone(),
                self.inner
                    .waveforms
                    .iter()
                    .map(|waveform| waveform.values.clone())
                    .collect(),
                self.inner.node_names.clone(),
                complex_state(&self.inner.floquet_multipliers),
                (
                    self.num_harmonics,
                    self.iterations,
                    self.residual_norm,
                    self.period,
                    self.inner.is_stable(),
                ),
                self.floquet_contract_state()?,
            ),
        ))
    }
}
