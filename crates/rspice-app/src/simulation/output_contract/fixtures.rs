//! Authored-deck execution followed by saved-output materialization and persistence.

use super::*;
pub(super) use crate::simulation::controller::test_execution::saved_outputs::run;

pub(super) fn output(kind: SavedOutputKind, name: &str, expression: &str) -> SavedOutput {
    SavedOutput::new(
        kind,
        name,
        expression,
        SavedOutputCompatibility::AllCompatibleAnalyses,
        SavedOutputPolicy::EveryAcceptedPoint,
        SavedOutputPrecision::FullSourcePrecision,
        SavedOutputStreaming::StoreOnly,
    )
    .unwrap()
}
