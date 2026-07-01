//! Rust backend selection frontier for shipped Verilog-A models.
//!
//! Run explicitly with:
//! `cargo test -p rspice-veriloga --test rust_backend_frontier shipped_rust_backend_frontier -- --ignored --nocapture`
//!
//! To focus on one package, file, or module:
//! `RSPICE_RUST_BACKEND_FRONTIER_FILTER=asmhemt cargo test -p rspice-veriloga --test rust_backend_frontier shipped_rust_backend_frontier -- --ignored --nocapture`

use rspice_veriloga::rust_backend::{
    RustBackendSelection, RustTranspileOptions, RustTranspiler, discover_veriloga_sources,
};
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::env;
use std::path::Path;

const FILTER_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_FILTER";
const REQUIRE_NO_LEGACY_ENV: &str = "RSPICE_RUST_BACKEND_FRONTIER_REQUIRE_NO_LEGACY";

#[test]
#[ignore = "full shipped Rust-backend frontier audit; run explicitly while scalar coverage is still moving"]
fn shipped_rust_backend_frontier() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    let mut sources =
        discover_veriloga_sources(&root).expect("discover shipped Verilog-A model sources");

    if let Ok(filter) = env::var(FILTER_ENV)
        && !filter.trim().is_empty()
    {
        sources = sources
            .into_iter()
            .filter(|source| source_matches_filter(source, &root, &filter))
            .collect();
        assert!(
            !sources.is_empty(),
            "{FILTER_ENV}={filter:?} did not match any shipped Verilog-A source or module"
        );
    }

    let mut options = CompilerOptions::default();
    options.include_paths.push(root.clone());
    let compiler = VerilogACompiler::new(options);
    let transpiler = RustTranspiler::new_auto(RustTranspileOptions::default());
    let mut counts = BackendSelectionCounts::default();
    let mut failures = Vec::new();

    for source in sources {
        for module in &source.modules {
            let compiled = match compiler
                .compile_file_canonical_ir_with_metadata(&source.path, Some(module))
            {
                Ok(compiled) => compiled,
                Err(error) => {
                    failures.push(format!(
                        "{} :: {} failed to compile canonical IR: {error}",
                        source
                            .path
                            .strip_prefix(&root)
                            .unwrap_or(&source.path)
                            .display(),
                        module
                    ));
                    continue;
                }
            };

            match transpiler.transpile_with_report(&compiled.artifact) {
                Ok(report) => {
                    counts.record(report.backend);
                    eprintln!(
                        "{:?} :: {} :: {}",
                        report.backend,
                        source
                            .path
                            .strip_prefix(&root)
                            .unwrap_or(&source.path)
                            .display(),
                        module
                    );
                }
                Err(error) => failures.push(format!(
                    "{} :: {} failed to transpile Rust backend: {error}",
                    source
                        .path
                        .strip_prefix(&root)
                        .unwrap_or(&source.path)
                        .display(),
                    module
                )),
            }
        }
    }

    assert!(
        failures.is_empty(),
        "frontier failures:\n{}",
        failures.join("\n")
    );
    eprintln!(
        "backend frontier: scalar={}, scalar-hybrid={}, legacy-native-local-fallback={}, legacy-device={}",
        counts.scalar, counts.hybrid, counts.legacy_native_local_fallback, counts.legacy_device
    );

    if env::var_os(REQUIRE_NO_LEGACY_ENV).is_some() {
        assert_eq!(
            counts.legacy_native_local_fallback + counts.legacy_device,
            0,
            "legacy backend selections remain in the shipped frontier"
        );
    }
}

fn source_matches_filter(
    source: &rspice_veriloga::rust_backend::VerilogASourceCandidate,
    root: &Path,
    filter: &str,
) -> bool {
    let filter = filter.to_ascii_lowercase();
    let path = source
        .path
        .strip_prefix(root)
        .unwrap_or(&source.path)
        .to_string_lossy()
        .replace('\\', "/")
        .to_ascii_lowercase();

    path.contains(&filter)
        || source
            .modules
            .iter()
            .any(|module| module.to_ascii_lowercase().contains(&filter))
}

#[derive(Debug, Default)]
struct BackendSelectionCounts {
    scalar: usize,
    hybrid: usize,
    legacy_native_local_fallback: usize,
    legacy_device: usize,
}

impl BackendSelectionCounts {
    fn record(&mut self, selection: RustBackendSelection) {
        match selection {
            RustBackendSelection::ScalarOptIr => self.scalar += 1,
            RustBackendSelection::ScalarHybrid => self.hybrid += 1,
            RustBackendSelection::LegacyNativeLocalFallback => {
                self.legacy_native_local_fallback += 1
            }
            RustBackendSelection::LegacyDevice => self.legacy_device += 1,
        }
    }
}
