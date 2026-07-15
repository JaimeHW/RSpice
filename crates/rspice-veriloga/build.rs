use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const DIGEST_INPUTS: &[&str] = &["build.rs", "Cargo.toml", "src"];

fn collect_files(root: &Path, path: &Path, files: &mut Vec<(String, PathBuf)>) {
    if path.is_file() {
        let logical = path
            .strip_prefix(root)
            .expect("generator source remains under its crate root")
            .to_string_lossy()
            .replace('\\', "/");
        files.push((logical, path.to_path_buf()));
        return;
    }

    let mut entries = fs::read_dir(path)
        .unwrap_or_else(|error| {
            panic!("cannot read generator source '{}': {error}", path.display())
        })
        .map(|entry| entry.expect("cannot read generator source entry").path())
        .collect::<Vec<_>>();
    entries.sort();
    for entry in entries {
        collect_files(root, &entry, files);
    }
}

fn normalized_text_bytes(path: &Path) -> Vec<u8> {
    let bytes = fs::read(path).unwrap_or_else(|error| {
        panic!("cannot read generator source '{}': {error}", path.display())
    });
    let mut normalized = Vec::with_capacity(bytes.len());
    let mut index = 0usize;
    while index < bytes.len() {
        if bytes[index] == b'\r' {
            if bytes.get(index + 1) == Some(&b'\n') {
                index += 1;
            }
            normalized.push(b'\n');
        } else {
            normalized.push(bytes[index]);
        }
        index += 1;
    }
    normalized
}

fn main() {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR"));
    let mut files = Vec::new();
    for input in DIGEST_INPUTS {
        let path = root.join(input);
        println!("cargo:rerun-if-changed={}", path.display());
        collect_files(&root, &path, &mut files);
    }
    let workspace_root = root
        .parent()
        .and_then(Path::parent)
        .expect("rspice-veriloga is nested beneath the workspace root");
    for name in ["Cargo.toml", "Cargo.lock"] {
        let path = workspace_root.join(name);
        println!("cargo:rerun-if-changed={}", path.display());
        files.push((format!("workspace/{name}"), path));
    }
    files.sort_by(|left, right| left.0.cmp(&right.0));

    let mut hasher = blake3::Hasher::new();
    hasher.update(b"rspice-veriloga-generator-source-v2\0");
    for (logical, path) in files {
        let bytes = normalized_text_bytes(&path);
        hasher.update(&(logical.len() as u64).to_le_bytes());
        hasher.update(logical.as_bytes());
        hasher.update(&(bytes.len() as u64).to_le_bytes());
        hasher.update(&bytes);
    }

    println!(
        "cargo:rustc-env=RSPICE_VERILOGA_GENERATOR_SOURCE_DIGEST={}",
        hasher.finalize().to_hex()
    );
}
