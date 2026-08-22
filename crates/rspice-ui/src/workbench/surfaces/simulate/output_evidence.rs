//! Resolving a specification's evidence from a retained dataset.
//!
//! One owner for the join between a named specification and the measurement
//! that supplies its value. The rules are deliberately one-directional: a
//! specification can only pass when a measurement of the same name succeeded
//! and its value is finite. No measurement, a failed one, or a non-finite
//! value are all "not a pass" — never a pass by omission.

/// A measured or authored quantity, spelled the way this workbench spells one
/// it is *showing* somebody.
///
/// Engineering notation rather than six fixed decimals. A bound of one
/// megahertz printed as `1000000.000000 Hz` claims six digits of precision the
/// author never wrote and buries the magnitude — the thing a reader checks —
/// in a run of zeros.
///
/// The display spelling, not the deck's. Every value this returns is printed
/// beside `spec.unit`, and beside a unit SPICE's `Meg` is not how anybody
/// writes a megahertz: `≥ 1.000 Meg Hz` has to be translated before it can be
/// held against a datasheet. `format_engineering_display` derives its digits
/// from the deck spelling, so the number is the same number and only the
/// prefix changes.
pub(super) fn quantity(value: f64) -> String {
    crate::state::property_types::format_engineering_display(value)
}

/// The specification's bound, spelled the way the page shows it.
pub(super) fn specification_limit(spec: &crate::state::SpecEntry) -> String {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => {
            format!(
                "{} … {} {}",
                quantity(minimum),
                quantity(maximum),
                spec.unit
            )
        }
        (Some(minimum), None) => format!("≥ {} {}", quantity(minimum), spec.unit),
        (None, Some(maximum)) => format!("≤ {} {}", quantity(maximum), spec.unit),
        (None, None) => "waveform · no scalar bound".to_owned(),
    }
}

/// The active retained dataset only when its frozen receipt belongs to the
/// active simulation plan.
///
/// Legacy datasets predate receipt ownership and remain usable for project
/// compatibility, but every current prepared run is fail-closed: a manual
/// deck or another plan can never become evidence merely because it is the
/// selected result.
pub(super) fn selected_plan_dataset(
    app: &crate::workbench::RSpiceApp,
) -> Option<&crate::state::SimulationRun> {
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())?;
    app.state.simulation.active_run_for_plan(plan_id)
}

/// The dataset a specification is judged against.
#[cfg(test)]
pub(super) fn selected_output_dataset(
    simulation: &crate::state::SimulationState,
) -> Option<&crate::state::SimulationRun> {
    simulation.active_run()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct OutputMeasurementEvidence {
    pub(super) value: f64,
    pub(super) measurement_passed: bool,
    /// How many attributed measurements of this name the dataset holds
    /// **within the specification's own scope**.
    ///
    /// One means the reported value is the scope's only answer. More than one
    /// means the scope swept — corners, temperatures, trials — and this value
    /// is a single point out of that set, not a summary of it. The page must
    /// say which, because a limit judged against one arbitrary point of a
    /// sweep is not a verdict on the sweep. Counting the whole dataset instead
    /// would overstate what a narrowed specification was judged against.
    pub(super) retained_measurements: usize,
}

impl OutputMeasurementEvidence {
    /// Whether the reported value stands for the whole dataset.
    pub(super) fn is_complete_coverage(&self) -> bool {
        self.retained_measurements <= 1
    }
}

/// The worst attributed measurement of this name in the dataset, with the
/// number of measurements it was chosen from.
///
/// Worst against the specification's own bound, not the most recent. A dataset
/// that holds several measurements of one name — the same measurement taken
/// under several analyses, or over a sweep once the runners attribute one per
/// point — has one verdict, and it is the verdict of its worst point. Reporting
/// the last one made a specification pass while a retained point failed it,
/// which is the one failure mode a limits table must not have.
///
/// Only analyses that succeeded and carry provenance are considered: an
/// unattributed result cannot be traced to the configuration that produced it,
/// so it is not evidence.
///
/// The specification's own point scope narrows the candidate set before the
/// worst of it is chosen, so a limit that claims to speak for one corner is
/// answered only by measurements the executor attributed to that corner.
#[cfg(test)]
pub(super) fn measurement_in_output_dataset(
    run: &crate::state::SimulationRun,
    spec: &crate::state::SpecEntry,
) -> Option<OutputMeasurementEvidence> {
    measurement_in_output_dataset_for_definition(run, spec, None)
}

/// Resolve live, non-terminal evidence against the governed producer binding
/// when one exists. Terminal pages use the run's frozen verdict directly;
/// this function keeps an in-flight preview from selecting a same-named
/// measurement emitted by the wrong analysis instance.
pub(super) fn measurement_in_output_dataset_for_definition(
    run: &crate::state::SimulationRun,
    spec: &crate::state::SpecEntry,
    definition: Option<&crate::state::SpecificationDefinition>,
) -> Option<OutputMeasurementEvidence> {
    let name = spec.measurement.as_str();
    let producing_analysis = definition.and_then(|definition| definition.producing_analysis);
    let candidates: Vec<(f64, bool)> = run
        .analyses
        .iter()
        .filter(|analysis| analysis.provenance.is_some())
        .filter(|analysis| {
            let provenance = analysis
                .provenance
                .as_ref()
                .expect("the preceding filter admitted only attributed analyses");
            spec.scope.admits(provenance.pvt_point())
                && producing_analysis
                    .is_none_or(|expected| expected == provenance.authored_source_instance_id())
        })
        .flat_map(|analysis| {
            let analysis_level = analysis.measurements.iter().filter_map(move |measurement| {
                if !measurement.name.eq_ignore_ascii_case(name) {
                    return None;
                }
                let value = measurement.value.filter(|value| value.is_finite())?;
                Some((
                    value,
                    analysis.success && measurement.passed && measurement.error.is_none(),
                ))
            });
            // A family that measured its own members contributes one candidate
            // per member, so the count this preview reports is the size of the
            // spread the frozen verdict will range over. Reading only the
            // analysis level here would have the preview say "the only
            // measurement of this name" about a five-hundred-trial sweep.
            let member_level = analysis
                .family_metadata
                .iter()
                .flat_map(|metadata| metadata.member_measurements())
                .filter_map(move |member| {
                    let evidence = member.evidence_for(name)?;
                    let value = evidence.value.filter(|value| value.is_finite())?;
                    Some((value, analysis.success && evidence.passed))
                });
            analysis_level.chain(member_level)
        })
        .collect();

    let retained_measurements = candidates.len();
    // Ordered so that the chosen entry is the one a reader would most want to
    // be told about: a measurement the engine could not complete first, then
    // the smallest margin. `total_cmp` rather than `partial_cmp` because every
    // value here is already finite and a panic on a tie is not acceptable.
    let (value, measurement_passed) = candidates.into_iter().reduce(|worst, candidate| {
        let rank = |(value, passed): (f64, bool)| (passed, margin_to_bound(spec, value));
        let (worst_passed, worst_margin) = rank(worst);
        let (candidate_passed, candidate_margin) = rank(candidate);
        match (worst_passed, candidate_passed) {
            (false, true) => worst,
            (true, false) => candidate,
            // A later point wins a tie, so a dataset re-run in place reports
            // its newest evidence rather than its oldest.
            _ if candidate_margin.total_cmp(&worst_margin).is_gt() => worst,
            _ => candidate,
        }
    })?;

    Some(OutputMeasurementEvidence {
        value,
        measurement_passed,
        retained_measurements,
    })
}

/// How much room a value has before it violates the specification.
///
/// Negative once the bound is broken, and more negative the worse the
/// violation, so ordering by this alone ranks failures ahead of passes and the
/// worst failure ahead of the rest. A specification with no scalar bound has no
/// ordering to offer, so every value ties and the latest one stands.
fn margin_to_bound(spec: &crate::state::SpecEntry, value: f64) -> f64 {
    match (spec.min, spec.max) {
        (Some(minimum), Some(maximum)) => (value - minimum).min(maximum - value),
        (Some(minimum), None) => value - minimum,
        (None, Some(maximum)) => maximum - value,
        (None, None) => 0.0,
    }
}
