//! Explicit task graphs independent of application setup or editor state.
//!
//! This is the graph stage of headless preparation. Source capture, saved-output
//! admission and snapshot authorization still follow it; these tasks cannot be
//! passed directly to a runner.

use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use rspice_app_types::product::{AnalysisInstanceId, ObjectRevision};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use rspice_simulation_contract::analysis_spec::AnalysisSpec;

use super::PreparedTask;
use super::task_preparation::{bind_prepared_task_dependencies, executes_via_spec};
use crate::error::{ServiceRunError, ServiceRunResult, ensure_not_aborted};
use crate::execution_identity::analysis_config_digest;
use crate::preparation::QueuedAnalysis;

/// A lowered analysis and its explicit ordering/artifact dependencies.
///
/// The caller owns stable identities. Array position is only a tie breaker for
/// ready tasks, never an implicit choice of a periodic or transient producer.
#[derive(Debug, Clone)]
pub struct HeadlessTaskRequest {
    pub instance_id: AnalysisInstanceId,
    pub label: String,
    pub dependencies: Vec<AnalysisInstanceId>,
    pub analysis: QueuedAnalysis,
}

/// Validate and order a headless graph, then bind the same exact producer
/// payloads used by application preparation. No editor or project is required.
///
/// `max_batch_runs` bounds task count and `max_result_values` bounds retained
/// graph edges. These are preparation limits, not an execution resource policy.
/// The resulting tasks still require source preparation and an authorized
/// [`super::PreparedRunSnapshot`] before execution.
pub fn prepare_headless_tasks(
    requests: Vec<HeadlessTaskRequest>,
    source_revision: ObjectRevision,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<PreparedTask>> {
    ensure_not_aborted(abort)?;
    check_limit(
        ResourceKind::BatchRuns,
        requests.len(),
        limits.max_batch_runs,
    )?;
    if requests.is_empty() {
        return Err(invalid(
            "A headless task graph must contain at least one analysis",
        ));
    }

    let mut positions = HashMap::with_capacity(requests.len());
    let mut edge_count = 0usize;
    for (index, request) in requests.iter().enumerate() {
        ensure_not_aborted(abort)?;
        if positions.insert(request.instance_id, index).is_some() {
            return Err(invalid(format!(
                "Duplicate analysis identity {}",
                request.instance_id
            )));
        }
        edge_count = edge_count.saturating_add(request.dependencies.len());
        check_limit(
            ResourceKind::ResultValues,
            edge_count,
            limits.max_result_values,
        )?;
        validate_request(request)?;
    }

    let mut remaining = Vec::with_capacity(requests.len());
    let mut dependents = vec![Vec::new(); requests.len()];
    let mut ready = BinaryHeap::new();
    for (index, request) in requests.iter().enumerate() {
        ensure_not_aborted(abort)?;
        let mut seen = HashSet::with_capacity(request.dependencies.len());
        for dependency in &request.dependencies {
            ensure_not_aborted(abort)?;
            if *dependency == request.instance_id {
                return Err(invalid(format!(
                    "Analysis {} cannot depend on itself",
                    request.instance_id
                )));
            }
            if !seen.insert(*dependency) {
                return Err(invalid(format!(
                    "Analysis {} repeats dependency {dependency}",
                    request.instance_id
                )));
            }
            let producer = positions.get(dependency).ok_or_else(|| {
                invalid(format!(
                    "Analysis {} references missing dependency {dependency}",
                    request.instance_id
                ))
            })?;
            dependents[*producer].push(index);
        }
        remaining.push(request.dependencies.len());
        if request.dependencies.is_empty() {
            ready.push(Reverse(index));
        }
    }

    let count = requests.len();
    let mut requests = requests.into_iter().map(Some).collect::<Vec<_>>();
    let mut tasks = Vec::with_capacity(count);
    while let Some(Reverse(index)) = ready.pop() {
        ensure_not_aborted(abort)?;
        let request = requests[index]
            .take()
            .ok_or_else(|| invalid("Analysis was ordered twice"))?;
        tasks.push(PreparedTask::new(
            request.instance_id,
            source_revision,
            request.dependencies,
            request.label,
            request.analysis,
        ));
        for dependent in &dependents[index] {
            ensure_not_aborted(abort)?;
            remaining[*dependent] -= 1;
            if remaining[*dependent] == 0 {
                ready.push(Reverse(*dependent));
            }
        }
    }
    if tasks.len() != count {
        return Err(invalid("Headless analysis dependencies contain a cycle"));
    }
    ensure_not_aborted(abort)?;
    bind_prepared_task_dependencies(&mut tasks, abort)?;
    ensure_not_aborted(abort)?;
    Ok(tasks)
}

fn validate_request(request: &HeadlessTaskRequest) -> ServiceRunResult<()> {
    let analysis = &request.analysis;
    let context = |message| invalid(format!("Analysis {}: {message}", request.instance_id));
    if request.label.trim().is_empty() {
        return Err(context("display label must not be empty".to_owned()));
    }
    analysis.spec.validate().map_err(&context)?;
    if let Some(reason) =
        crate::execution_identity::canonical_analysis_kind(&analysis.spec).execution_blocker()
    {
        return Err(context(reason.to_owned()));
    }
    if let Some(config) = &analysis.config {
        if executes_via_spec(&analysis.spec) {
            return Err(context(
                "this analysis must use its typed specification, not a config override".into(),
            ));
        }
        config
            .validate()
            .map_err(|errors| context(errors.join("; ")))?;
        let expected = crate::analysis_preparation::analysis_spec_to_config(&analysis.spec)
            .map_err(&context)?;
        let digest = |config| {
            analysis_config_digest(
                &analysis.analysis_line,
                &analysis.spec,
                Some(config),
                &analysis.spec_options,
                analysis.numeric_override.as_ref(),
            )
        };
        if digest(config) != digest(&expected) {
            return Err(context(
                "configuration disagrees with the typed analysis specification".into(),
            ));
        }
    }
    let expected_cards = match &analysis.spec {
        AnalysisSpec::Fft { request } => Some(request.to_card()),
        AnalysisSpec::AcData { table_options, .. } if !table_options.from_netlist => Some(
            crate::analysis_preparation::build_ac_data_command(&analysis.spec).map_err(&context)?,
        ),
        _ => None,
    };
    if let Some(expected) = expected_cards
        && analysis.analysis_line != expected
    {
        return Err(context(
            "generated observation/table cards disagree with the typed request".into(),
        ));
    }
    Ok(())
}

fn invalid(message: impl Into<String>) -> ServiceRunError {
    ServiceRunError::Failure(message.into())
}

fn check_limit(resource: ResourceKind, requested: usize, limit: usize) -> ServiceRunResult<()> {
    if requested > limit {
        return Err(ServiceRunError::ResourceLimit(ResourceLimitError {
            resource,
            requested,
            limit,
        }));
    }
    Ok(())
}

#[cfg(test)]
mod tests;
