//! Providers visible in the executable model and subcircuit namespaces.
//!
//! A library provides a model name when it holds the card of that name, or —
//! holding none — bins of the family the name binds (`nch.1` … `nch.48` for
//! `nch`), which is what an instance naming it then runs. A family competes
//! for its name exactly as a card does: two libraries providing it, as cards
//! or as families, contest it until the project decides which one runs.

use super::ModelCatalog;
use crate::{ModelConsumerScope, model_library_source_digest};
use rspice_app_types::product::ContentDigest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelDefinitionProvider {
    pub library: String,
    /// The card's name, or the family's as its first bin spells it.
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
                ModelConsumerScope::PrimitiveModel => {
                    let exact = library
                        .models
                        .values()
                        .map(|model| model.name.as_str())
                        .filter(|name| name.to_ascii_lowercase() == normalized)
                        .collect::<Vec<_>>();
                    if exact.is_empty() {
                        library
                            .bin_family(&normalized)
                            .map(|family| vec![family.name])
                            .unwrap_or_default()
                    } else {
                        exact
                    }
                }
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

#[cfg(test)]
mod tests {
    use crate::{DeviceModel, ModelCatalog, ModelLibrary, ModelResolutionRecords, ModelType};
    use crate::{ModelConsumerScope, model_library_source_digest};

    fn library(name: &str, cards: &[&str]) -> ModelLibrary {
        let mut library = ModelLibrary::new(name);
        for card in cards {
            library.add_model(
                DeviceModel::new(*card, ModelType::Nmos).with_geometry(1e-7, 1e-5, 1e-7, 1e-5),
            );
        }
        library
    }

    fn providers(catalog: &ModelCatalog, name: &str) -> Vec<(String, String)> {
        catalog
            .definition_providers(ModelConsumerScope::PrimitiveModel, name)
            .into_iter()
            .map(|provider| (provider.library, provider.definition))
            .collect()
    }

    /// A family is provided by the library holding its bins, and its bins stay
    /// provided by their own names.
    #[test]
    fn a_library_of_bins_provides_their_family() {
        let mut catalog = ModelCatalog::default();
        catalog.add_library(library("kit", &["NCH.1", "nch.2", "nch_25"]));
        assert_eq!(providers(&catalog, "nch"), [("kit".into(), "NCH".into())]);
        assert_eq!(
            providers(&catalog, "nch.2"),
            [("kit".into(), "nch.2".into())]
        );
        assert_eq!(
            providers(&catalog, "nch_25"),
            [("kit".into(), "nch_25".into())]
        );
        assert!(providers(&catalog, "nch_2").is_empty());
        catalog.add_library(library("solo", &["foo.1"]));
        assert_eq!(providers(&catalog, "foo"), [("solo".into(), "foo".into())]);
    }

    /// Two libraries providing one name contest it whether they hold the card
    /// or a family of it, and the project's decision settles it either way.
    #[test]
    fn a_family_contests_its_name_like_a_card() {
        let mut catalog = ModelCatalog::default();
        catalog.add_library(library("kit", &["nch.1", "nch.2"]));
        catalog.add_library(library("vendor", &["nch"]));
        let mut records = ModelResolutionRecords::default();
        let scope = ModelConsumerScope::PrimitiveModel;
        let refusal = records
            .effective_definition_provider(&catalog, scope, "nch")
            .expect_err("an undecided name has no provider");
        assert!(refusal.contains("kit, vendor"), "{refusal}");

        let record = records
            .resolve_definition_provider(&catalog, scope, "nch", "kit", "family", || Ok(1))
            .expect("the family can be chosen");
        assert_eq!(record.provider_definition, "nch");
        let effective = records
            .effective_definition_provider(&catalog, scope, "NCH")
            .expect("decided")
            .expect("a provider");
        assert_eq!(effective.library, "kit");
        assert_eq!(
            effective.source_digest,
            model_library_source_digest(catalog.get_library("kit").expect("kit"))
        );
        records
            .validate_against_catalog(&catalog)
            .expect("the decision names a provider the catalog still has");
    }
}
