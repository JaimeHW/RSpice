//! Embeds the built viewer runtime into the renderer binary.
//!
//! The component build workflow compiles `rspice-viewer` to wasm, runs
//! `wasm-bindgen`, and points `RSPICE_VIEWER_RUNTIME_DIR` at the output
//! (`viewer_bg.wasm` + `viewer.js`) before building this crate. The attested
//! `/rspice-publish` file therefore covers the exact runtime it seals into
//! bundles. Development builds leave the variable unset and compile without
//! a runtime; the binary refuses to render in that state, and the library
//! tests inject their own runtime bytes.

use std::{env, fs, path::Path};

fn main() {
    println!("cargo:rerun-if-env-changed=RSPICE_VIEWER_RUNTIME_DIR");
    let out_dir = env::var("OUT_DIR").expect("cargo always sets OUT_DIR");
    let target = Path::new(&out_dir).join("viewer_embed.rs");
    let source = match env::var("RSPICE_VIEWER_RUNTIME_DIR") {
        Ok(dir) if !dir.is_empty() => {
            let wasm = Path::new(&dir).join("viewer_bg.wasm");
            let js = Path::new(&dir).join("viewer.js");
            for path in [&wasm, &js] {
                assert!(
                    path.is_file(),
                    "RSPICE_VIEWER_RUNTIME_DIR must contain {}",
                    path.display()
                );
                println!("cargo:rerun-if-changed={}", path.display());
            }
            format!(
                "pub const EMBEDDED: bool = true;\n\
                 pub static VIEWER_WASM: &[u8] = include_bytes!({wasm:?});\n\
                 pub static VIEWER_JS: &[u8] = include_bytes!({js:?});\n"
            )
        }
        _ => "pub const EMBEDDED: bool = false;\n\
              pub static VIEWER_WASM: &[u8] = &[];\n\
              pub static VIEWER_JS: &[u8] = &[];\n"
            .to_string(),
    };
    fs::write(&target, source).expect("write viewer_embed.rs");
}
