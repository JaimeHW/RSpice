//! Circuit builder - constructs CircuitData from Netlist
//!
//! This module handles the conversion from parsed netlist elements
//! to the runtime circuit representation.

use super::{Engine, SimulationError, extract_dc_value};
use crate::netlist::{ElementKind, flatten_netlist};
use crate::{CircuitData, Netlist};
#[cfg(feature = "veriloga")]
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[cfg(feature = "veriloga")]
use std::io::{Read, Write};
#[cfg(feature = "veriloga")]
use std::path::{Path, PathBuf};
#[cfg(all(test, feature = "veriloga"))]
use std::sync::Mutex;
use std::sync::OnceLock;
#[cfg(feature = "veriloga")]
use std::sync::RwLock;
#[cfg(feature = "veriloga")]
use std::time::{Duration, Instant};

/// Embedded transistor model library used for fallback model resolution.
const BUILTIN_TRANSISTOR_LIB: &str = include_str!("../../../../models/spice/transistor.lib");

/// Lazily parsed builtin BJT model parameter map (MODEL_NAME -> params).
fn builtin_bjt_model_map() -> &'static HashMap<String, HashMap<String, f64>> {
    static BJT_MODELS: OnceLock<HashMap<String, HashMap<String, f64>>> = OnceLock::new();
    BJT_MODELS.get_or_init(|| {
        let mut map = HashMap::new();
        let Ok(netlist) = crate::netlist::parse_netlist(BUILTIN_TRANSISTOR_LIB) else {
            log::warn!("Failed to parse embedded transistor library for BJT fallback models");
            return map;
        };

        for model in netlist.models {
            if model.model_type.eq_ignore_ascii_case("NPN")
                || model.model_type.eq_ignore_ascii_case("PNP")
            {
                map.insert(
                    model.name.to_uppercase(),
                    model.params.into_iter().collect(),
                );
            }
        }
        map
    })
}

#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_RECORD_VERSION: u32 = 1;
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_LOCK_FILE: &str = ".rspice-veriloga-cache.lock";
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_LOCK_WAIT_TIMEOUT: Duration = Duration::from_secs(15);
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_LOCK_STALE_TIMEOUT: Duration = Duration::from_secs(180);
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_LOCK_POLL_INTERVAL: Duration = Duration::from_millis(25);
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_DEFAULT_MAX_ENTRIES: usize = 512;
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_DEFAULT_MAX_BYTES: u64 = 512 * 1024 * 1024;
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_MAX_ENTRIES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_ENTRIES";
#[cfg(feature = "veriloga")]
const VERILOGA_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_BYTES";

/// On-disk Verilog-A cache statistics.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACacheStats {
    /// Cache root directory.
    pub root: PathBuf,
    /// Number of persisted records.
    pub entry_count: usize,
    /// Total persisted bytes.
    pub total_bytes: u64,
    /// Active maximum entry budget.
    pub max_entries: usize,
    /// Active maximum byte budget.
    pub max_bytes: u64,
}

/// A single Verilog-A cache entry from disk.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACacheEntry {
    /// Serialized cache file location.
    pub cache_path: PathBuf,
    /// Canonical source file path.
    pub source_path: PathBuf,
    /// Canonical dependency list used for freshness checks.
    pub dependencies: Vec<PathBuf>,
    /// Serialized file size.
    pub size_bytes: u64,
    /// Record modification timestamp.
    pub modified_ns: Option<u128>,
}

/// Result of a cache prune/clear operation.
#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerilogACachePruneReport {
    /// Number of removed records.
    pub removed_entries: usize,
    /// Total bytes reclaimed.
    pub reclaimed_bytes: u64,
    /// Final cache stats after pruning.
    pub stats: VerilogACacheStats,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerilogADependencyFingerprint {
    canonical_path: PathBuf,
    modified_ns: Option<u128>,
    file_len: u64,
    content_hash: [u8; 32],
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VerilogADiskCacheRecord {
    version: u32,
    source_path: PathBuf,
    dependencies: Vec<VerilogADependencyFingerprint>,
    model: rspice_veriloga::CompiledModel,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct CachedVerilogAModel {
    dependencies: Vec<VerilogADependencyFingerprint>,
    model: rspice_veriloga::CompiledModel,
}

#[cfg(feature = "veriloga")]
fn veriloga_model_cache() -> &'static RwLock<HashMap<PathBuf, CachedVerilogAModel>> {
    static CACHE: OnceLock<RwLock<HashMap<PathBuf, CachedVerilogAModel>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[cfg(all(test, feature = "veriloga"))]
fn veriloga_cache_test_guard() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[cfg(feature = "veriloga")]
fn clear_in_memory_veriloga_cache() {
    if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.clear();
    }
}

#[cfg(feature = "veriloga")]
fn canonicalize_for_cache(path: &Path) -> PathBuf {
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(feature = "veriloga")]
fn normalize_model_key(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[cfg(feature = "veriloga")]
fn metadata_modified_ns(metadata: &std::fs::Metadata) -> Option<u128> {
    use std::time::UNIX_EPOCH;

    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
}

#[cfg(feature = "veriloga")]
fn hash_file(path: &Path) -> std::io::Result<[u8; 32]> {
    let mut file = std::fs::File::open(path)?;
    let mut hasher = blake3::Hasher::new();
    let mut buffer = [0_u8; 64 * 1024];

    loop {
        let read = file.read(&mut buffer)?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    Ok(*hasher.finalize().as_bytes())
}

#[cfg(feature = "veriloga")]
fn dependency_fingerprint(path: &Path) -> Option<VerilogADependencyFingerprint> {
    let canonical_path = canonicalize_for_cache(path);
    let metadata = std::fs::metadata(&canonical_path).ok()?;
    let content_hash = hash_file(&canonical_path).ok()?;
    Some(VerilogADependencyFingerprint {
        canonical_path,
        modified_ns: metadata_modified_ns(&metadata),
        file_len: metadata.len(),
        content_hash,
    })
}

#[cfg(feature = "veriloga")]
fn fingerprint_paths(
    paths: &[PathBuf],
) -> Result<Vec<VerilogADependencyFingerprint>, SimulationError> {
    let mut canonical_paths: Vec<PathBuf> =
        paths.iter().map(|p| canonicalize_for_cache(p)).collect();
    canonical_paths.sort();
    canonical_paths.dedup();

    let mut fingerprints = Vec::with_capacity(canonical_paths.len());
    for canonical_path in canonical_paths {
        let fingerprint = dependency_fingerprint(&canonical_path).ok_or_else(|| {
            SimulationError::Netlist(format!(
                "Verilog-A dependency does not exist or is unreadable: {}",
                canonical_path.display()
            ))
        })?;
        fingerprints.push(fingerprint);
    }

    Ok(fingerprints)
}

#[cfg(feature = "veriloga")]
fn dependency_matches_cached_fingerprint(dep: &VerilogADependencyFingerprint) -> bool {
    let metadata = match std::fs::metadata(&dep.canonical_path) {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    let current_modified_ns = metadata_modified_ns(&metadata);
    if metadata.len() == dep.file_len && current_modified_ns == dep.modified_ns {
        return true;
    }

    match hash_file(&dep.canonical_path) {
        Ok(hash) => hash == dep.content_hash,
        Err(_) => false,
    }
}

#[cfg(feature = "veriloga")]
fn dependencies_are_fresh(dependencies: &[VerilogADependencyFingerprint]) -> bool {
    dependencies
        .iter()
        .all(dependency_matches_cached_fingerprint)
}

#[cfg(feature = "veriloga")]
fn parse_cache_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
fn parse_cache_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
fn veriloga_cache_limits() -> (usize, u64) {
    (
        parse_cache_env_usize(
            VERILOGA_CACHE_MAX_ENTRIES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_ENTRIES,
        ),
        parse_cache_env_u64(
            VERILOGA_CACHE_MAX_BYTES_ENV,
            VERILOGA_CACHE_DEFAULT_MAX_BYTES,
        ),
    )
}

#[cfg(feature = "veriloga")]
fn veriloga_cache_root() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("RSPICE_VERILOGA_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        return cache_dir.join("rspice").join("veriloga");
    }

    std::env::temp_dir().join("rspice-veriloga-cache")
}

#[cfg(feature = "veriloga")]
fn cache_record_path_with_root(source_path: &Path, cache_root: &Path) -> PathBuf {
    let canonical = canonicalize_for_cache(source_path);
    let mut hasher = blake3::Hasher::new();
    hasher.update(canonical.to_string_lossy().as_bytes());
    let key = hasher.finalize().to_hex().to_string();
    cache_root.join(format!("{key}.bin"))
}

#[cfg(feature = "veriloga")]
#[allow(dead_code)]
fn cache_record_path(source_path: &Path) -> PathBuf {
    cache_record_path_with_root(source_path, &veriloga_cache_root())
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
struct VerilogACacheFileInfo {
    path: PathBuf,
    size_bytes: u64,
    modified_ns: Option<u128>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug)]
struct VerilogACacheDiskLock {
    lock_path: PathBuf,
}

#[cfg(feature = "veriloga")]
impl VerilogACacheDiskLock {
    fn acquire(root: &Path) -> Result<Self, String> {
        std::fs::create_dir_all(root).map_err(|e| {
            format!(
                "failed to create cache directory '{}': {}",
                root.display(),
                e
            )
        })?;
        let lock_path = root.join(VERILOGA_CACHE_LOCK_FILE);
        let start = Instant::now();

        loop {
            match std::fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&lock_path)
            {
                Ok(mut file) => {
                    let timestamp_ns = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .ok()
                        .map(|d| d.as_nanos())
                        .unwrap_or(0);
                    let _ = writeln!(
                        file,
                        "pid={} timestamp_ns={}",
                        std::process::id(),
                        timestamp_ns
                    );
                    return Ok(Self { lock_path });
                }
                Err(err) if err.kind() == std::io::ErrorKind::AlreadyExists => {
                    if cache_lock_is_stale(&lock_path) {
                        let _ = std::fs::remove_file(&lock_path);
                        continue;
                    }

                    if start.elapsed() >= VERILOGA_CACHE_LOCK_WAIT_TIMEOUT {
                        return Err(format!(
                            "timed out waiting for Verilog-A cache lock '{}'",
                            lock_path.display()
                        ));
                    }

                    std::thread::sleep(VERILOGA_CACHE_LOCK_POLL_INTERVAL);
                }
                Err(err) => {
                    return Err(format!(
                        "failed to acquire Verilog-A cache lock '{}': {}",
                        lock_path.display(),
                        err
                    ));
                }
            }
        }
    }
}

#[cfg(feature = "veriloga")]
impl Drop for VerilogACacheDiskLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_path);
    }
}

#[cfg(feature = "veriloga")]
fn cache_lock_is_stale(lock_path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(lock_path) else {
        return false;
    };
    let Ok(modified) = metadata.modified() else {
        return false;
    };
    match modified.elapsed() {
        Ok(elapsed) => elapsed > VERILOGA_CACHE_LOCK_STALE_TIMEOUT,
        Err(_) => false,
    }
}

#[cfg(feature = "veriloga")]
fn with_veriloga_cache_disk_lock<T>(
    operation: &str,
    f: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    let root = veriloga_cache_root();
    let _lock =
        VerilogACacheDiskLock::acquire(&root).map_err(|e| format!("{}: {}", operation, e))?;
    f(&root)
}

#[cfg(feature = "veriloga")]
fn list_cache_files(cache_root: &Path) -> Result<Vec<VerilogACacheFileInfo>, String> {
    if !cache_root.exists() {
        return Ok(Vec::new());
    }

    let dir_iter = std::fs::read_dir(cache_root).map_err(|e| {
        format!(
            "failed to list cache directory '{}': {}",
            cache_root.display(),
            e
        )
    })?;
    let mut files = Vec::new();
    for entry in dir_iter {
        let entry = entry.map_err(|e| format!("failed to read cache directory entry: {}", e))?;
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("bin") {
            continue;
        }
        let metadata = entry.metadata().map_err(|e| {
            format!(
                "failed to read cache file metadata '{}': {}",
                path.display(),
                e
            )
        })?;
        if !metadata.is_file() {
            continue;
        }
        files.push(VerilogACacheFileInfo {
            path,
            size_bytes: metadata.len(),
            modified_ns: metadata_modified_ns(&metadata),
        });
    }
    Ok(files)
}

#[cfg(feature = "veriloga")]
fn cache_stats_from_files(
    cache_root: &Path,
    files: &[VerilogACacheFileInfo],
) -> VerilogACacheStats {
    let (max_entries, max_bytes) = veriloga_cache_limits();
    VerilogACacheStats {
        root: cache_root.to_path_buf(),
        entry_count: files.len(),
        total_bytes: files.iter().map(|f| f.size_bytes).sum(),
        max_entries,
        max_bytes,
    }
}

#[cfg(feature = "veriloga")]
fn read_cache_record(path: &Path) -> Result<VerilogADiskCacheRecord, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read cache record '{}': {}", path.display(), e))?;
    bincode::deserialize::<VerilogADiskCacheRecord>(&bytes).map_err(|e| {
        format!(
            "failed to deserialize cache record '{}': {}",
            path.display(),
            e
        )
    })
}

#[cfg(feature = "veriloga")]
fn persist_model_to_disk_locked(
    source_path: &Path,
    entry: &CachedVerilogAModel,
    cache_root: &Path,
) -> Result<(), String> {
    let cache_path = cache_record_path_with_root(source_path, cache_root);
    if let Some(parent) = cache_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("failed to create cache directory: {}", e))?;
    }

    let record = VerilogADiskCacheRecord {
        version: VERILOGA_CACHE_RECORD_VERSION,
        source_path: canonicalize_for_cache(source_path),
        dependencies: entry.dependencies.clone(),
        model: entry.model.clone(),
    };
    let encoded = bincode::serialize(&record)
        .map_err(|e| format!("failed to serialize Verilog-A cache record: {}", e))?;

    let tmp_path = cache_path.with_extension(format!("tmp.{}", std::process::id()));
    std::fs::write(&tmp_path, encoded)
        .map_err(|e| format!("failed to write Verilog-A cache record: {}", e))?;

    if let Err(rename_err) = std::fs::rename(&tmp_path, &cache_path) {
        // std::fs::rename does not replace existing files on Windows.
        if cache_path.exists() {
            std::fs::remove_file(&cache_path).map_err(|e| {
                let _ = std::fs::remove_file(&tmp_path);
                format!(
                    "failed to replace existing Verilog-A cache record '{}': {} (rename error: {})",
                    cache_path.display(),
                    e,
                    rename_err
                )
            })?;
            std::fs::rename(&tmp_path, &cache_path).map_err(|e| {
                let _ = std::fs::remove_file(&tmp_path);
                format!(
                    "failed to finalize Verilog-A cache record '{}' after replacement: {}",
                    cache_path.display(),
                    e
                )
            })?;
        } else {
            let _ = std::fs::remove_file(&tmp_path);
            return Err(format!(
                "failed to finalize Verilog-A cache record '{}': {}",
                cache_path.display(),
                rename_err
            ));
        }
    }

    Ok(())
}

#[cfg(feature = "veriloga")]
fn remove_cache_file(path: &Path) {
    if let Err(err) = std::fs::remove_file(path) {
        if err.kind() != std::io::ErrorKind::NotFound {
            log::warn!(
                "failed to remove stale/corrupt Verilog-A cache file '{}': {}",
                path.display(),
                err
            );
        }
    }
}

#[cfg(feature = "veriloga")]
fn prune_veriloga_cache_locked(cache_root: &Path) -> Result<VerilogACachePruneReport, String> {
    let (max_entries, max_bytes) = veriloga_cache_limits();
    let mut files = list_cache_files(cache_root)?;
    files.sort_by(|a, b| {
        let left = a.modified_ns.unwrap_or(0);
        let right = b.modified_ns.unwrap_or(0);
        left.cmp(&right).then_with(|| a.path.cmp(&b.path))
    });

    let mut entry_count = files.len();
    let mut total_bytes: u64 = files.iter().map(|f| f.size_bytes).sum();
    let mut removed_entries = 0_usize;
    let mut reclaimed_bytes = 0_u64;

    for file in files {
        if entry_count <= max_entries && total_bytes <= max_bytes {
            break;
        }

        match std::fs::remove_file(&file.path) {
            Ok(()) => {
                entry_count = entry_count.saturating_sub(1);
                total_bytes = total_bytes.saturating_sub(file.size_bytes);
                removed_entries += 1;
                reclaimed_bytes = reclaimed_bytes.saturating_add(file.size_bytes);
            }
            Err(err) => {
                log::warn!(
                    "failed to evict Verilog-A cache record '{}': {}",
                    file.path.display(),
                    err
                );
            }
        }
    }

    let stats = VerilogACacheStats {
        root: cache_root.to_path_buf(),
        entry_count,
        total_bytes,
        max_entries,
        max_bytes,
    };

    Ok(VerilogACachePruneReport {
        removed_entries,
        reclaimed_bytes,
        stats,
    })
}

#[cfg(feature = "veriloga")]
fn persist_model_to_disk(source_path: &Path, entry: &CachedVerilogAModel) -> Result<(), String> {
    with_veriloga_cache_disk_lock("persist Verilog-A cache record", |cache_root| {
        persist_model_to_disk_locked(source_path, entry, cache_root)?;
        if let Err(err) = prune_veriloga_cache_locked(cache_root) {
            log::warn!("failed to prune Verilog-A cache after write: {}", err);
        }
        Ok(())
    })
}

#[cfg(feature = "veriloga")]
fn load_model_from_disk_locked(
    source_path: &Path,
    cache_root: &Path,
) -> Result<Option<CachedVerilogAModel>, String> {
    let cache_path = cache_record_path_with_root(source_path, cache_root);
    let record = match read_cache_record(&cache_path) {
        Ok(record) => record,
        Err(err) => {
            if cache_path.exists() {
                log::warn!("{}", err);
                remove_cache_file(&cache_path);
            }
            return Ok(None);
        }
    };

    if record.version != VERILOGA_CACHE_RECORD_VERSION {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    let requested_source = canonicalize_for_cache(source_path);
    let record_source = canonicalize_for_cache(&record.source_path);
    if requested_source != record_source {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    if !dependencies_are_fresh(&record.dependencies) {
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    Ok(Some(CachedVerilogAModel {
        dependencies: record.dependencies,
        model: record.model,
    }))
}

#[cfg(feature = "veriloga")]
fn load_model_from_disk(source_path: &Path) -> Option<CachedVerilogAModel> {
    match with_veriloga_cache_disk_lock("load Verilog-A cache record", |cache_root| {
        load_model_from_disk_locked(source_path, cache_root)
    }) {
        Ok(model) => model,
        Err(err) => {
            log::warn!("{}", err);
            None
        }
    }
}

/// Query on-disk Verilog-A cache statistics.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_stats() -> Result<VerilogACacheStats, String> {
    with_veriloga_cache_disk_lock("inspect Verilog-A cache", |cache_root| {
        let files = list_cache_files(cache_root)?;
        Ok(cache_stats_from_files(cache_root, &files))
    })
}

/// List persisted Verilog-A cache entries including dependency paths.
#[cfg(feature = "veriloga")]
pub fn veriloga_cache_entries() -> Result<Vec<VerilogACacheEntry>, String> {
    with_veriloga_cache_disk_lock("list Verilog-A cache entries", |cache_root| {
        let mut files = list_cache_files(cache_root)?;
        files.sort_by(|a, b| a.path.cmp(&b.path));

        let mut entries = Vec::with_capacity(files.len());
        for file in files {
            let Ok(record) = read_cache_record(&file.path) else {
                remove_cache_file(&file.path);
                continue;
            };
            if record.version != VERILOGA_CACHE_RECORD_VERSION {
                remove_cache_file(&file.path);
                continue;
            }
            entries.push(VerilogACacheEntry {
                cache_path: file.path,
                source_path: canonicalize_for_cache(&record.source_path),
                dependencies: record
                    .dependencies
                    .into_iter()
                    .map(|dep| dep.canonical_path)
                    .collect(),
                size_bytes: file.size_bytes,
                modified_ns: file.modified_ns,
            });
        }

        entries.sort_by(|a, b| a.source_path.cmp(&b.source_path));
        Ok(entries)
    })
}

/// Prune on-disk Verilog-A cache to configured limits.
#[cfg(feature = "veriloga")]
pub fn prune_veriloga_cache() -> Result<VerilogACachePruneReport, String> {
    with_veriloga_cache_disk_lock("prune Verilog-A cache", |cache_root| {
        prune_veriloga_cache_locked(cache_root)
    })
}

/// Clear all on-disk and in-memory Verilog-A cache entries.
#[cfg(feature = "veriloga")]
pub fn clear_veriloga_cache() -> Result<VerilogACachePruneReport, String> {
    with_veriloga_cache_disk_lock("clear Verilog-A cache", |cache_root| {
        let files = list_cache_files(cache_root)?;
        let mut removed_entries = 0_usize;
        let mut reclaimed_bytes = 0_u64;
        for file in files {
            match std::fs::remove_file(&file.path) {
                Ok(()) => {
                    removed_entries += 1;
                    reclaimed_bytes = reclaimed_bytes.saturating_add(file.size_bytes);
                }
                Err(err) => {
                    log::warn!(
                        "failed to remove Verilog-A cache entry '{}': {}",
                        file.path.display(),
                        err
                    );
                }
            }
        }
        clear_in_memory_veriloga_cache();
        let empty_files = list_cache_files(cache_root)?;
        let stats = cache_stats_from_files(cache_root, &empty_files);
        Ok(VerilogACachePruneReport {
            removed_entries,
            reclaimed_bytes,
            stats,
        })
    })
}

#[cfg(feature = "veriloga")]
fn resolve_cached_or_compile_veriloga(
    path: &Path,
) -> Result<rspice_veriloga::CompiledModel, SimulationError> {
    let canonical = canonicalize_for_cache(path);
    let mut stale_in_memory = false;

    if let Ok(cache) = veriloga_model_cache().read() {
        if let Some(entry) = cache.get(&canonical) {
            if dependencies_are_fresh(&entry.dependencies) {
                log::debug!("Verilog-A cache hit (memory): '{}'", canonical.display());
                return Ok(entry.model.clone());
            }
            stale_in_memory = true;
        }
    }

    if stale_in_memory {
        if let Ok(mut cache) = veriloga_model_cache().write() {
            cache.remove(&canonical);
        }
    }

    if let Some(entry) = load_model_from_disk(&canonical) {
        let model = entry.model.clone();
        if let Ok(mut cache) = veriloga_model_cache().write() {
            cache.insert(canonical.clone(), entry);
        }
        log::debug!("Verilog-A cache hit (disk): '{}'", canonical.display());
        return Ok(model);
    }

    log::info!("Verilog-A cache miss, compiling '{}'", canonical.display());
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let compiled = compiler.compile_file_with_metadata(path).map_err(|e| {
        SimulationError::Netlist(format!(
            "Failed to compile Verilog-A '{}': {}",
            path.display(),
            e
        ))
    })?;

    let dependencies = fingerprint_paths(&compiled.dependencies)?;
    let entry = CachedVerilogAModel {
        dependencies,
        model: compiled.model.clone(),
    };

    if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.insert(canonical.clone(), entry.clone());
    }

    if let Err(err) = persist_model_to_disk(&canonical, &entry) {
        log::warn!(
            "Failed to persist Verilog-A cache entry for '{}': {}",
            canonical.display(),
            err
        );
    }

    Ok(compiled.model)
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This allows UI workflows to compile once on import and reuse the compiled
/// artifact during simulation without recompilation.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let canonical_source = canonicalize_for_cache(source_path.as_ref());
    let mut dependency_paths = dependencies.to_vec();
    if dependency_paths.is_empty() {
        dependency_paths.push(canonical_source.clone());
    }
    let dependency_fingerprints = fingerprint_paths(&dependency_paths)
        .map_err(|e| format!("dependency fingerprinting failed: {}", e))?;

    let entry = CachedVerilogAModel {
        dependencies: dependency_fingerprints,
        model,
    };

    let mut cache = veriloga_model_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_string())?;
    cache.insert(canonical_source.clone(), entry.clone());
    drop(cache);

    if let Err(err) = persist_model_to_disk(&canonical_source, &entry) {
        log::warn!(
            "Failed to persist precompiled Verilog-A cache for '{}': {}",
            canonical_source.display(),
            err
        );
    }

    Ok(())
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This compatibility wrapper fingerprints only the source file path.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model(
    source_path: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    let dependency = vec![canonicalize_for_cache(source_path.as_ref())];
    register_precompiled_veriloga_model_with_dependencies(source_path, &dependency, model)
}

#[derive(Debug, Clone, Copy, Default)]
struct TransmissionLineModelParams {
    z0: Option<f64>,
    td: Option<f64>,
    freq: Option<f64>,
    nl: Option<f64>,
    r: Option<f64>,
    g: Option<f64>,
    len: Option<f64>,
    alpha: Option<f64>,
    atten: Option<f64>,
}

fn model_param(params: &[(String, f64)], names: &[&str]) -> Option<f64> {
    params.iter().find_map(|(name, value)| {
        if names
            .iter()
            .any(|candidate| name.eq_ignore_ascii_case(candidate))
        {
            Some(*value)
        } else {
            None
        }
    })
}

fn resolve_tline_model_params(
    netlist: &Netlist,
    model_name: &str,
) -> Option<TransmissionLineModelParams> {
    let model = netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))?;

    let mut params = TransmissionLineModelParams {
        z0: model_param(&model.params, &["Z0", "ZO"]),
        td: model_param(&model.params, &["TD", "TDELAY"]),
        freq: model_param(&model.params, &["F", "FREQ"]),
        nl: model_param(&model.params, &["NL"]),
        r: model_param(&model.params, &["R", "R0"]),
        g: model_param(&model.params, &["G", "G0"]),
        len: model_param(&model.params, &["LEN", "LENGTH"]),
        alpha: model_param(&model.params, &["ALPHA"]),
        atten: model_param(&model.params, &["ATTEN", "ATTENDB", "LOSSDB"]),
    };

    let l = model_param(&model.params, &["L", "L0"]);
    let c = model_param(&model.params, &["C", "C0"]);
    let len = params.len;

    if params.z0.is_none() {
        if let (Some(l), Some(c)) = (l, c) {
            if l > 0.0 && c > 0.0 {
                params.z0 = Some((l / c).sqrt());
            }
        }
    }

    if params.td.is_none() {
        if let (Some(f), Some(nl)) = (params.freq, params.nl) {
            if f > 0.0 {
                params.td = Some(nl / f);
            }
        }
    }

    if params.td.is_none() {
        if let (Some(l), Some(c), Some(len)) = (l, c, len) {
            if l > 0.0 && c > 0.0 && len > 0.0 {
                params.td = Some(len * (l * c).sqrt());
            }
        }
    }

    Some(params)
}

fn tline_model_attenuation(params: TransmissionLineModelParams, z0: f64) -> Option<f64> {
    let len = params.len.unwrap_or(1.0).max(0.0);

    // Explicit alpha (Np/unit length) takes precedence.
    if let Some(alpha) = params.alpha {
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    // ATTEN/ATTENDB: interpret <=1 as linear ratio, otherwise as dB.
    if let Some(atten) = params.atten {
        if atten.is_finite() && atten >= 0.0 {
            if atten <= 1.0 {
                return Some(atten);
            }
            let db_total = if params.len.is_some() {
                atten * len
            } else {
                atten
            };
            return Some(10_f64.powf(-db_total / 20.0));
        }
    }

    // Derive from primary RLGC line loss when available.
    let r = params.r.unwrap_or(0.0).max(0.0);
    let g = params.g.unwrap_or(0.0).max(0.0);
    if (r > 0.0 || g > 0.0) && z0.is_finite() && z0 > 0.0 {
        let alpha = r / (2.0 * z0) + g * z0 / 2.0;
        if alpha.is_finite() && alpha >= 0.0 {
            return Some((-alpha * len).exp());
        }
    }

    None
}

fn resolve_bjt_type_from_model(model_type: &str) -> Option<crate::netlist::BjtType> {
    if model_type.eq_ignore_ascii_case("NPN") {
        Some(crate::netlist::BjtType::Npn)
    } else if model_type.eq_ignore_ascii_case("PNP") {
        Some(crate::netlist::BjtType::Pnp)
    } else {
        None
    }
}

fn resolve_mos_type_from_model(model_type: &str) -> Option<crate::netlist::MosType> {
    if model_type.eq_ignore_ascii_case("NMOS") {
        Some(crate::netlist::MosType::Nmos)
    } else if model_type.eq_ignore_ascii_case("PMOS") {
        Some(crate::netlist::MosType::Pmos)
    } else {
        None
    }
}

fn resolve_jfet_type_from_model(model_type: &str) -> Option<crate::netlist::JfetType> {
    if model_type.eq_ignore_ascii_case("NJF") {
        Some(crate::netlist::JfetType::Njf)
    } else if model_type.eq_ignore_ascii_case("PJF") {
        Some(crate::netlist::JfetType::Pjf)
    } else {
        None
    }
}

fn resolve_mesfet_type_from_model(model_type: &str) -> Option<crate::netlist::MesfetType> {
    if model_type.eq_ignore_ascii_case("NMF") {
        Some(crate::netlist::MesfetType::Nmf)
    } else if model_type.eq_ignore_ascii_case("PMF") {
        Some(crate::netlist::MesfetType::Pmf)
    } else {
        None
    }
}

fn find_model_def<'a>(
    netlist: &'a Netlist,
    model_name: &str,
) -> Option<&'a crate::netlist::ModelDef> {
    netlist
        .models
        .iter()
        .find(|m| m.name.eq_ignore_ascii_case(model_name))
}

fn expected_model_type_text(expected_types: &[&str]) -> String {
    match expected_types {
        [] => String::new(),
        [single] => (*single).to_string(),
        [left, right] => format!("{left} or {right}"),
        _ => expected_types.join(", "),
    }
}

fn ensure_model_type(
    element_kind: &str,
    element_name: &str,
    model_name: &str,
    model_def: &crate::netlist::ModelDef,
    expected_types: &[&str],
) -> Result<(), SimulationError> {
    if expected_types
        .iter()
        .any(|kind| model_def.model_type.eq_ignore_ascii_case(kind))
    {
        return Ok(());
    }

    let expected = expected_model_type_text(expected_types);
    Err(SimulationError::Circuit(format!(
        "{} '{}' references model '{}' with incompatible type '{}'; expected {}",
        element_kind, element_name, model_name, model_def.model_type, expected
    )))
}

fn map_switch_state(state: crate::netlist::SwitchState) -> crate::device::SwitchState {
    match state {
        crate::netlist::SwitchState::On => crate::device::SwitchState::On,
        crate::netlist::SwitchState::Off => crate::device::SwitchState::Off,
    }
}

impl Engine {
    /// Build circuit from netlist (flattens subcircuits first)
    pub fn build_circuit(&self, netlist: &Netlist) -> Result<CircuitData, SimulationError> {
        let mut circuit = CircuitData::new();

        // Flatten subcircuit instances into top-level elements
        let flat_elements = flatten_netlist(netlist)
            .map_err(|e| SimulationError::Netlist(format!("Flattening error: {}", e)))?;

        // Debug: log all elements
        log::info!("Building circuit with {} elements:", flat_elements.len());
        for element in &flat_elements {
            log::info!(
                "  Element: {} nodes={:?} kind={:?}",
                element.name,
                element.nodes,
                element.kind
            );
        }

        #[cfg(feature = "veriloga")]
        let mut veriloga_models: HashMap<String, rspice_veriloga::CompiledModel> = HashMap::new();

        // Load and cache Verilog-A models referenced by .VERILOGA directives.
        #[cfg(feature = "veriloga")]
        {
            for include in &netlist.veriloga_includes {
                let model = resolve_cached_or_compile_veriloga(&include.file_path)?;

                let model_key = normalize_model_key(model.name.as_str());
                veriloga_models
                    .entry(model_key)
                    .or_insert_with(|| model.clone());

                if let Some(alias) = include.model_name.as_deref() {
                    veriloga_models
                        .entry(normalize_model_key(alias))
                        .or_insert_with(|| model.clone());
                }

                if let Some(stem) = include.file_path.file_stem().and_then(|s| s.to_str()) {
                    veriloga_models
                        .entry(normalize_model_key(stem))
                        .or_insert_with(|| model.clone());
                }

                log::info!(
                    "Loaded Verilog-A model '{}' from {}",
                    model.name,
                    include.file_path.display()
                );
            }
        }

        for element in &flat_elements {
            match &element.kind {
                ElementKind::Resistor { value } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.resistors.add(element.name.clone(), np, nn, *value);
                }
                ElementKind::Capacitor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    circuit.capacitors.add(element.name.clone(), np, nn, *value);
                }
                ElementKind::Inductor { value, .. } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    circuit
                        .inductors
                        .add(element.name.clone(), np, nn, branch, *value);
                }
                ElementKind::JilesAthertonInductor {
                    value: _,
                    model,
                    initial_current: _,
                } => {
                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Jiles-Atherton inductor '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Jiles-Atherton inductor",
                        &element.name,
                        model,
                        model_def,
                        &["CORE", "JA", "JILES", "JILESATHERTON"],
                    )?;
                    return Err(SimulationError::Circuit(format!(
                        "Jiles-Atherton inductor '{}' is not yet supported by the runtime solver (model '{}')",
                        element.name, model
                    )));
                }
                ElementKind::VoltageSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let dc_value = extract_dc_value(spec);
                    let (ac_mag, ac_phase) = super::extract_ac_value(spec);
                    log::debug!(
                        "VoltageSource {}: DC={}, AC_mag={}, AC_phase={}, spec={:?}",
                        element.name,
                        dc_value,
                        ac_mag,
                        ac_phase,
                        spec
                    );
                    // Clone spec for transient analysis if it's a time-varying source
                    let transient_spec = match spec {
                        crate::netlist::SourceSpec::Pulse { .. }
                        | crate::netlist::SourceSpec::Sin { .. }
                        | crate::netlist::SourceSpec::Pwl { .. }
                        | crate::netlist::SourceSpec::PwlFile { .. }
                        | crate::netlist::SourceSpec::Exp { .. } => Some(spec.clone()),
                        _ => None,
                    };
                    circuit.voltage_sources.add_with_ac_and_spec(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        dc_value,
                        ac_mag,
                        ac_phase,
                        transient_spec,
                    );
                }
                ElementKind::CurrentSource(spec) => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let dc_value = extract_dc_value(spec);
                    let (ac_mag, ac_phase) = super::extract_ac_value(spec);
                    circuit.current_sources.add_with_ac(
                        element.name.clone(),
                        np,
                        nn,
                        dc_value,
                        ac_mag,
                        ac_phase,
                    );
                }
                ElementKind::Diode { model } => {
                    let anode = circuit.get_or_create_node(&element.nodes[0]);
                    let cathode = circuit.get_or_create_node(&element.nodes[1]);
                    let mut diode = crate::device::Diode::new(element.name.clone(), anode, cathode);

                    // Look up model and apply parameters
                    let model_def = find_model_def(netlist, model);
                    if let Some(device_model) = model_def {
                        ensure_model_type(
                            "Diode",
                            &element.name,
                            model,
                            device_model,
                            &["D", "DIODE"],
                        )?;
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        diode = diode.with_model_params(&params_map);
                    }

                    circuit.diodes.add(diode);
                }
                ElementKind::Bjt { model, bjt_type } => {
                    let collector = circuit.get_or_create_node(&element.nodes[0]);
                    let base = circuit.get_or_create_node(&element.nodes[1]);
                    let emitter = circuit.get_or_create_node(&element.nodes[2]);

                    // Resolve polarity from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_bjt_type = if let Some(device_model) = model_def {
                        resolve_bjt_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "BJT '{}' references model '{}' with incompatible type '{}'; expected NPN or PNP",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *bjt_type
                    };

                    let mut bjt = match resolved_bjt_type {
                        crate::netlist::BjtType::Npn => crate::device::Bjt::new_npn(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                        crate::netlist::BjtType::Pnp => crate::device::Bjt::new_pnp(
                            element.name.clone(),
                            collector,
                            base,
                            emitter,
                        ),
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        // Convert Vec<(String, f64)> to HashMap for with_params
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        bjt = bjt.with_params(&params_map);
                    } else if let Some(params_map) =
                        builtin_bjt_model_map().get(&model.to_uppercase())
                    {
                        // Fallback to embedded transistor library models when no
                        // explicit .MODEL card is present in the parsed netlist.
                        bjt = bjt.with_params(params_map);
                        log::debug!(
                            "Applied embedded BJT fallback model '{}' to {}",
                            model,
                            element.name
                        );
                    }

                    circuit.bjts.add(bjt);
                }
                ElementKind::Mosfet { model, mos_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    let bulk = circuit.get_or_create_node(&element.nodes[3]);

                    // Resolve NMOS/PMOS from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_mos_type = if let Some(device_model) = model_def {
                        resolve_mos_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MOSFET '{}' references model '{}' with incompatible type '{}'; expected NMOS or PMOS",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *mos_type
                    };

                    let mut mosfet = match resolved_mos_type {
                        crate::netlist::MosType::Nmos => crate::device::Mosfet::new_nmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                        crate::netlist::MosType::Pmos => crate::device::Mosfet::new_pmos(
                            element.name.clone(),
                            drain,
                            gate,
                            source,
                            bulk,
                        ),
                    };

                    // Look up model and apply parameters including LEVEL
                    if let Some(device_model) = model_def {
                        // Convert Vec<(String, f64)> to HashMap for with_params
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();

                        // Extract LEVEL from params (default to 1)
                        let level = params_map.get("LEVEL").copied().unwrap_or(1.0) as i32;
                        mosfet = mosfet.with_level(level);

                        // Apply all model parameters (VTO, KP, GAMMA, KC, NC, etc.)
                        mosfet = mosfet.with_params(&params_map);
                    }

                    circuit.mosfets.add(mosfet);
                }
                ElementKind::Jfet { model, jfet_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);

                    // Resolve NJF/PJF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_jfet_type = if let Some(device_model) = model_def {
                        resolve_jfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "JFET '{}' references model '{}' with incompatible type '{}'; expected NJF or PJF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *jfet_type
                    };

                    let mut jfet = match resolved_jfet_type {
                        crate::netlist::JfetType::Njf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::JfetType::Pjf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        jfet = jfet.with_model_params(&params_map);
                    }

                    // Realistic extrinsic JFET series resistances (RD/RS) are modeled by
                    // inserting explicit linear resistors and connecting the intrinsic JFET
                    // to generated internal drain/source nodes.
                    let rd = if jfet.params.rd.is_finite() && jfet.params.rd > 0.0 {
                        jfet.params.rd
                    } else {
                        0.0
                    };
                    let rs = if jfet.params.rs.is_finite() && jfet.params.rs > 0.0 {
                        jfet.params.rs
                    } else {
                        0.0
                    };

                    if rd > 0.0 {
                        let dint_name = format!("{}.__dint", element.name);
                        let dint = circuit.get_or_create_node(&dint_name);
                        let rd_name = format!("{}.__rd", element.name);
                        circuit.resistors.add(rd_name, drain, dint, rd);
                        jfet.drain = dint;
                        jfet.params.rd = 0.0;
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                    }

                    circuit.jfets.push(jfet);
                }
                // MESFET (GaAs FET) - treat as JFET for now since physics are similar
                ElementKind::Mesfet { model, mesfet_type } => {
                    let drain = circuit.get_or_create_node(&element.nodes[0]);
                    let gate = circuit.get_or_create_node(&element.nodes[1]);
                    let source = circuit.get_or_create_node(&element.nodes[2]);
                    // MESFET uses similar equations to JFET - treat as N-channel JFET

                    // Resolve NMF/PMF from model card when available.
                    let model_def = find_model_def(netlist, model);
                    let resolved_mesfet_type = if let Some(device_model) = model_def {
                        resolve_mesfet_type_from_model(&device_model.model_type).ok_or_else(|| {
                            SimulationError::Circuit(format!(
                                "MESFET '{}' references model '{}' with incompatible type '{}'; expected NMF or PMF",
                                element.name, model, device_model.model_type
                            ))
                        })?
                    } else {
                        *mesfet_type
                    };

                    let mut jfet = match resolved_mesfet_type {
                        crate::netlist::MesfetType::Nmf => {
                            crate::device::Jfet::njf(&element.name, drain, gate, source)
                        }
                        crate::netlist::MesfetType::Pmf => {
                            crate::device::Jfet::pjf(&element.name, drain, gate, source)
                        }
                    };

                    // Look up model and apply parameters
                    if let Some(device_model) = model_def {
                        let params_map: std::collections::HashMap<String, f64> =
                            device_model.params.iter().cloned().collect();
                        jfet = jfet.with_model_params(&params_map);
                    }

                    // Apply the same RD/RS extrinsic-node expansion for MESFET aliases.
                    let rd = if jfet.params.rd.is_finite() && jfet.params.rd > 0.0 {
                        jfet.params.rd
                    } else {
                        0.0
                    };
                    let rs = if jfet.params.rs.is_finite() && jfet.params.rs > 0.0 {
                        jfet.params.rs
                    } else {
                        0.0
                    };

                    if rd > 0.0 {
                        let dint_name = format!("{}.__dint", element.name);
                        let dint = circuit.get_or_create_node(&dint_name);
                        let rd_name = format!("{}.__rd", element.name);
                        circuit.resistors.add(rd_name, drain, dint, rd);
                        jfet.drain = dint;
                        jfet.params.rd = 0.0;
                    }
                    if rs > 0.0 {
                        let sint_name = format!("{}.__sint", element.name);
                        let sint = circuit.get_or_create_node(&sint_name);
                        let rs_name = format!("{}.__rs", element.name);
                        circuit.resistors.add(rs_name, source, sint, rs);
                        jfet.source = sint;
                        jfet.params.rs = 0.0;
                    }

                    circuit.jfets.push(jfet);
                }
                // Controlled sources
                ElementKind::Vcvs {
                    gain,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    let branch = circuit.allocate_branch();
                    circuit
                        .vcvs
                        .add(element.name.clone(), np, nn, cp, cn, branch, *gain);
                }
                ElementKind::Vccs {
                    transconductance,
                    control_nodes,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(&control_nodes.0);
                    let cn = circuit.get_or_create_node(&control_nodes.1);
                    circuit
                        .vccs
                        .add(element.name.clone(), np, nn, cp, cn, *transconductance);
                }
                ElementKind::Cccs {
                    gain,
                    control_element,
                } => {
                    // CCCS needs the branch of a controlling voltage source
                    // Register for deferred resolution after all elements are added
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cccs_idx = circuit.cccs.len();
                    // Add with placeholder branch (will be resolved later)
                    circuit.cccs.add(element.name.clone(), np, nn, 0, *gain);
                    circuit.add_cccs_pending(cccs_idx, control_element.clone());
                }
                ElementKind::Ccvs {
                    transresistance,
                    control_element,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);
                    let ccvs_idx = circuit.ccvs.len();
                    // Add with placeholder control branch (will be resolved later)
                    circuit
                        .ccvs
                        .add(element.name.clone(), np, nn, branch, 0, *transresistance);
                    circuit.add_ccvs_pending(ccvs_idx, control_element.clone());
                }
                // Behavioral sources
                ElementKind::BehavioralVoltage { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let branch = circuit.allocate_branch_named(&element.name);

                    let bvs = crate::device::BehavioralVoltageSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        branch,
                        expression,
                    );
                    circuit.behavioral_sources.add_voltage(bvs);
                }
                ElementKind::BehavioralCurrent { expression } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let bcs = crate::device::BehavioralCurrentSource::new(
                        element.name.clone(),
                        np,
                        nn,
                        expression,
                    );
                    circuit.behavioral_sources.add_current(bcs);
                }
                // Flattened tree leaves external subcircuit-backed devices here
                // (for example, Verilog-A model instances).
                #[cfg(feature = "veriloga")]
                ElementKind::Subcircuit {
                    subckt_name,
                    params,
                } => {
                    if let Some(model) = veriloga_models.get(&normalize_model_key(subckt_name)) {
                        if element.nodes.len() != model.num_terminals {
                            return Err(SimulationError::Circuit(format!(
                                "Verilog-A instance '{}' expects {} terminals for model '{}', found {}",
                                element.name,
                                model.num_terminals,
                                subckt_name,
                                element.nodes.len()
                            )));
                        }

                        let mut node_ids = Vec::with_capacity(model.num_terminals);
                        for node_name in &element.nodes {
                            node_ids.push(if node_name.eq_ignore_ascii_case("0") {
                                0
                            } else {
                                circuit.get_or_create_node(node_name)
                            });
                        }

                        let mut device = crate::device::veriloga::VerilogADevice::new(
                            element.name.clone(),
                            model.clone(),
                            &node_ids,
                        );

                        // Allocate global circuit node indices for internal Verilog-A nodes.
                        if device.num_internal_nodes() > 0 {
                            let mut internal_nodes =
                                Vec::with_capacity(device.num_internal_nodes());
                            for idx in 0..device.num_internal_nodes() {
                                let node_name = format!("{}.__int{}", element.name, idx + 1);
                                internal_nodes.push(circuit.get_or_create_node(&node_name));
                            }
                            device.set_internal_node_indices(&internal_nodes);
                        }

                        for (name, value) in params {
                            let _ = device.set_parameter(name, *value);
                        }
                        device.set_temperature(self.config.temperature);
                        circuit.veriloga_devices.add(device);
                        continue;
                    }

                    return Err(SimulationError::Circuit(format!(
                        "Unresolved subcircuit instance '{}' referencing '{}'",
                        element.name, subckt_name
                    )));
                }
                #[cfg(not(feature = "veriloga"))]
                ElementKind::Subcircuit { subckt_name, .. } => {
                    return Err(SimulationError::Circuit(format!(
                        "Unresolved subcircuit instance '{}' referencing '{}'",
                        element.name, subckt_name
                    )));
                }

                // New element types
                ElementKind::VSwitch {
                    control_pos,
                    control_neg,
                    model,
                    initial_state,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);
                    let cp = circuit.get_or_create_node(control_pos);
                    let cn = circuit.get_or_create_node(control_neg);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Voltage-controlled switch '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Voltage-controlled switch",
                        &element.name,
                        model,
                        model_def,
                        &["SW", "VSWITCH", "VSW"],
                    )?;
                    let params_map: std::collections::HashMap<String, f64> =
                        model_def.params.iter().cloned().collect();

                    let mut sw = crate::device::VoltageSwitch::new(
                        element.name.clone(),
                        np,
                        nn, // Switch terminals
                        cp,
                        cn, // Control terminals
                    )
                    .with_params(&params_map);
                    if let Some(state) = initial_state {
                        sw = sw.with_initial_state(map_switch_state(*state));
                    }
                    circuit.vswitches.push(sw);
                }
                ElementKind::ISwitch {
                    control_element,
                    model,
                    initial_state,
                } => {
                    let np = circuit.get_or_create_node(&element.nodes[0]);
                    let nn = circuit.get_or_create_node(&element.nodes[1]);

                    let model_def = find_model_def(netlist, model).ok_or_else(|| {
                        SimulationError::Circuit(format!(
                            "Current-controlled switch '{}' references unknown model '{}'",
                            element.name, model
                        ))
                    })?;
                    ensure_model_type(
                        "Current-controlled switch",
                        &element.name,
                        model,
                        model_def,
                        &["CSW", "ISWITCH", "ISW"],
                    )?;
                    let params_map: std::collections::HashMap<String, f64> =
                        model_def.params.iter().cloned().collect();

                    let mut sw = crate::device::CurrentSwitch::new(
                        element.name.clone(),
                        np,
                        nn,
                        control_element.clone(), // Control source name
                    )
                    .with_params(&params_map);
                    if let Some(state) = initial_state {
                        sw = sw.with_initial_state(map_switch_state(*state));
                    }
                    let iswitch_idx = circuit.iswitches.len();
                    circuit.iswitches.push(sw);
                    circuit.add_iswitch_pending(iswitch_idx, control_element.clone());
                }
                ElementKind::TransmissionLine {
                    z0,
                    td,
                    freq,
                    nl,
                    model,
                } => {
                    if element.nodes.len() > 4 {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has {} nodes; coupled/multiconductor P-lines are not yet supported",
                            element.name,
                            element.nodes.len()
                        )));
                    }
                    if element.nodes.len() < 4 {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' requires 4 nodes",
                            element.name
                        )));
                    }

                    let p1p = circuit.get_or_create_node(&element.nodes[0]);
                    let p1n = circuit.get_or_create_node(&element.nodes[1]);
                    let p2p = circuit.get_or_create_node(&element.nodes[2]);
                    let p2n = circuit.get_or_create_node(&element.nodes[3]);

                    if let (Some(model_name), Some(model_def)) = (
                        model.as_deref(),
                        model
                            .as_deref()
                            .and_then(|name| find_model_def(netlist, name)),
                    ) {
                        ensure_model_type(
                            "Transmission line",
                            &element.name,
                            model_name,
                            model_def,
                            &["LTRA", "TXL"],
                        )?;
                    }

                    let model_params = model
                        .as_deref()
                        .and_then(|name| resolve_tline_model_params(netlist, name));

                    if model.is_some() && model_params.is_none() && z0.is_none() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' references unknown model '{}'",
                            element.name,
                            model.as_deref().unwrap_or_default()
                        )));
                    }

                    let freq_eff = (*freq).or(model_params.and_then(|m| m.freq));
                    let nl_eff = (*nl).or(model_params.and_then(|m| m.nl));

                    let delay = (*td)
                        .or_else(|| {
                            if let (Some(f), Some(n)) = (freq_eff, nl_eff) {
                                if f > 0.0 { Some(n / f) } else { None }
                            } else {
                                None
                            }
                        })
                        .or(model_params.and_then(|m| m.td))
                        .unwrap_or(1e-9);

                    let z0_eff = (*z0).or(model_params.and_then(|m| m.z0)).unwrap_or(50.0);
                    if z0_eff <= 0.0 || !z0_eff.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has invalid Z0={}",
                            element.name, z0_eff
                        )));
                    }
                    if delay <= 0.0 || !delay.is_finite() {
                        return Err(SimulationError::Circuit(format!(
                            "Transmission line '{}' has invalid TD={}",
                            element.name, delay
                        )));
                    }

                    let mut tline = crate::device::TransmissionLine::new(
                        element.name.clone(),
                        p1p,
                        p1n,
                        p2p,
                        p2n,
                        z0_eff,
                        delay,
                    );
                    tline.freq = freq_eff;
                    tline.nl = nl_eff;
                    if let Some(att) = model_params.and_then(|p| tline_model_attenuation(p, z0_eff))
                    {
                        tline.set_attenuation(att);
                    }
                    circuit.tlines.push(tline);
                }
                ElementKind::Coupling {
                    inductors,
                    coefficient,
                } => {
                    // Store coupling for later resolution
                    circuit.couplings.push(crate::device::InductorCoupling::new(
                        element.name.clone(),
                        inductors.clone(),
                        *coefficient,
                    ));
                }

                // XSPICE code model instances
                ElementKind::Xspice {
                    model,
                    ports,
                    params,
                } => {
                    // Convert parsed XspicePort to PortConnection with resolved node IDs
                    let mut connections: Vec<crate::xspice::PortConnection> = Vec::new();
                    for port in ports {
                        let connection = match port {
                            crate::netlist::XspicePort::Analog(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Analog(node)
                            }
                            crate::netlist::XspicePort::Digital(name) => {
                                let node = if name.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(name)
                                };
                                crate::xspice::PortConnection::Digital(node)
                            }
                            crate::netlist::XspicePort::AnalogVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::AnalogVector(nodes)
                            }
                            crate::netlist::XspicePort::DigitalVector(names) => {
                                let nodes: Vec<usize> = names
                                    .iter()
                                    .map(|n| {
                                        if n.eq_ignore_ascii_case("0") {
                                            0
                                        } else {
                                            circuit.get_or_create_node(n)
                                        }
                                    })
                                    .collect();
                                crate::xspice::PortConnection::DigitalVector(nodes)
                            }
                            crate::netlist::XspicePort::DifferentialVoltage { pos, neg }
                            | crate::netlist::XspicePort::DifferentialCurrent { pos, neg } => {
                                let pos_node = if pos.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(pos)
                                };
                                let neg_node = if neg.eq_ignore_ascii_case("0") {
                                    0
                                } else {
                                    circuit.get_or_create_node(neg)
                                };
                                crate::xspice::PortConnection::Differential(pos_node, neg_node)
                            }
                            crate::netlist::XspicePort::Null => crate::xspice::PortConnection::Null,
                        };
                        connections.push(connection);
                    }

                    // Look up the model in the registry and create instance
                    if let Some(code_model) = circuit.xspice_registry.get(model) {
                        match crate::xspice::XspiceInstance::new(
                            element.name.clone(),
                            code_model.clone(),
                            connections,
                            params,
                        ) {
                            Ok(instance) => {
                                circuit.xspice_instances.push(instance);
                                log::debug!(
                                    "Created XSPICE instance {}: model={}, ports={}",
                                    element.name,
                                    model,
                                    ports.len()
                                );
                            }
                            Err(e) => {
                                log::warn!(
                                    "Failed to create XSPICE instance {}: {}",
                                    element.name,
                                    e
                                );
                            }
                        }
                    } else {
                        log::warn!(
                            "Unknown XSPICE model '{}' for element {}",
                            model,
                            element.name
                        );
                    }
                }
            }
        }

        // Ensure ground reference exists
        // If no node "0" was specified, auto-select a reference node
        circuit.ensure_ground_reference();

        // Resolve all pending control element references after final node count
        // is established (required for current-controlled switch branch indexing).
        circuit
            .resolve_control_elements()
            .map_err(|e| SimulationError::Circuit(e.to_string()))?;

        Ok(circuit)
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod veriloga_cache_tests {
    use super::*;
    use std::fs;
    use std::sync::MutexGuard;
    use std::thread;
    use std::time::{Duration, SystemTime, UNIX_EPOCH};

    fn create_temp_dir(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time should be monotonic")
            .as_nanos();
        let dir = std::env::temp_dir().join(format!(
            "rspice_core_va_cache_{}_{}_{}",
            label,
            std::process::id(),
            nanos
        ));
        fs::create_dir_all(&dir).expect("failed to create temp directory");
        dir
    }

    fn dummy_model_named(name: &str) -> rspice_veriloga::CompiledModel {
        rspice_veriloga::CompiledModel {
            name: name.into(),
            num_terminals: 2,
            terminal_names: vec!["p".into(), "n".into()],
            parameters: vec![rspice_veriloga::codegen::CompiledParameter {
                name: "gain".into(),
                default: 1.0,
                min: Some(0.0),
                max: None,
            }],
            num_variables: 0,
            assignment_programs: Vec::new(),
            stamp_programs: Vec::new(),
            lookup_tables: Vec::new(),
            internal_nodes: 0,
            branch_currents: 0,
            laplace_filters: Vec::new(),
        }
    }

    fn dummy_model() -> rspice_veriloga::CompiledModel {
        dummy_model_named("dummy")
    }

    fn cache_test_lock() -> MutexGuard<'static, ()> {
        veriloga_cache_test_guard()
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn with_cache_env(label: &str, test_fn: impl FnOnce(PathBuf)) {
        let _lock = cache_test_lock();
        let dir = create_temp_dir(label);
        let dir_text = dir.to_string_lossy().to_string();

        // SAFETY: Tests serialize all environment access through cache_test_lock().
        unsafe {
            std::env::set_var("RSPICE_VERILOGA_CACHE_DIR", &dir_text);
            std::env::remove_var(VERILOGA_CACHE_MAX_ENTRIES_ENV);
            std::env::remove_var(VERILOGA_CACHE_MAX_BYTES_ENV);
        }
        clear_in_memory_veriloga_cache();
        test_fn(dir.clone());
        clear_in_memory_veriloga_cache();
        // SAFETY: Tests serialize all environment access through cache_test_lock().
        unsafe {
            std::env::remove_var("RSPICE_VERILOGA_CACHE_DIR");
            std::env::remove_var(VERILOGA_CACHE_MAX_ENTRIES_ENV);
            std::env::remove_var(VERILOGA_CACHE_MAX_BYTES_ENV);
        }
        let _ = fs::remove_dir_all(dir);
    }

    fn write_source(dir: &Path, name: &str, content: &str) -> PathBuf {
        let path = dir.join(name);
        fs::write(&path, content).expect("failed to write source file");
        path
    }

    fn make_cache_entry(paths: &[PathBuf], model_name: &str) -> CachedVerilogAModel {
        let dependencies = fingerprint_paths(paths).expect("expected dependency fingerprints");
        CachedVerilogAModel {
            dependencies,
            model: dummy_model_named(model_name),
        }
    }

    #[test]
    fn test_dependency_fingerprint_invalidates_after_file_change() {
        with_cache_env("invalidates", |dir| {
            let file = write_source(&dir, "model.va", "module m; endmodule\n");
            let fingerprint =
                dependency_fingerprint(&file).expect("initial dependency fingerprint expected");
            assert!(dependency_matches_cached_fingerprint(&fingerprint));

            fs::write(&file, "module m; parameter real x=1; endmodule\n")
                .expect("failed to update model file");
            assert!(!dependency_matches_cached_fingerprint(&fingerprint));
        });
    }

    #[test]
    fn test_fingerprint_paths_deduplicates_same_file() {
        with_cache_env("dedup", |dir| {
            let file = write_source(&dir, "model.va", "module m; endmodule\n");

            let canonical = file.canonicalize().expect("canonical path expected");
            let fingerprints = fingerprint_paths(&[file.clone(), canonical.clone()])
                .expect("fingerprints should succeed");
            assert_eq!(fingerprints.len(), 1);
            assert_eq!(fingerprints[0].canonical_path, canonical);
        });
    }

    #[test]
    fn test_cache_record_serialization_roundtrip() {
        with_cache_env("serde", |dir| {
            let file = write_source(&dir, "model.va", "module m; endmodule\n");
            let dep = dependency_fingerprint(&file).expect("dependency fingerprint expected");

            let record = VerilogADiskCacheRecord {
                version: VERILOGA_CACHE_RECORD_VERSION,
                source_path: file.canonicalize().expect("canonical path expected"),
                dependencies: vec![dep],
                model: dummy_model(),
            };

            let encoded =
                bincode::serialize(&record).expect("cache record should serialize successfully");
            let decoded: VerilogADiskCacheRecord =
                bincode::deserialize(&encoded).expect("cache record should deserialize");

            assert_eq!(decoded.version, VERILOGA_CACHE_RECORD_VERSION);
            assert_eq!(decoded.model.name.as_str(), "dummy");
            assert_eq!(decoded.dependencies.len(), 1);
        });
    }

    #[test]
    fn test_persist_and_load_model_roundtrip() {
        with_cache_env("persist_roundtrip", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let entry = make_cache_entry(std::slice::from_ref(&source), "roundtrip");

            persist_model_to_disk(&source, &entry).expect("cache persist should succeed");
            let loaded =
                load_model_from_disk(&source).expect("expected roundtrip cache entry to load");
            assert_eq!(loaded.model.name.as_str(), "roundtrip");
            assert_eq!(loaded.dependencies.len(), 1);
        });
    }

    #[test]
    fn test_persist_replaces_existing_cache_record() {
        with_cache_env("replace_record", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let first = make_cache_entry(std::slice::from_ref(&source), "v1");
            let second = make_cache_entry(std::slice::from_ref(&source), "v2");

            persist_model_to_disk(&source, &first).expect("first persist should succeed");
            persist_model_to_disk(&source, &second).expect("second persist should overwrite");

            let loaded = load_model_from_disk(&source).expect("expected overwritten cache entry");
            assert_eq!(loaded.model.name.as_str(), "v2");
        });
    }

    #[test]
    fn test_load_model_from_disk_removes_corrupt_record() {
        with_cache_env("corrupt_record", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let cache_path = cache_record_path(&source);
            fs::write(&cache_path, b"not-valid-bincode").expect("failed to write corrupt cache");

            assert!(load_model_from_disk(&source).is_none());
            assert!(!cache_path.exists(), "corrupt cache file should be removed");
        });
    }

    #[test]
    fn test_load_model_from_disk_removes_stale_dependency_record() {
        with_cache_env("stale_dependency", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let include = write_source(&dir, "defs.vams", "`define GAIN 1\n");
            let entry = make_cache_entry(&[source.clone(), include.clone()], "stale");
            persist_model_to_disk(&source, &entry).expect("persist should succeed");

            fs::write(&include, "`define GAIN 2\n").expect("failed to mutate dependency");
            let cache_path = cache_record_path(&source);
            assert!(load_model_from_disk(&source).is_none());
            assert!(
                !cache_path.exists(),
                "stale cache file should be removed after invalidation"
            );
        });
    }

    #[test]
    fn test_veriloga_cache_entries_reports_dependencies() {
        with_cache_env("list_entries", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let include = write_source(&dir, "defs.vams", "`define X 1\n");
            let entry = make_cache_entry(&[source.clone(), include.clone()], "entry_list");
            persist_model_to_disk(&source, &entry).expect("persist should succeed");

            let entries = veriloga_cache_entries().expect("cache entries should load");
            assert_eq!(entries.len(), 1);
            assert_eq!(entries[0].source_path, canonicalize_for_cache(&source));
            assert_eq!(entries[0].dependencies.len(), 2);
            assert!(
                entries[0]
                    .dependencies
                    .iter()
                    .any(|p| *p == canonicalize_for_cache(&include))
            );
        });
    }

    #[test]
    fn test_veriloga_cache_stats_reports_limits() {
        with_cache_env("stats_limits", |dir| {
            // SAFETY: Tests serialize all environment access through cache_test_lock().
            unsafe {
                std::env::set_var(VERILOGA_CACHE_MAX_ENTRIES_ENV, "7");
                std::env::set_var(VERILOGA_CACHE_MAX_BYTES_ENV, "1234");
            }

            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let entry = make_cache_entry(std::slice::from_ref(&source), "stats");
            persist_model_to_disk(&source, &entry).expect("persist should succeed");

            let stats = veriloga_cache_stats().expect("stats should succeed");
            assert_eq!(stats.max_entries, 7);
            assert_eq!(stats.max_bytes, 1234);
            assert_eq!(stats.entry_count, 1);
            assert!(stats.total_bytes > 0);
        });
    }

    #[test]
    fn test_prune_veriloga_cache_enforces_entry_limit() {
        with_cache_env("prune_entries", |dir| {
            // SAFETY: Tests serialize all environment access through cache_test_lock().
            unsafe {
                std::env::set_var(VERILOGA_CACHE_MAX_ENTRIES_ENV, "2");
                std::env::set_var(VERILOGA_CACHE_MAX_BYTES_ENV, "10485760");
            }

            let source_a = write_source(&dir, "a.va", "module a; endmodule\n");
            let source_b = write_source(&dir, "b.va", "module b; endmodule\n");
            let source_c = write_source(&dir, "c.va", "module c; endmodule\n");

            persist_model_to_disk(
                &source_a,
                &make_cache_entry(std::slice::from_ref(&source_a), "a"),
            )
            .expect("persist a");
            thread::sleep(Duration::from_millis(10));
            persist_model_to_disk(
                &source_b,
                &make_cache_entry(std::slice::from_ref(&source_b), "b"),
            )
            .expect("persist b");
            thread::sleep(Duration::from_millis(10));
            persist_model_to_disk(
                &source_c,
                &make_cache_entry(std::slice::from_ref(&source_c), "c"),
            )
            .expect("persist c");

            let report = prune_veriloga_cache().expect("prune should succeed");
            assert!(report.stats.entry_count <= 2);
            assert!(
                !cache_record_path(&source_a).exists(),
                "oldest entry should be evicted first"
            );
        });
    }

    #[test]
    fn test_prune_veriloga_cache_enforces_byte_limit() {
        with_cache_env("prune_bytes", |dir| {
            // SAFETY: Tests serialize all environment access through cache_test_lock().
            unsafe {
                std::env::set_var(VERILOGA_CACHE_MAX_ENTRIES_ENV, "128");
                std::env::set_var(VERILOGA_CACHE_MAX_BYTES_ENV, "1");
            }

            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let entry = make_cache_entry(std::slice::from_ref(&source), "bytes");
            persist_model_to_disk(&source, &entry).expect("persist should succeed");

            let report = prune_veriloga_cache().expect("prune should succeed");
            assert_eq!(report.stats.entry_count, 0);
            assert!(
                !cache_record_path(&source).exists(),
                "byte-limited cache should evict persisted record"
            );
        });
    }

    #[test]
    fn test_clear_veriloga_cache_removes_disk_and_memory_entries() {
        with_cache_env("clear_cache", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            register_precompiled_veriloga_model(&source, dummy_model_named("clear_test"))
                .expect("registration should succeed");
            assert!(
                !veriloga_model_cache().read().expect("read lock").is_empty(),
                "in-memory cache should contain entry before clear"
            );

            let report = clear_veriloga_cache().expect("clear should succeed");
            assert_eq!(report.stats.entry_count, 0);
            assert!(
                veriloga_model_cache().read().expect("read lock").is_empty(),
                "in-memory cache should be cleared"
            );
            assert!(report.removed_entries >= 1);
        });
    }

    #[test]
    fn test_cache_lock_is_released_after_operation() {
        with_cache_env("lock_release", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let entry = make_cache_entry(std::slice::from_ref(&source), "lock_release");
            persist_model_to_disk(&source, &entry).expect("persist should succeed");

            let lock_path = dir.join(VERILOGA_CACHE_LOCK_FILE);
            assert!(
                !lock_path.exists(),
                "cache lock file should not remain after operation"
            );
        });
    }

    #[test]
    fn test_cache_lock_waits_for_existing_lock() {
        with_cache_env("lock_wait", |dir| {
            let source = write_source(&dir, "model.va", "module m; endmodule\n");
            let entry = make_cache_entry(std::slice::from_ref(&source), "lock_wait");
            let lock_path = dir.join(VERILOGA_CACHE_LOCK_FILE);
            fs::write(&lock_path, "held").expect("failed to write lock file");

            let lock_path_for_thread = lock_path.clone();
            let releaser = thread::spawn(move || {
                thread::sleep(Duration::from_millis(80));
                let _ = fs::remove_file(lock_path_for_thread);
            });

            persist_model_to_disk(&source, &entry).expect("persist should wait and succeed");
            releaser.join().expect("lock release thread failed");
            assert!(!lock_path.exists(), "lock file should be cleaned up");
        });
    }
}
