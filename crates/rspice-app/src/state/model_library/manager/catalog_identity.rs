//! What this catalogue says it is, to whoever is asking.
//!
//! Two answers, because there are two questions. A prepared run and a
//! model-validation receipt need a *durable* identity that survives a project
//! file and can be compared years later, and they ask a handful of times per
//! session — [`ModelLibraryManager::execution_catalog_digest`] serializes every
//! library whole to give it. An inspection surface needs a repaint cache key
//! and asks on every frame, so it cannot afford that answer at all; it gets
//! [`ModelLibraryManager::design_inspection_catalog_key`], which hashes only
//! what a prepared deck reads. Both are content, so both expire when the
//! catalogue is replaced wholesale — the property a revision counter could not
//! have offered either of them.

#[cfg(test)]
mod tests;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash as _, Hasher as _};

use super::*;

/// The canonical SHA-256 identity of one library's accepted source.
///
/// A content-pinned library answers from its root pin. Serializing the whole
/// library is the fallback for one that was never pinned, and it is as
/// expensive as it sounds — see [`crate::state::CATALOG_LIBRARY_SERIALIZATIONS`],
/// which counts it, and the scale gate that holds the count at zero per frame.
pub(crate) fn model_library_source_digest(library: &ModelLibrary) -> ContentDigest {
    if let Some(root) = library.root_path.as_deref()
        && let Some(pin) = library.source_closure.iter().find(|pin| pin.path == root)
    {
        return pin.digest;
    }
    #[cfg(test)]
    crate::state::CATALOG_LIBRARY_SERIALIZATIONS.with(|count| count.set(count.get() + 1));
    let bytes = serde_json::to_value(library)
        .and_then(|canonical| serde_json::to_vec(&canonical))
        .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

impl ModelLibraryManager {
    /// Stable identity of the persisted model catalogue relevant to source
    /// preparation. Browser filters, selection, shipped-pack indexes, and
    /// audit ledgers are deliberately excluded.
    ///
    /// This is the durable one. It is written into a model-validation receipt
    /// and compared before a run is dispatched, so its bytes are a
    /// compatibility surface and the whole library participates. Nothing that
    /// repaints may ask for it; see
    /// [`Self::design_inspection_catalog_key`] for the answer that can be
    /// asked per frame.
    pub(crate) fn execution_catalog_digest(&self) -> ContentDigest {
        let mut libraries = self.libraries.values().collect::<Vec<_>>();
        libraries.sort_by(|left, right| left.name.cmp(&right.name));
        let mut hasher = Sha256::new();
        hasher.update(b"rspice.model-execution-catalog/v4\0");
        for library in libraries {
            // A library owns several `HashMap` fields, so serializing it
            // directly emits their entries in per-instance iteration order and
            // yields a different digest for identical content. Route through
            // `serde_json::Value`, whose objects are key-sorted maps, so the
            // catalogue identity depends only on the content itself. A prepared
            // run compares this digest before dispatch; an order-dependent one
            // expires authorized runs at random.
            #[cfg(test)]
            crate::state::CATALOG_LIBRARY_SERIALIZATIONS.with(|count| count.set(count.get() + 1));
            let bytes = serde_json::to_value(library)
                .and_then(|canonical| serde_json::to_vec(&canonical))
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        for record in self.resolution_records.values() {
            let bytes = serde_json::to_vec(record)
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }

    /// Cheap identity of everything the analysis-independent design inspection
    /// reads from this catalogue.
    ///
    /// # Why this is content, and not a counter
    ///
    /// [`ModelLibraryManager`] carries no revision counter, and adding one
    /// would not be sound. The manager is replaced *wholesale* when a project
    /// is opened, when a recovery comparison is accepted, when design history
    /// restores a candidate, and when a model-hub or import operation publishes
    /// a rebuilt candidate. A replacement arrives carrying whatever counter it
    /// was serialized with, which may be a value this session has already
    /// cached an answer against — and the page would then state this project's
    /// bin geometry from the previous project's cards. Hashing content has no
    /// such failure mode: a catalogue cannot present a key it did not earn,
    /// however it arrived.
    ///
    /// # What participates, and why that is all of it
    ///
    /// The guarded computation is
    /// `SimulationController::prepare_design_netlist_for_inspection`, which
    /// reads this catalogue in exactly three places:
    ///
    /// * `validate_projected_model_binding_authority` resolves every placed
    ///   instance through [`Self::definition_providers`], which enumerates each
    ///   library's `models` and `subcircuits` *names* — with a subcircuit's
    ///   section deciding whether it is visible at all — and each library's
    ///   source digest.
    /// * [`Self::seal_execution_sources_for_plan`] validates the plan bindings
    ///   and seals the closure, reading `name`, `root_path`,
    ///   `source_authority`, `source_closure`, `source_edges`,
    ///   `source_contents`, `corners`, and `selected_corner`.
    /// * Everything downstream reads the *sealed* snapshot: model cards are
    ///   materialized from the sealed bytes, never from the parsed projection,
    ///   so no field reaches the deck that sealing did not already carry.
    ///
    /// Retained bytes are named by this key rather than re-hashed into it.
    /// Sealing recomputes SHA-256 over every retained byte string and refuses
    /// the whole deck unless it equals the [`ModelSourcePin`] digest folded in
    /// here, so no deck the inspection can accept exists whose bytes are not
    /// the ones this key already names. Hashing them again would repeat the
    /// pass the guarded work performs, which is the cost this key exists to
    /// avoid; each retained path and byte length is folded in so a closure that
    /// is merely re-pinned still moves the key.
    ///
    /// Deliberately absent: `selected_library`, `filter_text`, and
    /// `filter_type` (browser presentation); the model-validation receipt (a
    /// claim *about* the catalogue that the inspection never reads); the
    /// shipped-pack index (an inventory of this machine, whose definitions
    /// reach a deck only once imported into a library); and every authoring,
    /// qualification, correlation, provenance, and presentation field a library
    /// owns. Their absence is the point — expanding a library row in the
    /// browser used to expire the exact model-bin inspection.
    pub(crate) fn design_inspection_catalog_key(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        "rspice.model-inspection-catalog/v1".hash(&mut hasher);
        // A `BTreeMap`, so its iteration order is the content's own.
        self.resolution_records.hash(&mut hasher);
        for library in self.libraries_sorted() {
            library.name.hash(&mut hasher);
            library.root_path.hash(&mut hasher);
            library.source_authority.hash(&mut hasher);
            library.source_closure.hash(&mut hasher);
            library.source_edges.hash(&mut hasher);
            for content in &library.source_contents {
                content.path.hash(&mut hasher);
                content.bytes.len().hash(&mut hasher);
            }
            hash_corner_set(&library.corners, &mut hasher);
            library.selected_corner.hash(&mut hasher);
            hash_definition_namespace(library, &mut hasher);
        }
        hasher.finish()
    }
}

/// Fold a library's process corners into the design-inspection key.
///
/// [`ProcessCorner`] carries `f64` fields and so cannot derive [`Hash`], and
/// hand-listing its fields here would quietly stop covering the next one
/// somebody adds. A library declares a handful of corners rather than a corpus,
/// so routing only the corners through canonical JSON stays affordable while
/// serde's derive keeps the coverage total. The map is re-keyed into a
/// [`BTreeMap`] first: `HashMap` iteration order belongs to the instance rather
/// than to the content, and a key that moved with it would never hit.
fn hash_corner_set(corners: &HashMap<String, ProcessCorner>, hasher: &mut DefaultHasher) {
    let ordered = corners
        .iter()
        .map(|(key, corner)| (key.as_str(), corner))
        .collect::<BTreeMap<_, _>>();
    match serde_json::to_vec(&ordered) {
        Ok(bytes) => bytes.hash(hasher),
        Err(error) => error.to_string().hash(hasher),
    }
}

/// Fold the definition names a provider decision can resolve to.
///
/// Only names and a subcircuit's section participate: that is precisely what
/// [`ModelLibraryManager::definition_providers`] compares, and the executable
/// cards themselves are materialized from the sealed source rather than from
/// these parsed values. Both maps iterate in instance order, so each entry is
/// hashed on its own and the entries are combined commutatively — the same fold
/// the symbol registry's key uses.
fn hash_definition_namespace(library: &ModelLibrary, hasher: &mut DefaultHasher) {
    let mut folded_xor = 0_u64;
    let mut folded_sum = 0_u64;
    for (key, model) in &library.models {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        model.name.hash(&mut entry);
        let entry = entry.finish();
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    for (key, subcircuit) in &library.subcircuits {
        let mut entry = DefaultHasher::new();
        key.hash(&mut entry);
        subcircuit.name.hash(&mut entry);
        subcircuit.section.hash(&mut entry);
        // Offset so a subcircuit can never fold to the same value as a model of
        // the same name in the same library.
        let entry = entry.finish().rotate_left(31) ^ 0x5375_6263_6B74_0001;
        folded_xor ^= entry;
        folded_sum = folded_sum.wrapping_add(entry.rotate_left(17));
    }
    library.models.len().hash(hasher);
    library.subcircuits.len().hash(hasher);
    folded_xor.hash(hasher);
    folded_sum.hash(hasher);
}
