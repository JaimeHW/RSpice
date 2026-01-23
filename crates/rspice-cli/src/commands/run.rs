//! Run Command - Execute SPICE simulations
//!
//! This is the main simulation execution command, supporting:
//! - All analysis types (DC, AC, Transient, Noise, etc.)
//! - Multiple output formats
//! - Progress reporting
//! - Waveform compression

use crate::cli::{CliError, Config, OutputFormat, RunArgs};
use crate::report::{JUnitReporter, JsonMeasReporter, SimulationReport, TapReporter};
use indicatif::{ProgressBar, ProgressStyle};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{Engine, Netlist, SimulationConfig};
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
    let sim_config = build_sim_config(&args, config);
    let engine = Engine::new(sim_config);

    // Monte Carlo mode
    if let Some(num_runs) = args.monte_carlo {
        return run_monte_carlo(
            &engine,
            &netlist,
            num_runs,
            args.seed.unwrap_or(1),
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
fn build_sim_config(args: &RunArgs, config: &Config) -> SimulationConfig {
    let mut sim_config = SimulationConfig::default();

    // Temperature (CLI overrides config)
    if let Some(temp) = args.temp {
        sim_config.temperature = temp + 273.15; // Convert to Kelvin
    } else {
        sim_config.temperature = config.simulation.temperature + 273.15;
    }

    // Tolerances
    if let Some(abstol) = args.abstol {
        sim_config.tolerance = abstol;
    } else {
        sim_config.tolerance = config.simulation.abstol;
    }

    // Iterations
    if let Some(maxiter) = args.maxiter {
        sim_config.max_iterations = maxiter;
    } else {
        sim_config.max_iterations = config.simulation.max_iterations;
    }

    // Timestep limits
    if let Some(min_step) = args.min_step {
        sim_config.min_timestep = min_step;
    }
    if let Some(max_step) = args.max_step {
        sim_config.max_timestep = max_step;
    }

    sim_config
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
            max_step: _,
        } => {
            let tstart = start.unwrap_or(0.0);
            run_transient(engine, netlist, *stop, *step, tstart, args, quiet)?;
        }
        AnalysisCommand::Ac {
            variation: _,
            points,
            start_freq,
            stop_freq,
        } => {
            run_ac(
                engine,
                netlist,
                *points,
                *start_freq,
                *stop_freq,
                args,
                verbose,
                quiet,
            )?;
        }
        AnalysisCommand::Noise {
            output_node,
            input_source,
            points,
            start_freq,
            stop_freq,
            ..
        } => {
            run_noise(
                engine,
                netlist,
                output_node,
                input_source,
                *points,
                *start_freq,
                *stop_freq,
                quiet,
            )?;
        }
        AnalysisCommand::Step(step_cmd) => {
            run_step(engine, netlist, step_cmd, args, verbose, quiet)?;
        }
        _ => {
            if !quiet {
                println!("Analysis type {:?} not yet fully supported", analysis);
            }
        }
    }
    Ok(())
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
    args: &RunArgs,
    quiet: bool,
) -> Result<(), CliError> {
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
        match engine.run_tran_compressed(netlist, tstop, tstep, compression) {
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
        engine.run_tran(netlist, tstop, tstep)
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

    // Generate frequency points (decade sweep)
    let frequencies: Vec<f64> = (0..points)
        .map(|i| start_freq * (stop_freq / start_freq).powf(i as f64 / (points as f64 - 1.0)))
        .collect();

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
    input_source: &str,
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

    // Generate frequency points
    let frequencies: Vec<f64> = (0..points)
        .map(|i| start_freq * (stop_freq / start_freq).powf(i as f64 / (points as f64 - 1.0)))
        .collect();

    match engine.run_ac(netlist, &frequencies) {
        Ok(_) => {
            if !quiet {
                println!("Noise Analysis: {} frequency points", frequencies.len());
                println!("  Output node: {}", output_node);
                println!("  Input source: {}", input_source);
                println!("  (Noise spectral density calculation pending full integration)");
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
    _args: &RunArgs,
    verbose: bool,
    quiet: bool,
) -> Result<(), CliError> {
    use rspice_core::netlist::{StepSweep, StepTarget};

    // Generate parameter values
    let values: Vec<f64> = match &step_cmd.sweep {
        StepSweep::Linear { start, stop, step } => {
            let mut vals = Vec::new();
            let mut v = *start;
            while v <= *stop {
                vals.push(v);
                v += step;
            }
            vals
        }
        StepSweep::Decade {
            points_per_decade,
            start,
            stop,
        } => {
            let decades = (stop / start).log10();
            let num_points = (decades * (*points_per_decade as f64)).ceil() as usize;
            (0..=num_points)
                .map(|i| start * 10_f64.powf(i as f64 / *points_per_decade as f64))
                .take_while(|&v| v <= *stop)
                .collect()
        }
        StepSweep::Octave {
            points_per_octave,
            start,
            stop,
        } => {
            let octaves = (stop / start).log2();
            let num_points = (octaves * (*points_per_octave as f64)).ceil() as usize;
            (0..=num_points)
                .map(|i| start * 2_f64.powf(i as f64 / *points_per_octave as f64))
                .take_while(|&v| v <= *stop)
                .collect()
        }
        StepSweep::List(vals) => vals.clone(),
    };

    if !quiet {
        let target_name = match step_cmd.target {
            StepTarget::Param => format!("PARAM {}", step_cmd.name),
            StepTarget::Device => format!("device {}", step_cmd.name),
            StepTarget::Model => format!("MODEL {}", step_cmd.name),
            StepTarget::Temp => "TEMP".to_string(),
        };
        println!(
            "Running .STEP sweep on {}: {} values ({:.3e} to {:.3e})...",
            target_name,
            values.len(),
            values.first().unwrap_or(&0.0),
            values.last().unwrap_or(&0.0)
        );
    }

    // Run DC OP for each parameter value
    for (i, value) in values.iter().enumerate() {
        if verbose && !quiet {
            println!(
                "  Step {}/{}: {} = {:.4e}",
                i + 1,
                values.len(),
                step_cmd.name,
                value
            );
        }
        match engine.run_dc_op(netlist) {
            Ok(result) => {
                if verbose && !quiet {
                    println!("    V(1) = {:.6} V", result.voltage(1));
                }
            }
            Err(e) => {
                eprintln!("  Step {} failed: {}", i + 1, e);
            }
        }
    }

    if !quiet {
        println!("✓ .STEP sweep complete: {} iterations", values.len());
    }
    Ok(())
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
            value: mr.value.map(|v| f64::from(v)),
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

    match engine.run_monte_carlo(netlist, num_runs, seed) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_sim_config_defaults() {
        let args = RunArgs {
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
        };
        let config = Config::default();
        let sim_config = build_sim_config(&args, &config);

        assert_eq!(sim_config.temperature, 300.15); // 27°C in K
        assert_eq!(sim_config.max_iterations, 50);
    }

    #[test]
    fn test_build_sim_config_overrides() {
        let args = RunArgs {
            input: std::path::PathBuf::from("test.sp"),
            output: None,
            format: OutputFormat::Raw,
            batch: false,
            temp: Some(85.0),
            meas: false,
            progress: false,
            node_names: false,
            compress: false,
            compress_tol: None,
            maxiter: Some(100),
            abstol: Some(1e-15),
            reltol: None,
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
        };
        let config = Config::default();
        let sim_config = build_sim_config(&args, &config);

        assert_eq!(sim_config.temperature, 358.15); // 85°C in K
        assert_eq!(sim_config.max_iterations, 100);
        assert_eq!(sim_config.tolerance, 1e-15);
    }
}
