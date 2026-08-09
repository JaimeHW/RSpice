//! Versioned benchmark-suite manifests.
//!
//! A benchmark gate must name the exact workloads it measured. Directory
//! discovery is intentionally not supported: adding an unrelated `.cir` file
//! must not silently redefine a published suite or invalidate every baseline.

use crate::error::BenchError;
use clap::Args;
use serde::Deserialize;
use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use std::time::{SystemTime, UNIX_EPOCH};

const SUITE_SCHEMA_VERSION: u32 = 1;
const MAX_MANIFEST_BYTES: u64 = 1024 * 1024;
const MAX_DECK_BYTES: u64 = 64 * 1024 * 1024;

#[derive(Args, Debug)]
pub struct SuiteArgs {
    /// Suite manifest to validate. Defaults to the checked-in macro-v1 suite.
    #[arg(long, value_name = "PATH")]
    pub suite: Option<PathBuf>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SuiteManifest {
    schema_version: u32,
    suite_id: String,
    suite_version: u32,
    description: String,
    methodology_version: u32,
    decks: Vec<DeckManifest>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DeckManifest {
    name: String,
    path: PathBuf,
    blake3: String,
    category: String,
    analyses: Vec<String>,
    correctness: CorrectnessContract,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CorrectnessContract {
    /// Stable identifier of the independently maintained conformance contract.
    pub contract_id: String,
    /// Minimum number of analysis runs the RSpice summary must report.
    pub minimum_runs: usize,
    /// Minimum number of result artifacts the preflight must produce.
    pub minimum_outputs: usize,
    /// Minimum number of evaluated `.MEAS` assertions, when the deck has them.
    #[serde(default)]
    pub minimum_measurements: usize,
}

#[derive(Debug, Clone)]
pub struct LoadedSuite {
    pub path: PathBuf,
    pub id: String,
    pub version: u32,
    pub description: String,
    pub methodology_version: u32,
    pub manifest_blake3: String,
    pub decks: Vec<LoadedDeck>,
}

#[derive(Debug, Clone)]
pub struct LoadedDeck {
    pub name: String,
    pub path: PathBuf,
    pub relative_path: PathBuf,
    pub blake3: String,
    pub category: String,
    pub analyses: Vec<String>,
    pub correctness: CorrectnessContract,
}

pub fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rspice-bench is a workspace crate under crates/")
        .to_path_buf()
}

pub fn default_suite_path() -> PathBuf {
    workspace_root().join("benchmarks/suites/macro-v1/suite.toml")
}

pub fn default_results_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();
    workspace_root().join(format!(
        "benchmarks/results/run-{timestamp}-{}.json",
        std::process::id()
    ))
}

pub fn resolve_suite_path(requested: Option<&Path>) -> PathBuf {
    requested.map_or_else(default_suite_path, Path::to_path_buf)
}

pub fn load(path: &Path) -> Result<LoadedSuite, BenchError> {
    let metadata = fs::symlink_metadata(path).map_err(|error| {
        BenchError::io(
            format!("failed to inspect benchmark suite `{}`", path.display()),
            error,
        )
    })?;
    if !metadata.file_type().is_file() {
        return Err(BenchError::Suite {
            message: format!(
                "suite manifest `{}` must be a regular, non-symlink file",
                path.display()
            ),
        });
    }
    if metadata.len() > MAX_MANIFEST_BYTES {
        return Err(BenchError::Suite {
            message: format!(
                "suite manifest `{}` exceeds the {} KiB input limit",
                path.display(),
                MAX_MANIFEST_BYTES / 1024
            ),
        });
    }
    let bytes = fs::read(path).map_err(|error| {
        BenchError::io(
            format!("failed to read benchmark suite `{}`", path.display()),
            error,
        )
    })?;
    let text = std::str::from_utf8(&bytes).map_err(|error| BenchError::Suite {
        message: format!("suite manifest `{}` is not UTF-8: {error}", path.display()),
    })?;
    let manifest: SuiteManifest = toml::from_str(text).map_err(|error| BenchError::Suite {
        message: format!(
            "failed to parse suite manifest `{}`: {error}",
            path.display()
        ),
    })?;
    validate_manifest(path, &manifest)?;

    let manifest_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let mut decks = Vec::with_capacity(manifest.decks.len());
    let mut hash_failures = Vec::new();
    for deck in manifest.decks {
        let resolved = manifest_dir.join(&deck.path);
        let metadata = fs::symlink_metadata(&resolved).map_err(|error| {
            BenchError::io(
                format!(
                    "failed to inspect suite deck `{}` declared by `{}`",
                    resolved.display(),
                    path.display()
                ),
                error,
            )
        })?;
        if !metadata.file_type().is_file() {
            return Err(BenchError::Suite {
                message: format!(
                    "suite deck `{}` must be a regular, non-symlink file",
                    resolved.display()
                ),
            });
        }
        if metadata.len() == 0 || metadata.len() > MAX_DECK_BYTES {
            return Err(BenchError::Suite {
                message: format!(
                    "suite deck `{}` must contain 1..={} bytes",
                    resolved.display(),
                    MAX_DECK_BYTES
                ),
            });
        }
        let bytes = fs::read(&resolved).map_err(|error| {
            BenchError::io(
                format!(
                    "failed to read suite deck `{}` declared by `{}`",
                    resolved.display(),
                    path.display()
                ),
                error,
            )
        })?;
        let actual = blake3::hash(&bytes).to_hex().to_string();
        if actual != deck.blake3.to_ascii_lowercase() {
            hash_failures.push(format!(
                "{}: declared {}, actual {}",
                deck.path.display(),
                deck.blake3,
                actual
            ));
        }
        decks.push(LoadedDeck {
            name: deck.name,
            path: resolved,
            relative_path: deck.path,
            blake3: actual,
            category: deck.category,
            analyses: deck.analyses,
            correctness: deck.correctness,
        });
    }
    if !hash_failures.is_empty() {
        return Err(BenchError::Suite {
            message: format!(
                "suite `{}` contains deck checksum mismatches:\n{}",
                path.display(),
                hash_failures.join("\n")
            ),
        });
    }

    Ok(LoadedSuite {
        path: path.to_path_buf(),
        id: manifest.suite_id,
        version: manifest.suite_version,
        description: manifest.description,
        methodology_version: manifest.methodology_version,
        manifest_blake3: blake3::hash(&bytes).to_hex().to_string(),
        decks,
    })
}

fn validate_manifest(path: &Path, manifest: &SuiteManifest) -> Result<(), BenchError> {
    let reject = |message: String| {
        Err(BenchError::Suite {
            message: format!("invalid suite manifest `{}`: {message}", path.display()),
        })
    };
    if manifest.schema_version != SUITE_SCHEMA_VERSION {
        return reject(format!(
            "schema_version is {}, expected {SUITE_SCHEMA_VERSION}",
            manifest.schema_version
        ));
    }
    if !is_identifier(&manifest.suite_id)
        || manifest.description.trim().is_empty()
        || manifest.description.trim() != manifest.description
    {
        return reject(
            "suite_id must be a portable identifier and description must be trimmed".to_string(),
        );
    }
    if manifest.suite_version == 0 || manifest.methodology_version == 0 {
        return reject("suite_version and methodology_version must be positive".to_string());
    }
    if manifest.decks.is_empty() {
        return reject("at least one deck is required".to_string());
    }

    let mut names = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut contract_ids = BTreeSet::new();
    for deck in &manifest.decks {
        if !is_identifier(&deck.name) || !is_identifier(&deck.category) {
            return reject(format!(
                "deck `{}` name and category must be portable identifiers",
                deck.name
            ));
        }
        if !names.insert(deck.name.to_ascii_lowercase()) {
            return reject(format!("duplicate deck name `{}`", deck.name));
        }
        if !is_safe_relative_path(&deck.path) {
            return reject(format!(
                "deck path `{}` must be a normalized relative path without `..`",
                deck.path.display()
            ));
        }
        if deck
            .path
            .extension()
            .and_then(|extension| extension.to_str())
            .map(str::to_ascii_lowercase)
            != Some("cir".to_string())
        {
            return reject(format!("deck `{}` must name a .cir file", deck.name));
        }
        let normalized_path = deck
            .path
            .to_string_lossy()
            .replace('\\', "/")
            .to_ascii_lowercase();
        if !paths.insert(normalized_path) {
            return reject(format!("duplicate deck path `{}`", deck.path.display()));
        }
        let analyses = deck
            .analyses
            .iter()
            .map(|item| item.trim().to_ascii_lowercase())
            .collect::<BTreeSet<_>>();
        if analyses.len() != deck.analyses.len()
            || deck
                .analyses
                .iter()
                .any(|analysis| !is_identifier(analysis))
        {
            return reject(format!("deck `{}` must declare its analyses", deck.name));
        }
        if deck.blake3.len() != 64 || !deck.blake3.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return reject(format!(
                "deck `{}` has a malformed BLAKE3 digest",
                deck.name
            ));
        }
        if !is_contract_id(&deck.correctness.contract_id)
            || deck.correctness.minimum_runs == 0
            || deck.correctness.minimum_outputs == 0
        {
            return reject(format!(
                "deck `{}` requires a non-empty correctness contract and positive run/output minima",
                deck.name
            ));
        }
        if !contract_ids.insert(deck.correctness.contract_id.to_ascii_lowercase()) {
            return reject(format!(
                "duplicate correctness contract `{}`",
                deck.correctness.contract_id
            ));
        }
    }
    Ok(())
}

fn is_safe_relative_path(path: &Path) -> bool {
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

fn is_identifier(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn is_contract_id(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.ends_with('/')
        && !value.contains("//")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'/'))
}

pub fn run(args: &SuiteArgs) -> Result<ExitCode, BenchError> {
    let path = resolve_suite_path(args.suite.as_deref());
    let suite = load(&path)?;
    println!(
        "suite {} v{}: {} decks; manifest {}",
        suite.id,
        suite.version,
        suite.decks.len(),
        suite.manifest_blake3
    );
    for deck in &suite.decks {
        println!("  {}  {}", deck.blake3, deck.relative_path.display());
    }
    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_normal_relative_paths_are_accepted() {
        assert!(is_safe_relative_path(Path::new("decks/ok.cir")));
        assert!(!is_safe_relative_path(Path::new(
            "./decks/not-normalized.cir"
        )));
        assert!(!is_safe_relative_path(Path::new("../escape.cir")));
        assert!(!is_safe_relative_path(&workspace_root()));
    }

    #[test]
    fn checked_in_suite_is_valid_and_complete() {
        let suite = load(&default_suite_path()).expect("checked-in benchmark suite is valid");
        assert_eq!(suite.id, "macro-v1");
        assert_eq!(suite.decks.len(), 7);
    }

    #[test]
    fn checked_in_schema_documents_are_structural_contracts() {
        let schema_root = workspace_root().join("benchmarks/schemas");
        for name in [
            "suite-v1.schema.json",
            "macro-result-v2.schema.json",
            "artifact-envelope-v1.schema.json",
        ] {
            let path = schema_root.join(name);
            let bytes = fs::read(&path).expect("checked-in schema is readable");
            let value: serde_json::Value =
                serde_json::from_slice(&bytes).expect("checked-in schema is valid JSON");
            assert_eq!(
                value.get("$schema").and_then(serde_json::Value::as_str),
                Some("https://json-schema.org/draft/2020-12/schema"),
                "{name} declares the supported JSON Schema draft"
            );
            assert_eq!(
                value.get("additionalProperties"),
                Some(&serde_json::Value::Bool(false)),
                "{name} rejects unknown top-level fields"
            );
        }

        let macro_schema: serde_json::Value = serde_json::from_slice(
            &fs::read(schema_root.join("macro-result-v2.schema.json"))
                .expect("macro schema is readable"),
        )
        .expect("macro schema parses");
        for definition in [
            "tool",
            "host",
            "suite",
            "configuration",
            "simulators",
            "preflight",
            "deckResult",
            "regressionGate",
        ] {
            assert_eq!(
                macro_schema.pointer(&format!("/$defs/{definition}/additionalProperties")),
                Some(&serde_json::Value::Bool(false)),
                "macro schema definition {definition} is closed and explicit"
            );
        }
    }
}
