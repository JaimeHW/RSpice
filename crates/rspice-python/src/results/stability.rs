//! Loop stability, pole-zero, and transfer-function results.
//!
//! `StbResult` carries the Tian loop-gain probe (`.STB`), which measures a
//! feedback loop without breaking it. `PoleZeroResult` carries the roots of the
//! small-signal network, and `TransferFunctionResult` the DC transfer gain with
//! its input and output resistances. Measured loop margins alone do not
//! establish closed-loop stability.

use super::*;

mod pole_zero;
mod stb;
mod transfer_function;

pub(crate) use pole_zero::PyPoleZeroResult;
pub(crate) use stb::PyStbResult;
pub(crate) use transfer_function::PyTransferFunctionResult;

/// Numerical qualification certificate for a complete eigenspectrum.
#[pyclass(name = "SpectrumCertificate", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PySpectrumCertificate {
    #[pyo3(get)]
    pub problem_order: usize,
    #[pyo3(get)]
    pub infinite_count: usize,
    #[pyo3(get)]
    pub max_backward_error: f64,
    #[pyo3(get)]
    pub qualification_tolerance: f64,
    #[pyo3(get)]
    pub asymptotically_stable: Option<bool>,
}

impl PySpectrumCertificate {
    fn from_core(certificate: &rspice_core::analysis::SpectrumCertificate) -> Self {
        Self {
            problem_order: certificate.problem_order,
            infinite_count: certificate.infinite_count,
            max_backward_error: certificate.max_backward_error,
            qualification_tolerance: certificate.qualification_tolerance,
            asymptotically_stable: certificate.asymptotically_stable,
        }
    }

    fn to_state(&self) -> SpectrumCertificateState {
        SpectrumCertificateState::WithStability((
            self.problem_order,
            self.infinite_count,
            self.max_backward_error,
            self.qualification_tolerance,
            self.asymptotically_stable,
        ))
    }
}

#[pymethods]
impl PySpectrumCertificate {
    /// Number of finite roots certified by the finite/infinite accounting.
    #[getter]
    fn finite_count(&self) -> usize {
        self.problem_order.saturating_sub(self.infinite_count)
    }

    /// Whether the worst backward error satisfies the strict threshold.
    #[getter]
    fn is_strictly_qualified(&self) -> bool {
        self.max_backward_error <= self.qualification_tolerance
    }

    fn __repr__(&self) -> String {
        format!(
            "SpectrumCertificate(order={}, finite={}, infinite={}, max_backward_error={:.3e}, tolerance={:.3e})",
            self.problem_order,
            self.finite_count(),
            self.infinite_count,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    /// Rebuild from pickled state. Not part of the public API.
    #[staticmethod]
    #[pyo3(signature = (problem_order, infinite_count, max_backward_error, qualification_tolerance, asymptotically_stable=None))]
    fn _unpickle(
        problem_order: usize,
        infinite_count: usize,
        max_backward_error: f64,
        qualification_tolerance: f64,
        asymptotically_stable: Option<bool>,
    ) -> PyResult<Self> {
        let certificate =
            spectrum_certificate_from_state(SpectrumCertificateState::WithStability((
                problem_order,
                infinite_count,
                max_backward_error,
                qualification_tolerance,
                asymptotically_stable,
            )))?;
        Ok(Self::from_core(&certificate))
    }

    #[allow(clippy::type_complexity)]
    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, SpectrumCertificateState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}

/// Completeness and numerical evidence for one returned pole or zero set.
#[pyclass(name = "RootSetEvidence", module = "rspice", from_py_object)]
#[derive(Debug, Clone)]
pub struct PyRootSetEvidence {
    #[pyo3(get)]
    pub kind: String,
    certificate: Option<PySpectrumCertificate>,
}

impl PyRootSetEvidence {
    fn from_core(evidence: &rspice_core::analysis::RootSetEvidence) -> PyResult<Self> {
        let (kind, _) = root_set_evidence_state(evidence)?;
        Ok(Self {
            kind,
            certificate: evidence.certificate().map(PySpectrumCertificate::from_core),
        })
    }

    fn to_state(&self) -> RootSetEvidenceState {
        (
            self.kind.clone(),
            self.certificate
                .as_ref()
                .map(PySpectrumCertificate::to_state),
        )
    }
}

#[pymethods]
impl PyRootSetEvidence {
    #[getter]
    fn certificate(&self) -> Option<PySpectrumCertificate> {
        self.certificate.clone()
    }

    /// True only for a strictly qualified complete root set.
    #[getter]
    fn is_qualified(&self) -> bool {
        matches!(self.kind.as_str(), "qualified" | "qualified_empty")
    }

    fn __repr__(&self) -> String {
        format!(
            "RootSetEvidence(kind='{}', certificate={})",
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
    fn _unpickle(kind: String, certificate: Option<SpectrumCertificateState>) -> PyResult<Self> {
        let evidence = root_set_evidence_from_state((kind, certificate))?;
        Self::from_core(&evidence)
    }

    fn __reduce__<'py>(
        &self,
        py: Python<'py>,
    ) -> PyResult<(Bound<'py, PyAny>, RootSetEvidenceState)> {
        Ok((unpickler::<Self>(py)?, self.to_state()))
    }
}
