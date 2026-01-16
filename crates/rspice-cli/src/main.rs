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
    
    log::info!("Title: {}", netlist.title);
    log::info!("Elements: {}", netlist.elements.len());
    log::info!("Analyses: {}", netlist.analyses.len());

    // TODO: Build circuit, run simulation, output results
    println!("Netlist '{}' parsed successfully.", netlist.title);
    println!("  {} element(s)", netlist.elements.len());
    println!("  {} analysis command(s)", netlist.analyses.len());
    
    for analysis in &netlist.analyses {
        println!("  Analysis: {:?}", analysis);
    }

    println!("\nSimulation engine under development...");

    Ok(())
}
