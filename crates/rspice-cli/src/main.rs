//! RSpice CLI - High-performance SPICE circuit simulator
//!
//! Usage:
//!   rspice <netlist.sp>              Run simulation
//!   rspice -o output.raw <netlist>   Specify output file
//!   rspice --help                    Show help

use clap::Parser;
use std::path::PathBuf;
use std::process::ExitCode;

mod commands;
mod output;

/// RSpice - High-performance SPICE circuit simulator
#[derive(Parser, Debug)]
#[command(name = "rspice")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input netlist file (.sp, .cir, .net)
    #[arg(value_name = "NETLIST")]
    input: Option<PathBuf>,

    /// Output file for simulation results
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,

    /// Output format (raw, csv, json)
    #[arg(short, long, default_value = "raw")]
    format: String,

    /// Verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Run in batch mode (no interactive prompts)
    #[arg(short, long)]
    batch: bool,

    /// Override simulation temperature (Celsius)
    #[arg(long, value_name = "TEMP")]
    temp: Option<f64>,

    /// Print version information
    #[arg(long)]
    print_version: bool,

    /// Print .MEAS measurement results
    #[arg(long)]
    meas: bool,

    /// Quiet mode - suppress progress output (for scripting)
    #[arg(short, long)]
    quiet: bool,

    /// Show progress bar with ETA for transient simulation
    #[arg(long)]
    progress: bool,

    /// Print node names in output (if available from netlist)
    #[arg(long)]
    node_names: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    // Initialize logging
    if args.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();
    }

    if args.print_version {
        println!("RSpice version {}", env!("CARGO_PKG_VERSION"));
        println!("High-performance SPICE circuit simulator");
        return ExitCode::SUCCESS;
    }

    match run(args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    let input_path = args
        .input
        .ok_or("No input file specified. Use --help for usage.")?;

    if !input_path.exists() {
        return Err(format!("Input file not found: {}", input_path.display()).into());
    }

    log::info!("Loading netlist: {}", input_path.display());

    // Parse the netlist
    let netlist = rspice_core::Netlist::parse_file(&input_path)?;

    if args.verbose {
        println!("Title: {}", netlist.title);
        println!("Elements: {}", netlist.elements.len());
        println!("Analyses: {}", netlist.analyses.len());
    }

    // Create simulation engine
    let engine = rspice_core::Engine::default();

    // Process all analysis commands in the netlist
    use rspice_core::netlist::AnalysisCommand;

    for (idx, analysis) in netlist.analyses.iter().enumerate() {
        if args.verbose {
            println!(
                "\nRunning analysis {}/{}: {:?}",
                idx + 1,
                netlist.analyses.len(),
                analysis
            );
        }

        match analysis {
            AnalysisCommand::Op => {
                if !args.quiet {
                    println!("Running DC operating point...");
                }
                match engine.run_dc_op(&netlist) {
                    Ok(result) => {
                        if !args.quiet {
                            println!("DC Operating Point:");
                            for i in 1..=result.node_voltages.len().min(10) {
                                println!("  V({}) = {:.6} V", i, result.voltage(i));
                            }
                            if result.node_voltages.len() > 10 {
                                println!("  ... ({} more nodes)", result.node_voltages.len() - 10);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("DC OP failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Dc {
                source,
                start,
                stop,
                step,
            } => {
                println!(
                    "Running DC sweep on {} from {} to {} by {}...",
                    source, start, stop, step
                );
                match engine.run_dc_sweep(&netlist, source, *start, *stop, *step) {
                    Ok(results) => {
                        println!("DC Sweep: {} points computed", results.len());
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
                            let format = if args.format == "ascii" {
                                rspice_core::analysis::RawFormat::Ascii
                            } else {
                                rspice_core::analysis::RawFormat::Binary
                            };
                            rspice_core::analysis::export_dc_sweep(
                                output_path,
                                &sweep_vals,
                                source,
                                &node_names,
                                &node_waveforms,
                                format,
                            )?;
                            println!("Results exported to: {}", output_path.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("DC sweep failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Tran {
                step,
                stop,
                start,
                max_step: _,
            } => {
                let tstart = start.unwrap_or(0.0);

                // Progress indicator for transient simulation
                use indicatif::{ProgressBar, ProgressStyle};

                let pb = if args.quiet {
                    ProgressBar::hidden()
                } else if args.progress {
                    // Full progress bar with ETA
                    let eta_steps = ((stop - tstart) / step) as u64;
                    let pb = ProgressBar::new(eta_steps);
                    pb.set_style(
                        ProgressStyle::default_bar()
                            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                            .unwrap()
                            .progress_chars("#>-"),
                    );
                    pb.set_message(format!("Transient: {} to {}", tstart, stop));
                    pb
                } else {
                    // Simple spinner
                    let pb = ProgressBar::new_spinner();
                    pb.set_style(
                        ProgressStyle::default_spinner()
                            .template("{spinner:.green} {msg}")
                            .unwrap(),
                    );
                    pb.set_message(format!(
                        "Running transient: {} to {} (step {})...",
                        tstart, stop, step
                    ));
                    pb.enable_steady_tick(std::time::Duration::from_millis(100));
                    pb
                };

                let result = engine.run_tran(&netlist, *stop, *step);
                pb.finish_and_clear();

                match result {
                    Ok(result) => {
                        if !args.quiet {
                            println!(
                                "✓ Transient complete: {} time points computed",
                                result.time.len()
                            );
                        }

                        // Process .MEAS commands if --meas flag is set
                        if args.meas {
                            use std::collections::HashMap;

                            // Build signals map from simulation results
                            let mut signals: HashMap<String, &[f64]> = HashMap::new();
                            for (i, waveform) in result.voltages.iter().enumerate() {
                                let name = format!("V({})", i + 1);
                                signals.insert(name.clone(), waveform.as_slice());
                                // Also add lowercase version for case-insensitive matching
                                signals.insert(name.to_lowercase(), waveform.as_slice());
                            }

                            // Use measurements from the netlist
                            let mut meas_engine = rspice_core::MeasureEngine::new();
                            for meas in &netlist.measurements {
                                meas_engine.add(meas.clone());
                            }
                            let meas_results = meas_engine.evaluate(&result.time, &signals);

                            if !args.quiet {
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
                        }

                        // Export if output path specified
                        if let Some(ref output_path) = args.output {
                            let node_names: Vec<String> =
                                (1..=result.voltages.len()).map(|i| i.to_string()).collect();
                            let format = if args.format == "ascii" {
                                rspice_core::analysis::RawFormat::Ascii
                            } else {
                                rspice_core::analysis::RawFormat::Binary
                            };
                            rspice_core::analysis::export_transient(
                                output_path,
                                &result.time,
                                &node_names,
                                &result.voltages,
                                format,
                            )?;
                            if !args.quiet {
                                println!("  Results exported to: {}", output_path.display());
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("✗ Transient failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Ac {
                variation: _,
                points,
                start_freq,
                stop_freq,
            } => {
                if !args.quiet {
                    println!(
                        "Running AC analysis: {} to {} Hz ({} points)...",
                        start_freq, stop_freq, points
                    );
                }

                // Generate frequency points (decade sweep)
                let frequencies: Vec<f64> = (0..*points)
                    .map(|i| {
                        start_freq
                            * (stop_freq / start_freq).powf(i as f64 / (*points as f64 - 1.0))
                    })
                    .collect();

                match engine.run_ac(&netlist, &frequencies) {
                    Ok(results) => {
                        if !args.quiet {
                            println!("AC Analysis: {} frequency points", results.len());
                            if args.verbose && !results.is_empty() {
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
                        // Bode plot CSV export
                        if let Some(ref output_path) = args.output {
                            if args.format == "csv" {
                                use std::io::Write;
                                let mut file = std::fs::File::create(output_path)?;
                                writeln!(file, "Frequency_Hz,Magnitude_dB,Phase_deg")?;
                                for r in &results {
                                    let mag_db = 20.0 * r.voltage_magnitude(1).log10();
                                    let phase_deg =
                                        r.voltage_phase(1) * 180.0 / std::f64::consts::PI;
                                    writeln!(
                                        file,
                                        "{:e},{:.4},{:.2}",
                                        r.frequency, mag_db, phase_deg
                                    )?;
                                }
                                if !args.quiet {
                                    println!("  Bode plot exported to: {}", output_path.display());
                                }
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("AC analysis failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Noise {
                output_node,
                input_source,
                variation: _,
                points,
                start_freq,
                stop_freq,
                ..
            } => {
                if !args.quiet {
                    println!(
                        "Running Noise analysis on {} from {} to {} Hz ({} points)...",
                        output_node, start_freq, stop_freq, points
                    );
                }

                // Generate frequency points (decade sweep)
                let frequencies: Vec<f64> = (0..*points)
                    .map(|i| {
                        start_freq
                            * (stop_freq / start_freq).powf(i as f64 / (*points as f64 - 1.0))
                    })
                    .collect();

                match engine.run_ac(&netlist, &frequencies) {
                    Ok(_ac_results) => {
                        if !args.quiet {
                            println!("Noise Analysis: {} frequency points", frequencies.len());
                            println!("  Output node: {}", output_node);
                            println!("  Input source: {}", input_source);
                            // TODO: Integrate with NoiseAnalysis engine for actual noise calculation
                            println!(
                                "  (Noise spectral density calculation pending full integration)"
                            );
                        }
                    }
                    Err(e) => {
                        eprintln!("Noise analysis failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Step(step_cmd) => {
                use rspice_core::netlist::{StepSweep, StepTarget};

                // Generate parameter values for the sweep
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

                if !args.quiet {
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
                    if args.verbose && !args.quiet {
                        println!(
                            "  Step {}/{}: {} = {:.4e}",
                            i + 1,
                            values.len(),
                            step_cmd.name,
                            value
                        );
                    }
                    // Note: actual parameter substitution would modify the netlist
                    // For now, we just run the analysis (demonstration)
                    match engine.run_dc_op(&netlist) {
                        Ok(result) => {
                            if args.verbose && !args.quiet {
                                println!("    V(1) = {:.6} V", result.voltage(1));
                            }
                        }
                        Err(e) => {
                            eprintln!("  Step {} failed: {}", i + 1, e);
                        }
                    }
                }

                if !args.quiet {
                    println!("✓ .STEP sweep complete: {} iterations", values.len());
                }
            }
            _ => {
                if !args.quiet {
                    println!("Analysis type {:?} not yet fully supported", analysis);
                }
            }
        }
    }

    if netlist.analyses.is_empty() {
        // Default: run DC operating point
        if !args.quiet {
            println!("No analysis commands - running default DC OP...");
        }
        match engine.run_dc_op(&netlist) {
            Ok(result) => {
                if !args.quiet {
                    println!("DC Operating Point:");
                    for i in 1..=result.node_voltages.len().min(10) {
                        println!("  V({}) = {:.6} V", i, result.voltage(i));
                    }
                }
            }
            Err(e) => {
                eprintln!("DC OP failed: {}", e);
            }
        }
    }

    if !args.quiet {
        println!("\nSimulation complete.");
    }
    Ok(())
}
