//! Resource-budget gate for the checked-in Verilog-A Rust bundle.
//!
//! Runtime benchmarks answer whether generated kernels are fast. This command
//! answers the two adjacent release questions: how much Rust source rustc must
//! ingest, and which models/files account for it. The report is deterministic
//! and authenticates every file against the generator manifest before applying
//! budgets.

use crate::error::BenchError;
use clap::Args;
use rspice_veriloga::rust_backend::{
    GeneratedBuiltinManifest, GeneratedBuiltinManifestFile, parse_generated_builtin_manifest,
};
use serde::Serialize;
use std::collections::BTreeMap;
use std::fs;
use std::num::NonZeroUsize;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_GENERATED_ROOT: &str = "crates/rspice-core/src/device/veriloga_generated";
const MANIFEST_FILE_NAME: &str = "manifest.txt";

/// Arguments for the `generated-rust` subcommand.
#[derive(Args, Debug)]
pub struct GeneratedRustArgs {
    /// Root containing the generated Rust bundle and its manifest.
    #[arg(long, default_value = DEFAULT_GENERATED_ROOT)]
    pub generated_root: PathBuf,

    /// Fail when total generated source exceeds this many bytes.
    #[arg(long, value_name = "BYTES")]
    pub max_source_bytes: Option<u64>,

    /// Fail when generated noise source exceeds this many bytes.
    #[arg(long, value_name = "BYTES")]
    pub max_noise_source_bytes: Option<u64>,

    /// Fail when any one model exceeds this many generated source bytes.
    #[arg(long, value_name = "BYTES")]
    pub max_model_source_bytes: Option<u64>,

    /// Fail when the bundle contains more than this many generated files.
    #[arg(long, value_name = "COUNT")]
    pub max_file_count: Option<usize>,

    /// Fail when any model's pooled workspace payload exceeds this many bytes per thread.
    #[arg(long, value_name = "BYTES")]
    pub max_pooled_workspace_payload_bytes: Option<u64>,

    /// Number of largest files and models retained in the report.
    #[arg(long, default_value = "20")]
    pub top: NonZeroUsize,

    /// Optional JSON report path.
    #[arg(long, value_name = "PATH")]
    pub out: Option<PathBuf>,
}

#[derive(Debug, Serialize)]
struct GeneratedRustReport {
    schema_version: u32,
    generated_root: String,
    bundle_digest: String,
    device_count: usize,
    file_count: usize,
    source_bytes: u64,
    source_lines: u64,
    noise_source_bytes: u64,
    max_retained_workspace_bytes_per_instance: u64,
    max_pooled_workspace_payload_bytes_per_thread: u64,
    max_legacy_dense_workspace_payload_bytes_per_instance: u64,
    categories: BTreeMap<String, SourceCategory>,
    largest_models: Vec<ModelSize>,
    largest_files: Vec<FileSize>,
    budgets: ResourceBudgets,
    passed: bool,
    failures: Vec<String>,
}

#[derive(Debug, Default, Serialize)]
struct SourceCategory {
    files: usize,
    bytes: u64,
    lines: u64,
}

#[derive(Debug, Serialize)]
struct ModelSize {
    module_name: String,
    public_model_name: String,
    backend: String,
    files: usize,
    bytes: u64,
    retained_workspace_bytes_per_instance: u64,
    pooled_workspace_payload_bytes_per_thread: u64,
    legacy_dense_workspace_payload_bytes_per_instance: u64,
}

#[derive(Debug, Serialize)]
struct FileSize {
    relative_path: String,
    category: String,
    bytes: u64,
    lines: u64,
}

#[derive(Debug, Serialize)]
struct ResourceBudgets {
    max_source_bytes: Option<u64>,
    max_noise_source_bytes: Option<u64>,
    max_model_source_bytes: Option<u64>,
    max_file_count: Option<usize>,
    max_pooled_workspace_payload_bytes: Option<u64>,
}

pub fn run(args: &GeneratedRustArgs) -> Result<ExitCode, BenchError> {
    let manifest_path = args.generated_root.join(MANIFEST_FILE_NAME);
    let manifest_text = fs::read_to_string(&manifest_path).map_err(|source| {
        BenchError::io(
            format!("read generated Rust manifest `{}`", manifest_path.display()),
            source,
        )
    })?;
    let manifest = parse_generated_builtin_manifest(&manifest_text).ok_or_else(|| {
        BenchError::GeneratedRust {
            message: format!(
                "`{}` is not a supported generated Rust manifest",
                manifest_path.display()
            ),
        }
    })?;
    let mut report = build_report(args, &manifest)?;
    apply_budgets(args, &mut report);
    print_report(&report);
    if let Some(path) = &args.out {
        write_report(path, &report)?;
    }
    Ok(if report.passed {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    })
}

fn build_report(
    args: &GeneratedRustArgs,
    manifest: &GeneratedBuiltinManifest,
) -> Result<GeneratedRustReport, BenchError> {
    let mut categories = BTreeMap::<String, SourceCategory>::new();
    let mut largest_files = Vec::with_capacity(manifest.files.len());
    let mut measured_bytes = 0u64;
    let mut source_lines = 0u64;
    let mut bundle_hasher = blake3::Hasher::new();

    for file in &manifest.files {
        let bytes = read_and_authenticate(&args.generated_root, file)?;
        let lines = count_lines(&bytes);
        let category = source_category(&file.relative_path);
        let summary = categories.entry(category.clone()).or_default();
        summary.files += 1;
        summary.bytes += file.bytes;
        summary.lines += lines;
        measured_bytes += file.bytes;
        source_lines += lines;
        update_digest_record(&mut bundle_hasher, file.relative_path.as_bytes());
        update_digest_record(&mut bundle_hasher, &bytes);
        largest_files.push(FileSize {
            relative_path: file.relative_path.clone(),
            category,
            bytes: file.bytes,
            lines,
        });
    }

    let measured_bundle_digest = bundle_hasher.finalize().to_hex().to_string();
    if measured_bytes != manifest.source_bytes || measured_bundle_digest != manifest.bundle_digest {
        return Err(BenchError::GeneratedRust {
            message: format!(
                "generated Rust bundle does not match its manifest: measured bytes={measured_bytes}, \
                 declared bytes={}, measured digest={measured_bundle_digest}, declared digest={}",
                manifest.source_bytes, manifest.bundle_digest
            ),
        });
    }

    largest_files.sort_unstable_by(|left, right| {
        right
            .bytes
            .cmp(&left.bytes)
            .then_with(|| left.relative_path.cmp(&right.relative_path))
    });
    largest_files.truncate(args.top.get());

    let mut largest_models = manifest
        .devices
        .iter()
        .map(|device| ModelSize {
            module_name: device.module_name.clone(),
            public_model_name: device.public_model_name.clone(),
            backend: format!("{:?}", device.backend),
            files: device.file_count,
            bytes: device.source_bytes,
            retained_workspace_bytes_per_instance: device
                .workspace
                .retained_workspace_bytes_per_instance,
            pooled_workspace_payload_bytes_per_thread: device
                .workspace
                .pooled_workspace_payload_bytes_per_thread,
            legacy_dense_workspace_payload_bytes_per_instance: device
                .workspace
                .legacy_dense_workspace_payload_bytes_per_instance,
        })
        .collect::<Vec<_>>();
    largest_models.sort_unstable_by(|left, right| {
        right
            .bytes
            .cmp(&left.bytes)
            .then_with(|| left.module_name.cmp(&right.module_name))
    });
    largest_models.truncate(args.top.get());

    let noise_source_bytes = categories.get("noise").map_or(0, |category| category.bytes);
    let max_retained_workspace_bytes_per_instance = manifest
        .devices
        .iter()
        .map(|device| device.workspace.retained_workspace_bytes_per_instance)
        .max()
        .unwrap_or(0);
    let max_pooled_workspace_payload_bytes_per_thread = manifest
        .devices
        .iter()
        .map(|device| device.workspace.pooled_workspace_payload_bytes_per_thread)
        .max()
        .unwrap_or(0);
    let max_legacy_dense_workspace_payload_bytes_per_instance = manifest
        .devices
        .iter()
        .map(|device| {
            device
                .workspace
                .legacy_dense_workspace_payload_bytes_per_instance
        })
        .max()
        .unwrap_or(0);
    Ok(GeneratedRustReport {
        schema_version: 1,
        generated_root: args.generated_root.display().to_string(),
        bundle_digest: manifest.bundle_digest.clone(),
        device_count: manifest.device_count,
        file_count: manifest.file_count,
        source_bytes: measured_bytes,
        source_lines,
        noise_source_bytes,
        max_retained_workspace_bytes_per_instance,
        max_pooled_workspace_payload_bytes_per_thread,
        max_legacy_dense_workspace_payload_bytes_per_instance,
        categories,
        largest_models,
        largest_files,
        budgets: ResourceBudgets {
            max_source_bytes: args.max_source_bytes,
            max_noise_source_bytes: args.max_noise_source_bytes,
            max_model_source_bytes: args.max_model_source_bytes,
            max_file_count: args.max_file_count,
            max_pooled_workspace_payload_bytes: args.max_pooled_workspace_payload_bytes,
        },
        passed: true,
        failures: Vec::new(),
    })
}

fn read_and_authenticate(
    generated_root: &Path,
    file: &GeneratedBuiltinManifestFile,
) -> Result<Vec<u8>, BenchError> {
    let path = generated_root.join(&file.relative_path);
    let bytes = fs::read(&path).map_err(|source| {
        BenchError::io(
            format!("read generated Rust source `{}`", path.display()),
            source,
        )
    })?;
    let digest = blake3::hash(&bytes).to_hex().to_string();
    if bytes.len() as u64 != file.bytes || digest != file.blake3 {
        return Err(BenchError::GeneratedRust {
            message: format!(
                "`{}` does not match the generated manifest: measured bytes={}, digest={digest}; \
                 declared bytes={}, digest={}",
                file.relative_path,
                bytes.len(),
                file.bytes,
                file.blake3
            ),
        });
    }
    Ok(bytes)
}

fn apply_budgets(args: &GeneratedRustArgs, report: &mut GeneratedRustReport) {
    check_u64_budget(
        "total generated source bytes",
        report.source_bytes,
        args.max_source_bytes,
        &mut report.failures,
    );
    check_u64_budget(
        "generated noise source bytes",
        report.noise_source_bytes,
        args.max_noise_source_bytes,
        &mut report.failures,
    );
    if let Some(limit) = args.max_model_source_bytes
        && let Some(model) = report.largest_models.first()
        && model.bytes > limit
    {
        report.failures.push(format!(
            "largest model '{}' contains {} generated bytes; budget is {limit}",
            model.module_name, model.bytes
        ));
    }
    if let Some(limit) = args.max_file_count
        && report.file_count > limit
    {
        report.failures.push(format!(
            "generated bundle contains {} files; budget is {limit}",
            report.file_count
        ));
    }
    if report.max_retained_workspace_bytes_per_instance != 0 {
        report.failures.push(format!(
            "generated instances retain up to {} scratch-workspace bytes; ABI v2 requires zero",
            report.max_retained_workspace_bytes_per_instance
        ));
    }
    check_u64_budget(
        "maximum pooled workspace payload bytes per thread",
        report.max_pooled_workspace_payload_bytes_per_thread,
        args.max_pooled_workspace_payload_bytes,
        &mut report.failures,
    );
    report.passed = report.failures.is_empty();
}

fn check_u64_budget(label: &str, measured: u64, limit: Option<u64>, failures: &mut Vec<String>) {
    if let Some(limit) = limit
        && measured > limit
    {
        failures.push(format!("{label} is {measured}; budget is {limit}"));
    }
}

fn print_report(report: &GeneratedRustReport) {
    println!(
        "generated-rust devices={} files={} source={} bytes lines={} noise={} bytes workspace-retained={} bytes workspace-pooled-max={} bytes legacy-dense-max={} bytes [{}]",
        report.device_count,
        report.file_count,
        report.source_bytes,
        report.source_lines,
        report.noise_source_bytes,
        report.max_retained_workspace_bytes_per_instance,
        report.max_pooled_workspace_payload_bytes_per_thread,
        report.max_legacy_dense_workspace_payload_bytes_per_instance,
        if report.passed { "ok" } else { "failed" }
    );
    for (name, category) in &report.categories {
        println!(
            "  {name:<18} {:>4} files {:>12} bytes {:>10} lines",
            category.files, category.bytes, category.lines
        );
    }
    if !report.largest_models.is_empty() {
        println!("  largest models:");
        for model in &report.largest_models {
            println!(
                "    {:<32} {:>12} source {:>10} pooled {:>10} legacy-dense {:>3} files {}",
                model.module_name,
                model.bytes,
                model.pooled_workspace_payload_bytes_per_thread,
                model.legacy_dense_workspace_payload_bytes_per_instance,
                model.files,
                model.backend
            );
        }
    }
    for failure in &report.failures {
        println!("  budget failure: {failure}");
    }
}

fn write_report(path: &Path, report: &GeneratedRustReport) -> Result<(), BenchError> {
    if let Some(parent) = path.parent()
        && !parent.as_os_str().is_empty()
    {
        fs::create_dir_all(parent).map_err(|source| {
            BenchError::io(
                format!("create generated Rust report dir `{}`", parent.display()),
                source,
            )
        })?;
    }
    let json = serde_json::to_string_pretty(report).map_err(|source| BenchError::Json {
        context: "serialize generated Rust resource report".into(),
        source,
    })?;
    fs::write(path, format!("{json}\n")).map_err(|source| {
        BenchError::io(
            format!("write generated Rust report `{}`", path.display()),
            source,
        )
    })
}

fn source_category(relative_path: &str) -> String {
    let file_name = relative_path.rsplit('/').next().unwrap_or(relative_path);
    if file_name.contains("noise") {
        "noise".to_string()
    } else if file_name.starts_with("stamp") {
        "stamp".to_string()
    } else if file_name == "kernel_runtime.rs" {
        "runtime".to_string()
    } else if file_name == "registry.rs" {
        "registry".to_string()
    } else if relative_path.contains('/') {
        "device-support".to_string()
    } else {
        "bundle-support".to_string()
    }
}

fn count_lines(bytes: &[u8]) -> u64 {
    if bytes.is_empty() {
        return 0;
    }
    let newlines = bytes.iter().filter(|&&byte| byte == b'\n').count() as u64;
    newlines + u64::from(bytes.last() != Some(&b'\n'))
}

fn update_digest_record(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_categories_are_stable() {
        assert_eq!(source_category("kernel_runtime.rs"), "runtime");
        assert_eq!(source_category("registry.rs"), "registry");
        assert_eq!(source_category("device/stamp_noise.rs"), "noise");
        assert_eq!(source_category("device/stamp_transient.rs"), "stamp");
        assert_eq!(source_category("device/state.rs"), "device-support");
    }

    #[test]
    fn line_count_handles_final_newline() {
        assert_eq!(count_lines(b""), 0);
        assert_eq!(count_lines(b"a"), 1);
        assert_eq!(count_lines(b"a\n"), 1);
        assert_eq!(count_lines(b"a\nb"), 2);
    }
}
