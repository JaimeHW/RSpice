use std::path::{Path, PathBuf};
use std::process::Command;

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
    println!(
        "cargo:rustc-env=RSPICE_BUILD_COMMIT={}",
        resolved_build_commit()
    );
}

fn resolved_build_commit() -> String {
    println!("cargo:rerun-if-env-changed=RSPICE_BUILD_COMMIT");
    if let Ok(configured) = std::env::var("RSPICE_BUILD_COMMIT") {
        return normalize_commit(&configured).unwrap_or_else(|| {
            panic!("RSPICE_BUILD_COMMIT must be exactly 40 hexadecimal characters")
        });
    }

    register_git_revision_inputs();
    git_output(&["rev-parse", "HEAD"])
        .and_then(|value| normalize_commit(&value))
        .unwrap_or_else(|| "unknown".to_string())
}

fn normalize_commit(value: &str) -> Option<String> {
    let value = value.trim();
    (value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .then(|| value.to_ascii_lowercase())
}

fn register_git_revision_inputs() {
    for arguments in [
        &["rev-parse", "--git-path", "HEAD"][..],
        &["rev-parse", "--git-path", "refs/heads/main"][..],
    ] {
        if let Some(path) = git_output(arguments) {
            emit_rerun_path(&path);
        }
    }
    if let Some(reference) = git_output(&["symbolic-ref", "-q", "HEAD"])
        && let Some(path) = git_output(&["rev-parse", "--git-path", &reference])
    {
        emit_rerun_path(&path);
    }
}

fn emit_rerun_path(path: &str) {
    let path = PathBuf::from(path);
    let path = if path.is_absolute() {
        path
    } else {
        Path::new(&std::env::var("CARGO_MANIFEST_DIR").unwrap_or_default()).join(path)
    };
    println!("cargo:rerun-if-changed={}", path.display());
}

fn git_output(arguments: &[&str]) -> Option<String> {
    let output = Command::new("git")
        .args(arguments)
        .current_dir(std::env::var("CARGO_MANIFEST_DIR").ok()?)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).trim().to_string())
}
