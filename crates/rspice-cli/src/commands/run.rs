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
mod export;
mod frequency;
mod shared;

use crate::report::{
    CsvMeasReporter, JUnitReporter, JsonMeasReporter, MeasurementReport, SimulationReport,
    TapReporter,
};

use crate::cli::{CliError, Config, MeasFormat, RunArgs};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergenceConfig, ConvergencePreset, Engine, Netlist, SimulationConfig,
    SimulationConfigOverrides, resolve_simulation_config,
};
use std::path::Path;
use std::time::Instant;

struct RunContext<'a> {
    engine: &'a Engine,
    netlist: &'a Netlist,
    args: &'a RunArgs,
    verbose: bool,
    quiet: bool,
    /// .MEAS results collected while analyses run, for CI/CD reporting
    /// (`--report-file` / `--meas-file`).
    measurements: std::cell::RefCell<Vec<MeasurementReport>>,
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
            measurements: std::cell::RefCell::new(Vec::new()),
        }
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
    let measurements = ctx.measurements.borrow().clone();

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
        measurements,
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
        match args.meas_format {
            Some(MeasFormat::Csv) => CsvMeasReporter::write(&reports, meas_file)?,
            Some(MeasFormat::Json) | None => JsonMeasReporter::write(&reports, meas_file)?,
        }
        if verbose {
            println!("Measurement report written to: {}", meas_file.display());
        }
    }

    if !quiet {
        println!("\nSimulation complete in {:.3}s.", duration);
    }

    if let Some(err_msg) = simulation_error {
        return Err(CliError::simulation_error(err_msg));
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
