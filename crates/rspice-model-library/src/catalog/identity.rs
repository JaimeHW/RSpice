//! Stable source and execution identities for the portable model catalog.

use super::ModelCatalog;
use crate::{ModelLibrary, ModelResolutionRecords};
use rspice_app_types::product::ContentDigest;
use sha2::{Digest as _, Sha256};

#[cfg(any(test, feature = "catalog-test-observation"))]
thread_local! {
    /// How many model libraries have been serialized whole on this thread.
    ///
    /// Canonicalizing a library routes it through `serde_json::Value`, which
    /// allocates a node per model, a node per parameter of every model, and —
    /// retained source bytes being a `Vec<u8>` — a node per byte of every
    /// pinned source file, so one pass over a production catalogue is the whole
    /// corpus several times over. The result is only ever *compared*, so the
    /// cost leaves no trace in what a frame paints; counting is the only way to
    /// state it as a test.
    pub static CATALOG_LIBRARY_SERIALIZATIONS: std::cell::Cell<usize> =
        const { std::cell::Cell::new(0) };
}

/// The canonical SHA-256 identity of one library's accepted source.
///
/// A content-pinned library answers from its root pin. Serializing the whole
/// library is the fallback for one that was never pinned, and it is as
/// expensive as it sounds. The existing scale gate requires zero whole-library
/// serializations per frame.
pub fn model_library_source_digest(library: &ModelLibrary) -> ContentDigest {
    if let Some(digest) = library.pinned_root_digest() {
        return digest;
    }
    #[cfg(any(test, feature = "catalog-test-observation"))]
    CATALOG_LIBRARY_SERIALIZATIONS.with(|count| count.set(count.get() + 1));
    let bytes = serde_json::to_value(library)
        .and_then(|canonical| serde_json::to_vec(&canonical))
        .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
    ContentDigest::from_bytes(Sha256::digest(bytes).into())
}

fn hash_validation_source_field(hasher: &mut Sha256, value: &[u8]) {
    hasher.update((value.len() as u64).to_le_bytes());
    hasher.update(value);
}

impl ModelCatalog {
    /// Stable identity of the ordered source closure retained by the catalog.
    pub fn model_validation_source_identity(&self) -> (u64, ContentDigest) {
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

    /// Durable execution identity; serializes each library and retained provider decision.
    pub fn execution_catalog_digest(&self, records: &ModelResolutionRecords) -> ContentDigest {
        let mut libraries = self.libraries().collect::<Vec<_>>();
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
            #[cfg(any(test, feature = "catalog-test-observation"))]
            CATALOG_LIBRARY_SERIALIZATIONS.with(|count| count.set(count.get() + 1));
            let bytes = serde_json::to_value(library)
                .and_then(|canonical| serde_json::to_vec(&canonical))
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        for record in records.as_map().values() {
            let bytes = serde_json::to_vec(record)
                .unwrap_or_else(|error| format!("serialization-error:{error}").into_bytes());
            hasher.update((bytes.len() as u64).to_le_bytes());
            hasher.update(bytes);
        }
        ContentDigest::from_bytes(hasher.finalize().into())
    }
}
