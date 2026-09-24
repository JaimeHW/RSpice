use std::env;
use std::fs;
use std::path::PathBuf;

mod build_identity;

/// The desktop entry point runs the whole application — eframe's event loop,
/// the app constructor, every frame — on the process main thread. Linux and
/// macOS give that thread 8 MiB; the MSVC linker gives it 1 MiB. `AppState`
/// is a few hundred kilobytes of inline state, and startup holds several
/// copies of it on the stack at once (the `Box::new` temporary of the app,
/// `RSpiceApp::new`'s local, the RON deserializer's partial struct, and the
/// defaults it falls back to), so the optimized frame chain needs about
/// 1.2 MiB and the release executable died at startup with
/// `STATUS_STACK_OVERFLOW`; the debug executable, whose frames are far
/// larger, never started unpatched. The reserve is address space, not memory
/// (pages commit on touch), so it is set well clear of both profiles rather
/// than to the day's measured need. Only the bin targets get it: tests run on
/// threads Rust already sizes itself.
fn reserve_main_thread_stack() {
    if env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        const MAIN_THREAD_STACK_RESERVE_BYTES: u64 = 64 << 20;
        println!("cargo:rustc-link-arg-bins=/STACK:{MAIN_THREAD_STACK_RESERVE_BYTES}");
    }
}

/// Compact relocation addends in the two production browser executables.
///
/// `wasm-ld --compress-relocations` uses fixed-width padded LEB encodings
/// while linking and then rewrites them to their canonical variable width.
/// This changes neither code nor data semantics. It must stay out of profiles
/// that carry DWARF, however, because shrinking relocation fields invalidates
/// debug-section offsets. Keep the contract exact: only the stripped
/// `web-release` profile for the browser target receives the linker option;
/// dev/debug, ordinary release, WASI, native desktop, and mobile builds remain
/// untouched. Cargo reports a custom profile that inherits `release` as
/// `PROFILE=release`, so the complete effective profile tuple is the stable
/// discriminator rather than an undocumented inference from `OUT_DIR`.
fn compress_web_release_wasm_relocations() {
    println!("cargo:rerun-if-env-changed=TARGET");
    println!("cargo:rerun-if-env-changed=PROFILE");
    println!("cargo:rerun-if-env-changed=OPT_LEVEL");
    println!("cargo:rerun-if-env-changed=DEBUG");
    println!("cargo:rerun-if-env-changed=CARGO_CFG_PANIC");
    if env::var("TARGET").as_deref() == Ok("wasm32-unknown-unknown")
        && env::var("PROFILE").as_deref() == Ok("release")
        && env::var("OPT_LEVEL").as_deref() == Ok("z")
        && env::var("DEBUG").as_deref() == Ok("false")
        && env::var("CARGO_CFG_PANIC").as_deref() == Ok("abort")
    {
        println!("cargo:rustc-link-arg-bins=--compress-relocations");
    }
}

fn main() {
    println!(
        "cargo:rustc-env=RSPICE_BUILD_TARGET={}",
        env::var("TARGET").expect("Cargo must provide TARGET")
    );
    println!(
        "cargo:rustc-env=RSPICE_BUILD_ARCH={}",
        env::var("CARGO_CFG_TARGET_ARCH").expect("Cargo must provide CARGO_CFG_TARGET_ARCH")
    );
    println!("cargo:rerun-if-env-changed=RSPICE_AUTOMATION_RUNTIME_KEY_ID");
    println!("cargo:rerun-if-env-changed=RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX");
    let runtime_key_id = env::var("RSPICE_AUTOMATION_RUNTIME_KEY_ID").unwrap_or_default();
    let runtime_public_key =
        env::var("RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX").unwrap_or_default();
    if runtime_key_id.is_empty() != runtime_public_key.is_empty() {
        panic!(
            "RSPICE_AUTOMATION_RUNTIME_KEY_ID and RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX must be supplied together"
        );
    }
    if !runtime_public_key.is_empty()
        && (runtime_public_key.len() != 64
            || !runtime_public_key
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit()))
    {
        panic!(
            "RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX must contain exactly 64 hexadecimal digits"
        );
    }
    println!("cargo:rustc-env=RSPICE_AUTOMATION_RUNTIME_KEY_ID={runtime_key_id}");
    println!("cargo:rustc-env=RSPICE_AUTOMATION_RUNTIME_PUBLIC_KEY_HEX={runtime_public_key}");

    // Development license material is an explicit build-profile capability,
    // not a side effect of `debug_assertions` (which can be enabled for a
    // release profile). Standard debug builds retain the signed sample-key
    // workflow; every release and custom profile fails closed unless backed by
    // a production ceremony key compiled into the application.
    println!("cargo:rustc-check-cfg=cfg(rspice_development_build)");
    if env::var("PROFILE").as_deref() == Ok("debug") {
        println!("cargo:rustc-cfg=rspice_development_build");
    }

    let manifest_dir =
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR must be set"));

    build_identity::emit(&manifest_dir);

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

    reserve_main_thread_stack();
    compress_web_release_wasm_relocations();

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
                .join("assets")
                .join("brand")
                .join("rspice.ico");
            println!("cargo:rerun-if-changed={}", icon.display());
            match icon.to_str() {
                Some(path) => {
                    let mut res = winresource::WindowsResource::new();
                    res.set_icon(path);
                    if let Err(e) = res.compile() {
                        println!("cargo:warning=rspice-ui: could not embed Windows .exe icon: {e}");
                    }
                }
                None => {
                    println!("cargo:warning=rspice-ui: icon path is not valid UTF-8, skipping")
                }
            }
        }
    }
}
