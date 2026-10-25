//! Authored-deck execution followed by saved-output materialization and persistence.

pub use crate::simulation::controller::test_execution::saved_outputs::run;
use crate::state::{
    SavedOutput, SavedOutputCompatibility, SavedOutputKind, SavedOutputPolicy,
    SavedOutputPrecision, SavedOutputStreaming,
};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::FrequencySweep;

pub fn output(kind: SavedOutputKind, name: &str, expression: &str) -> SavedOutput {
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

pub fn ac() -> AnalysisSpec {
    AnalysisSpec::Ac {
        start_freq: 1.0,
        stop_freq: 10.0,
        points_per_unit: 3,
        sweep: FrequencySweep::Linear,
    }
}
