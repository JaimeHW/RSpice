use std::env;
use std::path::{Path, PathBuf};

use rspice_veriloga::rust_backend::{
    GENERATED_BUILTIN_MANIFEST_FILE_NAME, REGENERATE_BUILTINS_COMMAND, validate_generated_builtins,
};

fn main() {
    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_BUILTINS_DIR");
    println!("cargo:rerun-if-env-changed=RSPICE_VERILOGA_REGENERATE_BUILTINS");
    println!("cargo:rustc-check-cfg=cfg(rspice_veriloga_builtins_generated)");

    if env::var_os("CARGO_FEATURE_VERILOGA_BUILTINS").is_none() {
        return;
    }

    if env::var_os("RSPICE_VERILOGA_REGENERATE_BUILTINS").is_some() {
        panic!(
            "RSPICE_VERILOGA_REGENERATE_BUILTINS is no longer supported during rspice-core builds. Run `{REGENERATE_BUILTINS_COMMAND}` explicitly, then rebuild."
        );
    }

    let generated_root = generated_source_root();
    let model_root = env::var_os("RSPICE_VERILOGA_BUILTINS_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(default_model_root);
    let generator_root = generator_crate_root();

    println!("cargo:rerun-if-changed={}", model_root.display());
    println!("cargo:rerun-if-changed={}", generator_root.display());
    println!(
        "cargo:rerun-if-changed={}",
        generated_root
            .join(GENERATED_BUILTIN_MANIFEST_FILE_NAME)
            .display()
    );
    println!(
        "cargo:rerun-if-changed={}",
        generated_root.join("registry.rs").display()
    );

    validate_generated_builtins(&model_root, &generated_root, &generator_root, true)
        .unwrap_or_else(|error| panic!("{error}"));
    println!("cargo:rustc-cfg=rspice_veriloga_builtins_generated");
}

fn generated_source_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    manifest_dir.join("src/device/veriloga_generated")
}

fn default_model_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    workspace_root(&manifest_dir).join("models/veriloga")
}

fn generator_crate_root() -> PathBuf {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    workspace_root(&manifest_dir).join("crates/rspice-veriloga")
}

fn workspace_root(manifest_dir: &Path) -> PathBuf {
    manifest_dir
        .parent()
        .and_then(Path::parent)
        .expect("rspice-core must live under workspace crates directory")
        .to_path_buf()
}
