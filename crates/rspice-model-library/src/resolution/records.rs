//! Durable provider decisions, validated against the current portable catalog.

use super::{MODEL_RESOLUTION_RECORD_SCHEMA_VERSION, ModelConsumerScope, ModelResolutionRecord};
use crate::{ModelCatalog, ModelDefinitionProvider, model_library_source_digest};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// The persisted canonical-name map of project-global provider decisions.
#[derive(Debug, Clone, Default, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ModelResolutionRecords {
    records: BTreeMap<String, ModelResolutionRecord>,
}

impl ModelResolutionRecords {
    pub fn as_map(&self) -> &BTreeMap<String, ModelResolutionRecord> {
        &self.records
    }

    pub fn record(
        &self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Option<&ModelResolutionRecord> {
        let normalized = definition.trim().to_ascii_lowercase();
        self.records.get(&scope.record_key(&normalized))
    }

    pub fn restore(
        &mut self,
        records: Vec<ModelResolutionRecord>,
        catalog: &ModelCatalog,
    ) -> Result<(), String> {
        let mut restored = BTreeMap::new();
        for record in records {
            record.validate()?;
            let key = record.key();
            if restored.insert(key.clone(), record).is_some() {
                return Err(format!("model-resolution record '{key}' is repeated"));
            }
        }
        self.records = restored;
        self.validate_against_catalog(catalog)
    }

    pub fn owned_records(&self) -> Vec<ModelResolutionRecord> {
        self.records.values().cloned().collect()
    }

    pub fn effective_definition_provider(
        &self,
        catalog: &ModelCatalog,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> Result<Option<ModelDefinitionProvider>, String> {
        let providers = catalog.definition_providers(scope, definition);
        match providers.as_slice() {
            [] => Ok(None),
            [provider] => Ok(Some(provider.clone())),
            _ => {
                let normalized_name = definition.trim().to_ascii_lowercase();
                let Some(record) = self.record(scope, &normalized_name) else {
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
        catalog: &ModelCatalog,
        scope: ModelConsumerScope,
        definition: &str,
        provider_library: &str,
        audit_reason: &str,
        now: impl FnOnce() -> Result<u64, String>,
    ) -> Result<ModelResolutionRecord, String> {
        let normalized_name = definition.trim().to_ascii_lowercase();
        let providers = catalog.definition_providers(scope, &normalized_name);
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
        let created_at_unix_ms = now()?;
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
        self.records.insert(record.key(), record.clone());
        Ok(record)
    }

    pub fn clear_definition_provider(
        &mut self,
        scope: ModelConsumerScope,
        definition: &str,
    ) -> bool {
        let normalized = definition.trim().to_ascii_lowercase();
        self.records
            .remove(&scope.record_key(&normalized))
            .is_some()
    }

    pub fn validate_against_catalog(&self, catalog: &ModelCatalog) -> Result<(), String> {
        for (key, record) in &self.records {
            record.validate()?;
            if record.key() != *key {
                return Err(format!(
                    "model-resolution map key '{key}' does not match its record identity '{}'",
                    record.key()
                ));
            }
            let provider = catalog
                .get_library(&record.provider_library)
                .ok_or_else(|| {
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
            let exact_provider_exists = catalog
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
}
