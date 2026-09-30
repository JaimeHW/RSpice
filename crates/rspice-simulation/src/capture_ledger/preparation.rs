//! Capture workload and storage gates evaluated before task expansion.

use super::CaptureLedger;
use crate::preparation::{PreparationError, PreparationStage};
use rspice_app_types::product::AnalysisInstanceId;
use rspice_simulation_contract::analysis_run_at::AnalysisRunAt;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::capture_group::{CaptureGroup, CaptureGroupMembership};
use rspice_simulation_contract::output_policy::SimulationSavePolicy;
use rspice_simulation_contract::saved_output::SavedOutput;

/// How many Run Set points each queued task is executed at.
///
/// The queue this gate is handed is the plan's, before PVT expansion, and each
/// task carries the participation its analysis declared. Resolving it here
/// through [`rspice_simulation_contract::run_set::participating_point_keys`] — the same
/// resolver that mints the expanded tasks — is what stops the gate charging a
/// nominal-only analysis for every corner of the matrix.
///
/// Fails closed twice: a space that does not expand exactly, and a
/// participation that does not resolve against it, are both priced at the
/// whole declared space. The expansion refuses the second case by name a few
/// steps later, so over-pricing here never becomes the reason a valid run is
/// refused.
pub fn plan_capture_workload<'a>(
    run_set: &rspice_simulation_contract::run_set::RunSetState,
    reference: rspice_simulation_contract::run_set::ReferencePoint,
    tasks: impl ExactSizeIterator<Item = (AnalysisInstanceId, &'a AnalysisRunAt)>,
) -> crate::capture_ledger::CaptureWorkload {
    use crate::capture_ledger::CaptureWorkload;
    use rspice_simulation_contract::run_set;

    let matrix = u64::try_from(run_set.point_count())
        .unwrap_or(u64::MAX)
        .max(1);
    let points = match run_set::resolve(run_set) {
        Some(points) if run_set.enabled_dimensions().next().is_some() && !points.is_empty() => {
            points
        }
        _ => return CaptureWorkload::uniform(matrix, tasks.len()),
    };

    let mut points_by_analysis: std::collections::HashMap<AnalysisInstanceId, u64> =
        std::collections::HashMap::new();
    let mut engine_task_points = 0u64;
    for (instance_id, run_at) in tasks {
        let count = run_set::participating_point_keys(run_at, &points, reference)
            .map_or(matrix, |keys| {
                u64::try_from(keys.len()).unwrap_or(u64::MAX).max(1)
            });
        // Several tasks can carry one instance identity — a PSS keeps its
        // spectrum companion under the same analysis — so the analysis is
        // priced at the widest participation any of them declared.
        points_by_analysis
            .entry(instance_id)
            .and_modify(|held| *held = (*held).max(count))
            .or_insert(count);
        engine_task_points = engine_task_points.saturating_add(count);
    }
    CaptureWorkload::narrowed(points_by_analysis, matrix, engine_task_points)
}

/// Refuse a plan whose retained evidence would not fit its declared ceiling.
///
/// The forecast is [`CaptureLedger::total_bytes`] — the same number the Save
/// page's ledger prints, from the same fold over the same groups, priced over
/// the same per-analysis workload. This used to be its own accumulation, which
/// meant a page could show a forecast under the ceiling while preparation
/// refused the run for exceeding it, and nothing in either place would have
/// said which was wrong.
pub fn validate_plan_saved_output_budget<'a>(
    groups: &[CaptureGroup],
    outputs: &[SavedOutput],
    membership: &CaptureGroupMembership,
    tasks: impl Iterator<Item = (AnalysisInstanceId, &'a AnalysisSpec)> + Clone,
    workload: &crate::capture_ledger::CaptureWorkload,
    policy: &SimulationSavePolicy,
    display_cache_samples: usize,
) -> Result<(), PreparationError> {
    let reports = outputs
        .iter()
        .map(|output| {
            crate::output_contract::preflight_saved_output(
                output,
                tasks.clone(),
                display_cache_samples,
            )
        })
        .collect::<Vec<_>>();
    let ledger = CaptureLedger::resolve(
        groups,
        outputs,
        &reports,
        membership,
        policy.output_selection_mode,
        workload,
    );
    // An unbounded output is refused before the ceiling is compared: a total
    // that silently omitted it would be a forecast of a different plan.
    if let Some(unprovable) = ledger.indeterminate().first() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Saved-output storage budget cannot be proven for '{}': {}",
                unprovable.name, unprovable.reason
            ),
        ));
    }
    let forecast = ledger.total_bytes();
    if forecast > policy.maximum_storage_bytes {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Saved-output forecast {} exceeds this plan's {} storage budget",
                rspice_simulation_contract::run_set::format_bytes(forecast),
                rspice_simulation_contract::run_set::format_bytes(policy.maximum_storage_bytes)
            ),
        ));
    }
    Ok(())
}
