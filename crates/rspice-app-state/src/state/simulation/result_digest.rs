//! Application adapters for canonical retained-result identity and storage budgets.

use super::*;
use crate::product::ContentDigest;
use rspice_results::result_digest::ResultDigestEncoding;

impl AnalysisResult {
    /// SHA-256 identity of the authoritative retained data and evidence for
    /// this analysis. Display labels, timestamps, colors and visibility are
    /// intentionally not part of the identity.
    #[must_use]
    pub fn result_data_digest(&self) -> ContentDigest {
        self.result_data_ref().digest(ResultDigestEncoding::CURRENT)
    }

    /// Logical bytes occupied by all authoritative retained result evidence.
    ///
    /// This deliberately follows the same complete field walk as the result
    /// digest, so adding an authenticated payload, measurement, waveform, or
    /// saved-output receipt cannot silently escape runtime retention budgets.
    #[must_use]
    pub fn retained_storage_bytes(&self) -> u64 {
        self.result_data_ref().retained_data_bytes()
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
