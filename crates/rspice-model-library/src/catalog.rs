//! Portable model catalog and atomic edits to project-owned definitions.
//!
//! The serialized representation remains the library-name map used by projects.
//! Host discovery, presentation selection, and execution services are separate.

use crate::{
    DeviceModel, ModelLevel, ModelLibrary, ModelSourceAuthority, ModelType, ProjectModelDefinition,
    ProjectModelRevisionDefinition,
};
use rspice_app_types::product::{ContentDigest, ModelSourceId, ObjectRevision};
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};
use std::collections::{BTreeMap, HashMap};

mod project_models;
#[cfg(test)]
mod tests;

pub use project_models::{ProjectModelCommit, ProjectModelTarget};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelCatalog {
    libraries: HashMap<String, ModelLibrary>,
}

impl ModelCatalog {
    pub fn add_library(&mut self, library: ModelLibrary) {
        self.libraries.insert(library.name.clone(), library);
    }

    pub fn remove_library(&mut self, name: &str) -> Option<ModelLibrary> {
        self.libraries.remove(name)
    }

    pub fn get_library(&self, name: &str) -> Option<&ModelLibrary> {
        self.libraries.get(name)
    }

    pub fn get_library_mut(&mut self, name: &str) -> Option<&mut ModelLibrary> {
        self.libraries.get_mut(name)
    }

    pub fn contains_library(&self, name: &str) -> bool {
        self.libraries.contains_key(name)
    }

    pub fn libraries(&self) -> impl Iterator<Item = &ModelLibrary> {
        self.libraries.values()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&String, &ModelLibrary)> {
        self.libraries.iter()
    }

    pub fn library_count(&self) -> usize {
        self.libraries.len()
    }

    pub fn clear(&mut self) {
        self.libraries.clear();
    }

    pub fn libraries_sorted(&self) -> Vec<&ModelLibrary> {
        let mut libs: Vec<_> = self.libraries.values().collect();
        libs.sort_by(|a, b| a.name.cmp(&b.name));
        libs
    }

    pub fn library_snapshot(&self) -> Vec<ModelLibrary> {
        self.libraries_sorted().into_iter().cloned().collect()
    }

    pub fn project_model_definition_identities(
        &self,
    ) -> Result<Vec<(ModelSourceId, String, ObjectRevision, ContentDigest)>, String> {
        let mut identities = Vec::new();
        for library in self
            .libraries
            .values()
            .filter(|library| library.source_authority.is_project_owned())
        {
            let ModelSourceAuthority::ProjectOwned {
                source_id,
                revision: library_revision,
                ..
            } = library.source_authority
            else {
                continue;
            };
            for (model_name, model) in &library.models {
                let metadata = library
                    .model_definition_metadata
                    .get(model_name)
                    .cloned()
                    .ok_or_else(|| {
                        format!(
                            "Project model '{}/{}' has no typed definition metadata",
                            library.name, model_name
                        )
                    })?;
                let definition = ProjectModelRevisionDefinition::new(
                    ProjectModelDefinition::from_device_model(model),
                    metadata,
                );
                let canonical = definition.canonical_source().map_err(|error| {
                    format!(
                        "Project model '{}/{}' cannot be authenticated for execution: {error}",
                        library.name, model_name
                    )
                })?;
                let definition_identity =
                    definition.project_source_identity().map_err(|error| {
                        format!(
                            "Project model '{}/{}' has invalid source identity: {error}",
                            library.name, model_name
                        )
                    })?;
                let revision = definition_identity
                    .as_ref()
                    .map_or(library_revision, |identity| identity.revision);
                let digest = ContentDigest::from_bytes(Sha256::digest(canonical.as_bytes()).into());
                if let Some(identity) = definition_identity
                    && (identity.source_id != source_id || identity.content_digest != digest)
                {
                    return Err(format!(
                        "Project model '{}/{}' definition identity does not match its retained source",
                        library.name, model_name
                    ));
                }
                identities.push((source_id, model_name.clone(), revision, digest));
            }
        }
        identities.sort_by(|left, right| {
            left.0
                .as_uuid()
                .cmp(&right.0.as_uuid())
                .then_with(|| {
                    left.1
                        .to_ascii_lowercase()
                        .cmp(&right.1.to_ascii_lowercase())
                })
                .then_with(|| left.2.cmp(&right.2))
                .then_with(|| left.3.cmp(&right.3))
        });
        identities.dedup();
        Ok(identities)
    }

    pub fn load_builtin_models(&mut self) {
        let core_manager = rspice_core::library::LibraryManager::new();
        let Some(content) = core_manager.get_library_content("foundation.lib") else {
            return;
        };
        let parsed = rspice_core::library::LibParser::new(".").parse_string(content);
        let library = self
            .libraries
            .entry("RSpice Foundation".to_owned())
            .or_insert_with(|| ModelLibrary::new("RSpice Foundation"));
        library.pack_id = Some("rspice-foundation".to_owned());

        for model in &parsed.top_level_models {
            let device_model = DeviceModel {
                name: model.name.clone(),
                // Built-in cards are compiled in at file scope; no `.lib`
                // section owns them.
                section: None,
                model_type: ModelType::from_name(&model.spice_type),
                spice_type: Some(model.spice_type.clone()),
                level: ModelLevel::from_spice_card(model.level, &model.spice_type),
                spice_level: model.level,
                model_version: model.version,
                description: model.description.clone().unwrap_or_default(),
                l_min: model.lmin,
                l_max: model.lmax,
                w_min: model.wmin,
                w_max: model.wmax,
                vdd: None,
                vth0: None,
                // Compiled in: there is no file on disk to reveal, so there is
                // no line in one either.
                file_path: None,
                parameters: model.parameters.clone(),
                string_parameters: model.string_params.clone(),
                source_line: None,
            };
            library.add_model(device_model);
        }
        for subcircuit in core_manager.subcircuits() {
            library.subcircuits.insert(
                subcircuit.name.clone(),
                crate::ModelSubcircuitInterface {
                    name: subcircuit.name.clone(),
                    ports: subcircuit.pins.clone(),
                    parameter_defaults: BTreeMap::new(),
                    description: subcircuit.description.clone(),
                    file_path: None,
                    source_line: None,
                    section: None,
                },
            );
        }
    }

    pub fn replace_library_snapshot(&mut self, libraries: Vec<ModelLibrary>) -> Result<(), String> {
        let expanded = self
            .libraries
            .iter()
            .map(|(name, library)| (name.clone(), library.expanded))
            .collect::<HashMap<_, _>>();
        let mut replacement = HashMap::with_capacity(libraries.len());
        for mut library in libraries {
            if replacement.contains_key(&library.name) {
                return Err(format!(
                    "Model-library snapshot repeats library '{}'",
                    library.name
                ));
            }
            if let Some(retained) = expanded.get(&library.name) {
                library.expanded = *retained;
            }
            replacement.insert(library.name.clone(), library);
        }
        self.libraries = replacement;
        Ok(())
    }
}
