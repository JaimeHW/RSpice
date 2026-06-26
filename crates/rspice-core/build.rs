use std::env;
use std::fmt::Write;
use std::fs;
use std::path::{Path, PathBuf};

use rspice_veriloga::canonical_ir::StableDigest;
use rspice_veriloga::rust_backend::{
    GeneratedBuiltinManifest, GeneratedRustDevice, RustTranspiler,
    cleanup_stale_generated_device_folders, discover_veriloga_sources,
    parse_generated_builtin_manifest, render_generated_builtin_manifest,
    render_runtime_support_module, resolve_generated_registry_model_names, write_generated_device,
    write_text_file_if_changed,
};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const MANIFEST_FILE_NAME: &str = "manifest.txt";

fn main() {
    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_BUILTINS_DIR");
    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_REGENERATE_BUILTINS");
    println!("cargo:rustc-check-cfg=cfg(rspice_veriloga_builtins_generated)");

    let generated_root = generated_source_root();

    let enabled = env::var_os("CARGO_FEATURE_VERILOGA_BUILTINS").is_some();
    if !enabled {
        return;
    }

    let model_root = env::var_os("RSPICE_VERILOGA_BUILTINS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_model_root);

    println!("cargo:rerun-if-changed={}", model_root.display());
    println!("cargo:rerun-if-changed={}", generated_root.display());
    if !model_root.exists() {
        panic!(
            "Verilog-A built-ins feature is enabled, but source directory '{}' does not exist. Set RSPICE_VERILOGA_BUILTINS_DIR or add models under the workspace models/veriloga directory.",
            model_root.display()
        );
    }
    if !model_root.is_dir() {
        panic!(
            "Verilog-A built-ins source path '{}' is not a directory",
            model_root.display()
        );
    }

    let source_tree_digest = tree_digest(&model_root, true)
        .unwrap_or_else(|error| panic!("failed to fingerprint Verilog-A built-ins: {error}"));
    let generator_digest = tree_digest(&generator_source_root(), false)
        .unwrap_or_else(|error| panic!("failed to fingerprint Verilog-A generator: {error}"));
    let force_regenerate = env::var_os("RSPICE_VERILOGA_REGENERATE_BUILTINS").is_some();

    if !force_regenerate
        && let Some(manifest) =
            read_generated_manifest(&generated_root, &source_tree_digest, &generator_digest)
    {
        let _ = manifest;
        println!("cargo:rustc-cfg=rspice_veriloga_builtins_generated");
        return;
    }

    let devices = generate_devices(&model_root, &generated_root)
        .unwrap_or_else(|error| panic!("failed to generate Verilog-A built-ins: {error}"));
    if devices.is_empty() {
        panic!(
            "Verilog-A built-ins feature is enabled, but no modules were discovered under '{}'",
            model_root.display()
        );
    }

    println!("cargo:rustc-cfg=rspice_veriloga_builtins_generated");
    write_support(&generated_root).expect("write generated Verilog-A support runtime");
    write_registry(&generated_root, &devices).expect("write generated Verilog-A registry");
    write_generated_manifest(
        &generated_root,
        GeneratedBuiltinManifest {
            source_tree_digest,
            generator_digest,
            device_count: devices.len(),
        },
    )
    .expect("write generated Verilog-A manifest");
}

fn generated_source_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    manifest_dir.join("src/device/veriloga_generated")
}

fn default_model_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("rspice-core must live under workspace crates directory")
        .join("models/veriloga")
}

fn generator_source_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    manifest_dir
        .parent()
        .expect("rspice-core must live under workspace crates directory")
        .join("rspice-veriloga/src")
}

fn read_generated_manifest(
    generated_root: &Path,
    source_tree_digest: &str,
    generator_digest: &str,
) -> Option<GeneratedBuiltinManifest> {
    if !generated_root.join("registry.rs").is_file() {
        return None;
    }
    if !generated_root.join("support.rs").is_file() {
        return None;
    }
    let manifest_path = generated_root.join(MANIFEST_FILE_NAME);
    let manifest = parse_generated_builtin_manifest(&fs::read_to_string(manifest_path).ok()?)?;
    (manifest.source_tree_digest == source_tree_digest
        && manifest.generator_digest == generator_digest
        && manifest.device_count > 0)
        .then_some(manifest)
}

fn write_support(generated_root: &Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(generated_root)?;
    write_text_file_if_changed(
        generated_root.join("support.rs"),
        &render_runtime_support_module(),
    )?;
    Ok(())
}

fn write_generated_manifest(
    generated_root: &Path,
    manifest: GeneratedBuiltinManifest,
) -> Result<(), Box<dyn std::error::Error>> {
    write_text_file_if_changed(
        generated_root.join(MANIFEST_FILE_NAME),
        &render_generated_builtin_manifest(&manifest),
    )?;
    Ok(())
}

fn tree_digest(root: &Path, emit_rerun: bool) -> Result<String, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    collect_tree_files(root, &mut files)?;
    files.sort();

    let mut input = String::new();
    for path in files {
        if emit_rerun {
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

fn collect_tree_files(
    root: &Path,
    files: &mut Vec<PathBuf>,
) -> Result<(), Box<dyn std::error::Error>> {
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
    devices_root: &Path,
) -> Result<Vec<GeneratedRustDevice>, Box<dyn std::error::Error>> {
    let candidates = discover_veriloga_sources(model_root)?;
    let mut options = CompilerOptions::default();
    options.include_paths.push(model_root.to_path_buf());
    let compiler = VerilogACompiler::new(options);
    let transpiler = RustTranspiler::default();
    let mut devices = Vec::new();

    for candidate in candidates {
        println!("cargo:rerun-if-changed={}", candidate.path.display());
        for module in &candidate.modules {
            let compiled =
                compiler.compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))?;
            for dependency in &compiled.dependencies {
                println!("cargo:rerun-if-changed={}", dependency.display());
            }
            let device = transpiler.transpile(&compiled.artifact)?;
            devices.push(device);
        }
    }

    devices.sort_by(|left, right| {
        left.public_model_name
            .cmp(&right.public_model_name)
            .then_with(|| left.folder_name.cmp(&right.folder_name))
    });
    cleanup_stale_generated_device_folders(
        devices_root,
        devices.iter().map(|device| device.folder_name.as_str()),
    )?;
    for device in &devices {
        write_generated_device(devices_root, device)?;
    }
    Ok(devices)
}

fn write_registry(
    registry_root: &Path,
    devices: &[GeneratedRustDevice],
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(registry_root)?;
    let registry_model_names = resolve_generated_registry_model_names(devices);

    let mut out = String::new();
    out.push_str("// Generated by rspice-core/build.rs. Do not edit.\n\n");
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
        writeln!(out, "    Device{index}({}::Instance),", device.folder_name)?;
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
                "            (Self::Device{index}(active), Self::Device{index}(snapshot)) => active.restore_from_snapshot(snapshot),"
            )?;
        }
        out.push_str("            (active, snapshot) => *active = snapshot,\n");
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("\n");
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
    out.push_str("\n");
    out.push_str(
        "    pub fn set_timepoint(&mut self, time: crate::Value, timestep: crate::Value) {\n",
    );
    if devices.is_empty() {
        out.push_str("        let _ = (self, time, timestep);\n");
    } else {
        out.push_str("        match self {\n");
        for (index, _device) in devices.iter().enumerate() {
            writeln!(
                out,
                "            Self::Device{index}(device) => device.set_timepoint(time, timestep),"
            )?;
        }
        out.push_str("        }\n");
    }
    out.push_str("    }\n");
    out.push_str("\n");
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
    out.push_str("\n");
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
    out.push_str("\n");
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
                "            let mut instance = {}::Instance::new(nodes);",
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
