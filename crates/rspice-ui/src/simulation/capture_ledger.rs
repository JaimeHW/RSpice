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
//! Every line is multiplied by the Run Set point count exactly once, here.

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
    /// Bounded bytes for this group's outputs, already priced over the Run Set.
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
    /// `engine_analysis_count` is supplied rather than derived because the two
    /// callers legitimately count different things: preparation knows the exact
    /// prepared task list, and the page knows only how many analysis instances
    /// are enabled. Passing it in keeps that difference visible at the call
    /// site instead of hiding two answers inside one function.
    #[must_use]
    pub fn resolve(
        groups: &[CaptureGroup],
        outputs: &[SavedOutput],
        reports: &[SavedOutputPreflightReport],
        membership: &CaptureGroupMembership,
        selection_mode: OutputSelectionMode,
        engine_analysis_count: usize,
        run_set_points: u64,
    ) -> Self {
        let points = run_set_points.max(1);
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
            match report.storage_estimate() {
                SavedOutputStorageEstimate::ExactBytes(bytes) => {
                    row.bytes = row.bytes.saturating_add(*bytes);
                }
                SavedOutputStorageEstimate::Indeterminate { reason } => {
                    row.indeterminate += 1;
                    indeterminate.push(IndeterminateOutput {
                        name: output.name.clone(),
                        reason: reason.clone(),
                    });
                }
            }
            shared_source_analyses
                .extend(report.retained_engine_source_analysis_ids().iter().copied());
        }
        for row in &mut rows {
            row.bytes = row.bytes.saturating_mul(points);
        }
        let save_all = selection_mode == OutputSelectionMode::SaveAll;
        Self {
            rows,
            // In Save All the whole engine result set is already reserved
            // below, and reserving the deferred share on top would count the
            // same retained state twice.
            shared_source_bytes: if save_all {
                0
            } else {
                retained_engine_source_upper_bound_bytes(shared_source_analyses.len())
                    .saturating_mul(points)
            },
            engine_ceiling_bytes: save_all.then(|| {
                retained_engine_source_upper_bound_bytes(engine_analysis_count)
                    .saturating_mul(points)
            }),
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

    fn core_group() -> CaptureGroup {
        let mut group = CaptureGroup::new("Core").expect("group name");
        group.rules.push(CaptureGroupRule::for_scope(
            InstancePath::parse_legacy("/x1").expect("scope"),
        ));
        group.points = Some(SavedOutputPolicy::EveryAcceptedPoint);
        group
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
            2,
            3,
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
            1,
            1,
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
            2,
            4,
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
            3,
            2,
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
            1,
            1,
        );

        assert_eq!(ledger.indeterminate_count(), 1);
        assert_eq!(ledger.indeterminate()[0].name, "a");
        assert_eq!(ledger.indeterminate()[0].reason, "no retained source");
        assert_eq!(ledger.rows()[0].indeterminate, 1);
        assert_eq!(ledger.total_bytes(), 0);
    }
}
