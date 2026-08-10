//! Resolving a specification's evidence from a retained dataset.
//!
//! One owner for the join between a named specification and the measurement
//! that supplies its value. The rules are deliberately one-directional: a
//! specification can only pass when a measurement of the same name succeeded
//! and its value is finite. No measurement, a failed one, or a non-finite
//! value are all "not a pass" — never a pass by omission.

/// The specification's bound, spelled the way the page shows it.
pub(super) fn specification_limit(spec: &crate::state::SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => {
            format!("{minimum:.6} … {maximum:.6} {}", spec.unit)
        }
        (Some(minimum), None) => format!("≥ {minimum:.6} {}", spec.unit),
        (None, Some(maximum)) => format!("≤ {maximum:.6} {}", spec.unit),
        (None, None) => "waveform · no scalar bound".to_owned(),
    }
}

/// The dataset a specification is judged against.
pub(super) fn selected_output_dataset(
    simulation: &crate::state::SimulationState,
) -> Option<&crate::state::SimulationRun> {
    simulation.active_run()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct OutputMeasurementEvidence {
    pub(super) value: f64,
    pub(super) measurement_passed: bool,
}

/// The most recent attributed measurement of this name in the dataset.
///
/// Only analyses that succeeded and carry provenance are considered: an
/// unattributed result cannot be traced to the configuration that produced it,
/// so it is not evidence.
pub(super) fn measurement_in_output_dataset(
    run: &crate::state::SimulationRun,
    name: &str,
) -> Option<OutputMeasurementEvidence> {
    run.analyses
        .iter()
        .rev()
        .filter(|analysis| analysis.success && analysis.provenance.is_some())
        .find_map(|analysis| {
            analysis
                .measurements
                .iter()
                .rev()
                .find(|measurement| measurement.name.eq_ignore_ascii_case(name))
                .and_then(|measurement| {
                    measurement
                        .value
                        .filter(|value| value.is_finite())
                        .map(|value| OutputMeasurementEvidence {
                            value,
                            measurement_passed: measurement.passed && measurement.error.is_none(),
                        })
                })
        })
}
