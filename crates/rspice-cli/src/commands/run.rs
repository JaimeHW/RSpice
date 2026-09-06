//! The `run` command: execute one deck and publish what it produced.
//!
//! This file is the entry point and the whole-run orchestration -- argument
//! handling, the multi-run `.ALTER`/`.DATA`/corner plan, the report files, and
//! the exit status. The work it orchestrates lives beside it:
//!
//! - [`deck`] turns arguments and a source file into something runnable, and
//!   refuses what this frontend has no route for, before any solver work;
//! - [`context`] is the per-run state a card's runner reads, and the
//!   dispatcher that turns each authored card into a call;
//! - [`naming`] decides where every artifact goes and which canonical identity
//!   it publishes under;
//! - [`axis`] executes a `.STEP`/`.TEMP`/`.DATA` coordinate set as one
//!   transaction;
//! - [`basic`], [`frequency`], [`advanced`] and [`periodic`] run the analysis
//!   families, and [`document`], [`fft_document`], [`fourier_document`] and
//!   [`restart`] publish what they produce.

#![allow(clippy::too_many_arguments)]

mod advanced;
mod axis;
mod basic;
mod context;
mod deck;
mod document;
mod fft_document;
mod fourier_document;
mod frequency;
mod naming;
mod periodic;
mod restart;
mod shared;

pub(crate) use crate::commands::export_table as export;
pub(crate) use document::PublishedResult;
pub(crate) use fft_document::read_fft_raw_artifact;
pub(crate) use naming::canonical_analysis_identities;

use axis::{
    canonical_coordinate_description, map_deck_plan_error, map_materialized_run_error, run_deck,
};
use context::{
    ArtifactCoordinate, ElaboratedDeckPaths, PlannedAnalysisIdentities, RunContext,
    RunContextSettings, RunIdentity, run_requested_mode,
};
use deck::{
    analyses_in_execution_order, build_sim_config, load_netlist_from_source,
    materialize_addresistors_artifact, parse_format_name, parse_options_for_run,
    preflight_deck_run_count, step_analysis_signature, validate_pss_flag_conflict,
    validate_run_numeric_args, validate_step_frontend_compatibility,
};
use naming::{
    axis_set_manifest_path, compose_run_label, conditional_step_schema_path, resolve_output_path,
    sanitize_run_tag, sibling_output_path, tag_output_path, xyce_addresistors_artifact_path,
};

use crate::report::{
    CsvMeasReporter, JUnitReporter, JsonMeasReporter, MeasurementReport, SimulationReport,
    TapReporter,
};

use crate::cli::{
    CliError, Config, MeasFormat, OutputFormat, PzTransferMode, RunArgs, map_atomic_output_error,
};
use crate::commands::publish;
use rspice_core::execution::{
    AnalysisInstanceId, AxisAssignment, AxisKind, DeckPlan, DeckPlanError, DeckPlanMaterializer,
    MaterializedAnalysis, MaterializedRunError, PlannedPostProcess, PostProcessSource,
    RunAxisValue, RunCoordinate, StepAxisTarget,
};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergencePreset, Engine, Netlist, SimulationConfig, SimulationConfigOverrides,
    resolve_simulation_config,
};
use std::path::PathBuf;
use std::time::Instant;

fn cancellation_cli_error(timeout_seconds: Option<f64>) -> CliError {
    match crate::abort::reason() {
        Some(crate::abort::AbortReason::Interrupt) => CliError::Interrupted,
        Some(crate::abort::AbortReason::Timeout) => CliError::TimedOut {
            seconds: timeout_seconds.unwrap_or(0.0),
        },
        None => CliError::InternalError {
            message: "operation was cancelled without a recorded process abort reason".to_string(),
        },
    }
}

fn map_cancellable_parse_error(
    error: rspice_core::netlist::ParseWithAbortError,
    timeout_seconds: Option<f64>,
) -> CliError {
    match error {
        rspice_core::netlist::ParseWithAbortError::Aborted => {
            cancellation_cli_error(timeout_seconds)
        }
        rspice_core::netlist::ParseWithAbortError::Parse(error) => {
            crate::commands::map_parse_error(error)
        }
    }
}

fn map_multi_run_error(
    error: rspice_core::netlist::multi_run::MultiRunError,
    timeout_seconds: Option<f64>,
) -> CliError {
    if error.is_aborted() {
        return cancellation_cli_error(timeout_seconds);
    }
    let suggestion = error.resource_limit_error().map_or_else(
        || Some("fix the .DATA table or its DATA=<name> reference".to_string()),
        |limit| {
            Some(format!(
                "reduce the workload or raise resources.max_{} above {}",
                limit.resource.as_str(),
                limit.requested
            ))
        },
    );
    CliError::ParseError {
        message: error.to_string(),
        line: None,
        suggestion,
    }
}

struct DeckOutcome {
    reports: Vec<SimulationReport>,
    outputs: Vec<PathBuf>,
}

pub fn execute(args: RunArgs, config: &Config, verbose: bool, quiet: bool) -> Result<(), CliError> {
    let from_stdin = crate::commands::is_stdin(&args.input);
    if !from_stdin && !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    crate::abort::install_interrupt_handler();
    validate_run_numeric_args(&args)?;
    // Held for the whole cancellable region. Dropping it on any exit path
    // closes the completion latch, so a deadline that expires after the run
    // is already over cannot announce a cancellation that never happened.
    let _timeout = args.timeout.map(crate::abort::arm_timeout);

    // A run that was killed (rather than cancelled) cannot clean up after
    // itself, so its staging files stay in the output directory. Reclaim the
    // ones whose writer is gone before this run starts staging its own.
    if let Some(output) = resolve_output_path(args.output.clone(), config)? {
        let directory = output
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
            .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf);
        publish::recover_stale_artifacts(&directory, quiet);
    }

    let resource_limits = config.resources.limits();
    let parse_options = parse_options_for_run(&args, resource_limits);
    log::info!("Loading netlist: {}", args.input.display());
    let source = if from_stdin {
        crate::commands::read_stdin_source_with_limits_and_abort(
            resource_limits,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?
    } else {
        Netlist::read_source_with_options_and_abort(
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?
    };

    // HSPICE `.ALTER` / `.DATA` constructs expand into several concrete
    // runs; a plain deck passes through as a single unlabeled run.
    let plan = rspice_core::netlist::multi_run::try_expand_multi_run_with_limits_and_abort(
        &source,
        resource_limits,
        &crate::abort::ProcessAbort,
    )
    .map_err(|error| map_multi_run_error(error, args.timeout))?;
    let multi_run = plan.len() > 1;
    if multi_run {
        // One Xyce-compatible sibling name cannot safely represent several
        // rewritten .ALTER/.DATA decks. Preflight every outer variant and its
        // complete Cartesian child-run count before workers start, so a late
        // materialization error or aggregate-budget failure cannot leave an
        // earlier variant's artifact behind.
        let mut concrete_runs = 0usize;
        for deck in &plan {
            let netlist = load_netlist_from_source(&deck.source, &args, config, false)?;
            let deck_runs = preflight_deck_run_count(&netlist, &args, config)?;
            concrete_runs =
                concrete_runs
                    .checked_add(deck_runs)
                    .ok_or_else(|| CliError::ResourceLimit {
                        path: args.input.clone(),
                        source: rspice_core::ResourceLimitError {
                            resource: rspice_core::ResourceKind::BatchRuns,
                            requested: usize::MAX,
                            limit: resource_limits.max_batch_runs,
                        },
                    })?;
            if netlist
                .options
                .add_resistors
                .as_ref()
                .is_some_and(|policy| !policy.is_empty())
            {
                return Err(CliError::InvalidArgument {
                    message: ".PREPROCESS ADDRESISTORS is ambiguous in a multi-run .ALTER/.DATA deck"
                        .to_string(),
                    suggestion: Some(
                        "run each expanded deck separately so each has its own <input>_xyce.cir artifact"
                            .to_string(),
                    ),
                });
            }
        }
        if concrete_runs > resource_limits.max_batch_runs {
            return Err(CliError::ResourceLimit {
                path: args.input.clone(),
                source: rspice_core::ResourceLimitError {
                    resource: rspice_core::ResourceKind::BatchRuns,
                    requested: concrete_runs,
                    limit: resource_limits.max_batch_runs,
                },
            });
        }
    }
    if multi_run && !quiet {
        println!(
            "Multi-run deck: {} runs (.alter/.data expansion)",
            plan.len()
        );
    }

    let start_time = Instant::now();
    let mut reports = Vec::with_capacity(plan.len());
    let mut outputs: Vec<PathBuf> = Vec::new();
    // Both the message and the typed details a failing report published, so
    // the plan-level exit status keeps the category a single-deck run would
    // have produced.
    let mut first_error: Option<(String, Option<crate::cli::ErrorDetails>)> = None;

    let workers = effective_jobs(args.jobs, plan.len(), config.resources.max_parallel_workers)?;
    if workers > 1 {
        // Parallel multi-run execution: every run is independent (own
        // parse, own engine, tagged output files). Per-run console
        // output is silenced — interleaved analysis chatter from N
        // workers is noise — and replaced by ordered status lines.
        if !quiet {
            println!("Running {} runs on {workers} workers", plan.len());
        }
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(workers)
            .build()
            .map_err(|e| CliError::InternalError {
                message: format!("failed to build the multi-run worker pool: {e}"),
            })?;
        let outcomes: Vec<Result<DeckOutcome, CliError>> = pool.install(|| {
            use rayon::prelude::*;
            plan.par_iter()
                .map(|deck| {
                    let netlist = load_netlist_from_source(&deck.source, &args, config, false)?;
                    run_deck(&netlist, &args, config, false, true, deck.label.as_deref())
                })
                .collect()
        });
        for (deck, outcome) in plan.iter().zip(outcomes) {
            let label = deck.label.as_deref().unwrap_or("base");
            let outcome = outcome?;
            if !quiet {
                if outcome.reports.iter().all(|report| report.passed) {
                    let duration: f64 = outcome
                        .reports
                        .iter()
                        .map(|report| report.duration_secs)
                        .sum();
                    println!("  ✓ {label} ({duration:.3}s)");
                } else {
                    let failure = outcome
                        .reports
                        .iter()
                        .find(|report| !report.passed)
                        .ok_or_else(|| CliError::InternalError {
                            message: format!(
                                "multi-run aggregate for '{label}' reported failure without a failed child report"
                            ),
                        })?;
                    println!("  ✗ {label}: {}", status_failure_summary(failure));
                }
            }
            if first_error.is_none() {
                first_error = first_reported_failure(&outcome.reports);
            }
            reports.extend(outcome.reports);
            outputs.extend(outcome.outputs);
        }
    } else {
        for deck in &plan {
            if multi_run && !quiet {
                println!("\n=== run: {} ===", deck.label.as_deref().unwrap_or("base"));
            }
            let netlist = load_netlist_from_source(&deck.source, &args, config, !quiet)?;
            validate_pss_flag_conflict(&netlist, &args)?;
            validate_step_frontend_compatibility(&netlist, &args)?;
            let addresistors_artifact =
                materialize_addresistors_artifact(&netlist, &args.input, from_stdin, args.timeout)?;
            let outcome = run_deck(
                &netlist,
                &args,
                config,
                verbose,
                quiet,
                deck.label.as_deref(),
            )?;
            if first_error.is_none() {
                first_error = first_reported_failure(&outcome.reports);
            }
            reports.extend(outcome.reports);
            outputs.extend(outcome.outputs);
            if let Some(path) = addresistors_artifact {
                outputs.push(path);
            }
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    let abort_reason = crate::abort::reason();
    if let Some(reason) = abort_reason {
        ensure_cancellation_report(&mut reports, &args.input, args.timeout, reason);
    }
    write_report_files(&reports, &args, verbose)?;

    let failed_measurements: Vec<&str> = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    let passed = first_error.is_none()
        && abort_reason.is_none()
        && (failed_measurements.is_empty() || args.allow_failed_meas);

    if let Some(ref summary_path) = args.summary {
        outputs.dedup();
        write_run_summary(
            summary_path,
            &args,
            &reports,
            &outputs,
            duration,
            passed,
            abort_reason,
            resource_limits,
            workers,
        )?;
    }

    // An abort outranks the per-analysis errors it caused: report files are
    // already on disk, but the exit status says interrupted/timed out.
    match abort_reason {
        Some(crate::abort::AbortReason::Interrupt) => return Err(CliError::Interrupted),
        Some(crate::abort::AbortReason::Timeout) => {
            return Err(CliError::TimedOut {
                seconds: args.timeout.unwrap_or(0.0),
            });
        }
        None => {}
    }

    if !quiet {
        println!("\nSimulation complete in {:.3}s.", duration);
    }

    if let Some((message, details)) = first_error {
        return Err(CliError::reported(message, details));
    }

    // The simulation itself succeeded; failed .MEAS checks still fail the
    // process so automation can trust the exit status.
    if !failed_measurements.is_empty() && !args.allow_failed_meas {
        return Err(CliError::VerificationFailed {
            message: format!(
                "{} measurement(s) failed: {}",
                failed_measurements.len(),
                failed_measurements.join(", ")
            ),
        });
    }

    Ok(())
}

/// Ensure every machine-readable report family carries the same cancellation
/// verdict as the process exit and JSON summary. Completed coordinate reports
/// remain available, while a distinct failed record prevents partial work
/// from being misclassified as a passing CI run.
fn ensure_cancellation_report(
    reports: &mut Vec<SimulationReport>,
    input: &std::path::Path,
    timeout_seconds: Option<f64>,
    reason: crate::abort::AbortReason,
) {
    let error = match reason {
        crate::abort::AbortReason::Interrupt => CliError::Interrupted,
        crate::abort::AbortReason::Timeout => CliError::TimedOut {
            seconds: timeout_seconds.unwrap_or(0.0),
        },
    };
    let error_message = error.to_string();
    let run_status_measurement = || MeasurementReport {
        name: "__rspice_run_status__".to_string(),
        value: None,
        raw_value: None,
        expected: None,
        tolerance: None,
        failure_limit: None,
        failure_limit_exceeded: false,
        passed: false,
        error: Some(error_message.clone()),
        record_index: None,
        event_axis: None,
        trigger_axis: None,
        target_axis: None,
        aggregate_policy: None,
    };
    if let Some(report) = reports.iter_mut().find(|report| {
        report
            .error_details
            .as_ref()
            .is_some_and(|details| details.category == "cancellation")
    }) {
        report.passed = false;
        if !report
            .measurements
            .iter()
            .any(|measurement| measurement.name == "__rspice_run_status__")
        {
            report.measurements.push(run_status_measurement());
        }
        return;
    }

    let base_name = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin");
    let label = match reason {
        crate::abort::AbortReason::Interrupt => "interrupted",
        crate::abort::AbortReason::Timeout => "timed-out",
    };
    reports.push(SimulationReport {
        name: format!("{base_name} [{label}]"),
        netlist: input.display().to_string(),
        passed: false,
        duration_secs: 0.0,
        error: Some(error_message.clone()),
        error_details: Some(error.details()),
        // Measurement-only JSON/CSV artifacts do not serialize report-level
        // failures. A reserved run-status record keeps cancellation visible
        // and failed in those formats as well as JUnit/TAP and summaries.
        measurements: vec![run_status_measurement()],
    });
}

/// Write the one-artifact JSON contract for automation: tool identity,
/// per-run status with every measurement, and the overall verdict that the
/// exit code will reflect. `-` writes to stdout.
fn write_run_summary(
    path: &std::path::Path,
    args: &RunArgs,
    reports: &[SimulationReport],
    outputs: &[PathBuf],
    duration: f64,
    passed: bool,
    abort_reason: Option<crate::abort::AbortReason>,
    resource_limits: rspice_core::ResourceLimits,
    workers: usize,
) -> Result<(), CliError> {
    let status = match abort_reason {
        Some(crate::abort::AbortReason::Interrupt) => "interrupted",
        Some(crate::abort::AbortReason::Timeout) => "timed_out",
        None if passed => "passed",
        None => "failed",
    };
    let passed_runs = reports.iter().filter(|report| report.passed).count();
    let measurement_count = reports
        .iter()
        .map(|report| report.measurements.len())
        .sum::<usize>();
    let passed_measurements = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|measurement| measurement.passed)
        .count();
    let json = serde_json::json!({
        "schema_version": 1,
        "tool": {
            "name": "rspice",
            "version": env!("CARGO_PKG_VERSION"),
            "target": env!("RSPICE_BUILD_TARGET"),
            "profile": env!("RSPICE_BUILD_PROFILE"),
            "commit": env!("RSPICE_BUILD_COMMIT"),
        },
        "run_id": crate::observability::run_id(),
        "netlist": args.input.display().to_string(),
        "status": status,
        "duration_secs": duration,
        "passed": passed,
        "execution": {
            "requested_jobs": args.jobs,
            "workers": workers,
            "parallel": workers > 1,
        },
        "counts": {
            "runs": reports.len(),
            "passed_runs": passed_runs,
            "failed_runs": reports.len().saturating_sub(passed_runs),
            "measurements": measurement_count,
            "passed_measurements": passed_measurements,
            "failed_measurements": measurement_count.saturating_sub(passed_measurements),
            "outputs": outputs.len(),
        },
        "resource_limits": resource_limits_summary(resource_limits),
        "outputs": outputs.iter().map(|p| p.display().to_string()).collect::<Vec<_>>(),
        "aborted": abort_reason.map(|reason| match reason {
            crate::abort::AbortReason::Interrupt => "interrupt",
            crate::abort::AbortReason::Timeout => "timeout",
        }),
        "runs": reports.iter().map(|report| {
            serde_json::json!({
                "name": report.name,
                "passed": report.passed,
                "error": report.error,
                "error_details": report.error_details,
                "duration_secs": report.duration_secs,
                "measurements": report.measurements.iter().map(|meas| {
                    serde_json::json!({
                        "name": meas.name,
                        "value": meas.value,
                        "expected": meas.expected,
                        "tolerance": meas.tolerance,
                        "passed": meas.passed,
                        "error": meas.error,
                    })
                }).collect::<Vec<_>>(),
            })
        }).collect::<Vec<_>>(),
    });

    if path.as_os_str() == "-" {
        match serde_json::to_string_pretty(&json) {
            Ok(text) => println!("{text}"),
            Err(e) => eprintln!("Error: failed to serialize run summary: {e}"),
        }
        return Ok(());
    }

    let text =
        serde_json::to_string_pretty(&json).map_err(|e| CliError::output_json_error(path, e))?;
    let document = text + "\n";
    publish::artifact(path, |writer| {
        writer
            .write_all(document.as_bytes())
            .map_err(|error| CliError::output_error(path, error))
    })
    .map_err(|error| map_atomic_output_error(path, error))?;
    Ok(())
}

fn resource_limits_summary(limits: rspice_core::ResourceLimits) -> serde_json::Value {
    serde_json::json!({
        "max_netlist_bytes": limits.max_netlist_bytes,
        "max_netlist_lines": limits.max_netlist_lines,
        "max_expanded_source_bytes": limits.max_expanded_source_bytes,
        "max_dependency_source_bytes": limits.max_dependency_source_bytes,
        "max_external_data_bytes": limits.max_external_data_bytes,
        "max_external_data_values": limits.max_external_data_values,
        "max_shared_cache_bytes": limits.max_shared_cache_bytes,
        "max_include_depth": limits.max_include_depth,
        "max_hierarchy_depth": limits.max_hierarchy_depth,
        "max_flattened_elements": limits.max_flattened_elements,
        "max_circuit_nodes": limits.max_circuit_nodes,
        "max_matrix_unknowns": limits.max_matrix_unknowns,
        "max_analysis_points": limits.max_analysis_points,
        "max_result_values": limits.max_result_values,
        "max_parallel_workers": limits.max_parallel_workers,
        "max_batch_runs": limits.max_batch_runs,
    })
}

fn requested_mode_name(args: &RunArgs) -> Option<&'static str> {
    if args.monte_carlo.is_some() {
        Some("--monte-carlo")
    } else if args.pss_freq.is_some() {
        Some("--pss-freq")
    } else if args.hb_freq.is_some() {
        Some("--hb-freq")
    } else if args.pz_input.is_some() || args.pz_output.is_some() {
        Some("--pz-input/--pz-output")
    } else if args.sens_output.is_some() || args.sens_param.is_some() {
        Some("--sens-output/--sens-param")
    } else if args.sparam.is_some() {
        Some("--sparam")
    } else if args.corners.is_some() {
        Some("--corners")
    } else {
        None
    }
}

/// What one concrete deck run produced: its report, the artifacts it staged,
/// and the typed contract each of those artifacts published under.
struct ConcreteDeckOutcome {
    report: SimulationReport,
    outputs: Vec<PathBuf>,
    published: Vec<PublishedResult>,
}

/// Run one concrete deck (all of its analyses) and assemble its report.
/// Multi-run failures don't abort the remaining runs — HSPICE semantics —
/// so errors land in the report instead of bubbling, except for setup
/// errors (bad output paths, alternate-mode failures).
fn run_concrete_deck(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
    identity: RunIdentity<'_>,
) -> Result<ConcreteDeckOutcome, CliError> {
    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let ctx = RunContext::new(
        &engine, netlist, args, config, verbose, quiet, run_label, identity,
    )?;

    let base_name = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .filter(|stem| *stem != "-")
        .unwrap_or("stdin")
        .to_string();
    let name = match run_label {
        Some(label) => format!("{base_name} [{label}]"),
        None => base_name,
    };

    let start_time = Instant::now();
    let requested_mode = run_requested_mode(&ctx, config)?;
    if requested_mode.ran() {
        if requested_mode.needs_measurement_finalization() {
            ctx.record_unevaluated_measurements();
        }
        // A command-line analysis mode deliberately supersedes the deck's
        // authored cards, so their planned identities stay unconsumed. Only
        // the deferred namespace failure is still a defect here.
        if let Some(message) = ctx.planned_namespace_error.borrow_mut().take() {
            return Err(CliError::InternalError { message });
        }
        let measurements = ctx.measurements.borrow().clone();
        let passed = measurements.iter().all(|meas| meas.passed);
        return Ok(ConcreteDeckOutcome {
            report: SimulationReport {
                name,
                netlist: args.input.display().to_string(),
                passed,
                duration_secs: start_time.elapsed().as_secs_f64(),
                error: None,
                error_details: None,
                measurements,
            },
            published: ctx.published.into_inner(),
            outputs: ctx.outputs.into_inner(),
        });
    }

    let mut ran_analysis = false;
    let mut simulation_error: Option<String> = None;
    let mut simulation_error_details: Option<crate::cli::ErrorDetails> = None;

    for (idx, analysis) in analyses_in_execution_order(netlist).enumerate() {
        if verbose {
            println!(
                "\nRunning analysis {}/{}: {:?}",
                idx + 1,
                netlist.analyses.len(),
                analysis
            );
        }

        ran_analysis = true;
        if let Err(e) = ctx.run_analysis(analysis) {
            if is_run_setup_or_output_error(&e) {
                return Err(e);
            }
            simulation_error_details = Some(e.details());
            simulation_error = Some(simulation_error_message(&e));
            break;
        }
    }

    if !ran_analysis && simulation_error.is_none() {
        if !quiet {
            println!("No analysis commands - running default DC OP...");
        }
        if let Err(e) = basic::run_dc_op(&ctx) {
            if is_run_setup_or_output_error(&e) {
                return Err(e);
            }
            simulation_error_details = Some(e.details());
            simulation_error = Some(simulation_error_message(&e));
        }
    }

    if args.meas && !quiet && netlist.measurements.is_empty() {
        println!("  No .MEAS statements found in netlist");
    }
    ctx.record_unevaluated_measurements();
    if simulation_error.is_none() {
        ctx.ensure_planned_namespaces_consumed()?;
    }

    let duration = start_time.elapsed().as_secs_f64();
    let measurements = ctx.measurements.borrow().clone();
    let passed = simulation_error.is_none() && measurements.iter().all(|meas| meas.passed);

    Ok(ConcreteDeckOutcome {
        report: SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed,
            duration_secs: duration,
            error: simulation_error,
            error_details: simulation_error_details,
            measurements,
        },
        published: ctx.published.into_inner(),
        outputs: ctx.outputs.into_inner(),
    })
}

/// Failure text for the run report: simulation errors carry their bare
/// message (re-wrapping at exit would otherwise print
/// "Simulation failed: Simulation failed: ...").
fn simulation_error_message(e: &CliError) -> String {
    match e {
        CliError::SimulationError { message, .. } => message.clone(),
        CliError::CoreSimulationError { source, .. } => source.to_string(),
        other => other.to_string(),
    }
}

/// The first recorded failure in a plan's reports, with the typed details it
/// published.
///
/// The message alone used to be all that survived, which turned every deck
/// failure — a capability refusal, a convergence failure, an exceeded budget —
/// into one undifferentiated simulation error at the process boundary.
fn first_reported_failure(
    reports: &[SimulationReport],
) -> Option<(String, Option<crate::cli::ErrorDetails>)> {
    reports.iter().find_map(|report| {
        report
            .error
            .clone()
            .map(|message| (message, report.error_details.clone()))
    })
}

fn is_run_setup_or_output_error(error: &CliError) -> bool {
    matches!(
        error,
        CliError::InvalidArgument { .. }
            | CliError::OutputError { .. }
            | CliError::OutputSerializationError { .. }
    ) || matches!(
        error.category(),
        // A failed publication is an output failure however it was typed:
        // demoting it into a run report would report a successful simulation
        // whose results never reached the disk.
        crate::cli::FailureCategory::Engine(rspice_core::SimulationErrorCategory::OutputCommit)
    )
}

fn status_failure_summary(report: &SimulationReport) -> String {
    if let Some(error) = &report.error {
        return error.clone();
    }

    let failed_measurements: Vec<&str> = report
        .measurements
        .iter()
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    if failed_measurements.is_empty() {
        "run failed".to_string()
    } else {
        format!(
            "{} measurement(s) failed: {}",
            failed_measurements.len(),
            failed_measurements.join(", ")
        )
    }
}

/// Write the CI/CD report artifacts covering every run.
fn write_report_files(
    reports: &[SimulationReport],
    args: &RunArgs,
    verbose: bool,
) -> Result<(), CliError> {
    if let Some(ref report_file) = args.report_file {
        match args.report_format {
            Some(crate::cli::ReportFormat::Junit) | None => {
                JUnitReporter::write(reports, report_file)?;
                if verbose {
                    println!("JUnit report written to: {}", report_file.display());
                }
            }
            Some(crate::cli::ReportFormat::Tap) => {
                TapReporter::write(reports, report_file)?;
                if verbose {
                    println!("TAP report written to: {}", report_file.display());
                }
            }
        }
    }

    if let Some(ref meas_file) = args.meas_file {
        match args.meas_format {
            Some(MeasFormat::Csv) => CsvMeasReporter::write(reports, meas_file)?,
            Some(MeasFormat::Json) | None => JsonMeasReporter::write(reports, meas_file)?,
        }
        if verbose {
            println!("Measurement report written to: {}", meas_file.display());
        }
    }
    Ok(())
}

/// Worker count for a multi-run plan: `--jobs 0` = all available cores up to
/// the configured worker budget, never more workers than runs, and single-run
/// plans stay serial. Explicit requests that would exceed the budget fail
/// instead of silently changing operator intent.
fn effective_jobs(
    requested: usize,
    runs: usize,
    max_parallel_workers: usize,
) -> Result<usize, CliError> {
    if max_parallel_workers == 0 {
        return Err(rspice_core::SimulationConfigError::ResourceLimit(
            rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ParallelWorkers,
                requested: 1,
                limit: 0,
            },
        )
        .into());
    }
    if runs <= 1 {
        return Ok(1);
    }
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    if requested == 0 {
        return Ok(cores.min(runs).min(max_parallel_workers).max(1));
    }

    let workers = requested.min(runs).max(1);
    if workers > max_parallel_workers {
        return Err(rspice_core::SimulationConfigError::ResourceLimit(
            rspice_core::ResourceLimitError {
                resource: rspice_core::ResourceKind::ParallelWorkers,
                requested: workers,
                limit: max_parallel_workers,
            },
        )
        .into());
    }
    Ok(workers)
}

#[cfg(test)]
mod step_cancellation_report_tests {
    use super::*;

    fn passing_report() -> SimulationReport {
        SimulationReport {
            name: "deck [step-000001]".to_string(),
            netlist: "deck.cir".to_string(),
            passed: true,
            duration_secs: 0.1,
            error: None,
            error_details: None,
            measurements: Vec::new(),
        }
    }

    #[test]
    fn timeout_appends_one_failed_typed_ci_report_after_partial_success() {
        let mut reports = vec![passing_report()];
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            Some(2.5),
            crate::abort::AbortReason::Timeout,
        );
        assert_eq!(reports.len(), 2);
        assert!(reports[0].passed);
        assert!(!reports[1].passed);
        assert_eq!(reports[1].name, "deck [timed-out]");
        assert!(
            reports[1]
                .error
                .as_deref()
                .is_some_and(|error| error.contains("2.5s"))
        );
        assert_eq!(
            reports[1]
                .error_details
                .as_ref()
                .map(|details| (details.code, details.category)),
            Some(("timed_out", "timeout"))
        );
        assert_eq!(reports[1].measurements.len(), 1);
        assert_eq!(reports[1].measurements[0].name, "__rspice_run_status__");
        assert!(!reports[1].measurements[0].passed);
    }

    #[test]
    fn existing_typed_cancellation_report_is_not_duplicated() {
        let interrupted = CliError::Interrupted;
        let mut reports = vec![SimulationReport {
            name: "deck [step-000002]".to_string(),
            netlist: "deck.cir".to_string(),
            passed: false,
            duration_secs: 0.2,
            error: Some(interrupted.to_string()),
            error_details: Some(interrupted.details()),
            measurements: Vec::new(),
        }];
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            None,
            crate::abort::AbortReason::Interrupt,
        );
        ensure_cancellation_report(
            &mut reports,
            std::path::Path::new("deck.cir"),
            None,
            crate::abort::AbortReason::Interrupt,
        );
        assert_eq!(reports.len(), 1);
        assert_eq!(reports[0].measurements.len(), 1);
        assert_eq!(reports[0].measurements[0].name, "__rspice_run_status__");
        assert!(!reports[0].measurements[0].passed);
        assert_eq!(
            reports[0]
                .error_details
                .as_ref()
                .map(|details| (details.code, details.category)),
            Some(("interrupted", "cancellation"))
        );

        let suffix = format!("{}_{}", std::process::id(), reports.len());
        let json_path = std::env::temp_dir().join(format!("rspice_cancel_{suffix}.json"));
        let csv_path = std::env::temp_dir().join(format!("rspice_cancel_{suffix}.csv"));
        JsonMeasReporter::write(&reports, &json_path).expect("write cancellation JSON");
        CsvMeasReporter::write(&reports, &csv_path).expect("write cancellation CSV");
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&json_path).expect("read cancellation JSON"))
                .expect("parse cancellation JSON");
        assert_eq!(json["failed"], 1);
        let csv = std::fs::read_to_string(&csv_path).expect("read cancellation CSV");
        assert!(csv.contains("__rspice_run_status__"), "{csv}");
        assert!(csv.contains(",false,Simulation interrupted,"), "{csv}");
        let _ = std::fs::remove_file(json_path);
        let _ = std::fs::remove_file(csv_path);
    }
}
