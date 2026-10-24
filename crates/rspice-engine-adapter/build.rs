//! Link settings for the engine executor.

fn main() {
    // Harmonic-balance and envelope execution can exceed MSVC's default 1 MiB
    // main-thread stack in a debug build, where Linux gives the same thread
    // 8 MiB. Reserve 8 MiB for the executable, as `rspice` does; Windows
    // commits pages only on demand. Test harness threads keep their own policy.
    if std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc") {
        println!("cargo:rustc-link-arg-bin=rspice-engine-adapter=/STACK:8388608");
    }
    println!("cargo:rerun-if-changed=build.rs");
}
