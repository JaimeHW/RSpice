//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod catalog_identity;
mod execution_sources;
mod project_models;
mod sealing;
mod source_bundle;

pub(crate) use catalog_identity::model_library_source_digest;
pub use execution_sources::SealedModelExecutionSources;
pub(crate) use execution_sources::SealedModelLibraryVerilogAAuthority;
use rspice_model_library::source_paths::portable_path_key;

use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use rspice_core::library::SpiceLibraryIndex;
#[cfg(test)]
use rspice_model_library::ModelValidationFindingSeverity;
use rspice_model_library::{
    MODEL_RESOLUTION_RECORD_SCHEMA_VERSION, ModelConsumerScope, ModelResolutionRecord,
    ModelValidationFinding, ModelValidationReceipt, ModelValidationReceiptInput,
    SimulationPlanModelBinding,
};

use super::{
    DeviceModel, ModelCorrelationState, ModelLibrary, ModelQualificationState,
    ModelSourceAuthority, ModelType, ProcessCorner, ProjectModelRevisionDefinition,
};
#[cfg(test)]
use super::{
    ModelLevel, ModelSourceEdge, ModelSourceEvidenceBinding, ModelSourcePin, ProjectModelDefinition,
};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
#[cfg(test)]
use rspice_app_types::product::ProcessCorner as CornerProcess;
#[cfg(test)]
use rspice_model_library::CornerModelBinding;

pub use rspice_model_library::ProjectModelCommit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelDefinitionProvider {
    pub library: String,
    pub definition: String,
    pub source_digest: ContentDigest,
}

const fn model_validation_platform() -> &'static str {
    if cfg!(target_arch = "wasm32") {
        "browser-wasm32"
    } else if cfg!(target_os = "windows") {
        "desktop-windows"
    } else if cfg!(target_os = "macos") {
        "desktop-macos"
    } else if cfg!(target_os = "linux") {
        "desktop-linux"
    } else {
        "desktop-unsupported"
    }
}

fn hash_validation_source_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

/// Manager for all model libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelLibraryManager {
    /// All libraries
    #[serde(rename = "libraries")]
    catalog: rspice_model_library::ModelCatalog,
    /// Currently selected library
    pub selected_library: Option<String>,
    /// Search filter
    pub filter_text: String,
    /// Filter by model type
    pub filter_type: Option<ModelType>,
    /// Durable project decisions for contested executable definitions. The
    /// map key is the canonical `scope:name` identity repeated by each value.
    #[serde(default)]
    resolution_records: BTreeMap<String, ModelResolutionRecord>,
    #[serde(default)]
    validation_receipt: Option<ModelValidationReceipt>,
    /// Index over the shipped model packs, when one was found on disk.
    ///
    /// Held rather than loaded: the packs carry around 199,000 definitions, so
    /// materializing them as `DeviceModel`s would cost far more memory than the
    /// catalogue view needs. Queries stream the on-disk index instead.
    ///
    /// Not serialized. It is a view of what is installed on this machine, so it
    /// is rediscovered on load rather than restored from a project file that may
    /// have been written elsewhere.
    #[serde(skip)]
    spice_packs: Option<Arc<SpiceLibraryIndex>>,
}

/// One definition found in the shipped packs rather than in a loaded library.
///
/// Deliberately not a [`DeviceModel`]: nothing here has been parsed, and
/// presenting an unparsed catalogue row as a loaded model would overstate what
/// the application knows about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackModelHit {
    /// Definition name as written in the source.
    pub name: String,
    /// `model` or `subckt`.
    pub kind: String,
    /// Canonical device class, such as `diode` or `mosfet-n`.
    pub device: String,
    /// Owning pack identifier.
    pub pack: String,
    /// Human-readable pack title.
    pub pack_name: String,
    /// Absolute path to the defining file, when the pack is known.
    pub source: Option<PathBuf>,
    /// 1-based line of the definition.
    pub line: usize,
    /// Whether RSpice has established the right to redistribute the pack.
    pub redistributable: bool,
    /// The individual source file is excluded from redistribution even when
    /// other files in the same pack are shippable.
    pub restricted: bool,
}

impl ModelLibraryManager {
    #[must_use]
    pub fn model_validation_receipt(&self) -> Option<&ModelValidationReceipt> {
        self.validation_receipt.as_ref()
    }

    pub(crate) fn invalidate_model_validation_receipt(&mut self) {
        self.validation_receipt = None;
    }

    pub(crate) fn restore_model_validation_receipt(
        &mut self,
        receipt: Option<ModelValidationReceipt>,
    ) -> Result<(), String> {
        if let Some(receipt) = receipt.as_ref() {
            receipt.verify()?;
        }
        self.validation_receipt = receipt;
        Ok(())
    }

    pub(crate) fn issue_model_validation_receipt(
        &mut self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
        findings: Vec<ModelValidationFinding>,
    ) -> Result<ModelValidationReceipt, String> {
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        let receipt = ModelValidationReceipt::issue(ModelValidationReceiptInput {
            project_revision,
            model_execution_plan_digest: plan_digest,
            execution_catalog_digest: self.execution_catalog_digest(),
            source_count,
            source_closure_digest,
            pdk_archive_digest,
            engine_version: env!("CARGO_PKG_VERSION").to_owned(),
            execution_schema_version,
            platform: model_validation_platform().to_owned(),
            findings,
            validated_at_unix_ms: crate::time_compat::checked_unix_time_ms().map_err(|error| {
                format!("system clock cannot timestamp model validation: {error}")
            })?,
        })?;
        self.validation_receipt = Some(receipt.clone());
        Ok(receipt)
    }

    pub(crate) fn validate_model_validation_receipt(
        &self,
        project_revision: ObjectRevision,
        plan_digest: ContentDigest,
        pdk_archive_digest: Option<ContentDigest>,
        execution_schema_version: u32,
    ) -> Result<&ModelValidationReceipt, String> {
        let receipt = self.validation_receipt.as_ref().ok_or_else(|| {
            "No durable model-validation receipt exists for this project revision.".to_owned()
        })?;
        receipt.verify()?;
        if receipt.project_revision != project_revision {
            return Err(
                "Model-validation receipt is stale after a project revision change.".to_owned(),
            );
        }
        if receipt.model_execution_plan_digest != plan_digest {
            return Err(
                "Model-validation receipt is stale after the execution plan changed.".to_owned(),
            );
        }
        if receipt.execution_catalog_digest != self.execution_catalog_digest() {
            return Err(
                "Model-validation receipt is stale after the model catalog changed.".to_owned(),
            );
        }
        if receipt.pdk_archive_digest != pdk_archive_digest {
            return Err(
                "Model-validation receipt is stale after the signed PDK changed.".to_owned(),
            );
        }
        if receipt.execution_schema_version != execution_schema_version
            || receipt.engine_version != env!("CARGO_PKG_VERSION")
            || receipt.platform != model_validation_platform()
        {
            return Err(
                "Model-validation receipt was produced by a different engine, schema, or platform."
                    .to_owned(),
            );
        }
        let (source_count, source_closure_digest) = self.model_validation_source_identity();
        if receipt.source_count != source_count
            || receipt.source_closure_digest != source_closure_digest
        {
            return Err(
                "Model-validation receipt is stale after source digests changed.".to_owned(),
            );
        }
        Ok(receipt)
    }

    fn model_validation_source_identity(&self) -> (u64, ContentDigest) {
        let mut identities = self
            .libraries_sorted()
            .into_iter()
            .flat_map(|library| {
                library
                    .source_closure
                    .iter()
                    .map(move |source| (library.name.clone(), source.digest.to_string()))
            })
            .collect::<Vec<_>>();
        identities.sort();
        let source_count = identities.len() as u64;
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-validation-source-closure/v1\0");
        for (library, digest) in identities {
            hash_validation_source_field(&mut hasher, library.as_bytes());
            hash_validation_source_field(&mut hasher, digest.as_bytes());
        }
        (
            source_count,
            ContentDigest::from_bytes(hasher.finalize().into()),
        )
    }

    #[must_use]
    pub fn model_resolution_record(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Option<&ModelResolutionRecord> {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records.get(&scope.record_key(&normalized))
    }

    pub(crate) fn restore_model_resolution_records(
        &mut self,
        records: Vec<ModelResolutionRecord>,
    ) -> Result<(), String> {
        let mut restored = BTreeMap::new();
        for record in records {
            record.validate()?;
            let key = record.key();
            if restored.insert(key.clone(), record).is_some() {
                return Err(format!("model-resolution record '{key}' is repeated"));
            }
        }
        self.resolution_records = restored;
        self.validate_model_resolution_records_against_catalog()
    }

    #[must_use]
    pub(crate) fn owned_model_resolution_records(&self) -> Vec<ModelResolutionRecord> {
        self.resolution_records.values().cloned().collect()
    }

    pub(crate) fn definition_providers(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Vec<ModelDefinitionProvider> {
        let normalized = definition.trim().to_ascii_lowercase();
        let mut providers = Vec::new();
        for library in self.libraries_sorted() {
            let active_sections = library.active_section_names();
            let names = match scope {
                ModelConsumerScope::PrimitiveModel => library
                    .models
                    .values()
                    .map(|model| model.name.as_str())
                    .collect::<Vec<_>>(),
                ModelConsumerScope::Subcircuit => library
                    .subcircuits
                    .values()
                    .filter(|subcircuit| {
                        subcircuit.section.as_deref().is_none_or(|section| {
                            active_sections
                                .iter()
                                .any(|active| active.eq_ignore_ascii_case(section))
                        })
                    })
                    .map(|subcircuit| subcircuit.name.as_str())
                    .collect::<Vec<_>>(),
            };
            for exact_name in names
                .into_iter()
                .filter(|name| name.to_ascii_lowercase() == normalized)
            {
                providers.push(ModelDefinitionProvider {
                    library: library.name.clone(),
                    definition: exact_name.to_owned(),
                    source_digest: model_library_source_digest(library),
                });
            }
        }
        providers.sort_by(|left, right| {
            left.library
                .cmp(&right.library)
                .then_with(|| left.definition.cmp(&right.definition))
                .then_with(|| {
                    left.source_digest
                        .to_string()
                        .cmp(&right.source_digest.to_string())
                })
        });
        providers
    }

    /// Resolve the one provider the flat executable SPICE namespace will use.
    ///
    /// Component properties may retain a library name for provenance, but that
    /// metadata is not an independent namespace selector. Every UI surface must
    /// consult this method so Properties, catalog binding, and the sealed run
    /// plan agree with the same project-global provider decision.
    pub(crate) fn effective_definition_provider(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        let providers = self.definition_providers(scope, definition);
        match providers.as_slice() {
            [] => Ok(None),
            [provider] => Ok(Some(provider.clone())),
            _ => {
                let normalized_name = definition.trim().to_ascii_lowercase();
                let Some(record) = self.model_resolution_record(scope, &normalized_name) else {
                    return Err(format!(
                        "{} '{}' has {} executable providers ({}); resolve the project-global provider before binding or editing an instance",
                        scope.label(),
                        definition.trim(),
                        providers.len(),
                        providers
                            .iter()
                            .map(|provider| provider.library.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ));
                };
                let winners = providers
                    .into_iter()
                    .filter(|provider| {
                        provider.library == record.provider_library
                            && provider.definition == record.provider_definition
                            && provider.source_digest == record.provider_source_digest
                    })
                    .collect::<Vec<_>>();
                match winners.as_slice() {
                    [provider] => Ok(Some(provider.clone())),
                    [] => Err(format!(
                        "project-global provider decision for {} '{}' no longer matches an authenticated catalog definition",
                        scope.label(),
                        definition.trim()
                    )),
                    _ => Err(format!(
                        "{} '{}' is repeated inside resolved provider '{}'; repair that source before binding an instance",
                        scope.label(),
                        definition.trim(),
                        record.provider_library
                    )),
                }
            }
        }
    }

    pub fn resolve_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
        provider_library: &str,
        audit_reason: &str,
    ) -> Result<ModelResolutionRecord, String> {
        let normalized_name = definition.trim().to_ascii_lowercase();
        let providers = self.definition_providers(scope, &normalized_name);
        if providers.len() < 2 {
            return Err(format!(
                "{} definition '{}' is not contested by multiple authenticated providers",
                scope.label(),
                definition.trim()
            ));
        }
        let provider = providers
            .iter()
            .find(|provider| provider.library == provider_library)
            .ok_or_else(|| {
                format!(
                    "'{provider_library}' is not an exact provider of contested {} '{}'",
                    scope.label(),
                    normalized_name
                )
            })?;
        if providers.iter().any(|candidate| {
            candidate != provider
                && candidate.library.eq_ignore_ascii_case(&provider.library)
                && candidate.source_digest == provider.source_digest
        }) {
            return Err(format!(
                "{} '{}' is defined more than once inside provider '{}'; repair the source because a provider decision cannot distinguish same-source duplicates",
                scope.label(),
                normalized_name,
                provider.library
            ));
        }
        let created_at_unix_ms = crate::time_compat::checked_unix_time_ms()
            .map_err(|error| format!("system clock cannot timestamp provider decision: {error}"))?;
        let record = ModelResolutionRecord {
            schema_version: MODEL_RESOLUTION_RECORD_SCHEMA_VERSION,
            consumer_scope: scope,
            normalized_name,
            provider_library: provider.library.clone(),
            provider_definition: provider.definition.clone(),
            provider_source_digest: provider.source_digest,
            audit_reason: audit_reason.to_owned(),
            created_at_unix_ms,
        };
        record.validate()?;
        self.resolution_records.insert(record.key(), record.clone());
        Ok(record)
    }

    pub fn clear_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> bool {
        let normalized = definition.trim().to_ascii_lowercase();
        self.resolution_records
            .remove(&scope.record_key(&normalized))
            .is_some()
    }

    fn validate_model_resolution_records_against_catalog(&self) -> Result<(), String> {
        for (key, record) in &self.resolution_records {
            record.validate()?;
            if record.key() != *key {
                return Err(format!(
                    "model-resolution map key '{key}' does not match its record identity '{}'",
                    record.key()
                ));
            }
            let provider = self.get_library(&record.provider_library).ok_or_else(|| {
                format!(
                    "provider decision for {} '{}' is stale because library '{}' was removed",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                )
            })?;
            if model_library_source_digest(provider) != record.provider_source_digest {
                return Err(format!(
                    "provider decision for {} '{}' is stale because source '{}' changed digest",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library
                ));
            }
            let exact_provider_exists = self
                .definition_providers(record.consumer_scope, &record.normalized_name)
                .into_iter()
                .any(|candidate| {
                    candidate.library == record.provider_library
                        && candidate.definition == record.provider_definition
                        && candidate.source_digest == record.provider_source_digest
                });
            if !exact_provider_exists {
                return Err(format!(
                    "provider decision for {} '{}' is stale because exact definition '{}/{}' is no longer available",
                    record.consumer_scope.label(),
                    record.normalized_name,
                    record.provider_library,
                    record.provider_definition
                ));
            }
        }
        Ok(())
    }

    /// Create a new manager
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a library
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.catalog.add_library(library);
    }

    /// Remove a library
    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        self.catalog.remove_library(name)
    }

    /// Get a library
    pub fn get_library(&self, name: &str) -> Option<&ModelLibrary> {
        self.catalog.get_library(name)
    }

    /// Get mutable library
    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut ModelLibrary> {
        self.catalog.get_library_mut(name)
    }

    /// Select a library, refusing a name this project no longer holds.
    ///
    /// It used to be a silent no-op, which is the worst of the three possible
    /// behaviours: every Models surface renders from the selection, so a route
    /// that named a library that had since gone left the *previous* one showing
    /// and read as a route that worked. Refusing by name lets the caller say so.
    pub fn select_library(&mut self, name: &str) -> Result<(), String> {
        if !self.catalog.contains_library(name) {
            return Err(format!(
                "Model library '{name}' is not loaded in this project, so the selection was not \
                 changed."
            ));
        }
        self.selected_library = Some(name.to_string());
        Ok(())
    }

    /// Libraries that declare `process`, in sorted order.
    ///
    /// The keyword is the one [`Self::reference_model_execution_plan`] looks a
    /// process section up by, so a route offered here lands on a library that
    /// plan would actually read. Answers "which library is this binding failure
    /// about" when the failure sentence itself names only the process.
    #[must_use]
    pub fn libraries_declaring_process(&self, process: crate::product::ProcessCorner) -> Vec<&str> {
        let keyword = process.short_name();
        self.libraries_sorted()
            .into_iter()
            .filter(|library| library.declares_process(keyword))
            .map(|library| library.name.as_str())
            .collect()
    }

    /// Get current library
    pub fn current_library(&self) -> Option<&ModelLibrary> {
        self.selected_library
            .as_ref()
            .and_then(|name| self.catalog.get_library(name))
    }

    /// Canonical identities of every project-owned model definition admitted
    /// to the executable model closure.
    ///
    /// Prepared simulation receipts retain these typed identities so later
    /// engineering evidence can prove that an exact model revision was present
    /// in the immutable run snapshot instead of trusting a display name or a
    /// user-entered digest.
    pub(crate) fn project_model_definition_identities(
        &self,
    ) -> Result<Vec<(ModelSourceId, String, ObjectRevision, ContentDigest)>, String> {
        self.catalog.project_model_definition_identities()
    }

    /// Search for models by name
    pub fn search_models(&self, pattern: &str) -> Vec<(&ModelLibrary, &DeviceModel)> {
        let pattern_lower = pattern.to_lowercase();
        let mut results = Vec::new();

        for lib in self.catalog.libraries() {
            for model in lib.models.values() {
                if model.name.to_lowercase().contains(&pattern_lower)
                    || model.description.to_lowercase().contains(&pattern_lower)
                {
                    if let Some(filter_type) = self.filter_type {
                        if model.model_type == filter_type {
                            results.push((lib, model));
                        }
                    } else {
                        results.push((lib, model));
                    }
                }
            }
        }

        results
    }

    /// Locate the shipped model packs, if this installation has them.
    ///
    /// Absence is normal, not a failure: the foundation library is compiled into
    /// the binary, and the browser build has no filesystem to find packs on.
    pub fn discover_spice_packs(&mut self) {
        self.spice_packs = match SpiceLibraryIndex::discover() {
            Ok(index) => index.map(Arc::new),
            Err(error) => {
                log::warn!("shipped SPICE model packs could not be read: {error}");
                None
            }
        };
    }

    /// The discovered pack index, when one is present.
    pub fn spice_packs(&self) -> Option<&SpiceLibraryIndex> {
        self.spice_packs.as_deref()
    }

    /// Selectable parts across the shipped packs, or zero when none were found.
    ///
    /// The addressable count rather than the raw definition total, so private
    /// helper cards inside macromodel bodies are never offered as parts.
    pub fn pack_definition_count(&self) -> usize {
        self.spice_packs
            .as_ref()
            .map_or(0, |index| index.part_count())
    }

    /// Search the shipped packs for definitions whose name contains `query`.
    ///
    /// Bounded by `limit` because the same API can inspect an explicitly opened
    /// developer corpus. An empty query returns nothing rather than everything.
    #[cfg(test)]
    pub fn search_pack_models(&self, query: &str, limit: usize) -> Vec<PackModelHit> {
        let trimmed = query.trim();
        if trimmed.is_empty() {
            return Vec::new();
        }
        let Some(index) = self.spice_packs.as_ref() else {
            return Vec::new();
        };

        let entries = match index.search_parts(trimmed, limit) {
            Ok(entries) => entries,
            Err(error) => {
                log::warn!("shipped SPICE model catalog could not be searched: {error}");
                return Vec::new();
            }
        };

        entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|p| p.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                    restricted: entry.restricted,
                }
            })
            .collect()
    }

    /// Browse a bounded first page or a canonical device-class projection of
    /// the shipped corpus without loading every catalog row into memory.
    pub fn browse_pack_models(
        &self,
        query: &str,
        pack_filter: Option<&str>,
        device_filters: &[&str],
        offset: usize,
        limit: usize,
    ) -> Result<(usize, Vec<PackModelHit>), String> {
        let Some(index) = self.spice_packs.as_ref() else {
            return Ok((0, Vec::new()));
        };
        let (total, entries) = index
            .query_parts(query, pack_filter, device_filters, offset, limit)
            .map_err(|error| format!("Shipped model catalog could not be read: {error}"))?;

        let hits = entries
            .into_iter()
            .map(|entry| {
                let pack = index.pack(&entry.pack);
                PackModelHit {
                    name: entry.name.clone(),
                    kind: entry.kind.clone(),
                    device: entry.device.clone(),
                    pack_name: pack.map_or_else(|| entry.pack.clone(), |p| p.name.clone()),
                    redistributable: pack.is_some_and(|pack| pack.redistributable),
                    source: entry.source_path(index),
                    line: entry.line,
                    pack: entry.pack,
                    restricted: entry.restricted,
                }
            })
            .collect();
        Ok((total, hits))
    }

    /// Whether a pack's executable entry bytes are available to attach now.
    ///
    /// Browser builds embed discovery metadata only. They must not enable an
    /// attach action whose synthetic catalog path can never be opened.
    #[must_use]
    pub fn spice_pack_entry_available(&self, pack_id: &str) -> bool {
        self.spice_packs.as_ref().is_some_and(|index| {
            index.source_files_available()
                && index
                    .pack(pack_id)
                    .and_then(|pack| pack.entry_path(index.root()))
                    .is_some_and(|entry| entry.is_file())
        })
    }

    /// Load a redistributable pack's declared entry as one authenticated model
    /// library. The caller publishes the resulting manager candidate at the
    /// project transaction boundary.
    pub fn attach_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Model pack '{}' cannot be embedded in a project because its redistribution grant is not established.",
                pack.name
            ));
        }
        let entry = pack.entry_path(index.root()).ok_or_else(|| {
            format!(
                "Model pack '{}' has no declared entry file to attach.",
                pack.name
            )
        })?;
        let library_name = self.load_catalog_source_without_collision(&entry)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    /// Retain one part's source from an installed Model Hub pack, recording
    /// which signed release the bytes came from.
    ///
    /// `files` is the release's expanded file set and `part_source` names the
    /// member that defines the part; everything unreachable from it is dropped
    /// rather than retained. The bytes are the argument, not a path, because a
    /// pack's sources are proved as bytes on every platform and only *happen*
    /// to be expanded on a disk when the store is a filesystem. Routing both
    /// stores through the one import is what makes a browser session's project
    /// and a desktop session's project the same document.
    ///
    /// What lands is the same retention an uploaded source goes through — the
    /// same closure, the same digest, the same read-only retained authority —
    /// with one fact added: the pin naming the release. The pin is evidence,
    /// never a resolution input; nothing about executing the library consults
    /// it, so a project whose pinned release has been uninstalled behaves
    /// exactly as it did the day it was saved.
    pub fn add_pack_release_part(
        &mut self,
        files: Vec<(String, Vec<u8>)>,
        part_source: &str,
        pin: super::PackPartPin,
    ) -> Result<String, String> {
        let (library_name, mut library) =
            source_bundle::build(part_source, Some(part_source), files, None)?;
        if let Some(existing) = self.catalog.get_library(&library_name) {
            // Different bytes under a name this project already uses is a
            // collision — unless the name is held by this same part of this
            // same pack, in which case it is the one thing it can honestly be:
            // another release of it. Adopting a release re-runs this path under
            // the newer version, and refusing there would leave detaching the
            // library — and losing the pin that says where it came from — as
            // the only way to move a pinned part forward.
            let same_pack_part = existing
                .pack_pin
                .as_ref()
                .is_some_and(|held| held.pack_id == pin.pack_id && held.part_id == pin.part_id);
            if existing.root_path != library.root_path && !same_pack_part {
                return Err(format!(
                    "Library name '{library_name}' is already owned by another model source; \
                     rename or detach it before adding '{}'.",
                    pin.part_id
                ));
            }
            // Identical bytes under an identity this project already minted:
            // keeping it means a second part from the same source file does
            // not renumber the source every surface already refers to.
            if let (
                ModelSourceAuthority::RetainedImport { source_id, .. },
                ModelSourceAuthority::RetainedImport {
                    source_id: replacement,
                    ..
                },
            ) = (existing.source_authority, &mut library.source_authority)
            {
                *replacement = source_id;
            }
        }
        let pack_id = pin.pack_id.clone();
        self.catalog.add_library(library);
        self.retain_pack_library_pinned(&library_name, &pack_id, Some(pin))?;
        Ok(library_name)
    }

    fn retain_pack_library(&mut self, library_name: &str, pack_id: &str) -> Result<(), String> {
        self.retain_pack_library_pinned(library_name, pack_id, None)
    }

    fn retain_pack_library_pinned(
        &mut self,
        library_name: &str,
        pack_id: &str,
        pin: Option<super::PackPartPin>,
    ) -> Result<(), String> {
        let library = self.catalog.get_library_mut(library_name).ok_or_else(|| {
            format!("Attached pack library '{library_name}' disappeared before publication")
        })?;
        let root = library.root_path.as_ref().ok_or_else(|| {
            format!("Attached pack library '{library_name}' has no root identity")
        })?;
        let root_digest = library
            .source_closure
            .iter()
            .find(|pin| pin.path == *root)
            .map(|pin| pin.digest)
            .ok_or_else(|| {
                format!(
                    "Attached pack library '{library_name}' did not retain its root source bytes"
                )
            })?;
        let source_id = match library.source_authority {
            ModelSourceAuthority::RetainedImport { source_id, .. } => source_id,
            _ => ModelSourceId::new(),
        };
        library.source_authority = ModelSourceAuthority::RetainedImport {
            source_id,
            digest: root_digest,
        };
        library.pack_id = Some(pack_id.to_owned());
        if pin.is_some() {
            library.pack_pin = pin;
        }
        Ok(())
    }

    /// Explicitly refresh a shipped-pack snapshot from the currently installed
    /// corpus, then immediately return it to retained project authority.
    pub fn refresh_spice_pack(&mut self, pack_id: &str) -> Result<String, String> {
        let entry = {
            let index = self.spice_packs.as_ref().ok_or_else(|| {
                "The shipped model corpus is not installed on this machine.".to_owned()
            })?;
            let pack = index
                .pack(pack_id)
                .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
            if !pack.redistributable {
                return Err(format!(
                    "Model pack '{}' can no longer be refreshed because its redistribution grant is not established.",
                    pack.name
                ));
            }
            pack.entry_path(index.root()).ok_or_else(|| {
                format!(
                    "Model pack '{}' has no declared entry file to refresh.",
                    pack.name
                )
            })?
        };
        let selected_corner = self
            .catalog
            .libraries()
            .find(|library| library.pack_id.as_deref() == Some(pack_id))
            .and_then(|library| library.selected_corner.clone());
        let library_name = self.load_library_file(&entry, selected_corner.as_deref())?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    /// Load the exact shipped source containing an addressable part. Restricted
    /// files fail closed instead of being silently copied into project data.
    pub fn add_spice_part(&mut self, pack_id: &str, part_name: &str) -> Result<String, String> {
        let index = self.spice_packs.as_ref().ok_or_else(|| {
            "The shipped model corpus is not installed on this machine.".to_owned()
        })?;
        let pack = index
            .pack(pack_id)
            .ok_or_else(|| format!("Model pack '{pack_id}' is no longer installed."))?;
        if !pack.redistributable {
            return Err(format!(
                "Part '{part_name}' cannot be copied from '{}' because its redistribution grant is not established.",
                pack.name
            ));
        }
        let matches = index
            .find_part(part_name)
            .map_err(|error| format!("Part '{part_name}' could not be resolved: {error}"))?;
        let entry = matches
            .into_iter()
            .find(|entry| entry.pack == pack_id)
            .ok_or_else(|| {
                format!(
                    "Part '{part_name}' is no longer present in pack '{}'.",
                    pack.name
                )
            })?;
        if entry.restricted {
            return Err(format!(
                "Part '{part_name}' is in a source file that is not licensed for project embedding."
            ));
        }
        let source = entry
            .source_path(index)
            .ok_or_else(|| format!("Part '{part_name}' has no installed source file."))?;
        let library_name = self.load_catalog_source_without_collision(&source)?;
        self.retain_pack_library(&library_name, pack_id)?;
        Ok(library_name)
    }

    fn load_catalog_source_without_collision(&mut self, path: &Path) -> Result<String, String> {
        let canonical = std::fs::canonicalize(path).map_err(|error| {
            format!(
                "Failed to resolve model source '{}': {error}",
                path.display()
            )
        })?;
        let library_name = canonical
            .file_stem()
            .and_then(|name| name.to_str())
            .ok_or_else(|| {
                format!(
                    "Model source '{}' has no valid file name.",
                    canonical.display()
                )
            })?
            .to_owned();
        if let Some(existing) = self.get_library(&library_name) {
            if existing.root_path.as_deref() == Some(canonical.as_path()) {
                return Ok(library_name);
            }
            return Err(format!(
                "Library name '{library_name}' is already owned by another source; rename or detach it before adding '{}'.",
                canonical.display()
            ));
        }
        self.load_library_file(&canonical, None)
    }

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        self.catalog.libraries_sorted()
    }

    /// Stable owned snapshot used by guarded multi-library project
    /// transactions. Presentation filters and the shipped-pack index remain
    /// manager state and are intentionally excluded.
    pub(crate) fn library_snapshot(&self) -> Vec<ModelLibrary> {
        self.catalog.library_snapshot()
    }

    /// Replace the complete loaded-library set while preserving presentation
    /// state owned by this manager.
    pub(crate) fn replace_library_snapshot(
        &mut self,
        libraries: Vec<ModelLibrary>,
    ) -> Result<(), String> {
        self.catalog.replace_library_snapshot(libraries)?;
        if self
            .selected_library
            .as_ref()
            .is_some_and(|name| !self.catalog.contains_library(name))
        {
            self.selected_library = None;
        }
        Ok(())
    }

    /// Total library count
    pub fn library_count(&self) -> usize {
        self.catalog.library_count()
    }

    /// Total model count across all libraries
    pub fn total_model_count(&self) -> usize {
        self.catalog.libraries().map(|l| l.model_count()).sum()
    }

    /// Clear all
    #[cfg(test)]
    pub fn clear(&mut self) {
        self.catalog.clear();
        self.resolution_records.clear();
        self.selected_library = None;
    }

    /// Load a library from a .lib file
    ///
    /// Parses the file using the rspice-core library parser and adds models
    /// to a new library entry.
    pub fn load_library_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
        section: Option<&str>,
    ) -> Result<String, String> {
        use rspice_core::library::LibParser;

        let path = std::fs::canonicalize(path.as_ref()).map_err(|error| {
            format!(
                "Failed to resolve model library '{}': {error}",
                path.as_ref().display()
            )
        })?;
        let base_dir = path.parent().unwrap_or(std::path::Path::new("."));
        // The parser captures the exact root-plus-include bytes it consumes.
        // Hash that captured closure so parsing and pinning cannot observe
        // different file versions during an explicit refresh.
        let mut parser = LibParser::new(base_dir);
        let result = parser.parse_file(&path).map_err(|error| {
            format!(
                "Failed to parse model library '{}': {error}",
                path.display()
            )
        })?;
        self.catalog.import_external_library(
            &path,
            result,
            section,
            source_bundle::limits(),
            source_bundle::capture_external_hdl_sources,
        )
    }

    /// Import one self-contained model source from authenticated bytes.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bytes(
        &mut self,
        file_name: &str,
        bytes: Vec<u8>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle(file_name, vec![(file_name.to_owned(), bytes)], section)
    }

    /// Import a browser-selected source tree with one unambiguous executable
    /// root. Every reachable `.include`, external `.lib`, and Verilog-A edge is
    /// resolved relative to its owner; unreachable uploads are ignored. If a
    /// tree has multiple possible roots, callers must use
    /// [`Self::load_library_bundle_from_root`] to make the choice explicit.
    #[cfg(any(test, target_arch = "wasm32"))]
    pub fn load_library_bundle(
        &mut self,
        display_name: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, None, files, section)
    }

    /// Import a browser-selected source tree from one explicit executable
    /// entry. Unreachable members are neither decoded nor retained.
    pub fn load_library_bundle_from_root(
        &mut self,
        display_name: &str,
        root_member: &str,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        self.load_library_bundle_with_root(display_name, Some(root_member), files, section)
    }

    fn load_library_bundle_with_root(
        &mut self,
        display_name: &str,
        root_member: Option<&str>,
        files: Vec<(String, Vec<u8>)>,
        section: Option<&str>,
    ) -> Result<String, String> {
        let (lib_name, library) = source_bundle::build(display_name, root_member, files, section)?;
        if self.catalog.contains_library(&lib_name) {
            return Err(format!(
                "Model library '{lib_name}' already exists; remove it before importing replacement bytes"
            ));
        }
        self.catalog.add_library(library);
        Ok(lib_name)
    }

    /// Resolve deterministic, self-contained model cards for a nominal run.
    pub fn reference_process_model_cards(
        &self,
        process: crate::product::ProcessCorner,
    ) -> Result<Vec<String>, String> {
        self.seal_execution_sources()?
            .reference_process_model_cards(process)
    }

    /// Resolve all process-specific sources required by a PVT run.
    #[cfg(test)]
    pub fn corner_model_bindings(
        &self,
        processes: &[CornerProcess],
    ) -> Result<Vec<CornerModelBinding>, String> {
        self.seal_execution_sources()?
            .corner_model_bindings(processes)
    }

    /// Atomically replace the exact external libraries governed by a PDK
    /// configuration.
    ///
    /// Discovery always runs against `next`; cached dialog rows never
    /// authorize application. Every enabled file is canonicalized and parsed
    /// into an isolated manager candidate. Scan, include, parse, or
    /// name-collision failure leaves both this manager and the retained PDK
    /// ownership provenance unchanged.
    pub fn replace_from_pdk_config(
        &mut self,
        previous: Option<&crate::state::pdk_config::PdkConfig>,
        next: &mut crate::state::pdk_config::PdkConfig,
    ) -> Result<usize, Vec<String>> {
        next.discover_model_files();
        if !next.scan_errors.is_empty() {
            return Err(next.scan_errors.clone());
        }

        let mut candidate = self.clone();
        let mut previously_managed = previous
            .into_iter()
            .flat_map(|config| config.managed_model_sources.iter())
            .map(|path| portable_path_key(path))
            .collect::<BTreeSet<_>>();
        if previously_managed.is_empty() {
            previously_managed.extend(
                previous
                    .into_iter()
                    .flat_map(|config| config.discovered_files.iter())
                    .map(|file| portable_path_key(&file.path)),
            );
        }
        let removed = candidate
            .catalog
            .iter()
            .filter(|&(_name, library)| {
                (matches!(library.source_authority, ModelSourceAuthority::External)
                    && library
                        .root_path
                        .as_ref()
                        .is_some_and(|path| previously_managed.contains(&portable_path_key(path))))
            })
            .map(|(name, _library)| name.clone())
            .collect::<Vec<_>>();
        for name in removed {
            candidate.catalog.remove_library(&name);
        }
        if candidate
            .selected_library
            .as_ref()
            .is_some_and(|name| !candidate.catalog.contains_library(name))
        {
            candidate.selected_library = None;
        }

        let mut loaded = 0;
        let mut errors = Vec::new();
        let mut admitted = Vec::new();
        let mut seen = BTreeSet::new();
        for file in &next.discovered_files {
            if !crate::state::pdk_config::MODEL_FILE_EXTENSIONS.contains(&file.extension.as_str()) {
                continue;
            }
            let canonical = match std::fs::canonicalize(&file.path) {
                Ok(path) => path,
                Err(error) => {
                    errors.push(format!(
                        "{}: failed to resolve discovered model source: {error}",
                        file.path.display()
                    ));
                    continue;
                }
            };
            if !seen.insert(portable_path_key(&canonical)) {
                continue;
            }
            match candidate.load_library_file(&canonical, None) {
                Ok(_) => {
                    admitted.push(canonical);
                    loaded += 1;
                }
                Err(error) => errors.push(format!("{}: {error}", file.path.display())),
            }
        }

        if errors.is_empty() {
            next.managed_model_sources = admitted;
            *self = candidate;
            Ok(loaded)
        } else {
            Err(errors)
        }
    }

    /// Populate the UI catalog from the embedded RSpice foundation library.
    ///
    /// The cards are read through the `.lib` parser rather than the core
    /// library index, because the index reduces every card to one of nine
    /// coarse types and a foundation card's *declared* type is what the rest
    /// of the catalog reasons with: `models_have_compatible_device_family`
    /// tells an n-channel MESFET from a p-channel one by the `NMF`/`PMF`
    /// token, and a partially-depleted SOI card from a plain MOSFET by its
    /// `LEVEL=57`. Reduced to `OTHER` and `Unknown`, an SOI card would be
    /// offered to bulk MOSFETs and withheld from the SOI symbol it exists for.
    ///
    /// The declared token is also what decides the card's [`ModelType`], for
    /// the same reason and through the same vocabulary the whole catalog
    /// reads. Classifying through the core index instead lost every family the
    /// index has no type for — VDMOS and MESFET both — and, having a `Njfet`
    /// it could not spell, discarded JFETs as well.
    pub fn load_builtin_models(&mut self) {
        self.catalog.load_builtin_models()
    }
}

#[cfg(test)]
mod builtin_catalog_tests;
#[cfg(test)]
mod tests;
