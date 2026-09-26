//! Application adapters for canonical retained-result identity and storage budgets.

use super::*;
use crate::product::ContentDigest;
use rspice_results::result_digest::ResultDigestEncoding;

impl AnalysisResult {
    /// SHA-256 identity of the authoritative retained data and evidence for
    /// this analysis. Display labels, timestamps, colors, visibility, and
    /// derived display caches are intentionally not part of the identity.
    #[must_use]
    pub fn result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::CURRENT)
    }

    /// Logical bytes occupied by all authoritative retained result evidence.
    ///
    /// This deliberately follows the same complete field walk as the result
    /// digest, so adding an authenticated payload, measurement, waveform, or
    /// saved-output receipt cannot silently escape runtime retention budgets.
    /// Presentation caches are added separately because they are intentionally
    /// excluded from immutable content identity.
    #[must_use]
    pub fn retained_storage_bytes(&self) -> u64 {
        let retained_bytes = self.result_data_ref().retained_data_bytes();
        let cache_bytes = self.waveforms.iter().fold(0_u64, |total, waveform| {
            let bytes = waveform.display_cache.as_ref().map_or(0_u64, |cache| {
                u64::try_from(cache.x.len())
                    .unwrap_or(u64::MAX)
                    .saturating_add(u64::try_from(cache.y.len()).unwrap_or(u64::MAX))
                    .saturating_mul(std::mem::size_of::<f32>() as u64)
                    .saturating_add(std::mem::size_of::<usize>() as u64)
            });
            total.saturating_add(bytes)
        });
        retained_bytes.saturating_add(cache_bytes)
    }

    /// Schema-v8 digest retained solely for authenticated migration. New
    /// result documents must use [`Self::result_data_digest`].
    #[must_use]
    pub(crate) fn legacy_v1_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V1)
    }

    /// Schema-v9 digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v2_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V2)
    }

    /// Schema-v10 digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v3_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V3)
    }

    /// Schema-v11 digest retained solely for authenticated migration. Its
    /// noise-summary encoding predates optional output noise and
    /// input-referred integrated noise evidence.
    #[must_use]
    pub(crate) fn legacy_v4_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V4)
    }

    /// Schema-v12 digest retained solely for authenticated migration. Its
    /// waveform encoding predates the per-waveform retained unit.
    #[must_use]
    pub(crate) fn legacy_v5_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V5)
    }

    /// Schema-v13 through schema-v15 digest retained solely for authenticated
    /// migration. Its pole-zero payload required a numeric DC gain.
    #[must_use]
    pub(crate) fn legacy_v6_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V6)
    }

    /// Schema-v16 digest retained solely for authenticated migration. It
    /// predates durable PSS/PSTB Floquet payloads.
    #[must_use]
    pub(crate) fn legacy_v7_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V7)
    }

    /// Schema-v17 digest retained solely for authenticated migration. It
    /// predates durable measurement FAILVALUE verification evidence.
    #[must_use]
    pub(crate) fn legacy_v8_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V8)
    }

    /// Schema-v18 digest retained solely for authenticated migration. It
    /// predates the digital bus table declared over retained event traces.
    #[must_use]
    pub(crate) fn legacy_v9_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V9)
    }

    /// Schema-v19 content identity, solely for authenticated migration.
    pub(crate) fn legacy_v10_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V10)
    }

    /// Schema-v20 content identity, solely for authenticated migration.
    pub(crate) fn legacy_v11_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V11)
    }

    fn result_data_digest_with_encoding(&self, version: ResultDigestEncoding) -> ContentDigest {
        self.result_data_ref().digest(version)
    }

    /// Digest used by project schemas 21 through 23, before physical bindings.
    pub(crate) fn legacy_v12_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V12)
    }

    /// Schema-v24 identity, solely for authenticated migration.
    pub(crate) fn legacy_v13_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V13)
    }

    /// Schema-v25 identity, solely for authenticated migration.
    pub(crate) fn legacy_v14_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V14)
    }

    pub(crate) fn legacy_v15_result_data_digest(&self) -> ContentDigest {
        self.result_data_digest_with_encoding(ResultDigestEncoding::V15)
    }
}

impl SimulationRun {
    /// SHA-256 identity of the ordered immutable analysis dataset retained by
    /// this run. Stable run/dataset IDs and wall-clock metadata are excluded;
    /// they address the dataset but do not define its sample content.
    #[must_use]
    pub fn dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT)
    }

    /// Schema-v8 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v1_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V1)
    }

    /// Schema-v9 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v2_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V2)
    }

    /// Schema-v10 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v3_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V3)
    }

    /// Schema-v11 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v4_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V4)
    }

    /// Schema-v12 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v5_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V5)
    }

    /// Schema-v13 through schema-v15 dataset digest retained solely for
    /// authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v6_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V6)
    }

    /// Schema-v16 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v7_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V7)
    }

    /// Schema-v17 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v8_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V8)
    }

    /// Schema-v18 dataset digest retained solely for authenticated migration.
    #[must_use]
    pub(crate) fn legacy_v9_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V9)
    }

    /// Schema-v19 dataset identity, solely for authenticated migration.
    pub(crate) fn legacy_v10_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V10)
    }

    /// Schema-v20 dataset identity, solely for authenticated migration.
    pub(crate) fn legacy_v11_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V11)
    }

    /// Dataset digest used by project schemas 21 through 23.
    pub(crate) fn legacy_v12_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V12)
    }

    /// Schema-v24 identity, solely for authenticated migration.
    pub(crate) fn legacy_v13_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V13)
    }

    /// Schema-v25 identity, solely for authenticated migration.
    pub(crate) fn legacy_v14_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V14)
    }

    pub(crate) fn legacy_v15_dataset_content_digest(&self) -> ContentDigest {
        self.dataset_content_digest_with_encoding(ResultDigestEncoding::V15)
    }

    fn dataset_content_digest_with_encoding(&self, version: ResultDigestEncoding) -> ContentDigest {
        rspice_results::result_digest::dataset_content_digest(
            self.analyses
                .iter()
                .map(|analysis| (analysis.id, analysis.result_data_ref())),
            version,
        )
    }
}

#[cfg(test)]
mod tests;
