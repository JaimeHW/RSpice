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

/// mimalloc outperforms the Windows system heap substantially on the
/// many small allocations of parsing, netlist expansion, and result
/// recording. Scoped to the CLI binary: rspice-core stays
/// allocator-agnostic and the Python module must not override the
/// interpreter's allocator.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod abort;
mod cli;
mod commands;
mod hdf5;
mod observability;
mod report;

use cli::{Cli, Commands, Config};

fn print_cli_error(error: &cli::CliError, format: cli::ErrorFormat) {
    match format {
        cli::ErrorFormat::Text => {
            eprintln!("Error: {error}");
            if let Some(suggestion) = error.suggestion() {
                eprintln!("Suggestion: {suggestion}");
            }
        }
        cli::ErrorFormat::Json => {
            let details = error.details();
            let payload = serde_json::json!({
                "schema_version": 1,
                "tool": {
                    "name": "rspice",
                    "version": env!("CARGO_PKG_VERSION"),
                },
                "run_id": observability::run_id(),
                "error": {
                    "message": error.to_string(),
                    "code": details.code,
                    "category": details.category,
                    "retryable": details.retryable,
                    "exit_code": error.exit_code() as u8,
                    "suggestion": error.suggestion(),
                    "analysis": details.analysis,
                    "iterations": details.iterations,
                    "resource": details.resource,
                    "requested": details.requested,
                    "limit": details.limit,
                },
            });
            match serde_json::to_string(&payload) {
                Ok(json) => eprintln!("{json}"),
                Err(serialization_error) => {
                    eprintln!("Error: {error}");
                    eprintln!(
                        "Error: failed to serialize the machine-readable diagnostic: {serialization_error}"
                    );
                }
            }
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    let error_format = cli.error_format;

    // Initialize logging based on verbosity
    let log_level = if cli.verbose {
        "debug"
    } else if cli.quiet {
        "error"
    } else {
        cli.log_level.as_deref().unwrap_or("warn")
    };
    observability::init(log_level, cli.log_format);

    // Load configuration
    let config = if let Some(ref config_path) = cli.config {
        match Config::load_file(config_path) {
            Ok(c) => c,
            Err(e) => {
                let err = cli::CliError::ConfigError {
                    message: e.to_string(),
                };
                print_cli_error(&err, error_format);
                return err.exit_code().into();
            }
        }
    } else {
        match Config::load() {
            Ok(c) => c,
            Err(e) => {
                let err = cli::CliError::ConfigError {
                    message: e.to_string(),
                };
                print_cli_error(&err, error_format);
                return err.exit_code().into();
            }
        }
    };

    // Execute command
    let result: Result<(), cli::CliError> = match cli.command {
        Commands::Run(args) => commands::run(args, &config, cli.verbose, cli.quiet),
        Commands::Health(args) => commands::health(args, &config, cli.verbose, cli.quiet),
        Commands::Info(args) => commands::info(args, &config, cli.verbose, cli.quiet),
        Commands::CompileVa(args) => commands::compile_va(args, &config, cli.verbose, cli.quiet),
        Commands::Check(args) => commands::check(args, &config, cli.verbose, cli.quiet),
        Commands::Convert(args) => commands::convert(args, cli.verbose, cli.quiet),
        Commands::Completions(args) => {
            use clap::CommandFactory;
            clap_complete::generate(
                args.shell,
                &mut Cli::command(),
                "rspice",
                &mut std::io::stdout(),
            );
            Ok(())
        }
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
                allow_truncated: args.allow_truncated,
                ignore_missing: args.ignore_missing,
                bless: args.bless,
                interpolate: args.interpolate,
            };
            commands::compare::execute(compare_args, cli.verbose, cli.quiet)
        }
    };

    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(ref e) => {
            print_cli_error(e, error_format);
            e.exit_code().into()
        }
    }
}
