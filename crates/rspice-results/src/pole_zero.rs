//! Retained pole-zero qualification certificates and complete root-set accounting.

/// Exact finite/infinite accounting and residual certificate retained for one
/// computed pole or zero spectrum.
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PoleZeroSpectrumCertificate {
    pub problem_order: u64,
    pub infinite_count: u64,
    pub max_backward_error: f64,
    pub qualification_tolerance: f64,
}

impl PoleZeroSpectrumCertificate {
    #[must_use]
    pub fn canonical_qualification_tolerance(problem_order: u64) -> Option<f64> {
        let problem_order = usize::try_from(problem_order).ok()?;
        rspice_core::analysis::pole_zero::SpectrumCertificate::exact(problem_order, 0)
            .map(|certificate| certificate.qualification_tolerance)
    }

    fn as_core(self) -> Option<rspice_core::analysis::pole_zero::SpectrumCertificate> {
        rspice_core::analysis::pole_zero::SpectrumCertificate::new(
            usize::try_from(self.problem_order).ok()?,
            usize::try_from(self.infinite_count).ok()?,
            self.max_backward_error,
            self.qualification_tolerance,
        )
    }

    #[must_use]
    pub fn finite_count(self) -> Option<u64> {
        self.as_core()
            .and_then(|certificate| u64::try_from(certificate.finite_count()).ok())
    }

    #[must_use]
    pub fn is_strictly_qualified(self) -> bool {
        self.as_core()
            .is_some_and(|certificate| certificate.is_strictly_qualified())
    }
}

/// Qualification state attached to one retained pole or zero vector.
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum PoleZeroRootSetEvidence {
    NotRequested,
    QualifiedEmpty {
        certificate: PoleZeroSpectrumCertificate,
    },
    Qualified {
        certificate: PoleZeroSpectrumCertificate,
    },
    Approximate {
        certificate: PoleZeroSpectrumCertificate,
    },
    /// Truthful migration state for results written before certificates were
    /// retained. This state never proves stability.
    #[default]
    LegacyUnknown,
}

impl PoleZeroRootSetEvidence {
    #[must_use]
    pub const fn label(&self) -> &'static str {
        match self {
            Self::NotRequested => "not requested",
            Self::QualifiedEmpty { .. } => "qualified empty",
            Self::Qualified { .. } => "qualified",
            Self::Approximate { .. } => "approximate",
            Self::LegacyUnknown => "legacy unknown",
        }
    }

    #[must_use]
    pub const fn certificate(&self) -> Option<PoleZeroSpectrumCertificate> {
        match self {
            Self::QualifiedEmpty { certificate }
            | Self::Qualified { certificate }
            | Self::Approximate { certificate } => Some(*certificate),
            Self::NotRequested | Self::LegacyUnknown => None,
        }
    }

    #[must_use]
    pub const fn is_qualified(&self) -> bool {
        matches!(self, Self::QualifiedEmpty { .. } | Self::Qualified { .. })
    }

    #[must_use]
    pub fn is_consistent_with_count(&self, root_count: usize) -> bool {
        let Ok(root_count) = u64::try_from(root_count) else {
            return false;
        };
        match self {
            Self::NotRequested => root_count == 0,
            Self::QualifiedEmpty { certificate } => {
                root_count == 0
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == Some(0)
            }
            Self::Qualified { certificate } => {
                root_count > 0
                    && certificate.is_strictly_qualified()
                    && certificate.finite_count() == Some(root_count)
            }
            Self::Approximate { certificate } => certificate.as_core().is_some_and(|certificate| {
                !certificate.is_strictly_qualified()
                    && u64::try_from(certificate.finite_count()).ok() == Some(root_count)
            }),
            Self::LegacyUnknown => true,
        }
    }
}
