use std::collections::VecDeque;
use std::env;
use std::fmt::Write;
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Instant;

use crate::canonical_ir::StableDigest;
use crate::{CompilerOptions, VerilogACompiler};

use super::{
    GeneratedBuiltinManifest, GeneratedRustDevice, RustBackendSelection, RustTranspiler,
    VERILOGA_DISCOVERY_SKIP_MARKER, VerilogASourceCandidate,
    cleanup_stale_generated_device_folders, discover_veriloga_sources,
    parse_generated_builtin_manifest, render_generated_builtin_manifest,
    resolve_generated_registry_model_names, write_generated_device, write_text_file_if_changed,
};

type BuiltinResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub const GENERATED_BUILTIN_MANIFEST_FILE_NAME: &str = "manifest.txt";
pub const REGENERATE_BUILTINS_COMMAND: &str = "cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins";

const GENERATOR_SOURCE_DIGEST_INPUTS: &[&str] = &[
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
    pub hybrid: usize,
    pub legacy_device: usize,
}

impl BuiltinBackendSelectionCounts {
    fn record(&mut self, selection: RustBackendSelection) {
        match selection {
            RustBackendSelection::ScalarOptIr => self.scalar += 1,
            RustBackendSelection::ScalarHybrid => self.hybrid += 1,
            RustBackendSelection::LegacyDevice => self.legacy_device += 1,
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
    let (devices, backend_counts, fallback_reasons) =
        generate_devices_with_stack(model_root.to_path_buf(), None, progress, jobs)?;
    reject_legacy_backend_selections(&backend_counts, &fallback_reasons)?;
    if devices.is_empty() {
        return Err(format!(
            "Verilog-A built-ins source directory '{}' does not contain any discovered modules",
            model_root.display()
        )
        .into());
    }

    reject_legacy_ad_runtime(&devices)?;
    write_devices(generated_root, &devices)?;
    remove_stale_support(generated_root)?;
    write_registry(generated_root, &devices)?;

    let manifest = GeneratedBuiltinManifest {
        source_tree_digest,
        generator_digest,
        device_count: devices.len(),
    };
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

    let (devices, backend_counts, fallback_reasons) = generate_devices_with_stack(
        model_root.to_path_buf(),
        Some(filter.to_string()),
        progress,
        jobs,
    )?;
    reject_legacy_backend_selections(&backend_counts, &fallback_reasons)?;
    if devices.is_empty() {
        return Err(format!(
            "Verilog-A built-ins source directory '{}' does not contain any modules matching filter '{filter}'",
            model_root.display()
        )
        .into());
    }

    reject_legacy_ad_runtime(&devices)?;
    write_device_subset(generated_root, &devices)?;

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
                "current manifest has source_tree_digest={}, generator_digest={}, device_count={}",
                manifest.source_tree_digest, manifest.generator_digest, manifest.device_count
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
    Ok(StableDigest::from_text(&input).as_hex())
}

fn file_digest(path: &Path) -> BuiltinResult<String> {
    let bytes = fs::read(path)?;
    let mut input = String::new();
    input.push_str(&bytes.len().to_string());
    input.push('\0');
    input.push_str(&String::from_utf8_lossy(&bytes));
    input.push('\0');
    Ok(StableDigest::from_text(&input).as_hex())
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
    (manifest.source_tree_digest == source_tree_digest
        && manifest.generator_digest == generator_digest
        && manifest.device_count > 0)
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
    "AdValue",
    "GenericScratch",
    "GenericReactiveScratch",
    "scratch:",
    "reactive_scratch:",
    "scratch.",
    "reactive_scratch.",
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

fn remove_stale_support(generated_root: &Path) -> BuiltinResult<()> {
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

fn tree_digest(root: &Path, emit_cargo_rerun: bool) -> BuiltinResult<String> {
    let mut files = Vec::new();
    collect_tree_files(root, &mut files)?;
    files.sort();

    let mut input = String::new();
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
        input.push_str(&relative);
        input.push('\0');
        input.push_str(&bytes.len().to_string());
        input.push('\0');
        input.push_str(&String::from_utf8_lossy(&bytes));
        input.push('\0');
    }

    Ok(StableDigest::from_text(&input).as_hex())
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
    for generated in generated {
        backend_counts.record(generated.backend);
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
    Ok((devices, backend_counts, fallback_reasons))
}

fn generate_devices_sequential(
    model_root: &Path,
    work_items: Vec<BuiltinModuleWorkItem>,
    progress: bool,
) -> BuiltinResult<Vec<GeneratedBuiltinModule>> {
    let total_modules = work_items.len();
    let mut options = CompilerOptions::default();
    options.include_paths.push(model_root.to_path_buf());
    let compiler = VerilogACompiler::new(options);
    let transpiler = RustTranspiler::new_auto(Default::default());
    let mut generated = Vec::with_capacity(total_modules);
    for (index, item) in work_items.into_iter().enumerate() {
        generated.push(generate_device_work_item(
            model_root,
            &compiler,
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
                let mut options = CompilerOptions::default();
                options.include_paths.push(model_root.clone());
                let compiler = VerilogACompiler::new(options);
                let transpiler = RustTranspiler::new_auto(Default::default());

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
                        &compiler,
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
    compiler: &VerilogACompiler,
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

    #[test]
    fn builtin_subset_filter_matches_module_without_selecting_siblings() {
        let model_root = Path::new("models");
        let candidates = vec![VerilogASourceCandidate {
            path: PathBuf::from("models/cmc/hicum.va"),
            modules: vec!["hicuml2".to_string(), "hicuml0".to_string()],
        }];
        let filter = BuiltinSourceFilter::new("hicuml2").expect("filter");

        let work_items = builtin_module_work_items(model_root, &candidates, Some(&filter));

        assert_eq!(
            work_items,
            vec![BuiltinModuleWorkItem {
                path: PathBuf::from("models/cmc/hicum.va"),
                module: "hicuml2".to_string(),
            }]
        );
    }

    #[test]
    fn builtin_subset_filter_matches_all_modules_when_path_matches() {
        let model_root = Path::new("models");
        let candidates = vec![VerilogASourceCandidate {
            path: PathBuf::from("models/cmc/hicum.va"),
            modules: vec!["hicuml2".to_string(), "hicuml0".to_string()],
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
    fn builtin_generation_rejects_legacy_backend_selections() {
        let counts = BuiltinBackendSelectionCounts {
            scalar: 1,
            hybrid: 0,
            legacy_device: 1,
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
}

fn write_registry(registry_root: &Path, devices: &[GeneratedRustDevice]) -> BuiltinResult<()> {
    std::fs::create_dir_all(registry_root)?;
    let registry_model_names = resolve_generated_registry_model_names(devices);

    let mut out = String::new();
    out.push_str("// Generated by rspice-veriloga-gen. Do not edit.\n\n");
    for device in devices {
        writeln!(
            out,
            "#[allow(non_snake_case)]\n#[path = \"{}/mod.rs\"]\npub mod {};",
            device.folder_name, device.folder_name
        )?;
    }
    out.push('\n');

    out.push_str("#[derive(Clone)]\n");
    out.push_str("pub enum GeneratedBuiltinKind {\n");
    for (index, device) in devices.iter().enumerate() {
        writeln!(
            out,
            "    Device{index}(Box<{}::Instance>),",
            device.folder_name
        )?;
    }
    out.push_str("}\n\n");

    out.push_str("impl GeneratedBuiltinKind {\n");
    out.push_str("    pub fn restore_from_snapshot(&mut self, snapshot: Self) {\n");
    if devices.is_empty() {
        out.push_str("        let _ = (self, snapshot);\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry cannot be restored\")\n",
        );
    } else {
        out.push_str("        match (self, snapshot) {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            (Self::Device{index}(active), Self::Device{index}(snapshot)) => active.restore_from_snapshot(*snapshot),"
            )?;
        }
        out.push_str("            (active, snapshot) => *active = snapshot,\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn stamp(&mut self, ctx: &super::GeneratedEvalContext<'_>, stamper: &mut super::GeneratedStamper<'_>) {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = (ctx, stamper, self);\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry cannot be stamped\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            Self::Device{index}(device) => device.stamp(ctx, stamper),"
            )?;
        }
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn set_timepoint(&mut self, time: crate::Value, timestep: crate::Value, ddt_coefficients: super::GeneratedDdtCoefficients) {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = (self, time, timestep, ddt_coefficients);\n");
    } else {
        out.push_str("        match self {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            Self::Device{index}(device) => device.set_timepoint(time, timestep, ddt_coefficients),"
            )?;
        }
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str("    pub fn accept_timestep(&mut self) {\n");
    if devices.is_empty() {
        out.push_str("        let _ = self;\n");
    } else {
        out.push_str("        match self {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            Self::Device{index}(device) => device.accept_timestep(),"
            )?;
        }
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push('\n');
    out.push_str(
        "    pub fn stamp_reactive(&mut self, ctx: &super::GeneratedEvalContext<'_>, stamper: &mut super::GeneratedReactiveStamper<'_>) {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = (ctx, stamper, self);\n");
        out.push_str(
            "        unreachable!(\"empty generated Verilog-A registry cannot be stamped\")\n",
        );
    } else {
        out.push_str("        match self {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            Self::Device{index}(device) => device.stamp_reactive(ctx, stamper),"
            )?;
        }
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("}\n\n");

    out.push_str("pub const BUILTIN_NAMES: &[&str] = &[\n");
    for registry_name in &registry_model_names {
        writeln!(out, "    {:?},", registry_name)?;
    }
    out.push_str("];\n\n");
    out.push_str("pub fn builtin_names() -> &'static [&'static str] {\n");
    out.push_str("    BUILTIN_NAMES\n");
    out.push_str("}\n");
    out.push('\n');
    out.push_str("pub fn node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for (device, registry_name) in devices.iter().zip(&registry_model_names) {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::TERMINAL_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn total_node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for (device, registry_name) in devices.iter().zip(&registry_model_names) {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::NODE_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(
        "pub fn internal_node_names(model_name: &str) -> Option<&'static [&'static str]> {\n",
    );
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for (device, registry_name) in devices.iter().zip(&registry_model_names) {
        writeln!(
            out,
            "        {:?} => Some(&{}::Instance::INTERNAL_NODE_NAMES),",
            registry_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn branch_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for (device, registry_name) in devices.iter().zip(&registry_model_names) {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::BRANCH_COUNT),",
            registry_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str(
        "pub fn instantiate(model_name: &str, nodes: &[usize], branches: &[usize], params: &[(String, crate::Value)]) -> Result<Option<GeneratedBuiltinKind>, String> {\n",
    );
    if devices.is_empty() {
        out.push_str("    let _ = (model_name, nodes, branches, params);\n");
        out.push_str("    Ok(None)\n");
    } else {
        out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
        for (index, (device, registry_name)) in
            devices.iter().zip(&registry_model_names).enumerate()
        {
            writeln!(
                out,
                "        {:?} => {{",
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
            out.push_str("                        instance.set_multiplicity(*value);\n");
            out.push_str("                    } else {\n");
            out.push_str("                        return Err(error);\n");
            out.push_str("                    }\n");
            out.push_str("                }\n");
            out.push_str("            }\n");
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
