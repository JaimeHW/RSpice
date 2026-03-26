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

#[cfg(test)]
fn run_analysis(
    engine: &Engine,
    netlist: &Netlist,
    analysis: &AnalysisCommand,
    args: &RunArgs,
    _config: &Config,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    RunContext::new(engine, netlist, args, verbose, quiet).run_analysis(analysis)
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

#[cfg(test)]
fn run_measurements(
    netlist: &Netlist,
    result: &rspice_core::engine::TransientResult,
    quiet: bool,
) -> Vec<crate::report::MeasurementReport> {
    basic::run_measurements(netlist, result, quiet)
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

#[cfg(test)]
use self::shared::{
    NodeResolver, generate_frequency_sweep, generate_step_values, parse_output_node,
    parse_voltage_probe_spec,
};
#[cfg(test)]
mod tests {
    use super::*;

    fn make_default_netlist() -> Netlist {
        Netlist::parse(
            r#"Test
.END
"#,
        )
        .expect("default test netlist should parse")
    }

    fn make_default_run_args() -> RunArgs {
        RunArgs {
            input: std::path::PathBuf::from("test.sp"),
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
        }
    }

    #[test]
    fn test_build_sim_config_defaults() {
        let args = make_default_run_args();
        let config = Config::default();
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.temperature, 300.15); // 27°C in K
        assert_eq!(sim_config.max_iterations, 50);
        assert_eq!(sim_config.tolerance, config.simulation.reltol);
        assert_eq!(
            sim_config.convergence_config.voltage_reltol,
            config.simulation.reltol
        );
        assert_eq!(
            sim_config.convergence_config.voltage_abstol,
            config.simulation.abstol
        );
        assert_eq!(
            sim_config.convergence_config.current_abstol,
            config.simulation.abstol
        );
        assert_eq!(
            sim_config.convergence_config.residual_reltol,
            config.simulation.residual_reltol
        );
        assert_eq!(sim_config.min_timestep, config.simulation.min_timestep);
        assert_eq!(sim_config.max_timestep, config.simulation.max_timestep);
    }

    #[test]
    fn test_build_sim_config_overrides() {
        let mut args = make_default_run_args();
        args.temp = Some(85.0);
        args.maxiter = Some(100);
        args.reltol = Some(5e-4);
        args.abstol = Some(1e-15);
        args.residual_reltol = Some(2e-4);
        let config = Config::default();
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.temperature, 358.15); // 85°C in K
        assert_eq!(sim_config.max_iterations, 100);
        assert_eq!(sim_config.tolerance, 5e-4);
        assert_eq!(sim_config.convergence_config.voltage_reltol, 5e-4);
        assert_eq!(sim_config.convergence_config.voltage_abstol, 1e-15);
        assert_eq!(sim_config.convergence_config.current_abstol, 1e-15);
        assert_eq!(sim_config.convergence_config.residual_reltol, 2e-4);
    }

    #[test]
    fn test_build_sim_config_abstol_override_keeps_reltol_target() {
        let mut args = make_default_run_args();
        args.abstol = Some(2e-14);
        let config = Config::default();
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.tolerance, config.simulation.reltol);
        assert_eq!(
            sim_config.convergence_config.voltage_reltol,
            config.simulation.reltol
        );
        assert_eq!(sim_config.convergence_config.voltage_abstol, 2e-14);
        assert_eq!(sim_config.convergence_config.current_abstol, 2e-14);
        assert_eq!(
            sim_config.convergence_config.residual_reltol,
            config.simulation.residual_reltol
        );
    }

    #[test]
    fn test_build_sim_config_fast_mode_preserves_explicit_tolerances() {
        let mut args = make_default_run_args();
        args.convergence = Some("fast".to_string());
        args.reltol = Some(8e-4);
        args.abstol = Some(4e-12);
        args.residual_reltol = Some(3e-4);
        let config = Config::default();
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert!(!sim_config.convergence_config.gmin_stepping);
        assert!(!sim_config.convergence_config.source_stepping);
        assert_eq!(sim_config.convergence_config.voltage_reltol, 8e-4);
        assert_eq!(sim_config.convergence_config.voltage_abstol, 4e-12);
        assert_eq!(sim_config.convergence_config.current_abstol, 4e-12);
        assert_eq!(sim_config.convergence_config.residual_reltol, 3e-4);
    }

    #[test]
    fn test_build_sim_config_residual_reltol_can_differ_from_voltage_reltol() {
        let mut args = make_default_run_args();
        args.reltol = Some(9e-4);
        args.residual_reltol = Some(5e-5);
        let config = Config::default();
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.convergence_config.voltage_reltol, 9e-4);
        assert_eq!(sim_config.convergence_config.residual_reltol, 5e-5);
    }

    #[test]
    fn test_build_sim_config_uses_config_residual_reltol_default() {
        let args = make_default_run_args();
        let mut config = Config::default();
        config.simulation.reltol = 1e-3;
        config.simulation.residual_reltol = 2e-4;
        let netlist = make_default_netlist();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.convergence_config.voltage_reltol, 1e-3);
        assert_eq!(sim_config.convergence_config.residual_reltol, 2e-4);
    }

    #[test]
    fn test_build_sim_config_uses_netlist_options_when_cli_unset() {
        let args = make_default_run_args();
        let config = Config::default();
        let netlist = Netlist::parse(
            r#"Test
.OPTIONS RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 RESIDUAL_RELTOL=5e-4
.END
"#,
        )
        .unwrap();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.convergence_config.voltage_reltol, 2e-4);
        assert_eq!(sim_config.convergence_config.voltage_abstol, 3e-6);
        assert_eq!(sim_config.convergence_config.current_abstol, 4e-12);
        assert_eq!(sim_config.convergence_config.residual_reltol, 5e-4);
    }

    #[test]
    fn test_build_sim_config_cli_overrides_netlist_options() {
        let mut args = make_default_run_args();
        args.reltol = Some(9e-4);
        args.abstol = Some(8e-13);
        args.residual_reltol = Some(7e-4);
        let config = Config::default();
        let netlist = Netlist::parse(
            r#"Test
.OPTIONS RELTOL=2e-4 VNTOL=3e-6 IABSTOL=4e-12 RESIDUAL_RELTOL=5e-4
.END
"#,
        )
        .unwrap();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.convergence_config.voltage_reltol, 9e-4);
        assert_eq!(sim_config.convergence_config.voltage_abstol, 8e-13);
        assert_eq!(sim_config.convergence_config.current_abstol, 8e-13);
        assert_eq!(sim_config.convergence_config.residual_reltol, 7e-4);
    }

    #[test]
    fn test_build_sim_config_netlist_reltol_backfills_residual_when_unspecified() {
        let args = make_default_run_args();
        let config = Config::default();
        let netlist = Netlist::parse(
            r#"Test
.OPTIONS RELTOL=6e-4
.END
"#,
        )
        .unwrap();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert_eq!(sim_config.convergence_config.voltage_reltol, 6e-4);
        assert_eq!(sim_config.convergence_config.residual_reltol, 6e-4);
    }

    #[test]
    fn test_build_sim_config_uses_netlist_temp_itl1_and_method() {
        let args = make_default_run_args();
        let config = Config::default();
        let netlist = Netlist::parse(
            r#"Test
.OPTIONS TEMP=85 ITL1=120 METHOD=GEAR
.END
"#,
        )
        .unwrap();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert!((sim_config.temperature - 358.15).abs() < 1e-12);
        assert_eq!(sim_config.max_iterations, 120);
        assert_eq!(
            sim_config.integration_method,
            rspice_core::analysis::IntegrationMethod::Gear2
        );
    }

    #[test]
    fn test_build_sim_config_cli_overrides_netlist_temp_and_itl1() {
        let mut args = make_default_run_args();
        args.temp = Some(27.0);
        args.maxiter = Some(90);
        let config = Config::default();
        let netlist = Netlist::parse(
            r#"Test
.OPTIONS TEMP=125 ITL1=120
.END
"#,
        )
        .unwrap();
        let sim_config = build_sim_config(&args, &config, &netlist);

        assert!((sim_config.temperature - 300.15).abs() < 1e-12);
        assert_eq!(sim_config.max_iterations, 90);
    }

    #[test]
    fn test_parse_output_node_voltage_syntax() {
        let resolver = NodeResolver {
            node_name_to_index: std::collections::HashMap::new(),
        };

        // V(3) should parse to node 3
        assert_eq!(parse_output_node("V(3)", &resolver), Some(3));

        // V(10) should parse to node 10
        assert_eq!(parse_output_node("V(10)", &resolver), Some(10));

        // Direct number should work
        assert_eq!(parse_output_node("5", &resolver), Some(5));
    }

    #[test]
    fn test_parse_output_node_named() {
        let mut node_map = std::collections::HashMap::new();
        node_map.insert("OUT".to_string(), 7);
        let resolver = NodeResolver {
            node_name_to_index: node_map,
        };

        // V(out) with named node resolves by netlist node map
        assert_eq!(parse_output_node("V(out)", &resolver), Some(7));

        // Invalid format returns None
        assert_eq!(parse_output_node("invalid", &resolver), None);
    }

    #[test]
    fn test_node_resolver_resolve_node() {
        let mut node_map = std::collections::HashMap::new();
        node_map.insert("IN".to_string(), 1);
        node_map.insert("OUT".to_string(), 2);
        let resolver = NodeResolver {
            node_name_to_index: node_map,
        };

        assert_eq!(resolver.resolve_node("in"), Some(1));
        assert_eq!(resolver.resolve_node("OUT"), Some(2));
        assert_eq!(resolver.resolve_node("0"), Some(0));
        assert_eq!(resolver.resolve_node("gnd"), Some(0));
        assert_eq!(resolver.resolve_node("3"), Some(3));
        assert_eq!(resolver.resolve_node("missing"), None);
    }

    #[test]
    fn test_parse_voltage_probe_spec() {
        assert_eq!(
            parse_voltage_probe_spec("V(out,ref)"),
            Some(("out".to_string(), Some("ref".to_string())))
        );
        assert_eq!(
            parse_voltage_probe_spec("V(3)"),
            Some(("3".to_string(), None))
        );
        assert_eq!(
            parse_voltage_probe_spec("out"),
            Some(("out".to_string(), None))
        );
        assert_eq!(parse_voltage_probe_spec(""), None);
    }

    #[test]
    fn test_run_measurements_supports_named_node_aliases() {
        let mut netlist = rspice_core::Netlist::default();
        netlist
            .measurements
            .push(rspice_core::analysis::MeasureStatement {
                name: "vout_final".to_string(),
                analysis: "TRAN".to_string(),
                measure_type: rspice_core::analysis::MeasureType::Find {
                    signal: "V(out)".to_string(),
                    at: Some(1.5e-3),
                    when_signal: None,
                    when_value: None,
                },
            });
        let result = rspice_core::engine::TransientResult {
            time: vec![0.0, 1e-3, 2e-3],
            voltages: vec![vec![5.0, 5.0, 5.0], vec![0.0, 2.5, 4.0]],
            branch_currents: vec![],
            num_nodes: 2,
            node_names: vec!["IN".to_string(), "OUT".to_string()],
            branch_names: vec![],
        };

        let reports = run_measurements(&netlist, &result, true);
        let measurement = reports
            .iter()
            .find(|report| report.name.eq_ignore_ascii_case("vout_final"))
            .expect("named-node measurement should be present");

        assert_eq!(measurement.value, Some(3.25));
        assert!(measurement.passed);
    }

    #[test]
    fn test_run_measurements_ignores_non_transient_statements() {
        let mut netlist = rspice_core::Netlist::default();
        netlist
            .measurements
            .push(rspice_core::analysis::MeasureStatement {
                name: "ac_only".to_string(),
                analysis: "AC".to_string(),
                measure_type: rspice_core::analysis::MeasureType::Find {
                    signal: "V(out)".to_string(),
                    at: Some(1.5e-3),
                    when_signal: None,
                    when_value: None,
                },
            });
        let result = rspice_core::engine::TransientResult {
            time: vec![0.0, 1e-3, 2e-3],
            voltages: vec![vec![5.0, 5.0, 5.0], vec![0.0, 2.5, 4.0]],
            branch_currents: vec![],
            num_nodes: 2,
            node_names: vec!["IN".to_string(), "OUT".to_string()],
            branch_names: vec![],
        };

        let reports = run_measurements(&netlist, &result, true);
        assert!(reports.is_empty());
    }

    #[test]
    fn test_generate_frequency_sweep_dec_and_lin() {
        let dec = generate_frequency_sweep(rspice_core::netlist::FreqVariation::Dec, 10, 1.0, 1e3);
        assert!(!dec.is_empty());
        assert!((dec.first().copied().unwrap_or(0.0) - 1.0).abs() < 1e-12);
        assert!((dec.last().copied().unwrap_or(0.0) - 1e3).abs() < 1e-6);

        let lin = generate_frequency_sweep(rspice_core::netlist::FreqVariation::Lin, 5, 10.0, 50.0);
        assert_eq!(lin.len(), 5);
        assert!((lin[0] - 10.0).abs() < 1e-12);
        assert!((lin[4] - 50.0).abs() < 1e-12);
    }

    #[test]
    fn test_generate_step_values_linear_descending() {
        let sweep = rspice_core::netlist::StepSweep::Linear {
            start: 10.0,
            stop: 2.0,
            step: -2.0,
        };
        let values = generate_step_values(&sweep).expect("descending linear sweep should work");
        assert_eq!(values, vec![10.0, 8.0, 6.0, 4.0, 2.0]);
    }

    #[test]
    fn test_generate_step_values_linear_rejects_inconsistent_step_sign() {
        let sweep = rspice_core::netlist::StepSweep::Linear {
            start: 0.0,
            stop: 10.0,
            step: -1.0,
        };
        let err = generate_step_values(&sweep).expect_err("invalid step sign should fail");
        assert!(
            err.to_string()
                .contains("step sign is inconsistent with start/stop range")
        );
    }

    #[test]
    fn test_generate_step_values_octave_descending() {
        let sweep = rspice_core::netlist::StepSweep::Octave {
            points_per_octave: 1,
            start: 8.0,
            stop: 1.0,
        };
        let values = generate_step_values(&sweep).expect("descending octave sweep should work");
        assert_eq!(values, vec![8.0, 4.0, 2.0, 1.0]);
    }

    #[test]
    fn test_run_analysis_parsed_pz_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* RC\nV1 in 0 1\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::PoleZero {
            input_pos: "in".to_string(),
            input_neg: "0".to_string(),
            output_pos: "out".to_string(),
            output_neg: "0".to_string(),
            transfer_type: rspice_core::netlist::PoleZeroTransferType::Voltage,
            analysis_type: rspice_core::netlist::PoleZeroAnalysisType::PoleZero,
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .PZ analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_sens_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* SENS\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.SENS V(out)\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Sensitivity {
            output_node: "out".to_string(),
            reference_node: None,
            ac_sweep: None,
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .SENS analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_noise_differential_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* Differential NOISE\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Noise {
            output_node: "out".to_string(),
            reference_node: Some("in".to_string()),
            input_source: "V1".to_string(),
            variation: rspice_core::netlist::FreqVariation::Dec,
            points: 5,
            start_freq: 1.0,
            stop_freq: 1e3,
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed differential .NOISE analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_disto_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* DISTO\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Disto {
            variation: rspice_core::netlist::FreqVariation::Dec,
            points: 5,
            start_freq: 1.0,
            stop_freq: 1e3,
            f2_over_f1: Some(1.5),
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .DISTO analysis should run");
    }

    #[test]
    fn test_run_analysis_disto_invalid_f2_ratio_errors() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* DISTO invalid\nV1 in 0 AC 1\nR1 in out 1k\nC1 out 0 1n\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Disto {
            variation: rspice_core::netlist::FreqVariation::Dec,
            points: 5,
            start_freq: 1.0,
            stop_freq: 1e3,
            f2_over_f1: Some(1.0),
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        let err = run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect_err("invalid f2_over_f1 should fail");
        assert!(err.to_string().contains("f2_over_f1"));
    }

    #[test]
    fn test_run_analysis_noise_invalid_input_source_errors() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* NOISE invalid source\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Noise {
            output_node: "out".to_string(),
            reference_node: None,
            input_source: "VMISSING".to_string(),
            variation: rspice_core::netlist::FreqVariation::Dec,
            points: 5,
            start_freq: 1.0,
            stop_freq: 1e3,
        };
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        let err = run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect_err("missing .NOISE input source should fail");
        assert!(err.to_string().contains("Invalid .NOISE input source"));
    }

    #[test]
    fn test_run_analysis_parsed_monte_carlo_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* MC\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 32 DIST UNIFORM SPREAD 0.02 PARAMS RVAL\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::MonteCarlo(rspice_core::netlist::MonteCarloCommand {
            runs: 32,
            seed: Some(9),
            distribution: rspice_core::netlist::MonteCarloDistribution::Uniform,
            relative_spread: 0.02,
            params: vec!["RVAL".to_string()],
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .MC analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_monte_carlo_worst_case_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* MC worstcase\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.MC 16 DIST WORSTCASE SPREAD 0.03 PARAMS RVAL\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::MonteCarlo(rspice_core::netlist::MonteCarloCommand {
            runs: 16,
            seed: Some(11),
            distribution: rspice_core::netlist::MonteCarloDistribution::WorstCase,
            relative_spread: 0.03,
            params: vec!["RVAL".to_string()],
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed worst-case .MC analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_step_param_command() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* STEP\n.PARAM RVAL=1k\nV1 in 0 1\nR1 in out {RVAL}\nR2 out 0 1k\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Step(rspice_core::netlist::StepCommand {
            target: rspice_core::netlist::StepTarget::Param,
            name: "RVAL".to_string(),
            param_name: None,
            sweep: rspice_core::netlist::StepSweep::List(vec![1e3, 2e3, 4e3]),
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .STEP PARAM analysis should run");
    }

    #[test]
    fn test_run_analysis_parsed_step_temp_command() {
        let netlist =
            rspice_core::netlist::parse_netlist("* STEP TEMP\nV1 in 0 1\nR1 in 0 1k\n.end\n")
                .expect("netlist should parse");
        let analysis = AnalysisCommand::Step(rspice_core::netlist::StepCommand {
            target: rspice_core::netlist::StepTarget::Temp,
            name: "TEMP".to_string(),
            param_name: None,
            sweep: rspice_core::netlist::StepSweep::List(vec![-40.0, 27.0, 125.0]),
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect("parsed .STEP TEMP analysis should run");
    }

    #[test]
    fn test_run_analysis_step_device_target_runs() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* STEP device target\nV1 in 0 1\nR1 in out 1k\nR2 out 0 1k\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Step(rspice_core::netlist::StepCommand {
            target: rspice_core::netlist::StepTarget::Device,
            name: "R1".to_string(),
            param_name: Some("VALUE".to_string()),
            sweep: rspice_core::netlist::StepSweep::List(vec![500.0, 1000.0]),
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect(".STEP DEVICE target should run");
    }

    #[test]
    fn test_run_analysis_step_model_target_runs() {
        let netlist = rspice_core::netlist::parse_netlist(
            "* STEP model target\nV1 in 0 1\nR1 in out 1k\nD1 out 0 DMOD\n.MODEL DMOD D (IS=1e-12 N=1)\n.end\n",
        )
        .expect("netlist should parse");
        let analysis = AnalysisCommand::Step(rspice_core::netlist::StepCommand {
            target: rspice_core::netlist::StepTarget::Model,
            name: "DMOD".to_string(),
            param_name: Some("IS".to_string()),
            sweep: rspice_core::netlist::StepSweep::List(vec![1e-12, 1e-8]),
        });
        let engine = Engine::default();
        let args = make_default_run_args();
        let config = Config::default();

        run_analysis(&engine, &netlist, &analysis, &args, &config, false, true)
            .expect(".STEP MODEL target should run");
    }
}
