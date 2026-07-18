//! CLI Argument Definitions
//!
//! Comprehensive argument structures for all subcommands using clap derive.

use clap::{Args, Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

/// Parse a numeric flag value, accepting SPICE suffixes (`4.7k`, `1u`,
/// `100meg`) alongside plain and scientific notation.
pub fn spice_value(s: &str) -> Result<f64, String> {
    rspice_core::netlist::lexer::parse_spice_value(s).map_err(|e| e.to_string())
}

/// Format used for fatal diagnostics written to stderr.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum ErrorFormat {
    /// Human-readable diagnostics with a separate suggestion line.
    #[default]
    Text,
    /// One machine-readable JSON document.
    Json,
}

/// Format used for diagnostic log records.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum LogFormat {
    /// Standard `env_logger` text records.
    #[default]
    Text,
    /// Newline-delimited JSON records with source and process context.
    Json,
}

/// Depth of the backend health probe.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, ValueEnum)]
pub enum HealthMode {
    /// Verify process startup and engine configuration only.
    Liveness,
    /// Exercise bounded parsing, circuit construction, and a DC solve.
    #[default]
    Readiness,
}

impl HealthMode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Liveness => "liveness",
            Self::Readiness => "readiness",
        }
    }
}

/// Version string with build metadata for `--version`.
const LONG_VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("RSPICE_BUILD_TARGET"),
    ", ",
    env!("RSPICE_BUILD_PROFILE"),
    " build, commit ",
    env!("RSPICE_BUILD_COMMIT"),
    ")"
);

/// RSpice - High-performance SPICE circuit simulator
#[derive(Parser, Debug)]
#[command(name = "rspice")]
#[command(author, version, about, long_about = None)]
#[command(long_version = LONG_VERSION)]
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

    /// Set log level
    #[arg(
        long,
        global = true,
        value_name = "LEVEL",
        value_parser = ["off", "error", "warn", "info", "debug", "trace"]
    )]
    pub log_level: Option<String>,

    /// Format diagnostic log records on stderr
    #[arg(long, global = true, value_enum, default_value_t = LogFormat::Text)]
    pub log_format: LogFormat,

    /// Format fatal diagnostics on stderr for automation
    #[arg(long, global = true, value_enum, default_value_t = ErrorFormat::Text)]
    pub error_format: ErrorFormat,
}

/// Available CLI subcommands
#[derive(Subcommand, Debug)]
#[allow(clippy::large_enum_variant)]
pub enum Commands {
    /// Run a SPICE simulation
    Run(RunArgs),

    /// Probe backend liveness or numerical readiness
    Health(HealthArgs),

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

    /// Generate shell completions on stdout
    Completions(CompletionsArgs),
}

/// Arguments for the `completions` subcommand
#[derive(Args, Debug)]
pub struct CompletionsArgs {
    /// Target shell
    #[arg(value_name = "SHELL")]
    pub shell: clap_complete::Shell,
}

/// Arguments for the backend health probe.
#[derive(Args, Debug)]
pub struct HealthArgs {
    /// Probe depth
    #[arg(long, value_enum, default_value_t = HealthMode::Readiness)]
    pub mode: HealthMode,

    /// Emit a versioned JSON health document
    #[arg(long)]
    pub json: bool,
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

    /// Output format (default: config `output.format`, else `raw`)
    #[arg(short, long, value_name = "FORMAT")]
    pub format: Option<OutputFormat>,

    /// Override simulation temperature (Celsius)
    #[arg(long, value_name = "TEMP", value_parser = spice_value)]
    pub temp: Option<f64>,

    /// Print .MEAS measurement results
    #[arg(long)]
    pub meas: bool,

    /// Exit 0 even when .MEAS measurements fail (default: failed
    /// measurements exit with code 3)
    #[arg(long)]
    pub allow_failed_meas: bool,

    /// Export results even when they contain NaN/Inf (default: non-finite
    /// results are a simulation error)
    #[arg(long)]
    pub allow_nonfinite: bool,

    /// Abort the run after this many seconds (exit code 124). Transient
    /// and DC sweep analyses stop at the next safe point.
    #[arg(long, value_name = "SECONDS", value_parser = spice_value)]
    pub timeout: Option<f64>,

    /// Override the .TRAN stop time without editing the deck (checkpoint
    /// segments must keep identical source text)
    #[arg(long, value_name = "TIME", value_parser = spice_value)]
    pub tran_stop: Option<f64>,

    /// Save the transient integrator state to FILE when the run completes,
    /// for later --resume
    #[arg(long, value_name = "FILE")]
    pub checkpoint: Option<PathBuf>,

    /// Resume a transient from a checkpoint written by --checkpoint; the
    /// deck must be byte-identical to the one that wrote it
    #[arg(long, value_name = "FILE")]
    pub resume: Option<PathBuf>,

    /// Show progress bar with ETA for transient simulation
    #[arg(long)]
    pub progress: bool,

    /// Enable waveform compression for long simulations
    #[arg(long)]
    pub compress: bool,

    /// Compression tolerance (default: 1e-4)
    #[arg(long, value_name = "TOL", requires = "compress", value_parser = spice_value)]
    pub compress_tol: Option<f64>,

    /// Parallel workers for multi-run plans (.alter/.data variants) and
    /// corner sweeps; 0 = all cores. With more than one worker, per-run
    /// console output reduces to status lines (files and reports carry
    /// the data)
    #[arg(short = 'j', long, value_name = "N", default_value_t = 1)]
    pub jobs: usize,

    /// Maximum Newton-Raphson iterations
    #[arg(long, value_name = "N")]
    pub maxiter: Option<usize>,

    /// Convergence tolerance
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub abstol: Option<f64>,

    /// Relative tolerance
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub reltol: Option<f64>,

    /// Relative residual tolerance for equation convergence checks
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub residual_reltol: Option<f64>,

    /// Minimum timestep for transient analysis
    #[arg(long, value_name = "TIME", value_parser = spice_value)]
    pub min_step: Option<f64>,

    /// Maximum timestep for transient analysis
    #[arg(long, value_name = "TIME", value_parser = spice_value)]
    pub max_step: Option<f64>,

    /// DC convergence mode: fast, default, or robust
    #[arg(long, value_name = "MODE", value_parser = ["fast", "default", "robust"])]
    pub convergence: Option<String>,

    /// Transient integration method (default: trapgear auto-switching)
    #[arg(
        long,
        value_name = "METHOD",
        value_parser = ["trap", "gear", "trapgear", "euler"]
    )]
    pub integration_method: Option<String>,

    /// Transient truncation-error tolerance (TRTOL)
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub trtol: Option<f64>,

    /// Initial GMIN conductance for convergence aids
    #[arg(long, value_name = "G", value_parser = spice_value)]
    pub gmin: Option<f64>,

    /// Voltage absolute tolerance (VNTOL)
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub voltage_abstol: Option<f64>,

    /// Current absolute tolerance
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub current_abstol: Option<f64>,

    /// Charge absolute tolerance (CHGTOL)
    #[arg(long, value_name = "TOL", value_parser = spice_value)]
    pub charge_abstol: Option<f64>,

    /// Limit exported signals (repeatable; overrides the netlist
    /// .SAVE/.PROBE/.PRINT selection), e.g. --save "V(out)" --save "I(v1)"
    #[arg(long = "save", value_name = "SIGNAL")]
    pub saves: Vec<String>,

    /// Additional search directories for .include/.lib directives (repeatable)
    #[arg(short = 'I', long = "include", value_name = "DIR")]
    pub includes: Vec<PathBuf>,

    /// Override or define a netlist parameter (repeatable)
    #[arg(short = 'D', long = "define", value_name = "NAME=VALUE")]
    pub defines: Vec<String>,

    /// Number of Monte Carlo iterations (enables Monte Carlo mode)
    #[arg(long, value_name = "N")]
    pub monte_carlo: Option<usize>,

    /// Random seed for Monte Carlo analysis
    #[arg(long, value_name = "SEED", requires = "monte_carlo")]
    pub seed: Option<u64>,

    /// Monte Carlo parameter distribution (default: gaussian)
    #[arg(
        long,
        value_name = "DIST",
        requires = "monte_carlo",
        value_parser = ["gaussian", "uniform", "worst-case"]
    )]
    pub mc_distribution: Option<String>,

    /// Monte Carlo relative spread: non-negative sigma for gaussian,
    /// tolerance for uniform/worst-case (0 = deterministic nominal samples;
    /// default: 0.01 = 1%)
    #[arg(long, value_name = "SPREAD", requires = "monte_carlo", value_parser = spice_value)]
    pub mc_spread: Option<f64>,

    /// Restrict Monte Carlo variation to specific parameters (repeatable)
    #[arg(long = "mc-param", value_name = "PARAM", requires = "monte_carlo")]
    pub mc_params: Vec<String>,

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

    /// Write a JSON run summary (tool version, per-run status, measurements,
    /// verdict) to FILE, or to stdout with `-`
    #[arg(long, value_name = "FILE")]
    pub summary: Option<PathBuf>,

    // =========================================================================
    // Advanced RF/Analog Analysis Options
    // =========================================================================
    /// PSS (Periodic Steady-State) fundamental frequency in Hz
    #[arg(
        long,
        value_name = "FREQ",
        help = "Enable PSS analysis at specified frequency",
        value_parser = spice_value
    )]
    pub pss_freq: Option<f64>,

    /// Number of harmonics for PSS analysis (default: 9)
    #[arg(long, value_name = "N", default_value = "9", requires = "pss_freq")]
    pub pss_harmonics: usize,

    /// PSS stabilization time before shooting method (default: auto)
    #[arg(long, value_name = "TIME", requires = "pss_freq", value_parser = spice_value)]
    pub pss_tstab: Option<f64>,

    /// HB (Harmonic Balance) fundamental frequency in Hz
    #[arg(
        long,
        value_name = "FREQ",
        help = "Enable HB analysis at specified frequency",
        value_parser = spice_value
    )]
    pub hb_freq: Option<f64>,

    /// Number of harmonics for HB analysis (default: 9)
    #[arg(long, value_name = "N", default_value = "9", requires = "hb_freq")]
    pub hb_harmonics: usize,

    /// PZ (Pole-Zero) analysis input node (name or index)
    #[arg(long, value_name = "NODE", help = "Input node for pole-zero analysis")]
    pub pz_input: Option<String>,

    /// PZ (Pole-Zero) analysis output node (name or index)
    #[arg(long, value_name = "NODE", requires = "pz_input")]
    pub pz_output: Option<String>,

    /// Sensitivity analysis output node (name or index)
    #[arg(
        long,
        value_name = "NODE",
        help = "Output node for sensitivity analysis"
    )]
    pub sens_output: Option<String>,

    /// Sensitivity analysis parameter name
    #[arg(long, value_name = "PARAM", requires = "sens_output")]
    pub sens_param: Option<String>,

    /// Sensitivity analysis parameter nominal value
    #[arg(long, value_name = "VALUE", requires = "sens_param", value_parser = spice_value)]
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

    /// Two-port S-parameter extraction: four port nodes as
    /// "P1+,P1-,P2+,P2-" (use 0 for grounded references). Uses the deck's
    /// .AC card for the sweep; ports are driven through Z0 with the other
    /// port terminated in Z0
    #[arg(long, value_name = "NODES")]
    pub sparam: Option<String>,

    /// S-parameter reference impedance in ohms (default: 50)
    #[arg(long, value_name = "OHMS", requires = "sparam", value_parser = spice_value)]
    pub sparam_z0: Option<f64>,

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

    /// Write a JSON interface summary (model name, terminals, parameters)
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
    #[arg(long, value_name = "VALUE", value_parser = spice_value)]
    pub start: Option<f64>,

    /// Time/frequency range end
    #[arg(long, value_name = "VALUE", value_parser = spice_value)]
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
    #[arg(long, value_name = "TOL", default_value = "1e-9", value_parser = spice_value)]
    pub abstol: f64,

    /// Relative tolerance (default: 1e-6)
    #[arg(long, value_name = "TOL", default_value = "1e-6", value_parser = spice_value)]
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

    /// Tolerate point-count mismatches and compare only the overlap
    /// (default: a truncated result fails)
    #[arg(long)]
    pub allow_truncated: bool,

    /// Tolerate golden variables missing from the result
    /// (default: missing variables fail)
    #[arg(long)]
    pub ignore_missing: bool,

    /// Accept the result as the new reference: copy it over the golden
    /// file when they differ (or when the golden file does not exist yet)
    #[arg(long)]
    pub bless: bool,

    /// Linearly resample the result onto the golden file's scale before
    /// comparing, so runs with different time grids compare point-for-point
    /// (the result scale must cover the golden range)
    #[arg(long)]
    pub interpolate: bool,
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
    /// HDF5 format
    Hdf5,
}
