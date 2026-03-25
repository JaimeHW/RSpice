use super::veriloga_cache::*;
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
fn test_dependency_matches_cached_fingerprint_rejects_stale_hash_even_with_current_metadata() {
    with_cache_env("hash_wins_over_metadata", |dir| {
        let file = write_source(&dir, "model.va", "`define X 1\n");
        let stale = dependency_fingerprint(&file).expect("initial fingerprint expected");

        fs::write(&file, "`define Y 1\n").expect("failed to rewrite dependency with same length");
        let metadata = fs::metadata(&file).expect("updated metadata should be available");
        let simulated_stale = VerilogADependencyFingerprint {
            canonical_path: stale.canonical_path.clone(),
            modified_ns: metadata_modified_ns(&metadata),
            file_len: metadata.len(),
            content_hash: stale.content_hash,
        };

        assert!(
            !dependency_matches_cached_fingerprint(&simulated_stale),
            "content hash should invalidate cache freshness even when metadata matches"
        );
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
        let loaded = load_model_from_disk(&source).expect("expected roundtrip cache entry to load");
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
