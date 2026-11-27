//! The model library.
//!
//! Owns every model a project can resolve, from every source, and seals the
//! exact set a run executed against. Sealing is by content digest, so a
//! library that changes underneath a completed run is detectable rather
//! than silently assumed identical.

mod catalog_identity;
mod project_models;
mod sealing;
mod source_bundle;

pub(crate) use rspice_model_library::ModelDefinitionProvider;
pub use rspice_model_library::model_library_source_digest;
use rspice_model_library::source_paths::portable_path_key;
pub use rspice_simulation::model_sources::SealedModelExecutionSources;

use serde::{Deserialize, Serialize};
#[cfg(any(test, not(target_arch = "wasm32")))]
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

use rspice_vocabulary::stamped::Stamped;

#[cfg(test)]
use rspice_model_library::{
    MODEL_RESOLUTION_RECORD_SCHEMA_VERSION, ModelValidationFindingSeverity,
};
use rspice_model_library::{
    ModelConsumerScope, ModelResolutionRecord, ModelResolutionRecords, ModelValidationFinding,
    ModelValidationReceipt, ModelValidationReceiptInput, SimulationPlanModelBinding,
};

#[cfg(any(test, feature = "test-support"))]
use super::ProjectModelDefinition;
use super::{
    DeviceModel, ModelLibrary, ModelSourceAuthority, ModelType, ProcessCorner,
    ProjectModelRevisionDefinition,
};
#[cfg(test)]
use super::{ModelLevel, ModelSourceEdge, ModelSourcePin};
use crate::product::{ContentDigest, ModelSourceId, ObjectRevision};
#[cfg(test)]
use rspice_model_library::CornerModelBinding;
#[cfg(test)]
use rspice_vocabulary::product::ProcessCorner as CornerProcess;

pub use rspice_model_library::ProjectModelCommit;

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

/// Manager for all model libraries
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModelLibraryManager {
    /// All libraries. This and the two records below are what a project
    /// saves of the manager, so each is stamped: a mutable access to it
    /// issues a new stamp, and what is derived from them is reused exactly
    /// until one does.
    #[serde(rename = "libraries")]
    catalog: Stamped<rspice_model_library::ModelCatalog>,
    /// Currently selected library
    pub selected_library: Option<String>,
    /// Search filter
    pub filter_text: String,
    /// Filter by model type
    pub filter_type: Option<ModelType>,
    /// Durable project decisions for contested executable definitions. The
    /// map key is the canonical `scope:name` identity repeated by each value.
    #[serde(default)]
    resolution_records: Stamped<ModelResolutionRecords>,
    #[serde(default)]
    validation_receipt: Stamped<Option<ModelValidationReceipt>>,
}

impl ModelLibraryManager {
    #[must_use]
    pub fn model_validation_receipt(&self) -> Option<&ModelValidationReceipt> {
        self.validation_receipt.as_ref()
    }

    /// The stamps of what a project saves of the manager, as they stand:
    /// equal stamps, equal catalog, resolution records and receipt. See
    /// [`rspice_vocabulary::stamped`].
    pub const fn content_stamp(&self) -> [u64; 3] {
        [
            self.catalog.stamp(),
            self.resolution_records.stamp(),
            self.validation_receipt.stamp(),
        ]
    }

    pub(crate) fn invalidate_model_validation_receipt(&mut self) {
        *self.validation_receipt = None;
    }

    pub fn restore_model_validation_receipt(
        &mut self,
        receipt: Option<ModelValidationReceipt>,
    ) -> Result<(), String> {
        if let Some(receipt) = receipt.as_ref() {
            receipt.verify()?;
        }
        *self.validation_receipt = receipt;
        Ok(())
    }

    pub fn issue_model_validation_receipt(
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
        *self.validation_receipt = Some(receipt.clone());
        Ok(receipt)
    }

    pub fn validate_model_validation_receipt(
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
            return Err("Model-validation receipt is stale after the PDK changed.".to_owned());
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
        self.catalog.model_validation_source_identity()
    }

    #[must_use]
    pub fn model_resolution_record(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Option<&ModelResolutionRecord> {
        self.resolution_records.record(scope, definition)
    }

    pub fn restore_model_resolution_records(
        &mut self,
        records: Vec<ModelResolutionRecord>,
    ) -> Result<(), String> {
        self.resolution_records.restore(records, &self.catalog)
    }

    #[must_use]
    pub fn owned_model_resolution_records(&self) -> Vec<ModelResolutionRecord> {
        self.resolution_records.owned_records()
    }

    pub fn definition_providers(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Vec<ModelDefinitionProvider> {
        self.catalog.definition_providers(scope, definition)
    }

    /// Resolve the one provider the flat executable SPICE namespace will use.
    ///
    /// Component properties may retain a library name for provenance, but that
    /// metadata is not an independent namespace selector. Every UI surface must
    /// consult this method so Properties, catalog binding, and the sealed run
    /// plan agree with the same project-global provider decision.
    pub fn effective_definition_provider(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        self.resolution_records
            .effective_definition_provider(&self.catalog, scope, definition)
    }

    /// The provider a run of a plan bound to `bindings` reads: its one bound
    /// provider of the name, or the project-global decision. What a surface
    /// says about an instance in the design follows this.
    pub fn effective_plan_definition_provider(
        &self,
        bindings: &[SimulationPlanModelBinding],
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        self.resolution_records.effective_plan_definition_provider(
            &self.catalog,
            bindings,
            scope,
            definition,
        )
    }

    pub fn resolve_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
        provider_library: &str,
        audit_reason: &str,
    ) -> Result<ModelResolutionRecord, String> {
        self.resolution_records.resolve_definition_provider(
            &self.catalog,
            scope,
            definition,
            provider_library,
            audit_reason,
            || {
                crate::time_compat::checked_unix_time_ms().map_err(|error| {
                    format!("system clock cannot timestamp provider decision: {error}")
                })
            },
        )
    }

    pub fn clear_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> bool {
        self.resolution_records
            .clear_definition_provider(scope, definition)
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

    pub fn catalog(&self) -> &rspice_model_library::ModelCatalog {
        &self.catalog
    }

    pub fn resolution_records(&self) -> &ModelResolutionRecords {
        &self.resolution_records
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

    /// Get libraries sorted by name
    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        self.catalog.libraries_sorted()
    }

    /// Stable owned snapshot used by guarded multi-library project
    /// transactions. Presentation filters remain manager state and are
    /// intentionally excluded.
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
    #[cfg(any(test, feature = "test-support"))]
    pub fn clear(&mut self) {
        self.catalog.clear();
        *self.resolution_records = ModelResolutionRecords::default();
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
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
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
    #[cfg(any(test, feature = "test-support", target_arch = "wasm32"))]
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

    /// Every executable library's cards for the nominal process.
    #[cfg(test)]
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
        crate::state::pdk_config::discover_model_files(next);
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
            // `.txt` names no format: a text file in a model folder is a model
            // source only when it declares a SPICE definition, so a folder's
            // notes and licences are passed over rather than refused.
            if file.extension == "txt" && !declares_spice_definition(&canonical) {
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

/// Whether the file at `path` declares a SPICE definition: a `.model`,
/// `.subckt`, `.lib`, `.include` or `.param` card at the start of a line.
fn declares_spice_definition(path: &Path) -> bool {
    std::fs::read(path)
        .ok()
        .and_then(|bytes| rspice_core::netlist::decode_source_bytes(&bytes).ok())
        .is_some_and(|text| {
            text.lines().any(|line| {
                line.split_whitespace().next().is_some_and(|card| {
                    [".model", ".subckt", ".lib", ".include", ".inc", ".param"]
                        .iter()
                        .any(|directive| card.eq_ignore_ascii_case(directive))
                })
            })
        })
}

#[cfg(test)]
mod builtin_catalog_tests;
#[cfg(test)]
mod tests;
