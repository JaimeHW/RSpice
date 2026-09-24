//! Executable model namespaces resolved from materialized source providers.
//!
//! Source adapters retain authentication and acquisition. This module owns exact
//! provider decisions, project-section overrides and the resulting plan identity.

use crate::{
    CornerModelBinding, ModelConsumerScope, ModelResolutionRecord, SEALED_MODEL_SOURCE_MARKER,
};
use rspice_app_types::product::{ContentDigest, ProcessCorner};
use sha2::{Digest as _, Sha256};
use std::collections::BTreeMap;

/// Immutable, content-addressed model namespace used by one nominal run.
///
/// This is the semantic boundary shared by preflight, save validation, and
/// prepared-run construction.  It records the exact per-library corner that
/// was selected when sources were sealed and rejects a contested executable
/// namespace before the engine can fall back to first-definition lookup.
#[derive(Debug, Clone)]
pub struct ModelExecutionPlan {
    reference_process: ProcessCorner,
    selected_library_corners: Vec<(String, Option<String>)>,
    bindings: Vec<CornerModelBinding>,
    applied_resolutions: Vec<ModelResolutionRecord>,
    digest: ContentDigest,
}

impl ModelExecutionPlan {
    /// Resolve one nominal process from already materialized provider sources.
    /// Library order and selected sections come from the caller's authenticated
    /// catalog snapshot and remain part of the plan identity.
    pub fn try_new(
        process: ProcessCorner,
        selected_library_corners: Vec<(String, Option<String>)>,
        materialized: Vec<MaterializedPlanBinding>,
        records: &[ModelResolutionRecord],
    ) -> Result<Self, String> {
        if materialized
            .iter()
            .any(|provider| provider.binding.process != process)
        {
            return Err(
                "Model execution plan contains a binding for a different process".to_owned(),
            );
        }
        let (bindings, applied_resolutions) =
            resolve_materialized_definition_namespace(materialized, records)?;

        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-plan/v2\0");
        hasher.update(process.short_name().as_bytes());
        for (library, corner) in &selected_library_corners {
            hash_plan_field(&mut hasher, library.as_bytes());
            hash_plan_field(&mut hasher, corner.as_deref().unwrap_or("").as_bytes());
        }
        for binding in &bindings {
            hash_plan_field(&mut hasher, binding.source_label.as_bytes());
            hash_plan_field(
                &mut hasher,
                binding.section.as_deref().unwrap_or("").as_bytes(),
            );
            hash_plan_field(&mut hasher, binding.materialized_model_cards.as_bytes());
        }
        for resolution in &applied_resolutions {
            let bytes = serde_json::to_vec(resolution).map_err(|error| {
                format!("Cannot digest applied model provider decision: {error}")
            })?;
            hash_plan_field(&mut hasher, &bytes);
        }
        let digest = ContentDigest::from_bytes(hasher.finalize().into());

        Ok(ModelExecutionPlan {
            reference_process: process,
            selected_library_corners,
            bindings,
            applied_resolutions,
            digest,
        })
    }

    #[must_use]
    pub const fn reference_process(&self) -> ProcessCorner {
        self.reference_process
    }

    #[must_use]
    pub fn selected_library_corners(&self) -> &[(String, Option<String>)] {
        &self.selected_library_corners
    }

    #[must_use]
    pub fn bindings(&self) -> &[CornerModelBinding] {
        &self.bindings
    }

    #[must_use]
    pub fn applied_resolutions(&self) -> &[ModelResolutionRecord] {
        &self.applied_resolutions
    }

    #[must_use]
    pub const fn digest(&self) -> ContentDigest {
        self.digest
    }

    #[must_use]
    pub fn model_cards(&self) -> Vec<String> {
        self.bindings
            .iter()
            .map(|binding| {
                format!(
                    "{SEALED_MODEL_SOURCE_MARKER}{}\n{}",
                    binding.source_label, binding.materialized_model_cards
                )
            })
            .collect()
    }
}

/// One materialized source and the provider identity established by its caller.
/// Source authentication and project-section override authorization belong to the
/// source adapter; construction checks that the cards are self-contained.
#[derive(Debug, Clone)]
pub struct MaterializedPlanBinding {
    binding: CornerModelBinding,
    provider_library: String,
    provider_source_digest: ContentDigest,
    allows_selected_section_override: bool,
}

impl MaterializedPlanBinding {
    pub fn try_new(
        binding: CornerModelBinding,
        provider_library: String,
        provider_source_digest: ContentDigest,
        allows_selected_section_override: bool,
    ) -> Result<Self, String> {
        binding.validate()?;
        Ok(Self {
            binding,
            provider_library,
            provider_source_digest,
            allows_selected_section_override,
        })
    }
}

#[derive(Debug, Clone)]
struct MaterializedDefinition {
    scope: ModelConsumerScope,
    normalized_name: String,
    exact_name: String,
    binding_index: usize,
    name_span: std::ops::Range<usize>,
}

fn hash_plan_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn materialized_definitions(binding_index: usize, cards: &str) -> Vec<MaterializedDefinition> {
    // The editor source map deliberately treats the first line as a SPICE
    // title. Prefix one title so the first actual model card is inspected.
    let prefix = "RSpice materialized provider\n";
    let wrapped = format!("{prefix}{cards}");
    let map = rspice_core::netlist::source_map_for_editor(&wrapped);
    map.model_defs
        .into_iter()
        .filter(|definition| definition.scope.is_none())
        .map(|definition| MaterializedDefinition {
            scope: ModelConsumerScope::PrimitiveModel,
            normalized_name: definition.name.to_ascii_lowercase(),
            exact_name: definition.name,
            binding_index,
            name_span: (definition.span.start - prefix.len())..(definition.span.end - prefix.len()),
        })
        .chain(
            map.subckt_defs
                .into_iter()
                .filter(|definition| definition.scope.is_none())
                .map(|definition| MaterializedDefinition {
                    scope: ModelConsumerScope::Subcircuit,
                    normalized_name: definition.name.to_ascii_lowercase(),
                    exact_name: definition.name,
                    binding_index,
                    name_span: (definition.span.start - prefix.len())
                        ..(definition.span.end - prefix.len()),
                }),
        )
        .collect()
}

/// Canonical project-owned model revisions intentionally carry one top-level
/// base card plus one complete card in each `.lib` section. Selecting such a
/// section must replace the base card, while duplicates in imported or signed
/// sources remain errors. Perform that one narrowly authorized rewrite before
/// resolving conflicts between independent providers.
fn apply_project_section_overrides(bindings: &mut [MaterializedPlanBinding]) -> Result<(), String> {
    let definitions = bindings
        .iter()
        .enumerate()
        .flat_map(|(index, binding)| {
            materialized_definitions(index, &binding.binding.materialized_model_cards)
        })
        .collect::<Vec<_>>();
    let mut groups = BTreeMap::<(ModelConsumerScope, String), Vec<&MaterializedDefinition>>::new();
    for definition in &definitions {
        groups
            .entry((definition.scope, definition.normalized_name.clone()))
            .or_default()
            .push(definition);
    }
    let mut losers = BTreeMap::<usize, Vec<&MaterializedDefinition>>::new();
    let mut unresolved = Vec::new();
    for ((scope, normalized_name), providers) in groups {
        let mut same_source = BTreeMap::<(String, String), Vec<&MaterializedDefinition>>::new();
        for definition in providers {
            let binding = &bindings[definition.binding_index];
            same_source
                .entry((
                    binding.provider_library.clone(),
                    binding.provider_source_digest.to_string(),
                ))
                .or_default()
                .push(definition);
        }
        for ((library, digest), definitions) in same_source {
            if definitions.len() < 2 {
                continue;
            }
            let binding_index = definitions[0].binding_index;
            let authorized = definitions
                .iter()
                .all(|definition| definition.binding_index == binding_index)
                && bindings[binding_index].allows_selected_section_override
                && bindings[binding_index].binding.section.is_some();
            if !authorized {
                unresolved.push(format!(
                    "{} '{}' is repeated inside authenticated provider '{}' at source {}",
                    scope.label(),
                    normalized_name,
                    library,
                    digest
                ));
                continue;
            }
            let winner_span = definitions
                .iter()
                .max_by_key(|definition| definition.name_span.start)
                .map(|definition| definition.name_span.clone())
                .expect("same-source override group is nonempty");
            for definition in definitions {
                if definition.name_span != winner_span {
                    losers
                        .entry(definition.binding_index)
                        .or_default()
                        .push(definition);
                }
            }
        }
    }
    if !unresolved.is_empty() {
        unresolved.truncate(8);
        return Err(format!(
            "Executable model namespace is contested and fails closed: {}. Repair the duplicate source before simulation.",
            unresolved.join("; ")
        ));
    }
    for (binding_index, mut definitions) in losers {
        definitions.sort_by_key(|definition| std::cmp::Reverse(definition.name_span.start));
        for definition in definitions {
            bindings[binding_index].binding.materialized_model_cards =
                mask_materialized_definition(
                    &bindings[binding_index].binding.materialized_model_cards,
                    definition,
                )?;
        }
    }
    Ok(())
}

/// Apply exact project-owned provider decisions to a frozen materialization.
/// Losing definitions are blanked before the engine parses the cards, so the
/// engine consumes one unambiguous namespace rather than relying on include
/// order or first-match lookup.
pub fn resolve_materialized_definition_namespace(
    mut bindings: Vec<MaterializedPlanBinding>,
    records: &[ModelResolutionRecord],
) -> Result<(Vec<CornerModelBinding>, Vec<ModelResolutionRecord>), String> {
    apply_project_section_overrides(&mut bindings)?;
    let definitions = bindings
        .iter()
        .enumerate()
        .flat_map(|(index, binding)| {
            materialized_definitions(index, &binding.binding.materialized_model_cards)
        })
        .collect::<Vec<_>>();
    let mut groups = BTreeMap::<(ModelConsumerScope, String), Vec<&MaterializedDefinition>>::new();
    for definition in &definitions {
        groups
            .entry((definition.scope, definition.normalized_name.clone()))
            .or_default()
            .push(definition);
    }
    let mut record_index = BTreeMap::new();
    for record in records {
        record.validate()?;
        let key = (record.consumer_scope, record.normalized_name.clone());
        if record_index.insert(key, record).is_some() {
            return Err(format!(
                "model-resolution record '{}' is repeated",
                record.key()
            ));
        }
    }
    let mut losers = BTreeMap::<usize, Vec<&MaterializedDefinition>>::new();
    let mut applied = Vec::new();
    let mut unresolved = Vec::new();

    for ((scope, normalized_name), providers) in groups {
        if providers.len() < 2 {
            continue;
        }
        let provider_descriptions = providers
            .iter()
            .map(|definition| {
                let binding = &bindings[definition.binding_index];
                format!(
                    "{}/{} at {} (source {})",
                    binding.provider_library,
                    definition.exact_name,
                    binding.binding.source_label,
                    binding.provider_source_digest
                )
            })
            .collect::<Vec<_>>();
        if providers.iter().enumerate().any(|(index, left)| {
            providers.iter().skip(index + 1).any(|right| {
                let left = &bindings[left.binding_index];
                let right = &bindings[right.binding_index];
                left.provider_library == right.provider_library
                    && left.provider_source_digest == right.provider_source_digest
            })
        }) {
            unresolved.push(format!(
                "{} '{}' is repeated inside one authenticated provider: {}",
                scope.label(),
                normalized_name,
                provider_descriptions.join(", ")
            ));
            continue;
        }
        let Some(record) = record_index.get(&(scope, normalized_name.clone())) else {
            unresolved.push(format!(
                "{} '{}' from {}",
                scope.label(),
                normalized_name,
                provider_descriptions.join(", ")
            ));
            continue;
        };
        let winners = providers
            .iter()
            .filter(|definition| {
                let binding = &bindings[definition.binding_index];
                binding.provider_library == record.provider_library
                    && binding.provider_source_digest == record.provider_source_digest
                    && definition.exact_name == record.provider_definition
            })
            .copied()
            .collect::<Vec<_>>();
        if winners.len() != 1 {
            unresolved.push(format!(
                "{} '{}' has a stale provider decision for '{}/{}' at source {}; active providers are {}",
                scope.label(),
                normalized_name,
                record.provider_library,
                record.provider_definition,
                record.provider_source_digest,
                provider_descriptions.join(", ")
            ));
            continue;
        }
        let winner = winners[0];
        for provider in providers {
            if !std::ptr::eq(provider, winner) {
                losers
                    .entry(provider.binding_index)
                    .or_default()
                    .push(provider);
            }
        }
        applied.push((*record).clone());
    }

    if !unresolved.is_empty() {
        unresolved.truncate(8);
        return Err(format!(
            "Executable model namespace is contested and fails closed: {}. Publish an exact source-qualified provider decision or repair the duplicate source before simulation.",
            unresolved.join("; ")
        ));
    }

    for (binding_index, mut definitions) in losers {
        definitions.sort_by_key(|definition| std::cmp::Reverse(definition.name_span.start));
        for definition in definitions {
            bindings[binding_index].binding.materialized_model_cards =
                mask_materialized_definition(
                    &bindings[binding_index].binding.materialized_model_cards,
                    definition,
                )?;
        }
    }
    applied.sort_by_key(|left| left.key());
    applied.dedup_by(|left, right| left.key() == right.key());
    let bindings = bindings
        .into_iter()
        .filter(|binding| !binding.binding.materialized_model_cards.trim().is_empty())
        .map(|binding| {
            binding.binding.validate()?;
            Ok(binding.binding)
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok((bindings, applied))
}

fn mask_materialized_definition(
    cards: &str,
    definition: &MaterializedDefinition,
) -> Result<String, String> {
    if definition.name_span.end > cards.len() {
        return Err(format!(
            "cannot apply provider decision for '{}' because its source span is invalid",
            definition.exact_name
        ));
    }
    let bytes = cards.as_bytes();
    let start = bytes[..definition.name_span.start]
        .iter()
        .rposition(|byte| *byte == b'\n')
        .map_or(0, |index| index + 1);
    let mut end = physical_line_end(bytes, start);
    match definition.scope {
        ModelConsumerScope::PrimitiveModel => {
            while end < bytes.len() {
                let next_end = physical_line_end(bytes, end);
                let line = std::str::from_utf8(&bytes[end..next_end]).map_err(|error| {
                    format!("materialized model source is not UTF-8 at continuation: {error}")
                })?;
                if line.trim_start().starts_with('+') {
                    end = next_end;
                } else {
                    break;
                }
            }
        }
        ModelConsumerScope::Subcircuit => {
            let mut cursor = start;
            let mut depth = 0_usize;
            let mut closed = false;
            while cursor < bytes.len() {
                let next = physical_line_end(bytes, cursor);
                let line = std::str::from_utf8(&bytes[cursor..next]).map_err(|error| {
                    format!("materialized subcircuit source is not UTF-8: {error}")
                })?;
                let head = line.split_whitespace().next().unwrap_or("");
                if head.eq_ignore_ascii_case(".subckt") {
                    depth += 1;
                } else if head.eq_ignore_ascii_case(".ends") {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        end = next;
                        closed = true;
                        break;
                    }
                }
                cursor = next;
            }
            if !closed {
                return Err(format!(
                    "cannot apply provider decision because subcircuit '{}' has no matching .ENDS",
                    definition.exact_name
                ));
            }
        }
    }
    let mut masked = bytes.to_vec();
    for byte in &mut masked[start..end] {
        if *byte != b'\n' && *byte != b'\r' {
            *byte = b' ';
        }
    }
    String::from_utf8(masked)
        .map_err(|error| format!("resolved model materialization is not UTF-8: {error}"))
}

fn physical_line_end(bytes: &[u8], start: usize) -> usize {
    bytes[start..]
        .iter()
        .position(|byte| *byte == b'\n')
        .map_or(bytes.len(), |offset| start + offset + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const APPROVED: &str = ".model d D (IS=1e-14)\n";
    const ALTERNATIVE: &str = ".model D D (IS=2e-14)\n.model spare D (IS=3e-14)\n";

    fn provider(name: &str, process: ProcessCorner, cards: &str) -> MaterializedPlanBinding {
        MaterializedPlanBinding::try_new(
            CornerModelBinding {
                process,
                source_label: format!("{name} [tt]"),
                section: Some("tt".to_owned()),
                materialized_model_cards: cards.to_owned(),
            },
            name.to_owned(),
            ContentDigest::from_bytes(Sha256::digest(cards.as_bytes()).into()),
            false,
        )
        .expect("self-contained source")
    }

    fn decision() -> ModelResolutionRecord {
        ModelResolutionRecord {
            schema_version: crate::MODEL_RESOLUTION_RECORD_SCHEMA_VERSION,
            consumer_scope: ModelConsumerScope::PrimitiveModel,
            normalized_name: "d".to_owned(),
            provider_library: "approved".to_owned(),
            provider_definition: "d".to_owned(),
            provider_source_digest: ContentDigest::from_bytes(
                Sha256::digest(APPROVED.as_bytes()).into(),
            ),
            audit_reason: "Reviewed provider.".to_owned(),
            created_at_unix_ms: 1,
        }
    }

    #[test]
    fn resolved_namespace_preserves_the_app_v2_digest() {
        let plan = ModelExecutionPlan::try_new(
            ProcessCorner::TT,
            vec![
                ("approved".to_owned(), Some("tt".to_owned())),
                ("alternative".to_owned(), Some("tt".to_owned())),
            ],
            vec![
                provider("approved", ProcessCorner::TT, APPROVED),
                provider("alternative", ProcessCorner::TT, ALTERNATIVE),
            ],
            &[decision()],
        )
        .expect("exact provider decision resolves the namespace");
        // The pre-extraction v2 byte encoding, including masked source spans and the decision.
        assert_eq!(
            plan.digest().to_string(),
            "285deb90088ac39970445b5d9016cd1b381abc73707dc23da84cb9f91d034d71"
        );
        assert_eq!(plan.applied_resolutions(), &[decision()]);
        let cards = plan.model_cards().join("\n");
        assert!(cards.contains(APPROVED.trim()));
        assert!(!cards.contains("IS=2e-14"));
        assert!(cards.contains(".model spare D (IS=3e-14)"));
    }

    #[test]
    fn namespace_rejects_malformed_and_duplicate_decisions() {
        let sources = vec![provider("approved", ProcessCorner::TT, APPROVED)];
        let mut invalid = decision();
        invalid.created_at_unix_ms = 0;
        assert!(
            resolve_materialized_definition_namespace(sources.clone(), &[invalid])
                .expect_err("invalid decisions cannot cross the namespace boundary")
                .contains("timestamp")
        );
        assert!(
            resolve_materialized_definition_namespace(sources, &[decision(), decision()])
                .expect_err("duplicate decisions cannot overwrite each other")
                .contains("repeated")
        );
    }

    #[test]
    fn nominal_plan_rejects_a_binding_from_another_process() {
        let error = ModelExecutionPlan::try_new(
            ProcessCorner::TT,
            Vec::new(),
            vec![provider("approved", ProcessCorner::FF, APPROVED)],
            &[],
        )
        .expect_err("the nominal process must describe every binding in its digest");
        assert!(error.contains("different process"), "{error}");
    }
}
