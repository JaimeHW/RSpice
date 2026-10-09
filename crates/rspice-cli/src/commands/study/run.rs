use std::collections::{BTreeMap, HashMap};
use std::io::Write;
use std::path::Path;
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use rspice_core::AbortSignal;
use rspice_formats::project_results::{
    ProjectSimulationResults, ProjectSimulationResultsData, ProjectSimulationRun,
    ProjectWaveformData,
};
use rspice_results::provenance::AnalysisResultProvenance;
use rspice_results::run::{ExecutionTarget, SimulationRun, SimulationRunLifecycle};
use rspice_results::saved_output::SavedOutputMaterializationStatus;
use rspice_simulation::execution::{PreparedRunAuthorization, SavePolicy};
use rspice_simulation::output_contract::materialization::{
    apply_saved_output_policy, materialize_saved_outputs,
};
use rspice_simulation::runner::SimulationRunner;
use serde::Serialize;
use serde_json::{Value, json};

use super::{check_abort, failure, invalid, prepare};
use crate::cli::{CliError, Config, StudyRunArgs, map_atomic_output_error};

pub(super) fn execute(args: StudyRunArgs, config: &Config, quiet: bool) -> Result<(), CliError> {
    crate::abort::install_interrupt_handler();
    let timeout = args
        .study
        .timeout
        .map(crate::abort::arm_timeout)
        .transpose()?;
    let result = execute_inner(&args, config, quiet);
    drop(timeout);
    match result {
        Ok(summary) => {
            if args.study.json {
                crate::console::line(format_args!(
                    "{}",
                    crate::observability::envelope("rspice.study.run", summary)
                ))?;
            } else if !quiet {
                crate::console::line(format_args!("Study completed: {}", args.output.display()))?;
            }
            Ok(())
        }
        Err(error) => {
            let error = if crate::abort::ProcessAbort.is_aborted() {
                super::super::run::cancellation_cli_error(args.study.timeout)
            } else {
                error
            };
            if args.study.json {
                crate::console::line(format_args!(
                    "{}",
                    crate::observability::envelope(
                        "rspice.study.run",
                        json!({
                            "success": false, "output": args.output,
                            "error": {"message": error.to_string(), "details": error.details()},
                        })
                    )
                ))?;
            }
            Err(error)
        }
    }
}

fn execute_inner(args: &StudyRunArgs, config: &Config, quiet: bool) -> Result<Value, CliError> {
    let prepared = prepare(&args.study.input, config)?;
    let report = prepared.report()?;
    let metadata = prepared.snapshot.metadata();
    let receipt = prepared
        .snapshot
        .prepared_run_receipt()
        .map_err(|error| invalid(&prepared.path, error.to_string(), None))?;
    let (destinations, _scope) =
        super::super::publish::destinations::begin(&[("study results", &args.output)], &[])?;
    for source in [&prepared.path, &prepared.circuit_path] {
        destinations.protect(source)?;
    }
    for source in config.source_paths() {
        destinations.protect(source)?;
    }
    for dependency in &metadata.sealed_source_dependencies {
        destinations.protect(dependency.resolved_path())?;
        destinations.protect(dependency.owner_path())?;
    }
    check_abort(args.study.timeout)?;
    let dispatch = PreparedRunAuthorization::default()
        .authorize_campaign_member(prepared.snapshot)
        .map_err(|error| invalid(&prepared.path, error, None))?;
    let policy = dispatch.save_policy();
    let started = Instant::now();
    let mut run: SimulationRun =
        SimulationRun::new_prepared(1, now(), ExecutionTarget::current(), receipt.clone());
    run.mark_running().map_err(internal)?;
    let mut pending = dispatch.into_tasks();
    let mut artifacts = HashMap::new();
    let mut executed_netlists = BTreeMap::new();
    let mut retained_bytes = 0u64;
    let mut runner = SimulationRunner::new();
    for task_receipt in receipt.tasks() {
        check_abort(args.study.timeout)?;
        let task = pending
            .pop_front()
            .ok_or_else(|| internal("prepared task queue ended early".into()))?;
        let instance_id = task.instance_id();
        let authored = &prepared.document.tasks[prepared.names[&instance_id.to_string()]];
        let label = task.label().to_owned();
        let contracts = task.saved_output_contracts().to_vec();
        let provenance = AnalysisResultProvenance::new_with_source_domain(
            receipt.source_domain(),
            instance_id,
            task.source_revision(),
            task.snapshot_digest(),
            task.dependencies().to_vec(),
        )
        .map_err(internal)?;
        executed_netlists.insert(
            instance_id.to_string(),
            Arc::clone(task.executable_netlist()),
        );
        let resolved = task
            .resolve_dependency_artifacts(&artifacts)
            .map_err(|error| internal(error.to_string()))?;
        let producer = resolved
            .artifact_producer()
            .map_err(|error| internal(error.to_string()))?;
        emit_progress(args, "task_started", &authored.id, None);
        runner
            .start_prepared(resolved, false)
            .map_err(|error| failure::execution(error, &authored.id, args.study.timeout))?;
        let mut next_progress = Instant::now();
        let result = loop {
            if crate::abort::ProcessAbort.is_aborted() {
                runner.abort();
            }
            if let Some(result) = runner.poll_result() {
                break result.map_err(|error| {
                    failure::execution(error, &authored.id, args.study.timeout)
                })?;
            }
            if Instant::now() >= next_progress {
                emit_progress(
                    args,
                    "task_progress",
                    &authored.id,
                    runner.progress_fraction(),
                );
                next_progress = Instant::now() + Duration::from_millis(250);
            }
            // Drain the bounded runner queue even when routine logs are hidden.
            for line in runner.drain_engine_log() {
                if !quiet {
                    crate::observability::diagnostic("study.engine", None, line.message);
                }
            }
            std::thread::sleep(Duration::from_millis(5));
        };
        check_abort(args.study.timeout)?;
        if let Some(artifact) = producer
            .capture(&result, &pending)
            .map_err(|error| internal(error.to_string()))?
        {
            artifacts.insert(instance_id, artifact);
        }
        let mut retained = rspice_simulation::result_conversion::convert(
            result,
            task_receipt.result_analysis_type(),
            &label,
            now,
        );
        retained.provenance = Some(provenance);
        if !retained.success {
            return Err(failure::classified(
                retained
                    .error_message
                    .unwrap_or_else(|| "result conversion failed".into()),
                "result_schema_mismatch",
                rspice_core::SimulationErrorCategory::ResultSchema,
            ));
        }
        if matches!(policy, SavePolicy::RetainEngineProducedResults) {
            materialize_saved_outputs(&mut retained, &contracts);
        } else {
            apply_saved_output_policy(&mut retained, policy, &contracts);
        }
        for output in &retained.saved_output_receipts {
            if let SavedOutputMaterializationStatus::Unavailable { reason } = &output.status {
                return Err(failure::classified(
                    reason.clone(),
                    "requested_signal_unavailable",
                    rspice_core::SimulationErrorCategory::SignalUnavailable,
                ));
            }
        }
        retained.validate_retained_evidence().map_err(|message| {
            failure::classified(
                message,
                "result_schema_mismatch",
                rspice_core::SimulationErrorCategory::ResultSchema,
            )
        })?;
        if let Some(measurement) = retained
            .measurements
            .iter()
            .find(|measurement| !measurement.passed)
        {
            return Err(CliError::VerificationFailed {
                message: format!(
                    "task {} measurement {}: {}",
                    authored.id,
                    measurement.name,
                    measurement
                        .error
                        .as_deref()
                        .unwrap_or("verification failed")
                ),
            });
        }
        retained_bytes =
            retained_bytes.saturating_add(retained.result_data_ref().retained_data_bytes());
        if let Some(limit) = policy.maximum_storage_bytes() {
            check_storage(retained_bytes, limit)?;
        }
        run.add_analysis(retained);
        // Once all of a producer's consumers have resolved, release its native
        // solver state. The completed retained result keeps its own evidence.
        artifacts.retain(|producer, _| {
            pending
                .iter()
                .any(|consumer| consumer.dependencies().contains(producer))
        });
        emit_progress(args, "task_completed", &authored.id, Some(1.0));
    }
    run.finish_lifecycle(SimulationRunLifecycle::Completed, || Ok(started.elapsed()))
        .map_err(internal)?;
    run.validate_provenance().map_err(internal)?;
    let results = ProjectSimulationResults::from(ProjectSimulationResultsData {
        runs: vec![ProjectSimulationRun::from_run(&run, |waveform| {
            ProjectWaveformData::from_waveform(waveform, String::new(), true)
        })],
        next_run_id: 1,
        ..Default::default()
    });
    results.validate().map_err(|message| {
        failure::classified(
            message,
            "result_schema_mismatch",
            rspice_core::SimulationErrorCategory::ResultSchema,
        )
    })?;
    let artifact = StudyRunArtifact {
        schema: "rspice.study.results",
        schema_version: 1,
        run_id: crate::observability::run_id(),
        tool: super::super::health::tool_identity(),
        study: &prepared.document,
        preparation: &report,
        results: &results,
        executed_netlists: &executed_netlists,
    };
    let storage_limit = policy.maximum_storage_bytes();
    let transaction = super::super::publish::begin()?;
    super::super::publish::artifact(&args.output, |output| {
        write_artifact(
            output,
            &artifact,
            storage_limit,
            &args.output,
            args.study.timeout,
        )
    })
    .map_err(|error| map_atomic_output_error(&args.output, error))?;
    check_abort(args.study.timeout)?;
    transaction.commit()?;
    Ok(json!({
        "success": true, "study_id": prepared.document.id, "output": args.output,
        "task_count": receipt.tasks().len(), "snapshot_digest": metadata.snapshot_digest,
        "dataset_digest": run.dataset_content_digest_with_encoding(rspice_results::result_digest::ResultDigestEncoding::CURRENT),
        "elapsed_seconds": started.elapsed().as_secs_f64(),
        "qualification_blocker": receipt.sign_off_blocker(),
    }))
}

#[derive(Serialize)]
struct StudyRunArtifact<'a> {
    schema: &'static str,
    schema_version: u32,
    run_id: &'static str,
    tool: Value,
    study: &'a rspice_simulation::study_document::StudyDocument,
    preparation: &'a Value,
    results: &'a ProjectSimulationResults,
    executed_netlists: &'a BTreeMap<String, Arc<str>>,
}

fn write_artifact(
    output: &mut dyn Write,
    artifact: &StudyRunArtifact<'_>,
    limit: Option<u64>,
    path: &Path,
    timeout: Option<f64>,
) -> Result<(), CliError> {
    struct BoundedWriter<'a> {
        output: &'a mut dyn Write,
        bytes: u64,
        limit: Option<u64>,
        failure: Option<CliError>,
        timeout: Option<f64>,
    }
    impl Write for BoundedWriter<'_> {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            let requested = self.bytes.saturating_add(bytes.len() as u64);
            let admission = check_abort(self.timeout).and_then(|()| {
                self.limit
                    .map_or(Ok(()), |limit| check_storage(requested, limit))
            });
            if let Err(error) = admission {
                let message = error.to_string();
                self.failure = Some(error);
                return Err(std::io::Error::other(message));
            }
            let written = self.output.write(bytes)?;
            self.bytes += written as u64;
            Ok(written)
        }
        fn flush(&mut self) -> std::io::Result<()> {
            self.output.flush()
        }
    }
    let mut writer = BoundedWriter {
        output,
        bytes: 0,
        limit,
        failure: None,
        timeout,
    };
    let result = serde_json::to_writer(&mut writer, artifact);
    if let Some(error) = writer.failure {
        return Err(error);
    }
    result.map_err(|source| {
        if source.is_io() {
            CliError::output_error(path, std::io::Error::other(source))
        } else {
            CliError::OutputSerializationError {
                path: path.into(),
                source,
            }
        }
    })
}

fn check_storage(bytes: u64, limit: u64) -> Result<(), CliError> {
    if bytes > limit {
        Err(failure::classified(
            format!(
                "study result storage requires {bytes} bytes; save_policy.maximum_storage_bytes is {limit}"
            ),
            "study.storage_limit",
            rspice_core::SimulationErrorCategory::ResourceLimit,
        ))
    } else {
        Ok(())
    }
}

fn emit_progress(args: &StudyRunArgs, event: &str, task: &str, fraction: Option<f32>) {
    if args.progress_json {
        crate::console::diagnostic_line(format_args!(
            "{}",
            crate::observability::envelope(
                "rspice.study.progress",
                json!({
                    "event": event, "task": task, "fraction": fraction,
                })
            )
        ));
    }
}

fn now() -> f64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs_f64()
}
fn internal(message: String) -> CliError {
    CliError::InternalError { message }
}
