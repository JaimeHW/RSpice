fn main() {
    // Surface build metadata for `rspice --version`.
    println!(
        "cargo:rustc-env=RSPICE_BUILD_TARGET={}",
        std::env::var("TARGET").unwrap_or_default()
    );
    println!(
        "cargo:rustc-env=RSPICE_BUILD_PROFILE={}",
        std::env::var("PROFILE").unwrap_or_default()
    );
}
