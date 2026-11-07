use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Component, Path};

mod build_support;
use build_support::verify_declared_generated_file;

const MANIFEST_SCHEMA_VERSION: u32 = 7;
const MANIFEST_FILE_NAME: &str = "manifest.txt";
const REGENERATE_COMMAND: &str = "cargo run -p rspice-veriloga --profile generator --bin rspice-veriloga-gen -- regenerate-builtins";

#[derive(Deserialize)]
struct GeneratedManifest {
    schema_version: u32,
    bundle_digest: String,
    file_count: usize,
    source_bytes: u64,
    files: Vec<GeneratedFile>,
}

#[derive(Deserialize)]
struct GeneratedFile {
    relative_path: String,
    bytes: u64,
    blake3: String,
}

fn main() {
    if env::var_os("CARGO_FEATURE_VERILOGA").is_some() {
        emit_veriloga_compiler_identity();
    }
    println!("cargo:rerun-if-env-changed=CARGO_FEATURE_VERILOGA_BUILTINS_BASE");
    if env::var_os("CARGO_FEATURE_VERILOGA_BUILTINS_BASE").is_none() {
        return;
    }

    let generated_root = Path::new("..").join("rspice-veriloga-models");
    verify_generated_bundle(&generated_root).unwrap_or_else(|error| {
        panic!(
            "generated Verilog-A built-in verification failed: {error}. Run `{REGENERATE_COMMAND}`"
        )
    });
}

fn verify_generated_bundle(generated_root: &Path) -> Result<(), String> {
    let manifest_path = generated_root.join(MANIFEST_FILE_NAME);
    println!("cargo:rerun-if-changed={}", manifest_path.display());
    let text = fs::read_to_string(&manifest_path)
        .map_err(|error| format!("read '{}': {error}", manifest_path.display()))?;
    let json_start = text
        .find('{')
        .ok_or_else(|| format!("'{}' is not a v3 JSON manifest", manifest_path.display()))?;
    let manifest: GeneratedManifest = serde_json::from_str(&text[json_start..])
        .map_err(|error| format!("parse '{}': {error}", manifest_path.display()))?;
    if manifest.schema_version != MANIFEST_SCHEMA_VERSION {
        return Err(format!(
            "manifest schema {} is unsupported; expected {}",
            manifest.schema_version, MANIFEST_SCHEMA_VERSION
        ));
    }
    if manifest.file_count != manifest.files.len() {
        return Err(format!(
            "manifest declares {} files but contains {} file records",
            manifest.file_count,
            manifest.files.len()
        ));
    }

    let mut expected_paths = BTreeSet::new();
    let mut bundle_hasher = blake3::Hasher::new();
    let mut source_bytes = 0u64;
    for file in &manifest.files {
        let relative = Path::new(&file.relative_path);
        if relative.is_absolute()
            || relative
                .components()
                .any(|component| !matches!(component, Component::Normal(_)))
        {
            return Err(format!(
                "manifest contains unsafe generated path '{}'",
                file.relative_path
            ));
        }
        if !expected_paths.insert(file.relative_path.clone()) {
            return Err(format!(
                "manifest contains duplicate generated path '{}'",
                file.relative_path
            ));
        }

        let path = generated_root.join(relative);
        println!("cargo:rerun-if-changed={}", path.display());
        let checkout_bytes =
            fs::read(&path).map_err(|error| format!("read '{}': {error}", path.display()))?;
        let bytes = verify_declared_generated_file(
            &file.relative_path,
            &checkout_bytes,
            file.bytes,
            &file.blake3,
        )?;
        source_bytes = source_bytes
            .checked_add(file.bytes)
            .ok_or_else(|| "generated source byte count overflowed u64".to_string())?;
        update_digest_record(&mut bundle_hasher, file.relative_path.as_bytes());
        update_digest_record(&mut bundle_hasher, bytes.as_ref());
    }

    if source_bytes != manifest.source_bytes {
        return Err(format!(
            "generated files contain {source_bytes} bytes; manifest declares {}",
            manifest.source_bytes
        ));
    }
    let bundle_digest = bundle_hasher.finalize().to_hex().to_string();
    if bundle_digest != manifest.bundle_digest {
        return Err(format!(
            "bundle digest is {bundle_digest}; manifest declares {}",
            manifest.bundle_digest
        ));
    }

    let actual_paths = collect_generated_files(generated_root)?;
    if actual_paths != expected_paths {
        let missing = expected_paths
            .difference(&actual_paths)
            .cloned()
            .collect::<Vec<_>>();
        let untracked = actual_paths
            .difference(&expected_paths)
            .cloned()
            .collect::<Vec<_>>();
        return Err(format!(
            "generated file census differs from manifest; missing={missing:?}, untracked={untracked:?}"
        ));
    }
    Ok(())
}

fn collect_generated_files(generated_root: &Path) -> Result<BTreeSet<String>, String> {
    let mut pending = vec![generated_root.to_path_buf()];
    let mut files = BTreeSet::new();
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)
            .map_err(|error| format!("read '{}': {error}", directory.display()))?
        {
            let entry = entry
                .map_err(|error| format!("read entry below '{}': {error}", directory.display()))?;
            let path = entry.path();
            let file_type = entry
                .file_type()
                .map_err(|error| format!("inspect '{}': {error}", path.display()))?;
            if file_type.is_dir() {
                pending.push(path);
            } else if file_type.is_file()
                && path.file_name().and_then(|name| name.to_str()) != Some(MANIFEST_FILE_NAME)
            {
                let relative = path
                    .strip_prefix(generated_root)
                    .map_err(|error| format!("resolve '{}': {error}", path.display()))?
                    .to_string_lossy()
                    .replace('\\', "/");
                files.insert(relative);
            }
        }
    }
    Ok(files)
}

fn update_digest_record(hasher: &mut blake3::Hasher, bytes: &[u8]) {
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
}

/// The sources whose content decides what the Verilog-A compiler emits.
const VERILOGA_COMPILER_SOURCES: &[&str] = &[
    "../rspice-veriloga/Cargo.toml",
    "../rspice-veriloga/build.rs",
    "../rspice-veriloga/src",
    "../rspice-veriloga-runtime/Cargo.toml",
    "../rspice-veriloga-runtime/src",
    "../../Cargo.lock",
];

/// Name the Verilog-A compiler this build links, for the compile cache.
///
/// A cached record or native image is the output of one compiler build and is
/// read back only by that build: the identity covers every source file of the
/// compiler and its runtime, the lock file that fixes their dependencies, the
/// crate version and the target. Any edit to any of them is a different
/// compiler, whose cache entries live under other names.
fn emit_veriloga_compiler_identity() {
    let mut hasher = blake3::Hasher::new();
    for source in VERILOGA_COMPILER_SOURCES {
        println!("cargo:rerun-if-changed={source}");
        hash_source_tree(&mut hasher, Path::new(source), source).unwrap_or_else(|error| {
            panic!("hashing the Verilog-A compiler sources failed: {error}")
        });
    }
    for key in ["CARGO_PKG_VERSION", "TARGET", "CARGO_CFG_TARGET_FEATURE"] {
        update_digest_record(&mut hasher, key.as_bytes());
        update_digest_record(&mut hasher, env::var(key).unwrap_or_default().as_bytes());
    }
    println!(
        "cargo:rustc-env=RSPICE_VERILOGA_COMPILER_IDENTITY={}",
        hasher.finalize().to_hex()
    );
}

/// Hash a file, or every file under a directory in path order, each under
/// its path relative to the workspace layout so a rename is a change too.
fn hash_source_tree(hasher: &mut blake3::Hasher, path: &Path, label: &str) -> Result<(), String> {
    let metadata =
        fs::metadata(path).map_err(|error| format!("inspect '{}': {error}", path.display()))?;
    if metadata.is_file() {
        let bytes =
            fs::read(path).map_err(|error| format!("read '{}': {error}", path.display()))?;
        update_digest_record(hasher, label.as_bytes());
        update_digest_record(hasher, &bytes);
        return Ok(());
    }
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("list '{}': {error}", path.display()))?
        .map(|entry| entry.map(|entry| entry.file_name()))
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("list '{}': {error}", path.display()))?;
    entries.sort();
    for name in entries {
        let name = name.to_string_lossy();
        hash_source_tree(
            hasher,
            &path.join(name.as_ref()),
            &format!("{label}/{name}"),
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unsafe_manifest_paths_are_rejected() {
        for path in ["../stamp.rs", "/stamp.rs", "device/../stamp.rs"] {
            let relative = Path::new(path);
            assert!(
                relative.is_absolute()
                    || relative
                        .components()
                        .any(|component| !matches!(component, Component::Normal(_))),
                "{path}"
            );
        }
    }
}
