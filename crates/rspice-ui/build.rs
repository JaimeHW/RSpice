use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    // Short commit hash for About / Copy diagnostics; "unknown" outside a
    // git checkout (release tarballs).
    let build_hash = std::process::Command::new("git")
        .args(["rev-parse", "--short=9", "HEAD"])
        .current_dir(&manifest_dir)
        .output()
        .ok()
        .filter(|out| out.status.success())
        .and_then(|out| String::from_utf8(out.stdout).ok())
        .map(|hash| hash.trim().to_owned())
        .filter(|hash| !hash.is_empty())
        .unwrap_or_else(|| "unknown".to_owned());
    println!("cargo:rustc-env=RSPICE_BUILD_HASH={build_hash}");
    let git_head = manifest_dir.join("..").join("..").join(".git").join("HEAD");
    if git_head.exists() {
        println!("cargo:rerun-if-changed={}", git_head.display());
    }

    let assets_dir = manifest_dir.join("assets").join("component_symbols");

    println!("cargo:rerun-if-changed={}", assets_dir.display());

    let mut asset_paths: Vec<_> = fs::read_dir(&assets_dir)
        .expect("component symbol asset directory must exist")
        .filter_map(|entry| {
            let path = entry.ok()?.path();
            (path.extension().and_then(|ext| ext.to_str()) == Some("svg")).then_some(path)
        })
        .collect();
    asset_paths.sort();

    let mut generated = String::from("pub static EMBEDDED_SYMBOLS: &[(&str, &str)] = &[\n");
    for path in asset_paths {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .expect("asset filename must be valid UTF-8");
        generated.push_str(&format!(
            "    ({:?}, include_str!(r#\"{}\"#)),\n",
            filename,
            path.display()
        ));
    }
    generated.push_str("];\n");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set"));
    let generated_path = out_dir.join("embedded_symbols.rs");
    fs::write(generated_path, generated).expect("embedded symbol table should be writable");

    // Embed the brand icon as the Windows .exe resource (what Explorer shows).
    // The `#[cfg(windows)]` gate is on the *host* (where winresource is a
    // build-dep); the TARGET_OS check ensures we only emit MSVC resource link
    // args when actually building a Windows binary — never when this same
    // Windows host cross-compiles to wasm. Non-fatal: a missing resource
    // compiler degrades to a warning so SDK-less environments still build.
    #[cfg(windows)]
    {
        let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
        if target_os == "windows" {
            let icon = manifest_dir
                .join("..")
                .join("..")
                .join("design")
                .join("brand")
                .join("export")
                .join("rspice.ico");
            println!("cargo:rerun-if-changed={}", icon.display());
            match icon.to_str() {
                Some(path) => {
                    let mut res = winresource::WindowsResource::new();
                    res.set_icon(path);
                    if let Err(e) = res.compile() {
                        println!(
                            "cargo:warning=rspice-ui: could not embed Windows .exe icon: {e}"
                        );
                    }
                }
                None => {
                    println!("cargo:warning=rspice-ui: icon path is not valid UTF-8, skipping")
                }
            }
        }
    }
}
