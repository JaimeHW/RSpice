//! The bounded failure vocabulary one executed directive can produce.
//!
//! Every variant here maps onto exactly one wire failure code in
//! [`crate::execute`]. Keeping the vocabulary in its own module means the
//! per-family runners in [`crate::family`] and the artifact publication in
//! [`crate::result_artifact`] share one refusal set instead of inventing a
//! string each.

use rspice_core::SimulationError;
use rspice_core::execution::ResultDocumentError;

/// Why one directive stopped without publishing its result.
pub(crate) enum DirectiveFailure {
    /// The engine itself refused, or the abort source fired.
    Engine(SimulationError),
    /// A retained value was not finite, so the run is outside the solver's
    /// validated range rather than merely unpleasant to serialize.
    NonFinite,
    /// The run retained more samples than one analysis may hold.
    SeriesBudget,
    /// The shared result document rejected the projection.
    ResultDocument(String),
    /// The planned artifact count exceeds the protocol ceiling.
    ResultArtifactLimit,
    /// One artifact exceeds the protocol per-file ceiling.
    ResultArtifactBytes,
    /// The aggregate retained result set exceeds the adapter memory budget.
    ResultSetBytes,
    /// The authored frequency grid is invalid or too large.
    FrequencyGrid(rspice_core::analysis::FrequencyGridError),
    /// The authored analysis configuration cannot be executed as written.
    InvalidAnalysis(String),
    /// The deck authors a form this build has no lossless mapping for.
    UnsupportedForm(String),
}

impl From<SimulationError> for DirectiveFailure {
    fn from(error: SimulationError) -> Self {
        Self::Engine(error)
    }
}

/// Map a shared-document error onto the adapter's refusal vocabulary.
///
/// Cancellation and byte-budget exhaustion are distinct operator outcomes, so
/// they keep their own codes instead of collapsing into "malformed document".
pub(crate) fn map_result_document_error(error: ResultDocumentError) -> DirectiveFailure {
    match error {
        ResultDocumentError::Aborted => DirectiveFailure::Engine(SimulationError::Aborted),
        ResultDocumentError::ArtifactTooLarge { .. } => DirectiveFailure::ResultSetBytes,
        other => DirectiveFailure::ResultDocument(other.to_string()),
    }
}

pub(crate) fn map_measurement_error(error: crate::measure::MeasurementError) -> DirectiveFailure {
    match error {
        crate::measure::MeasurementError::Aborted => {
            DirectiveFailure::Engine(SimulationError::Aborted)
        }
        crate::measure::MeasurementError::NonFinite => DirectiveFailure::NonFinite,
    }
}

/// Refuse when the abort source has already fired.
pub(crate) fn check_abort(
    abort: &dyn rspice_core::abort_signal::AbortSignal,
) -> Result<(), DirectiveFailure> {
    if abort.is_aborted() {
        Err(DirectiveFailure::Engine(SimulationError::Aborted))
    } else {
        Ok(())
    }
}
