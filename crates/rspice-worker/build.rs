use std::env;

// Match the application image's stripped web-release relocation policy.
fn main() {
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
