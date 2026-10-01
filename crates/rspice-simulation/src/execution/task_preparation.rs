//! Prepare task identities, dependency bindings and saved-output contracts.

use super::snapshot::bound_cards;
use super::{PreparationError, PreparationStage, PreparedTask};
use crate::analysis_preparation::AnalysisInputs;
use crate::execution_artifact::PreparedDependencyBinding;
use crate::execution_identity::{analysis_kind_tag, manual_deck_analysis_instance_id};
use crate::monte_carlo_checkpoint::preparation::resume_inputs_from_config;
use crate::preparation::QueuedAnalysis;
use crate::prepared_dependency::ExecutionArtifactKind;
use rspice_simulation_contract::analysis_draft::AnalysisDraft;
use rspice_simulation_contract::analysis_kind::AnalysisKind;
use rspice_simulation_contract::analysis_spec::AnalysisSpec;
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_simulation_contract::plan_model::FrozenSimulationPlan;
use std::collections::{HashMap, HashSet};

pub fn prepare_plan_tasks<'a, R, A, W>(
    inputs: AnalysisInputs<'a, R, A>,
    plan: &FrozenSimulationPlan,
    sealed_model_sources: &crate::model_sources::SealedModelExecutionSources,
    imported_checkpoints: &'a rspice_results::monte_carlo_checkpoint::MonteCarloCheckpointLibrary,
    source_path: Option<&std::path::Path>,
) -> Result<Vec<PreparedTask>, Vec<String>>
where
    R: AsRef<rspice_results::run::SimulationRun<A>>,
    A: AsRef<rspice_results::analysis_result::AnalysisResult<W>> + 'a,
    W: AsRef<rspice_results::waveform::RetainedWaveform> + 'a,
{
    let mut queue = Vec::with_capacity(plan.instances().len());
    let mut errors = Vec::new();

    for instance in plan.instances() {
        if let Some(reason) = instance.kind().execution_blocker() {
            errors.push(format!("{}: {reason}", instance.display_name()));
            continue;
        }
        let projected_setup = match inputs.sim_setup.frozen_instance_projection(plan, instance) {
            Ok(projection) => projection,
            Err(error) => {
                errors.push(format!("{}: {error}", instance.display_name()));
                continue;
            }
        };
        let projected_state = AnalysisInputs {
            sim_setup: &projected_setup,
            ..inputs
        };
        let dependency_ids = instance
            .dependencies()
            .iter()
            .map(|dependency| dependency.target())
            .collect();

        // Resolve the same exact draft and contextual prerequisite closure
        // used by the displayed directive. The queue and preview must
        // agree on the spec that reaches the engine.
        let spec = match crate::analysis_preparation::analysis_draft_spec(
            &projected_state,
            instance.draft(),
        ) {
            Ok(spec) => spec,
            Err(error) => {
                errors.push(format!("{}: {error}", instance.display_name()));
                continue;
            }
        };
        let analysis_line = match crate::analysis_preparation::analysis_spec_to_spice_line(
            &projected_state,
            instance.draft(),
            &spec,
        ) {
            Ok(line) => line,
            Err(e) => {
                errors.push(format!("{}: {}", instance.display_name(), e));
                continue;
            }
        };
        let periodic_producer =
            match crate::analysis_preparation::bound_periodic_producer(plan, instance) {
                Ok(producer) => producer,
                Err(error) => {
                    errors.push(error);
                    continue;
                }
            };
        let mut spec_options = match crate::analysis_preparation::analysis_spec_execution_options(
            projected_state.sim_setup,
            instance.draft(),
            periodic_producer.map(|producer| producer.draft()),
            &spec,
            sealed_model_sources,
        ) {
            Ok(opts) => opts,
            Err(e) => {
                errors.push(format!("{}: {}", instance.display_name(), e));
                continue;
            }
        };

        if instance.draft().pvt_base_analysis().is_some() {
            let mode = spec_options
                .temp
                .as_mut()
                .map(|config| &mut config.base_mode)
                .or_else(|| {
                    spec_options
                        .corner
                        .as_mut()
                        .map(|config| &mut config.base_mode)
                });
            if let Some(mode @ crate::sweeps::CornerBaseMode::Op) = mode {
                let config = instance
                    .draft()
                    .pvt_base_analysis()
                    .and_then(|base_id| {
                        plan.instances()
                            .iter()
                            .find(|candidate| candidate.id() == base_id)
                    })
                    .ok_or_else(|| {
                        format!(
                            "{} has no bound operating-point base",
                            instance.display_name()
                        )
                    })
                    .and_then(|base| match base.draft() {
                        AnalysisDraft::OperatingPoint(draft) => {
                            crate::analysis_preparation::build_op_spec(&projected_state, draft)
                        }
                        _ => Err(format!(
                            "{} has a non-operating-point base while OP mode is selected",
                            instance.display_name()
                        )),
                    })
                    .and_then(|spec| crate::analysis_preparation::analysis_spec_to_config(&spec));
                match config {
                    Ok(AnalysisConfig::DcOp(config)) => {
                        *mode = crate::sweeps::CornerBaseMode::ConfiguredOp(Box::new(config));
                    }
                    Ok(_) => {
                        unreachable!("the operating-point builder returns an OP configuration")
                    }
                    Err(error) => {
                        errors.push(format!("{}: {error}", instance.display_name()));
                        continue;
                    }
                }
            }
        }

        if matches!(
            instance.draft(),
            AnalysisDraft::MonteCarlo(_) | AnalysisDraft::Optimization(_)
        ) {
            match crate::analysis_preparation::compile_study_base(&inputs, plan, instance.draft()) {
                Ok(base) => spec_options.study_base = base,
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.display_name()));
                    continue;
                }
            }
        }

        let monte_carlo_resumes = if let AnalysisDraft::MonteCarlo(draft) = instance.draft() {
            match draft.to_config().and_then(|config| {
                resume_inputs_from_config(
                    inputs
                        .runs
                        .iter()
                        .flat_map(|run| run.as_ref().analyses.iter().map(AsRef::as_ref)),
                    imported_checkpoints,
                    config.checkpoint.as_ref(),
                )
            }) {
                Ok(resumes) => resumes,
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.display_name()));
                    continue;
                }
            }
        } else {
            Vec::new()
        };

        // A PSS request that asks to retain harmonics earns a second
        // prepared task for them. It is a task in its own right, with its
        // own identity and config digest, rather than a second result
        // smuggled out of the PSS task: harmonics are indexed by frequency
        // and the periodic waveform by time, so one analysis cannot carry
        // both, and aliasing the PSS task's authored identity would make
        // `find_analysis_by_source_instance` resolve the spectrum in its
        // place for every dependent analysis and retained pane binding.
        let spectrum_seed = match &spec {
            AnalysisSpec::Pss { num_harmonics, .. } if *num_harmonics > 0 => Some((
                *num_harmonics,
                analysis_line.clone(),
                spec_options.clone(),
                instance.id(),
                instance.display_name().to_owned(),
            )),
            _ => None,
        };

        // The analysis's own numerics travel with the task rather than
        // being resolved here: snapshot preparation owns the seam where a
        // task's deck is written, and it is the only place that can splice
        // them after per-point expansion has chosen that deck.
        let numeric_override = if let Some(carrier) = periodic_producer {
            // Reusing an orbit requires its authenticated numerical
            // circuit, including every producer-local .OPTIONS package.
            // Consumer response controls travel separately in its spec.
            carrier.numeric_override().cloned()
        } else if let Some(base_id) = instance.draft().pvt_base_analysis() {
            let base = plan
                .instances()
                .iter()
                .find(|base| base.id() == base_id)
                .expect("PVT base was resolved by frozen_instance_projection");
            let mut options = instance.numeric_override().cloned().unwrap_or_default();
            if let Some(base_options) = base.numeric_override() {
                options = options.with_base_options(base_options);
            }
            (!options.is_empty()).then_some(options)
        } else {
            instance.numeric_override().cloned()
        };

        let task = if executes_via_spec(&spec) {
            QueuedAnalysis {
                spec,
                config: None,
                spec_options,
                analysis_line,
                numeric_override: numeric_override.clone(),
            }
        } else {
            match crate::analysis_preparation::analysis_spec_to_config(&spec) {
                Ok(config) => {
                    if let Err(errs) = config.validate() {
                        errors.push(format!(
                            "{} config is invalid: {}",
                            instance.display_name(),
                            errs.join(", ")
                        ));
                        continue;
                    } else {
                        QueuedAnalysis {
                            spec,
                            config: Some(config),
                            spec_options,
                            analysis_line,
                            numeric_override: numeric_override.clone(),
                        }
                    }
                }
                Err(e) => {
                    errors.push(format!("{}: {}", instance.display_name(), e));
                    continue;
                }
            }
        };
        let run_at = instance.run_at().clone();
        let mut prepared = PreparedTask::new(
            instance.id(),
            plan.revision(),
            dependency_ids,
            instance.display_name(),
            task,
        )
        .with_run_at(run_at.clone())
        .with_monte_carlo_resumes(monte_carlo_resumes);
        if instance.kind() == AnalysisKind::SParameter {
            match crate::preparation::touchstone::touchstone_export_policy_for_dialog(
                &projected_state.sim_setup.sp,
                inputs.schematic,
                source_path,
            ) {
                Ok(policy) => {
                    prepared = prepared.with_touchstone_export_policy(policy);
                }
                Err(error) => {
                    errors.push(format!("{}: {error}", instance.display_name()));
                    continue;
                }
            }
        }
        queue.push(prepared);

        if let Some((num_harmonics, analysis_line, spec_options, producer, label)) = spectrum_seed {
            queue.push(
                PreparedTask::derived(
                    // Derived from the PSS it reads, not minted: the prepared
                    // snapshot digest covers task identity, so a fresh id
                    // would make an unchanged plan prepare differently every
                    // time and expire its own authorization at dispatch. The
                    // derivation travels on the task so a saved project can
                    // re-derive it from the PSS the plan authored.
                    producer,
                    super::PSS_SPECTRUM_ROLE,
                    plan.revision(),
                    vec![producer],
                    format!("{label} Spectrum"),
                    QueuedAnalysis {
                        spec: AnalysisSpec::PssSpectrum { num_harmonics },
                        config: None,
                        spec_options,
                        analysis_line,
                        // The spectrum is the same authored PSS solve read at
                        // a different index, so it resolves under the same
                        // numerics; a second task under the plan policy would
                        // report harmonics of a solve that never happened.
                        numeric_override,
                    },
                )
                // And at the same points. A spectrum that ran everywhere while
                // the PSS it reads ran at one point would be asking for
                // harmonics of solves that were never dispatched.
                .with_run_at(run_at),
            );
        }
    }

    if !errors.is_empty() {
        Err(errors)
    } else {
        // Before the producer identities are taken, because attaching a
        // card to a transient changes that transient's payload digest —
        // and the bindings below capture the digest a dependent must see.
        // A `.fft` card makes every requested sample time a solver stop,
        // so it is part of the solve it rides on, not an observation of it.
        bound_cards::attach_bound_observation_cards(&mut queue);
        let producer_identities = queue
            .iter()
            .map(|task| {
                (
                    task.instance_id(),
                    (
                        task.source_revision(),
                        task.config_digest(),
                        matches!(task.queued_analysis().spec, AnalysisSpec::Transient { .. }),
                        matches!(task.queued_analysis().spec, AnalysisSpec::Pss { .. }),
                        matches!(
                            task.queued_analysis().spec,
                            AnalysisSpec::HarmonicBalance { .. }
                        ),
                        matches!(
                            task.queued_analysis().spec,
                            AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
                        ),
                        matches!(
                            task.queued_analysis().spec,
                            AnalysisSpec::Qpss {
                                autonomous: false,
                                ..
                            }
                        ),
                    ),
                )
            })
            .collect::<HashMap<_, _>>();
        for task in &mut queue {
            // The kinds this task's request admits, in preference order.
            // A periodic small-signal request whose carrier is the
            // preceding periodic solve admits either family, and the one
            // it binds is whichever family the plan's own edge points at.
            let required_kinds = crate::prepared_dependency::required_artifact_kinds(
                &task.queued_analysis().spec,
                &task.queued_analysis().spec_options,
            );
            if required_kinds.is_empty() {
                continue;
            }
            let producers = task
                .dependencies()
                .iter()
                .filter_map(|dependency| {
                    let (revision, config_digest, transient, pss, hb, op, qpss) =
                        producer_identities.get(dependency)?;
                    let kind = required_kinds.iter().copied().find(|kind| match kind {
                        ExecutionArtifactKind::TransientTrajectory => *transient,
                        ExecutionArtifactKind::PeriodicState => *pss,
                        ExecutionArtifactKind::HbState => *hb,
                        ExecutionArtifactKind::QpssState => *qpss,
                        ExecutionArtifactKind::DcOperatingPointSeed => *op,
                    })?;
                    Some(match kind {
                        ExecutionArtifactKind::TransientTrajectory => {
                            PreparedDependencyBinding::transient_trajectory(
                                *dependency,
                                *revision,
                                *config_digest,
                            )
                        }
                        ExecutionArtifactKind::PeriodicState => {
                            PreparedDependencyBinding::periodic_state(
                                *dependency,
                                *revision,
                                *config_digest,
                            )
                        }
                        ExecutionArtifactKind::QpssState => PreparedDependencyBinding::qpss_state(
                            *dependency,
                            *revision,
                            *config_digest,
                        ),
                        ExecutionArtifactKind::HbState => PreparedDependencyBinding::hb_state(
                            *dependency,
                            *revision,
                            *config_digest,
                        ),
                        ExecutionArtifactKind::DcOperatingPointSeed => {
                            PreparedDependencyBinding::dc_operating_point_seed(
                                *dependency,
                                *revision,
                                *config_digest,
                            )
                        }
                    })
                })
                .collect::<Vec<_>>();
            if producers.len() != 1 {
                errors.push(format!(
                    "{} must bind exactly one prepared {} task, found {}",
                    task.queued_analysis().spec.run_type().display_name(),
                    required_kinds
                        .iter()
                        .map(|kind| kind.producer_label())
                        .collect::<Vec<_>>()
                        .join(" or "),
                    producers.len()
                ));
            } else {
                task.set_dependency_bindings(producers);
            }
        }
        if errors.is_empty() {
            Ok(queue)
        } else {
            Err(errors)
        }
    }
}

fn executes_via_spec(spec: &AnalysisSpec) -> bool {
    matches!(
        spec,
        AnalysisSpec::Tf { .. }
            | AnalysisSpec::AcData { .. }
            | AnalysisSpec::Disto { .. }
            | AnalysisSpec::Pnoise
            | AnalysisSpec::Pxf
            | AnalysisSpec::Pstb
            | AnalysisSpec::Stb { .. }
            | AnalysisSpec::MonteCarlo { .. }
            | AnalysisSpec::Parametric
            | AnalysisSpec::Corner
            | AnalysisSpec::Pss { .. }
            | AnalysisSpec::PssSpectrum { .. }
            | AnalysisSpec::HarmonicBalance { .. }
            | AnalysisSpec::Pac
            | AnalysisSpec::SParameter { .. }
            | AnalysisSpec::Envelope { .. }
            | AnalysisSpec::Fourier { .. }
            | AnalysisSpec::Fft { .. }
            | AnalysisSpec::Optimization { .. }
            | AnalysisSpec::Soa { .. }
            | AnalysisSpec::Qpss { .. }
            | AnalysisSpec::Hbsp { .. }
            | AnalysisSpec::Hbnoise { .. }
            | AnalysisSpec::Psp { .. }
            | AnalysisSpec::Qpac { .. }
            | AnalysisSpec::Qpnoise { .. }
            | AnalysisSpec::Qpxf { .. }
            | AnalysisSpec::TransientNoise { .. }
            | AnalysisSpec::DcMismatch { .. }
    )
}

pub(crate) fn prepare_manual_tasks(
    expanded_source_identity: rspice_app_types::product::ContentDigest,
    source_revision: rspice_app_types::product::ObjectRevision,
    tasks: Vec<QueuedAnalysis>,
) -> Result<Vec<PreparedTask>, PreparationError> {
    let mut kind_occurrences = std::collections::HashMap::<u8, usize>::new();
    let mut prepared = tasks
        .into_iter()
        .map(|task| {
            let occurrence = kind_occurrences
                .entry(analysis_kind_tag(&task.spec))
                .or_default();
            let current_occurrence = *occurrence;
            *occurrence += 1;
            let instance_id = manual_deck_analysis_instance_id(
                expanded_source_identity,
                &task.spec,
                current_occurrence,
            );
            let label = task.spec.run_type().display_name();
            PreparedTask::new(instance_id, source_revision, Vec::new(), label, task)
        })
        .collect::<Vec<_>>();

    let transient_producers = prepared
        .iter()
        .filter(|task| matches!(&task.queued_analysis().spec, AnalysisSpec::Transient { .. }))
        .map(|task| {
            (
                task.instance_id(),
                task.source_revision(),
                task.config_digest(),
            )
        })
        .collect::<Vec<_>>();
    let periodic_producers = prepared
        .iter()
        .filter(|task| matches!(&task.queued_analysis().spec, AnalysisSpec::Pss { .. }))
        .map(|task| {
            (
                task.instance_id(),
                task.source_revision(),
                task.config_digest(),
            )
        })
        .collect::<Vec<_>>();
    let qpss_producers = prepared
        .iter()
        .filter(|task| {
            matches!(
                &task.queued_analysis().spec,
                AnalysisSpec::Qpss {
                    autonomous: false,
                    ..
                }
            )
        })
        .map(|task| {
            (
                task.instance_id(),
                task.source_revision(),
                task.config_digest(),
            )
        })
        .collect::<Vec<_>>();
    let harmonic_balance_producers = prepared
        .iter()
        .filter(|task| {
            matches!(
                &task.queued_analysis().spec,
                AnalysisSpec::HarmonicBalance { .. }
            )
        })
        .map(|task| {
            (
                task.instance_id(),
                task.source_revision(),
                task.config_digest(),
            )
        })
        .collect::<Vec<_>>();
    let operating_point_producers = prepared
        .iter()
        .filter(|task| {
            matches!(
                &task.queued_analysis().spec,
                AnalysisSpec::LegacyDcOp | AnalysisSpec::DcOp { .. }
            )
        })
        .map(|task| {
            (
                task.instance_id(),
                task.source_revision(),
                task.config_digest(),
            )
        })
        .collect::<Vec<_>>();
    for task in &mut prepared {
        if matches!(task.queued_analysis().spec, AnalysisSpec::Fft { .. }) {
            // The engine binds every `.FFT` card in a deck to that deck's
            // *first* transient, so the Studio binds the same one — not
            // Fourier's stricter "exactly one transient".
            let Some((producer_id, producer_revision, producer_config_digest)) =
                transient_producers.first()
            else {
                return Err(PreparationError::new(
                    PreparationStage::AnalysisPlan,
                    ".FFT requires a completed authored .TRAN to post-process in the same deck",
                ));
            };
            task.set_dependencies(vec![*producer_id]);
            task.set_dependency_bindings(vec![PreparedDependencyBinding::transient_trajectory(
                *producer_id,
                *producer_revision,
                *producer_config_digest,
            )]);
            continue;
        }
        type ProducerIdentity = (
            rspice_app_types::product::AnalysisInstanceId,
            rspice_app_types::product::ObjectRevision,
            rspice_app_types::product::ContentDigest,
        );
        type BindingConstructor = fn(
            rspice_app_types::product::AnalysisInstanceId,
            rspice_app_types::product::ObjectRevision,
            rspice_app_types::product::ContentDigest,
        ) -> PreparedDependencyBinding;
        // The artifact kinds this request admits, in the order it prefers
        // them, and the producer list each one names. A periodic
        // small-signal request whose carrier is the preceding periodic
        // solve admits either family, so the deck decides: a deck holding
        // only an `.HB` binds the harmonic-balance state, and one holding
        // a `.PSS` binds the shooting state.
        let required_kinds = crate::prepared_dependency::required_artifact_kinds(
            &task.queued_analysis().spec,
            &task.queued_analysis().spec_options,
        );
        if required_kinds.is_empty() {
            continue;
        }
        let candidates = required_kinds
            .iter()
            .map(|kind| {
                let (producers, binding): (&[ProducerIdentity], BindingConstructor) = match kind {
                    ExecutionArtifactKind::TransientTrajectory => (
                        &transient_producers,
                        PreparedDependencyBinding::transient_trajectory,
                    ),
                    ExecutionArtifactKind::PeriodicState => (
                        &periodic_producers,
                        PreparedDependencyBinding::periodic_state,
                    ),
                    ExecutionArtifactKind::QpssState => {
                        (&qpss_producers, PreparedDependencyBinding::qpss_state)
                    }
                    ExecutionArtifactKind::HbState => (
                        &harmonic_balance_producers,
                        PreparedDependencyBinding::hb_state,
                    ),
                    ExecutionArtifactKind::DcOperatingPointSeed => (
                        &operating_point_producers,
                        PreparedDependencyBinding::dc_operating_point_seed,
                    ),
                };
                (*kind, producers, binding)
            })
            .collect::<Vec<_>>();
        let Some((_, producers, binding)) = candidates
            .iter()
            .copied()
            .find(|(_, producers, _)| producers.len() == 1)
        else {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                format!(
                    "Manual-deck {} requires exactly one prepared {} producer; found {}",
                    task.queued_analysis().spec.run_type().display_name(),
                    required_kinds
                        .iter()
                        .map(|kind| kind.producer_label())
                        .collect::<Vec<_>>()
                        .join(" or "),
                    candidates
                        .iter()
                        .map(|(_, producers, _)| producers.len())
                        .max()
                        .unwrap_or_default()
                ),
            ));
        };
        let [(producer_id, producer_revision, producer_config_digest)] = producers else {
            unreachable!("the selected producer list holds exactly one identity");
        };
        task.set_dependencies(vec![*producer_id]);
        task.set_dependency_bindings(vec![binding(
            *producer_id,
            *producer_revision,
            *producer_config_digest,
        )]);
    }

    // Analysis directives are declarative, so source order cannot make a
    // .FOUR consumer precede its .TRAN producer. Preserve authored order
    // among every currently-ready task while applying the exact graph.
    let mut ordered = Vec::with_capacity(prepared.len());
    let mut completed = HashSet::with_capacity(prepared.len());
    while !prepared.is_empty() {
        let Some(ready_index) = prepared.iter().position(|task| {
            task.dependencies()
                .iter()
                .all(|dependency| completed.contains(dependency))
        }) else {
            return Err(PreparationError::new(
                PreparationStage::AnalysisPlan,
                "Manual-deck analysis dependencies contain a cycle",
            ));
        };
        let task = prepared.remove(ready_index);
        completed.insert(task.instance_id());
        ordered.push(task);
    }
    Ok(ordered)
}

pub(crate) fn attach_saved_output_contracts(
    tasks: Vec<PreparedTask>,
    outputs: &[rspice_simulation_contract::saved_output::SavedOutput],
) -> Result<Vec<PreparedTask>, PreparationError> {
    if outputs.is_empty() {
        return Ok(tasks);
    }
    let analyses = tasks
        .iter()
        .map(|task| (task.instance_id(), &task.queued_analysis().spec))
        .collect::<Vec<_>>();
    let mut by_analysis = HashMap::with_capacity(tasks.len());
    for output in outputs {
        let contracts = crate::output_contract::compile_saved_output_contracts(
            output,
            analyses.iter().copied(),
        )
        .map_err(|error| PreparationError::new(PreparationStage::AnalysisPlan, error))?;
        for contract in contracts {
            by_analysis
                .entry(contract.analysis_id())
                .or_insert_with(Vec::new)
                .push(contract);
        }
    }
    Ok(tasks
        .into_iter()
        .map(|task| {
            let contracts = by_analysis.remove(&task.instance_id()).unwrap_or_default();
            task.with_saved_output_contracts(contracts)
        })
        .collect())
}
