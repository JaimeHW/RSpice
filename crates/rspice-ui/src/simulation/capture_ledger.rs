//! What a plan's retained evidence costs, capture group by capture group.
//!
//! This module owns that arithmetic once. The Save page's ledger and the
//! preparation-time budget gate used to derive the same forecast separately —
//! the page folded per-policy subtotals and multiplied by the Run Set point
//! count, and the gate folded a scalar and multiplied by the same count — which
//! meant the number an engineer approved on the page and the number that
//! refused their run were two opinions that happened to agree. They agree by
//! construction now: [`CaptureLedger::total_bytes`] is a projection of
//! [`CaptureLedger::rows`] plus the two plan-level allowances, and both callers
//! read it rather than summing anything of their own.
//!
//! Three things are counted, and they are kept apart because they are attributed
//! differently:
//!
//! * **Group rows.** Each output's bounded estimate lands in the row of the one
//!   group that owns it, per [`CaptureGroupMembership`]. Exactly one, so a
//!   ledger row plus a ledger row is never the same output twice.
//! * **Shared engine source.** A deferred output is evaluated after the run
//!   against the whole retained engine state of its analyses, and several
//!   deferred outputs on one analysis share that state. It is therefore a
//!   plan-level line rather than a group's: folding a shared ceiling into
//!   whichever group happened to hold the first deferred output would make a
//!   group's subtotal depend on outputs in other groups.
//! * **Save All ceiling.** In Save All the engine's whole bounded result
//!   allowance is reserved for every enabled analysis regardless of what is
//!   authored, so it is the same kind of plan-level line.
//!
//! Every line is priced over the Run Set exactly once, here — and per
//! analysis, not per plan. [`CaptureWorkload`] carries how many points each
//! analysis is executed at, because participation is a property of the
//! instance: multiplying every group by the whole matrix priced a
//! nominal-only analysis as if it crossed every corner, and the fail-closed
//! preparation gate refused runs that fit by exactly that factor.

use crate::product::CaptureGroupId;
use crate::simulation::SavedOutputStorageEstimate;
use crate::simulation::output_contract::{
    SavedOutputPreflightReport, retained_engine_source_upper_bound_bytes,
};
use crate::state::{CaptureGroup, CaptureGroupMembership, OutputSelectionMode, SavedOutput};

/// One group's line in the ledger.
pub struct CaptureLedgerRow {
    pub group: CaptureGroupId,
    pub name: String,
    /// What the group imposes on its members, or that it imposes nothing.
    pub policy: String,
    pub outputs: usize,
    /// Bounded bytes for this group's outputs, already priced over each
    /// producing analysis's own participation in the Run Set.
    pub bytes: u64,
    /// Outputs in this group whose size cannot be proven before the solve.
    pub indeterminate: usize,
}

/// One output whose retained size cannot be bounded before the run.
///
/// Retained with its reason because the preparation gate refuses by name: an
/// engineer told only that "something is indeterminate" has nothing to fix.
pub struct IndeterminateOutput {
    pub name: String,
    pub reason: String,
}

/// How many run-set points each analysis is executed at, and how many engine
/// analyses the queue holds.
///
/// Participation is a property of the instance — an analysis declared at the
/// nominal point alone costs one solve, not one per corner — so a scalar point
/// count cannot price a plan. Pricing every group at the whole matrix made the
/// fail-closed budget gate refuse runs that fit, by exactly the factor of the
/// PVT set.
///
/// Both callers resolve it through
/// [`crate::simulation::run_set::participating_point_keys`], the one resolver
/// the prepared expansion mints its tasks from, so the number the Save page
/// forecasts against and the number preparation refuses against are the same
/// projection of the same queue.
pub struct CaptureWorkload {
    points_by_analysis: std::collections::HashMap<crate::product::AnalysisInstanceId, u64>,
    /// Points for an analysis this projection does not name. A caller whose
    /// declared space does not expand exactly states the whole space here,
    /// which is the same fail-closed rule the workload table applies to an
    /// unresolved instance: pricing at zero is how a budget silently shrinks.
    default_points: u64,
    /// Points summed over every engine analysis whose complete result state
    /// Save All reserves. One PSS with a retained spectrum is two of them, so
    /// this is a sum over tasks rather than over authored instances.
    engine_task_points: u64,
}

impl CaptureWorkload {
    /// Every analysis at every declared point.
    ///
    /// What a caller states when there is nothing to narrow against — no
    /// global axes, or a composition that proposes its points from feedback.
    #[must_use]
    pub fn uniform(points: u64, engine_analysis_count: usize) -> Self {
        let points = points.max(1);
        Self {
            points_by_analysis: std::collections::HashMap::new(),
            default_points: points,
            engine_task_points: points
                .saturating_mul(u64::try_from(engine_analysis_count).unwrap_or(u64::MAX)),
        }
    }

    /// A projection that prices each named analysis at its own participation.
    #[must_use]
    pub fn narrowed(
        points_by_analysis: std::collections::HashMap<crate::product::AnalysisInstanceId, u64>,
        default_points: u64,
        engine_task_points: u64,
    ) -> Self {
        Self {
            points_by_analysis,
            default_points: default_points.max(1),
            engine_task_points,
        }
    }

    /// Points one analysis is executed at.
    #[must_use]
    pub fn points_for(&self, analysis: crate::product::AnalysisInstanceId) -> u64 {
        self.points_by_analysis
            .get(&analysis)
            .copied()
            .unwrap_or(self.default_points)
    }

    #[must_use]
    pub const fn default_points(&self) -> u64 {
        self.default_points
    }

    /// Points summed over every engine analysis in the queue.
    #[must_use]
    pub const fn engine_task_points(&self) -> u64 {
        self.engine_task_points
    }
}

/// What one output's bounded estimate costs over the queue that retains it.
///
/// `None` where the size cannot be bounded before the solve — an output whose
/// cost is not a number is not a number this may invent.
///
/// The one owner of "bytes × participation", so a surface that prices a single
/// output and the ledger that sums them cannot disagree. The Outputs registry's
/// `Est. size` cell used to multiply the *authored* record's estimate by the
/// declared point count, which is neither of the two things this does: it
/// priced the record rather than the group-projected output a run executes, and
/// it charged a nominal-only analysis for every corner. Two forecasts of one
/// output, and the one on the page that lists them was the wrong one.
///
/// A report that names no analysis — a fixture, or an output whose parts could
/// not be attributed — is priced at the caller's default, which is the whole
/// declared space.
#[must_use]
pub fn priced_output_bytes(
    report: &SavedOutputPreflightReport,
    workload: &CaptureWorkload,
) -> Option<u64> {
    let SavedOutputStorageEstimate::ExactBytes(bytes) = report.storage_estimate() else {
        return None;
    };
    if report.bytes_by_analysis().is_empty() {
        return Some(bytes.saturating_mul(workload.default_points()));
    }
    Some(
        report
            .bytes_by_analysis()
            .iter()
            .fold(0u64, |total, (analysis, analysis_bytes)| {
                total.saturating_add(analysis_bytes.saturating_mul(workload.points_for(*analysis)))
            }),
    )
}

/// The plan's retained-evidence forecast, partitioned the way it is printed.
pub struct CaptureLedger {
    rows: Vec<CaptureLedgerRow>,
    /// Retained engine state shared by deferred outputs, counted once for the
    /// plan.
    shared_source_bytes: u64,
    /// The Save All allowance, when that mode is selected.
    engine_ceiling_bytes: Option<u64>,
    indeterminate: Vec<IndeterminateOutput>,
}

impl CaptureLedger {
    /// Price one effective output set against the groups that own it.
    ///
    /// `reports` is parallel to `outputs`, as the preflight produces it, and
    /// `membership` was resolved from the same slice — so an output, its
    /// estimate and its owner are the same index in all three and cannot be
    /// paired up wrongly.
    ///
    /// `workload` says how many run-set points each analysis is executed at
    /// and how many engine analyses the queue holds. Both callers build it
    /// through [`crate::simulation::run_set::participating_point_keys`], the
    /// resolver the prepared expansion mints tasks from, so neither can price
    /// a nominal-only analysis at the whole PVT matrix — which is how a
    /// fail-closed budget came to refuse a run that fits.
    #[must_use]
    pub fn resolve(
        groups: &[CaptureGroup],
        outputs: &[SavedOutput],
        reports: &[SavedOutputPreflightReport],
        membership: &CaptureGroupMembership,
        selection_mode: OutputSelectionMode,
        workload: &CaptureWorkload,
    ) -> Self {
        let mut rows: Vec<CaptureLedgerRow> = groups
            .iter()
            .chain(std::iter::once(&CaptureGroup::ungrouped()))
            .map(|group| CaptureLedgerRow {
                group: group.id,
                name: group.name.clone(),
                policy: group.policy_summary(),
                outputs: 0,
                bytes: 0,
                indeterminate: 0,
            })
            .collect();
        let mut indeterminate = Vec::new();
        let mut shared_source_analyses = std::collections::HashSet::new();
        for (index, (output, report)) in outputs.iter().zip(reports).enumerate() {
            let owner = membership.owner(index);
            let Some(row) = rows.iter_mut().find(|row| row.group == owner) else {
                continue;
            };
            row.outputs += 1;
            match (
                priced_output_bytes(report, workload),
                report.storage_estimate(),
            ) {
                (Some(priced), _) => row.bytes = row.bytes.saturating_add(priced),
                (None, SavedOutputStorageEstimate::Indeterminate { reason }) => {
                    row.indeterminate += 1;
                    indeterminate.push(IndeterminateOutput {
                        name: output.name.clone(),
                        reason: reason.clone(),
                    });
                }
                (None, SavedOutputStorageEstimate::ExactBytes(_)) => {}
            }
            shared_source_analyses
                .extend(report.retained_engine_source_analysis_ids().iter().copied());
        }
        let save_all = selection_mode == OutputSelectionMode::SaveAll;
        let per_analysis_engine_bytes = retained_engine_source_upper_bound_bytes(1);
        Self {
            rows,
            // In Save All the whole engine result set is already reserved
            // below, and reserving the deferred share on top would count the
            // same retained state twice.
            shared_source_bytes: if save_all {
                0
            } else {
                shared_source_analyses.iter().fold(0u64, |total, analysis| {
                    total.saturating_add(
                        per_analysis_engine_bytes.saturating_mul(workload.points_for(*analysis)),
                    )
                })
            },
            engine_ceiling_bytes: save_all
                .then(|| per_analysis_engine_bytes.saturating_mul(workload.engine_task_points())),
            indeterminate,
        }
    }

    /// The groups, in resolution order, with the fallback group last.
    #[must_use]
    pub fn rows(&self) -> &[CaptureLedgerRow] {
        &self.rows
    }

    #[must_use]
    pub const fn shared_source_bytes(&self) -> u64 {
        self.shared_source_bytes
    }

    #[must_use]
    pub const fn engine_ceiling_bytes(&self) -> Option<u64> {
        self.engine_ceiling_bytes
    }

    #[must_use]
    pub fn indeterminate(&self) -> &[IndeterminateOutput] {
        &self.indeterminate
    }

    /// The whole forecast: every printed row plus every printed plan-level
    /// line, and nothing that is not printed.
    ///
    /// This is the number the budget is checked against and the number the
    /// page's status line shows. A row a surface chooses not to draw is still
    /// counted here, which is deliberate — a total that shrank when a row was
    /// hidden would be a different claim about the same plan.
    #[must_use]
    pub fn total_bytes(&self) -> u64 {
        self.rows
            .iter()
            .fold(self.shared_source_bytes, |total, row| {
                total.saturating_add(row.bytes)
            })
            .saturating_add(self.engine_ceiling_bytes.unwrap_or(0))
    }

    #[must_use]
    pub fn indeterminate_count(&self) -> usize {
        self.indeterminate.len()
    }
}

/// Project one plan's effective outputs onto its capture groups.
///
/// Membership is resolved once, and each output is rewritten in place with the
/// overrides of the one group that owns it. Both halves happen here because
/// they must not come apart: a surface forecasting against overridden policies
/// while execution ran the authored ones would be a forecast of a run that
/// never happens.
///
/// Called by [`crate::simulation::controller`] on both routes into the
/// effective output set — the page's preflight and the prepared run's — so the
/// number shown and the contract dispatched are the same projection.
pub fn project_onto_groups(
    groups: &[CaptureGroup],
    outputs: &mut [SavedOutput],
) -> CaptureGroupMembership {
    let membership = CaptureGroupMembership::resolve(groups, outputs);
    for (index, output) in outputs.iter_mut().enumerate() {
        let owner = membership.owner(index);
        if let Some(group) = groups.iter().find(|group| group.id == owner) {
            group.apply(output);
        }
    }
    membership
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        CaptureGroupRule, InstancePath, SavedOutputCompatibility, SavedOutputKind,
        SavedOutputPolicy, SavedOutputPrecision, SavedOutputStreaming,
    };

    fn output(name: &str, expression: &str) -> SavedOutput {
        SavedOutput::new(
            SavedOutputKind::RawVoltageOrCurrent,
            name,
            expression,
            SavedOutputCompatibility::OpTranAc,
            SavedOutputPolicy::SelectedAndFinalPoints,
            SavedOutputPrecision::FullSourcePrecision,
            SavedOutputStreaming::StoreOnly,
        )
        .expect("test output")
    }

    fn exact(bytes: u64) -> SavedOutputPreflightReport {
        SavedOutputPreflightReport::exact_for_test(bytes)
    }

    fn exact_for(
        analysis: crate::product::AnalysisInstanceId,
        bytes: u64,
    ) -> SavedOutputPreflightReport {
        SavedOutputPreflightReport::exact_for_analysis_test(analysis, bytes)
    }

    fn core_group() -> CaptureGroup {
        let mut group = CaptureGroup::new("Core").expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy("/x1").expect("scope"),
        ));
        group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
        group
    }

    /// An analysis narrowed to one point costs one point.
    ///
    /// The forecast used to multiply every group by the whole Run Set, so a
    /// transient declared at the nominal point alone was priced at 27x over a
    /// PVT set — and the preparation gate, which fails closed against the same
    /// number, refused runs that fit.
    #[test]
    fn each_analysis_is_priced_at_its_own_participation() {
        use crate::product::AnalysisInstanceId;

        let nominal = AnalysisInstanceId::new();
        let everywhere = AnalysisInstanceId::new();
        let outputs = vec![output("a", "V(n)"), output("b", "V(m)")];
        let reports = vec![exact_for(nominal, 100), exact_for(everywhere, 100)];
        let membership = CaptureGroupMembership::resolve(&[], &outputs);

        let workload = CaptureWorkload::narrowed(
            std::collections::HashMap::from([(nominal, 1), (everywhere, 27)]),
            27,
            28,
        );
        let ledger = CaptureLedger::resolve(
            &[],
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &workload,
        );

        assert_eq!(
            ledger.total_bytes(),
            100 + 100 * 27,
            "the nominal-only analysis costs one point, not the whole matrix"
        );

        // The same reports over an unnarrowed projection: both at 27.
        let uniform = CaptureLedger::resolve(
            &[],
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &CaptureWorkload::uniform(27, 2),
        );
        assert_eq!(uniform.total_bytes(), 200 * 27);
    }

    #[test]
    fn the_total_is_the_rows_and_nothing_but_the_rows() {
        let groups = vec![core_group()];
        let outputs = vec![output("a", "V(x1.n)"), output("b", "V(x2.n)")];
        let reports = vec![exact(100), exact(250)];
        let membership = CaptureGroupMembership::resolve(&groups, &outputs);

        let ledger = CaptureLedger::resolve(
            &groups,
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &CaptureWorkload::uniform(3, 2),
        );

        assert_eq!(
            ledger.rows().len(),
            2,
            "one authored group plus the fallback"
        );
        assert_eq!(ledger.rows()[0].name, "Core");
        assert_eq!(ledger.rows()[0].outputs, 1);
        assert_eq!(ledger.rows()[0].bytes, 100 * 3);
        assert_eq!(
            ledger.rows()[1].outputs,
            1,
            "the sibling scope is unclaimed"
        );
        assert_eq!(ledger.rows()[1].bytes, 250 * 3);
        assert_eq!(
            ledger.total_bytes(),
            ledger.rows().iter().map(|row| row.bytes).sum::<u64>()
                + ledger.shared_source_bytes()
                + ledger.engine_ceiling_bytes().unwrap_or(0),
            "the total is a projection of the printed lines, not a second walk"
        );
    }

    #[test]
    fn every_output_is_counted_in_exactly_one_row() {
        // Two groups whose rules both match every output, plus an explicit
        // claim: whichever way precedence resolves, the counts must partition.
        let mut first = core_group();
        first.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::RawVoltageOrCurrent,
        ));
        let mut second = CaptureGroup::new("Second").expect("group");
        second.rules.push(CaptureGroupRule::for_kind(
            SavedOutputKind::RawVoltageOrCurrent,
        ));
        let outputs = vec![
            output("a", "V(x1.n)"),
            output("b", "V(x2.n)"),
            output("c", "V(x3.n)"),
        ];
        second.members.push(outputs[2].id);
        let groups = vec![first, second];
        let reports = vec![exact(10), exact(20), exact(40)];
        let membership = CaptureGroupMembership::resolve(&groups, &outputs);

        let ledger = CaptureLedger::resolve(
            &groups,
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &CaptureWorkload::uniform(1, 1),
        );

        assert_eq!(
            ledger.rows().iter().map(|row| row.outputs).sum::<usize>(),
            outputs.len(),
            "three outputs are counted three times in total, not four"
        );
        assert_eq!(ledger.total_bytes(), 10 + 20 + 40);
    }

    #[test]
    fn a_group_override_reaches_the_output_the_contract_is_built_from() {
        let groups = vec![core_group()];
        let mut outputs = vec![output("a", "V(x1.n)"), output("b", "V(x2.n)")];

        let membership = project_onto_groups(&groups, &mut outputs);

        assert_eq!(membership.owner(0), groups[0].id);
        assert_eq!(
            outputs[0].save_policy,
            SavedOutputPolicy::EveryAcceptedPoint,
            "the owning group's override is what the run will execute"
        );
        assert_eq!(
            outputs[1].save_policy,
            SavedOutputPolicy::SelectedAndFinalPoints,
            "and an output no group claims keeps its own contract"
        );
    }

    /// Each axis on its own, in both directions, for every value it can take.
    ///
    /// The test above judges the `points` axis, which is the one the ledger
    /// prices — so a wrong answer there shows up as a wrong number. Streaming
    /// and precision move no byte count: a member left on `Store only` under a
    /// group that asked for live delivery, or written at full precision under
    /// a group that asked for a display cache, produces a run that reports
    /// exactly the size it was forecast at and stores something else. Nothing
    /// downstream contradicts it.
    ///
    /// So all three axes are walked, and each against both halves of the
    /// contract the module header states: a group that sets an axis imposes it
    /// on its members, and a group that leaves one unset leaves the member's
    /// own value in force. Both halves are asserted in the same pass over the
    /// same pair of outputs, because an `apply` that overwrote unconditionally
    /// and one that never wrote at all each satisfy one half on its own.
    ///
    /// Each axis is also checked to have left the other two alone, which is
    /// what makes these three overrides rather than one policy in three
    /// fields.
    #[test]
    fn a_group_imposes_the_axes_it_sets_and_only_those() {
        /// A group claiming `/x1`, configured on one axis and no other.
        fn claiming(configure: impl FnOnce(&mut CaptureGroup)) -> CaptureGroup {
            let mut group = CaptureGroup::new("Core").expect("group name");
            group.rules.push(CaptureGroupRule::for_scope(
                InstancePath::parse_legacy("/x1").expect("scope"),
            ));
            configure(&mut group);
            group
        }

        // What `output()` authors, restated so the "unset" half of each
        // assertion compares against a named contract rather than against
        // whatever the helper happens to build today.
        let authored = output("authored", "V(n)");
        assert_eq!(
            authored.save_policy,
            SavedOutputPolicy::SelectedAndFinalPoints
        );
        assert_eq!(
            authored.stored_precision,
            SavedOutputPrecision::FullSourcePrecision
        );
        assert_eq!(authored.streaming, SavedOutputStreaming::StoreOnly);

        for points in SavedOutputPolicy::ALL {
            let groups = vec![claiming(|group| group.points = Some(points))];
            let mut outputs = vec![output("claimed", "V(x1.n)"), output("free", "V(x2.n)")];
            project_onto_groups(&groups, &mut outputs);

            assert_eq!(
                outputs[0].save_policy, points,
                "a group that sets the point policy imposes it on its member"
            );
            assert_eq!(
                outputs[1].save_policy, authored.save_policy,
                "and states nothing about an output it does not hold"
            );
            assert_eq!(
                (outputs[0].stored_precision, outputs[0].streaming),
                (authored.stored_precision, authored.streaming),
                "a point policy is not a precision or a streaming decision"
            );
        }

        for precision in SavedOutputPrecision::ALL {
            let groups = vec![claiming(|group| group.precision = Some(precision))];
            let mut outputs = vec![output("claimed", "V(x1.n)"), output("free", "V(x2.n)")];
            project_onto_groups(&groups, &mut outputs);

            assert_eq!(
                outputs[0].stored_precision, precision,
                "a group that sets a precision imposes it on its member"
            );
            assert_eq!(
                outputs[1].stored_precision, authored.stored_precision,
                "and a member whose group sets none keeps its own"
            );
            assert_eq!(
                (outputs[0].save_policy, outputs[0].streaming),
                (authored.save_policy, authored.streaming),
                "a precision is not a point policy or a streaming decision"
            );
        }

        for streaming in SavedOutputStreaming::ALL {
            let groups = vec![claiming(|group| group.streaming = Some(streaming))];
            let mut outputs = vec![output("claimed", "V(x1.n)"), output("free", "V(x2.n)")];
            project_onto_groups(&groups, &mut outputs);

            assert_eq!(
                outputs[0].streaming, streaming,
                "a group that sets a streaming mode imposes it on its member"
            );
            assert_eq!(
                outputs[1].streaming, authored.streaming,
                "and a member whose group sets none keeps its own"
            );
            assert_eq!(
                (outputs[0].save_policy, outputs[0].stored_precision),
                (authored.save_policy, authored.stored_precision),
                "a streaming mode is not a point policy or a precision"
            );
        }

        // A group that sets nothing leaves its member exactly as it found it,
        // which is what makes membership in one a fact rather than a policy.
        let groups = vec![claiming(|_| {})];
        let mut outputs = vec![output("claimed", "V(x1.n)")];
        let membership = project_onto_groups(&groups, &mut outputs);
        assert_eq!(membership.owner(0), groups[0].id);
        assert_eq!(
            (
                outputs[0].save_policy,
                outputs[0].stored_precision,
                outputs[0].streaming
            ),
            (
                authored.save_policy,
                authored.stored_precision,
                authored.streaming
            )
        );
    }

    #[test]
    fn a_plan_with_no_groups_prices_exactly_as_it_did_before_groups_existed() {
        let mut outputs = vec![output("a", "V(x1.n)"), output("b", "V(x2.n)")];
        let authored = outputs.clone();
        let reports = vec![exact(100), exact(250)];

        let membership = project_onto_groups(&[], &mut outputs);
        let ledger = CaptureLedger::resolve(
            &[],
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &CaptureWorkload::uniform(4, 2),
        );

        assert_eq!(outputs, authored, "no group means no override");
        assert_eq!(ledger.rows().len(), 1);
        assert_eq!(ledger.rows()[0].name, crate::state::UNGROUPED_NAME);
        assert_eq!(ledger.total_bytes(), (100 + 250) * 4);
    }

    #[test]
    fn save_all_reserves_the_engine_ceiling_instead_of_the_deferred_share() {
        let outputs = vec![output("a", "V(x1.n)")];
        let reports = vec![exact(64)];
        let membership = CaptureGroupMembership::resolve(&[], &outputs);

        let ledger = CaptureLedger::resolve(
            &[],
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::SaveAll,
            &CaptureWorkload::uniform(2, 3),
        );

        assert_eq!(ledger.shared_source_bytes(), 0);
        assert_eq!(
            ledger.engine_ceiling_bytes(),
            Some(retained_engine_source_upper_bound_bytes(3).saturating_mul(2))
        );
        assert_eq!(
            ledger.total_bytes(),
            64 * 2
                + ledger
                    .engine_ceiling_bytes()
                    .expect("save all reserves one")
        );
    }

    #[test]
    fn an_unprovable_output_is_named_with_its_reason_rather_than_priced_at_zero() {
        let outputs = vec![output("a", "V(x1.n)")];
        let reports = vec![SavedOutputPreflightReport::invalid("no retained source")];
        let membership = CaptureGroupMembership::resolve(&[], &outputs);

        let ledger = CaptureLedger::resolve(
            &[],
            &outputs,
            &reports,
            &membership,
            OutputSelectionMode::ExplicitOnly,
            &CaptureWorkload::uniform(1, 1),
        );

        assert_eq!(ledger.indeterminate_count(), 1);
        assert_eq!(ledger.indeterminate()[0].name, "a");
        assert_eq!(ledger.indeterminate()[0].reason, "no retained source");
        assert_eq!(ledger.rows()[0].indeterminate, 1);
        assert_eq!(ledger.total_bytes(), 0);
    }
}
