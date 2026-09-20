//! Package-only release compile-time gate for generated Verilog-A models.
//!
//! A preparation build populates one isolated target directory. Every timed
//! sample then changes only the selected leaf crate's rustc metadata, forcing
//! that leaf to rebuild while retaining identical dependency artifacts. Cargo
//! JSON messages are audited so a dependency rebuild cannot silently pollute a
//! package-only timing.

use crate::error::BenchError;
use crate::generated_rust::authenticate_generated_bundle;
use crate::provenance::{self, HostInfo, ToolProvenance};
use crate::workspace_root;
use clap::{Args, ValueEnum};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};
use std::time::{Instant, SystemTime, UNIX_EPOCH};

const DEFAULT_GENERATED_ROOT: &str = "crates/rspice-veriloga-models";
const DEFAULT_TARGET_DIR: &str = "target/rspice-bench/generated-compile";
const MIN_SAMPLES: usize = 3;
const METHODOLOGY_VERSION: u32 = 1;
const MAX_FAILURE_OUTPUT_BYTES: usize = 32 * 1024;

/// Representative wide generated leaf packages maintained by the compile gate.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, ValueEnum)]
pub enum GeneratedCompileModel {
    /// BSIM-BULK, the LLVM loop-pass diagnostic model.
    #[value(name = "bsimbulk")]
    BsimBulk,
    /// BSIM-CMG, a second wide compact-model topology.
    #[value(name = "bsimcmg-va")]
    BsimCmgVa,
    /// HiSIM-HV, the largest cold package compile in the current catalog.
    #[value(name = "hisimhv-va")]
    HisimHvVa,
}

impl GeneratedCompileModel {
    const ALL: [Self; 3] = [Self::BsimBulk, Self::BsimCmgVa, Self::HisimHvVa];

    const fn package_name(self) -> &'static str {
        match self {
            Self::BsimBulk => "rspice-veriloga-model-bsimbulk",
            Self::BsimCmgVa => "rspice-veriloga-model-bsimcmg-va",
            Self::HisimHvVa => "rspice-veriloga-model-hisimhv-va",
        }
    }
}

/// Arguments for the `generated-compile` subcommand.
#[derive(Args, Debug)]
pub struct GeneratedCompileArgs {
    /// Models to compile. Repeatable; default is all maintained wide models.
    #[arg(long = "model", value_enum, value_name = "MODEL")]
    pub models: Vec<GeneratedCompileModel>,

    /// Compile the leaf package with its generated noise feature enabled.
    #[arg(long)]
    pub noise: bool,

    /// Timed package-only samples per model; must be at least three.
    #[arg(long, default_value_t = MIN_SAMPLES)]
    pub samples: usize,

    /// Cargo executable used for preparation and timed rustc invocations.
    #[arg(long, default_value = "cargo", value_name = "PATH")]
    pub cargo: PathBuf,

    /// Isolated Cargo target directory, relative to the workspace by default.
    #[arg(long, default_value = DEFAULT_TARGET_DIR, value_name = "PATH")]
    pub target_dir: PathBuf,

    /// Optional explicit Rust target triple.
    #[arg(long, value_name = "TRIPLE")]
    pub target: Option<String>,

    /// Generated bundle root whose authenticated digest enters the report.
    #[arg(long, default_value = DEFAULT_GENERATED_ROOT, value_name = "PATH")]
    pub generated_root: PathBuf,

    /// Existing generated-compile report to compare with this run.
    #[arg(long, value_name = "PATH")]
    pub baseline: Option<PathBuf>,

    /// Fail when a model median exceeds its baseline by this percentage.
    #[arg(long, requires = "baseline", value_name = "PERCENT")]
    pub max_regression_percent: Option<f64>,

    /// Permit baseline comparison across different host fingerprints.
    #[arg(long, requires = "baseline")]
    pub allow_host_mismatch: bool,

    /// Permit an untrusted debug or dirty-tree diagnostic measurement.
    #[arg(long)]
    pub exploratory: bool,

    /// Enable unstable rustc time-pass and LLVM time-trace diagnostics.
    #[arg(long)]
    pub llvm_diagnostics: bool,

    /// Directory for per-sample Cargo/rustc diagnostic logs.
    #[arg(long, requires = "llvm_diagnostics", value_name = "PATH")]
    pub diagnostics_dir: Option<PathBuf>,

    /// Optional immutable JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct GeneratedCompileReport {
    schema_version: u32,
    methodology_version: u32,
    generated_bundle_digest: String,
    generated_source_bytes: u64,
    generated_device_count: usize,
    generated_file_count: usize,
    cargo_version: String,
    target: String,
    profile: String,
    noise: bool,
    sample_count: usize,
    package_only_verified: bool,
    llvm_diagnostics: bool,
    models: Vec<ModelCompileReport>,
    comparisons: Vec<CompileComparison>,
    max_regression_percent: Option<f64>,
    passed: bool,
    failures: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct ModelCompileReport {
    package_name: String,
    preparation_ns: u64,
    samples_ns: Vec<u64>,
    minimum_ns: u64,
    median_ns: u64,
    maximum_ns: u64,
    rebuilt_packages_per_sample: Vec<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct CompileComparison {
    package_name: String,
    baseline_median_ns: u64,
    candidate_median_ns: u64,
    change_percent: f64,
}

#[derive(Debug, Deserialize)]
struct BaselineEnvelope {
    schema_version: u32,
    benchmark_id: String,
    trusted: bool,
    tool: BaselineTool,
    host: HostInfo,
    payload: GeneratedCompileReport,
}

#[derive(Debug, Deserialize)]
struct BaselineTool {
    profile: String,
    target: String,
    rustc: String,
}

#[derive(Debug, Deserialize)]
struct CargoMessage {
    reason: String,
    package_id: Option<String>,
    fresh: Option<bool>,
}

pub fn run(args: &GeneratedCompileArgs) -> Result<ExitCode, BenchError> {
    validate_args(args)?;
    let root = workspace_root();
    let generated_root = resolve_from_workspace(&root, &args.generated_root);
    let target_dir = resolve_from_workspace(&root, &args.target_dir);
    let diagnostics_dir = args
        .diagnostics_dir
        .as_ref()
        .map(|path| resolve_from_workspace(&root, path));
    if let Some(path) = &diagnostics_dir {
        fs::create_dir_all(path).map_err(|source| {
            BenchError::io(
                format!("create generated-compile diagnostics `{}`", path.display()),
                source,
            )
        })?;
    }

    let tool = provenance::tool();
    let host = provenance::host();
    require_trusted_environment(args, &tool)?;
    let bundle = authenticate_generated_bundle(&generated_root)?;
    let cargo_version = command_version(&args.cargo, "--version")?;
    let models = selected_models(args);
    let session = session_nonce();
    let mut measured = Vec::with_capacity(models.len());

    for (model_index, model) in models.iter().copied().enumerate() {
        let package = model.package_name();
        println!(
            "preparing {package} dependencies in {}",
            target_dir.display()
        );
        let preparation_start = Instant::now();
        let preparation = run_cargo(args, &root, &target_dir, package, CargoRun::Prepare, None)?;
        let preparation_ns = elapsed_ns(preparation_start.elapsed())?;
        require_success(package, "preparation", &preparation)?;

        let mut samples_ns = Vec::with_capacity(args.samples);
        let mut rebuilt_packages_per_sample = Vec::with_capacity(args.samples);
        for sample_index in 0..args.samples {
            let metadata = format!("rspice_bench_{session:x}_{model_index:x}_{sample_index:x}");
            let started = Instant::now();
            let output = run_cargo(
                args,
                &root,
                &target_dir,
                package,
                CargoRun::Timed,
                Some(&metadata),
            )?;
            let duration_ns = elapsed_ns(started.elapsed())?;
            if let Some(directory) = &diagnostics_dir {
                write_diagnostic_log(directory, package, sample_index, &output)?;
            }
            require_success(package, &format!("sample {}", sample_index + 1), &output)?;
            let rebuilt = rebuilt_packages(&output.stdout)?;
            verify_package_only_rebuild(package, &rebuilt, sample_index)?;
            println!(
                "  {package} sample {}/{}: {:.3} s",
                sample_index + 1,
                args.samples,
                duration_ns as f64 / 1_000_000_000.0
            );
            samples_ns.push(duration_ns);
            rebuilt_packages_per_sample.push(rebuilt.into_iter().collect());
        }
        let (minimum_ns, median_ns, maximum_ns) = distribution(&samples_ns)?;
        measured.push(ModelCompileReport {
            package_name: package.to_string(),
            preparation_ns,
            samples_ns,
            minimum_ns,
            median_ns,
            maximum_ns,
            rebuilt_packages_per_sample,
        });
    }

    let mut report = GeneratedCompileReport {
        schema_version: 1,
        methodology_version: METHODOLOGY_VERSION,
        generated_bundle_digest: bundle.digest,
        generated_source_bytes: bundle.source_bytes,
        generated_device_count: bundle.device_count,
        generated_file_count: bundle.file_count,
        cargo_version,
        target: args.target.clone().unwrap_or_else(|| {
            rustc_host_triple(&tool.rustc).unwrap_or_else(|| tool.target.clone())
        }),
        profile: "release".to_string(),
        noise: args.noise,
        sample_count: args.samples,
        package_only_verified: true,
        llvm_diagnostics: args.llvm_diagnostics,
        models: measured,
        comparisons: Vec::new(),
        max_regression_percent: args.max_regression_percent,
        passed: true,
        failures: Vec::new(),
    };
    if let Some(path) = &args.baseline {
        compare_baseline(args, &tool, &host, path, &mut report)?;
    }
    report.passed = report.failures.is_empty();
    print_report(&report);
    if let Some(path) = &args.out {
        crate::report::write(path, "rspice-generated-compile", &report, true)?;
    }
    Ok(if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

#[derive(Clone, Copy)]
enum CargoRun {
    Prepare,
    Timed,
}

fn run_cargo(
    args: &GeneratedCompileArgs,
    root: &Path,
    target_dir: &Path,
    package: &str,
    run: CargoRun,
    metadata: Option<&str>,
) -> Result<Output, BenchError> {
    let mut command = Command::new(&args.cargo);
    command
        .current_dir(root)
        .arg(match run {
            CargoRun::Prepare => "build",
            CargoRun::Timed => "rustc",
        })
        .args(["--locked", "--release", "-p", package, "--lib", "-j", "1"])
        .arg("--target-dir")
        .arg(target_dir);
    if args.noise {
        command.args(["--features", "veriloga-builtins-noise"]);
    }
    // Cargo fingerprints rustc-affecting environment for dependencies too.
    // Set this during the untimed preparation as well as timed diagnostic
    // samples so the first accepted sample cannot rebuild the dependency tree.
    if args.llvm_diagnostics {
        command.env("RUSTC_BOOTSTRAP", "1");
    }
    if let Some(target) = &args.target {
        command.args(["--target", target]);
    }
    if matches!(run, CargoRun::Timed) {
        command.arg("--message-format=json-render-diagnostics");
        command.arg("--");
        command.arg(format!("-Cmetadata={}", metadata.unwrap_or("rspice_bench")));
        if args.llvm_diagnostics {
            command.args(["-Ztime-passes", "-Zllvm-time-trace"]);
        }
    }
    command.output().map_err(|source| {
        BenchError::io(
            format!("execute Cargo for generated package `{package}`"),
            source,
        )
    })
}

fn rebuilt_packages(stdout: &[u8]) -> Result<BTreeSet<String>, BenchError> {
    let text = std::str::from_utf8(stdout).map_err(|error| BenchError::GeneratedCompile {
        message: format!("Cargo JSON output was not UTF-8: {error}"),
    })?;
    let mut rebuilt = BTreeSet::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let message: CargoMessage =
            serde_json::from_str(line).map_err(|error| BenchError::GeneratedCompile {
                message: format!("parse Cargo JSON message: {error}; line={line}"),
            })?;
        if message.reason == "compiler-artifact"
            && message.fresh == Some(false)
            && let Some(package_id) = message.package_id
        {
            rebuilt.insert(package_name_from_id(&package_id));
        }
    }
    Ok(rebuilt)
}

fn package_name_from_id(package_id: &str) -> String {
    if let Some(fragment) = package_id.rsplit('#').next()
        && let Some((name, _)) = fragment.rsplit_once('@')
    {
        return name.to_string();
    }
    if let Some((path, fragment)) = package_id.rsplit_once('#')
        && fragment
            .chars()
            .next()
            .is_some_and(|first| first.is_ascii_digit())
        && let Some(name) = path.rsplit('/').next()
        && !name.is_empty()
    {
        return name.to_string();
    }
    package_id
        .split_whitespace()
        .next()
        .unwrap_or(package_id)
        .to_string()
}

fn verify_package_only_rebuild(
    expected: &str,
    rebuilt: &BTreeSet<String>,
    sample_index: usize,
) -> Result<(), BenchError> {
    if rebuilt.len() == 1 && rebuilt.contains(expected) {
        return Ok(());
    }
    Err(BenchError::GeneratedCompile {
        message: format!(
            "{expected} sample {} was not package-only; rebuilt packages: {}",
            sample_index + 1,
            if rebuilt.is_empty() {
                "<none>".to_string()
            } else {
                rebuilt.iter().cloned().collect::<Vec<_>>().join(", ")
            }
        ),
    })
}

fn compare_baseline(
    args: &GeneratedCompileArgs,
    tool: &ToolProvenance,
    host: &HostInfo,
    path: &Path,
    report: &mut GeneratedCompileReport,
) -> Result<(), BenchError> {
    let bytes = fs::read(path).map_err(|source| {
        BenchError::io(
            format!("read generated-compile baseline `{}`", path.display()),
            source,
        )
    })?;
    let baseline: BaselineEnvelope =
        serde_json::from_slice(&bytes).map_err(|source| BenchError::Json {
            context: format!("parse generated-compile baseline `{}`", path.display()),
            source,
        })?;
    if baseline.schema_version != 1 || baseline.benchmark_id != "rspice-generated-compile" {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("`{}` is not a generated-compile v1 report", path.display()),
        });
    }
    if !args.exploratory && !baseline.trusted {
        return Err(BenchError::BenchmarkPolicy {
            message: "trusted comparison requires a trusted baseline report".to_string(),
        });
    }
    if !args.allow_host_mismatch && baseline.host.fingerprint != host.fingerprint {
        return Err(BenchError::BenchmarkPolicy {
            message: "generated-compile baseline host fingerprint differs; pass --allow-host-mismatch only for exploratory diagnosis".to_string(),
        });
    }
    if baseline.tool.profile != tool.profile
        || baseline.tool.target != tool.target
        || baseline.tool.rustc != tool.rustc
    {
        return Err(BenchError::BenchmarkPolicy {
            message: "generated-compile baseline tool profile, target, or rustc differs"
                .to_string(),
        });
    }
    if baseline.payload.methodology_version != METHODOLOGY_VERSION
        || baseline.payload.noise != report.noise
        || baseline.payload.target != report.target
        || baseline.payload.llvm_diagnostics != report.llvm_diagnostics
    {
        return Err(BenchError::BenchmarkPolicy {
            message: "generated-compile baseline methodology, target, noise mode, or diagnostic mode differs".to_string(),
        });
    }
    let baseline_models = baseline
        .payload
        .models
        .iter()
        .map(|model| (model.package_name.as_str(), model.median_ns))
        .collect::<BTreeMap<_, _>>();
    let candidate_names = report
        .models
        .iter()
        .map(|model| model.package_name.as_str())
        .collect::<BTreeSet<_>>();
    if baseline_models.keys().copied().collect::<BTreeSet<_>>() != candidate_names {
        return Err(BenchError::BenchmarkPolicy {
            message: "generated-compile baseline model selection differs".to_string(),
        });
    }
    for model in &report.models {
        let baseline_ns = baseline_models[model.package_name.as_str()];
        if baseline_ns == 0 {
            return Err(BenchError::BenchmarkPolicy {
                message: format!("baseline median for {} is zero", model.package_name),
            });
        }
        let change_percent = (model.median_ns as f64 / baseline_ns as f64 - 1.0) * 100.0;
        report.comparisons.push(CompileComparison {
            package_name: model.package_name.clone(),
            baseline_median_ns: baseline_ns,
            candidate_median_ns: model.median_ns,
            change_percent,
        });
        if let Some(limit) = args.max_regression_percent
            && change_percent > limit
        {
            report.failures.push(format!(
                "{} compile median regressed {change_percent:.2}%; budget is {limit:.2}%",
                model.package_name
            ));
        }
    }
    Ok(())
}

fn validate_args(args: &GeneratedCompileArgs) -> Result<(), BenchError> {
    if args.samples < MIN_SAMPLES {
        return Err(BenchError::BenchmarkPolicy {
            message: format!("--samples must be at least {MIN_SAMPLES}"),
        });
    }
    if args
        .max_regression_percent
        .is_some_and(|value| !value.is_finite() || value < 0.0)
    {
        return Err(BenchError::BenchmarkPolicy {
            message: "--max-regression-percent must be finite and non-negative".to_string(),
        });
    }
    if args.allow_host_mismatch && !args.exploratory {
        return Err(BenchError::BenchmarkPolicy {
            message: "--allow-host-mismatch requires --exploratory".to_string(),
        });
    }
    if args.llvm_diagnostics && !args.exploratory {
        return Err(BenchError::BenchmarkPolicy {
            message: "--llvm-diagnostics is diagnostic evidence and requires --exploratory"
                .to_string(),
        });
    }
    Ok(())
}

fn require_trusted_environment(
    args: &GeneratedCompileArgs,
    tool: &ToolProvenance,
) -> Result<(), BenchError> {
    if args.exploratory {
        return Ok(());
    }
    provenance::require_release(tool)?;
    if tool.git_commit.is_none() || tool.git_dirty != Some(false) {
        return Err(BenchError::BenchmarkPolicy {
            message: "trusted generated-compile runs require a clean Git worktree and commit; pass --exploratory for development measurements".to_string(),
        });
    }
    Ok(())
}

fn selected_models(args: &GeneratedCompileArgs) -> Vec<GeneratedCompileModel> {
    let mut models = if args.models.is_empty() {
        GeneratedCompileModel::ALL.to_vec()
    } else {
        args.models.clone()
    };
    models.sort_unstable();
    models.dedup();
    models
}

fn distribution(samples: &[u64]) -> Result<(u64, u64, u64), BenchError> {
    if samples.is_empty() {
        return Err(BenchError::Internal("compile distribution has no samples"));
    }
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let middle = sorted.len() / 2;
    let median = if sorted.len().is_multiple_of(2) {
        sorted[middle - 1]
            .checked_add(sorted[middle])
            .map(|sum| sum / 2)
            .unwrap_or_else(|| sorted[middle - 1] / 2 + sorted[middle] / 2)
    } else {
        sorted[middle]
    };
    Ok((sorted[0], median, sorted[sorted.len() - 1]))
}

fn print_report(report: &GeneratedCompileReport) {
    println!(
        "generated-compile bundle={} mode={} samples={} target={} [{}]",
        report.generated_bundle_digest,
        if report.noise { "noise" } else { "default" },
        report.sample_count,
        report.target,
        if report.passed { "ok" } else { "failed" }
    );
    for model in &report.models {
        println!(
            "  {:<40} min {:>9.3} s median {:>9.3} s max {:>9.3} s",
            model.package_name,
            model.minimum_ns as f64 / 1_000_000_000.0,
            model.median_ns as f64 / 1_000_000_000.0,
            model.maximum_ns as f64 / 1_000_000_000.0,
        );
    }
    for comparison in &report.comparisons {
        println!(
            "  comparison {:<29} {:+.2}%",
            comparison.package_name, comparison.change_percent
        );
    }
    for failure in &report.failures {
        eprintln!("FAIL {failure}");
    }
}

fn require_success(package: &str, phase: &str, output: &Output) -> Result<(), BenchError> {
    if output.status.success() {
        return Ok(());
    }
    Err(BenchError::GeneratedCompile {
        message: format!(
            "Cargo failed for `{package}` during {phase} ({}):\n{}",
            output.status,
            bounded_output(output)
        ),
    })
}

fn write_diagnostic_log(
    directory: &Path,
    package: &str,
    sample_index: usize,
    output: &Output,
) -> Result<(), BenchError> {
    let path = directory.join(format!("{package}-sample-{}.log", sample_index + 1));
    let mut bytes = output.stdout.clone();
    bytes.extend_from_slice(b"\n--- stderr ---\n");
    bytes.extend_from_slice(&output.stderr);
    fs::write(&path, bytes).map_err(|source| {
        BenchError::io(
            format!("write generated-compile diagnostic `{}`", path.display()),
            source,
        )
    })
}

fn bounded_output(output: &Output) -> String {
    let mut bytes = output.stdout.clone();
    bytes.extend_from_slice(&output.stderr);
    if bytes.len() > MAX_FAILURE_OUTPUT_BYTES {
        bytes = bytes.split_off(bytes.len() - MAX_FAILURE_OUTPUT_BYTES);
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

fn resolve_from_workspace(root: &Path, path: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        root.join(path)
    }
}

fn command_version(executable: &Path, argument: &str) -> Result<String, BenchError> {
    let output = Command::new(executable)
        .arg(argument)
        .output()
        .map_err(|source| BenchError::io("query Cargo version", source))?;
    if !output.status.success() {
        return Err(BenchError::GeneratedCompile {
            message: format!("Cargo version query failed: {}", bounded_output(&output)),
        });
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn rustc_host_triple(rustc_verbose: &str) -> Option<String> {
    rustc_verbose.lines().find_map(|line| {
        line.strip_prefix("host:")
            .map(str::trim)
            .filter(|host| !host.is_empty())
            .map(str::to_string)
    })
}

fn session_nonce() -> u128 {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    time ^ u128::from(std::process::id())
}

fn elapsed_ns(duration: std::time::Duration) -> Result<u64, BenchError> {
    u64::try_from(duration.as_nanos()).map_err(|_| BenchError::Internal("duration exceeds u64"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn package_ids_are_normalized_across_cargo_formats() {
        assert_eq!(
            package_name_from_id(
                "path+file:///workspace/models/bsimbulk#rspice-veriloga-model-bsimbulk@0.1.0"
            ),
            "rspice-veriloga-model-bsimbulk"
        );
        assert_eq!(
            package_name_from_id("rspice-veriloga-model-bsimbulk 0.1.0 (path+file:///workspace)"),
            "rspice-veriloga-model-bsimbulk"
        );
        assert_eq!(
            package_name_from_id("path+file:///C:/workspace/crates/rspice-veriloga-runtime#0.1.0"),
            "rspice-veriloga-runtime"
        );
    }

    #[test]
    fn cargo_artifacts_prove_a_package_only_rebuild() {
        let json = concat!(
            "{\"reason\":\"compiler-artifact\",\"package_id\":\"path+file:///workspace#rspice-veriloga-runtime@0.1.0\",\"fresh\":true}\n",
            "{\"reason\":\"compiler-artifact\",\"package_id\":\"path+file:///workspace#rspice-veriloga-model-bsimbulk@0.1.0\",\"fresh\":false}\n",
            "{\"reason\":\"build-finished\",\"success\":true}\n"
        );
        let rebuilt = rebuilt_packages(json.as_bytes()).expect("valid Cargo messages");
        verify_package_only_rebuild("rspice-veriloga-model-bsimbulk", &rebuilt, 0)
            .expect("only the selected leaf rebuilt");
    }

    #[test]
    fn distribution_reports_even_and_odd_medians() {
        assert_eq!(
            distribution(&[30, 10, 20]).expect("nonempty distribution"),
            (10, 20, 30)
        );
        assert_eq!(
            distribution(&[40, 10, 20, 30]).expect("nonempty distribution"),
            (10, 25, 40)
        );
    }

    #[test]
    fn default_model_set_is_complete_and_stable() {
        let args = GeneratedCompileArgs {
            models: Vec::new(),
            noise: false,
            samples: 3,
            cargo: PathBuf::from("cargo"),
            target_dir: PathBuf::from(DEFAULT_TARGET_DIR),
            target: None,
            generated_root: PathBuf::from(DEFAULT_GENERATED_ROOT),
            baseline: None,
            max_regression_percent: None,
            allow_host_mismatch: false,
            exploratory: true,
            llvm_diagnostics: false,
            diagnostics_dir: None,
            out: None,
        };
        assert_eq!(selected_models(&args), GeneratedCompileModel::ALL);
    }

    #[test]
    fn rustc_host_target_is_recorded_as_a_real_triple() {
        assert_eq!(
            rustc_host_triple("rustc 1.91.0\nhost: x86_64-pc-windows-msvc\nLLVM version: 20"),
            Some("x86_64-pc-windows-msvc".to_string())
        );
        assert_eq!(rustc_host_triple("rustc unavailable"), None);
    }
}
