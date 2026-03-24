//! Run Command - Execute SPICE simulations
//!
//! This is the main simulation execution command, supporting:
//! - All analysis types (DC, AC, Transient, Noise, etc.)
//! - Multiple output formats
//! - Progress reporting
//! - Waveform compression

#![allow(clippy::too_many_arguments)]

use crate::cli::{CliError, Config, OutputFormat, RunArgs};
use crate::report::{JUnitReporter, JsonMeasReporter, SimulationReport, TapReporter};
use indicatif::{ProgressBar, ProgressStyle};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{
    ConvergenceConfig, ConvergencePreset, Engine, Netlist, SimulationConfig,
    SimulationConfigOverrides,
    resolve_simulation_config,
};
use std::collections::HashMap;
use std::path::Path;
use std::time::Instant;

/// Execute the run command
pub fn execute(args: RunArgs, config: &Config, verbose: bool, quiet: bool) -> Result<(), CliError> {
    // Validate input file exists
    if !args.input.exists() {
        return Err(CliError::InputNotFound {
            path: args.input.clone(),
            source: std::io::Error::new(std::io::ErrorKind::NotFound, "File not found"),
        });
    }

    // Parse netlist
    log::info!("Loading netlist: {}", args.input.display());
    let netlist = load_netlist(&args.input)?;

    if verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    // Build simulation configuration
    let sim_config = build_sim_config(&args, config, &netlist);
    let engine = Engine::new(sim_config);

    // Monte Carlo mode
    if let Some(num_runs) = args.monte_carlo {
        return run_monte_carlo(
            &engine,
            &netlist,
            num_runs,
            args.seed.unwrap_or(1),
            rspice_core::analysis::Distribution::Gaussian { sigma: 0.01 },
            None,
            &args,
            verbose,
            quiet,
        );
    }

    // PSS (Periodic Steady-State) mode
    if let Some(freq) = args.pss_freq {
        return run_pss(
            &engine,
            &netlist,
            freq,
            args.pss_harmonics,
            args.pss_tstab,
            &args,
            verbose,
            quiet,
        );
    }

    // HB (Harmonic Balance) mode
    if let Some(freq) = args.hb_freq {
        return run_hb(
            &engine,
            &netlist,
            freq,
            args.hb_harmonics,
            &args,
            verbose,
            quiet,
        );
    }

    // PZ (Pole-Zero) mode
    if let (Some(input), Some(output)) = (args.pz_input, args.pz_output) {
        return run_pz(&engine, &netlist, input, output, verbose, quiet);
    }

    // Sensitivity mode
    if let (Some(output_node), Some(param)) = (args.sens_output, args.sens_param.as_ref()) {
        let value = args.sens_value.unwrap_or(1.0);
        return run_sensitivity(&engine, &netlist, output_node, param, value, verbose, quiet);
    }

    // Corner iteration mode
    if let Some(ref corners_str) = args.corners {
        return run_corner_sweep(
            &engine,
            &netlist,
            corners_str,
            &args,
            config,
            verbose,
            quiet,
        );
    }

    // Track if any analysis was run
    let mut ran_analysis = false;
    let start_time = Instant::now();
    let mut simulation_error: Option<String> = None;

    // Process all analysis commands
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
        if let Err(e) = run_analysis(&engine, &netlist, analysis, &args, config, verbose, quiet) {
            simulation_error = Some(e.to_string());
            break;
        }
    }

    // Default to DC operating point if no analyses specified
    if !ran_analysis && simulation_error.is_none() {
        if !quiet {
            println!("No analysis commands - running default DC OP...");
        }
        if let Err(e) = run_dc_op(&engine, &netlist, &args, quiet) {
            simulation_error = Some(e.to_string());
        }
    }

    let duration = start_time.elapsed().as_secs_f64();
    let passed = simulation_error.is_none();

    // Build simulation report
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
        measurements: Vec::new(), // Reserved for future measurement collection
    };

    // Write JUnit report if requested
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

    // Write measurement report if requested
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

    // Return error if simulation failed
    if let Some(err_msg) = simulation_error {
        return Err(CliError::SimulationError {
            message: err_msg,
            analysis: None,
        });
    }

    Ok(())
}

/// Load and validate netlist file
fn load_netlist(path: &Path) -> Result<Netlist, CliError> {
    Netlist::parse_file(path).map_err(|e| CliError::ParseError {
        message: e.to_string(),
        line: None,
        suggestion: None,
    })
}

/// Build simulation configuration from args and config
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

/// Run a single analysis
fn run_analysis(
    engine: &Engine,
    netlist: &Netlist,
    analysis: &AnalysisCommand,
    args: &RunArgs,
    _config: &Config,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    match analysis {
        AnalysisCommand::Op => {
            run_dc_op(engine, netlist, args, quiet)?;
        }
        AnalysisCommand::Dc {
            source,
            start,
            stop,
            step,
        } => {
            run_dc_sweep(engine, netlist, source, *start, *stop, *step, args, quiet)?;
        }
        AnalysisCommand::Tran {
            step,
            stop,
            start,
            max_step,
        } => {
            let tstart = start.unwrap_or(0.0);
            run_transient(
                engine, netlist, *stop, *step, tstart, *max_step, args, quiet,
            )?;
        }
        AnalysisCommand::Ac {
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            run_ac(
                engine,
                netlist,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                args,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::Disto {
            variation,
            points,
            start_freq,
            stop_freq,
            f2_over_f1,
        } => {
            run_disto(
                engine,
                netlist,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                *f2_over_f1,
                args,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::Noise {
            output_node,
            reference_node,
            input_source,
            variation,
            points,
            start_freq,
            stop_freq,
        } => {
            run_noise(
                engine,
                netlist,
                output_node,
                reference_node.as_deref(),
                input_source,
                *variation,
                *points,
                *start_freq,
                *stop_freq,
                quiet,
            )?;
        }
        AnalysisCommand::Sensitivity {
            output_node,
            reference_node,
            ac_sweep,
        } => {
            run_sensitivity_from_command(
                engine,
                netlist,
                output_node,
                reference_node.as_deref(),
                *ac_sweep,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::PoleZero {
            input_pos,
            input_neg,
            output_pos,
            output_neg,
            transfer_type,
            analysis_type,
        } => {
            run_pz_from_command(
                engine,
                netlist,
                input_pos,
                input_neg,
                output_pos,
                output_neg,
                *transfer_type,
                *analysis_type,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::Step(step_cmd) => {
            run_step(engine, netlist, step_cmd, args, verbose, quiet)?;
        }
        AnalysisCommand::Four {
            fundamental,
            outputs,
            num_harmonics,
        } => {
            run_fourier(
                engine,
                netlist,
                *fundamental,
                outputs,
                *num_harmonics,
                args,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::Temp { temperatures } => {
            run_temp(engine, netlist, temperatures, args, verbose, quiet)?;
        }
        AnalysisCommand::MonteCarlo(mc_cmd) => {
            run_monte_carlo_from_command(engine, netlist, mc_cmd, args, verbose, quiet)?;
        }
    }
    Ok(())
}

/// Run distortion analysis.
///
/// The CLI currently executes a linearized frequency sweep for .DISTO inputs.
/// Detailed harmonic/intermodulation metrics are available through the UI flow.
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
    if let Some(ratio) = f2_over_f1 {
        if !ratio.is_finite() || ratio <= 1.0 {
            return Err(CliError::simulation_error_in(
                format!(
                    "Invalid .DISTO f2_over_f1 ratio '{}': expected a finite value > 1",
                    ratio
                ),
                "DISTO",
            ));
        }
    }

    if verbose && !quiet {
        match f2_over_f1 {
            Some(ratio) => println!(
                "DISTO note: using linearized AC sweep in CLI (f2/f1={:.6}); full IMD metrics are available in rspice-ui",
                ratio
            ),
            None => println!(
                "DISTO note: using linearized AC sweep in CLI; full harmonic/IMD metrics are available in rspice-ui"
            ),
        }
    }

    run_ac(
        engine, netlist, variation, points, start_freq, stop_freq, args, verbose, quiet,
    )
}

/// Run DC operating point analysis
fn run_dc_op(
    engine: &Engine,
    netlist: &Netlist,
    args: &RunArgs,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!("Running DC operating point...");
    }

    match engine.run_dc_op(netlist) {
        Ok(result) => {
            if !quiet {
                println!("DC Operating Point:");
                for i in 1..=result.node_voltages.len().min(10) {
                    println!("  V({}) = {:.6} V", i, result.voltage(i));
                }
                if result.node_voltages.len() > 10 {
                    println!("  ... ({} more nodes)", result.node_voltages.len() - 10);
                }
            }

            // Write output file if requested
            if let Some(ref output_path) = args.output {
                write_dc_op_output(output_path, &result, args)?;
                if !quiet {
                    println!("Results exported to: {}", output_path.display());
                }
            }

            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC OP")),
    }
}

/// Write DC operating point results to file
fn write_dc_op_output(
    path: &std::path::Path,
    result: &rspice_core::solver::SimulationResult,
    args: &RunArgs,
) -> Result<(), CliError> {
    use std::io::Write;

    let mut file = std::fs::File::create(path).map_err(|e| CliError::OutputError {
        path: path.to_path_buf(),
        source: e,
    })?;

    match args.format {
        OutputFormat::Json => {
            let mut vars = serde_json::Map::new();
            for i in 1..=result.node_voltages.len() {
                vars.insert(format!("V({})", i), serde_json::json!(result.voltage(i)));
            }
            let json = serde_json::json!({
                "analysis": "dc_op",
                "variables": vars,
            });
            writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(|e| {
                CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                }
            })?;
        }
        OutputFormat::Csv => {
            writeln!(file, "node,voltage").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for i in 1..=result.node_voltages.len() {
                writeln!(file, "V({}),{:.9e}", i, result.voltage(i)).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
        }
        OutputFormat::Tsv => {
            writeln!(file, "node\tvoltage").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for i in 1..=result.node_voltages.len() {
                writeln!(file, "V({})\t{:.9e}", i, result.voltage(i)).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
        }
        OutputFormat::Raw | OutputFormat::RawAscii => {
            writeln!(file, "Title: DC Operating Point").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Plotname: DC OP").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Flags: real").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "No. Variables: {}", result.node_voltages.len()).map_err(|e| {
                CliError::OutputError {
                    path: path.to_path_buf(),
                    source: e,
                }
            })?;
            writeln!(file, "No. Points: 1").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "Variables:").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for i in 1..=result.node_voltages.len() {
                writeln!(file, "\t{}\tV({})\tvoltage", i - 1, i).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
            writeln!(file, "Values:").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            writeln!(file, "0").map_err(|e| CliError::OutputError {
                path: path.to_path_buf(),
                source: e,
            })?;
            for i in 1..=result.node_voltages.len() {
                writeln!(file, "\t{:.9e}", result.voltage(i)).map_err(|e| {
                    CliError::OutputError {
                        path: path.to_path_buf(),
                        source: e,
                    }
                })?;
            }
        }
        OutputFormat::Hdf5 => {
            // HDF5 requires feature flag
            return Err(CliError::SimulationError {
                message: "HDF5 output requires --features hdf5".to_string(),
                analysis: Some("DC OP".to_string()),
            });
        }
    }

    Ok(())
}

/// Run DC sweep analysis
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
    if !quiet {
        println!(
            "Running DC sweep on {} from {} to {} by {}...",
            source, start, stop, step
        );
    }

    match engine.run_dc_sweep(netlist, source, start, stop, step) {
        Ok(results) => {
            if !quiet {
                println!("DC Sweep: {} points computed", results.len());
            }

            // Export if output path specified
            if let Some(ref output_path) = args.output {
                let sweep_vals: Vec<f64> = results.iter().map(|(v, _)| *v).collect();
                let mut node_waveforms: Vec<Vec<f64>> = Vec::new();
                if !results.is_empty() {
                    let num_nodes = results[0].1.node_voltages.len();
                    for node in 0..num_nodes.min(5) {
                        let waveform: Vec<f64> =
                            results.iter().map(|(_, r)| r.voltage(node + 1)).collect();
                        node_waveforms.push(waveform);
                    }
                }
                let node_names: Vec<String> =
                    (1..=node_waveforms.len()).map(|i| i.to_string()).collect();
                let format = match args.format {
                    OutputFormat::RawAscii => rspice_core::analysis::RawFormat::Ascii,
                    _ => rspice_core::analysis::RawFormat::Binary,
                };
                rspice_core::analysis::export_dc_sweep(
                    output_path,
                    &sweep_vals,
                    source,
                    &node_names,
                    &node_waveforms,
                    format,
                )
                .map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

                if !quiet {
                    println!("Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "DC Sweep")),
    }
}

/// Run transient analysis
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
    // .tran step is the output interval. Internal solver max step can be
    // explicitly set by .tran tmax, otherwise use a practical adaptive default.
    let internal_max_step = max_step.unwrap_or(tstep * 10.0).max(1e-18);

    // Progress indicator
    let pb = if quiet {
        ProgressBar::hidden()
    } else if args.progress {
        let eta_steps = ((tstop - tstart) / tstep) as u64;
        let pb = ProgressBar::new(eta_steps);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message(format!("Transient: {} to {}", tstart, tstop));
        pb
    } else {
        let pb = ProgressBar::new_spinner();
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap(),
        );
        pb.set_message(format!(
            "Running transient: {} to {} (step {})...",
            tstart, tstop, tstep
        ));
        pb.enable_steady_tick(std::time::Duration::from_millis(100));
        pb
    };

    // Use compressed transient if --compress flag is set
    let result = if args.compress {
        let compression_tol = args.compress_tol.unwrap_or(1e-4);
        let compression = rspice_core::engine::CompressionConfig {
            enabled: true,
            abs_tol: compression_tol,
            rel_tol: compression_tol,
            min_interval: tstep / 10.0,
        };
        match engine.run_tran_compressed(netlist, tstop, internal_max_step, compression) {
            Ok(compressed) => {
                pb.finish_and_clear();
                if !quiet {
                    println!(
                        "✓ Transient complete (compressed): {} points (compression ratio: {:.1}x)",
                        compressed.time.len(),
                        (tstop / tstep) / compressed.time.len() as f64
                    );
                }
                // Convert to regular result for further processing
                let result: rspice_core::engine::TransientResult = compressed.into();
                Ok(result)
            }
            Err(e) => Err(e),
        }
    } else {
        pb.finish_and_clear();
        engine.run_tran(netlist, tstop, internal_max_step)
    };

    match result {
        Ok(result) => {
            if !quiet && !args.compress {
                println!(
                    "✓ Transient complete: {} time points computed",
                    result.time.len()
                );
            }

            // Process .MEAS commands if --meas flag is set
            if args.meas {
                run_measurements(netlist, &result, quiet);
            }

            // Export if output path specified
            if let Some(ref output_path) = args.output {
                let node_names: Vec<String> =
                    (1..=result.voltages.len()).map(|i| i.to_string()).collect();
                let format = match args.format {
                    OutputFormat::RawAscii => rspice_core::analysis::RawFormat::Ascii,
                    _ => rspice_core::analysis::RawFormat::Binary,
                };
                rspice_core::analysis::export_transient(
                    output_path,
                    &result.time,
                    &node_names,
                    &result.voltages,
                    format,
                )
                .map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

                if !quiet {
                    println!("  Results exported to: {}", output_path.display());
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Transient")),
    }
}

/// Run AC analysis
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
    if !quiet {
        println!(
            "Running AC analysis: {} to {} Hz ({} points)...",
            start_freq, stop_freq, points
        );
    }

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq);

    match engine.run_ac(netlist, &frequencies) {
        Ok(results) => {
            if !quiet {
                println!("AC Analysis: {} frequency points", results.len());
                if verbose && !results.is_empty() {
                    let first = &results[0];
                    let last = results.last().unwrap();
                    println!(
                        "  @ {:e} Hz: |V(1)| = {:.4}",
                        first.frequency,
                        first.voltage_magnitude(1)
                    );
                    println!(
                        "  @ {:e} Hz: |V(1)| = {:.4}",
                        last.frequency,
                        last.voltage_magnitude(1)
                    );
                }
            }

            // CSV export for Bode plot
            if let Some(ref output_path) = args.output {
                if matches!(args.format, OutputFormat::Csv) {
                    use std::io::Write;
                    let mut file =
                        std::fs::File::create(output_path).map_err(|e| CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        })?;
                    writeln!(file, "Frequency_Hz,Magnitude_dB,Phase_deg").map_err(|e| {
                        CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        }
                    })?;
                    for r in &results {
                        let mag_db = 20.0 * r.voltage_magnitude(1).log10();
                        let phase_deg = r.voltage_phase(1) * 180.0 / std::f64::consts::PI;
                        writeln!(file, "{:e},{:.4},{:.2}", r.frequency, mag_db, phase_deg)
                            .map_err(|e| CliError::OutputError {
                                path: output_path.clone(),
                                source: e,
                            })?;
                    }
                    if !quiet {
                        println!("  Bode plot exported to: {}", output_path.display());
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "AC")),
    }
}

/// Run noise analysis
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
    if !quiet {
        println!(
            "Running Noise analysis on {} from {} to {} Hz ({} points)...",
            output_node, start_freq, stop_freq, points
        );
    }

    let resolver = NodeResolver::from_netlist(engine, netlist)?;
    let output = resolver
        .resolve_node(output_node)
        .ok_or_else(|| CliError::SimulationError {
            message: format!("Invalid .NOISE output node '{}'", output_node),
            analysis: Some("Noise".to_string()),
        })?;
    let output_neg = match reference_node {
        Some(reference) => {
            Some(
                resolver
                    .resolve_node(reference)
                    .ok_or_else(|| CliError::SimulationError {
                        message: format!("Invalid .NOISE reference node '{}'", reference),
                        analysis: Some("Noise".to_string()),
                    })?,
            )
        }
        None => None,
    };

    let input_source_exists = netlist.elements.iter().any(|element| {
        element.name.eq_ignore_ascii_case(input_source)
            && matches!(
                element.kind,
                rspice_core::netlist::ElementKind::VoltageSource(_)
                    | rspice_core::netlist::ElementKind::CurrentSource(_)
            )
    });
    if !input_source_exists {
        return Err(CliError::SimulationError {
            message: format!(
                "Invalid .NOISE input source '{}': expected an independent V or I source name",
                input_source
            ),
            analysis: Some("Noise".to_string()),
        });
    }

    let frequencies = generate_frequency_sweep(variation, points, start_freq, stop_freq);
    match engine.run_noise_with_input_source(
        netlist,
        output,
        output_neg,
        input_source,
        &frequencies,
        engine.config().temperature,
    ) {
        Ok(results) => {
            if !quiet {
                println!("Noise Analysis: {} frequency points", results.len());
                if let Some(reference) = reference_node {
                    println!("  Output node: V({},{})", output_node, reference);
                } else {
                    println!("  Output node: V({})", output_node);
                }
                println!("  Input source: {}", input_source);
                if let (Some(first), Some(last)) = (results.first(), results.last()) {
                    println!(
                        "  @ {:e} Hz: output_noise={:.6e} V^2/Hz",
                        first.frequency, first.output_noise_density
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e}",
                        first.frequency, first.input_referred_density
                    );
                    println!(
                        "  @ {:e} Hz: output_noise={:.6e} V^2/Hz",
                        last.frequency, last.output_noise_density
                    );
                    println!(
                        "  @ {:e} Hz: input_referred={:.6e}",
                        last.frequency, last.input_referred_density
                    );
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Noise")),
    }
}

/// Run parametric step analysis
fn run_step(
    engine: &Engine,
    netlist: &Netlist,
    step_cmd: &rspice_core::netlist::StepCommand,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    use rspice_core::netlist::StepTarget;

    let values = generate_step_values(&step_cmd.sweep)?;
    if values.is_empty() {
        return Err(CliError::SimulationError {
            message: ".STEP produced no sweep values".to_string(),
            analysis: Some("Step".to_string()),
        });
    }

    match step_cmd.target {
        StepTarget::Param | StepTarget::Device | StepTarget::Model => {
            let target_desc = match step_cmd.target {
                StepTarget::Param => format!("PARAM {}", step_cmd.name),
                StepTarget::Device => {
                    if let Some(param) = &step_cmd.param_name {
                        format!("DEVICE {}.{}", step_cmd.name, param)
                    } else {
                        format!("DEVICE {}", step_cmd.name)
                    }
                }
                StepTarget::Model => {
                    if let Some(param) = &step_cmd.param_name {
                        format!("MODEL {}.{}", step_cmd.name, param)
                    } else {
                        format!("MODEL {}", step_cmd.name)
                    }
                }
                StepTarget::Temp => unreachable!("handled separately"),
            };

            if !quiet {
                println!(
                    "Running .STEP sweep on {}: {} values ({:.3e} to {:.3e})...",
                    target_desc,
                    values.len(),
                    values.first().unwrap_or(&0.0),
                    values.last().unwrap_or(&0.0)
                );
            }

            let sweep_results = engine
                .run_step_command(netlist, step_cmd, &values)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Step"))?;

            for (i, (value, result)) in sweep_results.iter().enumerate() {
                if verbose && !quiet {
                    println!(
                        "  Step {}/{}: {} = {:.4e}",
                        i + 1,
                        values.len(),
                        target_desc,
                        value
                    );
                    println!("    V(1) = {:.6} V", result.voltage(1));
                }
            }

            if !quiet {
                println!(
                    ".STEP sweep complete: {} converged / {} requested",
                    sweep_results.len(),
                    values.len()
                );
            }
            Ok(())
        }
        StepTarget::Temp => run_temp(engine, netlist, &values, args, verbose, quiet),
    }
}

/// Process .MEAS commands and return measurement reports
fn run_measurements(
    netlist: &Netlist,
    result: &rspice_core::engine::TransientResult,
    quiet: bool,
) -> Vec<crate::report::MeasurementReport> {
    use crate::report::MeasurementReport;

    // Build signals map from simulation results
    let mut signals: HashMap<String, &[f64]> = HashMap::new();
    for (i, waveform) in result.voltages.iter().enumerate() {
        let name = format!("V({})", i + 1);
        signals.insert(name.clone(), waveform.as_slice());
        signals.insert(name.to_lowercase(), waveform.as_slice());
    }

    let mut meas_engine = rspice_core::MeasureEngine::new();
    for meas in &netlist.measurements {
        meas_engine.add(meas.clone());
    }
    let meas_results = meas_engine.evaluate(&result.time, &signals);

    let mut reports = Vec::new();

    if !quiet {
        if meas_results.is_empty() && netlist.measurements.is_empty() {
            println!("  No .MEAS statements found in netlist");
        } else {
            println!("  Measurement Results ({}):", meas_results.len());
            for mr in &meas_results {
                if let Some(value) = mr.value {
                    println!("    {} = {:.6}", mr.name, value);
                } else {
                    println!(
                        "    {} = FAILED ({})",
                        mr.name,
                        mr.error.as_ref().unwrap_or(&String::new())
                    );
                }
            }
        }
    }

    // Convert to MeasurementReport
    for mr in meas_results {
        reports.push(MeasurementReport {
            name: mr.name,
            value: mr.value,
            expected: None,
            tolerance: None,
            passed: mr.value.is_some(),
            error: mr.error,
        });
    }

    reports
}

/// Run Monte Carlo statistical analysis
fn run_monte_carlo(
    engine: &Engine,
    netlist: &Netlist,
    num_runs: usize,
    seed: u64,
    distribution: rspice_core::analysis::Distribution,
    parameter_filter: Option<&[String]>,
    _args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!(
            "Running Monte Carlo analysis: {} iterations (seed={})",
            num_runs, seed
        );
    }

    // Progress bar for Monte Carlo
    let pb = if quiet {
        indicatif::ProgressBar::hidden()
    } else {
        let pb = indicatif::ProgressBar::new(num_runs as u64);
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );
        pb.set_message("Monte Carlo");
        pb
    };

    match engine.run_monte_carlo_with_options(
        netlist,
        num_runs,
        seed,
        distribution,
        parameter_filter,
    ) {
        Ok(result) => {
            pb.finish_and_clear();

            if !quiet {
                println!("✓ Monte Carlo complete: {} runs", result.num_runs);
                println!(
                    "  Convergence: {}/{} runs succeeded",
                    result.num_runs, num_runs
                );

                if verbose && !result.variables.is_empty() {
                    println!("\n  Statistical Summary:");
                    for (name, stats) in &result.variables {
                        println!("    {}:", name);
                        println!("      Mean:   {:.6}", stats.mean);
                        println!("      Std:    {:.6}", stats.std_dev);
                        println!("      Min:    {:.6}", stats.min);
                        println!("      Max:    {:.6}", stats.max);
                    }
                }
            }
            Ok(())
        }
        Err(e) => {
            pb.finish_and_clear();
            Err(CliError::simulation_error_in(e.to_string(), "Monte Carlo"))
        }
    }
}

fn run_monte_carlo_from_command(
    engine: &Engine,
    netlist: &Netlist,
    mc_cmd: &rspice_core::netlist::MonteCarloCommand,
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    let seed = args.seed.or(mc_cmd.seed).unwrap_or(1);
    let distribution = match mc_cmd.distribution {
        rspice_core::netlist::MonteCarloDistribution::Gaussian => {
            rspice_core::analysis::Distribution::Gaussian {
                sigma: mc_cmd.relative_spread,
            }
        }
        rspice_core::netlist::MonteCarloDistribution::Uniform => {
            rspice_core::analysis::Distribution::Uniform {
                tolerance: mc_cmd.relative_spread,
            }
        }
        rspice_core::netlist::MonteCarloDistribution::WorstCase => {
            rspice_core::analysis::Distribution::WorstCase {
                tolerance: mc_cmd.relative_spread,
            }
        }
    };
    let parameter_filter = if mc_cmd.params.is_empty() {
        None
    } else {
        Some(mc_cmd.params.as_slice())
    };

    run_monte_carlo(
        engine,
        netlist,
        mc_cmd.runs,
        seed,
        distribution,
        parameter_filter,
        args,
        verbose,
        quiet,
    )
}

/// Run PSS (Periodic Steady-State) analysis
fn run_pss(
    engine: &Engine,
    netlist: &Netlist,
    freq: f64,
    harmonics: usize,
    tstab: Option<f64>,
    _args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!(
            "Running PSS analysis: f₀ = {:.3e} Hz, {} harmonics",
            freq, harmonics
        );
    }

    // Build PSS configuration
    let mut config = rspice_core::analysis::PssConfig::new(freq);
    config.num_harmonics = harmonics;
    if let Some(t) = tstab {
        config.tstab = t;
    }

    match engine.run_pss(netlist, config) {
        Ok(pss_result) => {
            if !quiet {
                println!("✓ PSS converged in {} iterations", pss_result.iterations);
                println!("  Period: {:.6e} s", pss_result.period);
                println!("  Nodes: {}", pss_result.result.num_nodes());

                if verbose && pss_result.result.num_nodes() > 0 {
                    println!("\n  Harmonic content (node 1):");
                    let harm_data = pss_result.result.harmonics(1, 5);
                    for h in &harm_data {
                        println!(
                            "    H{}: mag={:.6e}, phase={:.2}° (f={:.3e} Hz)",
                            h.harmonic_number, h.magnitude, h.phase, h.frequency
                        );
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "PSS")),
    }
}

/// Run HB (Harmonic Balance) analysis
fn run_hb(
    engine: &Engine,
    netlist: &Netlist,
    freq: f64,
    harmonics: usize,
    _args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!(
            "Running HB analysis: f₀ = {:.3e} Hz, {} harmonics",
            freq, harmonics
        );
    }

    let config = rspice_core::analysis::HbConfig::new(freq).with_harmonics(harmonics);

    match engine.run_hb(netlist, config) {
        Ok(hb_result) => {
            if !quiet {
                println!("✓ HB converged");
                println!("  Nodes: {}", hb_result.result.num_nodes());
                println!("  Harmonics: {}", hb_result.result.num_harmonics);

                if verbose && !hb_result.result.spectral_voltages.is_empty() {
                    println!("\n  Spectral content (first node):");
                    let sv = &hb_result.result.spectral_voltages[0];
                    for k in 0..5.min(harmonics) {
                        println!(
                            "    H{}: mag={:.6e}, phase={:.2}°",
                            k,
                            sv.magnitude(k),
                            sv.phase(k).to_degrees()
                        );
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "HB")),
    }
}

/// Run PZ (Pole-Zero) analysis
fn run_pz(
    engine: &Engine,
    netlist: &Netlist,
    input_node: usize,
    output_node: usize,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!(
            "Running Pole-Zero analysis: input=node {}, output=node {}",
            input_node, output_node
        );
    }

    match engine.run_pz(netlist, input_node, output_node) {
        Ok(result) => {
            if !quiet {
                println!("✓ Pole-Zero analysis complete");
                println!("  Poles: {}", result.poles.len());
                println!("  Zeros: {}", result.zeros.len());

                if verbose {
                    println!("\n  Poles:");
                    for (i, pole) in result.poles.iter().enumerate() {
                        let freq = pole.im / (2.0 * std::f64::consts::PI);
                        let q = if pole.re.abs() > 1e-15 {
                            -pole.im / (2.0 * pole.re)
                        } else {
                            f64::INFINITY
                        };
                        println!(
                            "    P{}: {:.3e} + j{:.3e}  (f={:.3e} Hz, Q={:.2})",
                            i,
                            pole.re,
                            pole.im,
                            freq.abs(),
                            q
                        );
                    }
                    println!("\n  Zeros:");
                    for (i, zero) in result.zeros.iter().enumerate() {
                        println!("    Z{}: {:.3e} + j{:.3e}", i, zero.re, zero.im);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Pole-Zero")),
    }
}

/// Run Pole-Zero analysis from netlist `.PZ` command with named/differential ports.
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
    let resolver = NodeResolver::from_netlist(engine, netlist)?;

    let resolve = |node: &str| {
        resolver
            .resolve_node(node)
            .ok_or_else(|| CliError::SimulationError {
                message: format!("Invalid .PZ node reference '{}'", node),
                analysis: Some("Pole-Zero".to_string()),
            })
    };

    let in_pos = resolve(input_pos)?;
    let in_neg = resolve(input_neg)?;
    let out_pos = resolve(output_pos)?;
    let out_neg = resolve(output_neg)?;

    let input_is_current = matches!(
        transfer_type,
        rspice_core::netlist::PoleZeroTransferType::Current
    );
    let (compute_poles, compute_zeros) = match analysis_type {
        rspice_core::netlist::PoleZeroAnalysisType::PoleZero => (true, true),
        rspice_core::netlist::PoleZeroAnalysisType::PolesOnly => (true, false),
        rspice_core::netlist::PoleZeroAnalysisType::ZerosOnly => (false, true),
    };

    if !quiet {
        println!(
            "Running Pole-Zero analysis from netlist command: in=({},{}) out=({},{}) transfer={:?} mode={:?}",
            in_pos, in_neg, out_pos, out_neg, transfer_type, analysis_type
        );
    }

    match engine.run_pz_ports(
        netlist,
        in_pos,
        Some(in_neg),
        out_pos,
        Some(out_neg),
        input_is_current,
        compute_poles,
        compute_zeros,
    ) {
        Ok(result) => {
            if !quiet {
                println!("✓ Pole-Zero analysis complete");
                println!("  Poles: {}", result.poles.len());
                println!("  Zeros: {}", result.zeros.len());

                if verbose {
                    println!("\n  Poles:");
                    for (i, pole) in result.poles.iter().enumerate() {
                        let freq = pole.im / (2.0 * std::f64::consts::PI);
                        let q = if pole.re.abs() > 1e-15 {
                            -pole.im / (2.0 * pole.re)
                        } else {
                            f64::INFINITY
                        };
                        println!(
                            "    P{}: {:.3e} + j{:.3e}  (f={:.3e} Hz, Q={:.2})",
                            i,
                            pole.re,
                            pole.im,
                            freq.abs(),
                            q
                        );
                    }
                    println!("\n  Zeros:");
                    for (i, zero) in result.zeros.iter().enumerate() {
                        println!("    Z{}: {:.3e} + j{:.3e}", i, zero.re, zero.im);
                    }
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Pole-Zero")),
    }
}

/// Run sensitivity analysis
fn run_sensitivity(
    engine: &Engine,
    netlist: &Netlist,
    output_node: usize,
    param_name: &str,
    param_value: f64,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!(
            "Running Sensitivity analysis: ∂V({})/∂{} at {}={:.6e}",
            output_node, param_name, param_name, param_value
        );
    }

    match engine.run_sensitivity(netlist, output_node, param_name, param_value, None) {
        Ok(sensitivity) => {
            if !quiet {
                println!("✓ Sensitivity analysis complete");
                println!(
                    "  ∂V({})/∂{} = {:.6e} V/unit",
                    output_node, param_name, sensitivity
                );

                if verbose {
                    // Normalized sensitivity
                    let nominal_sens = sensitivity * param_value;
                    println!(
                        "  Normalized: {:.2}% change per 1% parameter variation",
                        nominal_sens * 100.0
                    );
                }
            }
            Ok(())
        }
        Err(e) => Err(CliError::simulation_error_in(e.to_string(), "Sensitivity")),
    }
}

/// Run sensitivity analysis from parsed `.SENS` command.
fn run_sensitivity_from_command(
    engine: &Engine,
    netlist: &Netlist,
    output_node: &str,
    reference_node: Option<&str>,
    ac_sweep: Option<rspice_core::netlist::SensitivityAcSweep>,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    let resolver = NodeResolver::from_netlist(engine, netlist)?;
    let out_pos = resolver
        .resolve_node(output_node)
        .ok_or_else(|| CliError::SimulationError {
            message: format!("Invalid .SENS output node '{}'", output_node),
            analysis: Some("Sensitivity".to_string()),
        })?;
    let out_neg = match reference_node {
        Some(node) => resolver
            .resolve_node(node)
            .ok_or_else(|| CliError::SimulationError {
                message: format!("Invalid .SENS reference node '{}'", node),
                analysis: Some("Sensitivity".to_string()),
            })?,
        None => 0,
    };

    let mut params: Vec<(String, f64)> = netlist
        .params
        .all_params()
        .into_iter()
        .filter(|(name, value)| {
            !name.starts_with("IC_")
                && !name.starts_with("NODESET_")
                && value.is_finite()
                && value.abs() > 0.0
        })
        .collect();
    params.sort_by(|a, b| a.0.cmp(&b.0));

    if params.is_empty() {
        return Err(CliError::SimulationError {
            message: ".SENS requires at least one non-zero top-level .PARAM".to_string(),
            analysis: Some("Sensitivity".to_string()),
        });
    }

    if let Some(ac) = ac_sweep {
        let freqs = generate_frequency_sweep(ac.variation, ac.points, ac.start_freq, ac.stop_freq);
        if freqs.is_empty() {
            return Err(CliError::SimulationError {
                message: "Invalid .SENS AC frequency sweep configuration".to_string(),
                analysis: Some("Sensitivity".to_string()),
            });
        }

        if !quiet {
            println!(
                "Running AC Sensitivity analysis: V({},{}) over {} frequencies",
                output_node,
                reference_node.unwrap_or("0"),
                freqs.len()
            );
        }

        for (param_name, param_value) in &params {
            let pos = engine
                .run_sensitivity_ac(netlist, out_pos, param_name, *param_value, &freqs, None)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity AC"))?;
            let combined = if out_neg == 0 {
                pos
            } else {
                let neg = engine
                    .run_sensitivity_ac(netlist, out_neg, param_name, *param_value, &freqs, None)
                    .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity AC"))?;
                pos.iter()
                    .zip(neg.iter())
                    .map(|(sp, sn)| sp - sn)
                    .collect::<Vec<_>>()
            };

            if !quiet {
                let first = combined.first().copied().unwrap_or(0.0);
                let last = combined.last().copied().unwrap_or(0.0);
                println!(
                    "  d|V|/d{}: {:.6e} @ {:e} Hz, {:.6e} @ {:e} Hz",
                    param_name,
                    first,
                    freqs.first().copied().unwrap_or(0.0),
                    last,
                    freqs.last().copied().unwrap_or(0.0)
                );
            }

            if verbose && !quiet {
                let peak = combined
                    .iter()
                    .map(|v| v.abs())
                    .fold(0.0_f64, |acc, v| acc.max(v));
                println!("    peak |d|V|/d{}| = {:.6e}", param_name, peak);
            }
        }

        return Ok(());
    }

    if !quiet {
        println!(
            "Running DC Sensitivity analysis: V({},{})",
            output_node,
            reference_node.unwrap_or("0")
        );
    }

    let mut results: Vec<(String, f64)> = Vec::with_capacity(params.len());
    for (param_name, param_value) in &params {
        let pos = engine
            .run_sensitivity(netlist, out_pos, param_name, *param_value, None)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity"))?;
        let combined = if out_neg == 0 {
            pos
        } else {
            let neg = engine
                .run_sensitivity(netlist, out_neg, param_name, *param_value, None)
                .map_err(|e| CliError::simulation_error_in(e.to_string(), "Sensitivity"))?;
            pos - neg
        };
        results.push((param_name.clone(), combined));
    }

    results.sort_by(|a, b| {
        b.1.abs()
            .partial_cmp(&a.1.abs())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    if !quiet {
        println!("✓ Sensitivity analysis complete");
        for (name, value) in &results {
            println!("  ∂V/∂{} = {:.6e}", name, value);
        }
    }

    Ok(())
}

/// Run corner sweep (process corners)
fn run_corner_sweep(
    engine: &Engine,
    netlist: &Netlist,
    corners_str: &str,
    args: &RunArgs,
    config: &Config,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    use rspice_core::analysis::advanced::ProcessCorner;

    let corner_strs: Vec<&str> = corners_str.split(',').map(|s| s.trim()).collect();

    // Parse corner names into ProcessCorner enum
    let corners: Vec<(ProcessCorner, String)> = corner_strs
        .iter()
        .enumerate()
        .map(|(i, s)| {
            let corner = match s.to_lowercase().as_str() {
                "tt" | "typ" | "typical" => ProcessCorner::TT,
                "ss" | "slow" => ProcessCorner::SS,
                "ff" | "fast" => ProcessCorner::FF,
                "sf" | "snfp" => ProcessCorner::SF,
                "fs" | "fnsp" => ProcessCorner::FS,
                _ => ProcessCorner::Custom(i as u8),
            };
            (corner, s.to_string())
        })
        .collect();

    if !quiet {
        println!("Running PVT corner sweep: {} corners", corners.len());
        if verbose {
            println!("  Process corners:");
            for (corner, name) in &corners {
                let nmos = corner.nmos_factor();
                let pmos = corner.pmos_factor();
                println!("    {}: NMOS={:.2}x, PMOS={:.2}x", name, nmos, pmos);
            }
        }
    }

    let mut results: Vec<(String, bool)> = Vec::new();

    for (i, (corner, name)) in corners.iter().enumerate() {
        if !quiet {
            println!("\n[{}/{}] Corner: {}", i + 1, corners.len(), name);
        }

        let nmos_factor = corner.nmos_factor();
        let pmos_factor = corner.pmos_factor();
        if verbose && !quiet {
            println!(
                "  Process scaling: NMOS={:.2}x, PMOS={:.2}x",
                nmos_factor, pmos_factor
            );
        }

        // Run the analyses from the netlist
        let mut corner_passed = true;
        for analysis in &netlist.analyses {
            if let Err(e) = run_analysis(engine, netlist, analysis, args, config, verbose, quiet) {
                if !quiet {
                    eprintln!("  Analysis failed: {}", e);
                }
                corner_passed = false;
            }
        }

        if netlist.analyses.is_empty() {
            if let Err(e) = run_dc_op(engine, netlist, args, quiet) {
                if !quiet {
                    eprintln!("  DC OP failed: {}", e);
                }
                corner_passed = false;
            }
        }

        results.push((name.clone(), corner_passed));
    }

    if !quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│        Corner Sweep Summary         │");
        println!("├─────────────────────────────────────┤");
        for (name, passed) in &results {
            let status = if *passed { "✓ PASS" } else { "✗ FAIL" };
            println!("│  {:6}  {:>24}  │", name, status);
        }
        println!("└─────────────────────────────────────┘");

        let passed_count = results.iter().filter(|(_, p)| *p).count();
        println!(
            "\n✓ Corner sweep complete: {}/{} corners passed",
            passed_count,
            corners.len()
        );
    }

    Ok(())
}

/// Run Fourier (THD) analysis
///
/// Performs transient simulation and computes Fourier components and THD.
/// This implements the .FOUR directive from SPICE.
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
    use rspice_core::analysis::{FourierAnalysis, FourierConfig};

    if !quiet {
        println!(
            "Running Fourier analysis: fundamental = {} Hz, {} harmonics",
            fundamental, num_harmonics
        );
        if verbose {
            println!("  Output nodes: {:?}", outputs);
        }
    }

    // Determine analysis window (need enough periods for accurate FFT)
    let period = 1.0 / fundamental;
    let analysis_time = period * (num_harmonics + 2) as f64;
    let tstep = period / 100.0; // 100 points per period for good resolution

    // Run transient to get waveform data
    let tran_result = engine
        .run_tran(netlist, analysis_time, tstep)
        .map_err(|e| CliError::simulation_error_in(e.to_string(), "Fourier (transient)"))?;

    // Configure Fourier analysis
    let config = FourierConfig::new(fundamental).with_harmonics(num_harmonics);
    let fourier = FourierAnalysis::new(config);
    let resolver = NodeResolver::from_netlist(engine, netlist)?;

    if !quiet {
        println!("\n┌────────────────────────────────────────────────────────────────┐");
        println!("│                    FOURIER ANALYSIS RESULTS                    │");
        println!("├────────────────────────────────────────────────────────────────┤");
    }

    // Analyze each output
    for output in outputs {
        if let Some((node_idx, reference_idx)) = resolver.parse_voltage_probe(output) {
            let result = if reference_idx == 0 {
                let waveform = tran_result.voltage_waveform(node_idx);
                fourier.analyze(&tran_result.time, waveform)
            } else {
                let pos_waveform = tran_result.voltage_waveform(node_idx);
                let neg_waveform = tran_result.voltage_waveform(reference_idx);
                let diff_waveform: Vec<f64> = pos_waveform
                    .iter()
                    .zip(neg_waveform.iter())
                    .map(|(vp, vn)| vp - vn)
                    .collect();
                fourier.analyze(&tran_result.time, &diff_waveform)
            };

            if !quiet {
                println!("│ Output: {:54} │", output);
                println!("├────────────────────────────────────────────────────────────────┤");
                println!("│  Harmonic    Frequency (Hz)    Magnitude    Phase (deg)       │");
                println!("├────────────────────────────────────────────────────────────────┤");

                for (i, harmonic) in result.harmonics.iter().enumerate() {
                    let freq = fundamental * (i + 1) as f64;
                    println!(
                        "│  {:3}        {:12.4e}      {:10.6}   {:10.2}         │",
                        i + 1,
                        freq,
                        harmonic.magnitude,
                        harmonic.phase.to_degrees()
                    );
                }

                println!("├────────────────────────────────────────────────────────────────┤");
                println!(
                    "│  DC Component:   {:10.6}                                    │",
                    result.dc_component
                );
                println!(
                    "│  THD:            {:10.4} %                                   │",
                    result.thd * 100.0
                );
                println!("└────────────────────────────────────────────────────────────────┘");
            }
        } else if !quiet {
            println!("│ Warning: Could not find node for output '{}'", output);
        }
    }

    // Write results to output file if specified
    if let Some(ref output_path) = args.output {
        if matches!(args.format, OutputFormat::Json) {
            use std::io::Write;
            let mut file =
                std::fs::File::create(output_path).map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

            let json = serde_json::json!({
                "analysis": "fourier",
                "fundamental_hz": fundamental,
                "num_harmonics": num_harmonics,
                "outputs": outputs,
            });

            writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(|e| {
                CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                }
            })?;

            if !quiet {
                println!("\nResults written to: {}", output_path.display());
            }
        }
    }

    Ok(())
}

/// Parse output node specification (e.g., "V(out)", "V(3)", "out").
/// Returns the positive node index only (reference defaults to ground).
#[cfg(test)]
fn parse_output_node(output: &str, resolver: &NodeResolver) -> Option<usize> {
    resolver.parse_voltage_probe(output).map(|(pos, _)| pos)
}

#[derive(Debug, Clone)]
struct NodeResolver {
    node_name_to_index: std::collections::HashMap<String, usize>,
}

impl NodeResolver {
    fn from_netlist(engine: &Engine, netlist: &Netlist) -> Result<Self, CliError> {
        let circuit = engine
            .build_circuit(netlist)
            .map_err(|e| CliError::simulation_error_in(e.to_string(), "Node Resolution"))?;

        let node_name_to_index = circuit
            .node_names_sorted()
            .iter()
            .enumerate()
            .map(|(idx, name)| (name.to_ascii_uppercase(), idx + 1))
            .collect();

        Ok(Self { node_name_to_index })
    }

    fn resolve_node(&self, node: &str) -> Option<usize> {
        let node = node.trim();
        if node.is_empty() {
            return None;
        }
        if node == "0" || node.eq_ignore_ascii_case("gnd") {
            return Some(0);
        }
        if let Ok(idx) = node.parse::<usize>() {
            return Some(idx);
        }

        self.node_name_to_index
            .get(&node.to_ascii_uppercase())
            .copied()
    }

    fn parse_voltage_probe(&self, spec: &str) -> Option<(usize, usize)> {
        let (pos_spec, neg_spec) = parse_voltage_probe_spec(spec)?;
        let pos = self.resolve_node(&pos_spec)?;
        let neg = match neg_spec {
            Some(ref_name) => self.resolve_node(&ref_name)?,
            None => 0,
        };
        Some((pos, neg))
    }
}

fn parse_voltage_probe_spec(spec: &str) -> Option<(String, Option<String>)> {
    let trimmed = spec.trim();
    if trimmed.is_empty() {
        return None;
    }

    if trimmed.len() >= 3
        && trimmed.get(..2).map(|s| s.eq_ignore_ascii_case("V(")) == Some(true)
        && trimmed.ends_with(')')
    {
        let inner = &trimmed[2..trimmed.len() - 1];
        let mut parts = inner.split(',').map(|s| s.trim()).filter(|s| !s.is_empty());
        let pos = parts.next()?.to_string();
        let neg = parts.next().map(|s| s.to_string());
        if parts.next().is_some() {
            return None;
        }
        return Some((pos, neg));
    }

    Some((trimmed.to_string(), None))
}

fn generate_step_values(sweep: &rspice_core::netlist::StepSweep) -> Result<Vec<f64>, CliError> {
    use rspice_core::netlist::StepSweep;

    const MAX_STEP_POINTS: usize = 1_000_000;

    match sweep {
        StepSweep::Linear { start, stop, step } => {
            if !start.is_finite() || !stop.is_finite() || !step.is_finite() {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep requires finite start/stop/step values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if step.abs() <= f64::EPSILON {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step cannot be zero".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }
            if (*stop - *start) * *step < 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP linear sweep step sign is inconsistent with start/stop range"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }

            let mut values = Vec::new();
            let mut v = *start;
            let tol = start.abs().max(stop.abs()).max(1.0) * 1e-12;

            for _ in 0..MAX_STEP_POINTS {
                values.push(v);
                let next = v + *step;
                let finished = if *step > 0.0 {
                    next > *stop + tol
                } else {
                    next < *stop - tol
                };
                if finished {
                    return Ok(values);
                }
                v = next;
            }

            Err(CliError::SimulationError {
                message: format!(
                    ".STEP linear sweep exceeded {} points; check start/stop/step values",
                    MAX_STEP_POINTS
                ),
                analysis: Some("Step".to_string()),
            })
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            if *points_per_decade == 0 {
                return Err(CliError::SimulationError {
                    message: ".STEP DEC sweep requires points_per_decade > 0".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP DEC sweep requires finite positive start/stop values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }

            let ascending = stop > start;
            let ratio = if ascending {
                stop / start
            } else {
                start / stop
            };
            let total_steps = (ratio.log10() * (*points_per_decade as f64)).ceil() as usize;
            let tol = stop.abs().max(1.0) * 1e-12;

            let mut values = Vec::with_capacity(total_steps + 2);
            for i in 0..=total_steps {
                let factor = 10_f64.powf(i as f64 / *points_per_decade as f64);
                let value = if ascending {
                    start * factor
                } else {
                    start / factor
                };

                let in_range = if ascending {
                    value <= *stop + tol
                } else {
                    value >= *stop - tol
                };
                if in_range {
                    values.push(value);
                }
            }

            if values.is_empty() {
                values.push(*start);
            }
            let last = *values.last().unwrap_or(start);
            if (last - *stop).abs() > tol {
                values.push(*stop);
            }

            Ok(values)
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            if *points_per_octave == 0 {
                return Err(CliError::SimulationError {
                    message: ".STEP OCT sweep requires points_per_octave > 0".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if !start.is_finite() || !stop.is_finite() || *start <= 0.0 || *stop <= 0.0 {
                return Err(CliError::SimulationError {
                    message: ".STEP OCT sweep requires finite positive start/stop values"
                        .to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if (*stop - *start).abs() <= f64::EPSILON {
                return Ok(vec![*start]);
            }

            let ascending = stop > start;
            let ratio = if ascending {
                stop / start
            } else {
                start / stop
            };
            let total_steps = (ratio.log2() * (*points_per_octave as f64)).ceil() as usize;
            let tol = stop.abs().max(1.0) * 1e-12;

            let mut values = Vec::with_capacity(total_steps + 2);
            for i in 0..=total_steps {
                let factor = 2_f64.powf(i as f64 / *points_per_octave as f64);
                let value = if ascending {
                    start * factor
                } else {
                    start / factor
                };

                let in_range = if ascending {
                    value <= *stop + tol
                } else {
                    value >= *stop - tol
                };
                if in_range {
                    values.push(value);
                }
            }

            if values.is_empty() {
                values.push(*start);
            }
            let last = *values.last().unwrap_or(start);
            if (last - *stop).abs() > tol {
                values.push(*stop);
            }

            Ok(values)
        }
        StepSweep::List(values) => {
            if values.is_empty() {
                return Err(CliError::SimulationError {
                    message: ".STEP LIST requires at least one value".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            if values.iter().any(|v| !v.is_finite()) {
                return Err(CliError::SimulationError {
                    message: ".STEP LIST values must be finite".to_string(),
                    analysis: Some("Step".to_string()),
                });
            }
            Ok(values.clone())
        }
    }
}

fn generate_frequency_sweep(
    variation: rspice_core::netlist::FreqVariation,
    points: usize,
    start_freq: f64,
    stop_freq: f64,
) -> Vec<f64> {
    if points == 0 || start_freq <= 0.0 || stop_freq <= 0.0 {
        return Vec::new();
    }
    if (stop_freq - start_freq).abs() < f64::EPSILON || points == 1 {
        return vec![start_freq];
    }

    match variation {
        rspice_core::netlist::FreqVariation::Lin => (0..points)
            .map(|i| start_freq + (stop_freq - start_freq) * (i as f64) / ((points - 1) as f64))
            .collect(),
        rspice_core::netlist::FreqVariation::Oct => {
            let octaves = (stop_freq / start_freq).log2().abs();
            let total_points = (octaves * points as f64).ceil() as usize + 1;
            (0..total_points)
                .map(|i| {
                    start_freq
                        * (stop_freq / start_freq)
                            .powf((i as f64) / ((total_points.saturating_sub(1)) as f64))
                })
                .collect()
        }
        rspice_core::netlist::FreqVariation::Dec => {
            let decades = (stop_freq / start_freq).log10().abs();
            let total_points = (decades * points as f64).ceil() as usize + 1;
            (0..total_points)
                .map(|i| {
                    start_freq
                        * (stop_freq / start_freq)
                            .powf((i as f64) / ((total_points.saturating_sub(1)) as f64))
                })
                .collect()
        }
    }
}

/// Run temperature sweep analysis
///
/// Executes DC operating point at each specified temperature.
/// This implements the .TEMP directive from SPICE.
fn run_temp(
    engine: &Engine,
    netlist: &Netlist,
    temperatures: &[f64],
    args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    if !quiet {
        println!("Running temperature sweep: {} points", temperatures.len());
        if verbose {
            println!("  Temperatures: {:?} °C", temperatures);
        }
    }

    let mut results: Vec<(f64, Vec<f64>)> = Vec::new();

    for (i, temp_c) in temperatures.iter().enumerate() {
        let temp_k = temp_c + 273.15; // Convert to Kelvin

        if !quiet {
            println!(
                "\n[{}/{}] Temperature: {:.1} °C ({:.1} K)",
                i + 1,
                temperatures.len(),
                temp_c,
                temp_k
            );
        }

        // Create engine with modified temperature
        let mut temp_config = engine.config().clone();
        temp_config.temperature = temp_k;
        let temp_engine = Engine::new(temp_config);

        match temp_engine.run_dc_op(netlist) {
            Ok(result) => {
                if verbose && !quiet {
                    // Print first few node voltages
                    for j in 1..=result.node_voltages.len().min(5) {
                        println!("  V({}) = {:.6} V", j, result.voltage(j));
                    }
                }
                results.push((*temp_c, result.node_voltages.clone()));
            }
            Err(e) => {
                if !quiet {
                    eprintln!("  DC OP failed at {:.1} °C: {}", temp_c, e);
                }
            }
        }
    }

    // Summary
    if !quiet {
        println!("\n┌─────────────────────────────────────┐");
        println!("│     Temperature Sweep Summary       │");
        println!("├─────────────────────────────────────┤");
        println!(
            "│  Points:  {:3}/{:3} converged         │",
            results.len(),
            temperatures.len()
        );
        println!(
            "│  Range:   {:6.1} °C to {:6.1} °C    │",
            temperatures.first().unwrap_or(&0.0),
            temperatures.last().unwrap_or(&0.0)
        );
        println!("└─────────────────────────────────────┘");
    }

    // Write results to output file if specified
    if let Some(ref output_path) = args.output {
        use std::io::Write;
        let mut file = std::fs::File::create(output_path).map_err(|e| CliError::OutputError {
            path: output_path.clone(),
            source: e,
        })?;

        match args.format {
            OutputFormat::Csv => {
                // Header
                let num_nodes = results.first().map(|(_, v)| v.len()).unwrap_or(0);
                let header: String = (1..=num_nodes).map(|i| format!(",V({})", i)).collect();
                writeln!(file, "Temperature_C{}", header).map_err(|e| CliError::OutputError {
                    path: output_path.clone(),
                    source: e,
                })?;

                // Data
                for (temp, voltages) in &results {
                    let values: String = voltages.iter().map(|v| format!(",{:.9e}", v)).collect();
                    writeln!(file, "{:.2}{}", temp, values).map_err(|e| CliError::OutputError {
                        path: output_path.clone(),
                        source: e,
                    })?;
                }
            }
            OutputFormat::Json => {
                let json = serde_json::json!({
                    "analysis": "temperature_sweep",
                    "temperatures_c": temperatures,
                    "results": results.iter().map(|(t, v)| {
                        serde_json::json!({
                            "temperature_c": t,
                            "voltages": v,
                        })
                    }).collect::<Vec<_>>(),
                });
                writeln!(file, "{}", serde_json::to_string_pretty(&json).unwrap()).map_err(
                    |e| CliError::OutputError {
                        path: output_path.clone(),
                        source: e,
                    },
                )?;
            }
            _ => {
                // Default: simple text output
                for (temp, voltages) in &results {
                    writeln!(file, "T={:.2}C: {:?}", temp, voltages).map_err(|e| {
                        CliError::OutputError {
                            path: output_path.clone(),
                            source: e,
                        }
                    })?;
                }
            }
        }

        if !quiet {
            println!("\nResults written to: {}", output_path.display());
        }
    }

    Ok(())
}

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
