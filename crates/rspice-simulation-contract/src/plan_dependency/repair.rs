//! Portable prerequisite synthesis against an authenticated source-admission view.

use crate::analysis_draft::AnalysisDraft;
use crate::analysis_kind::AnalysisKind;
use crate::drafts::TranSetup;
use crate::pss_draft::PssDialogState;

use super::{dependency_configuration_issue, fourier_requirement, periodic_state_requirement};

/// Circuit answers needed by plan repair without owning source elaboration.
///
/// The application authenticates and validates the exact circuit source set;
/// plan transactions consume this borrowed view and remain atomic on refusal.
pub trait PlanDependencySourceContext {
    fn periodic_sources(&self) -> Result<&[String], String>;
    fn availability_error(&self) -> Option<&str>;
    fn validate_pss_sources(&self, draft: &PssDialogState) -> Result<(), String>;
}

pub fn prerequisite_draft_for(
    dependent: &AnalysisDraft,
    prerequisite: AnalysisKind,
    context: &impl PlanDependencySourceContext,
) -> Result<AnalysisDraft, String> {
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fourier(fourier) = dependent
    {
        let requirement = fourier_requirement(fourier)
            .map_err(|detail| format!("Fourier configuration is invalid: {detail}"))?;
        let required_interval = requirement.required_sample_interval()?;
        // Preserve margin against text round-tripping and future solver output
        // interpolation by targeting 10 samples for every highest-basis cycle.
        let interval = required_interval * 0.8;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{:.12e}", requirement.stop_time),
            step: format!("{interval:.12e}"),
            start: format!("{:.12e}", requirement.start_time),
            max_step: format!("{interval:.12e}"),
            uic: false,
        }));
    }
    if prerequisite == AnalysisKind::Transient
        && let AnalysisDraft::Fft(fft) = dependent
    {
        let request = fft
            .to_request()
            .map_err(|detail| format!("FFT configuration is invalid: {detail}"))?;
        // Only an authored STOP can size a transient. Without one the card
        // takes the transient's own stop time, and the default transient is
        // exactly the run the author has not yet constrained.
        let Some(stop) = request.stop else {
            return Ok(AnalysisDraft::for_kind(prerequisite));
        };
        let start = request.start.unwrap_or(0.0);
        let step = (stop - start) / request.points as f64;
        return Ok(AnalysisDraft::Transient(TranSetup {
            stop: format!("{stop:.12e}"),
            step: format!("{step:.12e}"),
            start: format!("{:.12e}", 0.0),
            max_step: format!("{step:.12e}"),
            uic: false,
        }));
    }
    if prerequisite == AnalysisKind::Pss {
        if matches!(
            dependent,
            AnalysisDraft::Pac(_)
                | AnalysisDraft::Pnoise(_)
                | AnalysisDraft::Pxf(_)
                | AnalysisDraft::Pstb(_)
        ) {
            periodic_state_requirement(dependent)?;
        }
        let sources = context.periodic_sources()?;
        let draft = PssDialogState {
            tone_sources: sources.join(", "),
            ..Default::default()
        };
        context.validate_pss_sources(&draft)?;
        let draft = AnalysisDraft::Pss(draft);
        if let Some(issue) = dependency_configuration_issue(dependent, &draft) {
            return Err(issue.detail().to_owned());
        }
        return Ok(draft);
    }
    let draft = AnalysisDraft::for_kind(prerequisite);
    // A synthesized carrier is refused here for the same reason a chosen one
    // is: a repair that inserted it would leave the plan holding a dependency
    // its own contract rejects, with no further repair to offer.
    if prerequisite == AnalysisKind::HarmonicBalance
        && let Some(issue) = dependency_configuration_issue(dependent, &draft)
    {
        return Err(issue.detail().to_owned());
    }
    Ok(draft)
}

pub fn dependency_candidate_context_issue(
    prerequisite: AnalysisKind,
    candidate: &AnalysisDraft,
    context: &impl PlanDependencySourceContext,
) -> Option<String> {
    if prerequisite != AnalysisKind::Pss {
        return None;
    }
    let AnalysisDraft::Pss(pss) = candidate else {
        return Some("the prerequisite does not contain a PSS draft".to_owned());
    };
    context.validate_pss_sources(pss).err()
}
