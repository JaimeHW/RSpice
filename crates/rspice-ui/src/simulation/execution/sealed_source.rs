//! The one encoding under which a run's executable source is sealed.
//!
//! Preparation freezes the exact bytes it is about to hand the engine and
//! records their digest on the run receipt as
//! [`crate::state::PreparedRunReceipt::source_content_digest`]. That digest is
//! the only field of the receipt that authenticates deck *text*: the per-task
//! `config_digest` covers an analysis configuration, and the prepared-snapshot
//! digest covers the whole authorization — a graph, a policy set, and a target
//! — which no reader holding a deck can reconstruct.
//!
//! It is therefore also the only field a reader can check a retained deck
//! against, and checking it is the reason retained decks may be written to a
//! project file at all. So the domain separators live here rather than at the
//! sealing site: a verifier that spelled them a second time would drift from
//! the sealer silently, and every deck in the project would quietly stop
//! verifying.

use crate::product::ContentDigest;
use crate::state::AnalysisResultSourceDomain;

use super::canonical::content_digest;

/// Domain separator for the source a schematic-derived run executed.
const GENERATED_EXECUTABLE_SOURCE: &str = "rspice.generated-executable-source/v1";

/// Domain separator for the source a manual-deck run executed.
const MANUAL_EXECUTABLE_SOURCE: &str = "rspice.manual-executable-source/v1";

/// Seal the executable source a schematic-derived run will execute.
pub(in crate::simulation) fn generated_executable_source_digest(source: &str) -> ContentDigest {
    content_digest(GENERATED_EXECUTABLE_SOURCE, source.as_bytes())
}

/// Seal the executable source a manual-deck run will execute.
pub(in crate::simulation) fn manual_executable_source_digest(source: &str) -> ContentDigest {
    content_digest(MANUAL_EXECUTABLE_SOURCE, source.as_bytes())
}

/// Recompute a run receipt's sealed source digest over exact deck bytes.
///
/// `None` for a legacy-unclassified run: that domain names history recorded
/// before runs stated which kind of source they executed, and guessing a
/// domain for it would produce a digest that matches nothing and report a
/// tampered deck for a run nobody touched.
///
/// The result is a digest, never a verdict. What a mismatch means depends on
/// which point of the run is being read — a per-point corner source is not the
/// run-level source and was never sealed individually — and that judgment
/// belongs to the surface making the statement, not here.
#[must_use]
pub(crate) fn sealed_executable_source_digest(
    domain: AnalysisResultSourceDomain,
    source: &str,
) -> Option<ContentDigest> {
    match domain {
        AnalysisResultSourceDomain::SimulationPlan => {
            Some(generated_executable_source_digest(source))
        }
        AnalysisResultSourceDomain::ManualDeck => Some(manual_executable_source_digest(source)),
        AnalysisResultSourceDomain::LegacyUnclassified => None,
    }
}
