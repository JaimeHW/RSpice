use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use rspice_veriloga::rust_backend::validate_generated_builtins;

const LEGACY_LOCAL_SLOT_MARKERS: &[&str] =
    &["_slot: &mut f64", "_slot: &mut [f64]", " = *var_", "*var_"];

#[test]
fn generated_veriloga_bundle_passes_the_authoritative_generator_audit() {
    let workspace_root = workspace_root();
    let generated_root = generated_veriloga_root();

    let manifest = validate_generated_builtins(
        &workspace_root.join("models/veriloga"),
        &generated_root,
        &workspace_root.join("crates/rspice-veriloga"),
        false,
    )
    .unwrap_or_else(|error| {
        panic!("generated Verilog-A bundle must pass generator audit: {error}")
    });

    assert!(
        manifest.device_count > 0,
        "authoritative generated-builtin manifest must not be empty"
    );
}

#[test]
fn generated_veriloga_devices_do_not_use_legacy_local_slot_abi() {
    let generated_root = generated_veriloga_root();
    assert!(
        generated_root.is_dir(),
        "generated Verilog-A source root is missing: {}",
        generated_root.display()
    );

    let mut failures = Vec::new();
    scan_generated_rust(&generated_root, &mut |path, source| {
        for marker in LEGACY_LOCAL_SLOT_MARKERS {
            if source.contains(marker) {
                failures.push(format!("{} contains `{marker}`", display_path(path)));
            }
        }
    });

    assert!(
        failures.is_empty(),
        "generated Verilog-A devices must use the local frame ABI instead of helper slot parameters:\n{}",
        failures.join("\n")
    );
}

#[test]
fn generated_veriloga_devices_include_compact_local_frame_helpers() {
    let generated_root = generated_veriloga_root();

    let mut saw_local_frame_type = false;
    let mut saw_local_frame_initialization = false;
    let mut saw_local_frame_field_access = false;
    let mut saw_local_frame_argument = false;
    scan_generated_rust(&generated_root, &mut |_path, source| {
        saw_local_frame_type |= source.contains("pub(crate) struct StampLocals");
        saw_local_frame_initialization |= source.contains("let mut l = StampLocals::default()");
        saw_local_frame_field_access |= source.contains("l.f");
        saw_local_frame_argument |= source.contains("&mut l");
    });

    assert!(
        saw_local_frame_type
            && saw_local_frame_initialization
            && saw_local_frame_field_access
            && saw_local_frame_argument,
        "expected at least one generated Verilog-A device to use the compact StampLocals frame ABI"
    );
}

#[test]
fn generated_veriloga_devices_use_the_partitioned_kernel_runtime() {
    let generated_root = generated_veriloga_root();

    let mut saw_kernel_runtime = false;
    let mut saw_kernel_alias = false;
    let mut saw_partitioned_stamp = false;
    scan_generated_rust(&generated_root, &mut |path, source| {
        if path
            .file_name()
            .is_some_and(|name| name == "kernel_runtime.rs")
        {
            saw_kernel_runtime |= source.contains("pub(crate) struct AdValue")
                && source.contains("pub(crate) struct Scratch")
                && source.contains("pub(crate) struct ReactiveScratch");
        }
        saw_kernel_alias |= source.contains("KernelAdValue")
            && (source.contains("KernelScratch") || source.contains("KernelReactiveScratch"));
        saw_partitioned_stamp |= path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().starts_with("stamp_blocks_"));
    });

    assert!(
        saw_kernel_runtime && saw_kernel_alias && saw_partitioned_stamp,
        "expected generated Verilog-A output to use the current partitioned kernel runtime"
    );
}

#[test]
fn generated_veriloga_noise_is_one_pass_and_allocation_free() {
    let generated_root = generated_veriloga_root();
    let mut noise_files = 0usize;
    let mut failures = Vec::new();

    scan_generated_rust(&generated_root, &mut |path, source| {
        if !path.file_name().is_some_and(|name| name == "noise.rs") {
            return;
        }
        noise_files += 1;
        if !source.contains("pub fn evaluate_noise_sources(") {
            failures.push(format!(
                "{} does not expose the one-pass noise ABI",
                display_path(path)
            ));
        }
        for marker in [
            "pub fn evaluate_noise_source(",
            "source_index",
            "noise_variable_",
            "Vec::new()",
            "vec![",
            "matches!(source_index",
        ] {
            if source.contains(marker) {
                failures.push(format!("{} contains `{marker}`", display_path(path)));
            }
        }
    });

    assert!(
        noise_files > 0,
        "generated bundle has no noise translation units"
    );
    assert!(
        failures.is_empty(),
        "generated Verilog-A noise evaluation must traverse once and allocate no heap storage:\n{}",
        failures.join("\n")
    );
}

#[test]
fn generated_veriloga_noise_uses_bounded_helpers_and_a_compact_workspace() {
    const MAX_ATOMIC_HELPER_LINES: usize = 2_500;

    let generated_root = generated_veriloga_root();
    let mut partitioned_noise_files = 0usize;
    let mut failures = Vec::new();

    scan_generated_rust(&generated_root, &mut |path, source| {
        if !path.file_name().is_some_and(|name| name == "noise.rs")
            || !source.contains("let mut w = [0.0;")
        {
            return;
        }
        if !source.contains("w.fill(0.0);") {
            failures.push(format!(
                "{} does not reset its fixed noise workspace in one operation",
                display_path(path)
            ));
        }

        let helper_marker = "    #[inline(never)]\n    fn noise_";
        for helper in source.split(helper_marker).skip(1) {
            partitioned_noise_files += 1;
            let lines = helper.lines().count();
            if lines > MAX_ATOMIC_HELPER_LINES {
                let name = helper.split_once('(').map_or("<unknown>", |(name, _)| name);
                failures.push(format!(
                    "{} helper `{name}` has {lines} lines (limit {MAX_ATOMIC_HELPER_LINES})",
                    display_path(path)
                ));
            }
        }
    });

    assert!(
        partitioned_noise_files > 0,
        "expected generated noise schedules to use bounded helper methods"
    );
    assert!(
        failures.is_empty(),
        "generated Verilog-A noise helpers must remain bounded and use compact workspaces:\n{}",
        failures.join("\n")
    );
}

#[test]
fn generated_model_features_match_the_core_feature_catalog() {
    let workspace_root = workspace_root();
    let generated_root = generated_veriloga_root();
    let registry = fs::read_to_string(generated_root.join("registry.rs"))
        .expect("read generated built-in registry");
    let core_manifest = fs::read_to_string(workspace_root.join("crates/rspice-core/Cargo.toml"))
        .expect("read rspice-core manifest");

    let mut model_features = BTreeSet::new();
    for line in registry.lines() {
        let Some(feature) = line
            .trim()
            .strip_prefix("#[cfg(feature = \"")
            .and_then(|line| line.strip_suffix("\")]"))
        else {
            continue;
        };
        if feature.starts_with("veriloga-model-") {
            model_features.insert(feature.to_string());
        }
    }

    assert_eq!(
        model_features.len(),
        42,
        "every generated model must have one stable compile feature"
    );
    for feature in &model_features {
        assert!(
            core_manifest.contains(&format!("{feature} = [\"veriloga-builtins-base\"]")),
            "rspice-core is missing generated model feature `{feature}`"
        );
        assert!(
            core_manifest.contains(&format!("    \"{feature}\",")),
            "the `veriloga-builtins-models` feature is missing `{feature}`"
        );
    }
    assert!(
        core_manifest.contains("veriloga-builtins-noise = [\"veriloga-builtins-base\"]"),
        "rspice-core must expose generated noise as an independent feature"
    );
    assert!(
        core_manifest.contains(
            "veriloga-builtins = [\"veriloga-builtins-models\", \"veriloga-builtins-noise\"]"
        ),
        "the compatibility feature must enable the complete model catalog and noise"
    );

    let mut device_modules = 0usize;
    scan_generated_rust(&generated_root, &mut |path, source| {
        if path.file_name().is_some_and(|name| name == "mod.rs")
            && path.parent().is_some_and(|parent| parent != generated_root)
        {
            device_modules += 1;
            assert!(
                source.contains("#[cfg(feature = \"veriloga-builtins-noise\")]\npub mod noise;"),
                "{} does not feature-gate its noise translation unit",
                display_path(path)
            );
        }
    });
    assert_eq!(device_modules, model_features.len());
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rspice-veriloga must live under the workspace crates directory")
        .to_path_buf()
}

fn generated_veriloga_root() -> PathBuf {
    workspace_root().join("crates/rspice-core/src/device/veriloga_generated")
}

fn scan_generated_rust(root: &Path, visit: &mut dyn FnMut(&Path, &str)) {
    let mut pending = vec![root.to_path_buf()];
    let mut scanned = 0usize;
    while let Some(path) = pending.pop() {
        let entries = fs::read_dir(&path)
            .unwrap_or_else(|error| panic!("read generated directory {}: {error}", path.display()));
        for entry in entries {
            let path = entry
                .unwrap_or_else(|error| panic!("read generated directory entry: {error}"))
                .path();
            if path.is_dir() {
                pending.push(path);
            } else if path.extension().is_some_and(|extension| extension == "rs") {
                let source = fs::read_to_string(&path).unwrap_or_else(|error| {
                    panic!("read generated Rust file {}: {error}", path.display())
                });
                visit(&path, &source);
                scanned += 1;
            }
        }
    }

    assert!(
        scanned > 0,
        "no generated Rust files were scanned under {}",
        root.display()
    );
}

fn display_path(path: &Path) -> String {
    path.strip_prefix(workspace_root())
        .unwrap_or(path)
        .display()
        .to_string()
}
