//! Deterministically inventory and sign an already-staged managed CPython runtime.
//!
//! The private release seed is read from a bounded file, never from a command
//! line or environment variable. The staging directory must not already have
//! manifest outputs; release assembly therefore cannot accidentally bless a
//! partially replaced payload.

use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    io::Read as _,
    path::{Component, Path, PathBuf},
};

use ed25519_dalek::{Signer as _, SigningKey};
use rspice_automation_protocol::PROTOCOL_VERSION;
use rspice_automation_runtime::{
    MANIFEST_FILE, ManagedRuntimeManifest, RuntimeFile, RuntimeRequirement, RuntimeTrustStore,
    SIGNATURE_FILE, runtime_inventory_digest,
};
use semver::{Version, VersionReq};
use serde::Serialize;
use sha2::{Digest as _, Sha256};

const MAX_SIGNING_KEY_BYTES: u64 = 4_096;

#[derive(Debug)]
struct Options {
    runtime_root: PathBuf,
    runtime_build: String,
    target: String,
    architecture: String,
    python_version: String,
    python_abi: String,
    api_version: String,
    python_executable: String,
    worker_bootstrap: String,
    environment_digests: Vec<String>,
    key_id: String,
    signing_key_file: PathBuf,
}

#[derive(Serialize)]
struct SignatureDocument {
    key_id: String,
    signature_hex: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("managed-runtime packaging failed: {error}");
        std::process::exit(2);
    }
}

fn run() -> Result<(), String> {
    eprintln!("managed-runtime: parsing release inputs");
    let options = parse_options(env::args().skip(1))?;
    eprintln!("managed-runtime: validating release inputs");
    validate_options(&options)?;
    eprintln!("managed-runtime: opening staged payload");
    let root = canonical_safe_directory(&options.runtime_root)?;
    let manifest_path = root.join(MANIFEST_FILE);
    let signature_path = root.join(SIGNATURE_FILE);
    for output in [&manifest_path, &signature_path] {
        if output.exists() {
            return Err(format!(
                "{} already exists; package into a fresh staging directory",
                output.display()
            ));
        }
    }

    eprintln!("managed-runtime: loading release signing authority");
    let signing_key = read_signing_key(&options.signing_key_file)?;
    eprintln!("managed-runtime: inventorying staged payload");
    let files = inventory(&root, &options.python_executable)?;
    eprintln!("managed-runtime: inventoried {} files", files.len());
    let mut manifest = ManagedRuntimeManifest {
        schema: "rspice.managed-python-runtime/v2".to_owned(),
        runtime_build: options.runtime_build.clone(),
        target_triple: options.target.clone(),
        architecture: options.architecture.clone(),
        python_version: options.python_version.clone(),
        python_abi: options.python_abi.clone(),
        rspice_api_version: options.api_version.clone(),
        environment_digests_sha256: options.environment_digests.clone(),
        protocol_major: PROTOCOL_VERSION.major,
        protocol_minor: PROTOCOL_VERSION.minor,
        python_executable: portable_path(&options.python_executable)?,
        worker_bootstrap: portable_path(&options.worker_bootstrap)?,
        runtime_digest_sha256: String::new(),
        files,
    };
    ensure_member(&root, &manifest.python_executable, "Python executable")?;
    ensure_member(&root, &manifest.worker_bootstrap, "worker bootstrap")?;
    manifest.runtime_digest_sha256 = hex(&runtime_inventory_digest(&manifest)
        .map_err(|error| format!("could not calculate the runtime content identity: {error}"))?);
    eprintln!("managed-runtime: serializing signed manifest");
    let mut manifest_bytes = serde_json::to_vec_pretty(&manifest)
        .map_err(|error| format!("could not serialize the runtime manifest: {error}"))?;
    manifest_bytes.push(b'\n');
    let signature = signing_key.sign(&manifest_bytes);
    let signature_document = SignatureDocument {
        key_id: options.key_id.clone(),
        signature_hex: hex(&signature.to_bytes()),
    };
    let mut signature_bytes = serde_json::to_vec_pretty(&signature_document)
        .map_err(|error| format!("could not serialize the runtime signature: {error}"))?;
    signature_bytes.push(b'\n');

    write_new(&manifest_path, &manifest_bytes)?;
    if let Err(error) = write_new(&signature_path, &signature_bytes) {
        let _ = fs::remove_file(&manifest_path);
        return Err(error);
    }

    eprintln!("managed-runtime: self-verifying signed payload");
    let mut trust = RuntimeTrustStore::new();
    (|| -> Result<(), String> {
        let python = VersionReq::parse(&format!("={}", options.python_version))
            .map_err(|error| format!("could not construct the Python version pin: {error}"))?;
        let rspice_api = VersionReq::parse(&format!("={}", options.api_version))
            .map_err(|error| format!("could not construct the API version pin: {error}"))?;
        trust
            .add_key(options.key_id.clone(), signing_key.verifying_key())
            .map_err(|error| error.to_string())?;
        trust
            .verify(
                &root,
                &RuntimeRequirement {
                    target_triple: options.target.clone(),
                    architecture: options.architecture.clone(),
                    python,
                    rspice_api,
                },
            )
            .map_err(|error| error.to_string())?;
        Ok(())
    })()
    .map_err(|error| {
        let _ = fs::remove_file(&signature_path);
        let _ = fs::remove_file(&manifest_path);
        format!("self-verification rejected the staged runtime: {error}")
    })?;

    println!("runtime_root={}", root.display());
    println!("runtime_digest_sha256={}", manifest.runtime_digest_sha256);
    println!("signing_key_id={}", options.key_id);
    println!(
        "public_key_hex={}",
        hex(signing_key.verifying_key().as_bytes())
    );
    Ok(())
}

fn parse_options(arguments: impl Iterator<Item = String>) -> Result<Options, String> {
    let mut scalar = BTreeMap::<String, String>::new();
    let mut environment_digests = Vec::new();
    let mut arguments = arguments;
    while let Some(name) = arguments.next() {
        if name == "--help" || name == "-h" {
            return Err(usage().to_owned());
        }
        if !name.starts_with("--") {
            return Err(format!(
                "unexpected positional argument {name:?}\n{}",
                usage()
            ));
        }
        let value = arguments
            .next()
            .ok_or_else(|| format!("{name} requires a value"))?;
        if name == "--environment-digest" {
            environment_digests.push(value);
        } else if scalar.insert(name.clone(), value).is_some() {
            return Err(format!("{name} was supplied more than once"));
        }
    }
    let options = Options {
        runtime_root: PathBuf::from(take_required(&mut scalar, "--runtime-root")?),
        runtime_build: take_required(&mut scalar, "--runtime-build")?,
        target: take_required(&mut scalar, "--target")?,
        architecture: take_required(&mut scalar, "--architecture")?,
        python_version: take_required(&mut scalar, "--python-version")?,
        python_abi: scalar
            .remove("--python-abi")
            .unwrap_or_else(|| "cp314".to_owned()),
        api_version: scalar
            .remove("--api-version")
            .unwrap_or_else(|| "1.0.0".to_owned()),
        python_executable: take_required(&mut scalar, "--python-executable")?,
        worker_bootstrap: take_required(&mut scalar, "--worker-bootstrap")?,
        environment_digests,
        key_id: take_required(&mut scalar, "--key-id")?,
        signing_key_file: PathBuf::from(take_required(&mut scalar, "--signing-key-file")?),
    };
    if let Some(name) = scalar.keys().next() {
        return Err(format!("unknown option {name}"));
    }
    Ok(options)
}

fn take_required(values: &mut BTreeMap<String, String>, name: &str) -> Result<String, String> {
    values
        .remove(name)
        .ok_or_else(|| format!("missing required option {name}"))
}

fn usage() -> &'static str {
    "usage: rspice-managed-runtime-packager --runtime-root DIR --runtime-build ID \\
  --target TRIPLE --architecture ARCH --python-version 3.14.6 \\
  [--python-abi cp314] [--api-version 1.0.0] \\
  --python-executable PATH --worker-bootstrap PATH \\
  --environment-digest SHA256 [--environment-digest SHA256 ...] \\
  --key-id ID --signing-key-file FILE"
}

fn validate_options(options: &Options) -> Result<(), String> {
    let python = Version::parse(&options.python_version)
        .map_err(|error| format!("invalid Python version: {error}"))?;
    if python.major != 3 || python.minor != 14 || options.python_abi != "cp314" {
        return Err("the qualified native runtime must use the Python 3.14 / cp314 ABI".to_owned());
    }
    Version::parse(&options.api_version)
        .map_err(|error| format!("invalid RSpice API version: {error}"))?;
    for (label, value) in [
        ("runtime build", options.runtime_build.as_str()),
        ("target", options.target.as_str()),
        ("architecture", options.architecture.as_str()),
        ("key ID", options.key_id.as_str()),
    ] {
        if value.is_empty()
            || value.len() > 256
            || value.chars().any(|character| character.is_control())
        {
            return Err(format!(
                "{label} is empty, too long, or contains control text"
            ));
        }
    }
    if options.environment_digests.is_empty() {
        return Err("at least one signed environment digest is required".to_owned());
    }
    let mut unique = BTreeSet::new();
    for digest in &options.environment_digests {
        parse_hex::<32>(digest).map_err(|error| format!("invalid environment digest: {error}"))?;
        if !unique.insert(digest) {
            return Err(format!("duplicate environment digest {digest}"));
        }
    }
    Ok(())
}

fn canonical_safe_directory(path: &Path) -> Result<PathBuf, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(format!(
            "runtime root is not a safe directory: {}",
            path.display()
        ));
    }
    fs::canonicalize(path)
        .map_err(|error| format!("could not canonicalize {}: {error}", path.display()))
}

fn inventory(root: &Path, python_executable: &str) -> Result<Vec<RuntimeFile>, String> {
    let executable = portable_path(python_executable)?;
    let mut pending = vec![root.to_path_buf()];
    let mut files = Vec::new();
    while let Some(directory) = pending.pop() {
        let mut entries = fs::read_dir(&directory)
            .map_err(|error| format!("could not enumerate {}: {error}", directory.display()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| format!("could not enumerate {}: {error}", directory.display()))?;
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            let path = entry.path();
            let metadata = fs::symlink_metadata(&path)
                .map_err(|error| format!("could not inspect {}: {error}", path.display()))?;
            let relative = path
                .strip_prefix(root)
                .map_err(|_| format!("runtime member escaped its root: {}", path.display()))?;
            let logical = portable_relative_path(relative)?;
            if metadata.file_type().is_symlink() {
                return Err(format!("runtime member is a symbolic link: {logical}"));
            }
            if metadata.is_dir() {
                pending.push(path);
                continue;
            }
            if !metadata.is_file() {
                return Err(format!("runtime member is not a regular file: {logical}"));
            }
            if matches!(logical.as_str(), MANIFEST_FILE | SIGNATURE_FILE) {
                return Err(format!("runtime staging input already contains {logical}"));
            }
            let digest = hash_file(&path)?;
            files.push(RuntimeFile {
                path: logical.clone(),
                bytes: metadata.len(),
                sha256: hex(&digest),
                executable: logical == executable,
            });
        }
    }
    files.sort_by(|left, right| left.path.cmp(&right.path));
    if files.is_empty() {
        return Err("runtime staging directory is empty".to_owned());
    }
    Ok(files)
}

fn portable_path(value: &str) -> Result<String, String> {
    portable_relative_path(Path::new(value))
}

fn portable_relative_path(path: &Path) -> Result<String, String> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(format!(
            "runtime path must be non-empty and relative: {}",
            path.display()
        ));
    }
    let mut parts = Vec::new();
    for component in path.components() {
        match component {
            Component::Normal(part) => {
                let part = part
                    .to_str()
                    .ok_or_else(|| format!("runtime path is not UTF-8: {}", path.display()))?;
                if part.is_empty() || part.contains('/') || part.contains('\\') {
                    return Err(format!("runtime path is not portable: {}", path.display()));
                }
                parts.push(part);
            }
            _ => return Err(format!("runtime path is not portable: {}", path.display())),
        }
    }
    Ok(parts.join("/"))
}

fn ensure_member(root: &Path, logical: &str, label: &str) -> Result<(), String> {
    let path = root.join(logical);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|error| format!("{label} {logical:?} is unavailable: {error}"))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!("{label} {logical:?} is not a safe regular file"));
    }
    Ok(())
}

fn hash_file(path: &Path) -> Result<[u8; 32], String> {
    let mut source = fs::File::open(path)
        .map_err(|error| format!("could not open {}: {error}", path.display()))?;
    let mut digest = Sha256::new();
    // Keep the release packager safely below the 1 MiB default main-thread
    // stack used by Windows. This function is optimized into the release
    // assembly path, so a 1 MiB local buffer can overflow before `run` begins.
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = source
            .read(&mut buffer)
            .map_err(|error| format!("could not read {}: {error}", path.display()))?;
        if count == 0 {
            break;
        }
        digest.update(&buffer[..count]);
    }
    Ok(digest.finalize().into())
}

fn read_signing_key(path: &Path) -> Result<SigningKey, String> {
    let metadata = fs::symlink_metadata(path)
        .map_err(|error| format!("could not inspect signing key {}: {error}", path.display()))?;
    if !metadata.is_file()
        || metadata.file_type().is_symlink()
        || metadata.len() > MAX_SIGNING_KEY_BYTES
    {
        return Err("signing key must be a small, non-symlink regular file".to_owned());
    }
    let text = fs::read_to_string(path)
        .map_err(|error| format!("could not read signing key {}: {error}", path.display()))?;
    let seed = parse_hex::<32>(text.trim())?;
    Ok(SigningKey::from_bytes(&seed))
}

fn parse_hex<const N: usize>(value: &str) -> Result<[u8; N], String> {
    if value.len() != N * 2 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(format!("expected exactly {} hexadecimal digits", N * 2));
    }
    let mut result = [0_u8; N];
    for (index, pair) in value.as_bytes().chunks_exact(2).enumerate() {
        let pair = std::str::from_utf8(pair)
            .map_err(|_| "hexadecimal value is not valid UTF-8".to_owned())?;
        result[index] = u8::from_str_radix(pair, 16)
            .map_err(|_| "hexadecimal value is malformed".to_owned())?;
    }
    Ok(result)
}

fn hex(bytes: &[u8]) -> String {
    use std::fmt::Write as _;
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        let _ = write!(output, "{byte:02x}");
    }
    output
}

fn write_new(path: &Path, content: &[u8]) -> Result<(), String> {
    use std::io::Write as _;
    let mut options = fs::OpenOptions::new();
    options.write(true).create_new(true);
    let mut file = options
        .open(path)
        .map_err(|error| format!("could not create {}: {error}", path.display()))?;
    file.write_all(content)
        .and_then(|()| file.sync_all())
        .map_err(|error| format!("could not commit {}: {error}", path.display()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portable_paths_reject_parent_and_absolute_components() {
        assert_eq!(
            portable_path("worker/bootstrap.py").unwrap(),
            "worker/bootstrap.py"
        );
        assert!(portable_path("../python").is_err());
        assert!(portable_path("/python").is_err());
    }

    #[test]
    fn signing_seed_parser_is_exact() {
        assert_eq!(parse_hex::<32>(&"ab".repeat(32)).unwrap(), [0xab; 32]);
        assert!(parse_hex::<32>("ab").is_err());
    }
}
