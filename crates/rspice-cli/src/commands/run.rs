//! Run Command - Execute SPICE simulations
//!
//! This is the main simulation execution command, supporting:
//! - All analysis types (DC, AC, Transient, Noise, etc.)
//! - Multiple output formats
//! - Progress reporting
//! - Waveform compression

#![allow(clippy::too_many_arguments)]

mod advanced;
mod basic;
mod frequency;
mod shared;

pub(crate) use crate::commands::export_table as export;

use crate::report::{
    CsvMeasReporter, JUnitReporter, JsonMeasReporter, MeasurementReport, SimulationReport,
    TapReporter,
};

use crate::cli::{CliError, Config, MeasFormat, OutputFormat, RunArgs};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergenceConfig, ConvergencePreset, Engine, Netlist, SimulationConfig,
    SimulationConfigOverrides, resolve_simulation_config,
};
use std::path::PathBuf;
use std::time::Instant;

struct RunContext<'a> {
    engine: &'a Engine,
    netlist: &'a Netlist,
    args: &'a RunArgs,
    /// Resolved output format: CLI flag, else config `output.format`, else raw.
    format: OutputFormat,
    /// Resolved output path; relative paths land in config `output.output_directory`.
    output: Option<std::path::PathBuf>,
    /// CLI `--progress` or config `output.show_progress`.
    show_progress: bool,
    /// CLI `--compress` or config `simulation.compress_waveforms`.
    compress: bool,
    /// CLI `--compress-tol`, else config `simulation.compression_tolerance`.
    compress_tol: f64,
    /// More than one analysis card runs; output files get per-analysis tags.
    multi_analysis: bool,
    verbose: bool,
    quiet: bool,
    /// .MEAS results collected while analyses run, for CI/CD reporting
    /// (`--report-file` / `--meas-file`) and the process exit status.
    measurements: std::cell::RefCell<Vec<MeasurementReport>>,
    /// Analysis tags (upper-case) whose .MEAS statements were evaluated,
    /// so leftover measurements can fail loudly instead of being skipped.
    evaluated_meas: std::cell::RefCell<std::collections::HashSet<String>>,
    /// Result files this run resolved for export, for the `--summary`
    /// manifest.
    outputs: std::cell::RefCell<Vec<std::path::PathBuf>>,
}

impl<'a> RunContext<'a> {
    fn new(
        engine: &'a Engine,
        netlist: &'a Netlist,
        args: &'a RunArgs,
        config: &Config,
        verbose: bool,
        quiet: bool,
        run_label: Option<&str>,
    ) -> Result<Self, CliError> {
        let format = match args.format {
            Some(format) => format,
            None => parse_format_name(&config.output.format)?,
        };
        let mut output = resolve_output_path(args.output.clone(), config)?;
        // Multi-run decks tag each run's output so later runs cannot
        // silently overwrite earlier ones: `out.csv` -> `out.hot.csv`.
        if let (Some(label), Some(path)) = (run_label, output.as_mut()) {
            *path = tag_output_path(path, &sanitize_run_tag(label));
        }

        Ok(Self {
            engine,
            netlist,
            args,
            format,
            output,
            show_progress: args.progress || config.output.show_progress,
            compress: args.compress || config.simulation.compress_waveforms,
            compress_tol: args
                .compress_tol
                .unwrap_or(config.simulation.compression_tolerance),
            multi_analysis: netlist.analyses.len() > 1,
            verbose,
            quiet,
            measurements: std::cell::RefCell::new(Vec::new()),
            evaluated_meas: std::cell::RefCell::new(std::collections::HashSet::new()),
            outputs: std::cell::RefCell::new(Vec::new()),
        })
    }

    /// Record evaluated .MEAS results: print them under `--meas` and keep
    /// them for report files and the exit status.
    fn record_measurements(&self, analysis: &str, results: Vec<rspice_core::MeasureResult>) {
        if results.is_empty() {
            return;
        }
        self.evaluated_meas
            .borrow_mut()
            .insert(analysis.to_ascii_uppercase());

        if self.args.meas && !self.quiet {
            println!("  Measurement Results ({}, {}):", analysis, results.len());
            for mr in &results {
                match (mr.value, mr.passed) {
                    (Some(value), true) => println!(
                        "    {} = {}",
                        mr.name,
                        crate::report::format_spice_exponent(value)
                    ),
                    // Evaluated, but missed a declared GOAL.
                    (Some(value), false) => println!(
                        "    {} = {} FAILED ({})",
                        mr.name,
                        crate::report::format_spice_exponent(value),
                        mr.error.as_deref().unwrap_or("missed goal")
                    ),
                    (None, _) => println!(
                        "    {} = FAILED ({})",
                        mr.name,
                        mr.error.as_deref().unwrap_or("not evaluated")
                    ),
                }
            }
        }

        self.measurements
            .borrow_mut()
            .extend(results.into_iter().map(|mr| MeasurementReport {
                name: mr.name,
                value: mr.value,
                expected: mr.expected,
                tolerance: mr.tolerance,
                passed: mr.passed,
                error: mr.error,
            }));
    }

    /// Convert .MEAS statements whose analysis never evaluated them into
    /// explicit failures, so automation cannot mistake a skipped check for
    /// a passing one.
    fn record_unevaluated_measurements(&self) {
        let mut analyses: Vec<String> = self
            .netlist
            .measurements
            .iter()
            .map(|m| m.analysis.to_ascii_uppercase())
            .collect();
        analyses.sort();
        analyses.dedup();
        analyses.retain(|analysis| !self.evaluated_meas.borrow().contains(analysis));

        for analysis in analyses {
            let reason = match analysis.as_str() {
                "TRAN" | "DC" | "AC" | "NOISE" => {
                    format!("{analysis} analysis did not run")
                }
                other => format!("unknown .MEAS analysis kind '{other}'"),
            };
            let results = rspice_core::analysis::advanced::unevaluated_measurements(
                self.netlist,
                &analysis,
                &reason,
            );
            self.record_measurements(&analysis, results);
        }
    }

    /// Output path for one analysis.
    ///
    /// When the deck runs several analyses, each gets its own file so later
    /// analyses cannot silently overwrite earlier results:
    /// `out.csv` becomes `out.op.csv`, `out.tran.csv`, ...
    ///
    /// Every resolved path is remembered for the `--summary` manifest.
    fn output_path_for(&self, tag: &str) -> Option<std::path::PathBuf> {
        let path = self.output.clone()?;
        let resolved = if !self.multi_analysis {
            path
        } else {
            let mut file_name = path
                .file_stem()
                .map(|stem| stem.to_os_string())
                .unwrap_or_default();
            file_name.push(format!(".{tag}"));
            if let Some(ext) = path.extension() {
                file_name.push(".");
                file_name.push(ext);
            }
            path.with_file_name(file_name)
        };
        self.outputs.borrow_mut().push(resolved.clone());
        Some(resolved)
    }

    fn run_analysis(&self, analysis: &AnalysisCommand) -> Result<(), CliError> {
        match analysis {
            AnalysisCommand::Op => basic::run_dc_op(self)?,
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
                sweep2,
                mode: _,
            } => basic::run_dc_sweep(self, source, *start, *stop, *step, sweep2.as_ref())?,
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
                // The engine reads UIC from the netlist's own .TRAN card.
                uic: _,
            } => basic::run_transient(self, *stop, *step, start.unwrap_or(0.0), *max_step)?,
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => frequency::run_ac(self, *variation, *points, *start_freq, *stop_freq)?,
            AnalysisCommand::AcData { table_name } => frequency::run_ac_data(self, table_name)?,
            AnalysisCommand::Hb { frequencies } => {
                advanced::run_hb_from_command(self, frequencies)?
            }
            AnalysisCommand::Sp {
                variation,
                points,
                start_freq,
                stop_freq,
                do_noise,
            } => advanced::run_sparam_from_command(
                self,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *do_noise,
            )?,
            AnalysisCommand::Stb {
                variation,
                points,
                start_freq,
                stop_freq,
                probe,
            } => frequency::run_stb(self, *variation, *points, *start_freq, *stop_freq, probe)?,
            AnalysisCommand::Disto {
                variation,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            } => frequency::run_disto(
                self,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *f2_over_f1,
            )?,
            AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
            } => frequency::run_noise(
                self,
                output_node,
                reference_node.as_deref(),
                input_source,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
            )?,
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                output_is_current,
                filters,
                ac_sweep,
            } => frequency::run_sensitivity_from_command(
                self,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                filters,
                *ac_sweep,
            )?,
            AnalysisCommand::Tf {
                output_node,
                reference_node,
                output_is_current,
                input_source,
            } => frequency::run_tf_from_command(
                self,
                output_node,
                reference_node.as_deref(),
                *output_is_current,
                input_source,
            )?,
            AnalysisCommand::PoleZero {
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                transfer_type,
                analysis_type,
            } => frequency::run_pz_from_command(
                self,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                *transfer_type,
                *analysis_type,
            )?,
            AnalysisCommand::Step(step_cmd) => advanced::run_step(self, step_cmd)?,
            AnalysisCommand::Four {
                fundamental,
                outputs,
                num_harmonics,
            } => basic::run_fourier(self, *fundamental, outputs, *num_harmonics)?,
            AnalysisCommand::Temp { temperatures } => basic::run_temp(self, temperatures)?,
            AnalysisCommand::MonteCarlo(mc_cmd) => {
                advanced::run_monte_carlo_from_command(self, mc_cmd)?
            }
        }

        Ok(())
    }
}

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
    if let Some(seconds) = args.timeout {
        crate::abort::arm_timeout(seconds);
    }

    let resource_limits = config.resources.limits();
    let parse_options = rspice_core::netlist::NetlistParseOptions {
        resource_limits,
        ..rspice_core::netlist::NetlistParseOptions::default()
    };
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
        // rewritten .ALTER/.DATA decks. Reject before workers start so no
        // run can race another or leave a misleading partial artifact.
        for deck in &plan {
            let netlist = load_netlist_from_source(&deck.source, &args, config, false)?;
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
    let mut first_error: Option<String> = None;

    let workers = effective_jobs(args.jobs, plan.len());
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
        let outcomes: Vec<Result<(SimulationReport, Vec<PathBuf>), CliError>> =
            pool.install(|| {
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
            let (report, deck_outputs) = outcome?;
            if !quiet {
                if report.passed {
                    println!("  ✓ {label} ({:.3}s)", report.duration_secs);
                } else {
                    println!("  ✗ {label}: {}", status_failure_summary(&report));
                }
            }
            if first_error.is_none() {
                first_error = report.error.clone();
            }
            reports.push(report);
            outputs.extend(deck_outputs);
        }
    } else {
        for deck in &plan {
            if multi_run && !quiet {
                println!("\n=== run: {} ===", deck.label.as_deref().unwrap_or("base"));
            }
            let netlist = load_netlist_from_source(&deck.source, &args, config, !quiet)?;
            let addresistors_artifact =
                materialize_addresistors_artifact(&netlist, &args.input, from_stdin, args.timeout)?;
            let (report, deck_outputs) = run_deck(
                &netlist,
                &args,
                config,
                verbose,
                quiet,
                deck.label.as_deref(),
            )?;
            if first_error.is_none() {
                first_error = report.error.clone();
            }
            reports.push(report);
            outputs.extend(deck_outputs);
            if let Some(path) = addresistors_artifact {
                outputs.push(path);
            }
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    write_report_files(&reports, &args, verbose)?;

    let failed_measurements: Vec<&str> = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
    let abort_reason = crate::abort::reason();
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

    if let Some(err_msg) = first_error {
        return Err(CliError::simulation_error(err_msg));
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

fn materialize_addresistors_artifact(
    netlist: &Netlist,
    input: &std::path::Path,
    from_stdin: bool,
    timeout_seconds: Option<f64>,
) -> Result<Option<PathBuf>, CliError> {
    if netlist
        .options
        .add_resistors
        .as_ref()
        .is_none_or(|policy| policy.is_empty())
    {
        return Ok(None);
    }
    if from_stdin {
        return Err(CliError::InvalidArgument {
            message: ".PREPROCESS ADDRESISTORS requires a file-backed input; stdin has no unambiguous sibling artifact path"
                .to_string(),
            suggestion: Some(
                "save the deck to a file and run `rspice run <file>` to create <file>_xyce.cir"
                    .to_string(),
            ),
        });
    }

    let materialized = netlist
        .materialize_xyce_add_resistors_with_abort(&crate::abort::ProcessAbort)
        .map_err(|source| {
            if matches!(
                source,
                rspice_core::netlist::XyceAddResistorsMaterializationError::Aborted
            ) {
                match crate::abort::reason() {
                    Some(crate::abort::AbortReason::Interrupt) => return CliError::Interrupted,
                    Some(crate::abort::AbortReason::Timeout) => {
                        return CliError::TimedOut {
                            seconds: timeout_seconds.unwrap_or(0.0),
                        };
                    }
                    None => {}
                }
            }
            CliError::AddResistorsMaterialization { source }
        })?;
    let path = xyce_addresistors_artifact_path(input);
    atomic_write_addresistors_artifact(&path, materialized.derived_source.as_bytes()).map_err(
        |source| CliError::AddResistorsArtifactIo {
            path: path.clone(),
            source,
        },
    )?;
    Ok(Some(path))
}

fn xyce_addresistors_artifact_path(input: &std::path::Path) -> PathBuf {
    let mut name = input.as_os_str().to_os_string();
    name.push("_xyce.cir");
    PathBuf::from(name)
}

struct TemporaryArtifact {
    path: PathBuf,
    armed: bool,
}

impl Drop for TemporaryArtifact {
    fn drop(&mut self) {
        if self.armed {
            let _ = std::fs::remove_file(&self.path);
        }
    }
}

fn atomic_write_addresistors_artifact(path: &std::path::Path, bytes: &[u8]) -> std::io::Result<()> {
    use std::io::Write as _;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_TEMP: AtomicU64 = AtomicU64::new(0);

    reject_symlink_destination(path)?;
    let parent = path.parent().unwrap_or_else(|| std::path::Path::new("."));
    let file_name = path.file_name().ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "ADDRESISTORS artifact path has no file name",
        )
    })?;

    let mut opened = None;
    for _ in 0..128 {
        let sequence = NEXT_TEMP.fetch_add(1, Ordering::Relaxed);
        let mut temporary_name = file_name.to_os_string();
        temporary_name.push(format!(".tmp.{}.{sequence}", std::process::id()));
        let temporary_path = parent.join(temporary_name);
        match std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary_path)
        {
            Ok(file) => {
                opened = Some((temporary_path, file));
                break;
            }
            Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
            Err(error) => return Err(error),
        }
    }
    let (temporary_path, mut file) = opened.ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            "could not allocate a unique ADDRESISTORS temporary artifact",
        )
    })?;
    let mut guard = TemporaryArtifact {
        path: temporary_path.clone(),
        armed: true,
    };

    file.write_all(bytes)?;
    file.flush()?;
    file.sync_all()?;
    drop(file);

    // Recheck immediately before the atomic namespace operation. A racing
    // symlink can only be replaced as a directory entry by rename/MoveFileEx;
    // artifact bytes are never opened through it.
    reject_symlink_destination(path)?;
    replace_artifact_atomically(&temporary_path, path)?;
    guard.armed = false;
    Ok(())
}

fn reject_symlink_destination(path: &std::path::Path) -> std::io::Result<()> {
    match std::fs::symlink_metadata(path) {
        Ok(metadata) if metadata.file_type().is_symlink() => Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "refusing to replace ADDRESISTORS artifact symlink '{}'",
                path.display()
            ),
        )),
        Ok(metadata) if metadata.is_dir() => Err(std::io::Error::new(
            std::io::ErrorKind::IsADirectory,
            format!(
                "ADDRESISTORS artifact destination '{}' is a directory",
                path.display()
            ),
        )),
        Ok(_) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(error),
    }
}

#[cfg(not(windows))]
fn replace_artifact_atomically(
    from: &std::path::Path,
    to: &std::path::Path,
) -> std::io::Result<()> {
    std::fs::rename(from, to)?;
    std::fs::File::open(to.parent().unwrap_or_else(|| std::path::Path::new(".")))?.sync_all()
}

#[cfg(windows)]
fn replace_artifact_atomically(
    from: &std::path::Path,
    to: &std::path::Path,
) -> std::io::Result<()> {
    use std::os::windows::ffi::OsStrExt as _;
    use windows_sys::Win32::Storage::FileSystem::{
        MOVEFILE_REPLACE_EXISTING, MOVEFILE_WRITE_THROUGH, MoveFileExW,
    };

    let from_wide = from
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    let to_wide = to
        .as_os_str()
        .encode_wide()
        .chain(Some(0))
        .collect::<Vec<_>>();
    // SAFETY: both buffers are live, NUL-terminated UTF-16 paths. The flags
    // request same-directory atomic replacement and synchronous metadata
    // completion; the caller has closed the temporary file first.
    let result = unsafe {
        MoveFileExW(
            from_wide.as_ptr(),
            to_wide.as_ptr(),
            MOVEFILE_REPLACE_EXISTING | MOVEFILE_WRITE_THROUGH,
        )
    };
    if result == 0 {
        Err(std::io::Error::last_os_error())
    } else {
        Ok(())
    }
}

fn validate_run_numeric_args(args: &RunArgs) -> Result<(), CliError> {
    if matches!(args.maxiter, Some(0)) {
        return Err(invalid_run_arg(
            "--maxiter",
            "must be at least 1",
            "e.g. --maxiter 100",
        ));
    }

    require_celsius_arg("--temp", args.temp)?;
    require_positive_arg("--timeout", args.timeout)?;
    require_positive_arg("--tran-stop", args.tran_stop)?;
    require_positive_arg("--compress-tol", args.compress_tol)?;
    require_positive_arg("--abstol", args.abstol)?;
    require_positive_arg("--reltol", args.reltol)?;
    require_positive_arg("--residual-reltol", args.residual_reltol)?;
    require_positive_arg("--min-step", args.min_step)?;
    require_positive_arg("--max-step", args.max_step)?;
    if let (Some(min_step), Some(max_step)) = (args.min_step, args.max_step)
        && min_step > max_step
    {
        return Err(invalid_run_arg(
            "--min-step/--max-step",
            &format!("must satisfy --min-step <= --max-step, got {min_step} > {max_step}"),
            "set --min-step less than or equal to --max-step",
        ));
    }
    require_positive_arg("--trtol", args.trtol)?;
    require_non_negative_arg("--gmin", args.gmin)?;
    require_positive_arg("--voltage-abstol", args.voltage_abstol)?;
    require_positive_arg("--current-abstol", args.current_abstol)?;
    require_positive_arg("--charge-abstol", args.charge_abstol)?;
    require_non_negative_arg("--mc-spread", args.mc_spread)?;

    Ok(())
}

fn require_positive_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value <= 0.0)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be a positive finite number, got {value}"),
            "use a positive SPICE value",
        ));
    }
    Ok(())
}

fn require_non_negative_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value < 0.0)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be a finite non-negative number, got {value}"),
            "use 0 or a positive SPICE value",
        ));
    }
    Ok(())
}

fn require_celsius_arg(name: &str, value: Option<f64>) -> Result<(), CliError> {
    if let Some(value) = value
        && (!value.is_finite() || value <= -273.15)
    {
        return Err(invalid_run_arg(
            name,
            &format!("must be finite and above absolute zero, got {value} C"),
            "use a Celsius value greater than -273.15",
        ));
    }
    Ok(())
}

fn invalid_run_arg(name: &str, message: &str, suggestion: &str) -> CliError {
    CliError::InvalidArgument {
        message: format!("{name} {message}"),
        suggestion: Some(suggestion.to_string()),
    }
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
) -> Result<(), CliError> {
    let json = serde_json::json!({
        "tool": {
            "name": "rspice",
            "version": env!("CARGO_PKG_VERSION"),
        },
        "netlist": args.input.display().to_string(),
        "duration_secs": duration,
        "passed": passed,
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
                "duration_secs": report.duration_secs,
                "measurements": report.measurements.iter().map(|meas| {
                    serde_json::json!({
                        "name": meas.name,
                        "value": meas.value,
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
    std::fs::write(path, text + "\n").map_err(|e| CliError::output_error(path, e))?;
    Ok(())
}

/// Run one concrete deck (all of its analyses) and assemble its report.
/// Multi-run failures don't abort the remaining runs — HSPICE semantics —
/// so errors land in the report instead of bubbling, except for setup
/// errors (bad output paths, alternate-mode failures).
fn run_deck(
    netlist: &Netlist,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
    run_label: Option<&str>,
) -> Result<(SimulationReport, Vec<PathBuf>), CliError> {
    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::try_new(sim_config)?;
    let ctx = RunContext::new(&engine, netlist, args, config, verbose, quiet, run_label)?;

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
        let measurements = ctx.measurements.borrow().clone();
        let passed = measurements.iter().all(|meas| meas.passed);
        return Ok((
            SimulationReport {
                name,
                netlist: args.input.display().to_string(),
                passed,
                duration_secs: start_time.elapsed().as_secs_f64(),
                error: None,
                measurements,
            },
            ctx.outputs.into_inner(),
        ));
    }

    let mut ran_analysis = false;
    let mut simulation_error: Option<String> = None;

    for (idx, analysis) in netlist.analyses.iter().enumerate() {
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
            simulation_error = Some(simulation_error_message(&e));
        }
    }

    if args.meas && !quiet && netlist.measurements.is_empty() {
        println!("  No .MEAS statements found in netlist");
    }
    ctx.record_unevaluated_measurements();

    let duration = start_time.elapsed().as_secs_f64();
    let measurements = ctx.measurements.borrow().clone();
    let passed = simulation_error.is_none() && measurements.iter().all(|meas| meas.passed);

    Ok((
        SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed,
            duration_secs: duration,
            error: simulation_error,
            measurements,
        },
        ctx.outputs.into_inner(),
    ))
}

/// Failure text for the run report: simulation errors carry their bare
/// message (re-wrapping at exit would otherwise print
/// "Simulation failed: Simulation failed: ...").
fn simulation_error_message(e: &CliError) -> String {
    match e {
        CliError::SimulationError { message, .. } => message.clone(),
        other => other.to_string(),
    }
}

fn is_run_setup_or_output_error(error: &CliError) -> bool {
    matches!(
        error,
        CliError::OutputError { .. } | CliError::OutputSerializationError { .. }
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

/// Worker count for a multi-run plan: `--jobs 0` = all cores, never
/// more workers than runs, and single-run plans stay serial.
fn effective_jobs(requested: usize, runs: usize) -> usize {
    if runs <= 1 {
        return 1;
    }
    let cores = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    let workers = if requested == 0 { cores } else { requested };
    workers.min(runs).max(1)
}

/// `out.csv` + `hot` -> `out.hot.csv` (run-level analog of the
/// per-analysis tagging in `output_path_for`).
fn tag_output_path(path: &std::path::Path, tag: &str) -> PathBuf {
    let mut file_name = path
        .file_stem()
        .map(|stem| stem.to_os_string())
        .unwrap_or_default();
    file_name.push(format!(".{tag}"));
    if let Some(ext) = path.extension() {
        file_name.push(".");
        file_name.push(ext);
    }
    path.with_file_name(file_name)
}

/// Reduce a run label to a file-name-safe tag.
fn sanitize_run_tag(label: &str) -> String {
    let mut tag: String = label
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() {
                ch.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    while tag.contains("__") {
        tag = tag.replace("__", "_");
    }
    tag.trim_matches('_').to_string()
}

fn load_netlist_from_source(
    source: &str,
    args: &RunArgs,
    config: &Config,
    emit_diagnostics: bool,
) -> Result<Netlist, CliError> {
    let mut search_paths: Vec<PathBuf> = args.includes.clone();
    search_paths.extend(config.paths.include_paths.iter().cloned());
    search_paths.extend(config.paths.library_paths.iter().cloned());
    let parse_options = rspice_core::netlist::NetlistParseOptions {
        resource_limits: config.resources.limits(),
        ..rspice_core::netlist::NetlistParseOptions::default()
    };

    let parsed = if search_paths.is_empty() {
        Netlist::parse_with_path_and_options_and_abort(
            source,
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
    } else {
        Netlist::parse_with_search_paths_and_options_and_abort(
            source,
            &args.input,
            &search_paths,
            parse_options,
            &crate::abort::ProcessAbort,
        )
    };
    let mut netlist = parsed.map_err(|error| map_cancellable_parse_error(error, args.timeout))?;

    if !args.defines.is_empty() {
        let defines: Vec<(String, f64)> = args
            .defines
            .iter()
            .map(|define| parse_define(define))
            .collect::<Result<_, _>>()?;

        // Parameter overrides must win over the deck's own .PARAM assignments
        // and must be visible at parse time (simple `{param}` references are
        // resolved while parsing). Rewrite the include-expanded source so the
        // override is indistinguishable from a deck edit, then re-parse.
        let source = netlist
            .source_text
            .clone()
            .ok_or_else(|| CliError::InternalError {
                message: "netlist source unavailable for --define substitution".to_string(),
            })?;
        let rewritten = apply_defines_to_source(&source, &defines);
        netlist = Netlist::parse_with_path_and_options_and_abort(
            &rewritten,
            &args.input,
            parse_options,
            &crate::abort::ProcessAbort,
        )
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?;
        for (name, value) in &defines {
            netlist.params.set(name, *value);
        }
    }

    // --save replaces the deck's output selection outright: the caller is
    // asking for exactly these signals. Applied after any -D re-parse so the
    // override always wins.
    if !args.saves.is_empty() {
        let mut saves = rspice_core::netlist::SaveSet::default();
        let mut override_requests = Vec::with_capacity(args.saves.len());
        for (index, spec) in args.saves.iter().enumerate() {
            // The netlist parser falls back to a bare vector name for
            // anything unrecognized; a spec with parentheses that didn't
            // parse as V(...)/I(...) is a typo, not a vector name.
            let parsed = rspice_core::netlist::parse_save_probe(spec);
            let malformed = match &parsed {
                None => true,
                Some(rspice_core::netlist::SaveSignal::Raw(_)) => {
                    spec.contains('(') || spec.contains(')')
                }
                Some(_) => false,
            };
            if malformed {
                return Err(CliError::InvalidArgument {
                    message: format!("invalid --save probe '{spec}'"),
                    suggestion: Some(
                        "use forms like V(out), V(a,b), I(v1), @m1[id], or all".to_string(),
                    ),
                });
            }
            saves.signals.push(parsed.expect("checked above"));
            override_requests.push(rspice_core::netlist::OutputRequest::from_save_override(
                rspice_core::netlist::NetlistSourceLocation::in_file(
                    "<command line --save>",
                    index + 1,
                ),
                spec,
            ));
        }
        saves.apply_ground_policy(netlist.ground_policy());
        netlist.saves = saves;
        netlist.output_requests.retain(|request| {
            !matches!(
                request.directive,
                rspice_core::netlist::OutputDirectiveKind::Save
                    | rspice_core::netlist::OutputDirectiveKind::Probe
                    | rspice_core::netlist::OutputDirectiveKind::Print
                    | rspice_core::netlist::OutputDirectiveKind::Plot
            )
        });
        netlist.output_requests.extend(override_requests);
    }

    rspice_core::netlist::validate_output_symbols_with_abort(&netlist, &crate::abort::ProcessAbort)
        .map_err(|error| map_cancellable_parse_error(error, args.timeout))?;

    if emit_diagnostics {
        crate::commands::emit_netlist_diagnostics(&netlist, false);
    }

    Ok(netlist)
}

/// Parse a config `output.format` name.
fn parse_format_name(name: &str) -> Result<OutputFormat, CliError> {
    use clap::ValueEnum;
    OutputFormat::from_str(name, true).map_err(|_| CliError::ConfigError {
        message: format!(
            "invalid output.format '{}'; expected one of: raw, ascii, csv, json, tsv, hdf5",
            name
        ),
    })
}

/// Resolve `-o` against config `output.output_directory`.
///
/// Relative output paths are placed inside the configured directory (created
/// on demand); absolute paths are used as given.
fn resolve_output_path(
    output: Option<PathBuf>,
    config: &Config,
) -> Result<Option<PathBuf>, CliError> {
    let Some(path) = output else {
        return Ok(None);
    };
    let Some(dir) = config.output.output_directory.as_ref() else {
        return Ok(Some(path));
    };
    if path.is_absolute() {
        return Ok(Some(path));
    }

    std::fs::create_dir_all(dir).map_err(|e| CliError::OutputError {
        path: dir.clone(),
        source: e,
    })?;
    Ok(Some(dir.join(path)))
}

/// Parse a `-D NAME=VALUE` override; the value accepts SPICE suffixes (4.7k, 1u, ...).
fn parse_define(define: &str) -> Result<(String, f64), CliError> {
    let invalid = |message: String| CliError::InvalidArgument {
        message,
        suggestion: Some("use -D NAME=VALUE, e.g. -D RLOAD=4.7k".to_string()),
    };

    let (name, value) = define
        .split_once('=')
        .ok_or_else(|| invalid(format!("malformed --define '{}'", define)))?;
    let name = name.trim();
    if name.is_empty() {
        return Err(invalid(format!(
            "missing parameter name in --define '{}'",
            define
        )));
    }

    let value = rspice_core::netlist::lexer::parse_spice_value(value.trim())
        .map_err(|e| invalid(format!("invalid value in --define '{}': {}", define, e)))?;
    Ok((name.to_string(), value))
}

/// Apply `-D` overrides to include-expanded netlist source.
///
/// Each override is injected as a `.param` right after the title so it exists
/// before first use, and every top-level `.param` assignment of the same name
/// is rewritten in place so the deck cannot re-assign it. Assignments inside
/// `.subckt` bodies are left alone — overrides target global parameters only.
fn apply_defines_to_source(source: &str, defines: &[(String, f64)]) -> String {
    let mut out = String::with_capacity(source.len() + defines.len() * 32);
    let mut lines = source.lines();

    if let Some(title) = lines.next() {
        out.push_str(title);
        out.push('\n');
    }
    for (name, value) in defines {
        out.push_str(&format!(".param {}={:e}\n", name, value));
    }

    let mut subckt_depth = 0usize;
    let mut in_param_statement = false;
    for line in lines {
        let trimmed = line.trim_start();
        let keyword = trimmed
            .split_whitespace()
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();

        if keyword == ".subckt" {
            subckt_depth += 1;
        } else if keyword == ".ends" {
            subckt_depth = subckt_depth.saturating_sub(1);
        }

        let continues_param = in_param_statement && trimmed.starts_with('+');
        if keyword == ".param" || continues_param {
            in_param_statement = true;
            if subckt_depth == 0 {
                out.push_str(&rewrite_param_assignments(line, defines));
            } else {
                out.push_str(line);
            }
        } else {
            if !trimmed.is_empty() && !trimmed.starts_with('*') {
                in_param_statement = false;
            }
            out.push_str(line);
        }
        out.push('\n');
    }

    out
}

/// Rewrite `name=value` assignments in a `.param` line for overridden names.
///
/// Values are scanned with brace/paren depth tracking so expressions like
/// `{a + b}` or `max(1, 2)` stay intact.
fn rewrite_param_assignments(text: &str, defines: &[(String, f64)]) -> String {
    let chars: Vec<char> = text.chars().collect();
    let mut out = String::with_capacity(text.len());
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_alphabetic() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            let ident: String = chars[start..i].iter().collect();

            let mut j = i;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && chars[j] == '=' {
                j += 1;
                while j < chars.len() && chars[j].is_whitespace() {
                    j += 1;
                }
                let value_start = j;
                let mut depth = 0i32;
                while j < chars.len() {
                    match chars[j] {
                        '(' | '{' | '[' => depth += 1,
                        ')' | '}' | ']' => depth -= 1,
                        ch if depth == 0 && (ch.is_whitespace() || ch == ',') => break,
                        _ => {}
                    }
                    j += 1;
                }

                out.push_str(&ident);
                out.push('=');
                match defines
                    .iter()
                    .find(|(name, _)| name.eq_ignore_ascii_case(&ident))
                {
                    Some((_, value)) => out.push_str(&format!("{:e}", value)),
                    None => out.extend(&chars[value_start..j]),
                }
                i = j;
                continue;
            }

            out.push_str(&ident);
            continue;
        }

        out.push(c);
        i += 1;
    }

    out
}

fn build_sim_config(args: &RunArgs, config: &Config, netlist: &Netlist) -> SimulationConfig {
    let base = SimulationConfig {
        temperature: config.simulation.temperature + 273.15,
        max_iterations: config.simulation.max_iterations,
        min_timestep: config.simulation.min_timestep,
        max_timestep: config.simulation.max_timestep,
        tolerance: config.simulation.reltol,
        convergence_config: ConvergenceConfig {
            voltage_reltol: config.simulation.reltol,
            // SPICE tolerance semantics: ABSTOL bounds branch currents and
            // VNTOL bounds node voltages. Applying the 1e-12 current floor
            // to voltages demands convergence a million times tighter than
            // ngspice and stalls junction decks at conduction handoffs.
            // Deck-level `.options vntol` still overrides through the
            // config resolver.
            voltage_abstol: 1e-6,
            current_abstol: config.simulation.abstol,
            residual_reltol: config.simulation.residual_reltol,
            ..ConvergenceConfig::default()
        },
        resource_limits: config.resources.limits(),
        ..SimulationConfig::default()
    };

    let convergence_mode = args
        .convergence
        .as_deref()
        .unwrap_or(&config.simulation.convergence_mode);
    let convergence_preset = ConvergencePreset::from_mode_name(convergence_mode);

    let integration_method = args.integration_method.as_deref().map(|method| {
        use rspice_core::analysis::IntegrationMethod;
        match method {
            "euler" => IntegrationMethod::BackwardEuler,
            "trap" => IntegrationMethod::Trapezoidal,
            "gear" => IntegrationMethod::Gear2,
            _ => IntegrationMethod::TrapGear,
        }
    });

    let overrides = SimulationConfigOverrides {
        temperature_kelvin: args.temp.map(|temp_c| temp_c + 273.15),
        max_iterations: args.maxiter,
        min_timestep: args.min_step,
        max_timestep: args.max_step,
        integration_method,
        transient_trtol: args.trtol,
        transient_lte_reltol: None,
        transient_lte_abstol: None,
        transient_lte_reference: None,
        transient_new_bp_stepping: None,
        ramptime: None,
        convergence_preset,
        reltol: args.reltol,
        abstol: args.abstol,
        voltage_abstol: args.voltage_abstol,
        current_abstol: args.current_abstol,
        charge_abstol: args.charge_abstol,
        residual_reltol: args.residual_reltol,
        gmin_initial: args.gmin,
        digital_delay_type: None,
        spice_dialect: None,
        jfet_level2_model: None,
    };

    resolve_simulation_config(&base, Some(&netlist.options), &overrides)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestedModeOutcome {
    NotRequested,
    RanNeedsMeasurementFinalization,
    RanManagedMeasurements,
}

impl RequestedModeOutcome {
    fn ran(self) -> bool {
        !matches!(self, Self::NotRequested)
    }

    fn needs_measurement_finalization(self) -> bool {
        matches!(self, Self::RanNeedsMeasurementFinalization)
    }
}

fn run_requested_mode(
    ctx: &RunContext<'_>,
    _config: &Config,
) -> Result<RequestedModeOutcome, CliError> {
    if let Some(num_runs) = ctx.args.monte_carlo {
        let spread = ctx.args.mc_spread.unwrap_or(0.01);
        if !spread.is_finite() || spread < 0.0 {
            return Err(CliError::InvalidArgument {
                message: format!("--mc-spread must be a finite non-negative value, got {spread}"),
                suggestion: Some(
                    "Use 0 for deterministic samples, or e.g. --mc-spread 0.05 for 5% variation"
                        .to_string(),
                ),
            });
        }
        let distribution = match ctx.args.mc_distribution.as_deref() {
            Some("uniform") => rspice_core::analysis::Distribution::Uniform { tolerance: spread },
            Some("worst-case") => {
                rspice_core::analysis::Distribution::WorstCase { tolerance: spread }
            }
            _ => rspice_core::analysis::Distribution::Gaussian { sigma: spread },
        };
        let parameter_filter = if ctx.args.mc_params.is_empty() {
            None
        } else {
            Some(ctx.args.mc_params.as_slice())
        };
        advanced::run_monte_carlo(
            ctx,
            num_runs,
            ctx.args.seed.unwrap_or(1),
            distribution,
            parameter_filter,
        )?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(freq) = ctx.args.pss_freq {
        advanced::run_pss(ctx, freq, ctx.args.pss_harmonics, ctx.args.pss_tstab)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(freq) = ctx.args.hb_freq {
        advanced::run_hb(ctx, freq, ctx.args.hb_harmonics)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let (Some(input), Some(output)) =
        (ctx.args.pz_input.as_deref(), ctx.args.pz_output.as_deref())
    {
        let (input, output) = resolve_node_pair(ctx, input, output, "--pz-input/--pz-output")?;
        frequency::run_pz(ctx, input, output)?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let (Some(output_node), Some(param)) = (
        ctx.args.sens_output.as_deref(),
        ctx.args.sens_param.as_deref(),
    ) {
        let output_node = resolve_node(ctx, output_node, "--sens-output")?;
        frequency::run_sensitivity(ctx, output_node, param, ctx.args.sens_value.unwrap_or(1.0))?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(ports) = ctx.args.sparam.as_deref() {
        advanced::run_sparam(ctx, ports, ctx.args.sparam_z0.unwrap_or(50.0))?;
        return Ok(RequestedModeOutcome::RanNeedsMeasurementFinalization);
    }

    if let Some(corners_str) = ctx.args.corners.as_deref() {
        advanced::run_corner_sweep(ctx, corners_str)?;
        return Ok(RequestedModeOutcome::RanManagedMeasurements);
    }

    Ok(RequestedModeOutcome::NotRequested)
}

/// Resolve a node given by name or index for analysis flags.
fn resolve_node(ctx: &RunContext<'_>, node: &str, flag: &str) -> Result<usize, CliError> {
    let resolver = shared::NodeResolver::from_netlist(ctx.engine, ctx.netlist)?;
    resolver
        .resolve_node(node)
        .ok_or_else(|| CliError::InvalidArgument {
            message: format!("unknown node '{node}' for {flag}"),
            suggestion: Some("pass a node name from the netlist or a node index".to_string()),
        })
}

fn resolve_node_pair(
    ctx: &RunContext<'_>,
    first: &str,
    second: &str,
    flag: &str,
) -> Result<(usize, usize), CliError> {
    Ok((
        resolve_node(ctx, first, flag)?,
        resolve_node(ctx, second, flag)?,
    ))
}
