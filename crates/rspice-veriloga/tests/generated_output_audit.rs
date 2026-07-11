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
