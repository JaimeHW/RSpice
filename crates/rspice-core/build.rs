use std::collections::BTreeMap;
use std::env;
use std::fmt::Write;
use std::path::{Path, PathBuf};

use rspice_veriloga::rust_backend::{
    GeneratedRustDevice, RustTranspiler, cleanup_stale_generated_device_folders,
    discover_veriloga_sources, write_generated_device, write_text_file_if_changed,
};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn main() {
    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_BUILTINS_DIR");
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

    let devices = generate_devices(&model_root, &generated_root)
        .unwrap_or_else(|error| panic!("failed to generate Verilog-A built-ins: {error}"));
    if devices.is_empty() {
        panic!(
            "Verilog-A built-ins feature is enabled, but no modules were discovered under '{}'",
            model_root.display()
        );
    }

    println!(
        "cargo:warning=Generated {} Verilog-A built-in device(s): {}",
        devices.len(),
        devices
            .iter()
            .map(|device| device.public_model_name.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    println!("cargo:rustc-cfg=rspice_veriloga_builtins_generated");
    write_registry(&generated_root, &devices).expect("write generated Verilog-A registry");
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
    reject_duplicate_public_names(&devices)?;
    cleanup_stale_generated_device_folders(
        devices_root,
        devices.iter().map(|device| device.folder_name.as_str()),
    )?;
    for device in &devices {
        write_generated_device(devices_root, device)?;
    }
    Ok(devices)
}

fn reject_duplicate_public_names(
    devices: &[GeneratedRustDevice],
) -> Result<(), Box<dyn std::error::Error>> {
    let mut seen = BTreeMap::new();
    for device in devices {
        let key = device.public_model_name.to_ascii_uppercase();
        if let Some(previous) = seen.insert(key, device) {
            return Err(format!(
                "duplicate generated Verilog-A model name '{}': '{}' and '{}' both resolve to the same public model name",
                device.public_model_name, previous.folder_name, device.folder_name
            )
            .into());
        }
    }
    Ok(())
}

fn write_registry(
    registry_root: &Path,
    devices: &[GeneratedRustDevice],
) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(registry_root)?;

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

    out.push_str("#[derive(Debug, Clone)]\n");
    out.push_str("pub enum GeneratedBuiltinKind {\n");
    for (index, device) in devices.iter().enumerate() {
        writeln!(out, "    Device{index}({}::Instance),", device.folder_name)?;
    }
    out.push_str("}\n\n");

    out.push_str("impl GeneratedBuiltinKind {\n");
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
    for device in devices {
        writeln!(out, "    {:?},", device.public_model_name)?;
    }
    out.push_str("];\n\n");
    out.push_str("pub fn builtin_names() -> &'static [&'static str] {\n");
    out.push_str("    BUILTIN_NAMES\n");
    out.push_str("}\n");
    out.push_str("\n");
    out.push_str("pub fn node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for device in devices {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::TERMINAL_COUNT),",
            device.public_model_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn total_node_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for device in devices {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::NODE_COUNT),",
            device.public_model_name.to_ascii_uppercase(),
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
    for device in devices {
        writeln!(
            out,
            "        {:?} => Some(&{}::Instance::INTERNAL_NODE_NAMES),",
            device.public_model_name.to_ascii_uppercase(),
            device.folder_name
        )?;
    }
    out.push_str("        _ => None,\n");
    out.push_str("    }\n");
    out.push_str("}\n\n");
    out.push_str("pub fn branch_count(model_name: &str) -> Option<usize> {\n");
    out.push_str("    match model_name.to_ascii_uppercase().as_str() {\n");
    for device in devices {
        writeln!(
            out,
            "        {:?} => Some({}::Instance::BRANCH_COUNT),",
            device.public_model_name.to_ascii_uppercase(),
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
        for (index, device) in devices.iter().enumerate() {
            writeln!(
                out,
                "        {:?} => {{",
                device.public_model_name.to_ascii_uppercase()
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
