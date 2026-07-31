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

/// Each device is self-contained, and nothing streams a shared scratch frame.
///
/// This replaces three tests that pinned the opposite: a `StampLocals` frame
/// ABI, `KernelAdValue`/`KernelScratch` aliases over a partitioned
/// `kernel_runtime`, and `stamp_blocks_*` files. Those were the tier machinery's
/// shape, and asserting it now would assert that the rebuild has not happened —
/// the memory-indexed interpreter they describe is the thing the whole program
/// exists to remove. What is worth pinning is what replaced it: real control
/// flow in one body per device, with the helpers it uses carried alongside.
#[test]
fn generated_veriloga_devices_carry_their_own_helpers() {
    let generated_root = generated_veriloga_root();

    let mut partitioned = Vec::new();
    let mut streamed = Vec::new();
    let mut runtime_is_empty = false;
    let mut saw_packed_lane_type = false;
    scan_generated_rust(&generated_root, &mut |path, source| {
        let name = path.file_name().unwrap_or_default().to_string_lossy();
        if name == "kernel_runtime.rs" {
            // Empty is the whole point: every helper the packed form calls is
            // emitted into the device that calls it.
            runtime_is_empty = !source.contains("fn ");
            return;
        }
        if name.starts_with("stamp_blocks_") {
            partitioned.push(display_path(path));
        }
        if source.contains("KernelScratch") || source.contains("StampLocals") {
            streamed.push(display_path(path));
        }
        saw_packed_lane_type |= source.contains("struct Lanes<const N: usize>");
    });

    assert!(
        partitioned.is_empty(),
        "a body is one function now, not a partition:\n{}",
        partitioned.join("\n")
    );
    assert!(
        streamed.is_empty(),
        "no device streams a shared scratch frame any more:\n{}",
        streamed.join("\n")
    );
    assert!(
        runtime_is_empty,
        "kernel_runtime.rs has no callers left and must not be regenerated with a body"
    );
    assert!(
        saw_packed_lane_type,
        "a differentiated device carries the packed-lane newtype it emits against"
    );
}

#[test]
fn generated_veriloga_noise_is_one_pass_and_allocation_free() {
    let generated_root = generated_veriloga_root();
    let mut noise_files = 0usize;
    let mut failures = Vec::new();

    scan_generated_rust(&generated_root, &mut |path, source| {
        if path.file_name().is_none_or(|name| name != "noise.rs") {
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

/// Noise is a slice of the body, not a re-derivation of it.
///
/// This replaces a test that required a fixed `w` workspace reset in one
/// operation and a partition into bounded `noise_*` helpers. Both were
/// properties of a generator that re-emitted the whole model per magnitude —
/// which is why `noise.rs` was 55 MB, over half the checked-in tree, and why a
/// two-terminal resistor carried 3,722 lines of it. There is no workspace and no
/// partition now: the magnitudes are cut from the same CFG the stamp is, so what
/// is worth pinning is that the size collapse held.
#[test]
fn generated_veriloga_noise_is_a_slice_rather_than_a_second_model() {
    /// The whole corpus's noise, against 55,347,327 bytes before the slice.
    const MAX_TOTAL_NOISE_BYTES: usize = 20_000_000;

    let generated_root = generated_veriloga_root();
    let mut total = 0usize;
    let mut workspaces = Vec::new();

    scan_generated_rust(&generated_root, &mut |path, source| {
        if path.file_name().is_none_or(|name| name != "noise.rs") {
            return;
        }
        total += source.len();
        if source.contains("let mut w = [0.0;") {
            workspaces.push(display_path(path));
        }
    });

    assert!(
        workspaces.is_empty(),
        "a noise body is a slice of the stamp's CFG and needs no scratch workspace:\n{}",
        workspaces.join("\n")
    );
    assert!(
        total > 0 && total <= MAX_TOTAL_NOISE_BYTES,
        "generated noise is {total} bytes; the slice must keep it under {MAX_TOTAL_NOISE_BYTES}"
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
