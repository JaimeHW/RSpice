//! The PDK and model-library Verilog-A runtimes this process compiled, by the
//! sealed identity a deck names them by.
//!
//! Spectre keeps what it compiles from an `ahdl_include` source in a cache
//! keyed by the source's checksum, and a later run over the same source
//! reuses it. Here every preparation that compiles one of these sources (a
//! preflight, a run, a manual deck's validation, each campaign member) reads
//! and fills one cache, and [`warm`] compiles a project's sources on a
//! background thread as soon as a frame names them. The frame that then
//! validates a manual deck or queues a campaign reads the runtimes instead of
//! compiling them, and every member of a campaign shares one compile. A
//! preparation that asks while another thread compiles the same source waits
//! for that compile rather than starting a second one. One that a frame runs
//! under [`unless_compiling`] stops there instead, so the frame goes on and
//! asks again once [`compiling`] says the compile has ended.
//!
//! A compile that fails is not kept: the next preparation compiles the source
//! again and refuses it with its own diagnostic. The cache holds the sources
//! the latest [`warm`] named and what was compiled since; a warm naming other
//! sources (another project, an edited library) drops the rest.
//!
//! The browser has one thread, which compiles what it needs when it needs it.

use crate::model_sources::{SealedModelExecutionSources, SealedModelLibraryVerilogAAuthority};
use crate::veriloga::{PreparedRuntimeError, PreparedVerilogARuntime, PreparedVerilogARuntimeSet};

/// A PDK source's runtime, compiled once per process.
pub(crate) fn pdk_runtime(
    package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
    archive_digest: rspice_vocabulary::product::ContentDigest,
    artifacts: &[crate::pdk::SealedPdkVerilogAArtifact],
    binding: &crate::pdk::SealedPdkVerilogABinding,
) -> Result<PreparedVerilogARuntime, PreparedRuntimeError> {
    // The sealed bytes are checked on every call: a compiled runtime stands
    // for these bytes only.
    let bundle = crate::veriloga::verified_pdk_source_bundle(artifacts, binding)?;
    let compile = || {
        crate::veriloga::compile_verified_pdk_source_runtime(
            package,
            archive_digest,
            binding,
            &bundle,
        )
    };
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::compiled(native::Key::pdk(package, archive_digest, binding), || {
            compile().map(|runtime| native::Runtimes::Pdk(Box::new(runtime)))
        })
        .map(native::Runtimes::into_pdk)
    }
    #[cfg(target_arch = "wasm32")]
    {
        compile()
    }
}

/// A model library's runtimes, compiled once per process.
pub(crate) fn model_library_runtimes(
    authority: &SealedModelLibraryVerilogAAuthority,
) -> Result<PreparedVerilogARuntimeSet, PreparedRuntimeError> {
    let compile = || crate::project_veriloga::compile_model_library_source_runtimes(authority);
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::compiled(native::Key::model_library(authority), || {
            compile().map(native::Runtimes::ModelLibrary)
        })
        .map(native::Runtimes::into_model_library)
    }
    #[cfg(target_arch = "wasm32")]
    {
        compile()
    }
}

/// Compile, on a background thread, the PDK and reference model-library
/// sources a run of `sealed_models` installs that this process has not
/// compiled, and forget every other source. Returns at once, and copies no
/// source when every one is compiled or compiling.
pub fn warm(
    sealed_models: &SealedModelExecutionSources,
    reference_process: rspice_vocabulary::product::ProcessCorner,
) {
    #[cfg(not(target_arch = "wasm32"))]
    native::warm(sealed_models, reference_process);
    #[cfg(target_arch = "wasm32")]
    let _ = (sealed_models, reference_process);
}

/// Whether a source this thread's preparations read is compiling on another
/// thread: one the latest [`warm`] on this thread named, or one an
/// [`unless_compiling`] preparation stopped at. A preparation started now
/// would wait for it.
pub fn compiling() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::compiling()
    }
    #[cfg(target_arch = "wasm32")]
    {
        false
    }
}

/// Run `prepare`, which must not wait for a source another thread is
/// compiling: it returns `None` when it reached one, refused there instead of
/// waiting, and [`compiling`] then holds on this thread until that compile
/// ends. A source no thread is compiling compiles on this one, as it would
/// outside.
pub fn unless_compiling<R>(prepare: impl FnOnce() -> R) -> Option<R> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        native::unless_compiling(prepare)
    }
    #[cfg(target_arch = "wasm32")]
    {
        Some(prepare())
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::cell::{Cell, RefCell};
    use std::collections::HashMap;
    use std::sync::{Arc, Condvar, Mutex, MutexGuard};

    use super::*;

    /// What a source is compiled from, in full: equal keys compile to equal
    /// runtimes.
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    pub(super) enum Key {
        Pdk {
            /// Names the package manifest, the archive, the source and the
            /// root's digest.
            source_key: String,
            module_name: String,
            netlist_alias: String,
        },
        ModelLibrary {
            /// Names every source of the library's closure.
            closure: rspice_vocabulary::product::ContentDigest,
            /// Each root: its path, alias and selected module.
            roots: Vec<(std::path::PathBuf, Option<String>, Option<String>)>,
        },
    }

    impl Key {
        pub(super) fn pdk(
            package: &rspice_model_library::pdk::contracts::PdkTechnologyBinding,
            archive_digest: rspice_vocabulary::product::ContentDigest,
            binding: &crate::pdk::SealedPdkVerilogABinding,
        ) -> Self {
            Self::Pdk {
                source_key: crate::veriloga::pdk_source_name(package, archive_digest, binding)
                    .source_key,
                module_name: binding.module_name.clone(),
                netlist_alias: binding.netlist_alias.clone(),
            }
        }

        pub(super) fn model_library(authority: &SealedModelLibraryVerilogAAuthority) -> Self {
            Self::ModelLibrary {
                closure: authority.closure_digest(),
                roots: authority
                    .roots()
                    .iter()
                    .map(|root| {
                        (
                            root.path.clone(),
                            root.netlist_alias.clone(),
                            root.selected_module.clone(),
                        )
                    })
                    .collect(),
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(super) enum Runtimes {
        Pdk(Box<PreparedVerilogARuntime>),
        ModelLibrary(PreparedVerilogARuntimeSet),
    }

    impl Runtimes {
        pub(super) fn into_pdk(self) -> PreparedVerilogARuntime {
            match self {
                Self::Pdk(runtime) => *runtime,
                Self::ModelLibrary(_) => unreachable!("a PDK key holds a PDK runtime"),
            }
        }

        pub(super) fn into_model_library(self) -> PreparedVerilogARuntimeSet {
            match self {
                Self::ModelLibrary(runtimes) => runtimes,
                Self::Pdk(_) => unreachable!("a model-library key holds model-library runtimes"),
            }
        }
    }

    enum Slot {
        Compiled(Runtimes),
        Compiling(Arc<Compiling>),
    }

    /// A compile one thread runs, which a thread that needs the same source
    /// waits on.
    #[derive(Default)]
    struct Compiling {
        ended: Mutex<bool>,
        ending: Condvar,
    }

    impl Compiling {
        fn wait(&self) {
            let mut ended = self
                .ended
                .lock()
                .unwrap_or_else(|poison| poison.into_inner());
            while !*ended {
                ended = self
                    .ending
                    .wait(ended)
                    .unwrap_or_else(|poison| poison.into_inner());
            }
        }

        fn end(&self) {
            *self
                .ended
                .lock()
                .unwrap_or_else(|poison| poison.into_inner()) = true;
            self.ending.notify_all();
        }
    }

    type Slots = HashMap<Key, Slot>;

    static CACHE: Mutex<Option<Slots>> = Mutex::new(None);

    fn lock() -> MutexGuard<'static, Option<Slots>> {
        CACHE.lock().unwrap_or_else(|poison| poison.into_inner())
    }

    /// A thread's claim to compile `key`. Dropping it keeps what the compile
    /// produced, when it produced something, and in every case, a panic
    /// included, wakes the threads waiting on it.
    struct Claim {
        key: Key,
        compiling: Arc<Compiling>,
        compiled: Option<Runtimes>,
    }

    impl Claim {
        /// End the compile, keeping `compiled` when there is one.
        fn end(mut self, compiled: Option<Runtimes>) {
            self.compiled = compiled;
        }

        /// Claim `key`, which `slots` does not hold.
        fn insert(slots: &mut Slots, key: Key) -> Self {
            let compiling = Arc::new(Compiling::default());
            slots.insert(key.clone(), Slot::Compiling(Arc::clone(&compiling)));
            Self {
                key,
                compiling,
                compiled: None,
            }
        }
    }

    impl Drop for Claim {
        fn drop(&mut self) {
            let mut cache = lock();
            let slots = cache.get_or_insert_with(HashMap::new);
            let ours = matches!(
                slots.get(&self.key),
                Some(Slot::Compiling(compiling)) if Arc::ptr_eq(compiling, &self.compiling)
            );
            if ours {
                match self.compiled.take() {
                    Some(runtimes) => {
                        slots.insert(self.key.clone(), Slot::Compiled(runtimes));
                    }
                    None => {
                        slots.remove(&self.key);
                    }
                }
            }
            drop(cache);
            self.compiling.end();
        }
    }

    thread_local! {
        /// The sources this thread's preparations read that may still be
        /// compiling: those its latest warm named and those a preparation
        /// stopped at.
        static WATCHED: RefCell<Vec<Key>> = const { RefCell::new(Vec::new()) };
        /// Whether this thread's preparation refuses a source another thread
        /// is compiling instead of waiting, and whether it has refused one.
        static UNLESS_COMPILING: Cell<Option<bool>> = const { Cell::new(None) };
    }

    pub(super) fn compiling() -> bool {
        WATCHED.with_borrow_mut(|watched| {
            if watched.is_empty() {
                return false;
            }
            let cache = lock();
            watched.retain(|key| {
                matches!(
                    cache.as_ref().and_then(|slots| slots.get(key)),
                    Some(Slot::Compiling(_))
                )
            });
            !watched.is_empty()
        })
    }

    pub(super) fn unless_compiling<R>(prepare: impl FnOnce() -> R) -> Option<R> {
        let outer = UNLESS_COMPILING.replace(Some(false));
        let prepared = prepare();
        let stopped = UNLESS_COMPILING.replace(outer) == Some(true);
        if stopped {
            // An enclosing preparation stopped too.
            if outer.is_some() {
                UNLESS_COMPILING.set(Some(true));
            }
            None
        } else {
            Some(prepared)
        }
    }

    /// Claim `key` as a compile in flight until the returned guard drops,
    /// which ends it with nothing compiled.
    #[cfg(test)]
    pub(super) fn hold_compile(key: Key) -> impl Drop {
        let mut cache = lock();
        let claim = Claim::insert(cache.get_or_insert_with(HashMap::new), key);
        drop(cache);
        claim
    }

    pub(super) fn compiled(
        key: Key,
        compile: impl FnOnce() -> Result<Runtimes, PreparedRuntimeError>,
    ) -> Result<Runtimes, PreparedRuntimeError> {
        let claim = loop {
            let mut cache = lock();
            let slots = cache.get_or_insert_with(HashMap::new);
            match slots.get(&key) {
                Some(Slot::Compiled(runtimes)) => return Ok(runtimes.clone()),
                Some(Slot::Compiling(_)) if UNLESS_COMPILING.get().is_some() => {
                    drop(cache);
                    UNLESS_COMPILING.set(Some(true));
                    WATCHED.with_borrow_mut(|watched| {
                        if !watched.contains(&key) {
                            watched.push(key);
                        }
                    });
                    return Err(PreparedRuntimeError::Compile(
                        "the source is still compiling on another thread".to_owned(),
                    ));
                }
                Some(Slot::Compiling(compiling)) => {
                    let compiling = Arc::clone(compiling);
                    drop(cache);
                    compiling.wait();
                }
                None => break Claim::insert(slots, key),
            }
        };
        let result = compile();
        claim.end(result.as_ref().ok().cloned());
        result
    }

    /// What a warm compiles.
    enum Source {
        Pdk {
            package: rspice_model_library::pdk::contracts::PdkTechnologyBinding,
            archive_digest: rspice_vocabulary::product::ContentDigest,
            artifacts: Arc<[crate::pdk::SealedPdkVerilogAArtifact]>,
            binding: crate::pdk::SealedPdkVerilogABinding,
        },
        ModelLibrary(SealedModelLibraryVerilogAAuthority),
    }

    impl Source {
        fn compile(&self) -> Result<Runtimes, PreparedRuntimeError> {
            match self {
                Self::Pdk {
                    package,
                    archive_digest,
                    artifacts,
                    binding,
                } => crate::veriloga::compile_pdk_source_runtime(
                    package,
                    *archive_digest,
                    artifacts,
                    binding,
                )
                .map(|runtime| Runtimes::Pdk(Box::new(runtime))),
                Self::ModelLibrary(authority) => {
                    crate::project_veriloga::compile_model_library_source_runtimes(authority)
                        .map(Runtimes::ModelLibrary)
                }
            }
        }
    }

    pub(super) fn warm(
        sealed_models: &SealedModelExecutionSources,
        reference_process: rspice_vocabulary::product::ProcessCorner,
    ) {
        let pdk = sealed_models.pdk_veriloga_authority();
        let library = sealed_models
            .reference_model_library_veriloga_authority(reference_process)
            .ok()
            .flatten();
        let mut named = Vec::new();
        if let Some((package, archive_digest, _, bindings)) = pdk {
            named.extend(
                bindings
                    .iter()
                    .map(|binding| (Key::pdk(package, archive_digest, binding), Some(binding))),
            );
        }
        named.extend(
            library
                .as_ref()
                .map(|authority| (Key::model_library(authority), None)),
        );

        WATCHED.with_borrow_mut(|watched| {
            watched.clear();
            watched.extend(named.iter().map(|(key, _)| key.clone()));
        });
        // The sources not compiled are claimed here, before the thread
        // starts, so a preparation that needs one in the meantime waits for
        // the thread instead of compiling its own copy.
        let mut claimed = Vec::new();
        {
            let mut cache = lock();
            let slots = cache.get_or_insert_with(HashMap::new);
            slots.retain(|key, slot| {
                matches!(slot, Slot::Compiling(_)) || named.iter().any(|(named, _)| named == key)
            });
            for (key, binding) in named {
                if slots.contains_key(&key) {
                    continue;
                }
                let source = match (binding, pdk, &library) {
                    (Some(binding), Some((package, archive_digest, artifacts, _)), _) => {
                        Source::Pdk {
                            package: package.clone(),
                            archive_digest,
                            artifacts: Arc::from(artifacts),
                            binding: binding.clone(),
                        }
                    }
                    (None, _, Some(authority)) => Source::ModelLibrary(authority.clone()),
                    _ => continue,
                };
                claimed.push((Claim::insert(slots, key), source));
            }
        }
        if claimed.is_empty() {
            return;
        }
        // A source that fails to compile is refused, with its diagnostic, by
        // the preparation that next needs it; nothing here reports it. When
        // no thread starts, the claims are dropped unfulfilled and that
        // preparation compiles.
        let _ = std::thread::Builder::new()
            .name("rspice-veriloga-warm".to_owned())
            .spawn(move || {
                for (claim, source) in claimed {
                    claim.end(source.compile().ok());
                }
            });
    }
}

#[cfg(test)]
mod tests;
