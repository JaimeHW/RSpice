//! Shared prepared execution for authored-output regression tests.
//! The fixture authorizes an immutable task and runs the real solver through
//! the production result-retention path; it does not synthesize solved data.

use super::*;
use crate::simulation::controller::QueuedAnalysis;
use crate::simulation::execution::SavePolicy;
use crate::state::{OutputSelectionMode, SimulationRun};

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

pub(super) fn run(
    deck: &str,
    label: &str,
    spec: AnalysisSpec,
    analysis_line: &str,
    outputs: &[SavedOutput],
    output_selection_mode: OutputSelectionMode,
) -> SimulationRun {
    crate::simulation::runner::pvt_point_evidence::run_declaration(
        deck,
        label,
        QueuedAnalysis {
            numeric_override: None,
            spec,
            config: None,
            spec_options: Default::default(),
            analysis_line: analysis_line.to_owned(),
        },
        27.0,
        SavePolicy::PlanOwned {
            output_selection_mode,
            retained_dataset_limit: 10,
            maximum_storage_bytes: u64::MAX,
            live_streaming_enabled: false,
            retain_failure_diagnostics: true,
        },
        outputs,
    )
    .expect("a real prepared declaration executes")
}
