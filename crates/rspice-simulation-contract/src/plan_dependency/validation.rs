//! Pure compatibility rules for exact analysis prerequisites.

use crate::analysis_draft::AnalysisDraft;
use crate::dependency_contract::{
    FourierTransientRequirement, PeriodicStateCapability, TransientCapability,
    validate_fourier_transient_contract, validate_harmonic_balance_carrier_contract,
    validate_periodic_state_contract,
};
use crate::drafts::TranSetup;
use crate::fourier_draft::FourierDialogState;
use crate::pnoise_draft::NoiseReferenceType;
use crate::spice_value::parse_spice_value_checked;

pub fn fourier_requirement(
    draft: &FourierDialogState,
) -> Result<FourierTransientRequirement, String> {
    let config = draft.to_config()?;
    Ok(FourierTransientRequirement {
        start_time: config.start_time,
        stop_time: config.stop_time,
        fundamental_freq: config.fundamental_freq,
        num_harmonics: config.num_harmonics,
    })
}

fn transient_capability(draft: &TranSetup) -> Result<TransientCapability, String> {
    let max_timestep = match draft.max_step.trim() {
        "" => None,
        value if value.eq_ignore_ascii_case("auto") => None,
        value => Some(parse_spice_value_checked(value)?),
    };
    Ok(TransientCapability {
        start_time: parse_spice_value_checked(&draft.start)?,
        stop_time: parse_spice_value_checked(&draft.stop)?,
        step_time: parse_spice_value_checked(&draft.step)?,
        max_timestep,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencyConfigurationIssue {
    InvalidDependent(String),
    InvalidPrerequisite(String),
    Incompatible(String),
}

impl DependencyConfigurationIssue {
    pub fn detail(&self) -> &str {
        match self {
            Self::InvalidDependent(detail)
            | Self::InvalidPrerequisite(detail)
            | Self::Incompatible(detail) => detail,
        }
    }
}

pub fn dependency_configuration_issue(
    dependent: &AnalysisDraft,
    prerequisite: &AnalysisDraft,
) -> Option<DependencyConfigurationIssue> {
    if matches!(
        dependent,
        AnalysisDraft::Pac(_)
            | AnalysisDraft::Pnoise(_)
            | AnalysisDraft::Pxf(_)
            | AnalysisDraft::Pstb(_)
    ) && let AnalysisDraft::Pss(pss) = prerequisite
    {
        let (consumer, require_autonomous) = match periodic_state_requirement(dependent) {
            Ok(requirement) => requirement,
            Err(detail) => return Some(DependencyConfigurationIssue::InvalidDependent(detail)),
        };
        let pss = match pss.to_config() {
            Ok(pss) => pss,
            Err(detail) => {
                return Some(DependencyConfigurationIssue::InvalidPrerequisite(format!(
                    "PSS configuration is invalid: {detail}"
                )));
            }
        };
        return validate_periodic_state_contract(
            consumer,
            PeriodicStateCapability {
                // A PSS this editor built is a shooting solve. The capability
                // stays a field rather than becoming a constant because a
                // *sealed* specification may still carry the retired HB-PSS
                // formulation, and `execution::artifact` judges that one; this
                // is the live draft, which has no way to ask for it.
                shooting: true,
                autonomous: pss.osc_mode,
            },
            require_autonomous,
        )
        .err()
        .map(DependencyConfigurationIssue::Incompatible);
    }

    // The same question asked of the other periodic carrier the engine
    // accepts. `.PSTB` is absent because it cannot reach here: it declares no
    // harmonic-balance role in any carrier position.
    if matches!(
        dependent,
        AnalysisDraft::Pac(_) | AnalysisDraft::Pnoise(_) | AnalysisDraft::Pxf(_)
    ) && matches!(prerequisite, AnalysisDraft::HarmonicBalance(_))
    {
        let (consumer, require_autonomous) = match periodic_state_requirement(dependent) {
            Ok(requirement) => requirement,
            Err(detail) => return Some(DependencyConfigurationIssue::InvalidDependent(detail)),
        };
        return validate_harmonic_balance_carrier_contract(consumer, require_autonomous)
            .err()
            .map(DependencyConfigurationIssue::Incompatible);
    }

    // The engine's own rule, before the run rather than during it: a card
    // whose STOP is past a transient's stop time fails that transient.
    if let (AnalysisDraft::Fft(fft), AnalysisDraft::Transient(transient)) =
        (dependent, prerequisite)
    {
        let request = match fft.to_request() {
            Ok(request) => request,
            Err(detail) => {
                return Some(DependencyConfigurationIssue::InvalidDependent(format!(
                    "FFT configuration is invalid: {detail}"
                )));
            }
        };
        let capability = match transient_capability(transient) {
            Ok(capability) => capability,
            Err(detail) => {
                return Some(DependencyConfigurationIssue::InvalidPrerequisite(format!(
                    "Transient configuration is invalid: {detail}"
                )));
            }
        };
        let stop = request.stop.unwrap_or(capability.stop_time);
        return (stop > capability.stop_time).then(|| {
            DependencyConfigurationIssue::Incompatible(format!(
                "STOP {stop} exceeds transient stop time {}",
                capability.stop_time
            ))
        });
    }

    let (AnalysisDraft::Fourier(fourier), AnalysisDraft::Transient(transient)) =
        (dependent, prerequisite)
    else {
        return None;
    };
    let requirement = match fourier_requirement(fourier) {
        Ok(requirement) => requirement,
        Err(detail) => {
            return Some(DependencyConfigurationIssue::InvalidDependent(format!(
                "Fourier configuration is invalid: {detail}"
            )));
        }
    };
    let capability = match transient_capability(transient) {
        Ok(capability) => capability,
        Err(detail) => {
            return Some(DependencyConfigurationIssue::InvalidPrerequisite(format!(
                "Transient configuration is invalid: {detail}"
            )));
        }
    };
    validate_fourier_transient_contract(requirement, capability)
        .err()
        .map(DependencyConfigurationIssue::Incompatible)
}

pub fn periodic_state_requirement(
    dependent: &AnalysisDraft,
) -> Result<(&'static str, bool), String> {
    match dependent {
        AnalysisDraft::Pac(draft) => draft
            .to_config()
            .map(|_| ("PAC", false))
            .map_err(|detail| format!("PAC configuration is invalid: {detail}")),
        AnalysisDraft::Pxf(draft) => draft
            .to_config()
            .map(|_| ("PXF", false))
            .map_err(|detail| format!("PXF configuration is invalid: {detail}")),
        AnalysisDraft::Pstb(draft) => draft
            .to_config()
            .map(|_| ("PSTB", false))
            .map_err(|detail| format!("PSTB configuration is invalid: {detail}")),
        AnalysisDraft::Pnoise(draft) => draft
            .to_config()
            .map(|config| ("PNOISE", config.noise_ref == NoiseReferenceType::Phase))
            .map_err(|detail| format!("PNOISE configuration is invalid: {detail}")),
        _ => Err(format!(
            "{} does not consume a PSS periodic-state prerequisite",
            dependent.kind().label()
        )),
    }
}
