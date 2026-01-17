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
    let input_path = args.input.ok_or("No input file specified. Use --help for usage.")?;

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
            println!("\nRunning analysis {}/{}: {:?}", idx + 1, netlist.analyses.len(), analysis);
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
            AnalysisCommand::Dc { source, start, stop, step } => {
                println!("Running DC sweep on {} from {} to {} by {}...", source, start, stop, step);
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
                                    let waveform: Vec<f64> = results.iter().map(|(_, r)| r.voltage(node + 1)).collect();
                                    node_waveforms.push(waveform);
                                }
                            }
                            let node_names: Vec<String> = (1..=node_waveforms.len()).map(|i| i.to_string()).collect();
                            let format = if args.format == "ascii" { 
                                rspice_core::analysis::RawFormat::Ascii 
                            } else { 
                                rspice_core::analysis::RawFormat::Binary 
                            };
                            rspice_core::analysis::export_dc_sweep(output_path, &sweep_vals, source, &node_names, &node_waveforms, format)?;
                            println!("Results exported to: {}", output_path.display());
                        }
                    }
                    Err(e) => {
                        eprintln!("DC sweep failed: {}", e);
                    }
                }
            }
            AnalysisCommand::Tran { step, stop, start, max_step: _ } => {
                let tstart = start.unwrap_or(0.0);
                
                // Progress indicator for transient simulation
                use indicatif::{ProgressBar, ProgressStyle};
                
                let pb = if args.quiet {
                    ProgressBar::hidden()
                } else if args.progress {
                    // Full progress bar with ETA
                    let eta_steps = ((stop - tstart) / step) as u64;
                    let pb = ProgressBar::new(eta_steps);
                    pb.set_style(ProgressStyle::default_bar()
                        .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} ({eta})")
                        .unwrap()
                        .progress_chars("#>-"));
                    pb.set_message(format!("Transient: {} to {}", tstart, stop));
                    pb
                } else {
                    // Simple spinner
                    let pb = ProgressBar::new_spinner();
                    pb.set_style(ProgressStyle::default_spinner()
                        .template("{spinner:.green} {msg}")
                        .unwrap());
                    pb.set_message(format!("Running transient: {} to {} (step {})...", tstart, stop, step));
                    pb.enable_steady_tick(std::time::Duration::from_millis(100));
                    pb
                };
                
                let result = engine.run_tran(&netlist, *stop, *step);
                pb.finish_and_clear();
                
                match result {
                    Ok(result) => {
                        if !args.quiet {
                            println!("✓ Transient complete: {} time points computed", result.time.len());
                        }
                        
                        // Export if output path specified
                        if let Some(ref output_path) = args.output {
                            let node_names: Vec<String> = (1..=result.voltages.len()).map(|i| i.to_string()).collect();
                            let format = if args.format == "ascii" { 
                                rspice_core::analysis::RawFormat::Ascii 
                            } else { 
                                rspice_core::analysis::RawFormat::Binary 
                            };
                            rspice_core::analysis::export_transient(output_path, &result.time, &node_names, &result.voltages, format)?;
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
            AnalysisCommand::Ac { variation: _, points, start_freq, stop_freq } => {
                if !args.quiet {
                    println!("Running AC analysis: {} to {} Hz ({} points)...", start_freq, stop_freq, points);
                }
                
                // Generate frequency points (decade sweep)
                let frequencies: Vec<f64> = (0..*points)
                    .map(|i| start_freq * (stop_freq / start_freq).powf(i as f64 / (*points as f64 - 1.0)))
                    .collect();
                
                match engine.run_ac(&netlist, &frequencies) {
                    Ok(results) => {
                        if !args.quiet {
                            println!("AC Analysis: {} frequency points", results.len());
                            if args.verbose && !results.is_empty() {
                                let first = &results[0];
                                let last = results.last().unwrap();
                                println!("  @ {:e} Hz: |V(1)| = {:.4}", first.frequency, first.voltage_magnitude(1));
                                println!("  @ {:e} Hz: |V(1)| = {:.4}", last.frequency, last.voltage_magnitude(1));
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("AC analysis failed: {}", e);
                    }
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

