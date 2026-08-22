use std::env;
use std::path::PathBuf;

fn main() {
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let native_enabled = env::var_os("CARGO_FEATURE_NATIVE").is_some();
    if native_enabled && target_arch == "aarch64" && target_family == "unix" {
        let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
        let source = root.join("src/native/aarch64_runtime.c");
        println!("cargo:rerun-if-changed={}", source.display());
        cc::Build::new()
            .file(&source)
            .warnings(true)
            .extra_warnings(true)
            .compile("rspice_veriloga_aarch64_runtime");
    }
}
