//! Bounded native image retention and per-artifact compilation coordination.

use super::NativeModel;
#[cfg(feature = "native-bytecode-contract-tests")]
use crate::codegen::CompiledModel;
use crate::{PipelineControl, vm::VmError};
use smol_str::SmolStr;
use std::sync::{Arc, Condvar, Mutex, OnceLock};

type CompilationResult = Result<Arc<NativeModel>, String>;

struct Compilation {
    key: NativeCompileCacheKey,
    result: OnceLock<CompilationResult>,
}

#[derive(Default)]
struct CacheState {
    ready: ReadyCache,
    compiling: Vec<Arc<Compilation>>,
}

/// Construction is synchronous, but unrelated requests share only brief cache
/// operations. A waiter may cancel without cancelling the compiler that owns
/// its request. An owner finishes an already-started compilation and publishes
/// it for other callers before reporting its own cancellation.
pub(crate) struct NativeCompileCache {
    state: Mutex<CacheState>,
    changed: Condvar,
    max_compilers: usize,
}

impl Default for NativeCompileCache {
    fn default() -> Self {
        // Bound compiler scratch memory independently of retained code pages.
        // Two large models can make progress together without multiplying
        // their temporary graphs by every available core on a workstation.
        Self::new(std::thread::available_parallelism().map_or(1, |n| n.get().min(2)))
    }
}

impl NativeCompileCache {
    fn new(max_compilers: usize) -> Self {
        Self {
            state: Mutex::new(CacheState::default()),
            changed: Condvar::new(),
            max_compilers: max_compilers.max(1),
        }
    }

    pub(crate) fn get_or_compile(
        &self,
        key: NativeCompileCacheKey,
        control: &dyn PipelineControl,
        compile: impl FnOnce() -> CompilationResult,
    ) -> Result<Arc<NativeModel>, VmError> {
        let mut waiting: Option<Arc<Compilation>> = None;
        let job = loop {
            if control.is_cancelled() {
                return Err(VmError::CompilationCancelled);
            }
            let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
            if let Some(job) = &waiting {
                if let Some(result) = job.result.get() {
                    return result.clone().map_err(VmError::NativeJit);
                }
            } else if let Some(result) = state.ready.get(&key) {
                return result.map_err(VmError::NativeJit);
            } else if let Some(job) = state.compiling.iter().find(|job| {
                job.key == key && NativeCompileCacheRetention::for_key(&job.key).matches_key(&key)
            }) {
                waiting = Some(Arc::clone(job));
            } else if state.compiling.len() < self.max_compilers {
                let job = Arc::new(Compilation {
                    key,
                    result: OnceLock::new(),
                });
                state.compiling.push(Arc::clone(&job));
                break job;
            }
            // Completion notifies immediately; this timeout only bounds the
            // latency of a cancellation whose caller cannot notify this cache.
            let (state, _) = self
                .changed
                .wait_timeout(state, std::time::Duration::from_millis(25))
                .unwrap_or_else(|e| e.into_inner());
            drop(state);
        };

        let owner = CompilationOwner {
            cache: self,
            job,
            published: false,
        };
        let result = compile();
        owner.publish(result.clone());
        if control.is_cancelled() {
            return Err(VmError::CompilationCancelled);
        }
        result.map_err(VmError::NativeJit)
    }

    fn publish(&self, job: &Arc<Compilation>, result: CompilationResult, retain: bool) {
        let mut state = self.state.lock().unwrap_or_else(|e| e.into_inner());
        if retain {
            state.ready.insert(job.key.clone(), result.clone());
        }
        let published = job.result.set(result).is_ok();
        debug_assert!(published, "one owner must publish each compilation once");
        state.compiling.retain(|active| !Arc::ptr_eq(active, job));
        self.changed.notify_all();
    }
}

/// Unwinding must wake waiters and release capacity. Compiler panics propagate
/// to their owner; waiters receive an error, and a later request can retry.
struct CompilationOwner<'a> {
    cache: &'a NativeCompileCache,
    job: Arc<Compilation>,
    published: bool,
}

impl CompilationOwner<'_> {
    fn publish(mut self, result: CompilationResult) {
        self.cache.publish(&self.job, result, true);
        self.published = true;
    }
}

impl Drop for CompilationOwner<'_> {
    fn drop(&mut self) {
        if !self.published {
            self.cache.publish(
                &self.job,
                Err("native compilation unwound before publishing an image".into()),
                false,
            );
        }
    }
}

/// Content identity of one native compilation.
///
/// Keying on content rather than on the `Arc<CompiledModel>` address is what
/// lets a second engine build reuse the first build's image: the runtime model
/// cache hands back a freshly allocated `Arc` after a disk-cache hit, so
/// pointer identity reports a miss for a model that is byte-identical.
///
/// The digests are carried together because the emitted image is a function of
/// both artifacts the compiler consumes. `mir_digest` alone determines the
/// image for artifacts produced by one compiler build, but the pair costs
/// nothing and keeps the key honest if that ever stops being true.
#[derive(Clone)]
pub(crate) enum NativeCompileCacheKey {
    /// Internal bytecode-native contract-test cache lane. Production native
    /// construction must compile through `CanonicalMir`. Bytecode compilation
    /// is tied to the lifetime of the bytecode model allocation because that
    /// lane has no canonical artifact whose digest independently authenticates
    /// the complete compiler input.
    #[cfg(feature = "native-bytecode-contract-tests")]
    Bytecode {
        source_digest: SmolStr,
        module: SmolStr,
        owner: std::sync::Weak<CompiledModel>,
    },
    CanonicalMir {
        role: NativeCompileRole,
        mir_digest: SmolStr,
        source_digest: SmolStr,
        module: SmolStr,
        layout: blake3::Hash,
    },
}

/// What one cached image is for.
///
/// The two roles compile the same model from the same artifact into different
/// code, so they are different cache entries and not one entry reused: an
/// evaluation image holds the entries, the kernels and the assignment pass the
/// CFG plan still needs, and an observation image holds only the pass that
/// publishes the externally observable variables. Sharing the key would hand a
/// readback the evaluation's image, or an evaluation a set of entries that does
/// not exist.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum NativeCompileRole {
    Evaluation,
    Observation,
}

impl PartialEq for NativeCompileCacheKey {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            #[cfg(feature = "native-bytecode-contract-tests")]
            (
                Self::Bytecode {
                    source_digest: left_source,
                    module: left_module,
                    ..
                },
                Self::Bytecode {
                    source_digest: right_source,
                    module: right_module,
                    ..
                },
            ) => left_source == right_source && left_module == right_module,
            (
                Self::CanonicalMir {
                    role: left_role,
                    mir_digest: left_mir,
                    source_digest: left_source,
                    module: left_module,
                    layout: left_layout,
                },
                Self::CanonicalMir {
                    role: right_role,
                    mir_digest: right_mir,
                    source_digest: right_source,
                    module: right_module,
                    layout: right_layout,
                },
            ) => {
                left_role == right_role
                    && left_mir == right_mir
                    && left_source == right_source
                    && left_module == right_module
                    && left_layout == right_layout
            }
            #[cfg(feature = "native-bytecode-contract-tests")]
            _ => false,
        }
    }
}

impl Eq for NativeCompileCacheKey {}

/// Lifetime contract for a cached native image.
///
/// Canonical MIR authenticates every compiler input, so its images are
/// process-persistent and reusable across separately allocated runtime models.
/// The internal bytecode lane has no such artifact and is allocation-scoped:
/// its image and cached failures remain valid only while that exact
/// `CompiledModel` allocation is alive.
enum NativeCompileCacheRetention {
    Persistent,
    #[cfg(feature = "native-bytecode-contract-tests")]
    Model(std::sync::Weak<CompiledModel>),
}

impl NativeCompileCacheRetention {
    fn for_key(key: &NativeCompileCacheKey) -> Self {
        match key {
            #[cfg(feature = "native-bytecode-contract-tests")]
            NativeCompileCacheKey::Bytecode { owner, .. } => Self::Model(owner.clone()),
            NativeCompileCacheKey::CanonicalMir { .. } => Self::Persistent,
        }
    }

    fn is_dropped(&self) -> bool {
        match self {
            Self::Persistent => false,
            #[cfg(feature = "native-bytecode-contract-tests")]
            Self::Model(owner) => owner.strong_count() == 0,
        }
    }

    fn matches_key(&self, key: &NativeCompileCacheKey) -> bool {
        match (self, key) {
            (Self::Persistent, NativeCompileCacheKey::CanonicalMir { .. }) => true,
            #[cfg(feature = "native-bytecode-contract-tests")]
            (Self::Model(retained), NativeCompileCacheKey::Bytecode { owner, .. }) => {
                std::sync::Weak::ptr_eq(retained, owner)
            }
            #[cfg(feature = "native-bytecode-contract-tests")]
            _ => false,
        }
    }
}

/// Executable-image budget for the process-wide native compilation cache.
///
/// Entries hold committed executable pages, so the cache is bounded by bytes
/// with an additional entry limit covering metadata and failures. A failure
/// is reused until evicted, then a later request can retry it.
const NATIVE_COMPILE_CACHE_DEFAULT_MAX_BYTES: usize = 512 * 1024 * 1024;
const NATIVE_COMPILE_CACHE_MAX_ENTRIES: usize = 1024;

const NATIVE_COMPILE_CACHE_MAX_BYTES_ENV: &str = "RSPICE_VERILOGA_NATIVE_CACHE_MAX_BYTES";

struct ReadyEntry {
    key: NativeCompileCacheKey,
    retention: NativeCompileCacheRetention,
    compiled: Result<std::sync::Arc<NativeModel>, String>,
    image_bytes: usize,
}

#[derive(Default)]
struct ReadyCache {
    /// Most-recently-used first.
    entries: Vec<ReadyEntry>,
    image_bytes: usize,
}

impl ReadyCache {
    fn max_bytes() -> usize {
        std::env::var(NATIVE_COMPILE_CACHE_MAX_BYTES_ENV)
            .ok()
            .and_then(|raw| raw.trim().parse::<usize>().ok())
            .filter(|budget| *budget > 0)
            .unwrap_or(NATIVE_COMPILE_CACHE_DEFAULT_MAX_BYTES)
    }

    fn get(
        &mut self,
        key: &NativeCompileCacheKey,
    ) -> Option<Result<std::sync::Arc<NativeModel>, String>> {
        self.prune_dropped_bytecode_owners();
        let index = self
            .entries
            .iter()
            .position(|entry| entry.key == *key && entry.retention.matches_key(key))?;
        let entry = self.entries.remove(index);
        let compiled = entry.compiled.clone();
        self.entries.insert(0, entry);
        self.debug_assert_image_bytes();
        Some(compiled)
    }

    fn insert(
        &mut self,
        key: NativeCompileCacheKey,
        compiled: Result<std::sync::Arc<NativeModel>, String>,
    ) {
        self.prune_dropped_bytecode_owners();
        let retention = NativeCompileCacheRetention::for_key(&key);
        let image_bytes = compiled
            .as_ref()
            .map_or(0, |native| native.code_size_bytes());
        self.entries.insert(
            0,
            ReadyEntry {
                key,
                retention,
                compiled,
                image_bytes,
            },
        );
        self.image_bytes = self
            .image_bytes
            .checked_add(image_bytes)
            .expect("native compile cache image accounting exceeds addressable memory");
        self.evict_to(Self::max_bytes());
        self.debug_assert_image_bytes();
    }

    /// Remove test-lane images after their bytecode model allocation has been
    /// dropped. Canonical MIR entries deliberately have no owner and remain
    /// reusable across model allocations and engine rebuilds.
    fn prune_dropped_bytecode_owners(&mut self) {
        let mut index = 0;
        while index < self.entries.len() {
            if self.entries[index].retention.is_dropped() {
                let evicted = self.entries.remove(index);
                self.image_bytes = self
                    .image_bytes
                    .checked_sub(evicted.image_bytes)
                    .expect("native compile cache image accounting underflowed while pruning");
            } else {
                index += 1;
            }
        }
        self.debug_assert_image_bytes();
    }

    fn debug_assert_image_bytes(&self) {
        #[cfg(debug_assertions)]
        {
            let accounted = self.entries.iter().fold(0usize, |total, entry| {
                total
                    .checked_add(entry.image_bytes)
                    .expect("native compile cache entry sizes exceed addressable memory")
            });
            debug_assert_eq!(
                self.image_bytes, accounted,
                "native compile cache executable-image accounting drifted"
            );
        }
    }

    /// Drop least-recently-used images until the budget is met, always keeping
    /// the entry that was just inserted so one oversized model cannot evict
    /// itself into an infinite recompile loop.
    fn evict_to(&mut self, max_bytes: usize) {
        while (self.image_bytes > max_bytes && self.entries.len() > 1)
            || self.entries.len() > NATIVE_COMPILE_CACHE_MAX_ENTRIES
        {
            let Some(evicted) = self.entries.pop() else {
                break;
            };
            self.image_bytes = self
                .image_bytes
                .checked_sub(evicted.image_bytes)
                .expect("native compile cache image accounting underflowed while evicting");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::NoPipelineControl;
    use std::sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        mpsc,
    };
    use std::time::Duration;

    const TIMEOUT: Duration = Duration::from_secs(10);

    fn key(name: &str) -> NativeCompileCacheKey {
        NativeCompileCacheKey::CanonicalMir {
            role: NativeCompileRole::Evaluation,
            mir_digest: name.into(),
            source_digest: name.into(),
            module: name.into(),
            layout: blake3::hash(name.as_bytes()),
        }
    }

    struct Control {
        cancelled: AtomicBool,
        polls: AtomicUsize,
        waiting: mpsc::Sender<()>,
    }

    impl Control {
        fn new() -> (Self, mpsc::Receiver<()>) {
            let (waiting, observed) = mpsc::channel();
            (
                Self {
                    cancelled: AtomicBool::new(false),
                    polls: AtomicUsize::new(0),
                    waiting,
                },
                observed,
            )
        }
    }

    impl PipelineControl for Control {
        fn is_cancelled(&self) -> bool {
            // A second poll means the request has registered its wait and
            // released the cache mutex at least once.
            if self.polls.fetch_add(1, Ordering::Relaxed) == 1 {
                let _ = self.waiting.send(());
            }
            self.cancelled.load(Ordering::Acquire)
        }
    }

    #[test]
    fn independent_compilation_and_ready_hits_progress_during_a_blocked_compile() {
        let cache = NativeCompileCache::new(2);
        let _ = cache.get_or_compile(key("ready"), &NoPipelineControl, || Err("ready".into()));
        let (started, observed) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        let owner_cache = &cache;
        std::thread::scope(|scope| {
            let owner = scope.spawn(move || {
                owner_cache.get_or_compile(key("slow"), &NoPipelineControl, || {
                    started.send(()).unwrap();
                    let _ = blocked.recv_timeout(TIMEOUT);
                    Err("slow".into())
                })
            });
            observed.recv_timeout(TIMEOUT).unwrap();
            let (finished, results) = mpsc::channel();
            for name in ["ready", "independent"] {
                let finished = finished.clone();
                let cache = &cache;
                scope.spawn(move || {
                    let result = cache.get_or_compile(key(name), &NoPipelineControl, || {
                        assert_ne!(name, "ready", "a ready request must reuse its result");
                        Err(name.into())
                    });
                    finished.send((name, result.err())).unwrap();
                });
            }
            let first = results.recv_timeout(TIMEOUT);
            let second = results.recv_timeout(TIMEOUT);
            let _ = release.send(());
            owner.join().unwrap().unwrap_err();
            for (name, result) in [first.unwrap(), second.unwrap()] {
                assert_eq!(result, Some(VmError::NativeJit(name.into())));
            }
        });
    }

    #[test]
    fn same_key_waiters_share_failures_and_cannot_cancel_other_callers() {
        let cache = NativeCompileCache::new(1);
        let (started, observed) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        let (same_control, same_waiting) = Control::new();
        let (other_control, other_waiting) = Control::new();
        let (survivor_control, survivor_waiting) = Control::new();
        let owner_cache = &cache;
        std::thread::scope(|scope| {
            let owner = scope.spawn(move || {
                owner_cache.get_or_compile(key("one"), &NoPipelineControl, || {
                    started.send(()).unwrap();
                    let _ = blocked.recv_timeout(TIMEOUT);
                    Err("retained failure".into())
                })
            });
            observed.recv_timeout(TIMEOUT).unwrap();
            let same = scope.spawn(|| {
                cache.get_or_compile(key("one"), &same_control, || {
                    panic!("duplicate compilation")
                })
            });
            let other = scope.spawn(|| {
                cache.get_or_compile(key("two"), &other_control, || {
                    panic!("compiler limit exceeded")
                })
            });
            let survivor = scope.spawn(|| {
                cache.get_or_compile(key("one"), &survivor_control, || {
                    panic!("duplicate compilation")
                })
            });
            same_waiting.recv_timeout(TIMEOUT).unwrap();
            other_waiting.recv_timeout(TIMEOUT).unwrap();
            survivor_waiting.recv_timeout(TIMEOUT).unwrap();
            same_control.cancelled.store(true, Ordering::Release);
            other_control.cancelled.store(true, Ordering::Release);
            assert_eq!(
                same.join().unwrap().err(),
                Some(VmError::CompilationCancelled)
            );
            assert_eq!(
                other.join().unwrap().err(),
                Some(VmError::CompilationCancelled)
            );
            release.send(()).unwrap();
            let expected = Some(VmError::NativeJit("retained failure".into()));
            assert_eq!(owner.join().unwrap().err(), expected);
            assert_eq!(survivor.join().unwrap().err(), expected);
            assert_eq!(
                cache
                    .get_or_compile(key("one"), &NoPipelineControl, || panic!(
                        "cached failure recompiled"
                    ))
                    .err(),
                expected
            );
        });
    }

    #[test]
    fn unwinding_wakes_waiters_releases_capacity_and_allows_retry() {
        let cache = NativeCompileCache::new(1);
        let (started, observed) = mpsc::channel();
        let (release, blocked) = mpsc::channel();
        let (control, waiting) = Control::new();
        let owner_cache = &cache;
        std::thread::scope(|scope| {
            let owner = scope.spawn(move || {
                owner_cache.get_or_compile(key("panic"), &NoPipelineControl, || {
                    started.send(()).unwrap();
                    let _ = blocked.recv_timeout(TIMEOUT);
                    panic!("simulated compiler unwind");
                })
            });
            observed.recv_timeout(TIMEOUT).unwrap();
            let waiter = scope.spawn(|| {
                cache.get_or_compile(key("panic"), &control, || panic!("waiter compiled"))
            });
            waiting.recv_timeout(TIMEOUT).unwrap();
            release.send(()).unwrap();
            assert!(owner.join().is_err());
            assert!(
                waiter
                    .join()
                    .unwrap()
                    .unwrap_err()
                    .to_string()
                    .contains("unwound")
            );
        });
        assert_eq!(
            cache
                .get_or_compile(key("panic"), &NoPipelineControl, || Err("retried".into()))
                .err(),
            Some(VmError::NativeJit("retried".into()))
        );
    }

    #[test]
    fn failures_are_bounded_and_evicted_keys_can_retry() {
        let cache = NativeCompileCache::new(1);
        for index in 0..=NATIVE_COMPILE_CACHE_MAX_ENTRIES {
            let _ = cache.get_or_compile(key(&index.to_string()), &NoPipelineControl, || {
                Err("failed".into())
            });
        }
        assert_eq!(
            cache.state.lock().unwrap().ready.entries.len(),
            NATIVE_COMPILE_CACHE_MAX_ENTRIES
        );
        assert_eq!(
            cache
                .get_or_compile(key("0"), &NoPipelineControl, || Err("retried".into()))
                .err(),
            Some(VmError::NativeJit("retried".into()))
        );
    }

    #[cfg(target_arch = "x86_64")]
    fn image() -> Arc<NativeModel> {
        Arc::new(NativeModel::new_for_test(2, 1, vec![1], vec![0]))
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn cancelled_owner_publishes_a_reusable_image() {
        let cache = NativeCompileCache::new(1);
        let (control, _) = Control::new();
        let compiled = image();
        assert_eq!(
            cache
                .get_or_compile(key("image"), &control, || {
                    control.cancelled.store(true, Ordering::Release);
                    Ok(Arc::clone(&compiled))
                })
                .err(),
            Some(VmError::CompilationCancelled)
        );
        let reused = cache
            .get_or_compile(key("image"), &NoPipelineControl, || panic!("recompiled"))
            .unwrap();
        assert!(Arc::ptr_eq(&compiled, &reused));
    }

    #[cfg(target_arch = "x86_64")]
    #[test]
    fn image_eviction_preserves_live_users_and_recently_used_entries() {
        let compiled = image();
        let bytes = compiled.code_size_bytes();
        assert!(bytes > 0);
        let mut cache = ReadyCache::default();
        for name in ["a", "b", "c"] {
            cache.insert(key(name), Ok(Arc::clone(&compiled)));
        }
        assert_eq!(cache.image_bytes, 3 * bytes);
        let live_user = cache.get(&key("a")).unwrap().unwrap();
        cache.evict_to(bytes);
        assert_eq!(cache.image_bytes, bytes);
        assert_eq!(cache.entries.len(), 1);
        assert!(cache.get(&key("a")).is_some());
        assert!(cache.get(&key("b")).is_none());
        cache.evict_to(0);
        assert_eq!(cache.entries.len(), 1, "retain a single oversized image");
        drop(cache);
        assert!(Arc::ptr_eq(&compiled, &live_user));
    }
}
