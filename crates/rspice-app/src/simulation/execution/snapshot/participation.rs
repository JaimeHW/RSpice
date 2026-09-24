//! Expanding the plan across the Studio's global Run Set, one task per point
//! the analysis declared itself at.
//!
//! The point identity is part of the task identity, configuration digest,
//! prepared source, result attribution, and dependency mapping. Keeping the
//! expansion here — rather than a nominal task accompanied by descriptive PVT
//! metadata — is what makes the forecasted matrix and the authorized worker
//! queue one actual expansion.
//!
//! Participation is resolved here — [`resolve_run_set_participation`] turns
//! the keys [`crate::simulation::run_set::participating_point_keys`] returns
//! into the positions the expansion indexes by — and it is resolved in full
//! before anything is minted. A point an analysis does not declare itself
//! at simply does not mint a task: no identity is derived for it, so it
//! cannot be dispatched, cannot appear in a receipt, and cannot be re-derived
//! at project load. That is the whole of the mechanism — there is no second place where a
//! task is filtered out after being built, because a task that was built and
//! then dropped would already have consumed an identity.

use std::collections::{HashMap, HashSet};

use super::*;

/// Which positions of the declared space each authored analysis runs at.
///
/// Keyed by the identity a task carries *before* expansion, which is the
/// authored instance for a plan analysis and the derivation for a companion
/// such as a PSS spectrum. Every task reaching the expansion has an entry;
/// an analysis missing from the map is a preparation bug, and is reported as
/// one rather than silently run everywhere.
pub(super) type RunSetParticipation = HashMap<AnalysisInstanceId, HashSet<usize>>;

pub(super) fn run_set_point_task_label(
    original_label: &str,
    point: &PreparedPvtPoint,
    point_index: usize,
    point_count: usize,
) -> String {
    let mut label = format!(
        "{original_label} \u{00b7} point {}/{} \u{00b7} {}",
        point_index + 1,
        point_count,
        point.process.short_name(),
    );
    if let Some(voltage) = point.voltage {
        label.push_str(&format!(" \u{00b7} {voltage} V"));
    }
    label.push_str(&format!(
        " \u{00b7} {} \u{00b0}C",
        point.temperature_celsius
    ));
    for (name, value) in &point.parameter_overrides {
        label.push_str(&format!(" \u{00b7} param {name}={value}"));
    }
    for (name, value) in &point.source_overrides {
        label.push_str(&format!(" \u{00b7} source {name}={value}"));
    }
    label
}

/// Expand every ordinary plan analysis across the points it declared itself at.
pub(super) fn expand_global_run_set_tasks(
    tasks: Vec<PreparedTask>,
    pvt_points: &[PreparedPvtPoint],
    participation: &RunSetParticipation,
    executable_netlist: &str,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<Vec<PreparedTask>, PreparationError> {
    if pvt_points.is_empty() {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Global Run Set did not produce any execution points",
        ));
    }
    let mut requested = 0usize;
    for task in &tasks {
        let visited = participating_positions(task, participation)?;
        requested = requested.checked_add(visited.len()).ok_or_else(|| {
            PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Global Run Set task count overflowed the platform task capacity",
            )
        })?;
    }
    ensure_pvt_point_capacity(
        0,
        requested,
        rspice_core::ResourceLimits::default().max_batch_runs,
    )?;

    // A Temperature or Corner analysis owns another point declaration. Its
    // implicit cross-product with this global declaration would make both the
    // forecast and result-family authority ambiguous, so it remains an
    // explicit refusal. Ordinary spec-driven analyses receive their global
    // point through an authenticated deck below.
    //
    // The refusal names both sides — the instance by its shown name with its
    // identity beside it, and the declared space by its size — because "one of
    // these two has to own the space" is only actionable if the reader is told
    // which two. A message naming the analysis alone leaves them looking for a
    // nested declaration on a form that no longer has one.
    if let Some(task) = tasks.iter().find(|task| {
        matches!(task.task.spec, AnalysisSpec::Corner)
            || matches!(task.task.spec, AnalysisSpec::Parametric)
                && task.task.spec_options.temp.is_some()
    }) {
        return Err(PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "{} ({}) expands its own points, and the plan's global multi-point Run Set \
                 declares {}. Both would expand the same run: disable the global axes to let \
                 this analysis own the space, or disable this analysis to let the Run Set own it.",
                task.label,
                task.instance_id,
                point_count(pvt_points.len()),
            ),
        ));
    }

    // What each authored analysis is called, taken from the labels the plan
    // already seeded from `display_name`. A refusal that named a prerequisite
    // by its UUID would be telling the reader to widen something they would
    // then have to go and look up.
    let names: HashMap<AnalysisInstanceId, String> = tasks
        .iter()
        .map(|task| (task.instance_id, task.label.clone()))
        .collect();

    let mut prepared_sources = Vec::with_capacity(pvt_points.len());
    for point in pvt_points {
        prepared_sources.push(prepare_pvt_point_source(executable_netlist, point)?);
    }

    let mut identities = HashMap::with_capacity(requested);
    let mut derivations = HashMap::with_capacity(requested);
    for task in &tasks {
        for point_index in participating_positions(task, participation)? {
            if pvt_points.len() == 1 {
                identities.insert((task.instance_id, point_index), task.instance_id);
                continue;
            }
            let derivation = task.derive(global_run_set_point_role(
                point_index,
                &pvt_points[point_index],
            ));
            identities.insert((task.instance_id, point_index), derivation.instance_id());
            derivations.insert((task.instance_id, point_index), derivation);
        }
    }

    let mut expanded = Vec::with_capacity(requested);
    let mut task_points = Vec::with_capacity(requested);
    for task in tasks {
        let original_identity = task.instance_id;
        let original_label = task.label.clone();
        for point_index in participating_positions(&task, participation)? {
            let point = &pvt_points[point_index];
            let mut point_task = task.clone();
            if let Some(derivation) = derivations.get(&(original_identity, point_index)) {
                point_task.adopt_derived_identity(derivation.clone());
            }
            point_task.dependencies = task
                .dependencies
                .iter()
                .map(|dependency| {
                    identities
                        .get(&(*dependency, point_index))
                        .copied()
                        .ok_or_else(|| {
                            missing_producer(&original_label, &names, *dependency, point_index)
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;

            let (source_override, nominal_supply_voltage) = &prepared_sources[point_index];
            point_task.executable_netlist_override = source_override.clone();
            point_task.pvt_point = Some(
                crate::state::AnalysisResultPvtPoint::new(
                    point.process.short_name(),
                    point.voltage,
                    point.temperature_celsius,
                    point.corner_contract.as_ref().map(corner_contract_digest),
                    point_is_nominal(
                        point,
                        *nominal_supply_voltage,
                        reference_process,
                        reference_temperature_celsius,
                    ),
                )
                .map_err(|error| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        format!(
                            "Run Set point {}/{} cannot be attributed: {error}",
                            point_index + 1,
                            pvt_points.len()
                        ),
                    )
                })?,
            );

            if let Some(mut config) = operating_point_config(&point_task.task.spec) {
                use crate::simulation::dialog::OpRunPointContext;
                config.temperature_celsius = point.temperature_celsius;
                config.run_point = OpRunPointContext {
                    index: point_index,
                    count: pvt_points.len(),
                    process: point.process,
                    supply_voltage: point.voltage,
                    nominal_supply_voltage: *nominal_supply_voltage,
                    supply_source_names: point.supply_source_names.clone(),
                };
                point_task.task.spec = operating_point_spec(&config);
                point_task.task.config = Some(crate::simulation::AnalysisConfig::DcOp(config));
                point_task.execution_environment = None;
            } else {
                if let Some(crate::simulation::AnalysisConfig::Noise(config)) =
                    point_task.task.config.as_mut()
                {
                    config.temperature_kelvin =
                        rspice_core::constants::celsius_to_kelvin(point.temperature_celsius);
                }
                if let AnalysisSpec::Noise { temperature, .. } = &mut point_task.task.spec {
                    *temperature =
                        rspice_core::constants::celsius_to_kelvin(point.temperature_celsius);
                }
                let environment = crate::simulation::runner::AnalysisExecutionEnvironment {
                    temperature_celsius: point.temperature_celsius,
                    supply_voltage: point.voltage,
                    nominal_supply_voltage: *nominal_supply_voltage,
                    supply_source_names: point.supply_source_names.clone(),
                };

                // Configuration-backed analyses and Monte Carlo apply this
                // environment to their parsed deck at dispatch. A shooting
                // PSS applies the identical values from its same-point DC seed
                // artifact. Every other spec-driven service consumes normal
                // deck options, so freeze temperature and scaled supplies into
                // that task's authenticated source now.
                let materialize_environment = point_task.task.config.is_none()
                    && !matches!(point_task.task.spec, AnalysisSpec::MonteCarlo { .. })
                    && !matches!(
                        point_task.task.spec,
                        AnalysisSpec::Pss {
                            method: crate::simulation::multi_run::PssMethod::Shooting,
                            ..
                        }
                    );
                if materialize_environment {
                    let deck = point_task
                        .executable_netlist_override
                        .as_deref()
                        .unwrap_or(executable_netlist);
                    point_task.executable_netlist_override =
                        Some(materialize_spec_run_environment_source(
                            deck,
                            &environment,
                            point_index,
                            pvt_points.len(),
                        )?);
                }
                point_task.execution_environment = Some(environment);
            }

            point_task.label =
                run_set_point_task_label(&original_label, point, point_index, pvt_points.len());
            let point_instance_id = point_task.instance_id;
            point_task.saved_output_contracts = task
                .saved_output_contracts
                .iter()
                .map(|contract| {
                    contract
                        .rebind_analysis(point_instance_id, &point_task.task.spec)
                        .map_err(|error| {
                            PreparationError::new(
                                PreparationStage::AnalysisPlan,
                                format!(
                                    "Failed to bind saved output to Run Set point {}/{}: {error}",
                                    point_index + 1,
                                    pvt_points.len()
                                ),
                            )
                        })
                })
                .collect::<Result<Vec<_>, _>>()?;
            point_task.config_digest = point_task.payload_digest();
            task_points.push((original_identity, point_index));
            expanded.push(point_task);
        }
    }

    let producer_details = expanded
        .iter()
        .map(|task| (task.instance_id, (task.source_revision, task.config_digest)))
        .collect::<HashMap<_, _>>();
    for (task, (_, point_index)) in expanded.iter_mut().zip(task_points) {
        let consumer_label = task.label.clone();
        for binding in &mut task.dependency_bindings {
            let producer = identities
                .get(&(binding.producer_instance_id(), point_index))
                .copied()
                .ok_or_else(|| {
                    missing_producer(
                        &consumer_label,
                        &names,
                        binding.producer_instance_id(),
                        point_index,
                    )
                })?;
            let (revision, digest) = producer_details[&producer];
            binding.rebind_producer(producer, revision, digest);
        }
    }

    Ok(expanded)
}

/// The positions of the declared space `task` runs at, in declared order.
fn participating_positions(
    task: &PreparedTask,
    participation: &RunSetParticipation,
) -> Result<Vec<usize>, PreparationError> {
    let visited = participation.get(&task.instance_id).ok_or_else(|| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            format!(
                "Run Set task {} reached expansion without a resolved participation",
                task.label
            ),
        )
    })?;
    let mut positions: Vec<usize> = visited.iter().copied().collect();
    positions.sort_unstable();
    Ok(positions)
}

/// A dependent runs at a point its prerequisite does not.
///
/// This is the one composition participation makes possible and cannot resolve:
/// binding the consumer to the prerequisite's *other* point would cross two
/// conditions in one result and call it one solve, and dropping the consumer's
/// task would silently narrow a participation the operator declared. So it is a
/// refusal, and it names both sides — the narrowed prerequisite is the thing to
/// widen.
fn missing_producer(
    consumer_label: &str,
    names: &HashMap<AnalysisInstanceId, String>,
    producer: AnalysisInstanceId,
    point_index: usize,
) -> PreparationError {
    // The identity is kept beside the name rather than replaced by it: a name
    // is what the reader recognises, and the identity is what they can search
    // the plan for when two analyses have been given the same words.
    let named = names.get(&producer).map_or_else(
        || producer.to_string(),
        |label| format!("{label} ({producer})"),
    );
    PreparationError::new(
        PreparationStage::AnalysisPlan,
        format!(
            "{consumer_label} runs at Run Set point {}, but its prerequisite {named} does not. \
             Widen the prerequisite's run-set participation to cover every point its dependents \
             run at, or narrow the dependent to the points the prerequisite visits.",
            point_index + 1
        ),
    )
}

/// "1 point" or "N points", so a refusal reads as a sentence.
fn point_count(count: usize) -> String {
    if count == 1 {
        "1 point".to_owned()
    } else {
        format!("{count} points")
    }
}

/// Which positions of the declared space each prepared task runs at.
///
/// One entry per task, or a refusal naming the analysis. The resolution itself
/// belongs to `run_set::participating_point_keys` — this only turns the keys it
/// returns into the positions the expansion indexes by, and attaches the
/// analysis label to whatever it refuses, because "point selection resolves to
/// nothing" is unactionable without the name of the analysis that holds it.
pub(super) fn resolve_run_set_participation(
    tasks: &[PreparedTask],
    pvt_points: &[PreparedPvtPoint],
    run_set: Option<&PreparedRunSet>,
    reference_process: ProcessCorner,
    reference_temperature_celsius: f64,
) -> Result<RunSetParticipation, PreparationError> {
    let run_set = run_set.ok_or_else(|| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Global Run Set expansion was requested without a prepared Run Set",
        )
    })?;
    let points = crate::simulation::run_set::resolve(&run_set.state).ok_or_else(|| {
        PreparationError::new(
            PreparationStage::AnalysisPlan,
            "Run Set could not be expanded into exact execution points",
        )
    })?;
    let positions: HashMap<&str, usize> = pvt_points
        .iter()
        .enumerate()
        .filter_map(|(index, point)| point.run_set_point_key.as_deref().map(|key| (key, index)))
        .collect();
    let reference = crate::simulation::run_set::ReferencePoint {
        process: reference_process,
        temperature_celsius: reference_temperature_celsius,
    };

    let mut resolved = RunSetParticipation::with_capacity(tasks.len());
    for task in tasks {
        let keys =
            crate::simulation::run_set::participating_point_keys(&task.run_at, &points, reference)
                .map_err(|refusal| {
                    PreparationError::new(
                        PreparationStage::AnalysisPlan,
                        // Named against the declaration, so the refusal the
                        // preparation reports and the one the studio's point
                        // table prints spell the same points the same way.
                        format!(
                            "{}: {}",
                            task.label,
                            refusal.named_in(&run_set.state).message
                        ),
                    )
                })?;
        let mut visited = HashSet::with_capacity(keys.len());
        for key in keys {
            let position = positions.get(key.as_str()).copied().ok_or_else(|| {
                PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    format!(
                        "{} is scoped to Run Set point {key}, which the prepared point list does \
                         not contain",
                        task.label
                    ),
                )
            })?;
            visited.insert(position);
        }
        resolved.insert(task.instance_id, visited);
    }
    Ok(resolved)
}
