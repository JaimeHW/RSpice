//! Machine-code identity census over the shipped Verilog-A model set.
//!
//! The block model is meant to change nothing about what a single-block
//! program compiles to. This census makes that measurable rather than
//! argued: it compiles every shipped module through the production x64 path
//! and digests the published image, so the same run on any two revisions
//! either produces the same census digest or names the model that moved.
//!
//! It is `#[ignore]`d because compiling the whole compact-model census is
//! release-qualification work, not a per-commit gate. Run it with
//! `--release --features native -- --ignored --nocapture`.

use std::path::{Path, PathBuf};

use crate::rust_backend::discover_veriloga_sources;
use crate::{CompilerOptions, VerilogACompiler};

fn shipped_model_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga")
}

/// One shipped module's compiled machine-code digest.
struct ModelImageDigest {
    name: String,
    digest: String,
    bytes: usize,
}

fn census() -> Vec<ModelImageDigest> {
    let root = shipped_model_root();
    let candidates = discover_veriloga_sources(&root).expect("discover shipped Verilog-A sources");
    let mut digests = Vec::new();
    for candidate in candidates {
        for module in &candidate.modules {
            let mut options = CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiler = VerilogACompiler::new(options);
            let runtime = compiler
                .compile_file_runtime_with_metadata(&candidate.path, Some(module))
                .unwrap_or_else(|error| {
                    panic!("compile {} :: {module}: {error}", candidate.path.display())
                });
            let native = crate::native::x64::compile_model_with_canonical_ir(
                &runtime.model,
                &runtime.canonical_ir,
            )
            .unwrap_or_else(|error| {
                panic!(
                    "native compile {} :: {module}: {error}",
                    candidate.path.display()
                )
            });
            let image = native.image_bytes();
            digests.push(ModelImageDigest {
                name: module.to_string(),
                digest: blake3::hash(image).to_hex().to_string(),
                bytes: image.len(),
            });
        }
    }
    digests.sort_by(|left, right| left.name.cmp(&right.name));
    digests
}

#[test]
#[ignore = "release qualification; run with --release --features native -- --ignored --nocapture"]
fn shipped_model_machine_code_census_digest() {
    let census = census();
    assert!(
        !census.is_empty(),
        "the shipped model tree produced no compilable module"
    );
    let mut combined = blake3::Hasher::new();
    for entry in &census {
        eprintln!(
            "code-identity model={} bytes={} digest={}",
            entry.name, entry.bytes, entry.digest
        );
        combined.update(entry.name.as_bytes());
        combined.update(entry.digest.as_bytes());
    }
    eprintln!(
        "code-identity models={} census_digest={}",
        census.len(),
        combined.finalize().to_hex()
    );
}
