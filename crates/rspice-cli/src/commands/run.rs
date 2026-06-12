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
        let format = args
            .format
            .unwrap_or_else(|| parse_format_name(&config.output.format));
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
        })
    }

    /// Record evaluated .MEAS results: print them under `--meas` and keep
    /// them for report files and the exit status.
    fn record_measurements(
        &self,
        analysis: &str,
        results: Vec<rspice_core::MeasureResult>,
    ) {
        if results.is_empty() {
            return;
        }
        self.evaluated_meas
            .borrow_mut()
            .insert(analysis.to_ascii_uppercase());

        if self.args.meas && !self.quiet {
            println!("  Measurement Results ({}, {}):", analysis, results.len());
            for mr in &results {
                match mr.value {
                    Some(value) => println!(
                        "    {} = {}",
                        mr.name,
                        crate::report::format_spice_exponent(value)
                    ),
                    None => println!(
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
                expected: None,
                tolerance: None,
                passed: mr.value.is_some(),
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
                "TRAN" | "DC" => format!("{analysis} analysis did not run"),
                other => format!("{other} measurements are not supported by `rspice run` yet"),
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
    fn output_path_for(&self, tag: &str) -> Option<std::path::PathBuf> {
        let path = self.output.clone()?;
        if !self.multi_analysis {
            return Some(path);
        }

        let mut file_name = path
            .file_stem()
            .map(|stem| stem.to_os_string())
            .unwrap_or_default();
        file_name.push(format!(".{tag}"));
        if let Some(ext) = path.extension() {
            file_name.push(".");
            file_name.push(ext);
        }
        Some(path.with_file_name(file_name))
    }

    fn run_analysis(&self, analysis: &AnalysisCommand) -> Result<(), CliError> {
        match analysis {
            AnalysisCommand::Op => basic::run_dc_op(self)?,
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
            } => basic::run_dc_sweep(self, source, *start, *stop, *step)?,
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
            } => basic::run_transient(self, *stop, *step, start.unwrap_or(0.0), *max_step)?,
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => frequency::run_ac(self, *variation, *points, *start_freq, *stop_freq)?,
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
                ac_sweep,
            } => frequency::run_sensitivity_from_command(
                self,
                output_node,
                reference_node.as_deref(),
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

pub fn execute(args: RunArgs, config: &Config, verbose: bool, quiet: bool) -> Result<(), CliError> {
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    log::info!("Loading netlist: {}", args.input.display());
    let source = Netlist::read_source(&args.input).map_err(|e| CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    })?;

    // HSPICE `.ALTER` / `.DATA` constructs expand into several concrete
    // runs; a plain deck passes through as a single unlabeled run.
    let plan = rspice_core::netlist::multi_run::expand_multi_run(&source);
    let multi_run = plan.len() > 1;
    if multi_run && !quiet {
        println!("Multi-run deck: {} runs (.alter/.data expansion)", plan.len());
    }

    let start_time = Instant::now();
    let mut reports = Vec::with_capacity(plan.len());
    let mut first_error: Option<String> = None;

    for deck in &plan {
        if multi_run && !quiet {
            println!(
                "\n=== run: {} ===",
                deck.label.as_deref().unwrap_or("base")
            );
        }
        let netlist = load_netlist_from_source(&deck.source, &args, config)?;
        let report = run_deck(
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
    }

    let duration = start_time.elapsed().as_secs_f64();
    write_report_files(&reports, &args, verbose)?;

    if !quiet {
        println!("\nSimulation complete in {:.3}s.", duration);
    }

    if let Some(err_msg) = first_error {
        return Err(CliError::simulation_error(err_msg));
    }

    // The simulation itself succeeded; failed .MEAS checks still fail the
    // process so automation can trust the exit status.
    let failed_measurements: Vec<&str> = reports
        .iter()
        .flat_map(|report| &report.measurements)
        .filter(|meas| !meas.passed)
        .map(|meas| meas.name.as_str())
        .collect();
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
) -> Result<SimulationReport, CliError> {
    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(args, config, netlist);
    let engine = Engine::new(sim_config);
    let ctx = RunContext::new(&engine, netlist, args, config, verbose, quiet, run_label)?;

    let base_name = args
        .input
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("simulation")
        .to_string();
    let name = match run_label {
        Some(label) => format!("{base_name} [{label}]"),
        None => base_name,
    };

    if run_requested_mode(&ctx, config)? {
        return Ok(SimulationReport {
            name,
            netlist: args.input.display().to_string(),
            passed: true,
            duration_secs: 0.0,
            error: None,
            measurements: Vec::new(),
        });
    }

    let mut ran_analysis = false;
    let start_time = Instant::now();
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
            simulation_error = Some(simulation_error_message(&e));
            break;
        }
    }

    if !ran_analysis && simulation_error.is_none() {
        if !quiet {
            println!("No analysis commands - running default DC OP...");
        }
        if let Err(e) = basic::run_dc_op(&ctx) {
            simulation_error = Some(simulation_error_message(&e));
        }
    }

    if args.meas && !quiet && netlist.measurements.is_empty() {
        println!("  No .MEAS statements found in netlist");
    }
    ctx.record_unevaluated_measurements();

    let duration = start_time.elapsed().as_secs_f64();
    let passed = simulation_error.is_none();
    let measurements = ctx.measurements.borrow().clone();

    Ok(SimulationReport {
        name,
        netlist: args.input.display().to_string(),
        passed,
        duration_secs: duration,
        error: simulation_error,
        measurements,
    })
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
) -> Result<Netlist, CliError> {
    let mut search_paths: Vec<PathBuf> = args.includes.clone();
    search_paths.extend(config.paths.include_paths.iter().cloned());
    search_paths.extend(config.paths.library_paths.iter().cloned());

    let parsed = if search_paths.is_empty() {
        Netlist::parse_with_path(source, &args.input)
    } else {
        Netlist::parse_with_search_paths(source, &args.input, &search_paths)
    };
    let map_parse_error = |e: rspice_core::error::ParseError| CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    };
    let mut netlist = parsed.map_err(map_parse_error)?;

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
        let source = netlist.source_text.clone().ok_or_else(|| CliError::InternalError {
            message: "netlist source unavailable for --define substitution".to_string(),
        })?;
        let rewritten = apply_defines_to_source(&source, &defines);
        netlist = Netlist::parse_with_path(&rewritten, &args.input).map_err(map_parse_error)?;
        for (name, value) in &defines {
            netlist.params.set(name, *value);
        }
    }

    Ok(netlist)
}

/// Parse a config `output.format` name; unknown names warn and fall back to raw.
fn parse_format_name(name: &str) -> OutputFormat {
    use clap::ValueEnum;
    OutputFormat::from_str(name, true).unwrap_or_else(|_| {
        log::warn!("unrecognized output.format '{}' in config; using raw", name);
        OutputFormat::Raw
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
        return Err(invalid(format!("missing parameter name in --define '{}'", define)));
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
            voltage_abstol: config.simulation.abstol,
            current_abstol: config.simulation.abstol,
            residual_reltol: config.simulation.residual_reltol,
            ..ConvergenceConfig::default()
        },
        ..SimulationConfig::default()
    };

    let convergence_mode = args
        .convergence
        .as_deref()
        .unwrap_or(&config.simulation.convergence_mode);
    let convergence_preset = ConvergencePreset::from_mode_name(convergence_mode);

    let overrides = SimulationConfigOverrides {
        temperature_kelvin: args.temp.map(|temp_c| temp_c + 273.15),
        max_iterations: args.maxiter,
        min_timestep: args.min_step,
        max_timestep: args.max_step,
        integration_method: None,
        transient_trtol: None,
        convergence_preset,
        reltol: args.reltol,
        abstol: args.abstol,
        voltage_abstol: None,
        current_abstol: None,
        charge_abstol: None,
        residual_reltol: args.residual_reltol,
        gmin_initial: None,
    };

    resolve_simulation_config(&base, Some(&netlist.options), &overrides)
}

fn run_requested_mode(ctx: &RunContext<'_>, _config: &Config) -> Result<bool, CliError> {
    if let Some(num_runs) = ctx.args.monte_carlo {
        advanced::run_monte_carlo(
            ctx,
            num_runs,
            ctx.args.seed.unwrap_or(1),
            rspice_core::analysis::Distribution::Gaussian { sigma: 0.01 },
            None,
        )?;
        return Ok(true);
    }

    if let Some(freq) = ctx.args.pss_freq {
        advanced::run_pss(ctx, freq, ctx.args.pss_harmonics, ctx.args.pss_tstab)?;
        return Ok(true);
    }

    if let Some(freq) = ctx.args.hb_freq {
        advanced::run_hb(ctx, freq, ctx.args.hb_harmonics)?;
        return Ok(true);
    }

    if let (Some(input), Some(output)) = (ctx.args.pz_input, ctx.args.pz_output) {
        frequency::run_pz(ctx, input, output)?;
        return Ok(true);
    }

    if let (Some(output_node), Some(param)) = (ctx.args.sens_output, ctx.args.sens_param.as_deref())
    {
        frequency::run_sensitivity(ctx, output_node, param, ctx.args.sens_value.unwrap_or(1.0))?;
        return Ok(true);
    }

    if let Some(corners_str) = ctx.args.corners.as_deref() {
        advanced::run_corner_sweep(ctx, corners_str)?;
        return Ok(true);
    }

    Ok(false)
}
