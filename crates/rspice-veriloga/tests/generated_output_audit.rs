use std::fs;
use std::path::{Path, PathBuf};

const LEGACY_AD_MARKERS: &[&str] = &[
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

const LEGACY_LOCAL_SLOT_MARKERS: &[&str] =
    &["_slot: &mut f64", "_slot: &mut [f64]", " = *var_", "*var_"];

#[test]
fn generated_veriloga_devices_do_not_use_legacy_ad_runtime() {
    let generated_root = generated_veriloga_root();
    assert!(
        generated_root.is_dir(),
        "generated Verilog-A source root is missing: {}",
        generated_root.display()
    );

    let mut failures = Vec::new();
    scan_generated_rust(&generated_root, &mut |path, source| {
        for marker in LEGACY_AD_MARKERS {
            if source.contains(marker) {
                failures.push(format!("{} contains `{marker}`", display_path(path)));
            }
        }
    });

    assert!(
        failures.is_empty(),
        "generated Verilog-A devices must stay off the legacy AD runtime:\n{}",
        failures.join("\n")
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
fn generated_veriloga_devices_include_local_frame_helpers() {
    let generated_root = generated_veriloga_root();
    assert!(
        generated_root.is_dir(),
        "generated Verilog-A source root is missing: {}",
        generated_root.display()
    );

    let mut saw_local_frame_type = false;
    let mut saw_local_frame_access = false;
    scan_generated_rust(&generated_root, &mut |_path, source| {
        saw_local_frame_type |= source.contains("pub(crate) struct StampLocals");
        saw_local_frame_access |= source.contains("locals.var_");
    });

    assert!(
        saw_local_frame_type && saw_local_frame_access,
        "expected at least one generated Verilog-A device to use StampLocals local frame helpers"
    );
}

fn generated_veriloga_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../rspice-core/src/device/veriloga_generated")
        .components()
        .collect()
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
    path.strip_prefix(Path::new(env!("CARGO_MANIFEST_DIR")).join(".."))
        .unwrap_or(path)
        .display()
        .to_string()
}
