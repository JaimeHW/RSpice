//! Exact providers visible in the executable model and subcircuit namespaces.

use super::ModelCatalog;
use crate::{ModelConsumerScope, model_library_source_digest};
use rspice_app_types::product::ContentDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDefinitionProvider {
    pub library: String,
    pub definition: String,
    pub source_digest: ContentDigest,
}

impl ModelCatalog {
    pub fn definition_providers(
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
}
