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
use crate::transient_checkpoint::{TransientCheckpointInput, TransientCheckpointRequest};

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
///
/// A graph prepared without its source reads no file, so a `CHECKPOINT=`
/// card here must arrive with its state sealed in its request.
pub fn prepare_headless_tasks(
    requests: Vec<HeadlessTaskRequest>,
    source_revision: ObjectRevision,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<Vec<PreparedTask>> {
    prepare_headless_tasks_reading_checkpoints(
        requests,
        source_revision,
        limits,
        &|name| {
            Err(format!(
                "its checkpoint file {name} is not sealed with the request, and a graph \
                 prepared without its source reads no file"
            ))
        },
        abort,
    )
}

/// Reads the bytes of the checkpoint file a `CHECKPOINT=` card names.
pub(crate) type CheckpointReader<'a> = dyn Fn(&str) -> Result<Vec<u8>, String> + 'a;

/// [`prepare_headless_tasks`], reading each `CHECKPOINT=` card's file that
/// is not sealed with its request through `read_checkpoint`.
pub(crate) fn prepare_headless_tasks_reading_checkpoints(
    mut requests: Vec<HeadlessTaskRequest>,
    source_revision: ObjectRevision,
    limits: ResourceLimits,
    read_checkpoint: &CheckpointReader<'_>,
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
    bind_transient_states(&mut requests, &positions, read_checkpoint, limits, abort)?;

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
        AnalysisSpec::TransientNoise { .. } => Some(
            crate::analysis_preparation::build_transient_noise_command(&analysis.spec)
                .map_err(&context)?,
        ),
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

/// Bind each task whose card linearizes around a transient state to the
/// transient it names as its dependency.
///
/// The engine runs the state's transient with the card, so the card alone is
/// not a runnable request: the parent's `.TRAN` card is spliced ahead of the
/// task's own. The task's deck then holds both, its payload identity covers
/// both, and the worker sends the card through the engine's state entry,
/// whose one result is the task's result. A task is one state, so its card
/// names one time; a graph asks for several states with several tasks, each
/// naming the transient as its parent.
///
/// A `CHECKPOINT=` card's file is read once, here, with `read_checkpoint`,
/// and sealed into the task as its transient checkpoint request, which
/// carries the image to the worker as a transient resume's does.
fn bind_transient_states(
    requests: &mut [HeadlessTaskRequest],
    positions: &HashMap<AnalysisInstanceId, usize>,
    read_checkpoint: &CheckpointReader<'_>,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> ServiceRunResult<()> {
    for index in 0..requests.len() {
        ensure_not_aborted(abort)?;
        let request = &requests[index];
        let Some(keyword) = small_signal_state_keyword(&request.analysis.analysis_line) else {
            continue;
        };
        let context = |message: String| {
            invalid(format!(
                "Analysis {}: its card carries {keyword}=, which linearizes it around a \
                 transient state, and {message}",
                request.instance_id
            ))
        };
        if !matches!(
            request.analysis.spec,
            AnalysisSpec::Ac { .. } | AnalysisSpec::Noise { .. }
        ) {
            return Err(context(
                "a task runs .AC and .NOISE cards around a state; run this card at a state \
                 with `rspice run`"
                    .to_owned(),
            ));
        }
        let mut parents = request
            .dependencies
            .iter()
            .filter_map(|dependency| positions.get(dependency).copied())
            .filter(|&position| {
                matches!(
                    requests[position].analysis.spec,
                    AnalysisSpec::Transient { .. }
                )
            });
        let (Some(parent), None) = (parents.next(), parents.next()) else {
            return Err(context(
                "a task names the transient whose state it is as its one transient dependency"
                    .to_owned(),
            ));
        };
        let line = format!(
            "{}\n{}",
            requests[parent].analysis.analysis_line.trim(),
            request.analysis.analysis_line.trim()
        );
        let netlist = rspice_core::Netlist::parse(&format!("* transient state\n{line}\n.end\n"))
            .map_err(|error| {
                context(format!(
                    "the card does not read with its transient's: {error}"
                ))
            })?;
        let checkpoint = check_state_card(&netlist, &request.analysis.spec).map_err(context)?;
        let resume = match (checkpoint, &request.analysis.spec_options.tran_checkpoint) {
            (None, None) => None,
            // The caller sealed the state the card names with the request.
            (Some(_), Some(sealed)) if sealed.times.is_empty() && sealed.resume.is_some() => None,
            (Some(name), None) => {
                let bytes = read_checkpoint(name).map_err(context)?;
                Some(
                    TransientCheckpointInput::from_bytes_with_limits(bytes, limits, abort)
                        .map_err(|error| {
                            context(format!("its checkpoint file {name} does not read: {error}"))
                        })?,
                )
            }
            (_, Some(_)) => {
                return Err(context(
                    "a task's transient checkpoint request is the state its CHECKPOINT= card \
                     reads, with no schedule"
                        .to_owned(),
                ));
            }
        };
        let request = &mut requests[index].analysis;
        request.analysis_line = line;
        if let Some(resume) = resume {
            request.spec_options.tran_checkpoint = Some(TransientCheckpointRequest {
                times: Vec::new(),
                resume: Some(resume),
            });
        }
    }
    Ok(())
}

/// Check that a state task's deck is its transient's one `.TRAN` card and
/// its own card around one state, and that its card is its typed request:
/// the engine runs the card, so the two must agree. Returns the checkpoint
/// file a `CHECKPOINT=` card names.
fn check_state_card<'a>(
    netlist: &'a rspice_core::Netlist,
    spec: &AnalysisSpec,
) -> Result<Option<&'a str>, String> {
    use rspice_core::netlist::{AnalysisCommand, SmallSignalStateSource};
    let (
        [AnalysisCommand::Tran { .. }, card],
        [
            rspice_core::netlist::SmallSignalStateRequest {
                analysis_index: 1,
                source,
            },
        ],
    ) = (
        netlist.analyses.as_slice(),
        netlist.small_signal_states.as_slice(),
    )
    else {
        return Err(
            "its deck is not one .TRAN card, its transient's, and then its own one card".to_owned(),
        );
    };
    let checkpoint = match source {
        SmallSignalStateSource::TransientTimes(times) if times.len() == 1 => None,
        SmallSignalStateSource::TransientTimes(times) => {
            return Err(format!(
                "the card names {} times, and a task is one state: give each time a task of its own",
                times.len()
            ));
        }
        SmallSignalStateSource::CheckpointFile(name) => Some(name.as_str()),
    };
    let lowered = crate::manual_deck::command_to_queue_item(27.0, netlist, card)?.spec;
    if same_request(lowered, spec) {
        Ok(checkpoint)
    } else {
        Err(
            "the card is not the task's typed specification; the engine runs the card, so the \
             two must agree"
                .to_owned(),
        )
    }
}

/// Whether the specification a card lowers to is the task's. A noise task's
/// contribution detail and integration are policies of the result's
/// projection, not of the card, and its temperature is checked against the
/// circuit's where the card runs.
fn same_request(lowered: AnalysisSpec, requested: &AnalysisSpec) -> bool {
    match (lowered, requested) {
        (
            AnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                start_freq,
                stop_freq,
                points_per_decade,
                sweep,
                explicit_frequencies,
                data_table_name,
                ..
            },
            AnalysisSpec::Noise {
                contribution_detail,
                integration_mode,
                temperature,
                ..
            },
        ) => {
            AnalysisSpec::Noise {
                output_node,
                reference_node,
                input_source,
                start_freq,
                stop_freq,
                points_per_decade,
                sweep,
                explicit_frequencies,
                data_table_name,
                contribution_detail: *contribution_detail,
                integration_mode: *integration_mode,
                temperature: *temperature,
            } == *requested
        }
        (lowered, requested) => lowered == *requested,
    }
}

/// The transient-state keyword a small-signal card in `analysis_line` carries:
/// `ACTIMES=` or `CHECKPOINT=`, spelled with or without space around `=`.
fn small_signal_state_keyword(analysis_line: &str) -> Option<&'static str> {
    const CARDS: [&str; 8] = [
        ".ac", ".noise", ".tf", ".sp", ".stb", ".pz", ".sens", ".disto",
    ];
    for (_, line) in crate::netlist_preparation::executable_logical_lines(analysis_line) {
        let spaced = line.replace('=', " = ");
        let mut words = spaced.split_whitespace();
        let Some(card) = words.next() else {
            continue;
        };
        if !CARDS.iter().any(|name| card.eq_ignore_ascii_case(name)) {
            continue;
        }
        let words = words.collect::<Vec<_>>();
        for pair in words.windows(2) {
            if pair[1] != "=" {
                continue;
            }
            if pair[0].eq_ignore_ascii_case("actimes") {
                return Some("ACTIMES");
            }
            if pair[0].eq_ignore_ascii_case("checkpoint") {
                return Some("CHECKPOINT");
            }
        }
    }
    None
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
