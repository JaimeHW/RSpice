use rspice_core::testing::{XyceRunnerConfig, XyceTestResult, XyceTestRunner};
use std::path::PathBuf;

fn main() {
    if let Err(err) = run() {
        eprintln!("Xyce case runner error: {err}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(std::env::args().skip(1))?;
    if args.config.verbose {
        StderrLogger::install();
    }
    let runner = XyceTestRunner::new(&args.test_dir, args.config);
    let result = runner.run_test(&args.circuit);
    std::fs::write(&args.result, encode_xyce_test_result(&result)).map_err(|err| {
        format!(
            "failed to write result file '{}': {err}",
            args.result.display()
        )
    })?;
    Ok(())
}

fn encode_xyce_test_result(result: &XyceTestResult) -> String {
    let mut output = String::new();
    push_field(&mut output, "name", &result.name);
    push_field(&mut output, "relative_path", &result.relative_path);
    push_field(&mut output, "contract", &result.contract);
    push_field(&mut output, "passed", result.passed);
    push_field(
        &mut output,
        "expected_unsupported",
        result.expected_unsupported,
    );
    push_field(&mut output, "duration_ms", result.duration_ms);
    push_field(
        &mut output,
        "error",
        result.error.as_deref().unwrap_or_default(),
    );
    push_field(&mut output, "mismatch_count", result.mismatches.len());
    for (index, mismatch) in result.mismatches.iter().enumerate() {
        push_field(
            &mut output,
            &format!("mismatch.{index}"),
            format!(
                "row={} probe={} expected={:.17e} actual={:.17e} relative_error={:.17e}",
                mismatch.row,
                escape_field(&mismatch.probe),
                mismatch.expected,
                mismatch.actual,
                mismatch.relative_error
            ),
        );
    }
    output
}

fn push_field(output: &mut String, name: &str, value: impl ToString) {
    output.push_str(name);
    output.push('=');
    output.push_str(&escape_field(&value.to_string()));
    output.push('\n');
}

fn escape_field(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
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
    config: XyceRunnerConfig,
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut test_dir = None;
        let mut circuit = None;
        let mut result = None;
        let mut config = XyceRunnerConfig::default();

        while let Some(flag) = args.next() {
            match flag.as_str() {
                "--test-dir" => test_dir = Some(next_path(&mut args, &flag)?),
                "--circuit" => circuit = Some(next_path(&mut args, &flag)?),
                "--result" => result = Some(next_path(&mut args, &flag)?),
                "--relative-tolerance" => config.relative_tolerance = next_parse(&mut args, &flag)?,
                "--absolute-tolerance" => config.absolute_tolerance = next_parse(&mut args, &flag)?,
                "--voltage-absolute-tolerance" => {
                    config.voltage_absolute_tolerance = next_parse(&mut args, &flag)?
                }
                "--max-mismatches" => config.max_mismatches = next_parse(&mut args, &flag)?,
                "--max-time-per-test-ms" => {
                    config.max_time_per_test_ms = next_parse(&mut args, &flag)?
                }
                "--verbose" => config.verbose = next_parse(&mut args, &flag)?,
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
