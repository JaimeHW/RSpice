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
        self.result_data_ref().digest(ResultDigestEncoding::CURRENT)
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
}

impl SimulationRun {
    /// SHA-256 identity of the ordered immutable analysis dataset retained by
    /// this run. Stable run/dataset IDs and wall-clock metadata are excluded;
    /// they address the dataset but do not define its sample content.
    #[must_use]
    pub fn dataset_content_digest(&self) -> ContentDigest {
        self.data
            .dataset_content_digest_with_encoding(ResultDigestEncoding::CURRENT)
    }
}

#[cfg(test)]
mod tests;
