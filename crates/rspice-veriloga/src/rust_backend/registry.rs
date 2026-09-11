//! Assigning each generated device its unique name in `registry.rs`.
//!
//! Model names are not unique across a corpus — the same compact model ships
//! in several PDKs, sometimes at different revisions. When a public name is
//! claimed more than once, every claimant is disambiguated by its generated
//! namespace. Profiles can pin that namespace independently of source hashes,
//! keeping published names stable when a model or a shared header is repaired.
//!
//! Disambiguation is applied to *all* colliding devices rather than to all but
//! the first, and suffixed names are still checked against names already
//! reserved by unique models, so the mapping stays stable when an unrelated
//! model is added or removed.

use std::collections::{BTreeMap, BTreeSet};

use super::GeneratedRustDevice;

pub fn resolve_generated_registry_model_names(devices: &[GeneratedRustDevice]) -> Vec<String> {
    let mut public_name_counts = BTreeMap::new();
    for device in devices {
        *public_name_counts
            .entry(registry_key(&device.public_model_name))
            .or_insert(0usize) += 1;
    }

    let reserved_unique_names: BTreeSet<_> = public_name_counts
        .iter()
        .filter_map(|(name, count)| (*count == 1).then_some(name.clone()))
        .collect();
    let mut emitted = BTreeSet::new();
    let mut names = Vec::with_capacity(devices.len());

    for device in devices {
        let public_key = registry_key(&device.public_model_name);
        let is_duplicate_public_name =
            public_name_counts.get(&public_key).copied().unwrap_or(0) > 1;
        let base_name = if is_duplicate_public_name {
            format!(
                "{}__{}",
                device.public_model_name,
                registry_namespace_suffix(device)
            )
        } else {
            device.public_model_name.clone()
        };

        let mut candidate = base_name.clone();
        let mut collision_ordinal = 2usize;
        while emitted.contains(&registry_key(&candidate))
            || (is_duplicate_public_name
                && reserved_unique_names.contains(&registry_key(&candidate)))
        {
            candidate = format!("{base_name}__{collision_ordinal}");
            collision_ordinal += 1;
        }

        emitted.insert(registry_key(&candidate));
        names.push(candidate);
    }

    names
}

fn registry_digest_suffix(source_digest: &str) -> String {
    let suffix = source_digest.chars().take(8).collect::<String>();
    if suffix.is_empty() {
        "00000000".to_string()
    } else {
        suffix
    }
}

fn registry_namespace_suffix(device: &GeneratedRustDevice) -> String {
    device
        .folder_name
        .rsplit_once("__")
        .map(|(_, suffix)| suffix)
        .filter(|suffix| suffix.len() == 8 && suffix.bytes().all(|byte| byte.is_ascii_hexdigit()))
        .map(str::to_owned)
        .unwrap_or_else(|| registry_digest_suffix(&device.source_digest))
}

fn registry_key(name: &str) -> String {
    name.to_ascii_uppercase()
}
