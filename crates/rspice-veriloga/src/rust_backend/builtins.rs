//! Whole-corpus generation: discover, compile, emit, write, manifest.
//!
//! The orchestration layer behind `rspice-veriloga-gen`. It walks a model
//! tree, compiles each module to canonical IR, emits a device through the best
//! available backend tier, writes the result, and records what happened in
//! `manifest.txt`.
//!
//! Two properties matter more than speed here. Generation is *complete*: a
//! full regeneration rewrites every device plus `registry.rs` and the
//! manifest, which is why subset generation is a separate entry point that
//! refuses to touch either. And it is *accountable*: every device that could
//! not take the scalar backend surfaces a [`BuiltinBackendFallbackReason`],
//! and `REQUIRE_SCALAR_BUILTINS_ENV` promotes those to errors so a silent
//! regression off the fast tier cannot land.
//!
//! Both of those describe the tier cascade, and the corpus no longer goes
//! through it: [`builtin_transpiler`] selects the canonical CFG emitter
//! outright.

use std::collections::VecDeque;
use std::env;
use std::fmt::Write;
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use web_time::Instant;

use crate::{CompilerOptions, VerilogACompiler};

use super::{
    GENERATED_BUILTIN_MANIFEST_SCHEMA_VERSION, GeneratedBuiltinManifest,
    GeneratedBuiltinManifestDevice, GeneratedBuiltinManifestFile,
    GeneratedBuiltinWorkspaceResources, GeneratedRustDevice, RustBackendSelection, RustTranspiler,
    VERILOGA_DISCOVERY_SKIP_MARKER, VerilogACompileProfile, VerilogASourceCandidate,
    cleanup_stale_generated_device_folders, discover_veriloga_sources,
    parse_generated_builtin_manifest, render_generated_builtin_manifest,
    resolve_generated_registry_model_names, write_generated_device, write_text_file_if_changed,
};

type BuiltinResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const GENERATED_BUILTIN_MANIFEST_FILE_NAME: &str = "manifest.txt";
pub const REGENERATE_BUILTINS_COMMAND: &str = "cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins";
pub const REQUIRE_SCALAR_BUILTINS_ENV: &str = "RSPICE_RUST_BACKEND_REQUIRE_SCALAR_BUILTINS";

const GENERATOR_SOURCE_DIGEST_INPUTS: &[&str] = &[
    "../../Cargo.toml",
    "../../Cargo.lock",
    "build.rs",
    "Cargo.toml",
    "src/lib.rs",
    "src/ast.rs",
    "src/canonical_ir",
    "src/codegen",
    "src/disciplines.rs",
    "src/error.rs",
    "src/expr_converter.rs",
    "src/ir.rs",
    "src/laplace.rs",
    "src/lexer.rs",
    "src/parser",
    "src/preprocessor.rs",
    "src/rust_backend",
    "src/semantic",
    "src/semantic.rs",
    "src/source.rs",
    "src/stdlib.rs",
    "src/types.rs",
    "src/zfilter.rs",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinGenerationReport {
    pub manifest: GeneratedBuiltinManifest,
    pub backend_counts: BuiltinBackendSelectionCounts,
    pub fallback_reasons: Vec<BuiltinBackendFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinSubsetGenerationReport {
    pub device_count: usize,
    pub backend_counts: BuiltinBackendSelectionCounts,
    pub fallback_reasons: Vec<BuiltinBackendFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuiltinBackendFallbackReason {
    pub source: PathBuf,
    pub module: String,
    pub backend: RustBackendSelection,
    pub reason: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BuiltinBackendSelectionCounts {
    pub scalar: usize,
    pub sparse_local: usize,
    pub structured: usize,
    pub hybrid: usize,
    pub legacy_device: usize,
    pub canonical: usize,
}

impl BuiltinBackendSelectionCounts {
    fn record(&mut self, selection: RustBackendSelection) {
        match selection {
            RustBackendSelection::ScalarOptIr => self.scalar += 1,
            RustBackendSelection::SparseLocalKernel => self.sparse_local += 1,
            RustBackendSelection::StructuredKernel => self.structured += 1,
            RustBackendSelection::ScalarHybrid => self.hybrid += 1,
            RustBackendSelection::LegacyDevice => self.legacy_device += 1,
            RustBackendSelection::CanonicalCfg => self.canonical += 1,
        }
    }
}

pub fn regenerate_generated_builtins(
    model_root: &Path,
    generated_root: &Path,
    generator_root: &Path,
) -> BuiltinResult<BuiltinGenerationReport> {
    regenerate_generated_builtins_with_progress(model_root, generated_root, generator_root, false)
}

pub fn regenerate_generated_builtins_with_progress(
    model_root: &Path,
    generated_root: &Path,
    generator_root: &Path,
    progress: bool,
) -> BuiltinResult<BuiltinGenerationReport> {
    regenerate_generated_builtins_with_progress_and_jobs(
        model_root,
        generated_root,
        generator_root,
        progress,
        None,
    )
}

pub fn regenerate_generated_builtins_with_progress_and_jobs(
    model_root: &Path,
    generated_root: &Path,
    generator_root: &Path,
    progress: bool,
    jobs: Option<usize>,
) -> BuiltinResult<BuiltinGenerationReport> {
    validate_model_root(model_root)?;

    let source_tree_digest = tree_digest(model_root, false)?;
    let generator_digest = generator_digest(generator_root, false)?;
    let (devices, backend_counts, fallback_reasons, manifest_devices) =
        generate_devices_with_stack(model_root.to_path_buf(), None, progress, jobs)?;
    reject_legacy_backend_selections(&backend_counts, &fallback_reasons)?;
    reject_non_scalar_backend_selections_if_requested(&backend_counts, &fallback_reasons)?;
    if devices.is_empty() {
        return Err(format!(
            "Verilog-A built-ins source directory '{}' does not contain any discovered modules",
            model_root.display()
        )
        .into());
    }

    reject_legacy_ad_runtime(&devices)?;
    write_devices(generated_root, &devices)?;
    write_kernel_runtime(generated_root)?;
    write_registry(generated_root, &devices)?;

    let manifest = build_generated_manifest(
        generated_root,
        source_tree_digest,
        generator_digest,
        manifest_devices,
    )?;
    write_generated_manifest(generated_root, &manifest)?;

    Ok(BuiltinGenerationReport {
        manifest,
        backend_counts,
        fallback_reasons,
    })
}

pub fn generate_generated_builtin_subset_with_progress(
    model_root: &Path,
    generated_root: &Path,
    filter: &str,
    progress: bool,
) -> BuiltinResult<BuiltinSubsetGenerationReport> {
    generate_generated_builtin_subset_with_progress_and_jobs(
        model_root,
        generated_root,
        filter,
        progress,
        None,
    )
}

pub fn generate_generated_builtin_subset_with_progress_and_jobs(
    model_root: &Path,
    generated_root: &Path,
    filter: &str,
    progress: bool,
    jobs: Option<usize>,
) -> BuiltinResult<BuiltinSubsetGenerationReport> {
    validate_model_root(model_root)?;

    let (devices, backend_counts, fallback_reasons, _) = generate_devices_with_stack(
        model_root.to_path_buf(),
        Some(filter.to_string()),
        progress,
        jobs,
    )?;
    reject_legacy_backend_selections(&backend_counts, &fallback_reasons)?;
    reject_non_scalar_backend_selections_if_requested(&backend_counts, &fallback_reasons)?;
    if devices.is_empty() {
        return Err(format!(
            "Verilog-A built-ins source directory '{}' does not contain any modules matching filter '{filter}'",
            model_root.display()
        )
        .into());
    }

    reject_legacy_ad_runtime(&devices)?;
    write_device_subset(generated_root, &devices)?;
    write_kernel_runtime(generated_root)?;

    Ok(BuiltinSubsetGenerationReport {
        device_count: devices.len(),
        backend_counts,
        fallback_reasons,
    })
}

pub fn validate_generated_builtins(
    model_root: &Path,
    generated_root: &Path,
    generator_root: &Path,
    emit_cargo_rerun: bool,
) -> BuiltinResult<GeneratedBuiltinManifest> {
    validate_model_root(model_root)?;

    let source_tree_digest = tree_digest(model_root, emit_cargo_rerun)?;
    let generator_digest = generator_digest(generator_root, emit_cargo_rerun)?;
    if let Some(manifest) =
        read_generated_manifest(generated_root, &source_tree_digest, &generator_digest)
    {
        validate_generated_manifest_outputs(generated_root, &manifest)?;
        reject_legacy_ad_runtime_files(generated_root)?;
        return Ok(manifest);
    }

    Err(
        stale_generated_builtins_error(generated_root, &source_tree_digest, &generator_digest)
            .into(),
    )
}

fn validate_model_root(model_root: &Path) -> BuiltinResult<()> {
    if !model_root.exists() {
        return Err(format!(
            "Verilog-A built-ins source directory '{}' does not exist",
            model_root.display()
        )
        .into());
    }
    if !model_root.is_dir() {
        return Err(format!(
            "Verilog-A built-ins source path '{}' is not a directory",
            model_root.display()
        )
        .into());
    }
    Ok(())
}

fn stale_generated_builtins_error(
    generated_root: &Path,
    source_tree_digest: &str,
    generator_digest: &str,
) -> String {
    let manifest_path = generated_root.join(GENERATED_BUILTIN_MANIFEST_FILE_NAME);
    let current = fs::read_to_string(&manifest_path)
        .ok()
        .and_then(|text| parse_generated_builtin_manifest(&text))
        .map(|manifest| {
            format!(
                "current manifest has schema_version={}, source_tree_digest={}, generator_digest={}, bundle_digest={}, device_count={}, file_count={}, source_bytes={}",
                manifest.schema_version,
                manifest.source_tree_digest,
                manifest.generator_digest,
                manifest.bundle_digest,
                manifest.device_count,
                manifest.file_count,
                manifest.source_bytes,
            )
        })
        .unwrap_or_else(|| "current manifest is missing or invalid".to_string());

    format!(
        "generated Verilog-A built-ins are stale; expected source_tree_digest={source_tree_digest}, generator_digest={generator_digest}; {current}. Run `{REGENERATE_BUILTINS_COMMAND}` before building rspice-core with the veriloga-builtins feature."
    )
}

fn generator_digest(generator_root: &Path, emit_cargo_rerun: bool) -> BuiltinResult<String> {
    let mut input = String::new();
    for relative in GENERATOR_SOURCE_DIGEST_INPUTS {
        let path = generator_root.join(relative);
        let digest = if path.is_dir() {
            tree_digest(&path, emit_cargo_rerun)?
        } else if path.is_file() {
            if emit_cargo_rerun {
                println!("cargo:rerun-if-changed={}", path.display());
            }
            file_digest(&path)?
        } else {
            return Err(format!(
                "Verilog-A generator digest input '{}' does not exist",
                path.display()
            )
            .into());
        };
        input.push_str(relative);
        input.push('\0');
        input.push_str(&digest);
        input.push('\0');
    }
    Ok(blake3::hash(input.as_bytes()).to_hex().to_string())
}

fn file_digest(path: &Path) -> BuiltinResult<String> {
    let bytes = fs::read(path)?;
    let text = String::from_utf8(bytes)?;
    let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
    let mut input = String::new();
    input.push_str(&normalized.len().to_string());
    input.push('\0');
    input.push_str(&normalized);
    input.push('\0');
    Ok(blake3::hash(input.as_bytes()).to_hex().to_string())
}

fn read_generated_manifest(
    generated_root: &Path,
    source_tree_digest: &str,
    generator_digest: &str,
) -> Option<GeneratedBuiltinManifest> {
    if !generated_root.join("registry.rs").is_file() {
        return None;
    }
    let manifest_path = generated_root.join(GENERATED_BUILTIN_MANIFEST_FILE_NAME);
    let manifest = parse_generated_builtin_manifest(&fs::read_to_string(manifest_path).ok()?)?;
    (manifest.schema_version == GENERATED_BUILTIN_MANIFEST_SCHEMA_VERSION
        && manifest.source_tree_digest == source_tree_digest
        && manifest.generator_digest == generator_digest
        && manifest.device_count > 0
        && manifest.device_count == manifest.devices.len()
        && manifest.file_count == manifest.files.len())
    .then_some(manifest)
}

fn reject_legacy_ad_runtime(devices: &[GeneratedRustDevice]) -> BuiltinResult<()> {
    const MARKERS: &[&str] = LEGACY_AD_RUNTIME_MARKERS;

    for device in devices {
        for file in &device.files {
            if let Some(marker) = MARKERS
                .iter()
                .copied()
                .find(|marker| file.contents.contains(marker))
            {
                return Err(format!(
                    "{}:{} contains legacy Verilog-A AD marker {marker}",
                    device.folder_name, file.relative_path,
                )
                .into());
            }
        }
    }
    Ok(())
}

fn reject_legacy_ad_runtime_files(generated_root: &Path) -> BuiltinResult<()> {
    if generated_root.join("support.rs").is_file() {
        return Err(format!(
            "generated Verilog-A built-ins contain stale legacy support module '{}'",
            generated_root.join("support.rs").display()
        )
        .into());
    }
    if !generated_root.join("kernel_runtime.rs").is_file() {
        return Err(format!(
            "generated Verilog-A built-ins are missing structured kernel runtime '{}'",
            generated_root.join("kernel_runtime.rs").display()
        )
        .into());
    }

    let mut files = Vec::new();
    collect_tree_files(generated_root, &mut files)?;
    for path in files {
        let contents = fs::read_to_string(&path)?;
        if let Some(marker) = LEGACY_AD_RUNTIME_MARKERS
            .iter()
            .copied()
            .find(|marker| contents.contains(marker))
        {
            return Err(format!(
                "generated Verilog-A built-ins contain legacy AD marker {marker} in '{}'",
                path.display()
            )
            .into());
        }
    }
    Ok(())
}

const LEGACY_AD_RUNTIME_MARKERS: &[&str] = &[
    "GenericAdValue",
    "GenericScratch",
    "GenericReactiveScratch",
    "::support::",
];

fn write_devices(generated_root: &Path, devices: &[GeneratedRustDevice]) -> BuiltinResult<()> {
    cleanup_stale_generated_device_folders(
        generated_root,
        devices.iter().map(|device| device.folder_name.as_str()),
    )?;
    for device in devices {
        write_generated_device(generated_root, device)?;
    }
    Ok(())
}

fn write_device_subset(
    generated_root: &Path,
    devices: &[GeneratedRustDevice],
) -> BuiltinResult<()> {
    std::fs::create_dir_all(generated_root)?;
    for device in devices {
        write_generated_device(generated_root, device)?;
    }
    Ok(())
}

fn write_kernel_runtime(generated_root: &Path) -> BuiltinResult<()> {
    let mut generated_files = Vec::new();
    collect_tree_files(generated_root, &mut generated_files)?;
    let mut generated_sources = Vec::new();
    for path in generated_files {
        if path.extension().and_then(|extension| extension.to_str()) != Some("rs")
            || path.parent() == Some(generated_root)
        {
            continue;
        }
        generated_sources.push(fs::read_to_string(path)?);
    }
    let runtime = super::device::render_runtime_support_module_for_generated_sources(
        generated_sources.iter().map(String::as_str),
    );
    write_text_file_if_changed(generated_root.join("kernel_runtime.rs"), &runtime)?;
    let support = generated_root.join("support.rs");
    if support.is_file() {
        fs::remove_file(support)?;
    }
    Ok(())
}

fn write_generated_manifest(
    generated_root: &Path,
    manifest: &GeneratedBuiltinManifest,
) -> BuiltinResult<()> {
    write_text_file_if_changed(
        generated_root.join(GENERATED_BUILTIN_MANIFEST_FILE_NAME),
        &render_generated_builtin_manifest(manifest),
    )?;
    Ok(())
}

struct GeneratedOutputFingerprint {
    bundle_digest: String,
    source_bytes: u64,
    files: Vec<GeneratedBuiltinManifestFile>,
}

fn build_generated_manifest(
    generated_root: &Path,
    source_tree_digest: String,
    generator_digest: String,
    mut devices: Vec<GeneratedBuiltinManifestDevice>,
) -> BuiltinResult<GeneratedBuiltinManifest> {
    let output = generated_output_fingerprint(generated_root)?;
    for device in &mut devices {
        let prefix = format!("{}/", device.folder_name);
        let mut file_count = 0usize;
        let mut source_bytes = 0u64;
        for file in output
            .files
            .iter()
            .filter(|file| file.relative_path.starts_with(&prefix))
        {
            file_count += 1;
            source_bytes = source_bytes
                .checked_add(file.bytes)
                .ok_or("generated device output byte count overflowed u64")?;
        }
        device.file_count = file_count;
        device.source_bytes = source_bytes;
    }
    Ok(GeneratedBuiltinManifest {
        schema_version: GENERATED_BUILTIN_MANIFEST_SCHEMA_VERSION,
        source_tree_digest,
        generator_digest,
        bundle_digest: output.bundle_digest,
        device_count: devices.len(),
        file_count: output.files.len(),
        source_bytes: output.source_bytes,
        devices,
        files: output.files,
    })
}

fn generated_output_fingerprint(
    generated_root: &Path,
) -> BuiltinResult<GeneratedOutputFingerprint> {
    let mut paths = Vec::new();
    collect_tree_files(generated_root, &mut paths)?;
    paths.retain(|path| {
        path.file_name().and_then(|name| name.to_str())
            != Some(GENERATED_BUILTIN_MANIFEST_FILE_NAME)
    });
    paths.sort();

    let mut bundle_hasher = blake3::Hasher::new();
    let mut source_bytes = 0u64;
    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let relative_path = path
            .strip_prefix(generated_root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path)?;
        let byte_count = bytes.len() as u64;
        source_bytes = source_bytes
            .checked_add(byte_count)
            .ok_or("generated Verilog-A output byte count overflowed u64")?;
        update_digest_record(&mut bundle_hasher, relative_path.as_bytes());
        update_digest_record(&mut bundle_hasher, &bytes);
        files.push(GeneratedBuiltinManifestFile {
            relative_path,
            bytes: byte_count,
            blake3: blake3::hash(&bytes).to_hex().to_string(),
        });
    }

    Ok(GeneratedOutputFingerprint {
        bundle_digest: bundle_hasher.finalize().to_hex().to_string(),
        source_bytes,
        files,
    })
}

fn validate_generated_manifest_outputs(
    generated_root: &Path,
    manifest: &GeneratedBuiltinManifest,
) -> BuiltinResult<()> {
    let actual = generated_output_fingerprint(generated_root)?;
    if actual.bundle_digest != manifest.bundle_digest {
        return Err(format!(
            "generated Verilog-A bundle digest mismatch: manifest={}, actual={}; run `{REGENERATE_BUILTINS_COMMAND}`",
            manifest.bundle_digest, actual.bundle_digest
        )
        .into());
    }
    if actual.files != manifest.files
        || actual.files.len() != manifest.file_count
        || actual.source_bytes != manifest.source_bytes
    {
        return Err(format!(
            "generated Verilog-A bundle file census mismatch: manifest files={}, bytes={}; actual files={}, bytes={}; run `{REGENERATE_BUILTINS_COMMAND}`",
            manifest.file_count,
            manifest.source_bytes,
            actual.files.len(),
            actual.source_bytes,
        )
        .into());
    }

    for device in &manifest.devices {
        let prefix = format!("{}/", device.folder_name);
        let files = actual
            .files
            .iter()
            .filter(|file| file.relative_path.starts_with(&prefix))
            .collect::<Vec<_>>();
        let source_bytes = files.iter().map(|file| file.bytes).sum::<u64>();
        if files.len() != device.file_count || source_bytes != device.source_bytes {
            return Err(format!(
                "generated Verilog-A device '{}' resource manifest mismatch: manifest files={}, bytes={}; actual files={}, bytes={}",
                device.public_model_name,
                device.file_count,
                device.source_bytes,
                files.len(),
                source_bytes,
            )
            .into());
        }
    }
    Ok(())
}

fn update_digest_record(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

fn tree_digest(root: &Path, emit_cargo_rerun: bool) -> BuiltinResult<String> {
    let mut files = Vec::new();
    collect_tree_files(root, &mut files)?;
    files.sort();

    let mut hasher = blake3::Hasher::new();
    for path in files {
        if emit_cargo_rerun {
            println!("cargo:rerun-if-changed={}", path.display());
        }
        let relative = path
            .strip_prefix(root)
            .unwrap_or(&path)
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path)?;
        update_digest_record(&mut hasher, relative.as_bytes());
        update_digest_record(&mut hasher, &bytes);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

fn collect_tree_files(root: &Path, files: &mut Vec<PathBuf>) -> BuiltinResult<()> {
    if root.join(VERILOGA_DISCOVERY_SKIP_MARKER).is_file() {
        return Ok(());
    }

    for entry in fs::read_dir(root)? {
        let entry = entry?;
        let path = entry.path();
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            collect_tree_files(&path, files)?;
        } else if file_type.is_file() {
            files.push(path);
        }
    }
    Ok(())
}

fn generate_devices(
    model_root: &Path,
    filter: Option<&str>,
    progress: bool,
    requested_jobs: Option<usize>,
) -> BuiltinResult<(
    Vec<GeneratedRustDevice>,
    BuiltinBackendSelectionCounts,
    Vec<BuiltinBackendFallbackReason>,
    Vec<GeneratedBuiltinManifestDevice>,
)> {
    let model_root = model_root
        .canonicalize()
        .unwrap_or_else(|_| model_root.to_path_buf());
    let model_root = model_root.as_path();
    let candidates = discover_veriloga_sources(model_root)?;
    let filter = filter.map(BuiltinSourceFilter::new).transpose()?;
    let work_items = builtin_module_work_items(model_root, &candidates, filter.as_ref());
    let total_modules = work_items.len();
    let jobs = builtin_generator_jobs(requested_jobs, total_modules)?;
    if progress && total_modules > 0 {
        eprintln!(
            "generating {total_modules} Verilog-A built-ins with {jobs} generator job{}",
            if jobs == 1 { "" } else { "s" }
        );
        let _ = std::io::stderr().flush();
    }

    let generated = if jobs <= 1 {
        generate_devices_sequential(model_root, work_items, progress)?
    } else {
        generate_devices_parallel(model_root, work_items, progress, jobs)?
    };

    let mut devices = Vec::with_capacity(generated.len());
    let mut backend_counts = BuiltinBackendSelectionCounts::default();
    let mut fallback_reasons = Vec::new();
    let mut manifest_devices = Vec::with_capacity(generated.len());
    for generated in generated {
        backend_counts.record(generated.backend);
        let fallback_reason = generated
            .fallback_reason
            .as_ref()
            .map(|reason| reason.reason.clone());
        let workspace = generated_workspace_resources(&generated.device)?;
        manifest_devices.push(GeneratedBuiltinManifestDevice {
            module_name: generated.device.module_name.clone(),
            public_model_name: generated.device.public_model_name.clone(),
            folder_name: generated.device.folder_name.clone(),
            backend: generated.backend,
            fallback_reason,
            file_count: generated.device.files.len(),
            source_bytes: generated
                .device
                .files
                .iter()
                .map(|file| file.contents.len() as u64)
                .sum(),
            workspace,
        });
        if let Some(reason) = generated.fallback_reason {
            fallback_reasons.push(reason);
        }
        devices.push(generated.device);
    }

    devices.sort_by(|left, right| {
        left.public_model_name
            .cmp(&right.public_model_name)
            .then_with(|| left.folder_name.cmp(&right.folder_name))
    });
    manifest_devices.sort_by(|left, right| {
        left.public_model_name
            .cmp(&right.public_model_name)
            .then_with(|| left.folder_name.cmp(&right.folder_name))
    });
    Ok((devices, backend_counts, fallback_reasons, manifest_devices))
}

fn generated_workspace_resources(
    device: &GeneratedRustDevice,
) -> BuiltinResult<GeneratedBuiltinWorkspaceResources> {
    let state = generated_device_file(device, "state.rs")?;
    let stamp = generated_device_file(device, "stamp.rs")?;
    let variable_count = parse_generated_usize_const(state, "VARIABLE_COUNT")?;
    let node_count = parse_generated_usize_const(state, "NODE_COUNT")?;
    let branch_count = parse_generated_usize_const(state, "BRANCH_COUNT")?;
    let ddt_state_count = parse_generated_usize_const(state, "DDT_STATE_COUNT")?;
    let idt_state_count = parse_generated_usize_const(state, "IDT_STATE_COUNT")?;
    let uses_transient = stamp.contains("&TRANSIENT_SCRATCH_POOL");
    let uses_reactive = stamp.contains("&REACTIVE_SCRATCH_POOL");
    let transient_active_node_rows = parse_optional_workspace_row_count(
        stamp,
        "TRANSIENT_NODE_ACTIVE_DERIVATIVE_ROWS",
        uses_transient,
    )?;
    let transient_active_branch_rows = parse_optional_workspace_row_count(
        stamp,
        "TRANSIENT_BRANCH_ACTIVE_DERIVATIVE_ROWS",
        uses_transient,
    )?;
    let reactive_active_node_rows = parse_optional_workspace_row_count(
        stamp,
        "REACTIVE_NODE_ACTIVE_DERIVATIVE_ROWS",
        uses_reactive,
    )?;
    let reactive_active_branch_rows = parse_optional_workspace_row_count(
        stamp,
        "REACTIVE_BRANCH_ACTIVE_DERIVATIVE_ROWS",
        uses_reactive,
    )?;

    let transient_packed = uses_transient.then(|| {
        transient_workspace_payload_bytes(
            variable_count,
            node_count,
            branch_count,
            transient_active_node_rows,
            transient_active_branch_rows,
        )
    });
    let reactive_packed = uses_reactive.then(|| {
        reactive_workspace_payload_bytes(
            variable_count,
            node_count,
            branch_count,
            reactive_active_node_rows,
            reactive_active_branch_rows,
        )
    });
    let legacy_dense = uses_transient
        .then(|| dense_transient_workspace_payload_bytes(variable_count, node_count, branch_count))
        .into_iter()
        .chain(uses_reactive.then(|| {
            dense_reactive_workspace_payload_bytes(variable_count, node_count, branch_count)
        }))
        .try_fold(0u128, |total, bytes| total.checked_add(bytes))
        .ok_or("generated dense workspace resource estimate overflowed u128")?;
    let pooled = transient_packed
        .into_iter()
        .chain(reactive_packed)
        .try_fold(0u128, |total, bytes| total.checked_add(bytes))
        .and_then(|bytes| bytes.checked_mul(super::device::MAX_CACHED_SCRATCH_WORKSPACES as u128))
        .ok_or("generated packed workspace resource estimate overflowed u128")?;
    let stamp_state_payload = stamp_state_payload_bytes(ddt_state_count, idt_state_count);
    let stamp_state_heap_allocations = u64::from(stamp_state_payload != 0);
    let legacy_stamp_state_heap_allocations =
        u64::from(ddt_state_count != 0) * 6 + u64::from(idt_state_count != 0) * 3;

    Ok(GeneratedBuiltinWorkspaceResources {
        abi_version: 2,
        variable_count,
        node_count,
        branch_count,
        transient_active_node_rows,
        transient_active_branch_rows,
        reactive_active_node_rows,
        reactive_active_branch_rows,
        retained_workspace_bytes_per_instance: 0,
        pooled_workspace_payload_bytes_per_thread: u64::try_from(pooled)
            .map_err(|_| "generated packed workspace resource estimate exceeds u64")?,
        legacy_dense_workspace_payload_bytes_per_instance: u64::try_from(legacy_dense)
            .map_err(|_| "generated dense workspace resource estimate exceeds u64")?,
        stamp_state_payload_bytes_per_instance: u64::try_from(stamp_state_payload)
            .map_err(|_| "generated stamp-state resource estimate exceeds u64")?,
        stamp_state_heap_allocations_per_instance: stamp_state_heap_allocations,
        stamp_state_pointer_slots_per_instance: 1,
        legacy_stamp_state_heap_allocations_per_instance: legacy_stamp_state_heap_allocations,
        legacy_stamp_state_pointer_slots_per_instance: 9,
    })
}

fn generated_device_file<'a>(
    device: &'a GeneratedRustDevice,
    relative_path: &str,
) -> BuiltinResult<&'a str> {
    device
        .files
        .iter()
        .find(|file| file.relative_path == relative_path)
        .map(|file| file.contents.as_str())
        .ok_or_else(|| {
            format!(
                "generated device '{}' is missing required resource input '{}'",
                device.module_name, relative_path
            )
            .into()
        })
}

fn parse_optional_workspace_row_count(
    source: &str,
    name: &str,
    required: bool,
) -> BuiltinResult<usize> {
    match parse_generated_usize_const_optional(source, name)? {
        Some(value) => Ok(value),
        None if !required => Ok(0),
        None => {
            Err(format!("generated stamp is missing required workspace constant '{name}'").into())
        }
    }
}

fn parse_generated_usize_const(source: &str, name: &str) -> BuiltinResult<usize> {
    parse_generated_usize_const_optional(source, name)?.ok_or_else(|| {
        format!("generated source is missing required usize constant '{name}'").into()
    })
}

fn parse_generated_usize_const_optional(source: &str, name: &str) -> BuiltinResult<Option<usize>> {
    let marker = format!("const {name}: usize = ");
    let Some(line) = source.lines().find(|line| line.contains(&marker)) else {
        return Ok(None);
    };
    let value = line
        .split_once(&marker)
        .map(|(_, value)| value)
        .and_then(|value| value.trim().strip_suffix(';'))
        .ok_or_else(|| format!("generated usize constant '{name}' is malformed"))?;
    Ok(Some(value.parse::<usize>().map_err(|error| {
        format!("generated usize constant '{name}' has invalid value '{value}': {error}")
    })?))
}

fn derivative_matrix_payload_bytes(
    variable_count: usize,
    axis_count: usize,
    active_rows: usize,
) -> u128 {
    // Fixed-array row map (u32), packed active rows, one shared zero row,
    // and the thin/fat Box pointer payload in DerivativeMatrix.
    variable_count as u128 * 4
        + active_rows as u128 * axis_count as u128 * 8
        + axis_count as u128 * 8
        + 24
}

fn stamp_state_payload_bytes(ddt_state_count: usize, idt_state_count: usize) -> u128 {
    let f64_bytes = (ddt_state_count as u128 * 5 + idt_state_count as u128 * 2) * 8;
    let bool_bytes = ddt_state_count as u128 + idt_state_count as u128;
    let bytes = f64_bytes + bool_bytes;
    if bytes == 0 { 0 } else { (bytes + 7) & !7 }
}

fn transient_workspace_payload_bytes(
    variable_count: usize,
    node_count: usize,
    branch_count: usize,
    active_node_rows: usize,
    active_branch_rows: usize,
) -> u128 {
    variable_count as u128 * 10
        + derivative_matrix_payload_bytes(variable_count, node_count, active_node_rows)
        + derivative_matrix_payload_bytes(variable_count, branch_count, active_branch_rows)
        + 32
}

fn reactive_workspace_payload_bytes(
    variable_count: usize,
    node_count: usize,
    branch_count: usize,
    active_node_rows: usize,
    active_branch_rows: usize,
) -> u128 {
    variable_count as u128 * 18
        + 2 * derivative_matrix_payload_bytes(variable_count, node_count, active_node_rows)
        + 2 * derivative_matrix_payload_bytes(variable_count, branch_count, active_branch_rows)
        + 32
}

fn dense_transient_workspace_payload_bytes(
    variable_count: usize,
    node_count: usize,
    branch_count: usize,
) -> u128 {
    variable_count as u128 * 10
        + variable_count as u128 * (node_count + branch_count) as u128 * 8
        + 32
}

fn dense_reactive_workspace_payload_bytes(
    variable_count: usize,
    node_count: usize,
    branch_count: usize,
) -> u128 {
    variable_count as u128 * 18
        + variable_count as u128 * (node_count + branch_count) as u128 * 16
        + 32
}

/// The backend every shipped built-in is emitted through.
///
/// The canonical CFG emitter, selected outright rather than through `Auto`.
/// It carries all 42 corpus models, so the tier cascade has nothing left to
/// catch, and a cascade that can still catch something is a cascade that can
/// silently downgrade a model back onto a tier this rebuild exists to remove.
///
/// `Auto` and the tiers behind it are left in place rather than rewired,
/// because Phase 6 deletes them along with `RustBackendSelection` itself.
/// Pointing them at the canonical emitter first would be work done only to be
/// deleted; taking their last production caller away is what makes that
/// deletion a removal rather than a migration.
fn builtin_transpiler() -> RustTranspiler {
    RustTranspiler::new_canonical(Default::default())
}

fn generate_devices_sequential(
    model_root: &Path,
    work_items: Vec<BuiltinModuleWorkItem>,
    progress: bool,
) -> BuiltinResult<Vec<GeneratedBuiltinModule>> {
    let total_modules = work_items.len();
    let transpiler = builtin_transpiler();
    let mut generated = Vec::with_capacity(total_modules);
    for (index, item) in work_items.into_iter().enumerate() {
        generated.push(generate_device_work_item(
            model_root,
            &transpiler,
            item,
            index,
            total_modules,
            progress,
            None,
        )?);
    }
    Ok(generated)
}

fn generate_devices_parallel(
    model_root: &Path,
    work_items: Vec<BuiltinModuleWorkItem>,
    progress: bool,
    jobs: usize,
) -> BuiltinResult<Vec<GeneratedBuiltinModule>> {
    let total_modules = work_items.len();
    let queue: VecDeque<_> = work_items.into_iter().enumerate().collect();
    let queue = Arc::new(Mutex::new(queue));
    let generated = Arc::new(Mutex::new(Vec::with_capacity(total_modules)));
    let progress_lock = Arc::new(Mutex::new(()));
    let first_error = Arc::new(Mutex::new(None::<String>));
    let mut handles = Vec::with_capacity(jobs);

    for worker in 0..jobs {
        let model_root = model_root.to_path_buf();
        let queue = Arc::clone(&queue);
        let generated = Arc::clone(&generated);
        let progress_lock = Arc::clone(&progress_lock);
        let first_error = Arc::clone(&first_error);
        let handle = std::thread::Builder::new()
            .name(format!("rspice-veriloga-builtin-generator-{worker}"))
            .stack_size(256 * 1024 * 1024)
            .spawn(move || -> Result<(), String> {
                let transpiler = builtin_transpiler();

                loop {
                    if first_error
                        .lock()
                        .expect("generator error lock poisoned")
                        .is_some()
                    {
                        return Ok(());
                    }
                    let Some((index, item)) = queue
                        .lock()
                        .expect("generator work queue lock poisoned")
                        .pop_front()
                    else {
                        return Ok(());
                    };
                    let result = generate_device_work_item(
                        &model_root,
                        &transpiler,
                        item,
                        index,
                        total_modules,
                        progress,
                        Some(&progress_lock),
                    )
                    .map_err(|error| error.to_string());
                    match result {
                        Ok(module) => generated
                            .lock()
                            .expect("generator result lock poisoned")
                            .push(module),
                        Err(error) => {
                            *first_error.lock().expect("generator error lock poisoned") =
                                Some(error.clone());
                            return Err(error);
                        }
                    }
                }
            })?;
        handles.push(handle);
    }

    for handle in handles {
        match handle.join() {
            Ok(Ok(())) => {}
            Ok(Err(error)) => return Err(error.into()),
            Err(_) => return Err("Verilog-A built-in generator worker panicked".into()),
        }
    }

    if let Some(error) = first_error
        .lock()
        .expect("generator error lock poisoned")
        .clone()
    {
        return Err(error.into());
    }

    let mut generated = Arc::try_unwrap(generated)
        .map_err(|_| "Verilog-A generated module results still have references")?
        .into_inner()
        .map_err(|_| "Verilog-A generated module result lock is poisoned")?;
    generated.sort_by_key(|module| module.index);
    Ok(generated)
}

fn generate_device_work_item(
    model_root: &Path,
    transpiler: &RustTranspiler,
    item: BuiltinModuleWorkItem,
    index: usize,
    total_modules: usize,
    progress: bool,
    progress_lock: Option<&Mutex<()>>,
) -> BuiltinResult<GeneratedBuiltinModule> {
    let module_index = index + 1;
    let relative_source = item
        .path
        .strip_prefix(model_root)
        .unwrap_or(&item.path)
        .to_path_buf();
    if progress {
        log_generation_progress(
            progress_lock,
            format!(
                "generating Verilog-A built-in {module_index}/{total_modules}: {} :: {}",
                relative_source.display(),
                item.module
            ),
        );
    }
    let started = Instant::now();
    let mut options = CompilerOptions::default();
    options.include_paths.push(model_root.to_path_buf());
    options.defines = item.compile_profile.defines;
    options.undefines = item.compile_profile.undefines;
    let compiler = VerilogACompiler::new(options);
    let compiled =
        compiler.compile_file_canonical_ir_with_metadata(&item.path, Some(&item.module))?;
    if progress {
        log_generation_progress(
            progress_lock,
            format!(
                "compiled Verilog-A built-in {module_index}/{total_modules}: {} :: {} ({:.2?})",
                relative_source.display(),
                item.module,
                started.elapsed()
            ),
        );
    }
    let transpile_started = Instant::now();
    if progress {
        log_generation_progress(
            progress_lock,
            format!(
                "transpiling Verilog-A built-in {module_index}/{total_modules}: {} :: {}",
                relative_source.display(),
                item.module
            ),
        );
    }
    let report = transpiler.transpile_with_report(&compiled.artifact)?;
    let fallback_reason =
        report
            .fallback_reason
            .as_ref()
            .map(|reason| BuiltinBackendFallbackReason {
                source: relative_source.clone(),
                module: item.module.clone(),
                backend: report.backend,
                reason: reason.clone(),
            });
    if progress {
        let fallback = report
            .fallback_reason
            .as_ref()
            .map(|reason| format!(", fallback: {reason}"))
            .unwrap_or_default();
        log_generation_progress(
            progress_lock,
            format!(
                "generated Verilog-A built-in {module_index}/{total_modules}: {} :: {} ({:?}{fallback}, transpile {:.2?}, total {:.2?})",
                relative_source.display(),
                item.module,
                report.backend,
                transpile_started.elapsed(),
                started.elapsed()
            ),
        );
    }

    Ok(GeneratedBuiltinModule {
        index,
        device: report.device,
        backend: report.backend,
        fallback_reason,
    })
}

fn log_generation_progress(progress_lock: Option<&Mutex<()>>, message: String) {
    if let Some(progress_lock) = progress_lock {
        let _guard = progress_lock
            .lock()
            .expect("generator progress lock poisoned");
        eprintln!("{message}");
        let _ = std::io::stderr().flush();
    } else {
        eprintln!("{message}");
        let _ = std::io::stderr().flush();
    }
}

fn builtin_generator_jobs(requested: Option<usize>, total_modules: usize) -> BuiltinResult<usize> {
    if total_modules <= 1 {
        validate_requested_generator_jobs(requested)?;
        return Ok(1);
    }

    let requested = match requested {
        Some(jobs) => Some(jobs),
        None => env::var("RSPICE_VERILOGA_GENERATOR_JOBS")
            .ok()
            .map(|value| {
                value.parse::<usize>().map_err(|error| {
                    format!("RSPICE_VERILOGA_GENERATOR_JOBS must be a positive integer: {error}")
                })
            })
            .transpose()?,
    };
    let jobs = if let Some(jobs) = requested {
        if jobs == 0 {
            return Err("Verilog-A generator --jobs must be at least 1".into());
        }
        jobs
    } else {
        std::thread::available_parallelism()
            .map(|available| available.get())
            .unwrap_or(1)
            .min(4)
    };
    Ok(jobs.clamp(1, total_modules))
}

fn validate_requested_generator_jobs(requested: Option<usize>) -> BuiltinResult<()> {
    if requested == Some(0) {
        return Err("Verilog-A generator --jobs must be at least 1".into());
    }
    Ok(())
}

fn reject_legacy_backend_selections(
    counts: &BuiltinBackendSelectionCounts,
    fallback_reasons: &[BuiltinBackendFallbackReason],
) -> BuiltinResult<()> {
    if counts.legacy_device == 0 {
        return Ok(());
    }

    let mut details = fallback_reasons
        .iter()
        .filter(|reason| reason.backend == RustBackendSelection::LegacyDevice)
        .map(|reason| {
            format!(
                "{} :: {} ({})",
                reason.source.display(),
                reason.module,
                reason.reason
            )
        })
        .collect::<Vec<_>>();
    if details.is_empty() {
        details.push(format!(
            "{} generated module{} selected the legacy device backend",
            counts.legacy_device,
            if counts.legacy_device == 1 { "" } else { "s" }
        ));
    }

    Err(format!(
        "Verilog-A built-ins must use optimized Rust backends; legacy selections:\n{}",
        details.join("\n")
    )
    .into())
}

fn reject_non_scalar_backend_selections_if_requested(
    counts: &BuiltinBackendSelectionCounts,
    fallback_reasons: &[BuiltinBackendFallbackReason],
) -> BuiltinResult<()> {
    if env::var_os(REQUIRE_SCALAR_BUILTINS_ENV).is_none() {
        return Ok(());
    }

    reject_non_scalar_backend_selections(counts, fallback_reasons)
}

fn reject_non_scalar_backend_selections(
    counts: &BuiltinBackendSelectionCounts,
    fallback_reasons: &[BuiltinBackendFallbackReason],
) -> BuiltinResult<()> {
    if counts.sparse_local == 0 && counts.structured == 0 && counts.hybrid == 0 {
        return Ok(());
    }

    let mut details = fallback_reasons
        .iter()
        .filter(|reason| {
            matches!(
                reason.backend,
                RustBackendSelection::SparseLocalKernel
                    | RustBackendSelection::StructuredKernel
                    | RustBackendSelection::ScalarHybrid
            )
        })
        .map(|reason| {
            format!(
                "{} :: {} ({})",
                reason.source.display(),
                reason.module,
                reason.reason
            )
        })
        .collect::<Vec<_>>();
    if counts.sparse_local != 0
        && !fallback_reasons
            .iter()
            .any(|reason| reason.backend == RustBackendSelection::SparseLocalKernel)
    {
        details.push(format!(
            "{} generated module{} selected the sparse-local-kernel backend",
            counts.sparse_local,
            if counts.sparse_local == 1 { "" } else { "s" }
        ));
    }
    if counts.structured != 0
        && !fallback_reasons
            .iter()
            .any(|reason| reason.backend == RustBackendSelection::StructuredKernel)
    {
        details.push(format!(
            "{} generated module{} selected the structured-kernel backend",
            counts.structured,
            if counts.structured == 1 { "" } else { "s" }
        ));
    }
    if counts.hybrid != 0
        && !fallback_reasons
            .iter()
            .any(|reason| reason.backend == RustBackendSelection::ScalarHybrid)
    {
        details.push(format!(
            "{} generated module{} selected the scalar-hybrid backend",
            counts.hybrid,
            if counts.hybrid == 1 { "" } else { "s" }
        ));
    }

    Err(format!(
        "Verilog-A built-ins must use the direct scalar optimized Rust backend when {REQUIRE_SCALAR_BUILTINS_ENV}=1; non-scalar selections:\n{}",
        details.join("\n")
    )
    .into())
}

#[derive(Debug)]
struct GeneratedBuiltinModule {
    index: usize,
    device: GeneratedRustDevice,
    backend: RustBackendSelection,
    fallback_reason: Option<BuiltinBackendFallbackReason>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuiltinSourceFilter {
    terms: Vec<String>,
}

impl BuiltinSourceFilter {
    fn new(filter: &str) -> BuiltinResult<Self> {
        let terms = filter
            .split(|ch: char| ch == ',' || ch == ';' || ch.is_whitespace())
            .filter(|term| !term.is_empty())
            .map(|term| term.to_ascii_lowercase())
            .collect::<Vec<_>>();
        if terms.is_empty() {
            return Err("Verilog-A built-in generation filter cannot be empty".into());
        }
        Ok(Self { terms })
    }

    fn matches(&self, model_root: &Path, path: &Path, module: &str) -> bool {
        let relative = path
            .strip_prefix(model_root)
            .unwrap_or(path)
            .to_string_lossy()
            .replace('\\', "/")
            .to_ascii_lowercase();
        let module = module.to_ascii_lowercase();
        self.terms
            .iter()
            .any(|term| relative.contains(term) || module.contains(term))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BuiltinModuleWorkItem {
    path: PathBuf,
    module: String,
    compile_profile: VerilogACompileProfile,
}

fn builtin_module_work_items(
    model_root: &Path,
    candidates: &[VerilogASourceCandidate],
    filter: Option<&BuiltinSourceFilter>,
) -> Vec<BuiltinModuleWorkItem> {
    let mut work_items = Vec::new();
    for candidate in candidates {
        for module in &candidate.modules {
            if filter.is_none_or(|filter| filter.matches(model_root, &candidate.path, module)) {
                work_items.push(BuiltinModuleWorkItem {
                    path: candidate.path.clone(),
                    module: module.clone(),
                    compile_profile: candidate.compile_profile.clone(),
                });
            }
        }
    }
    work_items
}

fn generate_devices_with_stack(
    model_root: PathBuf,
    filter: Option<String>,
    progress: bool,
    jobs: Option<usize>,
) -> Result<
    (
        Vec<GeneratedRustDevice>,
        BuiltinBackendSelectionCounts,
        Vec<BuiltinBackendFallbackReason>,
        Vec<GeneratedBuiltinManifestDevice>,
    ),
    Box<dyn std::error::Error + Send + Sync>,
> {
    std::thread::Builder::new()
        .name("rspice-veriloga-builtin-generator".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || {
            generate_devices(&model_root, filter.as_deref(), progress, jobs)
                .map_err(|error| error.to_string())
        })?
        .join()
        .map_err(|_| "Verilog-A built-in generator thread panicked")?
        .map_err(|error| error.into())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn generated_device_fixture(folder_name: &str, public_model_name: &str) -> GeneratedRustDevice {
        GeneratedRustDevice {
            module_name: public_model_name.to_string(),
            public_model_name: public_model_name.to_string(),
            folder_name: folder_name.to_string(),
            files: Vec::new(),
            source_digest: "fixture".to_string(),
        }
    }

    fn temporary_registry_root(test_name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "rspice-veriloga-{test_name}-{}-{:?}",
            std::process::id(),
            std::thread::current().id()
        ))
    }

    #[test]
    fn builtin_subset_filter_matches_module_without_selecting_siblings() {
        let model_root = Path::new("models");
        let candidates = vec![VerilogASourceCandidate {
            path: PathBuf::from("models/cmc/hicum.va"),
            modules: vec!["hicuml2".to_string(), "hicuml0".to_string()],
            compile_profile: VerilogACompileProfile::default(),
        }];
        let filter = BuiltinSourceFilter::new("hicuml2").expect("filter");

        let work_items = builtin_module_work_items(model_root, &candidates, Some(&filter));

        assert_eq!(
            work_items,
            vec![BuiltinModuleWorkItem {
                path: PathBuf::from("models/cmc/hicum.va"),
                module: "hicuml2".to_string(),
                compile_profile: VerilogACompileProfile::default(),
            }]
        );
    }

    #[test]
    fn builtin_subset_filter_matches_all_modules_when_path_matches() {
        let model_root = Path::new("models");
        let candidates = vec![VerilogASourceCandidate {
            path: PathBuf::from("models/cmc/hicum.va"),
            modules: vec!["hicuml2".to_string(), "hicuml0".to_string()],
            compile_profile: VerilogACompileProfile::default(),
        }];
        let filter = BuiltinSourceFilter::new("cmc/hicum").expect("filter");

        let work_items = builtin_module_work_items(model_root, &candidates, Some(&filter));

        assert_eq!(work_items.len(), 2);
        assert_eq!(work_items[0].module, "hicuml2");
        assert_eq!(work_items[1].module, "hicuml0");
    }

    #[test]
    fn builtin_generator_jobs_clamps_requested_jobs_to_work_items() {
        assert_eq!(builtin_generator_jobs(Some(99), 3).expect("jobs"), 3);
    }

    #[test]
    fn builtin_generator_jobs_rejects_zero_jobs() {
        let error = builtin_generator_jobs(Some(0), 3).expect_err("zero jobs must fail");
        assert!(error.to_string().contains("must be at least 1"));
    }

    #[test]
    fn generated_manifest_authenticates_every_output_file() {
        let root = temporary_registry_root("manifest-authentication");
        let _ = fs::remove_dir_all(&root);
        let folder = root.join("fixture__fixture__12345678");
        fs::create_dir_all(&folder).expect("create generated fixture folder");
        fs::write(folder.join("stamp.rs"), "abc").expect("write generated fixture");
        fs::write(folder.join(".rspice-veriloga-generated"), "marker")
            .expect("write generated ownership marker");
        fs::write(root.join("registry.rs"), "registry").expect("write registry fixture");
        fs::write(root.join("kernel_runtime.rs"), "runtime").expect("write runtime fixture");

        let manifest = build_generated_manifest(
            &root,
            "source".to_string(),
            "generator".to_string(),
            vec![GeneratedBuiltinManifestDevice {
                module_name: "fixture".to_string(),
                public_model_name: "fixture".to_string(),
                folder_name: "fixture__fixture__12345678".to_string(),
                backend: RustBackendSelection::ScalarOptIr,
                fallback_reason: None,
                file_count: 1,
                source_bytes: 3,
                workspace: Default::default(),
            }],
        )
        .expect("build generated manifest");

        assert_eq!(
            manifest.schema_version,
            GENERATED_BUILTIN_MANIFEST_SCHEMA_VERSION
        );
        assert_eq!(manifest.file_count, 4);
        assert_eq!(manifest.devices[0].file_count, 2);
        assert_eq!(manifest.devices[0].source_bytes, 9);
        validate_generated_manifest_outputs(&root, &manifest)
            .expect("fresh generated output authenticates");

        fs::write(folder.join("stamp.rs"), "tampered").expect("tamper generated fixture");
        let error = validate_generated_manifest_outputs(&root, &manifest)
            .expect_err("tampered generated output must fail validation");
        assert!(
            error.to_string().contains("bundle digest mismatch"),
            "{error}"
        );

        fs::remove_dir_all(root).expect("remove generated fixture");
    }

    #[test]
    fn builtin_generation_rejects_legacy_backend_selections() {
        let counts = BuiltinBackendSelectionCounts {
            scalar: 1,
            sparse_local: 0,
            structured: 0,
            hybrid: 0,
            legacy_device: 1,
            canonical: 0,
        };
        let reasons = vec![BuiltinBackendFallbackReason {
            source: PathBuf::from("cmc/model.va"),
            module: "legacy_model".to_string(),
            backend: RustBackendSelection::LegacyDevice,
            reason: "scalar path: unsupported; hybrid scalar path: unsupported".to_string(),
        }];

        let error = reject_legacy_backend_selections(&counts, &reasons)
            .expect_err("legacy built-in selections must fail generation");

        let message = error.to_string();
        assert!(
            message.contains("must use optimized Rust backends"),
            "{message}"
        );
        assert!(
            message.contains("cmc/model.va :: legacy_model"),
            "{message}"
        );
    }

    #[test]
    fn builtin_generation_can_reject_non_scalar_backend_selections() {
        let counts = BuiltinBackendSelectionCounts {
            scalar: 1,
            sparse_local: 1,
            structured: 1,
            hybrid: 1,
            legacy_device: 0,
            canonical: 0,
        };
        let reasons = vec![BuiltinBackendFallbackReason {
            source: PathBuf::from("cmc/hisim.va"),
            module: "hisim".to_string(),
            backend: RustBackendSelection::ScalarHybrid,
            reason: "scalar path: source has 400000 emitted values over budget".to_string(),
        }];

        let error = reject_non_scalar_backend_selections(&counts, &reasons)
            .expect_err("strict scalar built-in generation must reject non-scalar selections");

        let message = error.to_string();
        assert!(message.contains(REQUIRE_SCALAR_BUILTINS_ENV), "{message}");
        assert!(message.contains("sparse-local-kernel backend"), "{message}");
        assert!(message.contains("structured-kernel backend"), "{message}");
        assert!(message.contains("cmc/hisim.va :: hisim"), "{message}");
    }

    #[test]
    fn builtin_generation_writes_kernel_runtime_and_removes_legacy_support() {
        let root = std::env::temp_dir().join(format!(
            "rspice-veriloga-kernel-runtime-{}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("create generated root");
        fs::write(root.join("support.rs"), "legacy support").expect("write legacy support fixture");
        let device = root.join("fixture");
        fs::create_dir_all(&device).expect("create generated device fixture");
        fs::write(
            device.join("stamp.rs"),
            "fn stamp(s: &mut Scratch) { s.store_scalar(0, 1.0); }",
        )
        .expect("write generated stamp fixture");
        fs::write(
            device.join("state.rs"),
            "fn allocate() { KernelScratch::new_box(); }",
        )
        .expect("write generated state fixture");

        write_kernel_runtime(&root).expect("write kernel runtime");

        assert!(!root.join("support.rs").exists());
        let runtime = fs::read_to_string(root.join("kernel_runtime.rs"))
            .expect("read generated kernel runtime");
        assert!(runtime.contains("pub(crate) struct Scratch<"), "{runtime}");
        assert!(
            runtime.contains("pub(crate) struct DerivativeMatrix<"),
            "{runtime}"
        );
        assert!(
            runtime.contains("pub(crate) dn: DerivativeMatrix<"),
            "{runtime}"
        );
        assert!(runtime.contains("#[inline(never)]"), "{runtime}");
        assert!(runtime.contains("pub(crate) fn new_box()"), "{runtime}");
        assert!(runtime.contains("pub(crate) fn store_scalar("), "{runtime}");
        assert!(
            runtime.contains("pub(crate) fn clear_derivatives_if_dirty("),
            "{runtime}"
        );
        assert!(
            !runtime
                .contains("pub(crate) fn store_div_scaled_product_sqrt_square_sum_denominator("),
            "{runtime}"
        );
        reject_legacy_ad_runtime_files(&root).expect("validate generated kernel runtime");
        fs::remove_dir_all(root).expect("remove generated root");
    }

    #[test]
    fn generated_workspace_resources_record_pooled_packed_abi() {
        let mut device = generated_device_fixture("workspace_model", "workspace");
        device.files = vec![
            crate::rust_backend::GeneratedRustFile {
                relative_path: "state.rs".to_string(),
                contents: [
                    "pub const VARIABLE_COUNT: usize = 4;",
                    "pub const NODE_COUNT: usize = 2;",
                    "pub const BRANCH_COUNT: usize = 1;",
                    "pub const DDT_STATE_COUNT: usize = 2;",
                    "pub const IDT_STATE_COUNT: usize = 1;",
                ]
                .join("\n"),
            },
            crate::rust_backend::GeneratedRustFile {
                relative_path: "stamp.rs".to_string(),
                contents: [
                    "let _ = &TRANSIENT_SCRATCH_POOL;",
                    "let _ = &REACTIVE_SCRATCH_POOL;",
                    "const TRANSIENT_NODE_ACTIVE_DERIVATIVE_ROWS: usize = 2;",
                    "const TRANSIENT_BRANCH_ACTIVE_DERIVATIVE_ROWS: usize = 0;",
                    "const REACTIVE_NODE_ACTIVE_DERIVATIVE_ROWS: usize = 1;",
                    "const REACTIVE_BRANCH_ACTIVE_DERIVATIVE_ROWS: usize = 1;",
                ]
                .join("\n"),
            },
        ];

        let resources = generated_workspace_resources(&device).expect("workspace resources");
        assert_eq!(resources.abi_version, 2);
        assert_eq!(resources.transient_active_node_rows, 2);
        assert_eq!(resources.transient_active_branch_rows, 0);
        assert_eq!(resources.reactive_active_node_rows, 1);
        assert_eq!(resources.reactive_active_branch_rows, 1);
        assert_eq!(resources.retained_workspace_bytes_per_instance, 0);
        assert_eq!(resources.pooled_workspace_payload_bytes_per_thread, 1_136);
        assert_eq!(
            resources.legacy_dense_workspace_payload_bytes_per_instance,
            464
        );
        assert_eq!(resources.stamp_state_payload_bytes_per_instance, 104);
        assert_eq!(resources.stamp_state_heap_allocations_per_instance, 1);
        assert_eq!(resources.stamp_state_pointer_slots_per_instance, 1);
        assert_eq!(
            resources.legacy_stamp_state_heap_allocations_per_instance,
            9
        );
        assert_eq!(resources.legacy_stamp_state_pointer_slots_per_instance, 9);
    }

    #[test]
    fn builtin_registry_dispatches_noise_metadata_and_evaluation_by_variant() {
        let root = temporary_registry_root("noise-registry");
        let _ = fs::remove_dir_all(&root);
        let devices = vec![
            generated_device_fixture("first_model", "first"),
            generated_device_fixture("second_model", "second"),
        ];

        write_registry(&root, &devices).expect("write generated registry");
        let registry = fs::read_to_string(root.join("registry.rs")).expect("read registry");

        assert!(
            registry.contains("#[cfg(feature = \"veriloga-model-first\")]"),
            "{registry}"
        );
        assert!(
            registry.contains("#[cfg(feature = \"veriloga-model-second\")]"),
            "{registry}"
        );
        assert!(
            registry.contains(
                "#[cfg(all(feature = \"veriloga-model-first\", feature = \"veriloga-builtins-noise\"))]"
            ),
            "{registry}"
        );
        assert!(
            registry.contains("instance.set_multiplicity(*value)?;"),
            "{registry}"
        );
        assert!(
            registry.contains("Self::Device0(device) => device.limiter_converged()"),
            "{registry}"
        );
        assert!(
            registry.contains("Self::Device1(device) => device.limiter_converged()"),
            "{registry}"
        );
        assert!(
            registry
                .contains("Self::Device0(_) => first_model::Instance::CHECKPOINT_MODEL_IDENTITY"),
            "{registry}"
        );
        assert!(
            registry
                .contains("Self::Device1(_) => second_model::Instance::CHECKPOINT_MODEL_IDENTITY"),
            "{registry}"
        );
        for dispatch in [
            "device.capture_persistent_state()",
            "device.validate_persistent_state_shape(state)",
            "device.restore_persistent_state(state)",
        ] {
            assert_eq!(registry.matches(dispatch).count(), 2, "{registry}");
        }
        assert!(
            registry.contains(
                "pub fn noise_descriptors(&self) -> &'static [super::GeneratedNoiseDescriptor]"
            ),
            "{registry}"
        );
        assert!(
            registry.contains("Self::Device0(_) => &first_model::NOISE_SOURCES"),
            "{registry}"
        );
        assert!(
            registry.contains("Self::Device1(_) => &second_model::NOISE_SOURCES"),
            "{registry}"
        );
        assert!(
            registry
                .contains("Self::Device0(device) => device.evaluate_noise_sources(ctx, visitor)"),
            "{registry}"
        );
        assert!(
            registry
                .contains("Self::Device1(device) => device.evaluate_noise_sources(ctx, visitor)"),
            "{registry}"
        );

        fs::remove_dir_all(root).expect("remove registry fixture");
    }

    #[test]
    fn generated_model_features_are_stable_and_cargo_safe() {
        assert_eq!(
            generated_model_feature_name("hisimsoi_va__5be18005"),
            "veriloga-model-hisimsoi-va-5be18005"
        );
        assert_eq!(
            generated_model_feature_name("EPFL_HEMT_10a"),
            "veriloga-model-epfl-hemt-10a"
        );
    }

    #[test]
    fn empty_builtin_registry_has_total_noise_dispatch_semantics() {
        let root = temporary_registry_root("empty-noise-registry");
        let _ = fs::remove_dir_all(&root);

        write_registry(&root, &[]).expect("write empty generated registry");
        let registry = fs::read_to_string(root.join("registry.rs")).expect("read registry");

        assert!(registry.contains("pub fn limiter_converged(&self) -> bool"));
        assert!(
            registry.contains("empty generated Verilog-A registry has no checkpoint identity"),
            "{registry}"
        );
        assert!(
            registry.contains("empty generated Verilog-A registry has no persistent state"),
            "{registry}"
        );
        assert!(registry.contains("        true\n"));
        assert!(registry.contains("        &[]"), "{registry}");
        assert!(
            registry.contains(
                "pub fn evaluate_noise_sources(&self, ctx: &super::GeneratedEvalContext<'_>, visitor: &mut dyn super::GeneratedNoiseVisitor)"
            ),
            "{registry}"
        );
        assert!(registry.contains("        Ok(())"), "{registry}");

        fs::remove_dir_all(root).expect("remove empty registry fixture");
    }
}

fn generated_model_feature_name(registry_name: &str) -> String {
    let mut feature = String::from("veriloga-model-");
    let mut separator_pending = false;
    for character in registry_name.chars() {
        if character.is_ascii_alphanumeric() {
            if separator_pending && !feature.ends_with('-') {
                feature.push('-');
            }
            feature.push(character.to_ascii_lowercase());
            separator_pending = false;
        } else {
            separator_pending = true;
        }
    }
    while feature.ends_with('-') {
        feature.pop();
    }
    feature
}

fn write_registry(registry_root: &Path, devices: &[GeneratedRustDevice]) -> BuiltinResult<()> {
    std::fs::create_dir_all(registry_root)?;
    let registry_model_names = resolve_generated_registry_model_names(devices);
    let feature_names = registry_model_names
        .iter()
        .map(|name| generated_model_feature_name(name))
        .collect::<Vec<_>>();

    let mut out = String::new();
    out.push_str("// Generated by rspice-veriloga-gen. Do not edit.\n\n");
    for (device, feature) in devices.iter().zip(&feature_names) {
        writeln!(
            out,
            "#[cfg(feature = {feature:?})]\n#[allow(non_snake_case)]\n#[path = \"{}/mod.rs\"]\npub mod {};",
            device.folder_name, device.folder_name,
        )?;
    }
    out.push('\n');

    out.push_str("#[derive(Clone)]\n");
    out.push_str("pub enum GeneratedBuiltinKind {\n");
    for (index, (device, feature)) in devices.iter().zip(&feature_names).enumerate() {
        writeln!(
            out,
            "    #[cfg(feature = {feature:?})]\n    Device{index}(Box<{}::Instance>),",
            device.folder_name,
        )?;
    }
    out.push_str("    #[doc(hidden)]\n");
    out.push_str("    __NonExhaustive(std::convert::Infallible),\n");
    out.push_str("}\n\n");

    out.push_str("impl GeneratedBuiltinKind {\n");
    out.push_str(
        "    pub(crate) fn capture_rollback_state(&self) -> super::GeneratedVerilogARollbackState {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry has no rollback state\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.capture_rollback_state(),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n\n");
    out.push_str(
        "    pub(crate) fn restore_rollback_state(&mut self, state: &super::GeneratedVerilogARollbackState) {\n",
    );
    out.push_str("        let _ = state;\n");
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry has no rollback state\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.restore_rollback_state(state),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n\n");
    out.push_str("    pub(crate) fn checkpoint_model_identity(&self) -> &'static str {\n");
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str("        unreachable!(\"empty generated Verilog-A registry has no checkpoint identity\")\n");
    } else {
        out.push_str("        match self {\n");
        for (index, (device, feature)) in devices.iter().zip(&feature_names).enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(_) => {}::Instance::CHECKPOINT_MODEL_IDENTITY,",
                device.folder_name,
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n\n");
    out.push_str(
        "    pub(crate) fn capture_persistent_state(&self) -> super::GeneratedVerilogAPersistentState {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str("        unreachable!(\"empty generated Verilog-A registry has no persistent state\")\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.capture_persistent_state(),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n\n");
    out.push_str("    pub(crate) fn validate_persistent_state_shape(&self, state: &super::GeneratedVerilogAPersistentState) -> Result<(), String> {\n");
    out.push_str("        let _ = state;\n");
    if devices.is_empty() {
        out.push_str("        let _ = (self, state);\n");
        out.push_str("        Err(\"empty generated Verilog-A registry has no persistent state\".to_string())\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.validate_persistent_state_shape(state),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n\n");
    out.push_str("    pub(crate) fn restore_persistent_state(&mut self, state: &super::GeneratedVerilogAPersistentState) -> Result<(), String> {\n");
    out.push_str("        let _ = state;\n");
    if devices.is_empty() {
        out.push_str("        let _ = (self, state);\n");
        out.push_str("        Err(\"empty generated Verilog-A registry has no persistent state\".to_string())\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.restore_persistent_state(state),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    pub fn limiter_converged(&self) -> bool {\n");
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str("        true\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.limiter_converged(),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn noise_descriptors(&self) -> &'static [super::GeneratedNoiseDescriptor] {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
        out.push_str("        &[]\n");
    } else {
        out.push_str("        match self {\n");
        for (index, (device, feature)) in devices.iter().zip(&feature_names).enumerate() {
            writeln!(
                out,
                "            #[cfg(all(feature = {feature:?}, feature = \"veriloga-builtins-noise\"))]\n            Self::Device{index}(_) => &{}::NOISE_SOURCES,",
                device.folder_name,
            )?;
        }
        out.push_str(
            "            #[cfg(feature = \"veriloga-builtins-noise\")]\n            Self::__NonExhaustive(value) => match *value {},\n",
        );
        out.push_str(
            "            #[cfg(not(feature = \"veriloga-builtins-noise\"))]\n            _ => &[],\n",
        );
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn evaluate_noise_sources(&self, ctx: &super::GeneratedEvalContext<'_>, visitor: &mut dyn super::GeneratedNoiseVisitor) -> Result<(), super::GeneratedNoiseEvaluationError> {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = (self, ctx, visitor);\n");
        out.push_str("        Ok(())\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(all(feature = {feature:?}, feature = \"veriloga-builtins-noise\"))]\n            Self::Device{index}(device) => device.evaluate_noise_sources(ctx, visitor),"
            )?;
        }
        out.push_str(
            "            #[cfg(feature = \"veriloga-builtins-noise\")]\n            Self::__NonExhaustive(value) => match *value {},\n",
        );
        out.push_str(
            "            #[cfg(not(feature = \"veriloga-builtins-noise\"))]\n            _ => {\n                let _ = (ctx, visitor);\n                Ok(())\n            }\n",
        );
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &super::GeneratedEvalContext<'_>, stamper: &mut super::GeneratedStamper<'_>) {\n",
    );
    out.push_str("        let _ = (&ctx, &stamper);\n");
    if devices.is_empty() {
        out.push_str("        let _ = (ctx, stamper, self);\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry cannot be stamped\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.stamp(ctx, stamper),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn set_timepoint(&mut self, time: crate::Value, timestep: crate::Value, ddt_coefficients: super::GeneratedDdtCoefficients) {\n",
    );
    out.push_str("        let _ = (time, timestep, ddt_coefficients);\n");
    if devices.is_empty() {
        out.push_str("        let _ = (self, time, timestep, ddt_coefficients);\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.set_timepoint(time, timestep, ddt_coefficients),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.accept_timestep(),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn stamp_reactive(&mut self, ctx: &super::GeneratedEvalContext<'_>, stamper: &mut super::GeneratedReactiveStamper<'_>) {\n",
    );
    out.push_str("        let _ = (&ctx, &stamper);\n");
    if devices.is_empty() {
        out.push_str("        let _ = (ctx, stamper, self);\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry cannot be stamped\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, feature) in feature_names.iter().enumerate() {
            writeln!(
                out,
                "            #[cfg(feature = {feature:?})]\n            Self::Device{index}(device) => device.stamp_reactive(ctx, stamper),"
            )?;
        }
        out.push_str("            Self::__NonExhaustive(value) => match *value {},\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("pub const BUILTIN_NAMES: &[&str] = &[\n");
    for (registry_name, feature) in registry_model_names.iter().zip(&feature_names) {
        writeln!(
            out,
            "    #[cfg(feature = {feature:?})]\n    {:?},",
            registry_name,
        )?;
    }
    out.push_str("];\n\n");
    out.push_str("pub fn builtin_names() -> &'static [&'static str] {\n");
    out.push_str("    BUILTIN_NAMES\n");
    out.push_str("}\n");
    out.push('\n');
    out.push_str("pub fn node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for ((device, registry_name), feature) in devices
        .iter()
        .zip(&registry_model_names)
        .zip(&feature_names)
    {
        writeln!(
            out,
            "        #[cfg(feature = {feature:?})]\n        {:?} => Some({}::Instance::TERMINAL_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name,
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn total_node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for ((device, registry_name), feature) in devices
        .iter()
        .zip(&registry_model_names)
        .zip(&feature_names)
    {
        writeln!(
            out,
            "        #[cfg(feature = {feature:?})]\n        {:?} => Some({}::Instance::NODE_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name,
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(
        "pub fn internal_node_names(model_name: &str) -> Option<&'static [&'static str]> {\n",
    );
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for ((device, registry_name), feature) in devices
        .iter()
        .zip(&registry_model_names)
        .zip(&feature_names)
    {
        writeln!(
            out,
            "        #[cfg(feature = {feature:?})]\n        {:?} => Some(&{}::Instance::INTERNAL_NODE_NAMES),",
            registry_name.to_ascii_uppercase(),
            device.folder_name,
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn branch_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for ((device, registry_name), feature) in devices
        .iter()
        .zip(&registry_model_names)
        .zip(&feature_names)
    {
        writeln!(
            out,
            "        #[cfg(feature = {feature:?})]\n        {:?} => Some({}::Instance::BRANCH_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name,
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(
        "pub fn instantiate(model_name: &str, nodes: &[usize], branches: &[usize], params: &[(String, crate::Value)]) -> Result<Option<GeneratedBuiltinKind>, String> {\n",
    );
    out.push_str("    let _ = (nodes, branches, params);\n");
    if devices.is_empty() {
        out.push_str("    let _ = (model_name, nodes, branches, params);\n");
        out.push_str("    Ok(None)\n");
    } else {
        out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
        for (index, ((device, registry_name), feature)) in devices
            .iter()
            .zip(&registry_model_names)
            .zip(&feature_names)
            .enumerate()
        {
            writeln!(
                out,
                "        #[cfg(feature = {feature:?})]\n        {:?} => {{",
                registry_name.to_ascii_uppercase()
            )?;
            writeln!(
                out,
                "            let mut instance = Box::new({}::Instance::new(nodes));",
                device.folder_name
            )?;
            out.push_str("            instance.set_branch_indices(branches);\n");
            out.push_str("            for (name, value) in params {\n");
            out.push_str(
                "                if let Err(error) = instance.set_parameter(name, *value) {\n",
            );
            out.push_str("                    if name.eq_ignore_ascii_case(\"m\") {\n");
            out.push_str("                        instance.set_multiplicity(*value)?;\n");
            out.push_str("                    } else {\n");
            out.push_str("                        return Err(error);\n");
            out.push_str("                    }\n");
            out.push_str("                }\n");
            out.push_str("            }\n");
            out.push_str("            instance.validate_parameters()?;\n");
            writeln!(
                out,
                "            Ok(Some(GeneratedBuiltinKind::Device{index}(instance)))"
            )?;
            out.push_str("        }\n");
        }
        out.push_str("        _ => Ok(None),\n");
        out.push_str("    }\n");
    }
    out.push_str("}\n");

    write_text_file_if_changed(registry_root.join("registry.rs"), &out)?;
    Ok(())
}
