use std::fmt::Write;
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

use crate::canonical_ir::StableDigest;
use crate::{CompilerOptions, VerilogACompiler};

use super::{
    GeneratedBuiltinManifest, GeneratedRustDevice, RustBackendSelection, RustTranspiler,
    VERILOGA_DISCOVERY_SKIP_MARKER, cleanup_stale_generated_device_folders,
    discover_veriloga_sources, parse_generated_builtin_manifest, render_generated_builtin_manifest,
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
    validate_model_root(model_root)?;

    let source_tree_digest = tree_digest(model_root, false)?;
    let generator_digest = generator_digest(generator_root, false)?;
    let (devices, backend_counts) =
        generate_devices_with_stack(model_root.to_path_buf(), progress)?;
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
    progress: bool,
) -> BuiltinResult<(Vec<GeneratedRustDevice>, BuiltinBackendSelectionCounts)> {
    let candidates = discover_veriloga_sources(model_root)?;
    let total_modules = candidates
        .iter()
        .map(|candidate| candidate.modules.len())
        .sum::<usize>();
    let mut options = CompilerOptions::default();
    options.include_paths.push(model_root.to_path_buf());
    let compiler = VerilogACompiler::new(options);
    let transpiler = RustTranspiler::new_auto(Default::default());
    let mut devices = Vec::new();
    let mut backend_counts = BuiltinBackendSelectionCounts::default();
    let mut module_index = 0usize;

    for candidate in candidates {
        for module in &candidate.modules {
            module_index += 1;
            if progress {
                eprintln!(
                    "generating Verilog-A built-in {module_index}/{total_modules}: {} :: {module}",
                    candidate
                        .path
                        .strip_prefix(model_root)
                        .unwrap_or(&candidate.path)
                        .display()
                );
                let _ = std::io::stderr().flush();
            }
            let compiled =
                compiler.compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))?;
            let report = transpiler.transpile_with_report(&compiled.artifact)?;
            backend_counts.record(report.backend);
            devices.push(report.device);
        }
    }

    devices.sort_by(|left, right| {
        left.public_model_name
            .cmp(&right.public_model_name)
            .then_with(|| left.folder_name.cmp(&right.folder_name))
    });
    Ok((devices, backend_counts))
}

fn generate_devices_with_stack(
    model_root: PathBuf,
    progress: bool,
) -> Result<
    (Vec<GeneratedRustDevice>, BuiltinBackendSelectionCounts),
    Box<dyn std::error::Error + Send + Sync>,
> {
    std::thread::Builder::new()
        .name("rspice-veriloga-builtin-generator".to_string())
        .stack_size(256 * 1024 * 1024)
        .spawn(move || generate_devices(&model_root, progress).map_err(|error| error.to_string()))?
        .join()
        .map_err(|_| "Verilog-A built-in generator thread panicked")?
        .map_err(|error| error.into())
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
