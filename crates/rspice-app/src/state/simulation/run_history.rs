//! Integration checks for retained history revision tracking.

use super::{RunHistory, SimulationRun};
use std::sync::Arc;

#[cfg(test)]
mod tests {
    use super::*;

    fn history() -> RunHistory {
        vec![SimulationRun::new(1), SimulationRun::new(2)].into()
    }

    #[test]
    fn nested_sample_edits_invalidate_history_without_a_manual_version_bump() {
        let mut runs = history();
        runs[0].add_analysis(
            super::super::AnalysisResult::new(
                1,
                super::super::AnalysisType::Transient,
                "Transient",
            )
            .with_waveforms(vec![super::super::WaveformData::new(
                "V(out)",
                vec![0.0, 1.0],
                vec![1.0, 2.0],
                "#ffffff",
            )]),
        );
        let revision = runs.revision();
        Arc::make_mut(&mut runs[0].analyses[0].waveforms[0].y)[1] = 3.0;
        assert_ne!(runs.revision(), revision);
    }
}
