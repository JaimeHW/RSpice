//! Canonical, device-local persistence for the complete shortcut profile library.
//!
//! This store is intentionally independent from project/session persistence:
//! shortcut profiles follow the user across projects, while project paths,
//! recents, macros, credentials, and other workspace authority must never enter
//! these bytes. Publication is compare-and-swap so a stale preferences dialog,
//! import, rollback, process, or browser tab cannot clobber a newer edit.

use std::fmt;

use serde_json::Value;

use crate::common::shortcut_artifacts::{canonical_json_bytes, hex_digest, sha256};
use crate::workbench::shortcuts::ShortcutProfileLibrary;

pub const SHORTCUT_LIBRARY_STORE_FORMAT: &str = "rspice.shortcut-profile-library";
pub const SHORTCUT_LIBRARY_STORE_SCHEMA_VERSION: u16 = 1;
pub const MAX_SHORTCUT_LIBRARY_STORE_BYTES: usize = 4 * 1024 * 1024;

#[cfg(not(target_arch = "wasm32"))]
const NATIVE_FILENAME: &str = "shortcut-profile-library.json";
#[cfg(target_arch = "wasm32")]
const BROWSER_STORAGE_KEY: &str = "rspice.shortcut-profile-library.canonical.v1";
#[cfg(target_arch = "wasm32")]
const BROWSER_LOCK_NAME: &str = "rspice-shortcut-profile-library-canonical-v1";
#[cfg(target_arch = "wasm32")]
const BROWSER_WRITE_TIMEOUT_MS: u32 = 15_000;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShortcutLibraryPersistenceError {
    code: &'static str,
    message: String,
}

impl ShortcutLibraryPersistenceError {
    fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }

    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// A browser write crossed `localStorage.setItem` but could not prove the
    /// exact stored bytes afterwards. The caller must not restore stale CAS
    /// authority or attempt another write until storage is reloaded.
    #[must_use]
    pub fn is_commit_in_doubt(&self) -> bool {
        self.code == "shortcut-library.browser-commit-in-doubt"
    }
}

impl fmt::Display for ShortcutLibraryPersistenceError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}: {}", self.code, self.message)
    }
}

impl std::error::Error for ShortcutLibraryPersistenceError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShortcutLibraryPersistenceToken {
    generation: u64,
    file_digest: [u8; 32],
}

impl ShortcutLibraryPersistenceToken {
    #[must_use]
    pub const fn generation(self) -> u64 {
        self.generation
    }

    #[must_use]
    pub const fn file_digest(self) -> [u8; 32] {
        self.file_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PersistedShortcutProfileLibrary {
    library: ShortcutProfileLibrary,
    token: ShortcutLibraryPersistenceToken,
}

impl PersistedShortcutProfileLibrary {
    #[must_use]
    pub const fn library(&self) -> &ShortcutProfileLibrary {
        &self.library
    }

    #[must_use]
    pub const fn token(&self) -> ShortcutLibraryPersistenceToken {
        self.token
    }

    /// Consume the durable result when the caller is ready to replace its live
    /// copy. This is the only library returned by a successful publication.
    #[must_use]
    pub fn into_library(self) -> ShortcutProfileLibrary {
        self.library
    }

    #[cfg(test)]
    pub(crate) fn test_snapshot(library: ShortcutProfileLibrary, generation: u64) -> Self {
        let file_digest = serde_json::to_vec(&library)
            .map(|bytes| sha256(&bytes))
            .unwrap_or([0; 32]);
        Self {
            library,
            token: ShortcutLibraryPersistenceToken {
                generation,
                file_digest,
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RetainedShortcutLibraryBytes {
    raw_bytes: Vec<u8>,
    digest: [u8; 32],
    generation: Option<u64>,
    reason: String,
}

impl RetainedShortcutLibraryBytes {
    #[must_use]
    pub fn raw_bytes(&self) -> &[u8] {
        &self.raw_bytes
    }

    #[must_use]
    pub const fn digest(&self) -> [u8; 32] {
        self.digest
    }

    #[must_use]
    pub const fn generation(&self) -> Option<u64> {
        self.generation
    }

    #[must_use]
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutProfileLibraryRestore {
    Missing,
    Compatible(Box<PersistedShortcutProfileLibrary>),
    /// Exact bytes from a future or incompatible format. They are retained for
    /// explicit repair/export and are never migrated or overwritten at startup.
    Incompatible(RetainedShortcutLibraryBytes),
    /// Exact bytes that claim the current format but fail validation.
    Corrupt(RetainedShortcutLibraryBytes),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortcutProfileLibraryStartup {
    Ready {
        persisted: Box<PersistedShortcutProfileLibrary>,
        initialized_from_session: bool,
    },
    Missing,
    Incompatible(RetainedShortcutLibraryBytes),
    Corrupt(RetainedShortcutLibraryBytes),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StoreExpectation {
    Missing,
    Digest([u8; 32]),
}

struct PreparedStoreWrite {
    expected: StoreExpectation,
    bytes: Vec<u8>,
    persisted: PersistedShortcutProfileLibrary,
}

#[cfg(any(test, not(target_arch = "wasm32")))]
trait ShortcutLibraryStore {
    fn read(&self) -> Result<Option<Vec<u8>>, ShortcutLibraryPersistenceError>;
    fn compare_exchange(
        &self,
        expected: StoreExpectation,
        bytes: &[u8],
    ) -> Result<(), ShortcutLibraryPersistenceError>;
}

fn retained(
    bytes: &[u8],
    generation: Option<u64>,
    reason: impl Into<String>,
) -> RetainedShortcutLibraryBytes {
    RetainedShortcutLibraryBytes {
        raw_bytes: bytes.to_vec(),
        digest: sha256(bytes),
        generation,
        reason: reason.into(),
    }
}

fn decode_store_bytes(bytes: &[u8]) -> ShortcutProfileLibraryRestore {
    if bytes.len() > MAX_SHORTCUT_LIBRARY_STORE_BYTES {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            None,
            format!(
                "shortcut library store exceeds the {} byte limit",
                MAX_SHORTCUT_LIBRARY_STORE_BYTES
            ),
        ));
    }
    let raw: Value = match serde_json::from_slice(bytes) {
        Ok(raw) => raw,
        Err(error) => {
            return ShortcutProfileLibraryRestore::Corrupt(retained(
                bytes,
                None,
                format!("shortcut library store is not valid JSON: {error}"),
            ));
        }
    };
    let Some(object) = raw.as_object() else {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            None,
            "shortcut library store root must be an object",
        ));
    };
    let generation = object.get("generation").and_then(Value::as_u64);
    let format = object.get("format").and_then(Value::as_str);
    let schema_version = object.get("schema-version").and_then(Value::as_u64);
    if format != Some(SHORTCUT_LIBRARY_STORE_FORMAT)
        || schema_version != Some(u64::from(SHORTCUT_LIBRARY_STORE_SCHEMA_VERSION))
    {
        return ShortcutProfileLibraryRestore::Incompatible(retained(
            bytes,
            generation,
            format!(
                "unsupported shortcut library store format {:?}, schema {:?}",
                format, schema_version
            ),
        ));
    }
    const EXPECTED_KEYS: [&str; 5] = [
        "format",
        "generation",
        "library",
        "payload-digest",
        "schema-version",
    ];
    if object.len() != EXPECTED_KEYS.len()
        || !EXPECTED_KEYS.iter().all(|key| object.contains_key(*key))
    {
        return ShortcutProfileLibraryRestore::Incompatible(retained(
            bytes,
            generation,
            "current envelope has unknown or missing fields",
        ));
    }
    let Some(generation) = generation.filter(|generation| *generation > 0) else {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            generation,
            "generation must be a positive integer",
        ));
    };
    let Some(library_value) = object.get("library").cloned() else {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            Some(generation),
            "library payload is missing",
        ));
    };
    if let Err(reason) = validate_device_local_privacy(&library_value) {
        return ShortcutProfileLibraryRestore::Corrupt(retained(bytes, Some(generation), reason));
    }
    let payload_bytes = match canonical_json_bytes(library_value.clone()) {
        Ok(bytes) => bytes,
        Err(error) => {
            return ShortcutProfileLibraryRestore::Corrupt(retained(
                bytes,
                Some(generation),
                format!("library payload cannot be canonicalized: {error}"),
            ));
        }
    };
    let expected_payload_digest = hex_digest(sha256(&payload_bytes));
    if object.get("payload-digest").and_then(Value::as_str)
        != Some(expected_payload_digest.as_str())
    {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            Some(generation),
            "library payload digest does not match the canonical payload",
        ));
    }
    let canonical_envelope = match canonical_json_bytes(raw.clone()) {
        Ok(bytes) => bytes,
        Err(error) => {
            return ShortcutProfileLibraryRestore::Corrupt(retained(
                bytes,
                Some(generation),
                format!("store envelope cannot be canonicalized: {error}"),
            ));
        }
    };
    if canonical_envelope != bytes {
        return ShortcutProfileLibraryRestore::Corrupt(retained(
            bytes,
            Some(generation),
            "store bytes are not in the canonical encoding",
        ));
    }
    let library: ShortcutProfileLibrary = match serde_json::from_value(library_value) {
        Ok(library) => library,
        Err(error) => {
            return ShortcutProfileLibraryRestore::Corrupt(retained(
                bytes,
                Some(generation),
                format!("library payload cannot be decoded: {error}"),
            ));
        }
    };
    if !library.is_compatible() {
        return ShortcutProfileLibraryRestore::Incompatible(retained(
            bytes,
            Some(generation),
            "library payload belongs to an incompatible shortcut profile version",
        ));
    }
    if let Err(reason) = validate_library(&library) {
        return ShortcutProfileLibraryRestore::Corrupt(retained(bytes, Some(generation), reason));
    }
    ShortcutProfileLibraryRestore::Compatible(Box::new(PersistedShortcutProfileLibrary {
        library,
        token: ShortcutLibraryPersistenceToken {
            generation,
            file_digest: sha256(bytes),
        },
    }))
}

fn validate_library(library: &ShortcutProfileLibrary) -> Result<(), String> {
    if !library.is_compatible() {
        return Err("shortcut profile library is incompatible".to_owned());
    }
    if library.revision() == u64::MAX {
        return Err("shortcut profile library revision is exhausted".to_owned());
    }
    if !library.active().audit().is_valid() {
        return Err("active shortcut profile does not pass its complete audit".to_owned());
    }
    if let Some(invalid) = library
        .named_presets()
        .find(|preset| !preset.profile().audit().is_valid())
    {
        return Err(format!(
            "shortcut preset '{}' does not pass its complete audit",
            invalid.name().as_str()
        ));
    }
    let value = serde_json::to_value(library)
        .map_err(|error| format!("shortcut profile library cannot be serialized: {error}"))?;
    validate_device_local_privacy(&value)
}

fn validate_device_local_privacy(value: &Value) -> Result<(), String> {
    fn walk(value: &Value, parent: Option<&str>, path: &mut Vec<String>) -> Result<(), String> {
        match value {
            Value::Object(object) => {
                for (key, child) in object {
                    let normalized = key
                        .chars()
                        .filter(|character| character.is_ascii_alphanumeric())
                        .flat_map(char::to_lowercase)
                        .collect::<String>();
                    let dynamic_command = parent == Some("commands");
                    let dynamic_preset = parent == Some("named-presets");
                    let protected_ack = key == "protected-override-acknowledgements";
                    let forbidden = [
                        "credential",
                        "password",
                        "secret",
                        "token",
                        "recent",
                        "path",
                        "project",
                        "macro",
                    ]
                    .iter()
                    .any(|term| normalized.contains(term));
                    if forbidden && !dynamic_command && !dynamic_preset && !protected_ack {
                        path.push(key.clone());
                        let rendered = path.join(".");
                        path.pop();
                        return Err(format!(
                            "shortcut library privacy contract rejects field '{rendered}'"
                        ));
                    }
                    if dynamic_command
                        && [
                            "credential",
                            "password",
                            "secret",
                            "token",
                            "recent",
                            "macro",
                        ]
                        .iter()
                        .any(|term| normalized.contains(term))
                    {
                        return Err(format!(
                            "shortcut library privacy contract rejects command entry '{key}'"
                        ));
                    }
                    path.push(key.clone());
                    walk(child, Some(key), path)?;
                    path.pop();
                }
                Ok(())
            }
            Value::Array(values) => {
                for (index, child) in values.iter().enumerate() {
                    path.push(index.to_string());
                    walk(child, parent, path)?;
                    path.pop();
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
    walk(value, None, &mut Vec::new())
}

fn encode_store_bytes(
    library: &ShortcutProfileLibrary,
    generation: u64,
) -> Result<Vec<u8>, ShortcutLibraryPersistenceError> {
    if generation == 0 {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.invalid-generation",
            "generation zero is reserved for missing storage",
        ));
    }
    validate_library(library).map_err(|message| {
        ShortcutLibraryPersistenceError::new("shortcut-library.invalid-library", message)
    })?;
    let library_value = serde_json::to_value(library).map_err(|error| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.serialize",
            format!("could not serialize shortcut profile library: {error}"),
        )
    })?;
    let payload_bytes = canonical_json_bytes(library_value.clone()).map_err(|error| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.serialize",
            format!("could not canonicalize shortcut profile library: {error}"),
        )
    })?;
    let bytes = canonical_json_bytes(serde_json::json!({
        "format": SHORTCUT_LIBRARY_STORE_FORMAT,
        "schema-version": SHORTCUT_LIBRARY_STORE_SCHEMA_VERSION,
        "generation": generation,
        "payload-digest": hex_digest(sha256(&payload_bytes)),
        "library": library_value,
    }))
    .map_err(|error| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.serialize",
            format!("could not serialize shortcut library store: {error}"),
        )
    })?;
    if bytes.len() > MAX_SHORTCUT_LIBRARY_STORE_BYTES {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.size-limit",
            format!(
                "shortcut library store requires {} bytes; limit is {} bytes",
                bytes.len(),
                MAX_SHORTCUT_LIBRARY_STORE_BYTES
            ),
        ));
    }
    match decode_store_bytes(&bytes) {
        ShortcutProfileLibraryRestore::Compatible(decoded)
            if decoded.library == *library && decoded.token.generation == generation => {}
        other => {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.self-validation",
                format!("serialized shortcut library failed strict self-validation: {other:?}"),
            ));
        }
    }
    Ok(bytes)
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn restore_from_store(
    store: &impl ShortcutLibraryStore,
) -> Result<ShortcutProfileLibraryRestore, ShortcutLibraryPersistenceError> {
    store.read().map(|bytes| {
        bytes
            .as_deref()
            .map_or(ShortcutProfileLibraryRestore::Missing, decode_store_bytes)
    })
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn create_in_store(
    store: &impl ShortcutLibraryStore,
    library: &ShortcutProfileLibrary,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    let prepared = prepare_create(library)?;
    store.compare_exchange(prepared.expected, &prepared.bytes)?;
    Ok(prepared.persisted)
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn update_in_store(
    store: &impl ShortcutLibraryStore,
    predecessor: &PersistedShortcutProfileLibrary,
    candidate: &ShortcutProfileLibrary,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    let prepared = prepare_update(predecessor, candidate)?;
    store.compare_exchange(prepared.expected, &prepared.bytes)?;
    Ok(prepared.persisted)
}

fn prepare_create(
    library: &ShortcutProfileLibrary,
) -> Result<PreparedStoreWrite, ShortcutLibraryPersistenceError> {
    let bytes = encode_store_bytes(library, 1)?;
    let persisted = compatible_after_publication(&bytes)?;
    Ok(PreparedStoreWrite {
        expected: StoreExpectation::Missing,
        bytes,
        persisted,
    })
}

fn prepare_update(
    predecessor: &PersistedShortcutProfileLibrary,
    candidate: &ShortcutProfileLibrary,
) -> Result<PreparedStoreWrite, ShortcutLibraryPersistenceError> {
    let predecessor_bytes = encode_store_bytes(&predecessor.library, predecessor.token.generation)?;
    if sha256(&predecessor_bytes) != predecessor.token.file_digest {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.invalid-predecessor",
            "predecessor snapshot does not match its persistence token",
        ));
    }
    if candidate.revision() <= predecessor.library.revision() {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.stale-revision",
            format!(
                "candidate revision {} must be newer than predecessor revision {}",
                candidate.revision(),
                predecessor.library.revision()
            ),
        ));
    }
    let generation = predecessor.token.generation.checked_add(1).ok_or_else(|| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.generation-exhausted",
            "shortcut library persistence generation is exhausted",
        )
    })?;
    let bytes = encode_store_bytes(candidate, generation)?;
    let persisted = compatible_after_publication(&bytes)?;
    Ok(PreparedStoreWrite {
        expected: StoreExpectation::Digest(predecessor.token.file_digest),
        bytes,
        persisted,
    })
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn edit_in_store(
    store: &impl ShortcutLibraryStore,
    predecessor: &PersistedShortcutProfileLibrary,
    edit: impl FnOnce(&mut ShortcutProfileLibrary) -> Result<(), String>,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    let mut candidate = predecessor.library.clone();
    edit(&mut candidate).map_err(|message| {
        ShortcutLibraryPersistenceError::new("shortcut-library.edit-rejected", message)
    })?;
    update_in_store(store, predecessor, &candidate)
}

fn compatible_after_publication(
    bytes: &[u8],
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    match decode_store_bytes(bytes) {
        ShortcutProfileLibraryRestore::Compatible(persisted) => Ok(*persisted),
        other => Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.self-validation",
            format!("published shortcut library is not restorable: {other:?}"),
        )),
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Debug, Clone)]
struct NativeShortcutLibraryStore {
    path: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl NativeShortcutLibraryStore {
    fn production() -> Result<Self, ShortcutLibraryPersistenceError> {
        let root = dirs::config_dir()
            .or_else(dirs::data_local_dir)
            .ok_or_else(|| {
                ShortcutLibraryPersistenceError::new(
                    "shortcut-library.config-directory",
                    "operating system did not provide a user configuration or local data directory",
                )
            })?;
        Ok(Self {
            path: root.join("RSpice").join(NATIVE_FILENAME),
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl ShortcutLibraryStore for NativeShortcutLibraryStore {
    fn read(&self) -> Result<Option<Vec<u8>>, ShortcutLibraryPersistenceError> {
        crate::io::durable_file::reconcile_publication(&self.path).map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.reconcile",
                format!("could not reconcile '{}': {error}", self.path.display()),
            )
        })?;
        let metadata = match std::fs::metadata(&self.path) {
            Ok(metadata) => metadata,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
            Err(error) => {
                return Err(ShortcutLibraryPersistenceError::new(
                    "shortcut-library.read",
                    format!("could not inspect '{}': {error}", self.path.display()),
                ));
            }
        };
        if !metadata.is_file() {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.read",
                format!("'{}' is not a regular file", self.path.display()),
            ));
        }
        if metadata.len() > MAX_SHORTCUT_LIBRARY_STORE_BYTES as u64 {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.size-limit",
                format!(
                    "'{}' exceeds the {} byte limit",
                    self.path.display(),
                    MAX_SHORTCUT_LIBRARY_STORE_BYTES
                ),
            ));
        }
        let bytes = std::fs::read(&self.path).map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.read",
                format!("could not read '{}': {error}", self.path.display()),
            )
        })?;
        if bytes.len() > MAX_SHORTCUT_LIBRARY_STORE_BYTES {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.size-limit",
                format!(
                    "'{}' grew beyond the {} byte limit while being read",
                    self.path.display(),
                    MAX_SHORTCUT_LIBRARY_STORE_BYTES
                ),
            ));
        }
        Ok(Some(bytes))
    }

    fn compare_exchange(
        &self,
        expected: StoreExpectation,
        bytes: &[u8],
    ) -> Result<(), ShortcutLibraryPersistenceError> {
        let expected = match expected {
            StoreExpectation::Missing => crate::io::durable_file::ExpectedContent::Missing,
            StoreExpectation::Digest(digest) => {
                crate::io::durable_file::ExpectedContent::Digest(digest)
            }
        };
        crate::io::durable_file::compare_exchange_bytes(&self.path, expected, bytes).map_err(
            |error| {
                ShortcutLibraryPersistenceError::new(
                    "shortcut-library.compare-exchange",
                    format!("could not publish '{}': {error}", self.path.display()),
                )
            },
        )
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub fn native_shortcut_library_path() -> Result<std::path::PathBuf, ShortcutLibraryPersistenceError>
{
    NativeShortcutLibraryStore::production().map(|store| store.path)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn restore_shortcut_profile_library_native()
-> Result<ShortcutProfileLibraryRestore, ShortcutLibraryPersistenceError> {
    restore_from_store(&NativeShortcutLibraryStore::production()?)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn create_shortcut_profile_library_native(
    library: &ShortcutProfileLibrary,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    create_in_store(&NativeShortcutLibraryStore::production()?, library)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn update_shortcut_profile_library_native(
    predecessor: &PersistedShortcutProfileLibrary,
    candidate: &ShortcutProfileLibrary,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    update_in_store(
        &NativeShortcutLibraryStore::production()?,
        predecessor,
        candidate,
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn edit_shortcut_profile_library_native(
    predecessor: &PersistedShortcutProfileLibrary,
    edit: impl FnOnce(&mut ShortcutProfileLibrary) -> Result<(), String>,
) -> Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError> {
    edit_in_store(
        &NativeShortcutLibraryStore::production()?,
        predecessor,
        edit,
    )
}

#[cfg(not(target_arch = "wasm32"))]
pub fn startup_shortcut_profile_library_native(
    compatible_session_library: Option<&ShortcutProfileLibrary>,
) -> Result<ShortcutProfileLibraryStartup, ShortcutLibraryPersistenceError> {
    let store = NativeShortcutLibraryStore::production()?;
    startup_from_store(&store, compatible_session_library)
}

#[cfg(any(test, not(target_arch = "wasm32")))]
fn startup_from_store(
    store: &impl ShortcutLibraryStore,
    compatible_session_library: Option<&ShortcutProfileLibrary>,
) -> Result<ShortcutProfileLibraryStartup, ShortcutLibraryPersistenceError> {
    Ok(match restore_from_store(store)? {
        ShortcutProfileLibraryRestore::Compatible(persisted) => {
            ShortcutProfileLibraryStartup::Ready {
                persisted,
                initialized_from_session: false,
            }
        }
        ShortcutProfileLibraryRestore::Missing => {
            if let Some(library) =
                compatible_session_library.filter(|library| library.is_compatible())
            {
                ShortcutProfileLibraryStartup::Ready {
                    persisted: Box::new(create_in_store(store, library)?),
                    initialized_from_session: true,
                }
            } else {
                ShortcutProfileLibraryStartup::Missing
            }
        }
        ShortcutProfileLibraryRestore::Incompatible(raw) => {
            ShortcutProfileLibraryStartup::Incompatible(raw)
        }
        ShortcutProfileLibraryRestore::Corrupt(raw) => ShortcutProfileLibraryStartup::Corrupt(raw),
    })
}

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrowserShortcutLibraryWriteToken(u64);

#[cfg(any(test, target_arch = "wasm32"))]
#[derive(Debug, Default)]
struct BrowserWriteLeaseState {
    next_generation: u64,
    active: Option<BrowserShortcutLibraryWriteToken>,
    commit_verified: bool,
    completed: bool,
}

#[cfg(any(test, target_arch = "wasm32"))]
impl BrowserWriteLeaseState {
    fn begin(
        &mut self,
    ) -> Result<BrowserShortcutLibraryWriteToken, ShortcutLibraryPersistenceError> {
        if self.active.is_some() {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-write-pending",
                "a shortcut library browser write is already pending or awaiting acknowledgement",
            ));
        }
        self.next_generation = self.next_generation.checked_add(1).ok_or_else(|| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-token-exhausted",
                "browser shortcut library operation tokens are exhausted",
            )
        })?;
        let token = BrowserShortcutLibraryWriteToken(self.next_generation);
        self.active = Some(token);
        self.commit_verified = false;
        self.completed = false;
        Ok(token)
    }

    fn is_current(&self, token: BrowserShortcutLibraryWriteToken) -> bool {
        self.active == Some(token) && !self.completed
    }

    fn complete(&mut self, token: BrowserShortcutLibraryWriteToken) -> bool {
        if !self.is_current(token) {
            return false;
        }
        self.completed = true;
        true
    }

    /// Cross the irreversible publication boundary after an exact storage
    /// reread. From this point cancellation must fail and the owner must poll
    /// the eventual completion, even while Web Lock cleanup is still pending.
    fn mark_commit_verified(&mut self, token: BrowserShortcutLibraryWriteToken) -> bool {
        if !self.is_current(token) {
            return false;
        }
        self.commit_verified = true;
        true
    }

    fn take_completed(&mut self, token: BrowserShortcutLibraryWriteToken) -> bool {
        if self.active != Some(token) || !self.completed {
            return false;
        }
        self.active = None;
        self.commit_verified = false;
        self.completed = false;
        true
    }

    fn cancel(&mut self, token: BrowserShortcutLibraryWriteToken) -> bool {
        if self.active != Some(token) || self.commit_verified || self.completed {
            return false;
        }
        self.active = None;
        self.commit_verified = false;
        self.completed = false;
        true
    }
}

#[cfg(any(test, target_arch = "wasm32"))]
fn finish_verified_browser_commit(
    commit: Result<(), ShortcutLibraryPersistenceError>,
    release: Result<(), ShortcutLibraryPersistenceError>,
) -> Result<(), ShortcutLibraryPersistenceError> {
    match commit {
        Ok(()) => {
            if let Err(error) = release {
                // Exact localStorage reread is the durable commit boundary.
                // A lock-manager cleanup error cannot truthfully reverse it.
                log::warn!(
                    "verified shortcut library commit succeeded, but Web Lock release reported: {error}"
                );
            }
            Ok(())
        }
        Err(error) => Err(error),
    }
}

#[cfg(target_arch = "wasm32")]
#[derive(Default)]
struct BrowserWriteOwner {
    lease: BrowserWriteLeaseState,
    completion: Option<Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError>>,
    repaint: Option<egui::Context>,
}

#[cfg(target_arch = "wasm32")]
thread_local! {
    static BROWSER_WRITE_OWNER: std::cell::RefCell<BrowserWriteOwner> =
        std::cell::RefCell::new(BrowserWriteOwner::default());
}

#[cfg(target_arch = "wasm32")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserShortcutProfileLibraryStartup {
    Ready(Box<PersistedShortcutProfileLibrary>),
    InitializationStarted(BrowserShortcutLibraryWriteToken),
    Missing,
    Incompatible(RetainedShortcutLibraryBytes),
    Corrupt(RetainedShortcutLibraryBytes),
}

/// Synchronously reads one complete localStorage value. The returned bytes are
/// an atomic browser storage observation; no eframe storage mirror is involved.
#[cfg(target_arch = "wasm32")]
pub fn restore_shortcut_profile_library_browser()
-> Result<ShortcutProfileLibraryRestore, ShortcutLibraryPersistenceError> {
    Ok(browser_storage_bytes()?
        .as_deref()
        .map_or(ShortcutProfileLibraryRestore::Missing, decode_store_bytes))
}

#[cfg(target_arch = "wasm32")]
pub fn start_create_shortcut_profile_library_browser(
    library: &ShortcutProfileLibrary,
    repaint: &egui::Context,
) -> Result<BrowserShortcutLibraryWriteToken, ShortcutLibraryPersistenceError> {
    start_browser_write(prepare_create(library)?, repaint)
}

#[cfg(target_arch = "wasm32")]
pub fn start_update_shortcut_profile_library_browser(
    predecessor: &PersistedShortcutProfileLibrary,
    candidate: &ShortcutProfileLibrary,
    repaint: &egui::Context,
) -> Result<BrowserShortcutLibraryWriteToken, ShortcutLibraryPersistenceError> {
    start_browser_write(prepare_update(predecessor, candidate)?, repaint)
}

/// Clone, edit, fully validate, and stage a browser CAS without changing the
/// caller's live library. Apply only the library returned by `poll_*` success.
#[cfg(target_arch = "wasm32")]
pub fn start_edit_shortcut_profile_library_browser(
    predecessor: &PersistedShortcutProfileLibrary,
    edit: impl FnOnce(&mut ShortcutProfileLibrary) -> Result<(), String>,
    repaint: &egui::Context,
) -> Result<BrowserShortcutLibraryWriteToken, ShortcutLibraryPersistenceError> {
    let mut candidate = predecessor.library.clone();
    edit(&mut candidate).map_err(|message| {
        ShortcutLibraryPersistenceError::new("shortcut-library.edit-rejected", message)
    })?;
    start_browser_write(prepare_update(predecessor, &candidate)?, repaint)
}

#[cfg(target_arch = "wasm32")]
pub fn poll_shortcut_profile_library_browser_write(
    token: BrowserShortcutLibraryWriteToken,
) -> Option<Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError>> {
    BROWSER_WRITE_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        if !owner.lease.take_completed(token) {
            return None;
        }
        owner.repaint = None;
        owner.completion.take()
    })
}

/// Cancels an operation before its synchronous localStorage commit point.
/// A late task checks the generation again while holding the Web Lock and is
/// therefore unable to publish or complete over a replacement operation.
#[cfg(target_arch = "wasm32")]
pub fn cancel_shortcut_profile_library_browser_write(
    token: BrowserShortcutLibraryWriteToken,
) -> bool {
    BROWSER_WRITE_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        let cancelled = owner.lease.cancel(token);
        if cancelled {
            owner.completion = None;
            if let Some(repaint) = owner.repaint.take() {
                repaint.request_repaint();
            }
        }
        cancelled
    })
}

#[cfg(target_arch = "wasm32")]
pub fn startup_shortcut_profile_library_browser(
    compatible_session_library: Option<&ShortcutProfileLibrary>,
    repaint: &egui::Context,
) -> Result<BrowserShortcutProfileLibraryStartup, ShortcutLibraryPersistenceError> {
    Ok(match restore_shortcut_profile_library_browser()? {
        ShortcutProfileLibraryRestore::Compatible(persisted) => {
            BrowserShortcutProfileLibraryStartup::Ready(persisted)
        }
        ShortcutProfileLibraryRestore::Missing => {
            if let Some(library) =
                compatible_session_library.filter(|library| library.is_compatible())
            {
                BrowserShortcutProfileLibraryStartup::InitializationStarted(
                    start_create_shortcut_profile_library_browser(library, repaint)?,
                )
            } else {
                BrowserShortcutProfileLibraryStartup::Missing
            }
        }
        ShortcutProfileLibraryRestore::Incompatible(raw) => {
            BrowserShortcutProfileLibraryStartup::Incompatible(raw)
        }
        ShortcutProfileLibraryRestore::Corrupt(raw) => {
            BrowserShortcutProfileLibraryStartup::Corrupt(raw)
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn start_browser_write(
    prepared: PreparedStoreWrite,
    repaint: &egui::Context,
) -> Result<BrowserShortcutLibraryWriteToken, ShortcutLibraryPersistenceError> {
    let token = BROWSER_WRITE_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        let token = owner.lease.begin()?;
        owner.completion = None;
        owner.repaint = Some(repaint.clone());
        Ok(token)
    })?;
    wasm_bindgen_futures::spawn_local(async move {
        let result = browser_compare_exchange(token, prepared.expected, &prepared.bytes)
            .await
            .map(|()| prepared.persisted);
        finish_browser_write(token, result);
    });
    Ok(token)
}

#[cfg(target_arch = "wasm32")]
fn browser_operation_is_current(token: BrowserShortcutLibraryWriteToken) -> bool {
    BROWSER_WRITE_OWNER.with(|owner| owner.borrow().lease.is_current(token))
}

#[cfg(target_arch = "wasm32")]
fn mark_browser_commit_verified(token: BrowserShortcutLibraryWriteToken) -> bool {
    BROWSER_WRITE_OWNER.with(|owner| owner.borrow_mut().lease.mark_commit_verified(token))
}

#[cfg(target_arch = "wasm32")]
fn finish_browser_write(
    token: BrowserShortcutLibraryWriteToken,
    result: Result<PersistedShortcutProfileLibrary, ShortcutLibraryPersistenceError>,
) {
    let repaint = BROWSER_WRITE_OWNER.with(|owner| {
        let mut owner = owner.borrow_mut();
        if !owner.lease.complete(token) {
            return None;
        }
        owner.completion = Some(result);
        owner.repaint.clone()
    });
    if let Some(repaint) = repaint {
        repaint.request_repaint();
    }
}

#[cfg(target_arch = "wasm32")]
fn browser_storage() -> Result<web_sys::Storage, ShortcutLibraryPersistenceError> {
    web_sys::window()
        .ok_or_else(|| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-storage",
                "browser window is unavailable",
            )
        })?
        .local_storage()
        .map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-storage",
                format!("localStorage access was rejected: {}", js_error(error)),
            )
        })?
        .ok_or_else(|| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-storage",
                "localStorage is unavailable",
            )
        })
}

#[cfg(target_arch = "wasm32")]
fn browser_storage_bytes() -> Result<Option<Vec<u8>>, ShortcutLibraryPersistenceError> {
    let value = browser_storage()?
        .get_item(BROWSER_STORAGE_KEY)
        .map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-read",
                format!("localStorage read was rejected: {}", js_error(error)),
            )
        })?;
    let Some(value) = value else {
        return Ok(None);
    };
    if value.len() > MAX_SHORTCUT_LIBRARY_STORE_BYTES {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.size-limit",
            format!(
                "browser shortcut library store exceeds the {} byte limit",
                MAX_SHORTCUT_LIBRARY_STORE_BYTES
            ),
        ));
    }
    Ok(Some(value.into_bytes()))
}

#[cfg(target_arch = "wasm32")]
async fn browser_compare_exchange(
    token: BrowserShortcutLibraryWriteToken,
    expected: StoreExpectation,
    bytes: &[u8],
) -> Result<(), ShortcutLibraryPersistenceError> {
    let lock = acquire_shortcut_library_web_lock().await?;
    let result = (|| {
        if !browser_operation_is_current(token) {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-cancelled",
                "browser shortcut library write was cancelled before publication",
            ));
        }
        let storage = browser_storage()?;
        let current = storage.get_item(BROWSER_STORAGE_KEY).map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-read",
                format!("localStorage CAS read was rejected: {}", js_error(error)),
            )
        })?;
        let current_digest = current.as_deref().map(str::as_bytes).map(sha256);
        let matches = match expected {
            StoreExpectation::Missing => current.is_none(),
            StoreExpectation::Digest(digest) => current_digest == Some(digest),
        };
        if !matches {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.compare-exchange",
                format!(
                    "browser shortcut library changed before publication (expected {expected:?}, found {current_digest:?})"
                ),
            ));
        }
        // No await is permitted from this generation check through setItem:
        // cancellation and another task cannot interleave at the commit point.
        if !browser_operation_is_current(token) {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-cancelled",
                "browser shortcut library write was cancelled before publication",
            ));
        }
        let text = std::str::from_utf8(bytes).map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.serialize",
                format!("canonical shortcut library bytes are not UTF-8: {error}"),
            )
        })?;
        storage
            .set_item(BROWSER_STORAGE_KEY, text)
            .map_err(|error| {
                ShortcutLibraryPersistenceError::new(
                    "shortcut-library.browser-publish",
                    format!(
                        "localStorage publication was rejected (quota/security): {}",
                        js_error(error)
                    ),
                )
            })?;
        // setItem has returned successfully. This is now non-cancellable even
        // if the exact verification read fails and the commit must be reported
        // as in-doubt rather than falsely restoring predecessor authority.
        if !mark_browser_commit_verified(token) {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-commit-in-doubt",
                "shortcut library bytes were stored, but publication ownership was lost before verification",
            ));
        }
        let verified = storage.get_item(BROWSER_STORAGE_KEY).map_err(|error| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-commit-in-doubt",
                format!(
                    "shortcut library bytes were stored, but exact verification could not be read: {}",
                    js_error(error)
                ),
            )
        })?;
        if verified.as_deref().map(str::as_bytes) != Some(bytes) {
            return Err(ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-commit-in-doubt",
                "shortcut library bytes were stored, but the exact verification read did not match",
            ));
        }
        Ok(())
    })();
    let release = release_shortcut_library_web_lock(lock).await;
    finish_verified_browser_commit(result, release)
}

#[cfg(target_arch = "wasm32")]
struct ShortcutLibraryWebLock {
    release: js_sys::Function,
    request: js_sys::Promise,
    _callback:
        wasm_bindgen::closure::Closure<dyn FnMut(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue>,
}

#[cfg(target_arch = "wasm32")]
async fn acquire_shortcut_library_web_lock()
-> Result<ShortcutLibraryWebLock, ShortcutLibraryPersistenceError> {
    use wasm_bindgen::JsCast as _;

    let window = web_sys::window().ok_or_else(|| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock",
            "browser window is unavailable",
        )
    })?;
    let navigator = js_sys::Reflect::get(&window, &wasm_bindgen::JsValue::from_str("navigator"))
        .map_err(|error| browser_lock_error("could not inspect browser navigator", error))?;
    let locks = js_sys::Reflect::get(&navigator, &wasm_bindgen::JsValue::from_str("locks"))
        .map_err(|error| browser_lock_error("could not inspect Web Locks", error))?;
    let request_method = js_sys::Reflect::get(&locks, &wasm_bindgen::JsValue::from_str("request"))
        .map_err(|error| browser_lock_error("could not inspect Web Locks request", error))?;
    if !request_method.is_function() {
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock",
            "Web Locks API is unavailable; cross-tab publication fails closed",
        ));
    }

    let mut release_resolver = None;
    let release_promise = js_sys::Promise::new(&mut |resolve, _reject| {
        release_resolver = Some(resolve.clone());
    });
    let release = release_resolver.ok_or_else(|| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock",
            "could not initialize Web Lock release",
        )
    })?;
    let acquired_resolver = std::rc::Rc::new(std::cell::RefCell::new(None));
    let acquired_resolver_for_promise = acquired_resolver.clone();
    let acquired_promise = js_sys::Promise::new(&mut move |resolve, _reject| {
        *acquired_resolver_for_promise.borrow_mut() = Some(resolve.clone());
    });
    let callback_resolver = acquired_resolver;
    let callback_release = release_promise.clone();
    let callback = wasm_bindgen::closure::Closure::<
        dyn FnMut(wasm_bindgen::JsValue) -> wasm_bindgen::JsValue,
    >::new(move |lock: wasm_bindgen::JsValue| {
        let acquired = !lock.is_null() && !lock.is_undefined();
        if let Some(resolve) = callback_resolver.borrow_mut().take() {
            let _ = resolve.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_bool(acquired),
            );
        }
        if acquired {
            callback_release.clone().into()
        } else {
            wasm_bindgen::JsValue::UNDEFINED
        }
    });
    let options = js_sys::Object::new();
    set_js_field(
        &options,
        "mode",
        &wasm_bindgen::JsValue::from_str("exclusive"),
    )?;
    set_js_field(
        &options,
        "ifAvailable",
        &wasm_bindgen::JsValue::from_bool(true),
    )?;
    let request = call_js_method(
        &locks,
        "request",
        &[
            wasm_bindgen::JsValue::from_str(BROWSER_LOCK_NAME),
            options.into(),
            callback.as_ref().clone(),
        ],
    )?
    .dyn_into::<js_sys::Promise>()
    .map_err(|_| {
        ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock",
            "Web Locks request did not return a Promise",
        )
    })?;
    let acquired = match await_browser_promise(acquired_promise, "Web Lock acquisition").await {
        Ok(value) => value.as_bool().unwrap_or(false),
        Err(error) => {
            let _ = release.call0(&wasm_bindgen::JsValue::UNDEFINED);
            callback.forget();
            return Err(error);
        }
    };
    if !acquired {
        let _ = await_browser_promise(request, "unavailable Web Lock cleanup").await;
        return Err(ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock-busy",
            "shortcut library is being saved in another RSpice tab",
        ));
    }
    Ok(ShortcutLibraryWebLock {
        release,
        request,
        _callback: callback,
    })
}

#[cfg(target_arch = "wasm32")]
async fn release_shortcut_library_web_lock(
    lock: ShortcutLibraryWebLock,
) -> Result<(), ShortcutLibraryPersistenceError> {
    lock.release
        .call0(&wasm_bindgen::JsValue::UNDEFINED)
        .map_err(|error| {
            browser_lock_error("could not release shortcut library Web Lock", error)
        })?;
    await_browser_promise(lock.request, "Web Lock release")
        .await
        .map(|_| ())
}

#[cfg(target_arch = "wasm32")]
async fn await_browser_promise(
    promise: js_sys::Promise,
    operation: &str,
) -> Result<wasm_bindgen::JsValue, ShortcutLibraryPersistenceError> {
    let race = js_sys::Array::new();
    race.push(&promise);
    race.push(&browser_timeout_promise(operation));
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::race(&race))
        .await
        .map_err(|error| browser_lock_error(operation, error))
}

#[cfg(target_arch = "wasm32")]
fn browser_timeout_promise(operation: &str) -> js_sys::Promise {
    let message = format!(
        "{operation} timed out after {} seconds",
        BROWSER_WRITE_TIMEOUT_MS / 1_000
    );
    js_sys::Promise::new(&mut move |_resolve, reject| {
        let timeout_reject = reject.clone();
        let timeout_message = message.clone();
        let callback = wasm_bindgen::closure::Closure::once_into_js(move || {
            let _ = timeout_reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(&timeout_message),
            );
        });
        let Some(window) = web_sys::window() else {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str("browser window is unavailable"),
            );
            return;
        };
        if let Err(error) = call_js_method(
            &window.into(),
            "setTimeout",
            &[
                callback,
                wasm_bindgen::JsValue::from_f64(BROWSER_WRITE_TIMEOUT_MS as f64),
            ],
        ) {
            let _ = reject.call1(
                &wasm_bindgen::JsValue::UNDEFINED,
                &wasm_bindgen::JsValue::from_str(error.message()),
            );
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn call_js_method(
    receiver: &wasm_bindgen::JsValue,
    name: &str,
    arguments: &[wasm_bindgen::JsValue],
) -> Result<wasm_bindgen::JsValue, ShortcutLibraryPersistenceError> {
    use wasm_bindgen::JsCast as _;

    let function = js_sys::Reflect::get(receiver, &wasm_bindgen::JsValue::from_str(name))
        .map_err(|error| browser_lock_error(&format!("could not inspect {name}"), error))?
        .dyn_into::<js_sys::Function>()
        .map_err(|_| {
            ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-api",
                format!("browser object has no callable {name} method"),
            )
        })?;
    let args = js_sys::Array::new();
    for argument in arguments {
        args.push(argument);
    }
    function
        .apply(receiver, &args)
        .map_err(|error| browser_lock_error(&format!("browser {name} call failed"), error))
}

#[cfg(target_arch = "wasm32")]
fn set_js_field(
    object: &js_sys::Object,
    name: &str,
    value: &wasm_bindgen::JsValue,
) -> Result<(), ShortcutLibraryPersistenceError> {
    js_sys::Reflect::set(object, &wasm_bindgen::JsValue::from_str(name), value)
        .map(|_| ())
        .map_err(|error| browser_lock_error(&format!("could not set Web Lock {name}"), error))
}

#[cfg(target_arch = "wasm32")]
fn browser_lock_error(
    operation: &str,
    error: wasm_bindgen::JsValue,
) -> ShortcutLibraryPersistenceError {
    ShortcutLibraryPersistenceError::new(
        "shortcut-library.browser-lock",
        format!("{operation}: {}", js_error(error)),
    )
}

#[cfg(target_arch = "wasm32")]
fn js_error(value: wasm_bindgen::JsValue) -> String {
    js_sys::Reflect::get(&value, &wasm_bindgen::JsValue::from_str("message"))
        .ok()
        .and_then(|message| message.as_string())
        .or_else(|| value.as_string())
        .unwrap_or_else(|| format!("browser error: {value:?}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    #[derive(Default)]
    struct MemoryStore {
        bytes: RefCell<Option<Vec<u8>>>,
    }

    impl ShortcutLibraryStore for MemoryStore {
        fn read(&self) -> Result<Option<Vec<u8>>, ShortcutLibraryPersistenceError> {
            Ok(self.bytes.borrow().clone())
        }

        fn compare_exchange(
            &self,
            expected: StoreExpectation,
            bytes: &[u8],
        ) -> Result<(), ShortcutLibraryPersistenceError> {
            let actual = self.bytes.borrow().as_deref().map(sha256);
            let matches = match expected {
                StoreExpectation::Missing => actual.is_none(),
                StoreExpectation::Digest(expected) => actual == Some(expected),
            };
            if !matches {
                return Err(ShortcutLibraryPersistenceError::new(
                    "shortcut-library.compare-exchange",
                    "memory store changed",
                ));
            }
            *self.bytes.borrow_mut() = Some(bytes.to_vec());
            Ok(())
        }
    }

    fn edited(predecessor: &PersistedShortcutProfileLibrary) -> ShortcutProfileLibrary {
        let mut candidate = predecessor.library().clone();
        candidate
            .edit_active(|profile| {
                profile.policies_mut().set_chord_timeout(
                    crate::workbench::shortcuts::ChordTimeoutPolicy::ThreeSeconds,
                );
            })
            .unwrap();
        candidate
    }

    #[test]
    fn deterministic_create_and_strict_restore() {
        let library = ShortcutProfileLibrary::default();
        let first = encode_store_bytes(&library, 1).unwrap();
        let second = encode_store_bytes(&library, 1).unwrap();
        assert_eq!(first, second);
        assert!(matches!(
            decode_store_bytes(&first),
            ShortcutProfileLibraryRestore::Compatible(_)
        ));
        let mut noncanonical = first;
        noncanonical.pop();
        assert!(matches!(
            decode_store_bytes(&noncanonical),
            ShortcutProfileLibraryRestore::Corrupt(_)
        ));
    }

    #[test]
    fn create_is_create_only_and_update_requires_exact_predecessor() {
        let store = MemoryStore::default();
        let first = create_in_store(&store, &ShortcutProfileLibrary::default()).unwrap();
        assert!(create_in_store(&store, &ShortcutProfileLibrary::default()).is_err());
        let candidate = edited(&first);
        let second = update_in_store(&store, &first, &candidate).unwrap();
        assert_eq!(second.token().generation(), 2);
        assert!(update_in_store(&store, &first, &edited(&first)).is_err());
    }

    #[test]
    fn late_external_edits_are_not_clobbered() {
        let store = MemoryStore::default();
        let first = create_in_store(&store, &ShortcutProfileLibrary::default()).unwrap();
        *store.bytes.borrow_mut() = Some(b"foreign late edit\n".to_vec());
        assert!(update_in_store(&store, &first, &edited(&first)).is_err());
        assert_eq!(
            store.bytes.borrow().as_deref(),
            Some(&b"foreign late edit\n"[..])
        );
    }

    #[test]
    fn future_and_corrupt_bytes_are_retained_and_never_initialized_over() {
        let store = MemoryStore::default();
        let future = canonical_json_bytes(serde_json::json!({
            "format": SHORTCUT_LIBRARY_STORE_FORMAT,
            "schema-version": 99,
            "generation": 7,
            "payload-digest": "future",
            "library": {},
        }))
        .unwrap();
        *store.bytes.borrow_mut() = Some(future.clone());
        let startup = startup_from_store(&store, Some(&ShortcutProfileLibrary::default())).unwrap();
        let ShortcutProfileLibraryStartup::Incompatible(raw) = startup else {
            panic!("future store was not retained");
        };
        assert_eq!(raw.raw_bytes(), future);
        assert_eq!(store.bytes.borrow().as_deref(), Some(future.as_slice()));

        let corrupt = b"{not-json".to_vec();
        *store.bytes.borrow_mut() = Some(corrupt.clone());
        assert!(matches!(
            startup_from_store(&store, Some(&ShortcutProfileLibrary::default())).unwrap(),
            ShortcutProfileLibraryStartup::Corrupt(_)
        ));
        assert_eq!(store.bytes.borrow().as_deref(), Some(corrupt.as_slice()));
    }

    #[test]
    fn missing_store_may_initialize_once_from_compatible_session() {
        let store = MemoryStore::default();
        let startup = startup_from_store(&store, Some(&ShortcutProfileLibrary::default())).unwrap();
        let ShortcutProfileLibraryStartup::Ready {
            persisted,
            initialized_from_session,
        } = startup
        else {
            panic!("compatible session was not initialized");
        };
        assert!(initialized_from_session);
        assert_eq!(persisted.token().generation(), 1);
    }

    #[test]
    fn edit_closure_persists_candidate_before_returning_it_for_live_apply() {
        let store = MemoryStore::default();
        let first = create_in_store(&store, &ShortcutProfileLibrary::default()).unwrap();
        let next = edit_in_store(&store, &first, |library| {
            library
                .edit_active(|profile| {
                    profile.policies_mut().set_chord_timeout(
                        crate::workbench::shortcuts::ChordTimeoutPolicy::ThreeSeconds,
                    );
                })
                .map_err(|error| error.to_string())?;
            Ok(())
        })
        .unwrap();
        assert_eq!(
            next.library().active().policies().chord_timeout(),
            crate::workbench::shortcuts::ChordTimeoutPolicy::ThreeSeconds
        );
        assert_eq!(next.token().generation(), 2);
    }

    #[test]
    fn privacy_contract_rejects_project_paths_macros_and_credentials() {
        for field in [
            "project-id",
            "source-path",
            "recent-files",
            "macro-source",
            "credential-token",
            "access-token",
        ] {
            let mut value = serde_json::to_value(ShortcutProfileLibrary::default()).unwrap();
            value
                .as_object_mut()
                .unwrap()
                .insert(field.to_owned(), Value::String("private".to_owned()));
            assert!(
                validate_device_local_privacy(&value).is_err(),
                "accepted {field}"
            );
        }
        let allowed = serde_json::json!({
            "commands": {"open-project": {"bindings": []}},
            "protected-override-acknowledgements": ["save-project"]
        });
        validate_device_local_privacy(&allowed).unwrap();
    }

    #[test]
    fn protected_override_acknowledgements_round_trip_in_device_local_store() {
        let library: ShortcutProfileLibrary = serde_json::from_value(serde_json::json!({
            "library-version": 1,
            "active": {
                "protected-override-acknowledgements": ["save-project"]
            },
            "named-presets": {},
            "revision": 0
        }))
        .unwrap();
        let bytes = encode_store_bytes(&library, 1).unwrap();
        let ShortcutProfileLibraryRestore::Compatible(restored) = decode_store_bytes(&bytes) else {
            panic!("protected acknowledgement store did not restore");
        };
        assert!(
            restored
                .library()
                .active()
                .protected_override_acknowledged(crate::workbench::commands::Command::Save)
        );
    }

    #[test]
    fn current_envelope_with_revision_max_is_retained_as_incompatible() {
        let mut value = serde_json::to_value(ShortcutProfileLibrary::default()).unwrap();
        value["revision"] = Value::from(u64::MAX);
        let payload = canonical_json_bytes(value.clone()).unwrap();
        let bytes = canonical_json_bytes(serde_json::json!({
            "format": SHORTCUT_LIBRARY_STORE_FORMAT,
            "schema-version": SHORTCUT_LIBRARY_STORE_SCHEMA_VERSION,
            "generation": 9,
            "payload-digest": hex_digest(sha256(&payload)),
            "library": value,
        }))
        .unwrap();
        assert!(matches!(
            decode_store_bytes(&bytes),
            ShortcutProfileLibraryRestore::Incompatible(_)
        ));
    }

    #[test]
    fn byte_limit_fails_before_store_mutation() {
        let store = MemoryStore::default();
        let oversized = "x".repeat(MAX_SHORTCUT_LIBRARY_STORE_BYTES);
        let library: ShortcutProfileLibrary = serde_json::from_value(serde_json::json!({
            "library-version": 1,
            "active": {"future-field": oversized},
            "named-presets": {},
            "revision": 0
        }))
        .unwrap();
        let error = create_in_store(&store, &library).unwrap_err();
        assert_eq!(error.code(), "shortcut-library.size-limit");
        assert!(store.bytes.borrow().is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn native_path_backend_round_trips_and_detects_cas_conflict() {
        let root =
            std::env::temp_dir().join(format!("rspice-shortcut-store-{}", uuid::Uuid::new_v4()));
        let store = NativeShortcutLibraryStore {
            path: root.join(NATIVE_FILENAME),
        };
        let first = create_in_store(&store, &ShortcutProfileLibrary::default()).unwrap();
        let restored = restore_from_store(&store).unwrap();
        assert!(matches!(
            restored,
            ShortcutProfileLibraryRestore::Compatible(_)
        ));
        std::fs::write(&store.path, b"late external edit\n").unwrap();
        assert!(update_in_store(&store, &first, &edited(&first)).is_err());
        assert_eq!(std::fs::read(&store.path).unwrap(), b"late external edit\n");
        let _ = std::fs::remove_dir_all(root);
    }

    #[test]
    fn browser_write_state_retains_completion_until_exact_token_poll() {
        let mut state = BrowserWriteLeaseState::default();
        let token = state.begin().unwrap();
        assert!(state.is_current(token));
        assert!(state.complete(token));
        assert!(!state.is_current(token));
        assert!(state.take_completed(token));
        assert!(!state.take_completed(token));
    }

    #[test]
    fn browser_write_state_ignores_cancelled_and_stale_completions() {
        let mut state = BrowserWriteLeaseState::default();
        let cancelled = state.begin().unwrap();
        assert!(state.cancel(cancelled));
        let replacement = state.begin().unwrap();
        assert_ne!(cancelled, replacement);
        assert!(!state.complete(cancelled));
        assert!(state.is_current(replacement));
        assert!(state.complete(replacement));
        assert!(state.take_completed(replacement));
    }

    #[test]
    fn browser_write_state_allows_only_one_owner_until_poll_or_cancel() {
        let mut state = BrowserWriteLeaseState::default();
        let first = state.begin().unwrap();
        assert_eq!(
            state.begin().unwrap_err().code(),
            "shortcut-library.browser-write-pending"
        );
        assert!(state.complete(first));
        assert_eq!(
            state.begin().unwrap_err().code(),
            "shortcut-library.browser-write-pending"
        );
        assert!(state.take_completed(first));
        assert!(state.begin().is_ok());
    }

    #[test]
    fn verified_browser_commit_cannot_be_cancelled_before_cleanup_completes() {
        let mut state = BrowserWriteLeaseState::default();
        let cancellable = state.begin().unwrap();
        assert!(state.cancel(cancellable));

        let committed = state.begin().unwrap();
        assert!(state.mark_commit_verified(committed));
        assert!(!state.cancel(committed));
        assert!(state.complete(committed));
        assert!(!state.cancel(committed));
        assert!(state.take_completed(committed));
        assert!(state.begin().is_ok());
    }

    #[test]
    fn verified_browser_commit_is_authoritative_even_if_lock_release_fails() {
        let release_error = ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-lock",
            "simulated release failure",
        );
        assert!(finish_verified_browser_commit(Ok(()), Err(release_error)).is_ok());

        let commit_error = ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-publish",
            "simulated commit failure",
        );
        assert_eq!(
            finish_verified_browser_commit(Err(commit_error.clone()), Ok(())).unwrap_err(),
            commit_error
        );
    }

    #[test]
    fn commit_in_doubt_errors_are_typed_and_never_retryable_as_ordinary_failures() {
        let error = ShortcutLibraryPersistenceError::new(
            "shortcut-library.browser-commit-in-doubt",
            "simulated post-set verification failure",
        );
        assert!(error.is_commit_in_doubt());
        assert!(
            !ShortcutLibraryPersistenceError::new(
                "shortcut-library.browser-publish",
                "simulated pre-commit failure",
            )
            .is_commit_in_doubt()
        );
    }
}
