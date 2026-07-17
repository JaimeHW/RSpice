use super::*;

#[cfg(feature = "veriloga")]
// Bump whenever a persisted runtime artifact or its integrity contract changes.
// Version 14 invalidates records written before canonical-IR digest validation
// became a mandatory cache-load boundary.
pub(super) const VERILOGA_CACHE_RECORD_VERSION: u32 = 14;
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_FILE: &str = ".rspice-veriloga-cache.lock";
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_WAIT_TIMEOUT: Duration = Duration::from_secs(15);
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_STALE_TIMEOUT: Duration = Duration::from_secs(180);
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) const VERILOGA_CACHE_LOCK_POLL_INTERVAL: Duration = Duration::from_millis(25);
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_ENTRIES: usize = 512;
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_DEFAULT_MAX_BYTES: u64 = 512 * 1024 * 1024;
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_MAX_ENTRIES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_ENTRIES";
#[cfg(feature = "veriloga")]
pub(super) const VERILOGA_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_CACHE_MAX_BYTES";

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
pub(super) struct VerilogADependencyFingerprint {
    pub(super) canonical_path: PathBuf,
    pub(super) modified_ns: Option<u128>,
    pub(super) file_len: u64,
    pub(super) content_hash: [u8; 32],
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub(super) struct VerilogADiskCacheRecord {
    pub(super) version: u32,
    pub(super) source_path: PathBuf,
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: rspice_veriloga::CompiledModel,
    pub(super) canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) struct CachedVerilogAModel {
    // Every production constructor validates the model/artifact pair before
    // this entry is admitted to the in-memory cache. Cache hits therefore only
    // need the comparatively cheap dependency freshness check.
    pub(super) dependencies: Vec<VerilogADependencyFingerprint>,
    pub(super) model: std::sync::Arc<rspice_veriloga::CompiledModel>,
    pub(super) canonical_ir:
        Option<std::sync::Arc<rspice_veriloga::canonical_ir::CanonicalIrArtifact>>,
}

/// Verify that cached bytecode and canonical IR can safely be paired at runtime.
///
/// A persisted entry is an optimization, never an authority. In particular,
/// older compiler builds can deserialize after an IR-digest change while still
/// looking structurally valid to serde. Treat that as a cache miss here rather
/// than allowing a stale artifact to reach the native JIT.
#[cfg(feature = "veriloga")]
fn validate_runtime_artifact_pair(
    model: &rspice_veriloga::CompiledModel,
    canonical_ir: Option<&rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    #[cfg(feature = "veriloga-native")]
    let artifact = canonical_ir.ok_or_else(|| {
        "native Verilog-A runtime cache entry requires canonical IR (no interpreter fallback)"
            .to_string()
    })?;
    #[cfg(not(feature = "veriloga-native"))]
    let Some(artifact) = canonical_ir else {
        return Ok(());
    };

    artifact.validate().map_err(|diagnostics| {
        let detail = diagnostics
            .first()
            .map(|diagnostic| diagnostic.message.as_str())
            .unwrap_or("canonical artifact validation failed");
        format!("canonical Verilog-A artifact failed integrity validation: {detail}")
    })?;

    if artifact.metadata.source_digest != model.source_digest {
        return Err(format!(
            "canonical Verilog-A source digest '{}' does not match compiled model digest '{}'",
            artifact.metadata.source_digest, model.source_digest
        ));
    }
    if artifact.mir.module_name != model.name {
        return Err(format!(
            "canonical Verilog-A module '{}' does not match compiled model '{}'",
            artifact.mir.module_name, model.name
        ));
    }
    if artifact.mir.equations.len() != model.stamp_programs.len() {
        return Err(format!(
            "canonical Verilog-A equation count {} does not match compiled stamp count {}",
            artifact.mir.equations.len(),
            model.stamp_programs.len()
        ));
    }

    Ok(())
}

#[cfg(feature = "veriloga")]
pub(super) fn veriloga_model_cache() -> &'static RwLock<HashMap<PathBuf, CachedVerilogAModel>> {
    static CACHE: OnceLock<RwLock<HashMap<PathBuf, CachedVerilogAModel>>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(HashMap::new()))
}

#[cfg(feature = "veriloga")]
pub(super) fn clear_in_memory_veriloga_cache() {
    if let Ok(mut cache) = veriloga_model_cache().write() {
        cache.clear();
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn canonicalize_for_cache(path: &Path) -> PathBuf {
    if is_project_veriloga_virtual_path(path) {
        return PathBuf::from(path.to_string_lossy().replace('\\', "/"));
    }
    std::fs::canonicalize(path).unwrap_or_else(|_| path.to_path_buf())
}

#[cfg(feature = "veriloga")]
fn is_project_veriloga_virtual_path(path: &Path) -> bool {
    let normalized = path.to_string_lossy().replace('\\', "/");
    normalized
        .split_once('/')
        .is_some_and(|(root, _)| root.eq_ignore_ascii_case("__rspice_project__"))
}

#[cfg(feature = "veriloga")]
pub(super) fn normalize_model_key(name: &str) -> String {
    name.to_ascii_uppercase()
}

#[cfg(feature = "veriloga")]
pub(super) fn metadata_modified_ns(metadata: &std::fs::Metadata) -> Option<u128> {
    use std::time::UNIX_EPOCH;

    metadata
        .modified()
        .ok()
        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
        .map(|d| d.as_nanos())
}

#[cfg(feature = "veriloga")]
pub(super) fn hash_file(path: &Path) -> std::io::Result<[u8; 32]> {
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
pub(super) fn dependency_fingerprint(path: &Path) -> Option<VerilogADependencyFingerprint> {
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
pub(super) fn fingerprint_paths(
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
pub(super) fn dependency_matches_cached_fingerprint(dep: &VerilogADependencyFingerprint) -> bool {
    match hash_file(&dep.canonical_path) {
        Ok(hash) => hash == dep.content_hash,
        Err(_) => false,
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn dependencies_are_fresh(dependencies: &[VerilogADependencyFingerprint]) -> bool {
    dependencies
        .iter()
        .all(dependency_matches_cached_fingerprint)
}

#[cfg(feature = "veriloga")]
pub(super) fn parse_cache_env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<usize>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
pub(super) fn parse_cache_env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|raw| raw.trim().parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default)
}

#[cfg(feature = "veriloga")]
pub(super) fn veriloga_cache_limits() -> (usize, u64) {
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

// `std::env::temp_dir` (and the lock's `Instant`/`thread::sleep`) abort on
// wasm32-unknown-unknown rather than erroring, so the entire disk side of
// the cache only exists on native targets; `with_veriloga_cache_disk_lock`
// is the single gate.
#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn veriloga_cache_root() -> PathBuf {
    if let Some(override_dir) = std::env::var_os("RSPICE_VERILOGA_CACHE_DIR") {
        return PathBuf::from(override_dir);
    }

    if let Some(cache_dir) = dirs::cache_dir() {
        return cache_dir.join("rspice").join("veriloga");
    }

    std::env::temp_dir().join("rspice-veriloga-cache")
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn cache_record_path_with_root(source_path: &Path, cache_root: &Path) -> PathBuf {
    let canonical = canonicalize_for_cache(source_path);
    let mut hasher = blake3::Hasher::new();
    hasher.update(canonical.to_string_lossy().as_bytes());
    let key = hasher.finalize().to_hex().to_string();
    cache_root.join(format!("{key}.json"))
}

#[cfg(feature = "veriloga")]
#[derive(Debug, Clone)]
pub(super) struct VerilogACacheFileInfo {
    path: PathBuf,
    size_bytes: u64,
    modified_ns: Option<u128>,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
#[derive(Debug)]
pub(super) struct VerilogACacheDiskLock {
    lock_path: PathBuf,
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
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

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
impl Drop for VerilogACacheDiskLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_file(&self.lock_path);
    }
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn cache_lock_is_stale(lock_path: &Path) -> bool {
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
pub(super) fn with_veriloga_cache_disk_lock<T>(
    operation: &str,
    f: impl FnOnce(&Path) -> Result<T, String>,
) -> Result<T, String> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = f;
        Err(format!(
            "{}: the Verilog-A disk cache is unavailable in the browser build",
            operation
        ))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let root = veriloga_cache_root();
        let _lock =
            VerilogACacheDiskLock::acquire(&root).map_err(|e| format!("{}: {}", operation, e))?;
        f(&root)
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn list_cache_files(cache_root: &Path) -> Result<Vec<VerilogACacheFileInfo>, String> {
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
        if !matches!(
            path.extension().and_then(|e| e.to_str()),
            Some("json" | "bin")
        ) {
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
pub(super) fn cache_stats_from_files(
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
pub(super) fn read_cache_record(path: &Path) -> Result<VerilogADiskCacheRecord, String> {
    let bytes = std::fs::read(path)
        .map_err(|e| format!("failed to read cache record '{}': {}", path.display(), e))?;
    serde_json::from_slice::<VerilogADiskCacheRecord>(&bytes).map_err(|e| {
        format!(
            "failed to deserialize cache record '{}': {}",
            path.display(),
            e
        )
    })
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn encode_cache_record(record: &VerilogADiskCacheRecord) -> Result<Vec<u8>, String> {
    serde_json::to_vec(record)
        .map_err(|e| format!("failed to serialize Verilog-A cache record: {}", e))
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn persist_model_to_disk_locked(
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
        model: entry.model.as_ref().clone(),
        canonical_ir: entry.canonical_ir.as_ref().map(|ir| ir.as_ref().clone()),
    };
    let encoded = encode_cache_record(&record)?;

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
pub(super) fn remove_cache_file(path: &Path) {
    if let Err(err) = std::fs::remove_file(path)
        && err.kind() != std::io::ErrorKind::NotFound
    {
        log::warn!(
            "failed to remove stale/corrupt Verilog-A cache file '{}': {}",
            path.display(),
            err
        );
    }
}

#[cfg(feature = "veriloga")]
pub(super) fn prune_veriloga_cache_locked(
    cache_root: &Path,
) -> Result<VerilogACachePruneReport, String> {
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
pub(super) fn persist_model_to_disk(
    source_path: &Path,
    entry: &CachedVerilogAModel,
) -> Result<(), String> {
    // No disk in the browser build: the in-memory cache is the cache.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        let _ = entry.canonical_ir.as_ref();
        Ok(())
    }
    #[cfg(not(target_arch = "wasm32"))]
    with_veriloga_cache_disk_lock("persist Verilog-A cache record", |cache_root| {
        persist_model_to_disk_locked(source_path, entry, cache_root)?;
        if let Err(err) = prune_veriloga_cache_locked(cache_root) {
            log::warn!("failed to prune Verilog-A cache after write: {}", err);
        }
        Ok(())
    })
}

#[cfg(all(feature = "veriloga", not(target_arch = "wasm32")))]
pub(super) fn load_model_from_disk_locked(
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

    if let Err(error) = validate_runtime_artifact_pair(&record.model, record.canonical_ir.as_ref())
    {
        log::warn!(
            "discarding invalid Verilog-A cache record '{}': {}",
            cache_path.display(),
            error
        );
        remove_cache_file(&cache_path);
        return Ok(None);
    }

    Ok(Some(CachedVerilogAModel {
        dependencies: record.dependencies,
        model: std::sync::Arc::new(record.model),
        canonical_ir: record.canonical_ir.map(std::sync::Arc::new),
    }))
}

#[cfg(feature = "veriloga")]
pub(super) fn load_model_from_disk(source_path: &Path) -> Option<CachedVerilogAModel> {
    // No disk in the browser build: only the in-memory cache can hit.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = source_path;
        None
    }
    #[cfg(not(target_arch = "wasm32"))]
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
            if validate_runtime_artifact_pair(&record.model, record.canonical_ir.as_ref()).is_err()
            {
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
pub(super) fn resolve_cached_or_compile_veriloga(
    path: &Path,
) -> Result<CachedVerilogAModel, SimulationError> {
    let canonical = canonicalize_for_cache(path);
    let mut stale_in_memory = false;

    if let Ok(cache) = veriloga_model_cache().read()
        && let Some(entry) = cache.get(&canonical)
    {
        if dependencies_are_fresh(&entry.dependencies) {
            log::debug!("Verilog-A cache hit (memory): '{}'", canonical.display());
            return Ok(entry.clone());
        }
        stale_in_memory = true;
    }

    if stale_in_memory && let Ok(mut cache) = veriloga_model_cache().write() {
        cache.remove(&canonical);
    }

    // A project-owned virtual key is an authenticated in-memory capability,
    // never a filesystem locator. A missing registration must fail closed;
    // disk cache and ambient files are not eligible fallbacks.
    if is_project_veriloga_virtual_path(path) {
        return Err(SimulationError::Netlist(format!(
            "Project Verilog-A runtime '{}' is not installed for this execution",
            path.display()
        )));
    }

    if let Some(entry) = load_model_from_disk(&canonical) {
        if let Ok(mut cache) = veriloga_model_cache().write() {
            cache.insert(canonical.clone(), entry.clone());
        }
        log::debug!("Verilog-A cache hit (disk): '{}'", canonical.display());
        return Ok(entry);
    }

    log::info!("Verilog-A cache miss, compiling '{}'", canonical.display());
    let compiler = rspice_veriloga::VerilogACompiler::default();
    let compiled = compiler
        .compile_file_runtime_with_metadata(path, None)
        .map_err(|e| {
            SimulationError::Netlist(format!(
                "Failed to compile Verilog-A '{}': {}",
                path.display(),
                e
            ))
        })?;

    validate_runtime_artifact_pair(&compiled.model, Some(&compiled.canonical_ir)).map_err(
        |error| {
            SimulationError::Netlist(format!(
                "Compiled Verilog-A runtime artifacts for '{}' failed integrity validation: {}",
                path.display(),
                error
            ))
        },
    )?;
    let dependencies = fingerprint_paths(&compiled.dependencies)?;
    let entry = CachedVerilogAModel {
        dependencies,
        model: std::sync::Arc::new(compiled.model),
        canonical_ir: Some(std::sync::Arc::new(compiled.canonical_ir)),
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

    Ok(entry)
}

#[cfg(feature = "veriloga")]
fn register_precompiled_veriloga_entry_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: Option<rspice_veriloga::canonical_ir::CanonicalIrArtifact>,
) -> Result<(), String> {
    #[cfg(feature = "veriloga-native")]
    if canonical_ir.is_none() {
        return Err(
            "native Verilog-A registration requires canonical IR; use register_precompiled_veriloga_runtime_with_dependencies"
                .to_string(),
        );
    }

    validate_runtime_artifact_pair(&model, canonical_ir.as_ref())?;

    let canonical_source = canonicalize_for_cache(source_path.as_ref());

    #[cfg(not(target_arch = "wasm32"))]
    let dependency_fingerprints = {
        let mut dependency_paths = dependencies.to_vec();
        if dependency_paths.is_empty() {
            dependency_paths.push(canonical_source.clone());
        }
        fingerprint_paths(&dependency_paths)
            .map_err(|e| format!("dependency fingerprinting failed: {}", e))?
    };
    // The browser build has no filesystem to fingerprint against; an empty
    // dependency set always reads as fresh, so the registered artifact is
    // simply trusted for the session.
    #[cfg(target_arch = "wasm32")]
    let dependency_fingerprints = {
        let _ = dependencies;
        Vec::new()
    };

    let entry = CachedVerilogAModel {
        dependencies: dependency_fingerprints,
        model: std::sync::Arc::new(model),
        canonical_ir: canonical_ir.map(std::sync::Arc::new),
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

/// Register a precompiled Verilog-A runtime artifact in the global engine cache.
///
/// Native builds require this paired model/canonical-IR artifact so the runtime
/// cannot silently fall back to bytecode-only construction.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_runtime_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
    canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(
        source_path,
        dependencies,
        model,
        Some(canonical_ir),
    )
}

/// Register an exact in-memory project-owned Verilog-A runtime for this
/// process session.
///
/// Unlike file-backed registration, this entry has no filesystem dependency
/// fingerprints and is never persisted to the shared disk cache. Its key is
/// still normalized exactly like an `.include` path, so a generated deck can
/// resolve the project-owned logical file name without writing source bytes to
/// an ambient directory.
#[cfg(feature = "veriloga")]
pub fn register_project_veriloga_runtime_for_session(
    source_key: impl AsRef<Path>,
    model: rspice_veriloga::CompiledModel,
    canonical_ir: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
) -> Result<(), String> {
    let source_key = source_key.as_ref();
    let key_text = source_key.to_string_lossy().replace('\\', "/");
    if !key_text.starts_with("__rspice_project__/")
        || key_text
            .split('/')
            .any(|component| component.is_empty() || matches!(component, "." | ".."))
    {
        return Err(
            "project Verilog-A runtime keys must be normalized content-addressed virtual paths under __rspice_project__/"
                .to_owned(),
        );
    }
    validate_runtime_artifact_pair(&model, Some(&canonical_ir))?;
    let normalized = canonicalize_for_cache(source_key);
    let entry = CachedVerilogAModel {
        dependencies: Vec::new(),
        model: std::sync::Arc::new(model),
        canonical_ir: Some(std::sync::Arc::new(canonical_ir)),
    };
    let mut cache = veriloga_model_cache()
        .write()
        .map_err(|_| "failed to acquire Verilog-A cache lock".to_owned())?;
    cache.insert(normalized, entry);
    Ok(())
}

/// Register a precompiled Verilog-A model in the global engine cache.
///
/// This allows UI workflows to compile once on import and reuse the compiled
/// artifact during simulation without recompilation. Native JIT builds should
/// use [`register_precompiled_veriloga_runtime_with_dependencies`] so the cache
/// carries canonical IR as well as the compiled model.
#[cfg(feature = "veriloga")]
pub fn register_precompiled_veriloga_model_with_dependencies(
    source_path: impl AsRef<Path>,
    dependencies: &[PathBuf],
    model: rspice_veriloga::CompiledModel,
) -> Result<(), String> {
    register_precompiled_veriloga_entry_with_dependencies(source_path, dependencies, model, None)
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

#[cfg(all(test, feature = "veriloga", not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_test_root(name: &str) -> PathBuf {
        static NEXT: AtomicU64 = AtomicU64::new(0);
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time is after the Unix epoch")
            .as_nanos();
        let sequence = NEXT.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!(
            "rspice-veriloga-cache-{name}-{}-{nonce}-{sequence}",
            std::process::id()
        ))
    }

    fn compiled_entry(source_path: &Path) -> CachedVerilogAModel {
        std::fs::write(
            source_path,
            r#"
`include "disciplines.vams"
module cached_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
        .expect("write temporary Verilog-A source");

        let runtime = rspice_veriloga::VerilogACompiler::default()
            .compile_file_runtime_with_metadata(source_path, None)
            .expect("compile runtime artifacts");
        let dependencies = fingerprint_paths(&runtime.dependencies).expect("fingerprint source");
        CachedVerilogAModel {
            dependencies,
            model: std::sync::Arc::new(runtime.model),
            canonical_ir: Some(std::sync::Arc::new(runtime.canonical_ir)),
        }
    }

    #[test]
    fn disk_cache_loads_a_valid_paired_runtime_artifact() {
        let root = unique_test_root("valid");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let entry = compiled_entry(&source_path);
        let cache_root = root.join("cache");

        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist valid cache record");
        let loaded = load_model_from_disk_locked(&source_path, &cache_root)
            .expect("load valid cache record")
            .expect("valid cache record is retained");

        assert_eq!(loaded.model.name, entry.model.name);
        assert_eq!(
            loaded
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest,
            entry
                .canonical_ir
                .as_ref()
                .expect("canonical IR")
                .hir_digest
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn disk_cache_discards_a_stale_canonical_artifact() {
        let root = unique_test_root("stale-artifact");
        let source_path = root.join("model.va");
        std::fs::create_dir_all(&root).expect("create temporary cache root");
        let mut entry = compiled_entry(&source_path);
        let artifact = std::sync::Arc::make_mut(
            entry
                .canonical_ir
                .as_mut()
                .expect("compiled entry carries canonical IR"),
        );
        artifact.hir_digest = "stale-hir-digest".into();
        let cache_root = root.join("cache");
        let cache_path = cache_record_path_with_root(&source_path, &cache_root);

        persist_model_to_disk_locked(&source_path, &entry, &cache_root)
            .expect("persist stale cache record");
        assert!(cache_path.is_file(), "test must materialize a cache record");
        assert!(
            load_model_from_disk_locked(&source_path, &cache_root)
                .expect("stale cache load is recoverable")
                .is_none(),
            "stale canonical IR must force a cache miss"
        );
        assert!(
            !cache_path.exists(),
            "stale cache record must be removed to prevent repeated failures"
        );

        std::fs::remove_dir_all(root).expect("remove temporary cache root");
    }

    #[test]
    fn project_owned_runtime_registration_requires_no_ambient_source_file() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000001/0123456789abcdef/model.va",
        );
        assert!(!source_key.exists());
        let report = rspice_veriloga::VerilogACompiler::default()
            .compile_runtime(
                "module owned(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
                None,
            )
            .expect("compile in-memory project source");

        register_project_veriloga_runtime_for_session(
            &source_key,
            report.model,
            report.canonical_ir,
        )
        .expect("register in-memory project runtime");

        let key = canonicalize_for_cache(&source_key);
        let mut cache = veriloga_model_cache().write().expect("cache lock");
        let entry = cache.get(&key).expect("session entry");
        assert!(entry.dependencies.is_empty());
        assert_eq!(entry.model.name.as_str(), "owned");
        assert!(entry.canonical_ir.is_some());
        cache.remove(&key);
    }

    #[test]
    fn same_file_name_in_distinct_projects_resolves_only_its_registered_runtime() {
        let compile = |module_name: &str| {
            rspice_veriloga::VerilogACompiler::default()
                .compile_runtime(
                    &format!(
                        "module {module_name}(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n"
                    ),
                    None,
                )
                .expect("compile project runtime")
        };
        let first_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000011/digest/model.va",
        );
        let second_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000022/digest/model.va",
        );
        let first = compile("first_owned");
        let second = compile("second_owned");
        register_project_veriloga_runtime_for_session(&first_key, first.model, first.canonical_ir)
            .unwrap();
        register_project_veriloga_runtime_for_session(
            &second_key,
            second.model,
            second.canonical_ir,
        )
        .unwrap();

        assert_eq!(
            resolve_cached_or_compile_veriloga(&first_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "first_owned"
        );
        assert_eq!(
            resolve_cached_or_compile_veriloga(&second_key)
                .unwrap()
                .model
                .name
                .as_str(),
            "second_owned"
        );

        let mut cache = veriloga_model_cache().write().unwrap();
        cache.remove(&canonicalize_for_cache(&first_key));
        cache.remove(&canonicalize_for_cache(&second_key));
    }

    #[test]
    fn project_virtual_cache_identity_is_lexical_even_if_an_ambient_path_exists() {
        let unique = unique_test_root("virtual-lexical")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        let parent = source_key.parent().expect("virtual key parent");
        std::fs::create_dir_all(parent).expect("materialize adversarial ambient directory");
        std::fs::write(
            &source_key,
            "ambient bytes must never define cache identity",
        )
        .expect("materialize adversarial ambient file");

        assert_eq!(canonicalize_for_cache(&source_key), source_key);

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(&unique))
            .expect("remove adversarial ambient path");
        assert_eq!(canonicalize_for_cache(&source_key), source_key);
    }

    #[test]
    fn missing_project_virtual_runtime_never_falls_back_to_ambient_io() {
        let source_key = PathBuf::from(
            "__rspice_project__/00000000-0000-0000-0000-000000000099/missing/model.va",
        );
        let error = resolve_cached_or_compile_veriloga(&source_key)
            .expect_err("unregistered project runtime must fail closed");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution")
        );
    }

    #[test]
    fn case_altered_project_virtual_path_never_compiles_an_ambient_file() {
        let unique = unique_test_root("virtual-case")
            .file_name()
            .expect("unique component")
            .to_string_lossy()
            .into_owned();
        let exact_source_key = PathBuf::from(format!(
            "__rspice_project__/{unique}/0123456789abcdef/model.va"
        ));
        std::fs::create_dir_all(exact_source_key.parent().expect("virtual key parent"))
            .expect("materialize adversarial ambient directory");
        std::fs::write(
            &exact_source_key,
            "module ambient_escape(p, n); inout p, n; electrical p, n; analog I(p,n) <+ V(p,n); endmodule\n",
        )
        .expect("materialize compilable adversarial ambient source");

        let altered_source_key = PathBuf::from(format!(
            "__RSPICE_PROJECT__/{unique}/0123456789abcdef/model.va"
        ));
        let error = resolve_cached_or_compile_veriloga(&altered_source_key)
            .expect_err("case-altered project key must fail before ambient compilation");
        assert!(
            error
                .to_string()
                .contains("is not installed for this execution"),
            "unexpected error: {error}"
        );

        std::fs::remove_dir_all(PathBuf::from("__rspice_project__").join(unique))
            .expect("remove adversarial ambient path");
    }
}
