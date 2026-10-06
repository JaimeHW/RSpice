//! RSpice CLI - High-performance SPICE circuit simulator
//!
//! Usage:
//!   rspice run <netlist.sp>           Run the analyses a deck requests
//!   rspice health                     Probe backend liveness or readiness
//!   rspice info <netlist.sp>          Summarize a netlist without simulating
//!   rspice check <netlist.sp>         Validate syntax, topology, and outputs
//!   rspice compile-va <model.va>      Compile a Verilog-A model
//!   rspice convert <in> <out>         Convert between result formats
//!   rspice compare <result> <golden>  Golden-file regression check
//!   rspice completions <shell>        Emit a shell completion script
//!   rspice --help                     Show help
//!
//! See `README.md` for flags, exit codes, and configuration.

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
mod console;
mod hdf5;
mod observability;
mod report;

use cli::{Cli, Commands, Config};

/// Where the failure was authored, rendered for a human reading a terminal.
///
/// A diagnostic that says "line 41" only inside its own prose costs the
/// operator a search; the location belongs beside the message.
fn source_context(details: &cli::ErrorDetails) -> Option<String> {
    let line = details.line?;
    Some(match &details.path {
        Some(path) => format!("{path}:{line}"),
        None => format!("line {line}"),
    })
}

fn print_cli_error(error: &cli::CliError, format: cli::ErrorFormat) {
    let details = error.details();
    match format {
        cli::ErrorFormat::Text => {
            match source_context(&details) {
                Some(location) => eprintln!("Error: {location}: {error}"),
                None => eprintln!("Error: {error}"),
            }
            let mut identity = Vec::new();
            if let Some(analysis) = &details.analysis_id {
                identity.push(format!("analysis {analysis}"));
            }
            if let Some(coordinate) = &details.coordinate_id {
                identity.push(format!("run {coordinate}"));
            }
            if !identity.is_empty() {
                eprintln!("  in {}", identity.join(", "));
            }
            if let Some(capability) = details.capability {
                eprintln!("  unsupported capability: {capability}");
            }
            if let Some(suggestion) = error.suggestion() {
                eprintln!("Suggestion: {suggestion}");
            }
        }
        cli::ErrorFormat::Json => {
            let payload = serde_json::json!({
                // Version 2: the fatal diagnostic's categories are the engine's own,
                // several category strings changed with them, and the exit codes
                // they map to are new. Adding fields alone would not have needed
                // a bump; changing what a category means does.
                "schema_version": 2,
                "tool": {
                    "name": "rspice",
                    "version": env!("CARGO_PKG_VERSION"),
                    "target": env!("RSPICE_BUILD_TARGET"),
                    "profile": env!("RSPICE_BUILD_PROFILE"),
                    "commit": env!("RSPICE_BUILD_COMMIT"),
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
                    "analysis_id": details.analysis_id,
                    "coordinate_id": details.coordinate_id,
                    "capability": details.capability,
                    "line": details.line,
                    "path": details.path,
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
    let arguments: Vec<_> = std::env::args_os().collect();
    let cli = match Cli::try_parse_from(&arguments) {
        Ok(cli) => cli,
        Err(error) => {
            // Clap cannot produce the full CLI on a usage error. Still honor
            // the explicitly requested diagnostic format, up to `--`.
            let mut json = false;
            let mut arguments = arguments.iter().skip(1);
            while let Some(argument) = arguments.next() {
                if argument == "--" {
                    break;
                }
                if argument == "--error-format" {
                    json = arguments.next().is_some_and(|value| value == "json");
                } else if let Some(value) = argument
                    .to_str()
                    .and_then(|value| value.strip_prefix("--error-format="))
                {
                    json = value == "json";
                }
            }
            if !json
                || matches!(
                    error.kind(),
                    clap::error::ErrorKind::DisplayHelp | clap::error::ErrorKind::DisplayVersion
                )
            {
                error.exit();
            }
            let error = cli::CliError::InvalidArgument {
                message: error.to_string(),
                suggestion: Some("run rspice --help for available commands and options".into()),
            };
            print_cli_error(&error, cli::ErrorFormat::Json);
            return error.exit_code().into();
        }
    };
    let error_format = cli.error_format;
    observability::set_machine_diagnostics(error_format == cli::ErrorFormat::Json);

    // Quiet execution has no observable log records, so avoid constructing
    // env_logger's filters and formatter on the latency-sensitive batch path.
    // Machine-readable fatal diagnostics still obtain their run ID lazily.
    if !cli.quiet {
        let log_level = if cli.verbose {
            "debug"
        } else {
            cli.log_level.as_deref().unwrap_or("warn")
        };
        let log_format = if error_format == cli::ErrorFormat::Json {
            cli::LogFormat::Json
        } else {
            cli.log_format
        };
        observability::init(log_level, log_format);
    }

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
        Commands::Models(args) => commands::models(args, cli.verbose, cli.quiet),
        Commands::CompileVa(args) => commands::compile_va(args, &config, cli.verbose, cli.quiet),
        Commands::Check(args) => commands::check(args, &config, cli.verbose, cli.quiet),
        Commands::Convert(args) => commands::convert(args, &config, cli.verbose, cli.quiet),
        Commands::Completions(args) => {
            use clap::CommandFactory;
            // clap_complete treats writer failures as internal panics. Its
            // output is bounded by our fixed command tree, so generate it in
            // memory and publish through the same fallible path as reports.
            let mut script = Vec::new();
            clap_complete::generate(args.shell, &mut Cli::command(), "rspice", &mut script);
            console::bytes(&script)
        }
        Commands::Compare(args) => {
            let compare_args = commands::compare::CompareArgs {
                section: args.section,
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
            commands::compare::execute(compare_args, &config, cli.verbose, cli.quiet)
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
