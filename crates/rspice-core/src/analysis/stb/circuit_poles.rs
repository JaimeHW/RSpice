//! Circuit modes are independent evidence from measured return-ratio margins.
use super::{StbAnalysisError, ensure_not_aborted, poll_abort};
use crate::analysis::pole_zero::{PoleSpectrum, StabilityVerdict};
use crate::{AbortSignal, ResourceLimits};

/// Why a circuit spectrum could not be retained alongside a valid loop sweep.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum CircuitPoleFailure {
    Unsupported {
        capability: String,
        detail: String,
    },
    Numerical {
        detail: String,
    },
    ResourceLimit {
        resource: String,
        requested: usize,
        limit: usize,
    },
}

/// Natural circuit poles in rad/s, computed at the same bias as the STB sweep.
/// Missing legacy evidence and failed extraction never imply an empty spectrum.
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(tag = "status", rename_all = "snake_case", deny_unknown_fields)]
pub enum CircuitPoleEvidence {
    #[default]
    NotComputed,
    Available {
        spectrum: PoleSpectrum,
    },
    Unavailable {
        cause: CircuitPoleFailure,
    },
}

impl CircuitPoleEvidence {
    pub fn is_not_computed(&self) -> bool {
        matches!(self, Self::NotComputed)
    }

    pub fn spectrum(&self) -> Option<&PoleSpectrum> {
        match self {
            Self::Available { spectrum } => Some(spectrum),
            _ => None,
        }
    }

    pub fn stability_verdict(&self) -> StabilityVerdict {
        self.spectrum().map_or(
            StabilityVerdict::Indeterminate,
            PoleSpectrum::stability_verdict,
        )
    }

    /// Five certificate slots, or four diagnostic slots without a spectrum.
    pub fn retained_value_count(&self) -> usize {
        self.spectrum()
            .map_or(4, |s| s.poles.len().saturating_mul(2).saturating_add(5))
    }

    pub fn diagnostic_bytes(&self) -> usize {
        match self {
            Self::Unavailable {
                cause: CircuitPoleFailure::Unsupported { capability, detail },
            } => capability.len().saturating_add(detail.len()),
            Self::Unavailable {
                cause: CircuitPoleFailure::Numerical { detail },
            } => detail.len(),
            Self::Unavailable {
                cause: CircuitPoleFailure::ResourceLimit { resource, .. },
            } => resource.len(),
            _ => 0,
        }
    }

    pub fn validate_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), StbAnalysisError> {
        ensure_not_aborted(abort)?;
        let invalid = || StbAnalysisError::InvalidSample {
            index: 0,
            reason: "invalid circuit-pole evidence",
        };
        if self.retained_value_count() > limits.max_result_values
            || self.diagnostic_bytes() > limits.max_external_data_bytes
        {
            return Err(StbAnalysisError::CapacityOverflow {
                object: "retained circuit-pole evidence",
            });
        }
        match self {
            Self::Available { spectrum } => {
                let certificate = spectrum.evidence.certificate().ok_or_else(invalid)?;
                if !spectrum.evidence.is_consistent_with(&spectrum.poles) {
                    return Err(invalid());
                }
                if certificate.problem_order > limits.max_matrix_unknowns {
                    return Err(StbAnalysisError::CapacityOverflow {
                        object: "retained circuit-pole descriptor",
                    });
                }
                for (index, pole) in spectrum.poles.iter().enumerate() {
                    poll_abort(abort, index)?;
                    if !pole.re.is_finite() || !pole.im.is_finite() {
                        return Err(invalid());
                    }
                }
            }
            Self::Unavailable { cause } => {
                let valid = match cause {
                    CircuitPoleFailure::Unsupported { capability, detail } => {
                        !capability.trim().is_empty() && !detail.trim().is_empty()
                    }
                    CircuitPoleFailure::Numerical { detail } => !detail.trim().is_empty(),
                    CircuitPoleFailure::ResourceLimit {
                        resource,
                        requested,
                        limit,
                    } => !resource.trim().is_empty() && requested > limit,
                };
                if !valid {
                    return Err(invalid());
                }
            }
            Self::NotComputed => {}
        }
        ensure_not_aborted(abort)
    }
}
