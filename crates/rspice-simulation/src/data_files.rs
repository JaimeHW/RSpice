//! The data files XSPICE code models read, sealed into a prepared run.
//!
//! Preparation finds every code-model parameter that names a data file
//! (`d_source` `input_file`, `d_vcd` `file`, the tables' `file`, ...) through
//! the engine's own parameter resolution, reads each file once under the run's
//! resource limits and binds its digest into the run's identity. The bytes
//! travel with the run, and whatever solves it registers them under the path
//! the model asks for, so the solve reads exactly what preparation read, with
//! or without a file system.
//!
//! A prepared run's deck is solved without a source path, so a model asks for
//! its path as the deck spells it. Preparation reads a relative path from the
//! root deck's directory, as a deck run from its own file resolves it.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, MutexGuard, OnceLock, PoisonError};

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::CodeModelDataFile;
use rspice_core::{ResourceKind, ResourceLimitError, ResourceLimits};
use rspice_vocabulary::product::ContentDigest;
use serde::{Deserialize, Serialize};

use crate::preparation::{PreparationError, PreparationStage};

/// One file's bytes under the path its code model reads.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct SealedDataFile {
    /// The path exactly as the code model hands it to the data-file loader.
    path: String,
    contents: String,
    digest: ContentDigest,
    /// The host file preparation read. It names no part of the run, and a
    /// worker never receives it.
    #[serde(skip)]
    read_from: Option<PathBuf>,
}

impl PartialEq for SealedDataFile {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path && self.digest == other.digest && self.contents == other.contents
    }
}

impl Eq for SealedDataFile {}

impl std::fmt::Debug for SealedDataFile {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("SealedDataFile")
            .field("path", &self.path)
            .field("bytes", &self.contents.len())
            .field("digest", &self.digest)
            .finish()
    }
}

/// The code-model data files one prepared run reads, in path order.
///
/// Shared, not copied, by every task of the run. Deserialized files are
/// checked by [`Self::validate`] before a host registers them.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct PreparedDataFiles {
    files: Arc<[SealedDataFile]>,
}

impl Serialize for PreparedDataFiles {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Encoded<'a> {
            files: &'a [SealedDataFile],
        }
        Encoded { files: &self.files }.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for PreparedDataFiles {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Encoded {
            files: Vec<SealedDataFile>,
        }
        Encoded::deserialize(deserializer).map(|encoded| Self {
            files: encoded.files.into(),
        })
    }
}

impl PreparedDataFiles {
    pub(crate) fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    /// Find and read every data file `deck`'s code models name.
    ///
    /// A relative path is read from `root`, the root deck's directory. With
    /// no root, a deck that is not a file of its own or that came sealed
    /// without a file system, any data file is refused. A file that cannot be
    /// read is refused, naming the instance, the parameter and the path.
    pub(crate) fn capture(
        deck: &str,
        root: Option<&Path>,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, PreparationError> {
        let mut files = BTreeMap::<String, SealedDataFile>::new();
        let mut total = 0usize;
        for named in discover(deck, limits, abort)? {
            PreparationError::check_abort(abort)?;
            if files.contains_key(&named.path) {
                continue;
            }
            let Some(root) = root else {
                return Err(PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!(
                        "{}, and this deck has no directory of its own to read it from",
                        describe(&named)
                    ),
                ));
            };
            let host = root.join(&named.path);
            let (contents, _) = rspice_core::resource::read_utf8_file_limited_with_metadata(
                &host,
                ResourceKind::ExternalDataBytes,
                limits.max_external_data_bytes,
            )
            .map_err(|error| {
                let context = format!("{}, read from {}", describe(&named), host.display());
                match error {
                    rspice_core::resource::ResourceReadError::ResourceLimit(error) => {
                        PreparationError::from_parse(
                            PreparationStage::SourceChecks,
                            &context,
                            rspice_core::netlist::ParseError::ResourceLimit(error).into(),
                        )
                    }
                    rspice_core::resource::ResourceReadError::Io(error) => PreparationError::new(
                        PreparationStage::SourceChecks,
                        format!("{context}, cannot be read: {error}"),
                    ),
                }
            })?;
            total = total.saturating_add(contents.len());
            PreparationError::check_limit(
                ResourceKind::DependencySourceBytes,
                total,
                limits.max_dependency_source_bytes,
            )?;
            files.insert(
                named.path.clone(),
                SealedDataFile {
                    digest: file_digest(&contents),
                    path: named.path,
                    contents,
                    read_from: Some(host),
                },
            );
        }
        Ok(Self {
            files: files.into_values().collect(),
        })
    }

    /// Refuse a deck a task solves whose code models name a file this run
    /// did not seal: a task's own deck can name one the run's deck does not.
    pub(crate) fn cover(
        &self,
        deck: &str,
        limits: ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), PreparationError> {
        for named in discover(deck, limits, abort)? {
            if !self.files.iter().any(|file| file.path == named.path) {
                return Err(PreparationError::new(
                    PreparationStage::SourceChecks,
                    format!(
                        "Executable netlist contains an unsealed external dependency (code-model data file): {}",
                        describe(&named)
                    ),
                ));
            }
        }
        Ok(())
    }

    /// Check carried files against their digests and the run's limits.
    pub(crate) fn validate(&self, limits: ResourceLimits) -> Result<(), String> {
        let mut paths = HashSet::new();
        let mut total = 0usize;
        for file in self.files.iter() {
            if file.path.is_empty() {
                return Err("A sealed code-model data file has no path".into());
            }
            if !paths.insert(file.path.as_str()) {
                return Err(format!(
                    "Code-model data file '{}' is sealed twice",
                    file.path
                ));
            }
            if file.digest != file_digest(&file.contents) {
                return Err(format!(
                    "Sealed code-model data file '{}' does not match its digest",
                    file.path
                ));
            }
            ResourceLimitError::ensure(
                ResourceKind::ExternalDataBytes,
                file.contents.len(),
                limits.max_external_data_bytes,
            )
            .map_err(|error| error.to_string())?;
            total = total.saturating_add(file.contents.len());
        }
        ResourceLimitError::ensure(
            ResourceKind::DependencySourceBytes,
            total,
            limits.max_dependency_source_bytes,
        )
        .map_err(|error| error.to_string())
    }

    /// What the run's identity binds: each path and the digest of its bytes.
    pub(crate) fn digest(&self) -> ContentDigest {
        let entries = self
            .files
            .iter()
            .map(|file| (file.path.as_str(), file.digest))
            .collect::<Vec<_>>();
        rspice_vocabulary::canonical::content_digest(
            "rspice.prepared-code-model-data-files/v1",
            &serde_json::to_vec(&entries).expect("data-file entries serialize"),
        )
    }

    /// The host files preparation read, for a host that must not overwrite
    /// them.
    pub(crate) fn host_paths(&self) -> Vec<PathBuf> {
        self.files
            .iter()
            .filter_map(|file| file.read_from.clone())
            .collect()
    }

    /// Register the files for the code models of a run that solves while
    /// the hold lasts.
    ///
    /// Runs in one process share the registry, so a path one run holds is
    /// held again by a run with the same bytes, and refused to a run with
    /// different ones. The last hold to drop unregisters the file.
    pub(crate) fn hold(&self, limits: ResourceLimits) -> Result<HeldDataFiles, String> {
        if self.files.is_empty() {
            return Ok(HeldDataFiles { paths: Vec::new() });
        }
        self.validate(limits)?;
        let mut leases = leases();
        let mut paths = Vec::with_capacity(self.files.len());
        for file in self.files.iter() {
            if let Err(error) = acquire(&mut leases, file, limits) {
                release(&mut leases, &paths);
                return Err(error);
            }
            paths.push(file.path.clone());
        }
        Ok(HeldDataFiles { paths })
    }
}

/// A run's data files, registered until this drops.
#[must_use = "the files are unregistered when the hold drops"]
pub(crate) struct HeldDataFiles {
    paths: Vec<String>,
}

impl Drop for HeldDataFiles {
    fn drop(&mut self) {
        if !self.paths.is_empty() {
            release(&mut leases(), &self.paths);
        }
    }
}

struct Lease {
    digest: ContentDigest,
    holders: usize,
}

fn leases() -> MutexGuard<'static, HashMap<String, Lease>> {
    static LEASES: OnceLock<Mutex<HashMap<String, Lease>>> = OnceLock::new();
    LEASES
        .get_or_init(Default::default)
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
}

fn acquire(
    leases: &mut HashMap<String, Lease>,
    file: &SealedDataFile,
    limits: ResourceLimits,
) -> Result<(), String> {
    match leases.get_mut(&file.path) {
        Some(lease) if lease.digest == file.digest => {
            lease.holders += 1;
            Ok(())
        }
        Some(_) => Err(format!(
            "Code-model data file '{}' is held by another running simulation with different contents",
            file.path
        )),
        None => {
            rspice_core::xspice::register_data_file_with_limits(
                file.path.clone(),
                file.contents.clone(),
                limits,
            )?;
            leases.insert(
                file.path.clone(),
                Lease {
                    digest: file.digest,
                    holders: 1,
                },
            );
            Ok(())
        }
    }
}

fn release(leases: &mut HashMap<String, Lease>, paths: &[String]) {
    for path in paths {
        let Some(lease) = leases.get_mut(path) else {
            continue;
        };
        lease.holders -= 1;
        if lease.holders == 0 {
            leases.remove(path);
            // Unregistering only fails when the registry's generation counter
            // is exhausted, and then nothing can be registered either.
            let _ = rspice_core::xspice::unregister_data_file(path);
        }
    }
}

fn file_digest(contents: &str) -> ContentDigest {
    rspice_vocabulary::canonical::content_digest(
        "rspice.code-model-data-file/v1",
        contents.as_bytes(),
    )
}

fn describe(named: &CodeModelDataFile) -> String {
    format!(
        "instance '{}' reads {}=\"{}\"",
        named.instance, named.parameter, named.path
    )
}

/// The code models with a parameter that names a data file, lowercased.
fn data_file_models() -> &'static [String] {
    static MODELS: OnceLock<Vec<String>> = OnceLock::new();
    MODELS.get_or_init(|| {
        let registry = rspice_core::xspice::CodeModelRegistry::with_builtins();
        registry
            .model_names()
            .into_iter()
            .filter(|name| {
                registry
                    .get(name)
                    .is_some_and(|model| !model.input_data_file_parameters().is_empty())
            })
            .map(str::to_ascii_lowercase)
            .collect()
    })
}

/// The data files `deck`'s code models name, as the engine resolves them
/// when it solves the deck: without a source path.
fn discover(
    deck: &str,
    limits: ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<Vec<CodeModelDataFile>, PreparationError> {
    // A code model is named by its type on a `.model` card, so a deck that
    // spells no such type needs no parse.
    let lower = deck.to_ascii_lowercase();
    if !data_file_models()
        .iter()
        .any(|model| lower.contains(model.as_str()))
    {
        return Ok(Vec::new());
    }
    let netlist = rspice_core::Netlist::parse_with_options_and_abort(
        deck,
        rspice_core::netlist::NetlistParseOptions {
            resource_limits: limits,
            ..Default::default()
        },
        abort,
    )
    .map_err(|error| {
        PreparationError::from_parse(
            PreparationStage::SourceChecks,
            "Executable source cannot name its code-model data files",
            error,
        )
    })?;
    rspice_core::Engine::new(rspice_core::SimulationConfig {
        resource_limits: limits,
        ..Default::default()
    })
    .code_model_data_files_with_abort(&netlist, abort)
    .map_err(|error| {
        PreparationError::from_simulation(
            PreparationStage::SourceChecks,
            "Executable source cannot name its code-model data files",
            error.into(),
        )
    })
}

// Each test writes its deck and data files to a host directory and prepares
// it through the native resolver, which a browser build does not have.
#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests;
