//! RSpice CLI - High-performance SPICE circuit simulator
//!
//! Usage:
//!   rspice run <netlist.sp>          Run simulation
//!   rspice info <netlist.sp>         Show netlist info
//!   rspice check <netlist.sp>        Validate netlist
//!   rspice compile-va <model.va>     Compile Verilog-A
//!   rspice convert <in> <out>        Convert formats
//!   rspice compare <result> <golden> Compare for CI/CD
//!   rspice --help                    Show help

use clap::Parser;
use std::process::ExitCode;

mod cli;
mod commands;
mod hdf5;
mod report;

use cli::{Cli, Commands, Config};

fn main() -> ExitCode {
    let cli = Cli::parse();

    // Initialize logging based on verbosity
    let log_level = if cli.verbose {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        cli.log_level.as_deref().unwrap_or("warn")
    };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    // Load configuration
    let config = if let Some(ref config_path) = cli.config {
        match Config::load_file(config_path) {
            Ok(c) => c,
            Err(e) => {
                let err = cli::CliError::ConfigError {
                    message: e.to_string(),
                };
                eprintln!("Error: {}", err);
                return err.exit_code().into();
            }
        }
    } else {
        Config::load()
    };

    // Execute command
    let result: Result<(), cli::CliError> = match cli.command {
        Commands::Run(args) => commands::run(args, &config, cli.verbose, cli.quiet),
        Commands::Info(args) => commands::info(args, cli.verbose, cli.quiet),
        Commands::CompileVa(args) => commands::compile_va(args, cli.verbose, cli.quiet),
        Commands::Check(args) => commands::check(args, cli.verbose, cli.quiet),
        Commands::Convert(args) => commands::convert(args, cli.verbose, cli.quiet),
        Commands::Compare(args) => {
            let compare_args = commands::compare::CompareArgs {
                result: args.result,
                golden: args.golden,
                abstol: args.abstol,
                reltol: args.reltol,
                format: if args.json {
                    cli::OutputFormat::Json
                } else {
                    cli::OutputFormat::Raw
                },
                variables: args.variables,
                fail_fast: args.fail_fast,
            };
            commands::compare::execute(compare_args, cli.verbose, cli.quiet)
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(ref e) => {
            eprintln!("Error: {}", e);
            if let Some(suggestion) = e.suggestion() {
                eprintln!("Suggestion: {}", suggestion);
            }
            e.exit_code().into()
        }
    }
}
