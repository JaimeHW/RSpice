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
                eprintln!("Error loading config: {}", e);
                return ExitCode::from(78); // CONFIG_ERROR
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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_cli_help() {
        let result = Cli::try_parse_from(["rspice", "run", "test.sp"]);
        assert!(result.is_ok());
    }

    #[test]
    fn test_cli_version() {
        let result = Cli::try_parse_from(["rspice", "--version"]);
        assert!(result.is_err());
    }

    #[test]
    fn test_run_command_parsing() {
        let cli = Cli::try_parse_from([
            "rspice",
            "run",
            "circuit.sp",
            "-o",
            "output.raw",
            "--temp",
            "85",
            "--meas",
        ])
        .unwrap();

        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input.to_str(), Some("circuit.sp"));
                assert_eq!(
                    args.output.as_ref().and_then(|p| p.to_str()),
                    Some("output.raw")
                );
                assert_eq!(args.temp, Some(85.0));
                assert!(args.meas);
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_info_command_parsing() {
        let cli = Cli::try_parse_from(["rspice", "info", "circuit.sp", "--detailed", "--models"])
            .unwrap();

        match cli.command {
            Commands::Info(args) => {
                assert!(args.detailed);
                assert!(args.models);
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_check_command_parsing() {
        let cli = Cli::try_parse_from([
            "rspice",
            "check",
            "circuit.sp",
            "--connectivity",
            "--strict",
        ])
        .unwrap();

        match cli.command {
            Commands::Check(args) => {
                assert!(args.connectivity);
                assert!(args.strict);
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_compare_command_parsing() {
        let cli = Cli::try_parse_from([
            "rspice",
            "compare",
            "result.csv",
            "golden.csv",
            "--abstol",
            "1e-6",
            "--json",
        ])
        .unwrap();

        match cli.command {
            Commands::Compare(args) => {
                assert_eq!(args.result.to_str(), Some("result.csv"));
                assert_eq!(args.golden.to_str(), Some("golden.csv"));
                assert!(args.json);
            }
            _ => panic!("Expected Compare command"),
        }
    }

    #[test]
    fn test_global_verbose_flag() {
        let cli = Cli::try_parse_from(["rspice", "-v", "run", "test.sp"]).unwrap();
        assert!(cli.verbose);
        assert!(!cli.quiet);
    }

    #[test]
    fn test_global_quiet_flag() {
        let cli = Cli::try_parse_from(["rspice", "-q", "run", "test.sp"]).unwrap();
        assert!(!cli.verbose);
        assert!(cli.quiet);
    }
}
