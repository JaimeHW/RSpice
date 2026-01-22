//! CLI Argument Definitions
//!
//! Comprehensive argument structures for all subcommands using clap derive.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// RSpice - High-performance SPICE circuit simulator
#[derive(Parser, Debug)]
#[command(name = "rspice")]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose output (debug level logging)
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Suppress all non-error output
    #[arg(short, long, global = true)]
    pub quiet: bool,

    /// Configuration file path (default: .rspicerc)
    #[arg(long, global = true, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Set log level (error, warn, info, debug, trace)
    #[arg(long, global = true, value_name = "LEVEL")]
    pub log_level: Option<String>,
}

/// Available CLI subcommands
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Run a SPICE simulation
    Run(RunArgs),

    /// Display netlist information without simulating
    Info(InfoArgs),

    /// Compile a Verilog-A model
    #[command(name = "compile-va")]
    CompileVa(CompileVaArgs),

    /// Validate netlist syntax and connectivity
    Check(CheckArgs),

    /// Convert between output formats
    Convert(ConvertArgs),

    /// Compare results against golden file (regression testing)
    Compare(CompareArgs),
}

/// Arguments for the `run` subcommand
#[derive(Args, Debug)]
pub struct RunArgs {
    /// Input netlist file (.sp, .cir, .net, .spice)
    #[arg(value_name = "NETLIST")]
    pub input: PathBuf,

    /// Output file for simulation results
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Output format
    #[arg(short, long, default_value = "raw", value_name = "FORMAT")]
    pub format: OutputFormat,

    /// Run in batch mode (no interactive prompts)
    #[arg(short, long)]
    pub batch: bool,

    /// Override simulation temperature (Celsius)
    #[arg(long, value_name = "TEMP")]
    pub temp: Option<f64>,

    /// Print .MEAS measurement results
    #[arg(long)]
    pub meas: bool,

    /// Show progress bar with ETA for transient simulation
    #[arg(long)]
    pub progress: bool,

    /// Print node names in output (use original netlist names)
    #[arg(long)]
    pub node_names: bool,

    /// Enable waveform compression for long simulations
    #[arg(long)]
    pub compress: bool,

    /// Compression tolerance (default: 1e-4)
    #[arg(long, value_name = "TOL", requires = "compress")]
    pub compress_tol: Option<f64>,

    /// Maximum Newton-Raphson iterations
    #[arg(long, value_name = "N")]
    pub maxiter: Option<usize>,

    /// Convergence tolerance
    #[arg(long, value_name = "TOL")]
    pub abstol: Option<f64>,

    /// Relative tolerance
    #[arg(long, value_name = "TOL")]
    pub reltol: Option<f64>,

    /// Minimum timestep for transient analysis
    #[arg(long, value_name = "TIME")]
    pub min_step: Option<f64>,

    /// Maximum timestep for transient analysis
    #[arg(long, value_name = "TIME")]
    pub max_step: Option<f64>,

    /// Additional include paths for .include directives
    #[arg(short = 'I', long = "include", value_name = "DIR")]
    pub includes: Vec<PathBuf>,

    /// Define a parameter (can be used multiple times)
    #[arg(short = 'D', long = "define", value_name = "NAME=VALUE")]
    pub defines: Vec<String>,

    /// Number of Monte Carlo iterations (enables Monte Carlo mode)
    #[arg(long, value_name = "N")]
    pub monte_carlo: Option<usize>,

    /// Random seed for Monte Carlo analysis
    #[arg(long, value_name = "SEED", requires = "monte_carlo")]
    pub seed: Option<u64>,

    /// CI/CD report format (junit, tap)
    #[arg(long, value_name = "FORMAT")]
    pub report_format: Option<ReportFormat>,

    /// CI/CD report output file
    #[arg(long, value_name = "FILE", requires = "report_format")]
    pub report_file: Option<PathBuf>,

    /// Machine-readable .MEAS output format (json, csv)
    #[arg(long, value_name = "FORMAT")]
    pub meas_format: Option<MeasFormat>,

    /// .MEAS output file (defaults to JSON format if --meas-format not specified)
    #[arg(long, value_name = "FILE")]
    pub meas_file: Option<PathBuf>,

    // =========================================================================
    // Advanced RF/Analog Analysis Options
    // =========================================================================
    /// PSS (Periodic Steady-State) fundamental frequency in Hz
    #[arg(
        long,
        value_name = "FREQ",
        help = "Enable PSS analysis at specified frequency"
    )]
    pub pss_freq: Option<f64>,

    /// Number of harmonics for PSS analysis (default: 9)
    #[arg(long, value_name = "N", default_value = "9", requires = "pss_freq")]
    pub pss_harmonics: usize,

    /// PSS stabilization time before shooting method (default: auto)
    #[arg(long, value_name = "TIME", requires = "pss_freq")]
    pub pss_tstab: Option<f64>,

    /// HB (Harmonic Balance) fundamental frequency in Hz
    #[arg(
        long,
        value_name = "FREQ",
        help = "Enable HB analysis at specified frequency"
    )]
    pub hb_freq: Option<f64>,

    /// Number of harmonics for HB analysis (default: 9)
    #[arg(long, value_name = "N", default_value = "9", requires = "hb_freq")]
    pub hb_harmonics: usize,

    /// PZ (Pole-Zero) analysis input node
    #[arg(long, value_name = "NODE", help = "Input node for pole-zero analysis")]
    pub pz_input: Option<usize>,

    /// PZ (Pole-Zero) analysis output node
    #[arg(long, value_name = "NODE", requires = "pz_input")]
    pub pz_output: Option<usize>,

    /// Sensitivity analysis output node
    #[arg(
        long,
        value_name = "NODE",
        help = "Output node for sensitivity analysis"
    )]
    pub sens_output: Option<usize>,

    /// Sensitivity analysis parameter name
    #[arg(long, value_name = "PARAM", requires = "sens_output")]
    pub sens_param: Option<String>,

    /// Sensitivity analysis parameter nominal value
    #[arg(long, value_name = "VALUE", requires = "sens_param")]
    pub sens_value: Option<f64>,

    // =========================================================================
    // Process Corner Options
    // =========================================================================
    /// Process corners to simulate (comma-separated: tt,ss,ff,sf,fs)
    #[arg(
        long,
        value_name = "CORNERS",
        help = "Run simulation for each specified corner"
    )]
    pub corners: Option<String>,

    /// Library file containing corner definitions
    #[arg(long, value_name = "FILE", requires = "corners")]
    pub corner_lib: Option<PathBuf>,
}

/// Arguments for the `info` subcommand
#[derive(Args, Debug)]
pub struct InfoArgs {
    /// Input netlist file
    #[arg(value_name = "NETLIST")]
    pub input: PathBuf,

    /// Show detailed element information
    #[arg(short, long)]
    pub detailed: bool,

    /// Show model definitions
    #[arg(long)]
    pub models: bool,

    /// Show subcircuit hierarchy
    #[arg(long)]
    pub hierarchy: bool,

    /// Show parameter values
    #[arg(long)]
    pub params: bool,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Arguments for the `compile-va` subcommand
#[derive(Args, Debug)]
pub struct CompileVaArgs {
    /// Verilog-A source file
    #[arg(value_name = "FILE")]
    pub input: PathBuf,

    /// Output compiled model (optional, for caching)
    #[arg(short, long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Additional include directories
    #[arg(short = 'I', long = "include", value_name = "DIR")]
    pub includes: Vec<PathBuf>,

    /// Enable strict LRM compliance mode
    #[arg(long)]
    pub strict: bool,

    /// Show detailed compilation information
    #[arg(long)]
    pub detailed: bool,

    /// Generate usage example in output
    #[arg(long)]
    pub show_usage: bool,
}

/// Arguments for the `check` subcommand
#[derive(Args, Debug)]
pub struct CheckArgs {
    /// Input netlist file
    #[arg(value_name = "NETLIST")]
    pub input: PathBuf,

    /// Check connectivity (warn about floating nodes)
    #[arg(long)]
    pub connectivity: bool,

    /// Check for undefined models
    #[arg(long)]
    pub models: bool,

    /// Treat warnings as errors
    #[arg(long)]
    pub strict: bool,

    /// Output in JSON format
    #[arg(long)]
    pub json: bool,
}

/// Arguments for the `convert` subcommand
#[derive(Args, Debug)]
pub struct ConvertArgs {
    /// Input file
    #[arg(value_name = "INPUT")]
    pub input: PathBuf,

    /// Output file
    #[arg(value_name = "OUTPUT")]
    pub output: PathBuf,

    /// Input format (auto-detected if not specified)
    #[arg(long, value_name = "FORMAT")]
    pub from: Option<OutputFormat>,

    /// Output format (required)
    #[arg(long, value_name = "FORMAT")]
    pub to: OutputFormat,

    /// Variables to include (default: all)
    #[arg(long, value_name = "VAR")]
    pub variables: Vec<String>,

    /// Time/frequency range start
    #[arg(long, value_name = "VALUE")]
    pub start: Option<f64>,

    /// Time/frequency range end
    #[arg(long, value_name = "VALUE")]
    pub stop: Option<f64>,
}

/// Arguments for the `compare` subcommand
#[derive(Args, Debug)]
pub struct CompareArgs {
    /// Result file to compare
    #[arg(value_name = "RESULT")]
    pub result: PathBuf,

    /// Golden (reference) file
    #[arg(value_name = "GOLDEN")]
    pub golden: PathBuf,

    /// Absolute tolerance (default: 1e-9)
    #[arg(long, value_name = "TOL", default_value = "1e-9")]
    pub abstol: f64,

    /// Relative tolerance (default: 1e-6)
    #[arg(long, value_name = "TOL", default_value = "1e-6")]
    pub reltol: f64,

    /// Variables to compare (default: all)
    #[arg(long, value_name = "VAR")]
    pub variables: Vec<String>,

    /// Stop on first difference
    #[arg(long)]
    pub fail_fast: bool,

    /// Output differences as JSON
    #[arg(long)]
    pub json: bool,
}

/// CI/CD report formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum ReportFormat {
    /// JUnit XML format
    Junit,
    /// TAP (Test Anything Protocol)
    Tap,
}

/// Measurement output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum MeasFormat {
    /// JSON format
    Json,
    /// CSV format
    Csv,
}

/// Supported output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    /// SPICE raw format (binary)
    Raw,
    /// SPICE raw format (ASCII)
    #[value(name = "ascii")]
    RawAscii,
    /// Comma-separated values
    Csv,
    /// JSON format
    Json,
    /// Tab-separated values
    Tsv,
    /// HDF5 format (requires --features hdf5)
    Hdf5,
}

#[allow(dead_code)] // Reserved APIs for output format handling
impl OutputFormat {
    /// Get file extension for this format
    pub fn extension(&self) -> &'static str {
        match self {
            OutputFormat::Raw | OutputFormat::RawAscii => "raw",
            OutputFormat::Csv => "csv",
            OutputFormat::Json => "json",
            OutputFormat::Tsv => "tsv",
            OutputFormat::Hdf5 => "h5",
        }
    }

    /// Check if format is binary
    pub fn is_binary(&self) -> bool {
        matches!(self, OutputFormat::Raw | OutputFormat::Hdf5)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_run_basic() {
        let cli = Cli::try_parse_from(["rspice", "run", "test.sp"]).unwrap();
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.input, PathBuf::from("test.sp"));
                assert_eq!(args.format, OutputFormat::Raw);
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_parse_run_with_output() {
        let cli = Cli::try_parse_from(["rspice", "run", "test.sp", "-o", "out.raw", "-f", "csv"])
            .unwrap();
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.output, Some(PathBuf::from("out.raw")));
                assert_eq!(args.format, OutputFormat::Csv);
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_cli_parse_info() {
        let cli = Cli::try_parse_from(["rspice", "info", "test.sp", "--detailed"]).unwrap();
        match cli.command {
            Commands::Info(args) => {
                assert_eq!(args.input, PathBuf::from("test.sp"));
                assert!(args.detailed);
            }
            _ => panic!("Expected Info command"),
        }
    }

    #[test]
    fn test_cli_parse_compile_va() {
        let cli = Cli::try_parse_from([
            "rspice",
            "compile-va",
            "model.va",
            "-I",
            "/include",
            "--strict",
        ])
        .unwrap();
        match cli.command {
            Commands::CompileVa(args) => {
                assert_eq!(args.input, PathBuf::from("model.va"));
                assert_eq!(args.includes, vec![PathBuf::from("/include")]);
                assert!(args.strict);
            }
            _ => panic!("Expected CompileVa command"),
        }
    }

    #[test]
    fn test_cli_parse_check() {
        let cli = Cli::try_parse_from(["rspice", "check", "test.sp", "--connectivity"]).unwrap();
        match cli.command {
            Commands::Check(args) => {
                assert_eq!(args.input, PathBuf::from("test.sp"));
                assert!(args.connectivity);
            }
            _ => panic!("Expected Check command"),
        }
    }

    #[test]
    fn test_cli_parse_convert() {
        let cli =
            Cli::try_parse_from(["rspice", "convert", "in.raw", "out.csv", "--to", "csv"]).unwrap();
        match cli.command {
            Commands::Convert(args) => {
                assert_eq!(args.input, PathBuf::from("in.raw"));
                assert_eq!(args.output, PathBuf::from("out.csv"));
                assert_eq!(args.to, OutputFormat::Csv);
            }
            _ => panic!("Expected Convert command"),
        }
    }

    #[test]
    fn test_cli_global_flags() {
        let cli = Cli::try_parse_from(["rspice", "-v", "run", "test.sp"]).unwrap();
        assert!(cli.verbose);
        assert!(!cli.quiet);
    }

    #[test]
    fn test_output_format_extension() {
        assert_eq!(OutputFormat::Raw.extension(), "raw");
        assert_eq!(OutputFormat::Csv.extension(), "csv");
        assert_eq!(OutputFormat::Json.extension(), "json");
    }

    #[test]
    fn test_run_with_defines() {
        let cli =
            Cli::try_parse_from(["rspice", "run", "test.sp", "-D", "VDD=1.8", "-D", "TEMP=85"])
                .unwrap();
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.defines.len(), 2);
                assert_eq!(args.defines[0], "VDD=1.8");
                assert_eq!(args.defines[1], "TEMP=85");
            }
            _ => panic!("Expected Run command"),
        }
    }

    #[test]
    fn test_run_with_monte_carlo() {
        let cli = Cli::try_parse_from([
            "rspice",
            "run",
            "test.sp",
            "--monte-carlo",
            "1000",
            "--seed",
            "42",
        ])
        .unwrap();
        match cli.command {
            Commands::Run(args) => {
                assert_eq!(args.monte_carlo, Some(1000));
                assert_eq!(args.seed, Some(42));
            }
            _ => panic!("Expected Run command"),
        }
    }
}
