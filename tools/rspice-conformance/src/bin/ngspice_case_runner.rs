//! Runs a single ngspice conformance case in its own process.
//!
//! `tests/ngspice_regression.rs` shells out to this binary once per deck so
//! that a panic, hang, or stack overflow in one case cannot take the whole
//! suite down with it. The comparison itself lives in
//! [`rspice_conformance::suites::ngspice`]; this binary is only the process
//! boundary and the result-file protocol.
//!
//! The encoded [`TestResult`](rspice_conformance::suites::ngspice::TestResult)
//! is written to `--result` and decoded by the parent with
//! [`decode_test_result`](rspice_conformance::suites::ngspice::decode_test_result). A
//! missing or unparsable result file is how the parent detects a crash, so
//! this binary writes the file even for a failing case and reserves a
//! non-zero exit for argument and I/O errors.
//!
//! The parent passes `--max-time-per-test-ms` as a *soft* deadline set just
//! below its own hard watchdog, leaving this process a grace window to write
//! a real result before it is killed outright.
//!
//! ```text
//! rspice-ngspice-case-runner --test-dir DIR --circuit DECK --result FILE
//!                            [--relative-tolerance F] [--absolute-tolerance F]
//!                            [--max-mismatches N] [--skip-unsupported BOOL]
//!                            [--max-time-per-test-ms MS] [--verbose BOOL]
//! ```
//!
//! `--test-dir`, `--circuit`, and `--result` are required; the rest default
//! to [`TestRunnerConfig::default`].

use rspice_conformance::suites::ngspice::{TestRunner, TestRunnerConfig, encode_test_result};
use std::path::PathBuf;

fn main() {
    if let Err(err) = run() {
        eprintln!("ngspice case runner error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(std::env::args().skip(1))?;
    if args.config.verbose {
        StderrLogger::install();
    }
    let runner = TestRunner::new(&args.test_dir, args.config);
    let result = runner.run_test(&args.circuit);
    std::fs::write(&args.result, encode_test_result(&result)).map_err(|err| {
        format!(
            "failed to write result file '{}': {err}",
            args.result.display()
        )
    })?;
    Ok(())
}

struct StderrLogger;

impl StderrLogger {
    fn install() {
        static LOGGER: StderrLogger = StderrLogger;
        if log::set_logger(&LOGGER).is_ok() {
            log::set_max_level(log::LevelFilter::Info);
        }
    }
}

impl log::Log for StderrLogger {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= log::Level::Info
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            eprintln!("[{}] {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

struct Args {
    test_dir: PathBuf,
    circuit: PathBuf,
    result: PathBuf,
    config: TestRunnerConfig,
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut test_dir = None;
        let mut circuit = None;
        let mut result = None;
        let mut config = TestRunnerConfig::default();

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--test-dir" => test_dir = Some(next_path(&mut args, &flag)?),
                "--circuit" => circuit = Some(next_path(&mut args, &flag)?),
                "--result" => result = Some(next_path(&mut args, &flag)?),
                "--relative-tolerance" => config.relative_tolerance = next_parse(&mut args, &flag)?,
                "--absolute-tolerance" => config.absolute_tolerance = next_parse(&mut args, &flag)?,
                "--max-mismatches" => config.max_mismatches = next_parse(&mut args, &flag)?,
                "--skip-unsupported" => config.skip_unsupported = next_parse(&mut args, &flag)?,
                "--verbose" => config.verbose = next_parse(&mut args, &flag)?,
                "--max-time-per-test-ms" => {
                    config.max_time_per_test_ms = next_parse(&mut args, &flag)?
                }
                _ => return Err(format!("unknown argument '{flag}'")),
            }
        }

        Ok(Self {
            test_dir: test_dir.ok_or_else(|| "missing --test-dir".to_string())?,
            circuit: circuit.ok_or_else(|| "missing --circuit".to_string())?,
            result: result.ok_or_else(|| "missing --result".to_string())?,
            config,
        })
    }
}

fn next_path<I>(args: &mut I, flag: &str) -> Result<PathBuf, String>
where
    I: Iterator<Item = String>,
{
    args.next()
        .map(PathBuf::from)
        .ok_or_else(|| format!("missing value for {flag}"))
}

fn next_parse<I, T>(args: &mut I, flag: &str) -> Result<T, String>
where
    I: Iterator<Item = String>,
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    args.next()
        .ok_or_else(|| format!("missing value for {flag}"))?
        .parse::<T>()
        .map_err(|err| format!("invalid value for {flag}: {err}"))
}
