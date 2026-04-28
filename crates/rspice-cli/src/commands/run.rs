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

use crate::cli::{CliError, Config, OutputFormat, RunArgs};
use crate::report::{JUnitReporter, JsonMeasReporter, SimulationReport, TapReporter};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergenceConfig, ConvergencePreset, Engine, Netlist, SimulationConfig,
    SimulationConfigOverrides, resolve_simulation_config,
};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;
use std::time::Instant;

struct RunContext<'a> {
    engine: &'a Engine,
    netlist: &'a Netlist,
    args: &'a RunArgs,
    verbose: bool,
    quiet: bool,
}

fn placeholder_run_args() -> &'static RunArgs {
    static RUN_ARGS: OnceLock<RunArgs> = OnceLock::new();

    RUN_ARGS.get_or_init(|| RunArgs {
        input: PathBuf::from("placeholder.sp"),
        output: None,
        format: OutputFormat::Raw,
        batch: false,
        temp: None,
        meas: false,
        progress: false,
        node_names: false,
        compress: false,
        compress_tol: None,
        maxiter: None,
        abstol: None,
        reltol: None,
        residual_reltol: None,
        min_step: None,
        max_step: None,
        includes: vec![],
        defines: vec![],
        monte_carlo: None,
        seed: None,
        report_format: None,
        report_file: None,
        meas_format: None,
        meas_file: None,
        pss_freq: None,
        pss_harmonics: 9,
        pss_tstab: None,
        hb_freq: None,
        hb_harmonics: 9,
        pz_input: None,
        pz_output: None,
        sens_output: None,
        sens_param: None,
        sens_value: None,
        corners: None,
        corner_lib: None,
        convergence: None,
    })
}

impl<'a> RunContext<'a> {
    fn new(
        engine: &'a Engine,
        netlist: &'a Netlist,
        args: &'a RunArgs,
        verbose: bool,
        quiet: bool,
    ) -> Self {
        Self {
            engine,
            netlist,
            args,
            verbose,
            quiet,
        }
    }

    fn without_args(engine: &'a Engine, netlist: &'a Netlist, verbose: bool, quiet: bool) -> Self {
        Self::new(engine, netlist, placeholder_run_args(), verbose, quiet)
    }

    fn run_analysis(&self, analysis: &AnalysisCommand) -> Result<(), CliError> {
        match analysis {
            AnalysisCommand::Op => run_dc_op(self.engine, self.netlist, self.args, self.quiet)?,
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
            } => run_dc_sweep(
                self.engine,
                self.netlist,
                source,
                *start,
                *stop,
                *step,
                self.args,
                self.quiet,
            )?,
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step,
            } => run_transient(
                self.engine,
                self.netlist,
                *stop,
                *step,
                start.unwrap_or(0.0),
                *max_step,
                self.args,
                self.quiet,
            )?,
            AnalysisCommand::Ac {
                variation,
                points,
                start_freq,
                stop_freq,
            } => run_ac(
                self.engine,
                self.netlist,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                self.args,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::Disto {
                variation,
                points,
                start_freq,
                stop_freq,
                f2_over_f1,
            } => run_disto(
                self.engine,
                self.netlist,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *f2_over_f1,
                self.args,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::Noise {
                output_node,
                reference_node,
                input_source,
                variation,
                points,
                start_freq,
                stop_freq,
            } => run_noise(
                self.engine,
                self.netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                self.quiet,
            )?,
            AnalysisCommand::Sensitivity {
                output_node,
                reference_node,
                ac_sweep,
            } => run_sensitivity_from_command(
                self.engine,
                self.netlist,
                output_node,
                reference_node.as_deref(),
                *ac_sweep,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::PoleZero {
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                transfer_type,
                analysis_type,
            } => run_pz_from_command(
                self.engine,
                self.netlist,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                *transfer_type,
                *analysis_type,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::Step(step_cmd) => run_step(
                self.engine,
                self.netlist,
                step_cmd,
                self.args,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::Four {
                fundamental,
                outputs,
                num_harmonics,
            } => run_fourier(
                self.engine,
                self.netlist,
                *fundamental,
                outputs,
                *num_harmonics,
                self.args,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::Temp { temperatures } => run_temp(
                self.engine,
                self.netlist,
                temperatures,
                self.args,
                self.verbose,
                self.quiet,
            )?,
            AnalysisCommand::MonteCarlo(mc_cmd) => run_monte_carlo_from_command(
                self.engine,
                self.netlist,
                mc_cmd,
                self.args,
                self.verbose,
                self.quiet,
            )?,
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
    let netlist = load_netlist(&args.input)?;

    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    let sim_config = build_sim_config(&args, config, &netlist);
    let engine = Engine::new(sim_config);
    let ctx = RunContext::new(&engine, &netlist, &args, verbose, quiet);

    if run_requested_mode(&ctx, config)? {
        return Ok(());
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
            simulation_error = Some(e.to_string());
            break;
        }
    }

    if !ran_analysis && simulation_error.is_none() {
        if !quiet {
            println!("No analysis commands - running default DC OP...");
        }
        if let Err(e) = basic::run_dc_op(&ctx) {
            simulation_error = Some(e.to_string());
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    let passed = simulation_error.is_none();

    let report = SimulationReport {
        name: args
            .input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("simulation")
            .to_string(),
        netlist: args.input.display().to_string(),
        passed,
        duration_secs: duration,
        error: simulation_error.clone(),
        measurements: Vec::new(),
    };

    if let Some(ref report_file) = args.report_file {
        let reports = vec![report.clone()];
        match args.report_format {
            Some(crate::cli::ReportFormat::Junit) | None => {
                JUnitReporter::write(&reports, report_file)?;
                if verbose {
                    println!("JUnit report written to: {}", report_file.display());
                }
            }
            Some(crate::cli::ReportFormat::Tap) => {
                TapReporter::write(&reports, report_file)?;
                if verbose {
                    println!("TAP report written to: {}", report_file.display());
                }
            }
        }
    }

    if let Some(ref meas_file) = args.meas_file {
        let reports = vec![report.clone()];
        JsonMeasReporter::write(&reports, meas_file)?;
        if verbose {
            println!("Measurement report written to: {}", meas_file.display());
        }
    }

    if !quiet {
        println!("\nSimulation complete in {:.3}s.", duration);
    }

    if let Some(err_msg) = simulation_error {
        return Err(CliError::SimulationError {
            message: err_msg,
            analysis: None,
        });
    }

    Ok(())
}

fn load_netlist(path: &Path) -> Result<Netlist, CliError> {
    Netlist::parse_file(path).map_err(|e| CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    })
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

fn run_requested_mode(ctx: &RunContext<'_>, config: &Config) -> Result<bool, CliError> {
    if let Some(num_runs) = ctx.args.monte_carlo {
        run_monte_carlo(
            ctx.engine,
            ctx.netlist,
            num_runs,
            ctx.args.seed.unwrap_or(1),
            rspice_core::analysis::Distribution::Gaussian { sigma: 0.01 },
            None,
            ctx.args,
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    if let Some(freq) = ctx.args.pss_freq {
        run_pss(
            ctx.engine,
            ctx.netlist,
            freq,
            ctx.args.pss_harmonics,
            ctx.args.pss_tstab,
            ctx.args,
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    if let Some(freq) = ctx.args.hb_freq {
        run_hb(
            ctx.engine,
            ctx.netlist,
            freq,
            ctx.args.hb_harmonics,
            ctx.args,
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    if let (Some(input), Some(output)) = (ctx.args.pz_input, ctx.args.pz_output) {
        run_pz(
            ctx.engine,
            ctx.netlist,
            input,
            output,
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    if let (Some(output_node), Some(param)) = (ctx.args.sens_output, ctx.args.sens_param.as_deref())
    {
        run_sensitivity(
            ctx.engine,
            ctx.netlist,
            output_node,
            param,
            ctx.args.sens_value.unwrap_or(1.0),
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    if let Some(corners_str) = ctx.args.corners.as_deref() {
        run_corner_sweep(
            ctx.engine,
            ctx.netlist,
            corners_str,
            ctx.args,
            config,
            ctx.verbose,
            ctx.quiet,
        )?;
        return Ok(true);
    }

    Ok(false)
}


fn run_disto(
    engine: &Engine,
    netlist: &Netlist,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
    f2_over_f1: Option<f64>,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_disto(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        variation,
        points,
        start_freq,
        stop_freq,
        f2_over_f1,
    )
}

fn run_dc_op(
    engine: &Engine,
    netlist: &Netlist,
    args: &RunArgs,
    quiet: bool,
) -> Result<(), CliError> {
    basic::run_dc_op(&RunContext::new(engine, netlist, args, false, quiet))
}

fn run_dc_sweep(
    engine: &Engine,
    netlist: &Netlist,
    source: &str,
    start: f64,
    stop: f64,
    step: f64,
    args: &RunArgs,
    quiet: bool,
) -> Result<(), CliError> {
    basic::run_dc_sweep(
        &RunContext::new(engine, netlist, args, false, quiet),
        source,
        start,
        stop,
        step,
    )
}

fn run_transient(
    engine: &Engine,
    netlist: &Netlist,
    tstop: f64,
    tstep: f64,
    tstart: f64,
    max_step: Option<f64>,
    args: &RunArgs,
    quiet: bool,
) -> Result<(), CliError> {
    basic::run_transient(
        &RunContext::new(engine, netlist, args, false, quiet),
        tstop,
        tstep,
        tstart,
        max_step,
    )
}

fn run_ac(
    engine: &Engine,
    netlist: &Netlist,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_ac(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        variation,
        points,
        start_freq,
        stop_freq,
    )
}

fn run_noise(
    engine: &Engine,
    netlist: &Netlist,
    output_node: &str,
    reference_node: Option<&str>,
    input_source: &str,
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_noise(
        &RunContext::without_args(engine, netlist, false, quiet),
        output_node,
        reference_node,
        input_source,
        variation,
        points,
        start_freq,
        stop_freq,
    )
}

fn run_step(
    engine: &Engine,
    netlist: &Netlist,
    step_cmd: &rspice_core::netlist::StepCommand,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_step(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        step_cmd,
    )
}


fn run_monte_carlo(
    engine: &Engine,
    netlist: &Netlist,
    num_runs: usize,
    seed: u64,
    distribution: rspice_core::analysis::Distribution,
    parameter_filter: Option<&[String]>,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_monte_carlo(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        num_runs,
        seed,
        distribution,
        parameter_filter,
    )
}

fn run_monte_carlo_from_command(
    engine: &Engine,
    netlist: &Netlist,
    mc_cmd: &rspice_core::netlist::MonteCarloCommand,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_monte_carlo_from_command(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        mc_cmd,
    )
}

fn run_pss(
    engine: &Engine,
    netlist: &Netlist,
    freq: f64,
    harmonics: usize,
    tstab: Option<f64>,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_pss(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        freq,
        harmonics,
        tstab,
    )
}

fn run_hb(
    engine: &Engine,
    netlist: &Netlist,
    freq: f64,
    harmonics: usize,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_hb(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        freq,
        harmonics,
    )
}

fn run_pz(
    engine: &Engine,
    netlist: &Netlist,
    input_node: usize,
    output_node: usize,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_pz(
        &RunContext::without_args(engine, netlist, verbose, quiet),
        input_node,
        output_node,
    )
}

fn run_pz_from_command(
    engine: &Engine,
    netlist: &Netlist,
    input_pos: &str,
    input_neg: &str,
    output_pos: &str,
    output_neg: &str,
    transfer_type: rspice_core::netlist::PoleZeroTransferType,
    analysis_type: rspice_core::netlist::PoleZeroAnalysisType,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_pz_from_command(
        &RunContext::without_args(engine, netlist, verbose, quiet),
        input_pos,
        input_neg,
        output_pos,
        output_neg,
        transfer_type,
        analysis_type,
    )
}

fn run_sensitivity(
    engine: &Engine,
    netlist: &Netlist,
    output_node: usize,
    param_name: &str,
    param_value: f64,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_sensitivity(
        &RunContext::without_args(engine, netlist, verbose, quiet),
        output_node,
        param_name,
        param_value,
    )
}

fn run_sensitivity_from_command(
    engine: &Engine,
    netlist: &Netlist,
    output_node: &str,
    reference_node: Option<&str>,
    ac_sweep: Option<rspice_core::netlist::SensitivityAcSweep>,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    frequency::run_sensitivity_from_command(
        &RunContext::without_args(engine, netlist, verbose, quiet),
        output_node,
        reference_node,
        ac_sweep,
    )
}

fn run_corner_sweep(
    engine: &Engine,
    netlist: &Netlist,
    corners_str: &str,
    args: &RunArgs,
    _config: &Config,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    advanced::run_corner_sweep(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        corners_str,
    )
}

fn run_fourier(
    engine: &Engine,
    netlist: &Netlist,
    fundamental: f64,
    outputs: &[String],
    num_harmonics: usize,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    basic::run_fourier(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        fundamental,
        outputs,
        num_harmonics,
    )
}

fn run_temp(
    engine: &Engine,
    netlist: &Netlist,
    temperatures: &[f64],
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    basic::run_temp(
        &RunContext::new(engine, netlist, args, verbose, quiet),
        temperatures,
    )
}
