use serde::Deserialize;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Component, Path};

mod build_support;
use build_support::verify_declared_generated_file;

const MANIFEST_SCHEMA_VERSION: u32 = 5;
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
